use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_output_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    for i in (1..=32).rev() {
        transform.gen_output_transform(i);
    }
    println!("{}", transform.out());
}
