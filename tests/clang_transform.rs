use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_in_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(32);
    assert_eq!(
        r##"#define IN_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    uint32_t t0v9;\
    uint32_t t0v10;\
    uint32_t t0v11;\
    uint32_t t0v12;\
    uint32_t t0v13;\
    uint32_t t0v14;\
    uint32_t t0v15;\
    uint32_t t0v16;\
    uint32_t t0v17;\
    uint32_t t0v18;\
    uint32_t t0v19;\
    uint32_t t0v20;\
    uint32_t t0v21;\
    uint32_t t0v22;\
    uint32_t t0v23;\
    uint32_t t0v24;\
    uint32_t t0v25;\
    uint32_t t0v26;\
    uint32_t t0v27;\
    uint32_t t0v28;\
    uint32_t t0v29;\
    uint32_t t0v30;\
    uint32_t t0v31;\
    uint32_t t0v32;\
    t0v0 = ((((S)[0]) & 0x0000ffffU) | ((((S)[16]) & 0x0000ffffU) << 16));\
    t0v1 = (((((S)[0]) & 0xffff0000U) >> 16) | (((S)[16]) & 0xffff0000U));\
    t0v2 = ((((S)[1]) & 0x0000ffffU) | ((((S)[17]) & 0x0000ffffU) << 16));\
    t0v3 = (((((S)[1]) & 0xffff0000U) >> 16) | (((S)[17]) & 0xffff0000U));\
    t0v4 = ((((S)[2]) & 0x0000ffffU) | ((((S)[18]) & 0x0000ffffU) << 16));\
    t0v5 = (((((S)[2]) & 0xffff0000U) >> 16) | (((S)[18]) & 0xffff0000U));\
    t0v6 = ((((S)[3]) & 0x0000ffffU) | ((((S)[19]) & 0x0000ffffU) << 16));\
    t0v7 = (((((S)[3]) & 0xffff0000U) >> 16) | (((S)[19]) & 0xffff0000U));\
    t0v8 = ((((S)[4]) & 0x0000ffffU) | ((((S)[20]) & 0x0000ffffU) << 16));\
    t0v9 = (((((S)[4]) & 0xffff0000U) >> 16) | (((S)[20]) & 0xffff0000U));\
    t0v10 = ((((S)[5]) & 0x0000ffffU) | ((((S)[21]) & 0x0000ffffU) << 16));\
    t0v11 = (((((S)[5]) & 0xffff0000U) >> 16) | (((S)[21]) & 0xffff0000U));\
    t0v12 = ((((S)[6]) & 0x0000ffffU) | ((((S)[22]) & 0x0000ffffU) << 16));\
    t0v13 = (((((S)[6]) & 0xffff0000U) >> 16) | (((S)[22]) & 0xffff0000U));\
    t0v14 = ((((S)[7]) & 0x0000ffffU) | ((((S)[23]) & 0x0000ffffU) << 16));\
    t0v15 = (((((S)[7]) & 0xffff0000U) >> 16) | (((S)[23]) & 0xffff0000U));\
    t0v16 = ((((S)[8]) & 0x0000ffffU) | ((((S)[24]) & 0x0000ffffU) << 16));\
    t0v17 = (((((S)[8]) & 0xffff0000U) >> 16) | (((S)[24]) & 0xffff0000U));\
    t0v18 = ((((S)[9]) & 0x0000ffffU) | ((((S)[25]) & 0x0000ffffU) << 16));\
    t0v19 = (((((S)[9]) & 0xffff0000U) >> 16) | (((S)[25]) & 0xffff0000U));\
    t0v20 = ((((S)[10]) & 0x0000ffffU) | ((((S)[26]) & 0x0000ffffU) << 16));\
    t0v21 = (((((S)[10]) & 0xffff0000U) >> 16) | (((S)[26]) & 0xffff0000U));\
    t0v22 = ((((S)[11]) & 0x0000ffffU) | ((((S)[27]) & 0x0000ffffU) << 16));\
    t0v23 = (((((S)[11]) & 0xffff0000U) >> 16) | (((S)[27]) & 0xffff0000U));\
    t0v24 = ((((S)[12]) & 0x0000ffffU) | ((((S)[28]) & 0x0000ffffU) << 16));\
    t0v25 = (((((S)[12]) & 0xffff0000U) >> 16) | (((S)[28]) & 0xffff0000U));\
    t0v26 = ((((S)[13]) & 0x0000ffffU) | ((((S)[29]) & 0x0000ffffU) << 16));\
    t0v27 = (((((S)[13]) & 0xffff0000U) >> 16) | (((S)[29]) & 0xffff0000U));\
    t0v28 = ((((S)[14]) & 0x0000ffffU) | ((((S)[30]) & 0x0000ffffU) << 16));\
    t0v29 = (((((S)[14]) & 0xffff0000U) >> 16) | (((S)[30]) & 0xffff0000U));\
    t0v30 = ((((S)[15]) & 0x0000ffffU) | ((((S)[31]) & 0x0000ffffU) << 16));\
    t0v31 = (((((S)[15]) & 0xffff0000U) >> 16) | (((S)[31]) & 0xffff0000U));\
    t0v32 = ((t0v0 & 0x00ff00ffU) | ((t0v16 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v16 & 0xff00ff00U));\
    t0v16 = ((t0v2 & 0x00ff00ffU) | ((t0v18 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v18 & 0xff00ff00U));\
    t0v18 = ((t0v4 & 0x00ff00ffU) | ((t0v20 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v20 & 0xff00ff00U));\
    t0v20 = ((t0v6 & 0x00ff00ffU) | ((t0v22 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v22 & 0xff00ff00U));\
    t0v22 = ((t0v8 & 0x00ff00ffU) | ((t0v24 & 0x00ff00ffU) << 8));\
    t0v8 = (((t0v8 & 0xff00ff00U) >> 8) | (t0v24 & 0xff00ff00U));\
    t0v24 = ((t0v10 & 0x00ff00ffU) | ((t0v26 & 0x00ff00ffU) << 8));\
    t0v10 = (((t0v10 & 0xff00ff00U) >> 8) | (t0v26 & 0xff00ff00U));\
    t0v26 = ((t0v12 & 0x00ff00ffU) | ((t0v28 & 0x00ff00ffU) << 8));\
    t0v12 = (((t0v12 & 0xff00ff00U) >> 8) | (t0v28 & 0xff00ff00U));\
    t0v28 = ((t0v14 & 0x00ff00ffU) | ((t0v30 & 0x00ff00ffU) << 8));\
    t0v14 = (((t0v14 & 0xff00ff00U) >> 8) | (t0v30 & 0xff00ff00U));\
    t0v30 = ((t0v1 & 0x00ff00ffU) | ((t0v17 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v17 & 0xff00ff00U));\
    t0v17 = ((t0v3 & 0x00ff00ffU) | ((t0v19 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v19 & 0xff00ff00U));\
    t0v19 = ((t0v5 & 0x00ff00ffU) | ((t0v21 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v5 & 0xff00ff00U) >> 8) | (t0v21 & 0xff00ff00U));\
    t0v21 = ((t0v7 & 0x00ff00ffU) | ((t0v23 & 0x00ff00ffU) << 8));\
    t0v7 = (((t0v7 & 0xff00ff00U) >> 8) | (t0v23 & 0xff00ff00U));\
    t0v23 = ((t0v9 & 0x00ff00ffU) | ((t0v25 & 0x00ff00ffU) << 8));\
    t0v9 = (((t0v9 & 0xff00ff00U) >> 8) | (t0v25 & 0xff00ff00U));\
    t0v25 = ((t0v11 & 0x00ff00ffU) | ((t0v27 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v11 & 0xff00ff00U) >> 8) | (t0v27 & 0xff00ff00U));\
    t0v27 = ((t0v13 & 0x00ff00ffU) | ((t0v29 & 0x00ff00ffU) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00U) >> 8) | (t0v29 & 0xff00ff00U));\
    t0v29 = ((t0v15 & 0x00ff00ffU) | ((t0v31 & 0x00ff00ffU) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v31 & 0xff00ff00U));\
    t0v31 = ((t0v32 & 0x0f0f0f0fU) | ((t0v22 & 0x0f0f0f0fU) << 4));\
    t0v22 = (((t0v32 & 0xf0f0f0f0U) >> 4) | (t0v22 & 0xf0f0f0f0U));\
    t0v32 = ((t0v16 & 0x0f0f0f0fU) | ((t0v24 & 0x0f0f0f0fU) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v24 & 0xf0f0f0f0U));\
    t0v24 = ((t0v18 & 0x0f0f0f0fU) | ((t0v26 & 0x0f0f0f0fU) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0U) >> 4) | (t0v26 & 0xf0f0f0f0U));\
    t0v26 = ((t0v20 & 0x0f0f0f0fU) | ((t0v28 & 0x0f0f0f0fU) << 4));\
    t0v20 = (((t0v20 & 0xf0f0f0f0U) >> 4) | (t0v28 & 0xf0f0f0f0U));\
    t0v28 = ((t0v0 & 0x0f0f0f0fU) | ((t0v8 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v8 & 0xf0f0f0f0U));\
    t0v8 = ((t0v2 & 0x0f0f0f0fU) | ((t0v10 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v10 & 0xf0f0f0f0U));\
    t0v10 = ((t0v4 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v4 = (((t0v4 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v6 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v6 = (((t0v6 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v30 & 0x0f0f0f0fU) | ((t0v23 & 0x0f0f0f0fU) << 4));\
    t0v23 = (((t0v30 & 0xf0f0f0f0U) >> 4) | (t0v23 & 0xf0f0f0f0U));\
    t0v30 = ((t0v17 & 0x0f0f0f0fU) | ((t0v25 & 0x0f0f0f0fU) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0U) >> 4) | (t0v25 & 0xf0f0f0f0U));\
    t0v25 = ((t0v19 & 0x0f0f0f0fU) | ((t0v27 & 0x0f0f0f0fU) << 4));\
    t0v19 = (((t0v19 & 0xf0f0f0f0U) >> 4) | (t0v27 & 0xf0f0f0f0U));\
    t0v27 = ((t0v21 & 0x0f0f0f0fU) | ((t0v29 & 0x0f0f0f0fU) << 4));\
    t0v21 = (((t0v21 & 0xf0f0f0f0U) >> 4) | (t0v29 & 0xf0f0f0f0U));\
    t0v29 = ((t0v1 & 0x0f0f0f0fU) | ((t0v9 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v9 & 0xf0f0f0f0U));\
    t0v9 = ((t0v3 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v5 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v5 = (((t0v5 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v7 & 0x0f0f0f0fU) | ((t0v15 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v15 & 0xf0f0f0f0U));\
    t0v15 = ((t0v31 & 0x33333333U) | ((t0v24 & 0x33333333U) << 2));\
    t0v24 = (((t0v31 & 0xccccccccU) >> 2) | (t0v24 & 0xccccccccU));\
    t0v31 = ((t0v32 & 0x33333333U) | ((t0v26 & 0x33333333U) << 2));\
    t0v26 = (((t0v32 & 0xccccccccU) >> 2) | (t0v26 & 0xccccccccU));\
    t0v32 = ((t0v22 & 0x33333333U) | ((t0v18 & 0x33333333U) << 2));\
    t0v18 = (((t0v22 & 0xccccccccU) >> 2) | (t0v18 & 0xccccccccU));\
    t0v22 = ((t0v16 & 0x33333333U) | ((t0v20 & 0x33333333U) << 2));\
    t0v16 = (((t0v16 & 0xccccccccU) >> 2) | (t0v20 & 0xccccccccU));\
    t0v20 = ((t0v28 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v10 = (((t0v28 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v28 = ((t0v8 & 0x33333333U) | ((t0v12 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v12 & 0xccccccccU));\
    t0v12 = ((t0v0 & 0x33333333U) | ((t0v4 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v4 & 0xccccccccU));\
    t0v4 = ((t0v2 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v2 = (((t0v2 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v14 & 0x33333333U) | ((t0v25 & 0x33333333U) << 2));\
    t0v14 = (((t0v14 & 0xccccccccU) >> 2) | (t0v25 & 0xccccccccU));\
    t0v25 = ((t0v30 & 0x33333333U) | ((t0v27 & 0x33333333U) << 2));\
    t0v27 = (((t0v30 & 0xccccccccU) >> 2) | (t0v27 & 0xccccccccU));\
    t0v30 = ((t0v23 & 0x33333333U) | ((t0v19 & 0x33333333U) << 2));\
    t0v19 = (((t0v23 & 0xccccccccU) >> 2) | (t0v19 & 0xccccccccU));\
    t0v23 = ((t0v17 & 0x33333333U) | ((t0v21 & 0x33333333U) << 2));\
    t0v17 = (((t0v17 & 0xccccccccU) >> 2) | (t0v21 & 0xccccccccU));\
    t0v21 = ((t0v29 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v11 = (((t0v29 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v29 = ((t0v9 & 0x33333333U) | ((t0v13 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v13 & 0xccccccccU));\
    t0v13 = ((t0v1 & 0x33333333U) | ((t0v5 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v5 & 0xccccccccU));\
    t0v5 = ((t0v3 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v3 = (((t0v3 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    (D0) = ((t0v15 & 0x55555555U) | ((t0v31 & 0x55555555U) << 1));\
    (D1) = (((t0v15 & 0xaaaaaaaaU) >> 1) | (t0v31 & 0xaaaaaaaaU));\
    (D2) = ((t0v24 & 0x55555555U) | ((t0v26 & 0x55555555U) << 1));\
    (D3) = (((t0v24 & 0xaaaaaaaaU) >> 1) | (t0v26 & 0xaaaaaaaaU));\
    (D4) = ((t0v32 & 0x55555555U) | ((t0v22 & 0x55555555U) << 1));\
    (D5) = (((t0v32 & 0xaaaaaaaaU) >> 1) | (t0v22 & 0xaaaaaaaaU));\
    (D6) = ((t0v18 & 0x55555555U) | ((t0v16 & 0x55555555U) << 1));\
    (D7) = (((t0v18 & 0xaaaaaaaaU) >> 1) | (t0v16 & 0xaaaaaaaaU));\
    (D8) = ((t0v20 & 0x55555555U) | ((t0v28 & 0x55555555U) << 1));\
    (D9) = (((t0v20 & 0xaaaaaaaaU) >> 1) | (t0v28 & 0xaaaaaaaaU));\
    (D10) = ((t0v10 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D11) = (((t0v10 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D12) = ((t0v12 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D13) = (((t0v12 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D14) = ((t0v0 & 0x55555555U) | ((t0v2 & 0x55555555U) << 1));\
    (D15) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v2 & 0xaaaaaaaaU));\
    (D16) = ((t0v6 & 0x55555555U) | ((t0v25 & 0x55555555U) << 1));\
    (D17) = (((t0v6 & 0xaaaaaaaaU) >> 1) | (t0v25 & 0xaaaaaaaaU));\
    (D18) = ((t0v14 & 0x55555555U) | ((t0v27 & 0x55555555U) << 1));\
    (D19) = (((t0v14 & 0xaaaaaaaaU) >> 1) | (t0v27 & 0xaaaaaaaaU));\
    (D20) = ((t0v30 & 0x55555555U) | ((t0v23 & 0x55555555U) << 1));\
    (D21) = (((t0v30 & 0xaaaaaaaaU) >> 1) | (t0v23 & 0xaaaaaaaaU));\
    (D22) = ((t0v19 & 0x55555555U) | ((t0v17 & 0x55555555U) << 1));\
    (D23) = (((t0v19 & 0xaaaaaaaaU) >> 1) | (t0v17 & 0xaaaaaaaaU));\
    (D24) = ((t0v21 & 0x55555555U) | ((t0v29 & 0x55555555U) << 1));\
    (D25) = (((t0v21 & 0xaaaaaaaaU) >> 1) | (t0v29 & 0xaaaaaaaaU));\
    (D26) = ((t0v11 & 0x55555555U) | ((t0v9 & 0x55555555U) << 1));\
    (D27) = (((t0v11 & 0xaaaaaaaaU) >> 1) | (t0v9 & 0xaaaaaaaaU));\
    (D28) = ((t0v13 & 0x55555555U) | ((t0v5 & 0x55555555U) << 1));\
    (D29) = (((t0v13 & 0xaaaaaaaaU) >> 1) | (t0v5 & 0xaaaaaaaaU));\
    (D30) = ((t0v1 & 0x55555555U) | ((t0v3 & 0x55555555U) << 1));\
    (D31) = (((t0v1 & 0xaaaaaaaaU) >> 1) | (t0v3 & 0xaaaaaaaaU));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(26);
    assert_eq!(
        r##"#define IN_TRANSFORM_B26(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    uint32_t t0v9;\
    uint32_t t0v10;\
    uint32_t t0v11;\
    uint32_t t0v12;\
    uint32_t t0v13;\
    uint32_t t0v14;\
    uint32_t t0v15;\
    uint32_t t0v16;\
    uint32_t t0v17;\
    uint32_t t0v18;\
    uint32_t t0v19;\
    uint32_t t0v20;\
    uint32_t t0v21;\
    uint32_t t0v22;\
    uint32_t t0v23;\
    uint32_t t0v24;\
    uint32_t t0v25;\
    uint32_t t0v26;\
    uint32_t t0v27;\
    uint32_t t0v28;\
    uint32_t t0v29;\
    uint32_t t0v30;\
    uint32_t t0v31;\
    uint32_t t0v32;\
    t0v0 = ((((S)[0]) & 0x0000ffffU) | ((((S)[16]) & 0x0000ffffU) << 16));\
    t0v1 = (((((S)[0]) & 0xffff0000U) >> 16) | (((S)[16]) & 0xffff0000U));\
    t0v2 = ((((S)[1]) & 0x0000ffffU) | ((((S)[17]) & 0x0000ffffU) << 16));\
    t0v3 = (((((S)[1]) & 0xffff0000U) >> 16) | (((S)[17]) & 0xffff0000U));\
    t0v4 = ((((S)[2]) & 0x0000ffffU) | ((((S)[18]) & 0x0000ffffU) << 16));\
    t0v5 = (((((S)[2]) & 0xffff0000U) >> 16) | (((S)[18]) & 0xffff0000U));\
    t0v6 = ((((S)[3]) & 0x0000ffffU) | ((((S)[19]) & 0x0000ffffU) << 16));\
    t0v7 = (((((S)[3]) & 0xffff0000U) >> 16) | (((S)[19]) & 0xffff0000U));\
    t0v8 = ((((S)[4]) & 0x0000ffffU) | ((((S)[20]) & 0x0000ffffU) << 16));\
    t0v9 = (((((S)[4]) & 0xffff0000U) >> 16) | (((S)[20]) & 0xffff0000U));\
    t0v10 = ((((S)[5]) & 0x0000ffffU) | ((((S)[21]) & 0x0000ffffU) << 16));\
    t0v11 = (((((S)[5]) & 0xffff0000U) >> 16) | (((S)[21]) & 0xffff0000U));\
    t0v12 = ((((S)[6]) & 0x0000ffffU) | ((((S)[22]) & 0x0000ffffU) << 16));\
    t0v13 = (((((S)[6]) & 0xffff0000U) >> 16) | (((S)[22]) & 0xffff0000U));\
    t0v14 = ((((S)[7]) & 0x0000ffffU) | ((((S)[23]) & 0x0000ffffU) << 16));\
    t0v15 = (((((S)[7]) & 0xffff0000U) >> 16) | (((S)[23]) & 0xffff0000U));\
    t0v16 = ((((S)[8]) & 0x0000ffffU) | ((((S)[24]) & 0x0000ffffU) << 16));\
    t0v17 = (((((S)[8]) & 0xffff0000U) >> 16) | (((S)[24]) & 0xffff0000U));\
    t0v18 = ((((S)[9]) & 0x0000ffffU) | ((((S)[25]) & 0x0000ffffU) << 16));\
    t0v19 = (((((S)[9]) & 0xffff0000U) >> 16) | (((S)[25]) & 0xffff0000U));\
    t0v20 = ((((S)[10]) & 0x0000ffffU) | ((((S)[26]) & 0x0000ffffU) << 16));\
    t0v21 = (((((S)[10]) & 0xffff0000U) >> 16) | (((S)[26]) & 0xffff0000U));\
    t0v22 = ((((S)[11]) & 0x0000ffffU) | ((((S)[27]) & 0x0000ffffU) << 16));\
    t0v23 = (((((S)[11]) & 0xffff0000U) >> 16) | (((S)[27]) & 0xffff0000U));\
    t0v24 = ((((S)[12]) & 0x0000ffffU) | ((((S)[28]) & 0x0000ffffU) << 16));\
    t0v25 = (((((S)[12]) & 0xffff0000U) >> 16) | (((S)[28]) & 0xffff0000U));\
    t0v26 = ((((S)[13]) & 0x0000ffffU) | ((((S)[29]) & 0x0000ffffU) << 16));\
    t0v27 = (((((S)[13]) & 0xffff0000U) >> 16) | (((S)[29]) & 0xffff0000U));\
    t0v28 = ((((S)[14]) & 0x0000ffffU) | ((((S)[30]) & 0x0000ffffU) << 16));\
    t0v29 = (((((S)[14]) & 0xffff0000U) >> 16) | (((S)[30]) & 0xffff0000U));\
    t0v30 = ((((S)[15]) & 0x0000ffffU) | ((((S)[31]) & 0x0000ffffU) << 16));\
    t0v31 = (((((S)[15]) & 0xffff0000U) >> 16) | (((S)[31]) & 0xffff0000U));\
    t0v32 = ((t0v0 & 0x00ff00ffU) | ((t0v16 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v16 & 0xff00ff00U));\
    t0v16 = ((t0v2 & 0x00ff00ffU) | ((t0v18 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v18 & 0xff00ff00U));\
    t0v18 = ((t0v4 & 0x00ff00ffU) | ((t0v20 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v20 & 0xff00ff00U));\
    t0v20 = ((t0v6 & 0x00ff00ffU) | ((t0v22 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v22 & 0xff00ff00U));\
    t0v22 = ((t0v8 & 0x00ff00ffU) | ((t0v24 & 0x00ff00ffU) << 8));\
    t0v8 = (((t0v8 & 0xff00ff00U) >> 8) | (t0v24 & 0xff00ff00U));\
    t0v24 = ((t0v10 & 0x00ff00ffU) | ((t0v26 & 0x00ff00ffU) << 8));\
    t0v10 = (((t0v10 & 0xff00ff00U) >> 8) | (t0v26 & 0xff00ff00U));\
    t0v26 = ((t0v12 & 0x00ff00ffU) | ((t0v28 & 0x00ff00ffU) << 8));\
    t0v12 = (((t0v12 & 0xff00ff00U) >> 8) | (t0v28 & 0xff00ff00U));\
    t0v28 = ((t0v14 & 0x00ff00ffU) | ((t0v30 & 0x00ff00ffU) << 8));\
    t0v14 = (((t0v14 & 0xff00ff00U) >> 8) | (t0v30 & 0xff00ff00U));\
    t0v30 = ((t0v1 & 0x00ff00ffU) | ((t0v17 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v17 & 0xff00ff00U));\
    t0v17 = ((t0v3 & 0x00ff00ffU) | ((t0v19 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v19 & 0xff00ff00U));\
    t0v19 = ((t0v5 & 0x00ff00ffU) | ((t0v21 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v5 & 0xff00ff00U) >> 8) | (t0v21 & 0xff00ff00U));\
    t0v21 = ((t0v7 & 0x00ff00ffU) | ((t0v23 & 0x00ff00ffU) << 8));\
    t0v7 = (((t0v7 & 0xff00ff00U) >> 8) | (t0v23 & 0xff00ff00U));\
    t0v23 = ((t0v9 & 0x00ff00ffU) | ((t0v25 & 0x00ff00ffU) << 8));\
    t0v9 = (((t0v9 & 0xff00ff00U) >> 8) | (t0v25 & 0xff00ff00U));\
    t0v25 = ((t0v11 & 0x00ff00ffU) | ((t0v27 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v11 & 0xff00ff00U) >> 8) | (t0v27 & 0xff00ff00U));\
    t0v27 = ((t0v13 & 0x00ff00ffU) | ((t0v29 & 0x00ff00ffU) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00U) >> 8) | (t0v29 & 0xff00ff00U));\
    t0v29 = ((t0v15 & 0x00ff00ffU) | ((t0v31 & 0x00ff00ffU) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v31 & 0xff00ff00U));\
    t0v31 = ((t0v32 & 0x0f0f0f0fU) | ((t0v22 & 0x0f0f0f0fU) << 4));\
    t0v22 = (((t0v32 & 0xf0f0f0f0U) >> 4) | (t0v22 & 0xf0f0f0f0U));\
    t0v32 = ((t0v16 & 0x0f0f0f0fU) | ((t0v24 & 0x0f0f0f0fU) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v24 & 0xf0f0f0f0U));\
    t0v24 = ((t0v18 & 0x0f0f0f0fU) | ((t0v26 & 0x0f0f0f0fU) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0U) >> 4) | (t0v26 & 0xf0f0f0f0U));\
    t0v26 = ((t0v20 & 0x0f0f0f0fU) | ((t0v28 & 0x0f0f0f0fU) << 4));\
    t0v20 = (((t0v20 & 0xf0f0f0f0U) >> 4) | (t0v28 & 0xf0f0f0f0U));\
    t0v28 = ((t0v0 & 0x0f0f0f0fU) | ((t0v8 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v8 & 0xf0f0f0f0U));\
    t0v8 = ((t0v2 & 0x0f0f0f0fU) | ((t0v10 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v10 & 0xf0f0f0f0U));\
    t0v10 = ((t0v4 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v4 = (((t0v4 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v6 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v6 = (((t0v6 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v30 & 0x0f0f0f0fU) | ((t0v23 & 0x0f0f0f0fU) << 4));\
    t0v23 = (((t0v30 & 0xf0f0f0f0U) >> 4) | (t0v23 & 0xf0f0f0f0U));\
    t0v30 = ((t0v17 & 0x0f0f0f0fU) | ((t0v25 & 0x0f0f0f0fU) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0U) >> 4) | (t0v25 & 0xf0f0f0f0U));\
    t0v25 = ((t0v19 & 0x0f0f0f0fU) | ((t0v27 & 0x0f0f0f0fU) << 4));\
    t0v19 = (((t0v19 & 0xf0f0f0f0U) >> 4) | (t0v27 & 0xf0f0f0f0U));\
    t0v27 = ((t0v21 & 0x0f0f0f0fU) | ((t0v29 & 0x0f0f0f0fU) << 4));\
    t0v21 = (((t0v21 & 0xf0f0f0f0U) >> 4) | (t0v29 & 0xf0f0f0f0U));\
    t0v1 = ((t0v1 & 0x0f0f0f0fU) | ((t0v9 & 0x0f0f0f0fU) << 4));\
    t0v3 = ((t0v3 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v5 = ((t0v5 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v7 = ((t0v7 & 0x0f0f0f0fU) | ((t0v15 & 0x0f0f0f0fU) << 4));\
    t0v9 = ((t0v31 & 0x33333333U) | ((t0v24 & 0x33333333U) << 2));\
    t0v11 = (((t0v31 & 0xccccccccU) >> 2) | (t0v24 & 0xccccccccU));\
    t0v13 = ((t0v32 & 0x33333333U) | ((t0v26 & 0x33333333U) << 2));\
    t0v15 = (((t0v32 & 0xccccccccU) >> 2) | (t0v26 & 0xccccccccU));\
    t0v24 = ((t0v22 & 0x33333333U) | ((t0v18 & 0x33333333U) << 2));\
    t0v18 = (((t0v22 & 0xccccccccU) >> 2) | (t0v18 & 0xccccccccU));\
    t0v22 = ((t0v16 & 0x33333333U) | ((t0v20 & 0x33333333U) << 2));\
    t0v16 = (((t0v16 & 0xccccccccU) >> 2) | (t0v20 & 0xccccccccU));\
    t0v20 = ((t0v28 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v10 = (((t0v28 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v26 = ((t0v8 & 0x33333333U) | ((t0v12 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v12 & 0xccccccccU));\
    t0v12 = ((t0v0 & 0x33333333U) | ((t0v4 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v4 & 0xccccccccU));\
    t0v4 = ((t0v2 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v2 = (((t0v2 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v14 & 0x33333333U) | ((t0v25 & 0x33333333U) << 2));\
    t0v14 = (((t0v14 & 0xccccccccU) >> 2) | (t0v25 & 0xccccccccU));\
    t0v25 = ((t0v30 & 0x33333333U) | ((t0v27 & 0x33333333U) << 2));\
    t0v27 = (((t0v30 & 0xccccccccU) >> 2) | (t0v27 & 0xccccccccU));\
    t0v28 = ((t0v23 & 0x33333333U) | ((t0v19 & 0x33333333U) << 2));\
    t0v19 = (((t0v23 & 0xccccccccU) >> 2) | (t0v19 & 0xccccccccU));\
    t0v23 = ((t0v17 & 0x33333333U) | ((t0v21 & 0x33333333U) << 2));\
    t0v17 = (((t0v17 & 0xccccccccU) >> 2) | (t0v21 & 0xccccccccU));\
    t0v1 = ((t0v1 & 0x33333333U) | ((t0v5 & 0x33333333U) << 2));\
    t0v3 = ((t0v3 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    (D0) = ((t0v9 & 0x55555555U) | ((t0v13 & 0x55555555U) << 1));\
    (D1) = (((t0v9 & 0xaaaaaaaaU) >> 1) | (t0v13 & 0xaaaaaaaaU));\
    (D2) = ((t0v11 & 0x55555555U) | ((t0v15 & 0x55555555U) << 1));\
    (D3) = (((t0v11 & 0xaaaaaaaaU) >> 1) | (t0v15 & 0xaaaaaaaaU));\
    (D4) = ((t0v24 & 0x55555555U) | ((t0v22 & 0x55555555U) << 1));\
    (D5) = (((t0v24 & 0xaaaaaaaaU) >> 1) | (t0v22 & 0xaaaaaaaaU));\
    (D6) = ((t0v18 & 0x55555555U) | ((t0v16 & 0x55555555U) << 1));\
    (D7) = (((t0v18 & 0xaaaaaaaaU) >> 1) | (t0v16 & 0xaaaaaaaaU));\
    (D8) = ((t0v20 & 0x55555555U) | ((t0v26 & 0x55555555U) << 1));\
    (D9) = (((t0v20 & 0xaaaaaaaaU) >> 1) | (t0v26 & 0xaaaaaaaaU));\
    (D10) = ((t0v10 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D11) = (((t0v10 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D12) = ((t0v12 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D13) = (((t0v12 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D14) = ((t0v0 & 0x55555555U) | ((t0v2 & 0x55555555U) << 1));\
    (D15) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v2 & 0xaaaaaaaaU));\
    (D16) = ((t0v6 & 0x55555555U) | ((t0v25 & 0x55555555U) << 1));\
    (D17) = (((t0v6 & 0xaaaaaaaaU) >> 1) | (t0v25 & 0xaaaaaaaaU));\
    (D18) = ((t0v14 & 0x55555555U) | ((t0v27 & 0x55555555U) << 1));\
    (D19) = (((t0v14 & 0xaaaaaaaaU) >> 1) | (t0v27 & 0xaaaaaaaaU));\
    (D20) = ((t0v28 & 0x55555555U) | ((t0v23 & 0x55555555U) << 1));\
    (D21) = (((t0v28 & 0xaaaaaaaaU) >> 1) | (t0v23 & 0xaaaaaaaaU));\
    (D22) = ((t0v19 & 0x55555555U) | ((t0v17 & 0x55555555U) << 1));\
    (D23) = (((t0v19 & 0xaaaaaaaaU) >> 1) | (t0v17 & 0xaaaaaaaaU));\
    (D24) = ((t0v1 & 0x55555555U) | ((t0v3 & 0x55555555U) << 1));\
    (D25) = (((t0v1 & 0xaaaaaaaaU) >> 1) | (t0v3 & 0xaaaaaaaaU));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(16);
    assert_eq!(
        r##"#define IN_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    uint32_t t0v9;\
    uint32_t t0v10;\
    uint32_t t0v11;\
    uint32_t t0v12;\
    uint32_t t0v13;\
    uint32_t t0v14;\
    uint32_t t0v15;\
    uint32_t t0v16;\
    t0v0 = ((((S)[0]) & 0xffffU) | (((S)[16]) << 16));\
    t0v1 = ((((S)[1]) & 0xffffU) | (((S)[17]) << 16));\
    t0v2 = ((((S)[2]) & 0xffffU) | (((S)[18]) << 16));\
    t0v3 = ((((S)[3]) & 0xffffU) | (((S)[19]) << 16));\
    t0v4 = ((((S)[4]) & 0xffffU) | (((S)[20]) << 16));\
    t0v5 = ((((S)[5]) & 0xffffU) | (((S)[21]) << 16));\
    t0v6 = ((((S)[6]) & 0xffffU) | (((S)[22]) << 16));\
    t0v7 = ((((S)[7]) & 0xffffU) | (((S)[23]) << 16));\
    t0v8 = ((((S)[8]) & 0xffffU) | (((S)[24]) << 16));\
    t0v9 = ((((S)[9]) & 0xffffU) | (((S)[25]) << 16));\
    t0v10 = ((((S)[10]) & 0xffffU) | (((S)[26]) << 16));\
    t0v11 = ((((S)[11]) & 0xffffU) | (((S)[27]) << 16));\
    t0v12 = ((((S)[12]) & 0xffffU) | (((S)[28]) << 16));\
    t0v13 = ((((S)[13]) & 0xffffU) | (((S)[29]) << 16));\
    t0v14 = ((((S)[14]) & 0xffffU) | (((S)[30]) << 16));\
    t0v15 = ((((S)[15]) & 0xffffU) | (((S)[31]) << 16));\
    t0v16 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    t0v9 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v3 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v11 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v5 & 0x00ff00ffU) | ((t0v13 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v5 & 0xff00ff00U) >> 8) | (t0v13 & 0xff00ff00U));\
    t0v13 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v7 & 0x00ff00ffU) | ((t0v15 & 0x00ff00ffU) << 8));\
    t0v7 = (((t0v7 & 0xff00ff00U) >> 8) | (t0v15 & 0xff00ff00U));\
    t0v15 = ((t0v16 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v11 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v16 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v3 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v15 & 0x33333333U) | ((t0v12 & 0x33333333U) << 2));\
    t0v12 = (((t0v15 & 0xccccccccU) >> 2) | (t0v12 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x33333333U) | ((t0v13 & 0x33333333U) << 2));\
    t0v13 = (((t0v16 & 0xccccccccU) >> 2) | (t0v13 & 0xccccccccU));\
    t0v16 = ((t0v11 & 0x33333333U) | ((t0v9 & 0x33333333U) << 2));\
    t0v9 = (((t0v11 & 0xccccccccU) >> 2) | (t0v9 & 0xccccccccU));\
    t0v11 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v14 & 0x33333333U) | ((t0v5 & 0x33333333U) << 2));\
    t0v5 = (((t0v14 & 0xccccccccU) >> 2) | (t0v5 & 0xccccccccU));\
    t0v14 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    (D0) = ((t0v7 & 0x55555555U) | ((t0v15 & 0x55555555U) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaU) >> 1) | (t0v15 & 0xaaaaaaaaU));\
    (D2) = ((t0v12 & 0x55555555U) | ((t0v13 & 0x55555555U) << 1));\
    (D3) = (((t0v12 & 0xaaaaaaaaU) >> 1) | (t0v13 & 0xaaaaaaaaU));\
    (D4) = ((t0v16 & 0x55555555U) | ((t0v11 & 0x55555555U) << 1));\
    (D5) = (((t0v16 & 0xaaaaaaaaU) >> 1) | (t0v11 & 0xaaaaaaaaU));\
    (D6) = ((t0v9 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D7) = (((t0v9 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D8) = ((t0v10 & 0x55555555U) | ((t0v14 & 0x55555555U) << 1));\
    (D9) = (((t0v10 & 0xaaaaaaaaU) >> 1) | (t0v14 & 0xaaaaaaaaU));\
    (D10) = ((t0v5 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D11) = (((t0v5 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D12) = ((t0v6 & 0x55555555U) | ((t0v2 & 0x55555555U) << 1));\
    (D13) = (((t0v6 & 0xaaaaaaaaU) >> 1) | (t0v2 & 0xaaaaaaaaU));\
    (D14) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
    (D15) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v1 & 0xaaaaaaaaU));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(13);
    assert_eq!(
        r##"#define IN_TRANSFORM_B13(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    uint32_t t0v9;\
    uint32_t t0v10;\
    uint32_t t0v11;\
    uint32_t t0v12;\
    uint32_t t0v13;\
    uint32_t t0v14;\
    uint32_t t0v15;\
    uint32_t t0v16;\
    t0v0 = ((((S)[0]) & 0xffffU) | (((S)[16]) << 16));\
    t0v1 = ((((S)[1]) & 0xffffU) | (((S)[17]) << 16));\
    t0v2 = ((((S)[2]) & 0xffffU) | (((S)[18]) << 16));\
    t0v3 = ((((S)[3]) & 0xffffU) | (((S)[19]) << 16));\
    t0v4 = ((((S)[4]) & 0xffffU) | (((S)[20]) << 16));\
    t0v5 = ((((S)[5]) & 0xffffU) | (((S)[21]) << 16));\
    t0v6 = ((((S)[6]) & 0xffffU) | (((S)[22]) << 16));\
    t0v7 = ((((S)[7]) & 0xffffU) | (((S)[23]) << 16));\
    t0v8 = ((((S)[8]) & 0xffffU) | (((S)[24]) << 16));\
    t0v9 = ((((S)[9]) & 0xffffU) | (((S)[25]) << 16));\
    t0v10 = ((((S)[10]) & 0xffffU) | (((S)[26]) << 16));\
    t0v11 = ((((S)[11]) & 0xffffU) | (((S)[27]) << 16));\
    t0v12 = ((((S)[12]) & 0xffffU) | (((S)[28]) << 16));\
    t0v13 = ((((S)[13]) & 0xffffU) | (((S)[29]) << 16));\
    t0v14 = ((((S)[14]) & 0xffffU) | (((S)[30]) << 16));\
    t0v15 = ((((S)[15]) & 0xffffU) | (((S)[31]) << 16));\
    t0v16 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    t0v9 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v3 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v11 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v5 & 0x00ff00ffU) | ((t0v13 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v5 & 0xff00ff00U) >> 8) | (t0v13 & 0xff00ff00U));\
    t0v13 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v7 & 0x00ff00ffU) | ((t0v15 & 0x00ff00ffU) << 8));\
    t0v7 = (((t0v7 & 0xff00ff00U) >> 8) | (t0v15 & 0xff00ff00U));\
    t0v15 = ((t0v16 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v11 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v16 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v3 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v15 & 0x33333333U) | ((t0v12 & 0x33333333U) << 2));\
    t0v12 = (((t0v15 & 0xccccccccU) >> 2) | (t0v12 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x33333333U) | ((t0v13 & 0x33333333U) << 2));\
    t0v13 = (((t0v16 & 0xccccccccU) >> 2) | (t0v13 & 0xccccccccU));\
    t0v16 = ((t0v11 & 0x33333333U) | ((t0v9 & 0x33333333U) << 2));\
    t0v9 = (((t0v11 & 0xccccccccU) >> 2) | (t0v9 & 0xccccccccU));\
    t0v11 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v14 & 0x33333333U) | ((t0v5 & 0x33333333U) << 2));\
    t0v5 = (((t0v14 & 0xccccccccU) >> 2) | (t0v5 & 0xccccccccU));\
    t0v14 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v0 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v1 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    (D0) = ((t0v7 & 0x55555555U) | ((t0v15 & 0x55555555U) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaU) >> 1) | (t0v15 & 0xaaaaaaaaU));\
    (D2) = ((t0v12 & 0x55555555U) | ((t0v13 & 0x55555555U) << 1));\
    (D3) = (((t0v12 & 0xaaaaaaaaU) >> 1) | (t0v13 & 0xaaaaaaaaU));\
    (D4) = ((t0v16 & 0x55555555U) | ((t0v11 & 0x55555555U) << 1));\
    (D5) = (((t0v16 & 0xaaaaaaaaU) >> 1) | (t0v11 & 0xaaaaaaaaU));\
    (D6) = ((t0v9 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D7) = (((t0v9 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D8) = ((t0v10 & 0x55555555U) | ((t0v14 & 0x55555555U) << 1));\
    (D9) = (((t0v10 & 0xaaaaaaaaU) >> 1) | (t0v14 & 0xaaaaaaaaU));\
    (D10) = ((t0v5 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D11) = (((t0v5 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D12) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(8);
    assert_eq!(
        r##"#define IN_TRANSFORM_B8(D0, D1, D2, D3, D4, D5, D6, D7, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    t0v0 = ((((((S)[0]) & 0xffU) | ((((S)[8]) & 0xffU) << 8)) | ((((S)[16]) & 0xffU) << 16)) | (((S)[24]) << 24));\
    t0v1 = ((((((S)[1]) & 0xffU) | ((((S)[9]) & 0xffU) << 8)) | ((((S)[17]) & 0xffU) << 16)) | (((S)[25]) << 24));\
    t0v2 = ((((((S)[2]) & 0xffU) | ((((S)[10]) & 0xffU) << 8)) | ((((S)[18]) & 0xffU) << 16)) | (((S)[26]) << 24));\
    t0v3 = ((((((S)[3]) & 0xffU) | ((((S)[11]) & 0xffU) << 8)) | ((((S)[19]) & 0xffU) << 16)) | (((S)[27]) << 24));\
    t0v4 = ((((((S)[4]) & 0xffU) | ((((S)[12]) & 0xffU) << 8)) | ((((S)[20]) & 0xffU) << 16)) | (((S)[28]) << 24));\
    t0v5 = ((((((S)[5]) & 0xffU) | ((((S)[13]) & 0xffU) << 8)) | ((((S)[21]) & 0xffU) << 16)) | (((S)[29]) << 24));\
    t0v6 = ((((((S)[6]) & 0xffU) | ((((S)[14]) & 0xffU) << 8)) | ((((S)[22]) & 0xffU) << 16)) | (((S)[30]) << 24));\
    t0v7 = ((((((S)[7]) & 0xffU) | ((((S)[15]) & 0xffU) << 8)) | ((((S)[23]) & 0xffU) << 16)) | (((S)[31]) << 24));\
    t0v8 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v3 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v5 & 0x33333333U) << 2));\
    t0v5 = (((t0v8 & 0xccccccccU) >> 2) | (t0v5 & 0xccccccccU));\
    t0v8 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    (D0) = ((t0v7 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D2) = ((t0v5 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D3) = (((t0v5 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D4) = ((t0v6 & 0x55555555U) | ((t0v2 & 0x55555555U) << 1));\
    (D5) = (((t0v6 & 0xaaaaaaaaU) >> 1) | (t0v2 & 0xaaaaaaaaU));\
    (D6) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
    (D7) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v1 & 0xaaaaaaaaU));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(3);
    assert_eq!(
        r##"#define IN_TRANSFORM_B3(D0, D1, D2, S) \
{\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    t0v0 = ((((((((((S)[0]) & 0xfU) | ((((S)[4]) & 0xfU) << 4)) | ((((S)[8]) & 0xfU) << 8)) | ((((S)[12]) & 0xfU) << 12)) | ((((S)[16]) & 0xfU) << 16)) | ((((S)[20]) & 0xfU) << 20)) | ((((S)[24]) & 0xfU) << 24)) | (((S)[28]) << 28));\
    t0v1 = ((((((((((S)[1]) & 0xfU) | ((((S)[5]) & 0xfU) << 4)) | ((((S)[9]) & 0xfU) << 8)) | ((((S)[13]) & 0xfU) << 12)) | ((((S)[17]) & 0xfU) << 16)) | ((((S)[21]) & 0xfU) << 20)) | ((((S)[25]) & 0xfU) << 24)) | (((S)[29]) << 28));\
    t0v2 = ((((((((((S)[2]) & 0xfU) | ((((S)[6]) & 0xfU) << 4)) | ((((S)[10]) & 0xfU) << 8)) | ((((S)[14]) & 0xfU) << 12)) | ((((S)[18]) & 0xfU) << 16)) | ((((S)[22]) & 0xfU) << 20)) | ((((S)[26]) & 0xfU) << 24)) | (((S)[30]) << 28));\
    t0v3 = ((((((((((S)[3]) & 0xfU) | ((((S)[7]) & 0xfU) << 4)) | ((((S)[11]) & 0xfU) << 8)) | ((((S)[15]) & 0xfU) << 12)) | ((((S)[19]) & 0xfU) << 16)) | ((((S)[23]) & 0xfU) << 20)) | ((((S)[27]) & 0xfU) << 24)) | (((S)[31]) << 28));\
    t0v4 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    (D0) = ((t0v4 & 0x55555555U) | ((t0v2 & 0x55555555U) << 1));\
    (D1) = (((t0v4 & 0xaaaaaaaaU) >> 1) | (t0v2 & 0xaaaaaaaaU));\
    (D2) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define IN_TRANSFORM_B1(D0, S) \
{\
    (D0) = ((((((((((((((((((((((((((((((((((S)[0]) & 0x1U) | ((((S)[1]) & 0x1U) << 1)) | ((((S)[2]) & 0x1U) << 2)) | ((((S)[3]) & 0x1U) << 3)) | ((((S)[4]) & 0x1U) << 4)) | ((((S)[5]) & 0x1U) << 5)) | ((((S)[6]) & 0x1U) << 6)) | ((((S)[7]) & 0x1U) << 7)) | ((((S)[8]) & 0x1U) << 8)) | ((((S)[9]) & 0x1U) << 9)) | ((((S)[10]) & 0x1U) << 10)) | ((((S)[11]) & 0x1U) << 11)) | ((((S)[12]) & 0x1U) << 12)) | ((((S)[13]) & 0x1U) << 13)) | ((((S)[14]) & 0x1U) << 14)) | ((((S)[15]) & 0x1U) << 15)) | ((((S)[16]) & 0x1U) << 16)) | ((((S)[17]) & 0x1U) << 17)) | ((((S)[18]) & 0x1U) << 18)) | ((((S)[19]) & 0x1U) << 19)) | ((((S)[20]) & 0x1U) << 20)) | ((((S)[21]) & 0x1U) << 21)) | ((((S)[22]) & 0x1U) << 22)) | ((((S)[23]) & 0x1U) << 23)) | ((((S)[24]) & 0x1U) << 24)) | ((((S)[25]) & 0x1U) << 25)) | ((((S)[26]) & 0x1U) << 26)) | ((((S)[27]) & 0x1U) << 27)) | ((((S)[28]) & 0x1U) << 28)) | ((((S)[29]) & 0x1U) << 29)) | ((((S)[30]) & 0x1U) << 30)) | (((S)[31]) << 31));\
}
"##,
        transform.out()
    );

    let mut transform = CLANG_TRANSFORM_U64.transform();
    transform.gen_input_transform(32);
    assert_eq!(
        r##"#define IN_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    uint64_t t0v0;\
    uint64_t t0v1;\
    uint64_t t0v2;\
    uint64_t t0v3;\
    uint64_t t0v4;\
    uint64_t t0v5;\
    uint64_t t0v6;\
    uint64_t t0v7;\
    uint64_t t0v8;\
    uint64_t t0v9;\
    uint64_t t0v10;\
    uint64_t t0v11;\
    uint64_t t0v12;\
    uint64_t t0v13;\
    uint64_t t0v14;\
    uint64_t t0v15;\
    uint64_t t0v16;\
    uint64_t t0v17;\
    uint64_t t0v18;\
    uint64_t t0v19;\
    uint64_t t0v20;\
    uint64_t t0v21;\
    uint64_t t0v22;\
    uint64_t t0v23;\
    uint64_t t0v24;\
    uint64_t t0v25;\
    uint64_t t0v26;\
    uint64_t t0v27;\
    uint64_t t0v28;\
    uint64_t t0v29;\
    uint64_t t0v30;\
    uint64_t t0v31;\
    uint64_t t0v32;\
    t0v0 = ((*((const uint64_t*)(((S) + 0)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 32)))) & 0xffffffffULL) << 32);\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL);\
    t0v2 = ((*((const uint64_t*)(((S) + 2)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 34)))) & 0xffffffffULL) << 32);\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL);\
    t0v4 = ((*((const uint64_t*)(((S) + 4)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 36)))) & 0xffffffffULL) << 32);\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL);\
    t0v6 = ((*((const uint64_t*)(((S) + 6)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 38)))) & 0xffffffffULL) << 32);\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL);\
    t0v8 = ((*((const uint64_t*)(((S) + 8)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 40)))) & 0xffffffffULL) << 32);\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL);\
    t0v10 = ((*((const uint64_t*)(((S) + 10)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 42)))) & 0xffffffffULL) << 32);\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL);\
    t0v12 = ((*((const uint64_t*)(((S) + 12)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 44)))) & 0xffffffffULL) << 32);\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL);\
    t0v14 = ((*((const uint64_t*)(((S) + 14)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 46)))) & 0xffffffffULL) << 32);\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL);\
    t0v16 = ((*((const uint64_t*)(((S) + 16)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 48)))) & 0xffffffffULL) << 32);\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL);\
    t0v18 = ((*((const uint64_t*)(((S) + 18)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 50)))) & 0xffffffffULL) << 32);\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL);\
    t0v20 = ((*((const uint64_t*)(((S) + 20)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 52)))) & 0xffffffffULL) << 32);\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL);\
    t0v22 = ((*((const uint64_t*)(((S) + 22)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 54)))) & 0xffffffffULL) << 32);\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL);\
    t0v24 = ((*((const uint64_t*)(((S) + 24)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 56)))) & 0xffffffffULL) << 32);\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL);\
    t0v26 = ((*((const uint64_t*)(((S) + 26)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 58)))) & 0xffffffffULL) << 32);\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL);\
    t0v28 = ((*((const uint64_t*)(((S) + 28)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 60)))) & 0xffffffffULL) << 32);\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL);\
    t0v30 = ((*((const uint64_t*)(((S) + 30)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 62)))) & 0xffffffffULL) << 32);\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL);\
    t0v32 = ((t0v0 & 0x0000ffff0000ffffULL) | ((t0v16 & 0x0000ffff0000ffffULL) << 16));\
    t0v0 = (((t0v0 & 0xffff0000ffff0000ULL) >> 16) | (t0v16 & 0xffff0000ffff0000ULL));\
    t0v16 = ((t0v1 & 0x0000ffff0000ffffULL) | ((t0v17 & 0x0000ffff0000ffffULL) << 16));\
    t0v1 = (((t0v1 & 0xffff0000ffff0000ULL) >> 16) | (t0v17 & 0xffff0000ffff0000ULL));\
    t0v17 = ((t0v2 & 0x0000ffff0000ffffULL) | ((t0v18 & 0x0000ffff0000ffffULL) << 16));\
    t0v2 = (((t0v2 & 0xffff0000ffff0000ULL) >> 16) | (t0v18 & 0xffff0000ffff0000ULL));\
    t0v18 = ((t0v3 & 0x0000ffff0000ffffULL) | ((t0v19 & 0x0000ffff0000ffffULL) << 16));\
    t0v3 = (((t0v3 & 0xffff0000ffff0000ULL) >> 16) | (t0v19 & 0xffff0000ffff0000ULL));\
    t0v19 = ((t0v4 & 0x0000ffff0000ffffULL) | ((t0v20 & 0x0000ffff0000ffffULL) << 16));\
    t0v4 = (((t0v4 & 0xffff0000ffff0000ULL) >> 16) | (t0v20 & 0xffff0000ffff0000ULL));\
    t0v20 = ((t0v5 & 0x0000ffff0000ffffULL) | ((t0v21 & 0x0000ffff0000ffffULL) << 16));\
    t0v5 = (((t0v5 & 0xffff0000ffff0000ULL) >> 16) | (t0v21 & 0xffff0000ffff0000ULL));\
    t0v21 = ((t0v6 & 0x0000ffff0000ffffULL) | ((t0v22 & 0x0000ffff0000ffffULL) << 16));\
    t0v6 = (((t0v6 & 0xffff0000ffff0000ULL) >> 16) | (t0v22 & 0xffff0000ffff0000ULL));\
    t0v22 = ((t0v7 & 0x0000ffff0000ffffULL) | ((t0v23 & 0x0000ffff0000ffffULL) << 16));\
    t0v7 = (((t0v7 & 0xffff0000ffff0000ULL) >> 16) | (t0v23 & 0xffff0000ffff0000ULL));\
    t0v23 = ((t0v8 & 0x0000ffff0000ffffULL) | ((t0v24 & 0x0000ffff0000ffffULL) << 16));\
    t0v8 = (((t0v8 & 0xffff0000ffff0000ULL) >> 16) | (t0v24 & 0xffff0000ffff0000ULL));\
    t0v24 = ((t0v9 & 0x0000ffff0000ffffULL) | ((t0v25 & 0x0000ffff0000ffffULL) << 16));\
    t0v9 = (((t0v9 & 0xffff0000ffff0000ULL) >> 16) | (t0v25 & 0xffff0000ffff0000ULL));\
    t0v25 = ((t0v10 & 0x0000ffff0000ffffULL) | ((t0v26 & 0x0000ffff0000ffffULL) << 16));\
    t0v10 = (((t0v10 & 0xffff0000ffff0000ULL) >> 16) | (t0v26 & 0xffff0000ffff0000ULL));\
    t0v26 = ((t0v11 & 0x0000ffff0000ffffULL) | ((t0v27 & 0x0000ffff0000ffffULL) << 16));\
    t0v11 = (((t0v11 & 0xffff0000ffff0000ULL) >> 16) | (t0v27 & 0xffff0000ffff0000ULL));\
    t0v27 = ((t0v12 & 0x0000ffff0000ffffULL) | ((t0v28 & 0x0000ffff0000ffffULL) << 16));\
    t0v12 = (((t0v12 & 0xffff0000ffff0000ULL) >> 16) | (t0v28 & 0xffff0000ffff0000ULL));\
    t0v28 = ((t0v13 & 0x0000ffff0000ffffULL) | ((t0v29 & 0x0000ffff0000ffffULL) << 16));\
    t0v13 = (((t0v13 & 0xffff0000ffff0000ULL) >> 16) | (t0v29 & 0xffff0000ffff0000ULL));\
    t0v29 = ((t0v14 & 0x0000ffff0000ffffULL) | ((t0v30 & 0x0000ffff0000ffffULL) << 16));\
    t0v14 = (((t0v14 & 0xffff0000ffff0000ULL) >> 16) | (t0v30 & 0xffff0000ffff0000ULL));\
    t0v30 = ((t0v15 & 0x0000ffff0000ffffULL) | ((t0v31 & 0x0000ffff0000ffffULL) << 16));\
    t0v15 = (((t0v15 & 0xffff0000ffff0000ULL) >> 16) | (t0v31 & 0xffff0000ffff0000ULL));\
    t0v31 = ((t0v32 & 0x00ff00ff00ff00ffULL) | ((t0v23 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v23 = (((t0v32 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v23 & 0xff00ff00ff00ff00ULL));\
    t0v32 = ((t0v16 & 0x00ff00ff00ff00ffULL) | ((t0v24 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v24 & 0xff00ff00ff00ff00ULL));\
    t0v24 = ((t0v17 & 0x00ff00ff00ff00ffULL) | ((t0v25 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v25 & 0xff00ff00ff00ff00ULL));\
    t0v25 = ((t0v18 & 0x00ff00ff00ff00ffULL) | ((t0v26 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v26 & 0xff00ff00ff00ff00ULL));\
    t0v26 = ((t0v19 & 0x00ff00ff00ff00ffULL) | ((t0v27 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v27 & 0xff00ff00ff00ff00ULL));\
    t0v27 = ((t0v20 & 0x00ff00ff00ff00ffULL) | ((t0v28 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v28 & 0xff00ff00ff00ff00ULL));\
    t0v28 = ((t0v21 & 0x00ff00ff00ff00ffULL) | ((t0v29 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v21 = (((t0v21 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v29 & 0xff00ff00ff00ff00ULL));\
    t0v29 = ((t0v22 & 0x00ff00ff00ff00ffULL) | ((t0v30 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v30 & 0xff00ff00ff00ff00ULL));\
    t0v30 = ((t0v0 & 0x00ff00ff00ff00ffULL) | ((t0v8 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v8 & 0xff00ff00ff00ff00ULL));\
    t0v8 = ((t0v1 & 0x00ff00ff00ff00ffULL) | ((t0v9 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v9 & 0xff00ff00ff00ff00ULL));\
    t0v9 = ((t0v2 & 0x00ff00ff00ff00ffULL) | ((t0v10 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v10 & 0xff00ff00ff00ff00ULL));\
    t0v10 = ((t0v3 & 0x00ff00ff00ff00ffULL) | ((t0v11 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v11 & 0xff00ff00ff00ff00ULL));\
    t0v11 = ((t0v4 & 0x00ff00ff00ff00ffULL) | ((t0v12 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v12 & 0xff00ff00ff00ff00ULL));\
    t0v12 = ((t0v5 & 0x00ff00ff00ff00ffULL) | ((t0v13 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v5 = (((t0v5 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v13 & 0xff00ff00ff00ff00ULL));\
    t0v13 = ((t0v6 & 0x00ff00ff00ff00ffULL) | ((t0v14 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v14 & 0xff00ff00ff00ff00ULL));\
    t0v14 = ((t0v7 & 0x00ff00ff00ff00ffULL) | ((t0v15 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v7 = (((t0v7 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v15 & 0xff00ff00ff00ff00ULL));\
    t0v15 = ((t0v31 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v26 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v26 = (((t0v31 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v26 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v31 = ((t0v32 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v27 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v27 = (((t0v32 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v27 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v32 = ((t0v24 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v28 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v24 = (((t0v24 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v28 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v28 = ((t0v25 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v29 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v25 = (((t0v25 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v29 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v29 = ((t0v23 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v19 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v19 = (((t0v23 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v19 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v23 = ((t0v16 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v20 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v20 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v20 = ((t0v17 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v21 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v21 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v21 = ((t0v18 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v22 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v22 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v22 = ((t0v30 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v11 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v11 = (((t0v30 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v11 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v30 = ((t0v8 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v12 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v12 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v12 = ((t0v9 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v13 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v13 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v13 = ((t0v10 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v14 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v14 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v14 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v7 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v7 = ((t0v15 & 0x3333333333333333ULL) | ((t0v32 & 0x3333333333333333ULL) << 2));\
    t0v15 = (((t0v15 & 0xccccccccccccccccULL) >> 2) | (t0v32 & 0xccccccccccccccccULL));\
    t0v32 = ((t0v31 & 0x3333333333333333ULL) | ((t0v28 & 0x3333333333333333ULL) << 2));\
    t0v28 = (((t0v31 & 0xccccccccccccccccULL) >> 2) | (t0v28 & 0xccccccccccccccccULL));\
    t0v31 = ((t0v26 & 0x3333333333333333ULL) | ((t0v24 & 0x3333333333333333ULL) << 2));\
    t0v24 = (((t0v26 & 0xccccccccccccccccULL) >> 2) | (t0v24 & 0xccccccccccccccccULL));\
    t0v26 = ((t0v27 & 0x3333333333333333ULL) | ((t0v25 & 0x3333333333333333ULL) << 2));\
    t0v25 = (((t0v27 & 0xccccccccccccccccULL) >> 2) | (t0v25 & 0xccccccccccccccccULL));\
    t0v27 = ((t0v29 & 0x3333333333333333ULL) | ((t0v20 & 0x3333333333333333ULL) << 2));\
    t0v20 = (((t0v29 & 0xccccccccccccccccULL) >> 2) | (t0v20 & 0xccccccccccccccccULL));\
    t0v29 = ((t0v23 & 0x3333333333333333ULL) | ((t0v21 & 0x3333333333333333ULL) << 2));\
    t0v21 = (((t0v23 & 0xccccccccccccccccULL) >> 2) | (t0v21 & 0xccccccccccccccccULL));\
    t0v23 = ((t0v19 & 0x3333333333333333ULL) | ((t0v17 & 0x3333333333333333ULL) << 2));\
    t0v17 = (((t0v19 & 0xccccccccccccccccULL) >> 2) | (t0v17 & 0xccccccccccccccccULL));\
    t0v19 = ((t0v16 & 0x3333333333333333ULL) | ((t0v18 & 0x3333333333333333ULL) << 2));\
    t0v16 = (((t0v16 & 0xccccccccccccccccULL) >> 2) | (t0v18 & 0xccccccccccccccccULL));\
    t0v18 = ((t0v22 & 0x3333333333333333ULL) | ((t0v12 & 0x3333333333333333ULL) << 2));\
    t0v12 = (((t0v22 & 0xccccccccccccccccULL) >> 2) | (t0v12 & 0xccccccccccccccccULL));\
    t0v22 = ((t0v30 & 0x3333333333333333ULL) | ((t0v13 & 0x3333333333333333ULL) << 2));\
    t0v13 = (((t0v30 & 0xccccccccccccccccULL) >> 2) | (t0v13 & 0xccccccccccccccccULL));\
    t0v30 = ((t0v11 & 0x3333333333333333ULL) | ((t0v9 & 0x3333333333333333ULL) << 2));\
    t0v9 = (((t0v11 & 0xccccccccccccccccULL) >> 2) | (t0v9 & 0xccccccccccccccccULL));\
    t0v11 = ((t0v8 & 0x3333333333333333ULL) | ((t0v10 & 0x3333333333333333ULL) << 2));\
    t0v8 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v10 & 0xccccccccccccccccULL));\
    t0v10 = ((t0v14 & 0x3333333333333333ULL) | ((t0v5 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v14 & 0xccccccccccccccccULL) >> 2) | (t0v5 & 0xccccccccccccccccULL));\
    t0v14 = ((t0v4 & 0x3333333333333333ULL) | ((t0v6 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v4 & 0xccccccccccccccccULL) >> 2) | (t0v6 & 0xccccccccccccccccULL));\
    t0v6 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    t0v2 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v1 & 0xccccccccccccccccULL) >> 2) | (t0v3 & 0xccccccccccccccccULL));\
    (D0) = ((t0v7 & 0x5555555555555555ULL) | ((t0v32 & 0x5555555555555555ULL) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v32 & 0xaaaaaaaaaaaaaaaaULL));\
    (D2) = ((t0v15 & 0x5555555555555555ULL) | ((t0v28 & 0x5555555555555555ULL) << 1));\
    (D3) = (((t0v15 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v28 & 0xaaaaaaaaaaaaaaaaULL));\
    (D4) = ((t0v31 & 0x5555555555555555ULL) | ((t0v26 & 0x5555555555555555ULL) << 1));\
    (D5) = (((t0v31 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v26 & 0xaaaaaaaaaaaaaaaaULL));\
    (D6) = ((t0v24 & 0x5555555555555555ULL) | ((t0v25 & 0x5555555555555555ULL) << 1));\
    (D7) = (((t0v24 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v25 & 0xaaaaaaaaaaaaaaaaULL));\
    (D8) = ((t0v27 & 0x5555555555555555ULL) | ((t0v29 & 0x5555555555555555ULL) << 1));\
    (D9) = (((t0v27 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v29 & 0xaaaaaaaaaaaaaaaaULL));\
    (D10) = ((t0v20 & 0x5555555555555555ULL) | ((t0v21 & 0x5555555555555555ULL) << 1));\
    (D11) = (((t0v20 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v21 & 0xaaaaaaaaaaaaaaaaULL));\
    (D12) = ((t0v23 & 0x5555555555555555ULL) | ((t0v19 & 0x5555555555555555ULL) << 1));\
    (D13) = (((t0v23 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v19 & 0xaaaaaaaaaaaaaaaaULL));\
    (D14) = ((t0v17 & 0x5555555555555555ULL) | ((t0v16 & 0x5555555555555555ULL) << 1));\
    (D15) = (((t0v17 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v16 & 0xaaaaaaaaaaaaaaaaULL));\
    (D16) = ((t0v18 & 0x5555555555555555ULL) | ((t0v22 & 0x5555555555555555ULL) << 1));\
    (D17) = (((t0v18 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v22 & 0xaaaaaaaaaaaaaaaaULL));\
    (D18) = ((t0v12 & 0x5555555555555555ULL) | ((t0v13 & 0x5555555555555555ULL) << 1));\
    (D19) = (((t0v12 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v13 & 0xaaaaaaaaaaaaaaaaULL));\
    (D20) = ((t0v30 & 0x5555555555555555ULL) | ((t0v11 & 0x5555555555555555ULL) << 1));\
    (D21) = (((t0v30 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v11 & 0xaaaaaaaaaaaaaaaaULL));\
    (D22) = ((t0v9 & 0x5555555555555555ULL) | ((t0v8 & 0x5555555555555555ULL) << 1));\
    (D23) = (((t0v9 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v8 & 0xaaaaaaaaaaaaaaaaULL));\
    (D24) = ((t0v10 & 0x5555555555555555ULL) | ((t0v14 & 0x5555555555555555ULL) << 1));\
    (D25) = (((t0v10 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v14 & 0xaaaaaaaaaaaaaaaaULL));\
    (D26) = ((t0v5 & 0x5555555555555555ULL) | ((t0v4 & 0x5555555555555555ULL) << 1));\
    (D27) = (((t0v5 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v4 & 0xaaaaaaaaaaaaaaaaULL));\
    (D28) = ((t0v6 & 0x5555555555555555ULL) | ((t0v2 & 0x5555555555555555ULL) << 1));\
    (D29) = (((t0v6 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v2 & 0xaaaaaaaaaaaaaaaaULL));\
    (D30) = ((t0v0 & 0x5555555555555555ULL) | ((t0v1 & 0x5555555555555555ULL) << 1));\
    (D31) = (((t0v0 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v1 & 0xaaaaaaaaaaaaaaaaULL));\
}
"##,
        transform.out(),
    );
    let mut transform = CLANG_TRANSFORM_U64.transform();
    transform.gen_input_transform(23);
    assert_eq!(
        r##"#define IN_TRANSFORM_B23(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, S) \
{\
    uint64_t t0v0;\
    uint64_t t0v1;\
    uint64_t t0v2;\
    uint64_t t0v3;\
    uint64_t t0v4;\
    uint64_t t0v5;\
    uint64_t t0v6;\
    uint64_t t0v7;\
    uint64_t t0v8;\
    uint64_t t0v9;\
    uint64_t t0v10;\
    uint64_t t0v11;\
    uint64_t t0v12;\
    uint64_t t0v13;\
    uint64_t t0v14;\
    uint64_t t0v15;\
    uint64_t t0v16;\
    uint64_t t0v17;\
    uint64_t t0v18;\
    uint64_t t0v19;\
    uint64_t t0v20;\
    uint64_t t0v21;\
    uint64_t t0v22;\
    uint64_t t0v23;\
    uint64_t t0v24;\
    uint64_t t0v25;\
    uint64_t t0v26;\
    uint64_t t0v27;\
    uint64_t t0v28;\
    uint64_t t0v29;\
    uint64_t t0v30;\
    uint64_t t0v31;\
    uint64_t t0v32;\
    t0v0 = ((*((const uint64_t*)(((S) + 0)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 32)))) & 0xffffffffULL) << 32);\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL);\
    t0v2 = ((*((const uint64_t*)(((S) + 2)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 34)))) & 0xffffffffULL) << 32);\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL);\
    t0v4 = ((*((const uint64_t*)(((S) + 4)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 36)))) & 0xffffffffULL) << 32);\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL);\
    t0v6 = ((*((const uint64_t*)(((S) + 6)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 38)))) & 0xffffffffULL) << 32);\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL);\
    t0v8 = ((*((const uint64_t*)(((S) + 8)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 40)))) & 0xffffffffULL) << 32);\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL);\
    t0v10 = ((*((const uint64_t*)(((S) + 10)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 42)))) & 0xffffffffULL) << 32);\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL);\
    t0v12 = ((*((const uint64_t*)(((S) + 12)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 44)))) & 0xffffffffULL) << 32);\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL);\
    t0v14 = ((*((const uint64_t*)(((S) + 14)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 46)))) & 0xffffffffULL) << 32);\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL);\
    t0v16 = ((*((const uint64_t*)(((S) + 16)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 48)))) & 0xffffffffULL) << 32);\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL);\
    t0v18 = ((*((const uint64_t*)(((S) + 18)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 50)))) & 0xffffffffULL) << 32);\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL);\
    t0v20 = ((*((const uint64_t*)(((S) + 20)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 52)))) & 0xffffffffULL) << 32);\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL);\
    t0v22 = ((*((const uint64_t*)(((S) + 22)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 54)))) & 0xffffffffULL) << 32);\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL);\
    t0v24 = ((*((const uint64_t*)(((S) + 24)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 56)))) & 0xffffffffULL) << 32);\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL);\
    t0v26 = ((*((const uint64_t*)(((S) + 26)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 58)))) & 0xffffffffULL) << 32);\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL);\
    t0v28 = ((*((const uint64_t*)(((S) + 28)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 60)))) & 0xffffffffULL) << 32);\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL);\
    t0v30 = ((*((const uint64_t*)(((S) + 30)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 62)))) & 0xffffffffULL) << 32);\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL);\
    t0v32 = ((t0v0 & 0x0000ffff0000ffffULL) | ((t0v16 & 0x0000ffff0000ffffULL) << 16));\
    t0v0 = (((t0v0 & 0xffff0000ffff0000ULL) >> 16) | (t0v16 & 0xffff0000ffff0000ULL));\
    t0v16 = ((t0v1 & 0x0000ffff0000ffffULL) | ((t0v17 & 0x0000ffff0000ffffULL) << 16));\
    t0v1 = (((t0v1 & 0xffff0000ffff0000ULL) >> 16) | (t0v17 & 0xffff0000ffff0000ULL));\
    t0v17 = ((t0v2 & 0x0000ffff0000ffffULL) | ((t0v18 & 0x0000ffff0000ffffULL) << 16));\
    t0v2 = (((t0v2 & 0xffff0000ffff0000ULL) >> 16) | (t0v18 & 0xffff0000ffff0000ULL));\
    t0v18 = ((t0v3 & 0x0000ffff0000ffffULL) | ((t0v19 & 0x0000ffff0000ffffULL) << 16));\
    t0v3 = (((t0v3 & 0xffff0000ffff0000ULL) >> 16) | (t0v19 & 0xffff0000ffff0000ULL));\
    t0v19 = ((t0v4 & 0x0000ffff0000ffffULL) | ((t0v20 & 0x0000ffff0000ffffULL) << 16));\
    t0v4 = (((t0v4 & 0xffff0000ffff0000ULL) >> 16) | (t0v20 & 0xffff0000ffff0000ULL));\
    t0v20 = ((t0v5 & 0x0000ffff0000ffffULL) | ((t0v21 & 0x0000ffff0000ffffULL) << 16));\
    t0v5 = (((t0v5 & 0xffff0000ffff0000ULL) >> 16) | (t0v21 & 0xffff0000ffff0000ULL));\
    t0v21 = ((t0v6 & 0x0000ffff0000ffffULL) | ((t0v22 & 0x0000ffff0000ffffULL) << 16));\
    t0v6 = (((t0v6 & 0xffff0000ffff0000ULL) >> 16) | (t0v22 & 0xffff0000ffff0000ULL));\
    t0v22 = ((t0v7 & 0x0000ffff0000ffffULL) | ((t0v23 & 0x0000ffff0000ffffULL) << 16));\
    t0v7 = (((t0v7 & 0xffff0000ffff0000ULL) >> 16) | (t0v23 & 0xffff0000ffff0000ULL));\
    t0v23 = ((t0v8 & 0x0000ffff0000ffffULL) | ((t0v24 & 0x0000ffff0000ffffULL) << 16));\
    t0v8 = (((t0v8 & 0xffff0000ffff0000ULL) >> 16) | (t0v24 & 0xffff0000ffff0000ULL));\
    t0v24 = ((t0v9 & 0x0000ffff0000ffffULL) | ((t0v25 & 0x0000ffff0000ffffULL) << 16));\
    t0v9 = (((t0v9 & 0xffff0000ffff0000ULL) >> 16) | (t0v25 & 0xffff0000ffff0000ULL));\
    t0v25 = ((t0v10 & 0x0000ffff0000ffffULL) | ((t0v26 & 0x0000ffff0000ffffULL) << 16));\
    t0v10 = (((t0v10 & 0xffff0000ffff0000ULL) >> 16) | (t0v26 & 0xffff0000ffff0000ULL));\
    t0v26 = ((t0v11 & 0x0000ffff0000ffffULL) | ((t0v27 & 0x0000ffff0000ffffULL) << 16));\
    t0v11 = (((t0v11 & 0xffff0000ffff0000ULL) >> 16) | (t0v27 & 0xffff0000ffff0000ULL));\
    t0v27 = ((t0v12 & 0x0000ffff0000ffffULL) | ((t0v28 & 0x0000ffff0000ffffULL) << 16));\
    t0v12 = (((t0v12 & 0xffff0000ffff0000ULL) >> 16) | (t0v28 & 0xffff0000ffff0000ULL));\
    t0v28 = ((t0v13 & 0x0000ffff0000ffffULL) | ((t0v29 & 0x0000ffff0000ffffULL) << 16));\
    t0v13 = (((t0v13 & 0xffff0000ffff0000ULL) >> 16) | (t0v29 & 0xffff0000ffff0000ULL));\
    t0v29 = ((t0v14 & 0x0000ffff0000ffffULL) | ((t0v30 & 0x0000ffff0000ffffULL) << 16));\
    t0v14 = (((t0v14 & 0xffff0000ffff0000ULL) >> 16) | (t0v30 & 0xffff0000ffff0000ULL));\
    t0v30 = ((t0v15 & 0x0000ffff0000ffffULL) | ((t0v31 & 0x0000ffff0000ffffULL) << 16));\
    t0v15 = (((t0v15 & 0xffff0000ffff0000ULL) >> 16) | (t0v31 & 0xffff0000ffff0000ULL));\
    t0v31 = ((t0v32 & 0x00ff00ff00ff00ffULL) | ((t0v23 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v23 = (((t0v32 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v23 & 0xff00ff00ff00ff00ULL));\
    t0v32 = ((t0v16 & 0x00ff00ff00ff00ffULL) | ((t0v24 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v24 & 0xff00ff00ff00ff00ULL));\
    t0v24 = ((t0v17 & 0x00ff00ff00ff00ffULL) | ((t0v25 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v25 & 0xff00ff00ff00ff00ULL));\
    t0v25 = ((t0v18 & 0x00ff00ff00ff00ffULL) | ((t0v26 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v26 & 0xff00ff00ff00ff00ULL));\
    t0v26 = ((t0v19 & 0x00ff00ff00ff00ffULL) | ((t0v27 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v27 & 0xff00ff00ff00ff00ULL));\
    t0v27 = ((t0v20 & 0x00ff00ff00ff00ffULL) | ((t0v28 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v28 & 0xff00ff00ff00ff00ULL));\
    t0v28 = ((t0v21 & 0x00ff00ff00ff00ffULL) | ((t0v29 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v21 = (((t0v21 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v29 & 0xff00ff00ff00ff00ULL));\
    t0v29 = ((t0v22 & 0x00ff00ff00ff00ffULL) | ((t0v30 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v30 & 0xff00ff00ff00ff00ULL));\
    t0v0 = ((t0v0 & 0x00ff00ff00ff00ffULL) | ((t0v8 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v1 = ((t0v1 & 0x00ff00ff00ff00ffULL) | ((t0v9 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v2 = ((t0v2 & 0x00ff00ff00ff00ffULL) | ((t0v10 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v3 = ((t0v3 & 0x00ff00ff00ff00ffULL) | ((t0v11 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v4 = ((t0v4 & 0x00ff00ff00ff00ffULL) | ((t0v12 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v5 = ((t0v5 & 0x00ff00ff00ff00ffULL) | ((t0v13 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v6 = ((t0v6 & 0x00ff00ff00ff00ffULL) | ((t0v14 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v7 = ((t0v7 & 0x00ff00ff00ff00ffULL) | ((t0v15 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v8 = ((t0v31 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v26 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v9 = (((t0v31 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v26 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v10 = ((t0v32 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v27 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v11 = (((t0v32 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v27 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v12 = ((t0v24 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v28 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v13 = (((t0v24 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v28 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v14 = ((t0v25 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v29 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v15 = (((t0v25 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v29 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v24 = ((t0v23 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v19 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v19 = (((t0v23 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v19 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v23 = ((t0v16 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v20 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v20 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v20 = ((t0v17 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v21 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v21 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v21 = ((t0v18 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v22 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v22 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v22 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v7 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v7 = ((t0v8 & 0x3333333333333333ULL) | ((t0v12 & 0x3333333333333333ULL) << 2));\
    t0v8 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v12 & 0xccccccccccccccccULL));\
    t0v12 = ((t0v10 & 0x3333333333333333ULL) | ((t0v14 & 0x3333333333333333ULL) << 2));\
    t0v10 = (((t0v10 & 0xccccccccccccccccULL) >> 2) | (t0v14 & 0xccccccccccccccccULL));\
    t0v14 = ((t0v9 & 0x3333333333333333ULL) | ((t0v13 & 0x3333333333333333ULL) << 2));\
    t0v9 = (((t0v9 & 0xccccccccccccccccULL) >> 2) | (t0v13 & 0xccccccccccccccccULL));\
    t0v13 = ((t0v11 & 0x3333333333333333ULL) | ((t0v15 & 0x3333333333333333ULL) << 2));\
    t0v11 = (((t0v11 & 0xccccccccccccccccULL) >> 2) | (t0v15 & 0xccccccccccccccccULL));\
    t0v15 = ((t0v24 & 0x3333333333333333ULL) | ((t0v20 & 0x3333333333333333ULL) << 2));\
    t0v20 = (((t0v24 & 0xccccccccccccccccULL) >> 2) | (t0v20 & 0xccccccccccccccccULL));\
    t0v24 = ((t0v23 & 0x3333333333333333ULL) | ((t0v21 & 0x3333333333333333ULL) << 2));\
    t0v21 = (((t0v23 & 0xccccccccccccccccULL) >> 2) | (t0v21 & 0xccccccccccccccccULL));\
    t0v23 = ((t0v19 & 0x3333333333333333ULL) | ((t0v17 & 0x3333333333333333ULL) << 2));\
    t0v17 = (((t0v19 & 0xccccccccccccccccULL) >> 2) | (t0v17 & 0xccccccccccccccccULL));\
    t0v19 = ((t0v16 & 0x3333333333333333ULL) | ((t0v18 & 0x3333333333333333ULL) << 2));\
    t0v16 = (((t0v16 & 0xccccccccccccccccULL) >> 2) | (t0v18 & 0xccccccccccccccccULL));\
    t0v18 = ((t0v22 & 0x3333333333333333ULL) | ((t0v5 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v22 & 0xccccccccccccccccULL) >> 2) | (t0v5 & 0xccccccccccccccccULL));\
    t0v22 = ((t0v4 & 0x3333333333333333ULL) | ((t0v6 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v4 & 0xccccccccccccccccULL) >> 2) | (t0v6 & 0xccccccccccccccccULL));\
    t0v6 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    t0v2 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v1 & 0xccccccccccccccccULL) >> 2) | (t0v3 & 0xccccccccccccccccULL));\
    (D0) = ((t0v7 & 0x5555555555555555ULL) | ((t0v12 & 0x5555555555555555ULL) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v12 & 0xaaaaaaaaaaaaaaaaULL));\
    (D2) = ((t0v8 & 0x5555555555555555ULL) | ((t0v10 & 0x5555555555555555ULL) << 1));\
    (D3) = (((t0v8 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v10 & 0xaaaaaaaaaaaaaaaaULL));\
    (D4) = ((t0v14 & 0x5555555555555555ULL) | ((t0v13 & 0x5555555555555555ULL) << 1));\
    (D5) = (((t0v14 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v13 & 0xaaaaaaaaaaaaaaaaULL));\
    (D6) = ((t0v9 & 0x5555555555555555ULL) | ((t0v11 & 0x5555555555555555ULL) << 1));\
    (D7) = (((t0v9 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v11 & 0xaaaaaaaaaaaaaaaaULL));\
    (D8) = ((t0v15 & 0x5555555555555555ULL) | ((t0v24 & 0x5555555555555555ULL) << 1));\
    (D9) = (((t0v15 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v24 & 0xaaaaaaaaaaaaaaaaULL));\
    (D10) = ((t0v20 & 0x5555555555555555ULL) | ((t0v21 & 0x5555555555555555ULL) << 1));\
    (D11) = (((t0v20 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v21 & 0xaaaaaaaaaaaaaaaaULL));\
    (D12) = ((t0v23 & 0x5555555555555555ULL) | ((t0v19 & 0x5555555555555555ULL) << 1));\
    (D13) = (((t0v23 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v19 & 0xaaaaaaaaaaaaaaaaULL));\
    (D14) = ((t0v17 & 0x5555555555555555ULL) | ((t0v16 & 0x5555555555555555ULL) << 1));\
    (D15) = (((t0v17 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v16 & 0xaaaaaaaaaaaaaaaaULL));\
    (D16) = ((t0v18 & 0x5555555555555555ULL) | ((t0v22 & 0x5555555555555555ULL) << 1));\
    (D17) = (((t0v18 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v22 & 0xaaaaaaaaaaaaaaaaULL));\
    (D18) = ((t0v5 & 0x5555555555555555ULL) | ((t0v4 & 0x5555555555555555ULL) << 1));\
    (D19) = (((t0v5 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v4 & 0xaaaaaaaaaaaaaaaaULL));\
    (D20) = ((t0v6 & 0x5555555555555555ULL) | ((t0v2 & 0x5555555555555555ULL) << 1));\
    (D21) = (((t0v6 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v2 & 0xaaaaaaaaaaaaaaaaULL));\
    (D22) = ((t0v0 & 0x5555555555555555ULL) | ((t0v1 & 0x5555555555555555ULL) << 1));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U64.transform();
    transform.gen_input_transform(7);
    assert_eq!(
        r##"#define IN_TRANSFORM_B7(D0, D1, D2, D3, D4, D5, D6, S) \
{\
    uint64_t t0v0;\
    uint64_t t0v1;\
    uint64_t t0v2;\
    uint64_t t0v3;\
    uint64_t t0v4;\
    uint64_t t0v5;\
    uint64_t t0v6;\
    uint64_t t0v7;\
    uint64_t t0v8;\
    uint64_t t0v9;\
    uint64_t t0v10;\
    uint64_t t0v11;\
    uint64_t t0v12;\
    uint64_t t0v13;\
    uint64_t t0v14;\
    uint64_t t0v15;\
    uint64_t t0v16;\
    uint64_t t0v17;\
    uint64_t t0v18;\
    uint64_t t0v19;\
    uint64_t t0v20;\
    uint64_t t0v21;\
    uint64_t t0v22;\
    uint64_t t0v23;\
    uint64_t t0v24;\
    uint64_t t0v25;\
    uint64_t t0v26;\
    uint64_t t0v27;\
    uint64_t t0v28;\
    uint64_t t0v29;\
    uint64_t t0v30;\
    uint64_t t0v31;\
    uint64_t t0v32;\
    t0v0 = ((*((const uint64_t*)(((S) + 0)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 32)))) & 0xffffffffULL) << 32);\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL);\
    t0v2 = ((*((const uint64_t*)(((S) + 2)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 34)))) & 0xffffffffULL) << 32);\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL);\
    t0v4 = ((*((const uint64_t*)(((S) + 4)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 36)))) & 0xffffffffULL) << 32);\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL);\
    t0v6 = ((*((const uint64_t*)(((S) + 6)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 38)))) & 0xffffffffULL) << 32);\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL);\
    t0v8 = ((*((const uint64_t*)(((S) + 8)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 40)))) & 0xffffffffULL) << 32);\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL);\
    t0v10 = ((*((const uint64_t*)(((S) + 10)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 42)))) & 0xffffffffULL) << 32);\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL);\
    t0v12 = ((*((const uint64_t*)(((S) + 12)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 44)))) & 0xffffffffULL) << 32);\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL);\
    t0v14 = ((*((const uint64_t*)(((S) + 14)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 46)))) & 0xffffffffULL) << 32);\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL);\
    t0v16 = ((*((const uint64_t*)(((S) + 16)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 48)))) & 0xffffffffULL) << 32);\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL);\
    t0v18 = ((*((const uint64_t*)(((S) + 18)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 50)))) & 0xffffffffULL) << 32);\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL);\
    t0v20 = ((*((const uint64_t*)(((S) + 20)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 52)))) & 0xffffffffULL) << 32);\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL);\
    t0v22 = ((*((const uint64_t*)(((S) + 22)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 54)))) & 0xffffffffULL) << 32);\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL);\
    t0v24 = ((*((const uint64_t*)(((S) + 24)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 56)))) & 0xffffffffULL) << 32);\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL);\
    t0v26 = ((*((const uint64_t*)(((S) + 26)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 58)))) & 0xffffffffULL) << 32);\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL);\
    t0v28 = ((*((const uint64_t*)(((S) + 28)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 60)))) & 0xffffffffULL) << 32);\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL);\
    t0v30 = ((*((const uint64_t*)(((S) + 30)))) & 0xffffffffULL) | (((*((const uint64_t*)(((S) + 62)))) & 0xffffffffULL) << 32);\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) & 0xffffffff00000000ULL) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL);\
    t0v32 = ((((t0v0 & 0x000000ff000000ffULL) | ((t0v8 & 0x000000ff000000ffULL) << 8)) | ((t0v16 & 0x000000ff000000ffULL) << 16)) | ((t0v24 & 0x000000ff000000ffULL) << 24));\
    t0v0 = ((((t0v1 & 0x000000ff000000ffULL) | ((t0v9 & 0x000000ff000000ffULL) << 8)) | ((t0v17 & 0x000000ff000000ffULL) << 16)) | ((t0v25 & 0x000000ff000000ffULL) << 24));\
    t0v1 = ((((t0v2 & 0x000000ff000000ffULL) | ((t0v10 & 0x000000ff000000ffULL) << 8)) | ((t0v18 & 0x000000ff000000ffULL) << 16)) | ((t0v26 & 0x000000ff000000ffULL) << 24));\
    t0v2 = ((((t0v3 & 0x000000ff000000ffULL) | ((t0v11 & 0x000000ff000000ffULL) << 8)) | ((t0v19 & 0x000000ff000000ffULL) << 16)) | ((t0v27 & 0x000000ff000000ffULL) << 24));\
    t0v3 = ((((t0v4 & 0x000000ff000000ffULL) | ((t0v12 & 0x000000ff000000ffULL) << 8)) | ((t0v20 & 0x000000ff000000ffULL) << 16)) | ((t0v28 & 0x000000ff000000ffULL) << 24));\
    t0v4 = ((((t0v5 & 0x000000ff000000ffULL) | ((t0v13 & 0x000000ff000000ffULL) << 8)) | ((t0v21 & 0x000000ff000000ffULL) << 16)) | ((t0v29 & 0x000000ff000000ffULL) << 24));\
    t0v5 = ((((t0v6 & 0x000000ff000000ffULL) | ((t0v14 & 0x000000ff000000ffULL) << 8)) | ((t0v22 & 0x000000ff000000ffULL) << 16)) | ((t0v30 & 0x000000ff000000ffULL) << 24));\
    t0v6 = ((((t0v7 & 0x000000ff000000ffULL) | ((t0v15 & 0x000000ff000000ffULL) << 8)) | ((t0v23 & 0x000000ff000000ffULL) << 16)) | ((t0v31 & 0x000000ff000000ffULL) << 24));\
    t0v7 = ((t0v32 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v32 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v3 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v8 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v7 & 0x3333333333333333ULL) | ((t0v4 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v7 & 0xccccccccccccccccULL) >> 2) | (t0v4 & 0xccccccccccccccccULL));\
    t0v7 = ((t0v8 & 0x3333333333333333ULL) | ((t0v5 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v5 & 0xccccccccccccccccULL));\
    t0v8 = ((t0v3 & 0x3333333333333333ULL) | ((t0v1 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v3 & 0xccccccccccccccccULL) >> 2) | (t0v1 & 0xccccccccccccccccULL));\
    t0v3 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    (D0) = ((t0v6 & 0x5555555555555555ULL) | ((t0v7 & 0x5555555555555555ULL) << 1));\
    (D1) = (((t0v6 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v7 & 0xaaaaaaaaaaaaaaaaULL));\
    (D2) = ((t0v4 & 0x5555555555555555ULL) | ((t0v5 & 0x5555555555555555ULL) << 1));\
    (D3) = (((t0v4 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v5 & 0xaaaaaaaaaaaaaaaaULL));\
    (D4) = ((t0v8 & 0x5555555555555555ULL) | ((t0v3 & 0x5555555555555555ULL) << 1));\
    (D5) = (((t0v8 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v3 & 0xaaaaaaaaaaaaaaaaULL));\
    (D6) = ((t0v1 & 0x5555555555555555ULL) | ((t0v0 & 0x5555555555555555ULL) << 1));\
}
"##,
        transform.out()
    );

    // Intel MMX
    let mut transform = CLANG_TRANSFORM_INTEL_MMX.transform();
    transform.gen_input_transform(32);
    assert_eq!(
        r##"#define IN_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    const __m64 c8 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c9 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c7 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*5));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*6));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*7));\
    const __m64 c0 = (*(const __m64*)(transform_const_tbl + 2*8));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*9));\
    __m64 t0v0;\
    __m64 t0v1;\
    __m64 t0v2;\
    __m64 t0v3;\
    __m64 t0v4;\
    __m64 t0v5;\
    __m64 t0v6;\
    __m64 t0v7;\
    __m64 t0v8;\
    __m64 t0v9;\
    __m64 t0v10;\
    __m64 t0v11;\
    __m64 t0v12;\
    __m64 t0v13;\
    __m64 t0v14;\
    __m64 t0v15;\
    __m64 t0v16;\
    __m64 t0v17;\
    __m64 t0v18;\
    __m64 t0v19;\
    __m64 t0v20;\
    __m64 t0v21;\
    __m64 t0v22;\
    __m64 t0v23;\
    __m64 t0v24;\
    __m64 t0v25;\
    __m64 t0v26;\
    __m64 t0v27;\
    __m64 t0v28;\
    __m64 t0v29;\
    __m64 t0v30;\
    __m64 t0v31;\
    __m64 t0v32;\
    t0v0 = _m_punpckldq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v1 = _m_punpckhdq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v2 = _m_punpckldq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v3 = _m_punpckhdq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v4 = _m_punpckldq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v5 = _m_punpckhdq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v6 = _m_punpckldq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v7 = _m_punpckhdq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v8 = _m_punpckldq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v9 = _m_punpckhdq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v10 = _m_punpckldq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v11 = _m_punpckhdq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v12 = _m_punpckldq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v13 = _m_punpckhdq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v14 = _m_punpckldq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v15 = _m_punpckhdq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v16 = _m_punpckldq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v17 = _m_punpckhdq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v18 = _m_punpckldq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v19 = _m_punpckhdq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v20 = _m_punpckldq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v21 = _m_punpckhdq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v22 = _m_punpckldq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v23 = _m_punpckhdq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v24 = _m_punpckldq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v25 = _m_punpckhdq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v26 = _m_punpckldq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v27 = _m_punpckhdq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v28 = _m_punpckldq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v29 = _m_punpckhdq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v30 = _m_punpckldq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v31 = _m_punpckhdq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v32 = _m_por(_m_pand(t0v0, c0), _m_pslldi(_m_pand(t0v16, c0), 16));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c1), 16), _m_pand(t0v16, c1));\
    t0v16 = _m_por(_m_pand(t0v1, c0), _m_pslldi(_m_pand(t0v17, c0), 16));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c1), 16), _m_pand(t0v17, c1));\
    t0v17 = _m_por(_m_pand(t0v2, c0), _m_pslldi(_m_pand(t0v18, c0), 16));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c1), 16), _m_pand(t0v18, c1));\
    t0v18 = _m_por(_m_pand(t0v3, c0), _m_pslldi(_m_pand(t0v19, c0), 16));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v3, c1), 16), _m_pand(t0v19, c1));\
    t0v19 = _m_por(_m_pand(t0v4, c0), _m_pslldi(_m_pand(t0v20, c0), 16));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v4, c1), 16), _m_pand(t0v20, c1));\
    t0v20 = _m_por(_m_pand(t0v5, c0), _m_pslldi(_m_pand(t0v21, c0), 16));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v5, c1), 16), _m_pand(t0v21, c1));\
    t0v21 = _m_por(_m_pand(t0v6, c0), _m_pslldi(_m_pand(t0v22, c0), 16));\
    t0v6 = _m_por(_m_psrldi(_m_pand(t0v6, c1), 16), _m_pand(t0v22, c1));\
    t0v22 = _m_por(_m_pand(t0v7, c0), _m_pslldi(_m_pand(t0v23, c0), 16));\
    t0v7 = _m_por(_m_psrldi(_m_pand(t0v7, c1), 16), _m_pand(t0v23, c1));\
    t0v23 = _m_por(_m_pand(t0v8, c0), _m_pslldi(_m_pand(t0v24, c0), 16));\
    t0v8 = _m_por(_m_psrldi(_m_pand(t0v8, c1), 16), _m_pand(t0v24, c1));\
    t0v24 = _m_por(_m_pand(t0v9, c0), _m_pslldi(_m_pand(t0v25, c0), 16));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v9, c1), 16), _m_pand(t0v25, c1));\
    t0v25 = _m_por(_m_pand(t0v10, c0), _m_pslldi(_m_pand(t0v26, c0), 16));\
    t0v10 = _m_por(_m_psrldi(_m_pand(t0v10, c1), 16), _m_pand(t0v26, c1));\
    t0v26 = _m_por(_m_pand(t0v11, c0), _m_pslldi(_m_pand(t0v27, c0), 16));\
    t0v11 = _m_por(_m_psrldi(_m_pand(t0v11, c1), 16), _m_pand(t0v27, c1));\
    t0v27 = _m_por(_m_pand(t0v12, c0), _m_pslldi(_m_pand(t0v28, c0), 16));\
    t0v12 = _m_por(_m_psrldi(_m_pand(t0v12, c1), 16), _m_pand(t0v28, c1));\
    t0v28 = _m_por(_m_pand(t0v13, c0), _m_pslldi(_m_pand(t0v29, c0), 16));\
    t0v13 = _m_por(_m_psrldi(_m_pand(t0v13, c1), 16), _m_pand(t0v29, c1));\
    t0v29 = _m_por(_m_pand(t0v14, c0), _m_pslldi(_m_pand(t0v30, c0), 16));\
    t0v14 = _m_por(_m_psrldi(_m_pand(t0v14, c1), 16), _m_pand(t0v30, c1));\
    t0v30 = _m_por(_m_pand(t0v15, c0), _m_pslldi(_m_pand(t0v31, c0), 16));\
    t0v15 = _m_por(_m_psrldi(_m_pand(t0v15, c1), 16), _m_pand(t0v31, c1));\
    t0v31 = _m_por(_m_pand(t0v32, c2), _m_pslldi(_m_pand(t0v23, c2), 8));\
    t0v23 = _m_por(_m_psrldi(_m_pand(t0v32, c3), 8), _m_pand(t0v23, c3));\
    t0v32 = _m_por(_m_pand(t0v16, c2), _m_pslldi(_m_pand(t0v24, c2), 8));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c3), 8), _m_pand(t0v24, c3));\
    t0v24 = _m_por(_m_pand(t0v17, c2), _m_pslldi(_m_pand(t0v25, c2), 8));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v17, c3), 8), _m_pand(t0v25, c3));\
    t0v25 = _m_por(_m_pand(t0v18, c2), _m_pslldi(_m_pand(t0v26, c2), 8));\
    t0v18 = _m_por(_m_psrldi(_m_pand(t0v18, c3), 8), _m_pand(t0v26, c3));\
    t0v26 = _m_por(_m_pand(t0v19, c2), _m_pslldi(_m_pand(t0v27, c2), 8));\
    t0v19 = _m_por(_m_psrldi(_m_pand(t0v19, c3), 8), _m_pand(t0v27, c3));\
    t0v27 = _m_por(_m_pand(t0v20, c2), _m_pslldi(_m_pand(t0v28, c2), 8));\
    t0v20 = _m_por(_m_psrldi(_m_pand(t0v20, c3), 8), _m_pand(t0v28, c3));\
    t0v28 = _m_por(_m_pand(t0v21, c2), _m_pslldi(_m_pand(t0v29, c2), 8));\
    t0v21 = _m_por(_m_psrldi(_m_pand(t0v21, c3), 8), _m_pand(t0v29, c3));\
    t0v29 = _m_por(_m_pand(t0v22, c2), _m_pslldi(_m_pand(t0v30, c2), 8));\
    t0v22 = _m_por(_m_psrldi(_m_pand(t0v22, c3), 8), _m_pand(t0v30, c3));\
    t0v30 = _m_por(_m_pand(t0v0, c2), _m_pslldi(_m_pand(t0v8, c2), 8));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c3), 8), _m_pand(t0v8, c3));\
    t0v8 = _m_por(_m_pand(t0v1, c2), _m_pslldi(_m_pand(t0v9, c2), 8));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c3), 8), _m_pand(t0v9, c3));\
    t0v9 = _m_por(_m_pand(t0v2, c2), _m_pslldi(_m_pand(t0v10, c2), 8));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c3), 8), _m_pand(t0v10, c3));\
    t0v10 = _m_por(_m_pand(t0v3, c2), _m_pslldi(_m_pand(t0v11, c2), 8));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v3, c3), 8), _m_pand(t0v11, c3));\
    t0v11 = _m_por(_m_pand(t0v4, c2), _m_pslldi(_m_pand(t0v12, c2), 8));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v4, c3), 8), _m_pand(t0v12, c3));\
    t0v12 = _m_por(_m_pand(t0v5, c2), _m_pslldi(_m_pand(t0v13, c2), 8));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v5, c3), 8), _m_pand(t0v13, c3));\
    t0v13 = _m_por(_m_pand(t0v6, c2), _m_pslldi(_m_pand(t0v14, c2), 8));\
    t0v6 = _m_por(_m_psrldi(_m_pand(t0v6, c3), 8), _m_pand(t0v14, c3));\
    t0v14 = _m_por(_m_pand(t0v7, c2), _m_pslldi(_m_pand(t0v15, c2), 8));\
    t0v7 = _m_por(_m_psrldi(_m_pand(t0v7, c3), 8), _m_pand(t0v15, c3));\
    t0v15 = _m_por(_m_pand(t0v31, c4), _m_pslldi(_m_pand(t0v26, c4), 4));\
    t0v26 = _m_por(_m_psrldi(_m_pand(t0v31, c5), 4), _m_pand(t0v26, c5));\
    t0v31 = _m_por(_m_pand(t0v32, c4), _m_pslldi(_m_pand(t0v27, c4), 4));\
    t0v27 = _m_por(_m_psrldi(_m_pand(t0v32, c5), 4), _m_pand(t0v27, c5));\
    t0v32 = _m_por(_m_pand(t0v24, c4), _m_pslldi(_m_pand(t0v28, c4), 4));\
    t0v24 = _m_por(_m_psrldi(_m_pand(t0v24, c5), 4), _m_pand(t0v28, c5));\
    t0v28 = _m_por(_m_pand(t0v25, c4), _m_pslldi(_m_pand(t0v29, c4), 4));\
    t0v25 = _m_por(_m_psrldi(_m_pand(t0v25, c5), 4), _m_pand(t0v29, c5));\
    t0v29 = _m_por(_m_pand(t0v23, c4), _m_pslldi(_m_pand(t0v19, c4), 4));\
    t0v19 = _m_por(_m_psrldi(_m_pand(t0v23, c5), 4), _m_pand(t0v19, c5));\
    t0v23 = _m_por(_m_pand(t0v16, c4), _m_pslldi(_m_pand(t0v20, c4), 4));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c5), 4), _m_pand(t0v20, c5));\
    t0v20 = _m_por(_m_pand(t0v17, c4), _m_pslldi(_m_pand(t0v21, c4), 4));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v17, c5), 4), _m_pand(t0v21, c5));\
    t0v21 = _m_por(_m_pand(t0v18, c4), _m_pslldi(_m_pand(t0v22, c4), 4));\
    t0v18 = _m_por(_m_psrldi(_m_pand(t0v18, c5), 4), _m_pand(t0v22, c5));\
    t0v22 = _m_por(_m_pand(t0v30, c4), _m_pslldi(_m_pand(t0v11, c4), 4));\
    t0v11 = _m_por(_m_psrldi(_m_pand(t0v30, c5), 4), _m_pand(t0v11, c5));\
    t0v30 = _m_por(_m_pand(t0v8, c4), _m_pslldi(_m_pand(t0v12, c4), 4));\
    t0v8 = _m_por(_m_psrldi(_m_pand(t0v8, c5), 4), _m_pand(t0v12, c5));\
    t0v12 = _m_por(_m_pand(t0v9, c4), _m_pslldi(_m_pand(t0v13, c4), 4));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v9, c5), 4), _m_pand(t0v13, c5));\
    t0v13 = _m_por(_m_pand(t0v10, c4), _m_pslldi(_m_pand(t0v14, c4), 4));\
    t0v10 = _m_por(_m_psrldi(_m_pand(t0v10, c5), 4), _m_pand(t0v14, c5));\
    t0v14 = _m_por(_m_pand(t0v0, c4), _m_pslldi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_pslldi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_por(_m_pand(t0v2, c4), _m_pslldi(_m_pand(t0v6, c4), 4));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c5), 4), _m_pand(t0v6, c5));\
    t0v6 = _m_por(_m_pand(t0v3, c4), _m_pslldi(_m_pand(t0v7, c4), 4));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v3, c5), 4), _m_pand(t0v7, c5));\
    t0v7 = _m_por(_m_pand(t0v15, c6), _m_pslldi(_m_pand(t0v32, c6), 2));\
    t0v15 = _m_por(_m_psrldi(_m_pand(t0v15, c7), 2), _m_pand(t0v32, c7));\
    t0v32 = _m_por(_m_pand(t0v31, c6), _m_pslldi(_m_pand(t0v28, c6), 2));\
    t0v28 = _m_por(_m_psrldi(_m_pand(t0v31, c7), 2), _m_pand(t0v28, c7));\
    t0v31 = _m_por(_m_pand(t0v26, c6), _m_pslldi(_m_pand(t0v24, c6), 2));\
    t0v24 = _m_por(_m_psrldi(_m_pand(t0v26, c7), 2), _m_pand(t0v24, c7));\
    t0v26 = _m_por(_m_pand(t0v27, c6), _m_pslldi(_m_pand(t0v25, c6), 2));\
    t0v25 = _m_por(_m_psrldi(_m_pand(t0v27, c7), 2), _m_pand(t0v25, c7));\
    t0v27 = _m_por(_m_pand(t0v29, c6), _m_pslldi(_m_pand(t0v20, c6), 2));\
    t0v20 = _m_por(_m_psrldi(_m_pand(t0v29, c7), 2), _m_pand(t0v20, c7));\
    t0v29 = _m_por(_m_pand(t0v23, c6), _m_pslldi(_m_pand(t0v21, c6), 2));\
    t0v21 = _m_por(_m_psrldi(_m_pand(t0v23, c7), 2), _m_pand(t0v21, c7));\
    t0v23 = _m_por(_m_pand(t0v19, c6), _m_pslldi(_m_pand(t0v17, c6), 2));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v19, c7), 2), _m_pand(t0v17, c7));\
    t0v19 = _m_por(_m_pand(t0v16, c6), _m_pslldi(_m_pand(t0v18, c6), 2));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c7), 2), _m_pand(t0v18, c7));\
    t0v18 = _m_por(_m_pand(t0v22, c6), _m_pslldi(_m_pand(t0v12, c6), 2));\
    t0v12 = _m_por(_m_psrldi(_m_pand(t0v22, c7), 2), _m_pand(t0v12, c7));\
    t0v22 = _m_por(_m_pand(t0v30, c6), _m_pslldi(_m_pand(t0v13, c6), 2));\
    t0v13 = _m_por(_m_psrldi(_m_pand(t0v30, c7), 2), _m_pand(t0v13, c7));\
    t0v30 = _m_por(_m_pand(t0v11, c6), _m_pslldi(_m_pand(t0v9, c6), 2));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v11, c7), 2), _m_pand(t0v9, c7));\
    t0v11 = _m_por(_m_pand(t0v8, c6), _m_pslldi(_m_pand(t0v10, c6), 2));\
    t0v8 = _m_por(_m_psrldi(_m_pand(t0v8, c7), 2), _m_pand(t0v10, c7));\
    t0v10 = _m_por(_m_pand(t0v14, c6), _m_pslldi(_m_pand(t0v5, c6), 2));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v14, c7), 2), _m_pand(t0v5, c7));\
    t0v14 = _m_por(_m_pand(t0v4, c6), _m_pslldi(_m_pand(t0v6, c6), 2));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v4, c7), 2), _m_pand(t0v6, c7));\
    t0v6 = _m_por(_m_pand(t0v0, c6), _m_pslldi(_m_pand(t0v2, c6), 2));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c7), 2), _m_pand(t0v2, c7));\
    t0v2 = _m_por(_m_pand(t0v1, c6), _m_pslldi(_m_pand(t0v3, c6), 2));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c7), 2), _m_pand(t0v3, c7));\
    (D0) = _m_por(_m_pand(t0v7, c8), _m_pslldi(_m_pand(t0v32, c8), 1));\
    (D1) = _m_por(_m_psrldi(_m_pand(t0v7, c9), 1), _m_pand(t0v32, c9));\
    (D2) = _m_por(_m_pand(t0v15, c8), _m_pslldi(_m_pand(t0v28, c8), 1));\
    (D3) = _m_por(_m_psrldi(_m_pand(t0v15, c9), 1), _m_pand(t0v28, c9));\
    (D4) = _m_por(_m_pand(t0v31, c8), _m_pslldi(_m_pand(t0v26, c8), 1));\
    (D5) = _m_por(_m_psrldi(_m_pand(t0v31, c9), 1), _m_pand(t0v26, c9));\
    (D6) = _m_por(_m_pand(t0v24, c8), _m_pslldi(_m_pand(t0v25, c8), 1));\
    (D7) = _m_por(_m_psrldi(_m_pand(t0v24, c9), 1), _m_pand(t0v25, c9));\
    (D8) = _m_por(_m_pand(t0v27, c8), _m_pslldi(_m_pand(t0v29, c8), 1));\
    (D9) = _m_por(_m_psrldi(_m_pand(t0v27, c9), 1), _m_pand(t0v29, c9));\
    (D10) = _m_por(_m_pand(t0v20, c8), _m_pslldi(_m_pand(t0v21, c8), 1));\
    (D11) = _m_por(_m_psrldi(_m_pand(t0v20, c9), 1), _m_pand(t0v21, c9));\
    (D12) = _m_por(_m_pand(t0v23, c8), _m_pslldi(_m_pand(t0v19, c8), 1));\
    (D13) = _m_por(_m_psrldi(_m_pand(t0v23, c9), 1), _m_pand(t0v19, c9));\
    (D14) = _m_por(_m_pand(t0v17, c8), _m_pslldi(_m_pand(t0v16, c8), 1));\
    (D15) = _m_por(_m_psrldi(_m_pand(t0v17, c9), 1), _m_pand(t0v16, c9));\
    (D16) = _m_por(_m_pand(t0v18, c8), _m_pslldi(_m_pand(t0v22, c8), 1));\
    (D17) = _m_por(_m_psrldi(_m_pand(t0v18, c9), 1), _m_pand(t0v22, c9));\
    (D18) = _m_por(_m_pand(t0v12, c8), _m_pslldi(_m_pand(t0v13, c8), 1));\
    (D19) = _m_por(_m_psrldi(_m_pand(t0v12, c9), 1), _m_pand(t0v13, c9));\
    (D20) = _m_por(_m_pand(t0v30, c8), _m_pslldi(_m_pand(t0v11, c8), 1));\
    (D21) = _m_por(_m_psrldi(_m_pand(t0v30, c9), 1), _m_pand(t0v11, c9));\
    (D22) = _m_por(_m_pand(t0v9, c8), _m_pslldi(_m_pand(t0v8, c8), 1));\
    (D23) = _m_por(_m_psrldi(_m_pand(t0v9, c9), 1), _m_pand(t0v8, c9));\
    (D24) = _m_por(_m_pand(t0v10, c8), _m_pslldi(_m_pand(t0v14, c8), 1));\
    (D25) = _m_por(_m_psrldi(_m_pand(t0v10, c9), 1), _m_pand(t0v14, c9));\
    (D26) = _m_por(_m_pand(t0v5, c8), _m_pslldi(_m_pand(t0v4, c8), 1));\
    (D27) = _m_por(_m_psrldi(_m_pand(t0v5, c9), 1), _m_pand(t0v4, c9));\
    (D28) = _m_por(_m_pand(t0v6, c8), _m_pslldi(_m_pand(t0v2, c8), 1));\
    (D29) = _m_por(_m_psrldi(_m_pand(t0v6, c9), 1), _m_pand(t0v2, c9));\
    (D30) = _m_por(_m_pand(t0v0, c8), _m_pslldi(_m_pand(t0v1, c8), 1));\
    (D31) = _m_por(_m_psrldi(_m_pand(t0v0, c9), 1), _m_pand(t0v1, c9));\
}
"##,
        transform.out(),
    );
    let mut transform = CLANG_TRANSFORM_INTEL_MMX.transform();
    transform.gen_input_transform(23);
    assert_eq!(
        r##"#define IN_TRANSFORM_B23(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, S) \
{\
    const __m64 c8 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c9 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c7 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*5));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*6));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*7));\
    const __m64 c0 = (*(const __m64*)(transform_const_tbl + 2*8));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*9));\
    __m64 t0v0;\
    __m64 t0v1;\
    __m64 t0v2;\
    __m64 t0v3;\
    __m64 t0v4;\
    __m64 t0v5;\
    __m64 t0v6;\
    __m64 t0v7;\
    __m64 t0v8;\
    __m64 t0v9;\
    __m64 t0v10;\
    __m64 t0v11;\
    __m64 t0v12;\
    __m64 t0v13;\
    __m64 t0v14;\
    __m64 t0v15;\
    __m64 t0v16;\
    __m64 t0v17;\
    __m64 t0v18;\
    __m64 t0v19;\
    __m64 t0v20;\
    __m64 t0v21;\
    __m64 t0v22;\
    __m64 t0v23;\
    __m64 t0v24;\
    __m64 t0v25;\
    __m64 t0v26;\
    __m64 t0v27;\
    __m64 t0v28;\
    __m64 t0v29;\
    __m64 t0v30;\
    __m64 t0v31;\
    __m64 t0v32;\
    t0v0 = _m_punpckldq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v1 = _m_punpckhdq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v2 = _m_punpckldq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v3 = _m_punpckhdq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v4 = _m_punpckldq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v5 = _m_punpckhdq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v6 = _m_punpckldq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v7 = _m_punpckhdq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v8 = _m_punpckldq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v9 = _m_punpckhdq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v10 = _m_punpckldq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v11 = _m_punpckhdq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v12 = _m_punpckldq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v13 = _m_punpckhdq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v14 = _m_punpckldq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v15 = _m_punpckhdq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v16 = _m_punpckldq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v17 = _m_punpckhdq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v18 = _m_punpckldq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v19 = _m_punpckhdq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v20 = _m_punpckldq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v21 = _m_punpckhdq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v22 = _m_punpckldq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v23 = _m_punpckhdq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v24 = _m_punpckldq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v25 = _m_punpckhdq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v26 = _m_punpckldq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v27 = _m_punpckhdq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v28 = _m_punpckldq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v29 = _m_punpckhdq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v30 = _m_punpckldq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v31 = _m_punpckhdq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v32 = _m_por(_m_pand(t0v0, c0), _m_pslldi(_m_pand(t0v16, c0), 16));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c1), 16), _m_pand(t0v16, c1));\
    t0v16 = _m_por(_m_pand(t0v1, c0), _m_pslldi(_m_pand(t0v17, c0), 16));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c1), 16), _m_pand(t0v17, c1));\
    t0v17 = _m_por(_m_pand(t0v2, c0), _m_pslldi(_m_pand(t0v18, c0), 16));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c1), 16), _m_pand(t0v18, c1));\
    t0v18 = _m_por(_m_pand(t0v3, c0), _m_pslldi(_m_pand(t0v19, c0), 16));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v3, c1), 16), _m_pand(t0v19, c1));\
    t0v19 = _m_por(_m_pand(t0v4, c0), _m_pslldi(_m_pand(t0v20, c0), 16));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v4, c1), 16), _m_pand(t0v20, c1));\
    t0v20 = _m_por(_m_pand(t0v5, c0), _m_pslldi(_m_pand(t0v21, c0), 16));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v5, c1), 16), _m_pand(t0v21, c1));\
    t0v21 = _m_por(_m_pand(t0v6, c0), _m_pslldi(_m_pand(t0v22, c0), 16));\
    t0v6 = _m_por(_m_psrldi(_m_pand(t0v6, c1), 16), _m_pand(t0v22, c1));\
    t0v22 = _m_por(_m_pand(t0v7, c0), _m_pslldi(_m_pand(t0v23, c0), 16));\
    t0v7 = _m_por(_m_psrldi(_m_pand(t0v7, c1), 16), _m_pand(t0v23, c1));\
    t0v23 = _m_por(_m_pand(t0v8, c0), _m_pslldi(_m_pand(t0v24, c0), 16));\
    t0v8 = _m_por(_m_psrldi(_m_pand(t0v8, c1), 16), _m_pand(t0v24, c1));\
    t0v24 = _m_por(_m_pand(t0v9, c0), _m_pslldi(_m_pand(t0v25, c0), 16));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v9, c1), 16), _m_pand(t0v25, c1));\
    t0v25 = _m_por(_m_pand(t0v10, c0), _m_pslldi(_m_pand(t0v26, c0), 16));\
    t0v10 = _m_por(_m_psrldi(_m_pand(t0v10, c1), 16), _m_pand(t0v26, c1));\
    t0v26 = _m_por(_m_pand(t0v11, c0), _m_pslldi(_m_pand(t0v27, c0), 16));\
    t0v11 = _m_por(_m_psrldi(_m_pand(t0v11, c1), 16), _m_pand(t0v27, c1));\
    t0v27 = _m_por(_m_pand(t0v12, c0), _m_pslldi(_m_pand(t0v28, c0), 16));\
    t0v12 = _m_por(_m_psrldi(_m_pand(t0v12, c1), 16), _m_pand(t0v28, c1));\
    t0v28 = _m_por(_m_pand(t0v13, c0), _m_pslldi(_m_pand(t0v29, c0), 16));\
    t0v13 = _m_por(_m_psrldi(_m_pand(t0v13, c1), 16), _m_pand(t0v29, c1));\
    t0v29 = _m_por(_m_pand(t0v14, c0), _m_pslldi(_m_pand(t0v30, c0), 16));\
    t0v14 = _m_por(_m_psrldi(_m_pand(t0v14, c1), 16), _m_pand(t0v30, c1));\
    t0v30 = _m_por(_m_pand(t0v15, c0), _m_pslldi(_m_pand(t0v31, c0), 16));\
    t0v15 = _m_por(_m_psrldi(_m_pand(t0v15, c1), 16), _m_pand(t0v31, c1));\
    t0v31 = _m_por(_m_pand(t0v32, c2), _m_pslldi(_m_pand(t0v23, c2), 8));\
    t0v23 = _m_por(_m_psrldi(_m_pand(t0v32, c3), 8), _m_pand(t0v23, c3));\
    t0v32 = _m_por(_m_pand(t0v16, c2), _m_pslldi(_m_pand(t0v24, c2), 8));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c3), 8), _m_pand(t0v24, c3));\
    t0v24 = _m_por(_m_pand(t0v17, c2), _m_pslldi(_m_pand(t0v25, c2), 8));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v17, c3), 8), _m_pand(t0v25, c3));\
    t0v25 = _m_por(_m_pand(t0v18, c2), _m_pslldi(_m_pand(t0v26, c2), 8));\
    t0v18 = _m_por(_m_psrldi(_m_pand(t0v18, c3), 8), _m_pand(t0v26, c3));\
    t0v26 = _m_por(_m_pand(t0v19, c2), _m_pslldi(_m_pand(t0v27, c2), 8));\
    t0v19 = _m_por(_m_psrldi(_m_pand(t0v19, c3), 8), _m_pand(t0v27, c3));\
    t0v27 = _m_por(_m_pand(t0v20, c2), _m_pslldi(_m_pand(t0v28, c2), 8));\
    t0v20 = _m_por(_m_psrldi(_m_pand(t0v20, c3), 8), _m_pand(t0v28, c3));\
    t0v28 = _m_por(_m_pand(t0v21, c2), _m_pslldi(_m_pand(t0v29, c2), 8));\
    t0v21 = _m_por(_m_psrldi(_m_pand(t0v21, c3), 8), _m_pand(t0v29, c3));\
    t0v29 = _m_por(_m_pand(t0v22, c2), _m_pslldi(_m_pand(t0v30, c2), 8));\
    t0v22 = _m_por(_m_psrldi(_m_pand(t0v22, c3), 8), _m_pand(t0v30, c3));\
    t0v0 = _m_por(_m_pand(t0v0, c2), _m_pslldi(_m_pand(t0v8, c2), 8));\
    t0v1 = _m_por(_m_pand(t0v1, c2), _m_pslldi(_m_pand(t0v9, c2), 8));\
    t0v2 = _m_por(_m_pand(t0v2, c2), _m_pslldi(_m_pand(t0v10, c2), 8));\
    t0v3 = _m_por(_m_pand(t0v3, c2), _m_pslldi(_m_pand(t0v11, c2), 8));\
    t0v4 = _m_por(_m_pand(t0v4, c2), _m_pslldi(_m_pand(t0v12, c2), 8));\
    t0v5 = _m_por(_m_pand(t0v5, c2), _m_pslldi(_m_pand(t0v13, c2), 8));\
    t0v6 = _m_por(_m_pand(t0v6, c2), _m_pslldi(_m_pand(t0v14, c2), 8));\
    t0v7 = _m_por(_m_pand(t0v7, c2), _m_pslldi(_m_pand(t0v15, c2), 8));\
    t0v8 = _m_por(_m_pand(t0v31, c4), _m_pslldi(_m_pand(t0v26, c4), 4));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v31, c5), 4), _m_pand(t0v26, c5));\
    t0v10 = _m_por(_m_pand(t0v32, c4), _m_pslldi(_m_pand(t0v27, c4), 4));\
    t0v11 = _m_por(_m_psrldi(_m_pand(t0v32, c5), 4), _m_pand(t0v27, c5));\
    t0v12 = _m_por(_m_pand(t0v24, c4), _m_pslldi(_m_pand(t0v28, c4), 4));\
    t0v13 = _m_por(_m_psrldi(_m_pand(t0v24, c5), 4), _m_pand(t0v28, c5));\
    t0v14 = _m_por(_m_pand(t0v25, c4), _m_pslldi(_m_pand(t0v29, c4), 4));\
    t0v15 = _m_por(_m_psrldi(_m_pand(t0v25, c5), 4), _m_pand(t0v29, c5));\
    t0v24 = _m_por(_m_pand(t0v23, c4), _m_pslldi(_m_pand(t0v19, c4), 4));\
    t0v19 = _m_por(_m_psrldi(_m_pand(t0v23, c5), 4), _m_pand(t0v19, c5));\
    t0v23 = _m_por(_m_pand(t0v16, c4), _m_pslldi(_m_pand(t0v20, c4), 4));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c5), 4), _m_pand(t0v20, c5));\
    t0v20 = _m_por(_m_pand(t0v17, c4), _m_pslldi(_m_pand(t0v21, c4), 4));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v17, c5), 4), _m_pand(t0v21, c5));\
    t0v21 = _m_por(_m_pand(t0v18, c4), _m_pslldi(_m_pand(t0v22, c4), 4));\
    t0v18 = _m_por(_m_psrldi(_m_pand(t0v18, c5), 4), _m_pand(t0v22, c5));\
    t0v22 = _m_por(_m_pand(t0v0, c4), _m_pslldi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_pslldi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_por(_m_pand(t0v2, c4), _m_pslldi(_m_pand(t0v6, c4), 4));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c5), 4), _m_pand(t0v6, c5));\
    t0v6 = _m_por(_m_pand(t0v3, c4), _m_pslldi(_m_pand(t0v7, c4), 4));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v3, c5), 4), _m_pand(t0v7, c5));\
    t0v7 = _m_por(_m_pand(t0v8, c6), _m_pslldi(_m_pand(t0v12, c6), 2));\
    t0v8 = _m_por(_m_psrldi(_m_pand(t0v8, c7), 2), _m_pand(t0v12, c7));\
    t0v12 = _m_por(_m_pand(t0v10, c6), _m_pslldi(_m_pand(t0v14, c6), 2));\
    t0v10 = _m_por(_m_psrldi(_m_pand(t0v10, c7), 2), _m_pand(t0v14, c7));\
    t0v14 = _m_por(_m_pand(t0v9, c6), _m_pslldi(_m_pand(t0v13, c6), 2));\
    t0v9 = _m_por(_m_psrldi(_m_pand(t0v9, c7), 2), _m_pand(t0v13, c7));\
    t0v13 = _m_por(_m_pand(t0v11, c6), _m_pslldi(_m_pand(t0v15, c6), 2));\
    t0v11 = _m_por(_m_psrldi(_m_pand(t0v11, c7), 2), _m_pand(t0v15, c7));\
    t0v15 = _m_por(_m_pand(t0v24, c6), _m_pslldi(_m_pand(t0v20, c6), 2));\
    t0v20 = _m_por(_m_psrldi(_m_pand(t0v24, c7), 2), _m_pand(t0v20, c7));\
    t0v24 = _m_por(_m_pand(t0v23, c6), _m_pslldi(_m_pand(t0v21, c6), 2));\
    t0v21 = _m_por(_m_psrldi(_m_pand(t0v23, c7), 2), _m_pand(t0v21, c7));\
    t0v23 = _m_por(_m_pand(t0v19, c6), _m_pslldi(_m_pand(t0v17, c6), 2));\
    t0v17 = _m_por(_m_psrldi(_m_pand(t0v19, c7), 2), _m_pand(t0v17, c7));\
    t0v19 = _m_por(_m_pand(t0v16, c6), _m_pslldi(_m_pand(t0v18, c6), 2));\
    t0v16 = _m_por(_m_psrldi(_m_pand(t0v16, c7), 2), _m_pand(t0v18, c7));\
    t0v18 = _m_por(_m_pand(t0v22, c6), _m_pslldi(_m_pand(t0v5, c6), 2));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v22, c7), 2), _m_pand(t0v5, c7));\
    t0v22 = _m_por(_m_pand(t0v4, c6), _m_pslldi(_m_pand(t0v6, c6), 2));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v4, c7), 2), _m_pand(t0v6, c7));\
    t0v6 = _m_por(_m_pand(t0v0, c6), _m_pslldi(_m_pand(t0v2, c6), 2));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c7), 2), _m_pand(t0v2, c7));\
    t0v2 = _m_por(_m_pand(t0v1, c6), _m_pslldi(_m_pand(t0v3, c6), 2));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c7), 2), _m_pand(t0v3, c7));\
    (D0) = _m_por(_m_pand(t0v7, c8), _m_pslldi(_m_pand(t0v12, c8), 1));\
    (D1) = _m_por(_m_psrldi(_m_pand(t0v7, c9), 1), _m_pand(t0v12, c9));\
    (D2) = _m_por(_m_pand(t0v8, c8), _m_pslldi(_m_pand(t0v10, c8), 1));\
    (D3) = _m_por(_m_psrldi(_m_pand(t0v8, c9), 1), _m_pand(t0v10, c9));\
    (D4) = _m_por(_m_pand(t0v14, c8), _m_pslldi(_m_pand(t0v13, c8), 1));\
    (D5) = _m_por(_m_psrldi(_m_pand(t0v14, c9), 1), _m_pand(t0v13, c9));\
    (D6) = _m_por(_m_pand(t0v9, c8), _m_pslldi(_m_pand(t0v11, c8), 1));\
    (D7) = _m_por(_m_psrldi(_m_pand(t0v9, c9), 1), _m_pand(t0v11, c9));\
    (D8) = _m_por(_m_pand(t0v15, c8), _m_pslldi(_m_pand(t0v24, c8), 1));\
    (D9) = _m_por(_m_psrldi(_m_pand(t0v15, c9), 1), _m_pand(t0v24, c9));\
    (D10) = _m_por(_m_pand(t0v20, c8), _m_pslldi(_m_pand(t0v21, c8), 1));\
    (D11) = _m_por(_m_psrldi(_m_pand(t0v20, c9), 1), _m_pand(t0v21, c9));\
    (D12) = _m_por(_m_pand(t0v23, c8), _m_pslldi(_m_pand(t0v19, c8), 1));\
    (D13) = _m_por(_m_psrldi(_m_pand(t0v23, c9), 1), _m_pand(t0v19, c9));\
    (D14) = _m_por(_m_pand(t0v17, c8), _m_pslldi(_m_pand(t0v16, c8), 1));\
    (D15) = _m_por(_m_psrldi(_m_pand(t0v17, c9), 1), _m_pand(t0v16, c9));\
    (D16) = _m_por(_m_pand(t0v18, c8), _m_pslldi(_m_pand(t0v22, c8), 1));\
    (D17) = _m_por(_m_psrldi(_m_pand(t0v18, c9), 1), _m_pand(t0v22, c9));\
    (D18) = _m_por(_m_pand(t0v5, c8), _m_pslldi(_m_pand(t0v4, c8), 1));\
    (D19) = _m_por(_m_psrldi(_m_pand(t0v5, c9), 1), _m_pand(t0v4, c9));\
    (D20) = _m_por(_m_pand(t0v6, c8), _m_pslldi(_m_pand(t0v2, c8), 1));\
    (D21) = _m_por(_m_psrldi(_m_pand(t0v6, c9), 1), _m_pand(t0v2, c9));\
    (D22) = _m_por(_m_pand(t0v0, c8), _m_pslldi(_m_pand(t0v1, c8), 1));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_MMX.transform();
    transform.gen_input_transform(7);
    assert_eq!(
        r##"#define IN_TRANSFORM_B7(D0, D1, D2, D3, D4, D5, D6, S) \
{\
    const __m64 c0 = (*(const __m64*)(transform_const2_tbl + 2*3));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*5));\
    __m64 t0v0;\
    __m64 t0v1;\
    __m64 t0v2;\
    __m64 t0v3;\
    __m64 t0v4;\
    __m64 t0v5;\
    __m64 t0v6;\
    __m64 t0v7;\
    __m64 t0v8;\
    __m64 t0v9;\
    __m64 t0v10;\
    __m64 t0v11;\
    __m64 t0v12;\
    __m64 t0v13;\
    __m64 t0v14;\
    __m64 t0v15;\
    __m64 t0v16;\
    __m64 t0v17;\
    __m64 t0v18;\
    __m64 t0v19;\
    __m64 t0v20;\
    __m64 t0v21;\
    __m64 t0v22;\
    __m64 t0v23;\
    __m64 t0v24;\
    __m64 t0v25;\
    __m64 t0v26;\
    __m64 t0v27;\
    __m64 t0v28;\
    __m64 t0v29;\
    __m64 t0v30;\
    __m64 t0v31;\
    __m64 t0v32;\
    t0v0 = _m_punpckldq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v1 = _m_punpckhdq((*((const __m64*)(((S) + 0)))), (*((const __m64*)(((S) + 32)))));\
    t0v2 = _m_punpckldq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v3 = _m_punpckhdq((*((const __m64*)(((S) + 2)))), (*((const __m64*)(((S) + 34)))));\
    t0v4 = _m_punpckldq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v5 = _m_punpckhdq((*((const __m64*)(((S) + 4)))), (*((const __m64*)(((S) + 36)))));\
    t0v6 = _m_punpckldq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v7 = _m_punpckhdq((*((const __m64*)(((S) + 6)))), (*((const __m64*)(((S) + 38)))));\
    t0v8 = _m_punpckldq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v9 = _m_punpckhdq((*((const __m64*)(((S) + 8)))), (*((const __m64*)(((S) + 40)))));\
    t0v10 = _m_punpckldq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v11 = _m_punpckhdq((*((const __m64*)(((S) + 10)))), (*((const __m64*)(((S) + 42)))));\
    t0v12 = _m_punpckldq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v13 = _m_punpckhdq((*((const __m64*)(((S) + 12)))), (*((const __m64*)(((S) + 44)))));\
    t0v14 = _m_punpckldq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v15 = _m_punpckhdq((*((const __m64*)(((S) + 14)))), (*((const __m64*)(((S) + 46)))));\
    t0v16 = _m_punpckldq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v17 = _m_punpckhdq((*((const __m64*)(((S) + 16)))), (*((const __m64*)(((S) + 48)))));\
    t0v18 = _m_punpckldq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v19 = _m_punpckhdq((*((const __m64*)(((S) + 18)))), (*((const __m64*)(((S) + 50)))));\
    t0v20 = _m_punpckldq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v21 = _m_punpckhdq((*((const __m64*)(((S) + 20)))), (*((const __m64*)(((S) + 52)))));\
    t0v22 = _m_punpckldq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v23 = _m_punpckhdq((*((const __m64*)(((S) + 22)))), (*((const __m64*)(((S) + 54)))));\
    t0v24 = _m_punpckldq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v25 = _m_punpckhdq((*((const __m64*)(((S) + 24)))), (*((const __m64*)(((S) + 56)))));\
    t0v26 = _m_punpckldq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v27 = _m_punpckhdq((*((const __m64*)(((S) + 26)))), (*((const __m64*)(((S) + 58)))));\
    t0v28 = _m_punpckldq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v29 = _m_punpckhdq((*((const __m64*)(((S) + 28)))), (*((const __m64*)(((S) + 60)))));\
    t0v30 = _m_punpckldq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v31 = _m_punpckhdq((*((const __m64*)(((S) + 30)))), (*((const __m64*)(((S) + 62)))));\
    t0v32 = _m_por(_m_por(_m_por(_m_pand(t0v0, c0), _m_pslldi(_m_pand(t0v8, c0), 8)), _m_pslldi(_m_pand(t0v16, c0), 16)), _m_pslldi(t0v24, 24));\
    t0v0 = _m_por(_m_por(_m_por(_m_pand(t0v1, c0), _m_pslldi(_m_pand(t0v9, c0), 8)), _m_pslldi(_m_pand(t0v17, c0), 16)), _m_pslldi(t0v25, 24));\
    t0v1 = _m_por(_m_por(_m_por(_m_pand(t0v2, c0), _m_pslldi(_m_pand(t0v10, c0), 8)), _m_pslldi(_m_pand(t0v18, c0), 16)), _m_pslldi(t0v26, 24));\
    t0v2 = _m_por(_m_por(_m_por(_m_pand(t0v3, c0), _m_pslldi(_m_pand(t0v11, c0), 8)), _m_pslldi(_m_pand(t0v19, c0), 16)), _m_pslldi(t0v27, 24));\
    t0v3 = _m_por(_m_por(_m_por(_m_pand(t0v4, c0), _m_pslldi(_m_pand(t0v12, c0), 8)), _m_pslldi(_m_pand(t0v20, c0), 16)), _m_pslldi(t0v28, 24));\
    t0v4 = _m_por(_m_por(_m_por(_m_pand(t0v5, c0), _m_pslldi(_m_pand(t0v13, c0), 8)), _m_pslldi(_m_pand(t0v21, c0), 16)), _m_pslldi(t0v29, 24));\
    t0v5 = _m_por(_m_por(_m_por(_m_pand(t0v6, c0), _m_pslldi(_m_pand(t0v14, c0), 8)), _m_pslldi(_m_pand(t0v22, c0), 16)), _m_pslldi(t0v30, 24));\
    t0v6 = _m_por(_m_por(_m_por(_m_pand(t0v7, c0), _m_pslldi(_m_pand(t0v15, c0), 8)), _m_pslldi(_m_pand(t0v23, c0), 16)), _m_pslldi(t0v31, 24));\
    t0v7 = _m_por(_m_pand(t0v32, c1), _m_pslldi(_m_pand(t0v3, c1), 4));\
    t0v3 = _m_por(_m_psrldi(_m_pand(t0v32, c2), 4), _m_pand(t0v3, c2));\
    t0v8 = _m_por(_m_pand(t0v0, c1), _m_pslldi(_m_pand(t0v4, c1), 4));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c2), 4), _m_pand(t0v4, c2));\
    t0v4 = _m_por(_m_pand(t0v1, c1), _m_pslldi(_m_pand(t0v5, c1), 4));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v1, c2), 4), _m_pand(t0v5, c2));\
    t0v5 = _m_por(_m_pand(t0v2, c1), _m_pslldi(_m_pand(t0v6, c1), 4));\
    t0v2 = _m_por(_m_psrldi(_m_pand(t0v2, c2), 4), _m_pand(t0v6, c2));\
    t0v6 = _m_por(_m_pand(t0v7, c3), _m_pslldi(_m_pand(t0v4, c3), 2));\
    t0v4 = _m_por(_m_psrldi(_m_pand(t0v7, c4), 2), _m_pand(t0v4, c4));\
    t0v7 = _m_por(_m_pand(t0v8, c3), _m_pslldi(_m_pand(t0v5, c3), 2));\
    t0v5 = _m_por(_m_psrldi(_m_pand(t0v8, c4), 2), _m_pand(t0v5, c4));\
    t0v8 = _m_por(_m_pand(t0v3, c3), _m_pslldi(_m_pand(t0v1, c3), 2));\
    t0v1 = _m_por(_m_psrldi(_m_pand(t0v3, c4), 2), _m_pand(t0v1, c4));\
    t0v3 = _m_por(_m_pand(t0v0, c3), _m_pslldi(_m_pand(t0v2, c3), 2));\
    t0v0 = _m_por(_m_psrldi(_m_pand(t0v0, c4), 2), _m_pand(t0v2, c4));\
    (D0) = _m_por(_m_pand(t0v6, c5), _m_pslldi(_m_pand(t0v7, c5), 1));\
    (D1) = _m_por(_m_psrldi(_m_pand(t0v6, c6), 1), _m_pand(t0v7, c6));\
    (D2) = _m_por(_m_pand(t0v4, c5), _m_pslldi(_m_pand(t0v5, c5), 1));\
    (D3) = _m_por(_m_psrldi(_m_pand(t0v4, c6), 1), _m_pand(t0v5, c6));\
    (D4) = _m_por(_m_pand(t0v8, c5), _m_pslldi(_m_pand(t0v3, c5), 1));\
    (D5) = _m_por(_m_psrldi(_m_pand(t0v8, c6), 1), _m_pand(t0v3, c6));\
    (D6) = _m_por(_m_pand(t0v1, c5), _m_pslldi(_m_pand(t0v0, c5), 1));\
}
"##,
        transform.out()
    );
}
