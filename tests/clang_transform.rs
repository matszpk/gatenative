use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_in_transform() {
    let mut transform = CLANG_TRANSFORM_INTEL_AVX512.transform();
    for i in (1..=32).rev() {
        transform.gen_input_transform(i);
    }
    println!("Code: {}", transform.out());
}
