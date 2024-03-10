use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_out_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    for i in (17..=32).rev() {
        transform.gen_output_transform(i);
    }
    println!("{}", transform.out());
}
