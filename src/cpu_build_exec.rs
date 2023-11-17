use crate::clang_writer::*;
use gatesim::*;
use libloading::{Library, Symbol};
use static_init::dynamic;
use std::process::Command;
use thiserror::Error;

use std::env::{self, temp_dir};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Error, Debug)]
enum DetectCPUError {
    #[error("IO error {0}")]
    IOError(#[from] io::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CPUExtension {
    NoExtension,
    IntelMMX,
    IntelSSE,
    IntelAVX,
    IntelAVX512,
    ARMNEON,
}

fn detect_cpu_from_file(file: impl BufRead) -> Result<CPUExtension, DetectCPUError> {
    let mut have_fp = false;
    let mut is_armv8 = false;
    let mut have_flags = false;
    for rl in file.lines() {
        let line = rl?;
        if line.starts_with("flags") || line.starts_with("Features") {
            if line.find(" avx512").is_some() {
                return Ok(CPUExtension::IntelAVX512);
            } else if line.find(" avx").is_some() {
                return Ok(CPUExtension::IntelAVX);
            } else if line.find(" sse").is_some() {
                return Ok(CPUExtension::IntelSSE);
            } else if line.find(" mmx").is_some() {
                return Ok(CPUExtension::IntelMMX);
            } else if line.find(" neon").is_some() {
                return Ok(CPUExtension::ARMNEON);
            } else if line.find(" fp").is_some() {
                have_fp = true;
            }
            have_flags = true;
        } else if line.starts_with("CPU architecture: 8") {
            is_armv8 = true;
        }
        if have_flags && is_armv8 {
            if have_fp {
                // NEON is default for ARMv8 with FP.
                return Ok(CPUExtension::ARMNEON);
            } else {
                return Ok(CPUExtension::NoExtension);
            }
        }
    }
    Ok(CPUExtension::NoExtension)
}

fn detect_cpu() -> Result<CPUExtension, DetectCPUError> {
    detect_cpu_from_file(BufReader::new(File::open("/proc/cpuinfo")?))
}

#[dynamic]
static CPU_EXTENSION: CPUExtension = detect_cpu().unwrap_or(CPUExtension::NoExtension);

// configurations

struct BuildConfig<'a> {
    writer_config: &'a CLangWriterConfig<'a>,
    extra_flags: &'a [&'a str],
}

#[cfg(target_pointer_width = "32")]
const BUILD_CONFIG_U32: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_U32,
    extra_flags: &[],
};

#[cfg(target_pointer_width = "64")]
const BUILD_CONFIG_U64: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_U64,
    extra_flags: &[],
};

const BUILD_CONFIG_INTEL_MMX: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_MMX,
    extra_flags: &["-mmmx"],
};

const BUILD_CONFIG_INTEL_SSE: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_SSE,
    extra_flags: &["-msse"],
};

const BUILD_CONFIG_INTEL_AVX: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX,
    extra_flags: &["-mavx"],
};

const BUILD_CONFIG_INTEL_AVX512: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_INTEL_AVX512,
    extra_flags: &["-mavx512f"],
};

const BUILD_CONFIG_ARM_NEON: BuildConfig = BuildConfig {
    writer_config: &CLANG_WRITER_ARM_NEON,
    extra_flags: &["-mfpu=neon"],
};

fn get_build_config(cpu_ext: CPUExtension) -> BuildConfig<'static> {
    match cpu_ext {
        CPUExtension::NoExtension => {
            #[cfg(target_pointer_width = "32")]
            {
                BUILD_CONFIG_U32
            }
            #[cfg(target_pointer_width = "64")]
            {
                BUILD_CONFIG_U64
            }
        }
        CPUExtension::IntelMMX => BUILD_CONFIG_INTEL_MMX,
        CPUExtension::IntelSSE => BUILD_CONFIG_INTEL_SSE,
        CPUExtension::IntelAVX => BUILD_CONFIG_INTEL_AVX,
        CPUExtension::IntelAVX512 => BUILD_CONFIG_INTEL_AVX512,
        CPUExtension::ARMNEON => BUILD_CONFIG_ARM_NEON,
    }
}

// shared library object

#[dynamic]
static mut TIMESTAMP: u128 = {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
};

#[dynamic]
static GATE_SYS_CC: String = env::var("GATE_SYS_CC").unwrap_or("clang".to_string());

fn get_timestamp() -> u128 {
    let mut lock = TIMESTAMP.write();
    let old = *lock;
    *lock += 1;
    old
}

struct SharedLib {
    cpu_ext: CPUExtension,
    source_path: PathBuf,
    shared_library_path: PathBuf,
}

impl SharedLib {
    fn new() -> Self {
        Self::new_with_cpu_ext(*CPU_EXTENSION)
    }

    fn new_with_cpu_ext(cpu_ext: CPUExtension) -> Self {
        let temp_dir_path = temp_dir();
        let unix_time = get_timestamp();
        Self {
            cpu_ext,
            source_path: temp_dir_path.join(format!("gate_x4x_source_{}.c", unix_time)),
            shared_library_path: temp_dir_path.join(format!("gate_x4x_lib_{}.so", unix_time)),
        }
    }

    fn build(&mut self, source: &[u8]) -> Result<Library, Box<dyn std::error::Error>> {
        fs::write(&self.source_path, source)?;
        let extra_flags = get_build_config(self.cpu_ext).extra_flags;
        let args = {
            let mut args = vec![];
            args.extend(extra_flags);
            args.extend([
                "-shared",
                "-FPIC",
                "-o",
                self.shared_library_path.to_str().unwrap(),
                "-Wall",
                "-O2",
                self.source_path.to_str().unwrap(),
            ]);
            args
        };

        let output = Command::new(&*GATE_SYS_CC).args(args).output()?;
        if !output.status.success() {
            return Err(String::from_utf8(output.stderr)?.into());
        }
        let lib = unsafe { Library::new(&self.shared_library_path)? };
        if fs::remove_file(&self.source_path).is_err() {
            eprintln!("File {:?} doesn't exist", self.source_path);
        }
        if fs::remove_file(&self.shared_library_path).is_err() {
            eprintln!("File {:?} doesn't exist", self.shared_library_path);
        }
        Ok(lib)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cpu_from_file() {
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelMMX,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelSSE,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX,
            detect_cpu_from_file(&mut BufReader::new(&b"flags\t\t: fpu mmx sse sse2 avx"[..]))
                .unwrap()
        );
        assert_eq!(
            CPUExtension::IntelAVX512,
            detect_cpu_from_file(&mut BufReader::new(
                &b"flags\t\t: fpu mmx sse sse2 avx avx512f"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(&b"Features\t: fp neon"[..])).unwrap()
        );
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(
                &b"Features\t: xxx\nCPU architecture: 8\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(
                &b"Features\t: fp\nCPU architecture: 8\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::NoExtension,
            detect_cpu_from_file(&mut BufReader::new(
                &b"CPU architecture: 8\nFeatures\t: xxx\n"[..]
            ))
            .unwrap()
        );
        assert_eq!(
            CPUExtension::ARMNEON,
            detect_cpu_from_file(&mut BufReader::new(
                &b"CPU architecture: 8\nFeatures\t: fp\n"[..]
            ))
            .unwrap()
        );
    }
}
