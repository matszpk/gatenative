use gatesim::*;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use thiserror::Error;

#[derive(Error, Debug)]
enum DetectCPUError {
    #[error("Unknown error")]
    SyntaxError,
    #[error("IO error {0}")]
    IOError(#[from] io::Error),
}

#[derive(Clone, Copy)]
enum CPUExtension {
    NoExtension,
    IntelMMX,
    IntelSSE,
    IntelAVX,
    IntelAVX512,
    ARMNEON,
}

fn detect_cpu(file: impl BufRead) -> Result<CPUExtension, DetectCPUError> {
    let mut have_fp = false;
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
                return Ok(CPUExtension::IntelSSE);
            } else if line.find(" neon").is_some() {
                return Ok(CPUExtension::ARMNEON);
            } else if line.find(" fp").is_some() {
                have_fp = true;
            }
        } else if line.starts_with("CPU architecture: 8") {
            // NEON is default for ARMv8 with FP.
            return if have_fp {
                Ok(CPUExtension::ARMNEON)
            } else {
                Ok(CPUExtension::NoExtension)
            };
        }
    }
    Ok(CPUExtension::NoExtension)
}
