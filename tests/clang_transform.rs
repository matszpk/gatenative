use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_in_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(32);
    println!("Code: {}", transform.out());
}
