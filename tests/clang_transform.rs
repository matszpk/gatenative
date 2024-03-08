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
}
