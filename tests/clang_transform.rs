use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_in_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(29);
    transform.gen_input_transform(25);
    transform.gen_input_transform(18);
    transform.gen_input_transform(16);
    transform.gen_input_transform(12);
    transform.gen_input_transform(8);
    println!("Code: {}", transform.out());
}
