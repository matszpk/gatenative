use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_in_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
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
    t0v0 = ((((S)[0]) & 0x0000ffffU) | (((S)[16]) << 16));\
    t0v1 = ((((S)[0]) >> 16) | (((S)[16]) & 0xffff0000U));\
    t0v2 = ((((S)[1]) & 0x0000ffffU) | (((S)[17]) << 16));\
    t0v3 = ((((S)[1]) >> 16) | (((S)[17]) & 0xffff0000U));\
    t0v4 = ((((S)[2]) & 0x0000ffffU) | (((S)[18]) << 16));\
    t0v5 = ((((S)[2]) >> 16) | (((S)[18]) & 0xffff0000U));\
    t0v6 = ((((S)[3]) & 0x0000ffffU) | (((S)[19]) << 16));\
    t0v7 = ((((S)[3]) >> 16) | (((S)[19]) & 0xffff0000U));\
    t0v8 = ((((S)[4]) & 0x0000ffffU) | (((S)[20]) << 16));\
    t0v9 = ((((S)[4]) >> 16) | (((S)[20]) & 0xffff0000U));\
    t0v10 = ((((S)[5]) & 0x0000ffffU) | (((S)[21]) << 16));\
    t0v11 = ((((S)[5]) >> 16) | (((S)[21]) & 0xffff0000U));\
    t0v12 = ((((S)[6]) & 0x0000ffffU) | (((S)[22]) << 16));\
    t0v13 = ((((S)[6]) >> 16) | (((S)[22]) & 0xffff0000U));\
    t0v14 = ((((S)[7]) & 0x0000ffffU) | (((S)[23]) << 16));\
    t0v15 = ((((S)[7]) >> 16) | (((S)[23]) & 0xffff0000U));\
    t0v16 = ((((S)[8]) & 0x0000ffffU) | (((S)[24]) << 16));\
    t0v17 = ((((S)[8]) >> 16) | (((S)[24]) & 0xffff0000U));\
    t0v18 = ((((S)[9]) & 0x0000ffffU) | (((S)[25]) << 16));\
    t0v19 = ((((S)[9]) >> 16) | (((S)[25]) & 0xffff0000U));\
    t0v20 = ((((S)[10]) & 0x0000ffffU) | (((S)[26]) << 16));\
    t0v21 = ((((S)[10]) >> 16) | (((S)[26]) & 0xffff0000U));\
    t0v22 = ((((S)[11]) & 0x0000ffffU) | (((S)[27]) << 16));\
    t0v23 = ((((S)[11]) >> 16) | (((S)[27]) & 0xffff0000U));\
    t0v24 = ((((S)[12]) & 0x0000ffffU) | (((S)[28]) << 16));\
    t0v25 = ((((S)[12]) >> 16) | (((S)[28]) & 0xffff0000U));\
    t0v26 = ((((S)[13]) & 0x0000ffffU) | (((S)[29]) << 16));\
    t0v27 = ((((S)[13]) >> 16) | (((S)[29]) & 0xffff0000U));\
    t0v28 = ((((S)[14]) & 0x0000ffffU) | (((S)[30]) << 16));\
    t0v29 = ((((S)[14]) >> 16) | (((S)[30]) & 0xffff0000U));\
    t0v30 = ((((S)[15]) & 0x0000ffffU) | (((S)[31]) << 16));\
    t0v31 = ((((S)[15]) >> 16) | (((S)[31]) & 0xffff0000U));\
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
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
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
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
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
    t0v0 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v1 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    (D0) = ((t0v7 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D2) = ((t0v5 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D3) = (((t0v5 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D4) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
    (D5) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v1 & 0xaaaaaaaaU));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    (D0) = ((((((((((((((((((((((((((((((((((S)[0]) & 0x1U) | ((((S)[1]) & 0x1U) << 1)) | ((((S)[2]) & 0x1U) << 2)) | ((((S)[3]) & 0x1U) << 3)) | ((((S)[4]) & 0x1U) << 4)) | ((((S)[5]) & 0x1U) << 5)) | ((((S)[6]) & 0x1U) << 6)) | ((((S)[7]) & 0x1U) << 7)) | ((((S)[8]) & 0x1U) << 8)) | ((((S)[9]) & 0x1U) << 9)) | ((((S)[10]) & 0x1U) << 10)) | ((((S)[11]) & 0x1U) << 11)) | ((((S)[12]) & 0x1U) << 12)) | ((((S)[13]) & 0x1U) << 13)) | ((((S)[14]) & 0x1U) << 14)) | ((((S)[15]) & 0x1U) << 15)) | ((((S)[16]) & 0x1U) << 16)) | ((((S)[17]) & 0x1U) << 17)) | ((((S)[18]) & 0x1U) << 18)) | ((((S)[19]) & 0x1U) << 19)) | ((((S)[20]) & 0x1U) << 20)) | ((((S)[21]) & 0x1U) << 21)) | ((((S)[22]) & 0x1U) << 22)) | ((((S)[23]) & 0x1U) << 23)) | ((((S)[24]) & 0x1U) << 24)) | ((((S)[25]) & 0x1U) << 25)) | ((((S)[26]) & 0x1U) << 26)) | ((((S)[27]) & 0x1U) << 27)) | ((((S)[28]) & 0x1U) << 28)) | ((((S)[29]) & 0x1U) << 29)) | ((((S)[30]) & 0x1U) << 30)) | (((S)[31]) << 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_U64.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
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
    t0v0 = (((*((const uint64_t*)(((S) + 0)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 32)))) << 32));\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL));\
    t0v2 = (((*((const uint64_t*)(((S) + 2)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 34)))) << 32));\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL));\
    t0v4 = (((*((const uint64_t*)(((S) + 4)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 36)))) << 32));\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL));\
    t0v6 = (((*((const uint64_t*)(((S) + 6)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 38)))) << 32));\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL));\
    t0v8 = (((*((const uint64_t*)(((S) + 8)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 40)))) << 32));\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL));\
    t0v10 = (((*((const uint64_t*)(((S) + 10)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 42)))) << 32));\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL));\
    t0v12 = (((*((const uint64_t*)(((S) + 12)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 44)))) << 32));\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL));\
    t0v14 = (((*((const uint64_t*)(((S) + 14)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 46)))) << 32));\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL));\
    t0v16 = (((*((const uint64_t*)(((S) + 16)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 48)))) << 32));\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL));\
    t0v18 = (((*((const uint64_t*)(((S) + 18)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 50)))) << 32));\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL));\
    t0v20 = (((*((const uint64_t*)(((S) + 20)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 52)))) << 32));\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL));\
    t0v22 = (((*((const uint64_t*)(((S) + 22)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 54)))) << 32));\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL));\
    t0v24 = (((*((const uint64_t*)(((S) + 24)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 56)))) << 32));\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL));\
    t0v26 = (((*((const uint64_t*)(((S) + 26)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 58)))) << 32));\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL));\
    t0v28 = (((*((const uint64_t*)(((S) + 28)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 60)))) << 32));\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL));\
    t0v30 = (((*((const uint64_t*)(((S) + 30)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 62)))) << 32));\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL));\
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
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
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
    t0v0 = (((*((const uint64_t*)(((S) + 0)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 32)))) << 32));\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL));\
    t0v2 = (((*((const uint64_t*)(((S) + 2)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 34)))) << 32));\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL));\
    t0v4 = (((*((const uint64_t*)(((S) + 4)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 36)))) << 32));\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL));\
    t0v6 = (((*((const uint64_t*)(((S) + 6)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 38)))) << 32));\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL));\
    t0v8 = (((*((const uint64_t*)(((S) + 8)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 40)))) << 32));\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL));\
    t0v10 = (((*((const uint64_t*)(((S) + 10)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 42)))) << 32));\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL));\
    t0v12 = (((*((const uint64_t*)(((S) + 12)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 44)))) << 32));\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL));\
    t0v14 = (((*((const uint64_t*)(((S) + 14)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 46)))) << 32));\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL));\
    t0v16 = (((*((const uint64_t*)(((S) + 16)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 48)))) << 32));\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL));\
    t0v18 = (((*((const uint64_t*)(((S) + 18)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 50)))) << 32));\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL));\
    t0v20 = (((*((const uint64_t*)(((S) + 20)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 52)))) << 32));\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL));\
    t0v22 = (((*((const uint64_t*)(((S) + 22)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 54)))) << 32));\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL));\
    t0v24 = (((*((const uint64_t*)(((S) + 24)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 56)))) << 32));\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL));\
    t0v26 = (((*((const uint64_t*)(((S) + 26)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 58)))) << 32));\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL));\
    t0v28 = (((*((const uint64_t*)(((S) + 28)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 60)))) << 32));\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL));\
    t0v30 = (((*((const uint64_t*)(((S) + 30)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 62)))) << 32));\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL));\
    t0v0 = ((t0v0 & 0x0000ffff0000ffffULL) | ((t0v16 & 0x0000ffff0000ffffULL) << 16));\
    t0v1 = ((t0v1 & 0x0000ffff0000ffffULL) | ((t0v17 & 0x0000ffff0000ffffULL) << 16));\
    t0v2 = ((t0v2 & 0x0000ffff0000ffffULL) | ((t0v18 & 0x0000ffff0000ffffULL) << 16));\
    t0v3 = ((t0v3 & 0x0000ffff0000ffffULL) | ((t0v19 & 0x0000ffff0000ffffULL) << 16));\
    t0v4 = ((t0v4 & 0x0000ffff0000ffffULL) | ((t0v20 & 0x0000ffff0000ffffULL) << 16));\
    t0v5 = ((t0v5 & 0x0000ffff0000ffffULL) | ((t0v21 & 0x0000ffff0000ffffULL) << 16));\
    t0v6 = ((t0v6 & 0x0000ffff0000ffffULL) | ((t0v22 & 0x0000ffff0000ffffULL) << 16));\
    t0v7 = ((t0v7 & 0x0000ffff0000ffffULL) | ((t0v23 & 0x0000ffff0000ffffULL) << 16));\
    t0v8 = ((t0v8 & 0x0000ffff0000ffffULL) | ((t0v24 & 0x0000ffff0000ffffULL) << 16));\
    t0v9 = ((t0v9 & 0x0000ffff0000ffffULL) | ((t0v25 & 0x0000ffff0000ffffULL) << 16));\
    t0v10 = ((t0v10 & 0x0000ffff0000ffffULL) | ((t0v26 & 0x0000ffff0000ffffULL) << 16));\
    t0v11 = ((t0v11 & 0x0000ffff0000ffffULL) | ((t0v27 & 0x0000ffff0000ffffULL) << 16));\
    t0v12 = ((t0v12 & 0x0000ffff0000ffffULL) | ((t0v28 & 0x0000ffff0000ffffULL) << 16));\
    t0v13 = ((t0v13 & 0x0000ffff0000ffffULL) | ((t0v29 & 0x0000ffff0000ffffULL) << 16));\
    t0v14 = ((t0v14 & 0x0000ffff0000ffffULL) | ((t0v30 & 0x0000ffff0000ffffULL) << 16));\
    t0v15 = ((t0v15 & 0x0000ffff0000ffffULL) | ((t0v31 & 0x0000ffff0000ffffULL) << 16));\
    t0v16 = ((t0v0 & 0x00ff00ff00ff00ffULL) | ((t0v8 & 0x00ff00ff00ff00ffULL) << 8));\
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
    t0v15 = ((t0v16 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v11 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v11 = (((t0v16 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v11 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v16 = ((t0v8 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v12 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
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
    t0v7 = ((t0v15 & 0x3333333333333333ULL) | ((t0v12 & 0x3333333333333333ULL) << 2));\
    t0v12 = (((t0v15 & 0xccccccccccccccccULL) >> 2) | (t0v12 & 0xccccccccccccccccULL));\
    t0v15 = ((t0v16 & 0x3333333333333333ULL) | ((t0v13 & 0x3333333333333333ULL) << 2));\
    t0v13 = (((t0v16 & 0xccccccccccccccccULL) >> 2) | (t0v13 & 0xccccccccccccccccULL));\
    t0v16 = ((t0v11 & 0x3333333333333333ULL) | ((t0v9 & 0x3333333333333333ULL) << 2));\
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
    (D0) = ((t0v7 & 0x5555555555555555ULL) | ((t0v15 & 0x5555555555555555ULL) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v15 & 0xaaaaaaaaaaaaaaaaULL));\
    (D2) = ((t0v12 & 0x5555555555555555ULL) | ((t0v13 & 0x5555555555555555ULL) << 1));\
    (D3) = (((t0v12 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v13 & 0xaaaaaaaaaaaaaaaaULL));\
    (D4) = ((t0v16 & 0x5555555555555555ULL) | ((t0v11 & 0x5555555555555555ULL) << 1));\
    (D5) = (((t0v16 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v11 & 0xaaaaaaaaaaaaaaaaULL));\
    (D6) = ((t0v9 & 0x5555555555555555ULL) | ((t0v8 & 0x5555555555555555ULL) << 1));\
    (D7) = (((t0v9 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v8 & 0xaaaaaaaaaaaaaaaaULL));\
    (D8) = ((t0v10 & 0x5555555555555555ULL) | ((t0v14 & 0x5555555555555555ULL) << 1));\
    (D9) = (((t0v10 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v14 & 0xaaaaaaaaaaaaaaaaULL));\
    (D10) = ((t0v5 & 0x5555555555555555ULL) | ((t0v4 & 0x5555555555555555ULL) << 1));\
    (D11) = (((t0v5 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v4 & 0xaaaaaaaaaaaaaaaaULL));\
    (D12) = ((t0v6 & 0x5555555555555555ULL) | ((t0v2 & 0x5555555555555555ULL) << 1));\
    (D13) = (((t0v6 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v2 & 0xaaaaaaaaaaaaaaaaULL));\
    (D14) = ((t0v0 & 0x5555555555555555ULL) | ((t0v1 & 0x5555555555555555ULL) << 1));\
    (D15) = (((t0v0 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v1 & 0xaaaaaaaaaaaaaaaaULL));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
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
    t0v0 = (((*((const uint64_t*)(((S) + 0)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 32)))) << 32));\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL));\
    t0v2 = (((*((const uint64_t*)(((S) + 2)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 34)))) << 32));\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL));\
    t0v4 = (((*((const uint64_t*)(((S) + 4)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 36)))) << 32));\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL));\
    t0v6 = (((*((const uint64_t*)(((S) + 6)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 38)))) << 32));\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL));\
    t0v8 = (((*((const uint64_t*)(((S) + 8)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 40)))) << 32));\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL));\
    t0v10 = (((*((const uint64_t*)(((S) + 10)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 42)))) << 32));\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL));\
    t0v12 = (((*((const uint64_t*)(((S) + 12)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 44)))) << 32));\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL));\
    t0v14 = (((*((const uint64_t*)(((S) + 14)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 46)))) << 32));\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL));\
    t0v16 = (((*((const uint64_t*)(((S) + 16)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 48)))) << 32));\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL));\
    t0v18 = (((*((const uint64_t*)(((S) + 18)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 50)))) << 32));\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL));\
    t0v20 = (((*((const uint64_t*)(((S) + 20)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 52)))) << 32));\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL));\
    t0v22 = (((*((const uint64_t*)(((S) + 22)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 54)))) << 32));\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL));\
    t0v24 = (((*((const uint64_t*)(((S) + 24)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 56)))) << 32));\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL));\
    t0v26 = (((*((const uint64_t*)(((S) + 26)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 58)))) << 32));\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL));\
    t0v28 = (((*((const uint64_t*)(((S) + 28)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 60)))) << 32));\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL));\
    t0v30 = (((*((const uint64_t*)(((S) + 30)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 62)))) << 32));\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL));\
    t0v0 = ((((t0v0 & 0x000000ff000000ffULL) | ((t0v8 & 0x000000ff000000ffULL) << 8)) | ((t0v16 & 0x000000ff000000ffULL) << 16)) | ((t0v24 & 0x000000ff000000ffULL) << 24));\
    t0v1 = ((((t0v1 & 0x000000ff000000ffULL) | ((t0v9 & 0x000000ff000000ffULL) << 8)) | ((t0v17 & 0x000000ff000000ffULL) << 16)) | ((t0v25 & 0x000000ff000000ffULL) << 24));\
    t0v2 = ((((t0v2 & 0x000000ff000000ffULL) | ((t0v10 & 0x000000ff000000ffULL) << 8)) | ((t0v18 & 0x000000ff000000ffULL) << 16)) | ((t0v26 & 0x000000ff000000ffULL) << 24));\
    t0v3 = ((((t0v3 & 0x000000ff000000ffULL) | ((t0v11 & 0x000000ff000000ffULL) << 8)) | ((t0v19 & 0x000000ff000000ffULL) << 16)) | ((t0v27 & 0x000000ff000000ffULL) << 24));\
    t0v4 = ((((t0v4 & 0x000000ff000000ffULL) | ((t0v12 & 0x000000ff000000ffULL) << 8)) | ((t0v20 & 0x000000ff000000ffULL) << 16)) | ((t0v28 & 0x000000ff000000ffULL) << 24));\
    t0v5 = ((((t0v5 & 0x000000ff000000ffULL) | ((t0v13 & 0x000000ff000000ffULL) << 8)) | ((t0v21 & 0x000000ff000000ffULL) << 16)) | ((t0v29 & 0x000000ff000000ffULL) << 24));\
    t0v6 = ((((t0v6 & 0x000000ff000000ffULL) | ((t0v14 & 0x000000ff000000ffULL) << 8)) | ((t0v22 & 0x000000ff000000ffULL) << 16)) | ((t0v30 & 0x000000ff000000ffULL) << 24));\
    t0v7 = ((((t0v7 & 0x000000ff000000ffULL) | ((t0v15 & 0x000000ff000000ffULL) << 8)) | ((t0v23 & 0x000000ff000000ffULL) << 16)) | ((t0v31 & 0x000000ff000000ffULL) << 24));\
    t0v8 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v3 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v7 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v7 = ((t0v8 & 0x3333333333333333ULL) | ((t0v5 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v5 & 0xccccccccccccccccULL));\
    t0v8 = ((t0v4 & 0x3333333333333333ULL) | ((t0v6 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v4 & 0xccccccccccccccccULL) >> 2) | (t0v6 & 0xccccccccccccccccULL));\
    t0v0 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v1 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    (D0) = ((t0v7 & 0x5555555555555555ULL) | ((t0v8 & 0x5555555555555555ULL) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v8 & 0xaaaaaaaaaaaaaaaaULL));\
    (D2) = ((t0v5 & 0x5555555555555555ULL) | ((t0v4 & 0x5555555555555555ULL) << 1));\
    (D3) = (((t0v5 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v4 & 0xaaaaaaaaaaaaaaaaULL));\
    (D4) = ((t0v0 & 0x5555555555555555ULL) | ((t0v1 & 0x5555555555555555ULL) << 1));\
    (D5) = (((t0v0 & 0xaaaaaaaaaaaaaaaaULL) >> 1) | (t0v1 & 0xaaaaaaaaaaaaaaaaULL));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
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
    t0v0 = (((*((const uint64_t*)(((S) + 0)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 32)))) << 32));\
    t0v1 = (((*((const uint64_t*)(((S) + 0)))) >> 32) | ((*((const uint64_t*)(((S) + 32)))) & 0xffffffff00000000ULL));\
    t0v2 = (((*((const uint64_t*)(((S) + 2)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 34)))) << 32));\
    t0v3 = (((*((const uint64_t*)(((S) + 2)))) >> 32) | ((*((const uint64_t*)(((S) + 34)))) & 0xffffffff00000000ULL));\
    t0v4 = (((*((const uint64_t*)(((S) + 4)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 36)))) << 32));\
    t0v5 = (((*((const uint64_t*)(((S) + 4)))) >> 32) | ((*((const uint64_t*)(((S) + 36)))) & 0xffffffff00000000ULL));\
    t0v6 = (((*((const uint64_t*)(((S) + 6)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 38)))) << 32));\
    t0v7 = (((*((const uint64_t*)(((S) + 6)))) >> 32) | ((*((const uint64_t*)(((S) + 38)))) & 0xffffffff00000000ULL));\
    t0v8 = (((*((const uint64_t*)(((S) + 8)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 40)))) << 32));\
    t0v9 = (((*((const uint64_t*)(((S) + 8)))) >> 32) | ((*((const uint64_t*)(((S) + 40)))) & 0xffffffff00000000ULL));\
    t0v10 = (((*((const uint64_t*)(((S) + 10)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 42)))) << 32));\
    t0v11 = (((*((const uint64_t*)(((S) + 10)))) >> 32) | ((*((const uint64_t*)(((S) + 42)))) & 0xffffffff00000000ULL));\
    t0v12 = (((*((const uint64_t*)(((S) + 12)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 44)))) << 32));\
    t0v13 = (((*((const uint64_t*)(((S) + 12)))) >> 32) | ((*((const uint64_t*)(((S) + 44)))) & 0xffffffff00000000ULL));\
    t0v14 = (((*((const uint64_t*)(((S) + 14)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 46)))) << 32));\
    t0v15 = (((*((const uint64_t*)(((S) + 14)))) >> 32) | ((*((const uint64_t*)(((S) + 46)))) & 0xffffffff00000000ULL));\
    t0v16 = (((*((const uint64_t*)(((S) + 16)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 48)))) << 32));\
    t0v17 = (((*((const uint64_t*)(((S) + 16)))) >> 32) | ((*((const uint64_t*)(((S) + 48)))) & 0xffffffff00000000ULL));\
    t0v18 = (((*((const uint64_t*)(((S) + 18)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 50)))) << 32));\
    t0v19 = (((*((const uint64_t*)(((S) + 18)))) >> 32) | ((*((const uint64_t*)(((S) + 50)))) & 0xffffffff00000000ULL));\
    t0v20 = (((*((const uint64_t*)(((S) + 20)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 52)))) << 32));\
    t0v21 = (((*((const uint64_t*)(((S) + 20)))) >> 32) | ((*((const uint64_t*)(((S) + 52)))) & 0xffffffff00000000ULL));\
    t0v22 = (((*((const uint64_t*)(((S) + 22)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 54)))) << 32));\
    t0v23 = (((*((const uint64_t*)(((S) + 22)))) >> 32) | ((*((const uint64_t*)(((S) + 54)))) & 0xffffffff00000000ULL));\
    t0v24 = (((*((const uint64_t*)(((S) + 24)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 56)))) << 32));\
    t0v25 = (((*((const uint64_t*)(((S) + 24)))) >> 32) | ((*((const uint64_t*)(((S) + 56)))) & 0xffffffff00000000ULL));\
    t0v26 = (((*((const uint64_t*)(((S) + 26)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 58)))) << 32));\
    t0v27 = (((*((const uint64_t*)(((S) + 26)))) >> 32) | ((*((const uint64_t*)(((S) + 58)))) & 0xffffffff00000000ULL));\
    t0v28 = (((*((const uint64_t*)(((S) + 28)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 60)))) << 32));\
    t0v29 = (((*((const uint64_t*)(((S) + 28)))) >> 32) | ((*((const uint64_t*)(((S) + 60)))) & 0xffffffff00000000ULL));\
    t0v30 = (((*((const uint64_t*)(((S) + 30)))) & 0x00000000ffffffffULL) | ((*((const uint64_t*)(((S) + 62)))) << 32));\
    t0v31 = (((*((const uint64_t*)(((S) + 30)))) >> 32) | ((*((const uint64_t*)(((S) + 62)))) & 0xffffffff00000000ULL));\
    (D0) = ((((((((((((((((((((((((((((((((t0v0 & 0x0000000100000001ULL) | ((t0v1 & 0x0000000100000001ULL) << 1)) | ((t0v2 & 0x0000000100000001ULL) << 2)) | ((t0v3 & 0x0000000100000001ULL) << 3)) | ((t0v4 & 0x0000000100000001ULL) << 4)) | ((t0v5 & 0x0000000100000001ULL) << 5)) | ((t0v6 & 0x0000000100000001ULL) << 6)) | ((t0v7 & 0x0000000100000001ULL) << 7)) | ((t0v8 & 0x0000000100000001ULL) << 8)) | ((t0v9 & 0x0000000100000001ULL) << 9)) | ((t0v10 & 0x0000000100000001ULL) << 10)) | ((t0v11 & 0x0000000100000001ULL) << 11)) | ((t0v12 & 0x0000000100000001ULL) << 12)) | ((t0v13 & 0x0000000100000001ULL) << 13)) | ((t0v14 & 0x0000000100000001ULL) << 14)) | ((t0v15 & 0x0000000100000001ULL) << 15)) | ((t0v16 & 0x0000000100000001ULL) << 16)) | ((t0v17 & 0x0000000100000001ULL) << 17)) | ((t0v18 & 0x0000000100000001ULL) << 18)) | ((t0v19 & 0x0000000100000001ULL) << 19)) | ((t0v20 & 0x0000000100000001ULL) << 20)) | ((t0v21 & 0x0000000100000001ULL) << 21)) | ((t0v22 & 0x0000000100000001ULL) << 22)) | ((t0v23 & 0x0000000100000001ULL) << 23)) | ((t0v24 & 0x0000000100000001ULL) << 24)) | ((t0v25 & 0x0000000100000001ULL) << 25)) | ((t0v26 & 0x0000000100000001ULL) << 26)) | ((t0v27 & 0x0000000100000001ULL) << 27)) | ((t0v28 & 0x0000000100000001ULL) << 28)) | ((t0v29 & 0x0000000100000001ULL) << 29)) | ((t0v30 & 0x0000000100000001ULL) << 30)) | ((t0v31 & 0x0000000100000001ULL) << 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_MMX.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
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
    t0v32 = _m_por(_m_pand(t0v0, c0), _m_pslldi(t0v16, 16));\
    t0v0 = _m_por(_m_psrldi(t0v0, 16), _m_pand(t0v16, c1));\
    t0v16 = _m_por(_m_pand(t0v1, c0), _m_pslldi(t0v17, 16));\
    t0v1 = _m_por(_m_psrldi(t0v1, 16), _m_pand(t0v17, c1));\
    t0v17 = _m_por(_m_pand(t0v2, c0), _m_pslldi(t0v18, 16));\
    t0v2 = _m_por(_m_psrldi(t0v2, 16), _m_pand(t0v18, c1));\
    t0v18 = _m_por(_m_pand(t0v3, c0), _m_pslldi(t0v19, 16));\
    t0v3 = _m_por(_m_psrldi(t0v3, 16), _m_pand(t0v19, c1));\
    t0v19 = _m_por(_m_pand(t0v4, c0), _m_pslldi(t0v20, 16));\
    t0v4 = _m_por(_m_psrldi(t0v4, 16), _m_pand(t0v20, c1));\
    t0v20 = _m_por(_m_pand(t0v5, c0), _m_pslldi(t0v21, 16));\
    t0v5 = _m_por(_m_psrldi(t0v5, 16), _m_pand(t0v21, c1));\
    t0v21 = _m_por(_m_pand(t0v6, c0), _m_pslldi(t0v22, 16));\
    t0v6 = _m_por(_m_psrldi(t0v6, 16), _m_pand(t0v22, c1));\
    t0v22 = _m_por(_m_pand(t0v7, c0), _m_pslldi(t0v23, 16));\
    t0v7 = _m_por(_m_psrldi(t0v7, 16), _m_pand(t0v23, c1));\
    t0v23 = _m_por(_m_pand(t0v8, c0), _m_pslldi(t0v24, 16));\
    t0v8 = _m_por(_m_psrldi(t0v8, 16), _m_pand(t0v24, c1));\
    t0v24 = _m_por(_m_pand(t0v9, c0), _m_pslldi(t0v25, 16));\
    t0v9 = _m_por(_m_psrldi(t0v9, 16), _m_pand(t0v25, c1));\
    t0v25 = _m_por(_m_pand(t0v10, c0), _m_pslldi(t0v26, 16));\
    t0v10 = _m_por(_m_psrldi(t0v10, 16), _m_pand(t0v26, c1));\
    t0v26 = _m_por(_m_pand(t0v11, c0), _m_pslldi(t0v27, 16));\
    t0v11 = _m_por(_m_psrldi(t0v11, 16), _m_pand(t0v27, c1));\
    t0v27 = _m_por(_m_pand(t0v12, c0), _m_pslldi(t0v28, 16));\
    t0v12 = _m_por(_m_psrldi(t0v12, 16), _m_pand(t0v28, c1));\
    t0v28 = _m_por(_m_pand(t0v13, c0), _m_pslldi(t0v29, 16));\
    t0v13 = _m_por(_m_psrldi(t0v13, 16), _m_pand(t0v29, c1));\
    t0v29 = _m_por(_m_pand(t0v14, c0), _m_pslldi(t0v30, 16));\
    t0v14 = _m_por(_m_psrldi(t0v14, 16), _m_pand(t0v30, c1));\
    t0v30 = _m_por(_m_pand(t0v15, c0), _m_pslldi(t0v31, 16));\
    t0v15 = _m_por(_m_psrldi(t0v15, 16), _m_pand(t0v31, c1));\
    t0v31 = _m_por(_m_pand(t0v32, c2), _m_psllwi(t0v23, 8));\
    t0v23 = _m_por(_m_psrlwi(t0v32, 8), _m_pand(t0v23, c3));\
    t0v32 = _m_por(_m_pand(t0v16, c2), _m_psllwi(t0v24, 8));\
    t0v16 = _m_por(_m_psrlwi(t0v16, 8), _m_pand(t0v24, c3));\
    t0v24 = _m_por(_m_pand(t0v17, c2), _m_psllwi(t0v25, 8));\
    t0v17 = _m_por(_m_psrlwi(t0v17, 8), _m_pand(t0v25, c3));\
    t0v25 = _m_por(_m_pand(t0v18, c2), _m_psllwi(t0v26, 8));\
    t0v18 = _m_por(_m_psrlwi(t0v18, 8), _m_pand(t0v26, c3));\
    t0v26 = _m_por(_m_pand(t0v19, c2), _m_psllwi(t0v27, 8));\
    t0v19 = _m_por(_m_psrlwi(t0v19, 8), _m_pand(t0v27, c3));\
    t0v27 = _m_por(_m_pand(t0v20, c2), _m_psllwi(t0v28, 8));\
    t0v20 = _m_por(_m_psrlwi(t0v20, 8), _m_pand(t0v28, c3));\
    t0v28 = _m_por(_m_pand(t0v21, c2), _m_psllwi(t0v29, 8));\
    t0v21 = _m_por(_m_psrlwi(t0v21, 8), _m_pand(t0v29, c3));\
    t0v29 = _m_por(_m_pand(t0v22, c2), _m_psllwi(t0v30, 8));\
    t0v22 = _m_por(_m_psrlwi(t0v22, 8), _m_pand(t0v30, c3));\
    t0v30 = _m_por(_m_pand(t0v0, c2), _m_psllwi(t0v8, 8));\
    t0v0 = _m_por(_m_psrlwi(t0v0, 8), _m_pand(t0v8, c3));\
    t0v8 = _m_por(_m_pand(t0v1, c2), _m_psllwi(t0v9, 8));\
    t0v1 = _m_por(_m_psrlwi(t0v1, 8), _m_pand(t0v9, c3));\
    t0v9 = _m_por(_m_pand(t0v2, c2), _m_psllwi(t0v10, 8));\
    t0v2 = _m_por(_m_psrlwi(t0v2, 8), _m_pand(t0v10, c3));\
    t0v10 = _m_por(_m_pand(t0v3, c2), _m_psllwi(t0v11, 8));\
    t0v3 = _m_por(_m_psrlwi(t0v3, 8), _m_pand(t0v11, c3));\
    t0v11 = _m_por(_m_pand(t0v4, c2), _m_psllwi(t0v12, 8));\
    t0v4 = _m_por(_m_psrlwi(t0v4, 8), _m_pand(t0v12, c3));\
    t0v12 = _m_por(_m_pand(t0v5, c2), _m_psllwi(t0v13, 8));\
    t0v5 = _m_por(_m_psrlwi(t0v5, 8), _m_pand(t0v13, c3));\
    t0v13 = _m_por(_m_pand(t0v6, c2), _m_psllwi(t0v14, 8));\
    t0v6 = _m_por(_m_psrlwi(t0v6, 8), _m_pand(t0v14, c3));\
    t0v14 = _m_por(_m_pand(t0v7, c2), _m_psllwi(t0v15, 8));\
    t0v7 = _m_por(_m_psrlwi(t0v7, 8), _m_pand(t0v15, c3));\
    t0v15 = _m_por(_m_pand(t0v31, c4), _m_psllwi(_m_pand(t0v26, c4), 4));\
    t0v26 = _m_por(_m_psrlwi(_m_pand(t0v31, c5), 4), _m_pand(t0v26, c5));\
    t0v31 = _m_por(_m_pand(t0v32, c4), _m_psllwi(_m_pand(t0v27, c4), 4));\
    t0v27 = _m_por(_m_psrlwi(_m_pand(t0v32, c5), 4), _m_pand(t0v27, c5));\
    t0v32 = _m_por(_m_pand(t0v24, c4), _m_psllwi(_m_pand(t0v28, c4), 4));\
    t0v24 = _m_por(_m_psrlwi(_m_pand(t0v24, c5), 4), _m_pand(t0v28, c5));\
    t0v28 = _m_por(_m_pand(t0v25, c4), _m_psllwi(_m_pand(t0v29, c4), 4));\
    t0v25 = _m_por(_m_psrlwi(_m_pand(t0v25, c5), 4), _m_pand(t0v29, c5));\
    t0v29 = _m_por(_m_pand(t0v23, c4), _m_psllwi(_m_pand(t0v19, c4), 4));\
    t0v19 = _m_por(_m_psrlwi(_m_pand(t0v23, c5), 4), _m_pand(t0v19, c5));\
    t0v23 = _m_por(_m_pand(t0v16, c4), _m_psllwi(_m_pand(t0v20, c4), 4));\
    t0v16 = _m_por(_m_psrlwi(_m_pand(t0v16, c5), 4), _m_pand(t0v20, c5));\
    t0v20 = _m_por(_m_pand(t0v17, c4), _m_psllwi(_m_pand(t0v21, c4), 4));\
    t0v17 = _m_por(_m_psrlwi(_m_pand(t0v17, c5), 4), _m_pand(t0v21, c5));\
    t0v21 = _m_por(_m_pand(t0v18, c4), _m_psllwi(_m_pand(t0v22, c4), 4));\
    t0v18 = _m_por(_m_psrlwi(_m_pand(t0v18, c5), 4), _m_pand(t0v22, c5));\
    t0v22 = _m_por(_m_pand(t0v30, c4), _m_psllwi(_m_pand(t0v11, c4), 4));\
    t0v11 = _m_por(_m_psrlwi(_m_pand(t0v30, c5), 4), _m_pand(t0v11, c5));\
    t0v30 = _m_por(_m_pand(t0v8, c4), _m_psllwi(_m_pand(t0v12, c4), 4));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c5), 4), _m_pand(t0v12, c5));\
    t0v12 = _m_por(_m_pand(t0v9, c4), _m_psllwi(_m_pand(t0v13, c4), 4));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c5), 4), _m_pand(t0v13, c5));\
    t0v13 = _m_por(_m_pand(t0v10, c4), _m_psllwi(_m_pand(t0v14, c4), 4));\
    t0v10 = _m_por(_m_psrlwi(_m_pand(t0v10, c5), 4), _m_pand(t0v14, c5));\
    t0v14 = _m_por(_m_pand(t0v0, c4), _m_psllwi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_psllwi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_por(_m_pand(t0v2, c4), _m_psllwi(_m_pand(t0v6, c4), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c5), 4), _m_pand(t0v6, c5));\
    t0v6 = _m_por(_m_pand(t0v3, c4), _m_psllwi(_m_pand(t0v7, c4), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v3, c5), 4), _m_pand(t0v7, c5));\
    t0v7 = _m_por(_m_pand(t0v15, c6), _m_psllwi(_m_pand(t0v32, c6), 2));\
    t0v15 = _m_por(_m_psrlwi(_m_pand(t0v15, c7), 2), _m_pand(t0v32, c7));\
    t0v32 = _m_por(_m_pand(t0v31, c6), _m_psllwi(_m_pand(t0v28, c6), 2));\
    t0v28 = _m_por(_m_psrlwi(_m_pand(t0v31, c7), 2), _m_pand(t0v28, c7));\
    t0v31 = _m_por(_m_pand(t0v26, c6), _m_psllwi(_m_pand(t0v24, c6), 2));\
    t0v24 = _m_por(_m_psrlwi(_m_pand(t0v26, c7), 2), _m_pand(t0v24, c7));\
    t0v26 = _m_por(_m_pand(t0v27, c6), _m_psllwi(_m_pand(t0v25, c6), 2));\
    t0v25 = _m_por(_m_psrlwi(_m_pand(t0v27, c7), 2), _m_pand(t0v25, c7));\
    t0v27 = _m_por(_m_pand(t0v29, c6), _m_psllwi(_m_pand(t0v20, c6), 2));\
    t0v20 = _m_por(_m_psrlwi(_m_pand(t0v29, c7), 2), _m_pand(t0v20, c7));\
    t0v29 = _m_por(_m_pand(t0v23, c6), _m_psllwi(_m_pand(t0v21, c6), 2));\
    t0v21 = _m_por(_m_psrlwi(_m_pand(t0v23, c7), 2), _m_pand(t0v21, c7));\
    t0v23 = _m_por(_m_pand(t0v19, c6), _m_psllwi(_m_pand(t0v17, c6), 2));\
    t0v17 = _m_por(_m_psrlwi(_m_pand(t0v19, c7), 2), _m_pand(t0v17, c7));\
    t0v19 = _m_por(_m_pand(t0v16, c6), _m_psllwi(_m_pand(t0v18, c6), 2));\
    t0v16 = _m_por(_m_psrlwi(_m_pand(t0v16, c7), 2), _m_pand(t0v18, c7));\
    t0v18 = _m_por(_m_pand(t0v22, c6), _m_psllwi(_m_pand(t0v12, c6), 2));\
    t0v12 = _m_por(_m_psrlwi(_m_pand(t0v22, c7), 2), _m_pand(t0v12, c7));\
    t0v22 = _m_por(_m_pand(t0v30, c6), _m_psllwi(_m_pand(t0v13, c6), 2));\
    t0v13 = _m_por(_m_psrlwi(_m_pand(t0v30, c7), 2), _m_pand(t0v13, c7));\
    t0v30 = _m_por(_m_pand(t0v11, c6), _m_psllwi(_m_pand(t0v9, c6), 2));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v11, c7), 2), _m_pand(t0v9, c7));\
    t0v11 = _m_por(_m_pand(t0v8, c6), _m_psllwi(_m_pand(t0v10, c6), 2));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c7), 2), _m_pand(t0v10, c7));\
    t0v10 = _m_por(_m_pand(t0v14, c6), _m_psllwi(_m_pand(t0v5, c6), 2));\
    t0v5 = _m_por(_m_psrlwi(_m_pand(t0v14, c7), 2), _m_pand(t0v5, c7));\
    t0v14 = _m_por(_m_pand(t0v4, c6), _m_psllwi(_m_pand(t0v6, c6), 2));\
    t0v4 = _m_por(_m_psrlwi(_m_pand(t0v4, c7), 2), _m_pand(t0v6, c7));\
    t0v6 = _m_por(_m_pand(t0v0, c6), _m_psllwi(_m_pand(t0v2, c6), 2));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c7), 2), _m_pand(t0v2, c7));\
    t0v2 = _m_por(_m_pand(t0v1, c6), _m_psllwi(_m_pand(t0v3, c6), 2));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c7), 2), _m_pand(t0v3, c7));\
    (D0) = _m_por(_m_pand(t0v7, c8), _m_psllwi(_m_pand(t0v32, c8), 1));\
    (D1) = _m_por(_m_psrlwi(_m_pand(t0v7, c9), 1), _m_pand(t0v32, c9));\
    (D2) = _m_por(_m_pand(t0v15, c8), _m_psllwi(_m_pand(t0v28, c8), 1));\
    (D3) = _m_por(_m_psrlwi(_m_pand(t0v15, c9), 1), _m_pand(t0v28, c9));\
    (D4) = _m_por(_m_pand(t0v31, c8), _m_psllwi(_m_pand(t0v26, c8), 1));\
    (D5) = _m_por(_m_psrlwi(_m_pand(t0v31, c9), 1), _m_pand(t0v26, c9));\
    (D6) = _m_por(_m_pand(t0v24, c8), _m_psllwi(_m_pand(t0v25, c8), 1));\
    (D7) = _m_por(_m_psrlwi(_m_pand(t0v24, c9), 1), _m_pand(t0v25, c9));\
    (D8) = _m_por(_m_pand(t0v27, c8), _m_psllwi(_m_pand(t0v29, c8), 1));\
    (D9) = _m_por(_m_psrlwi(_m_pand(t0v27, c9), 1), _m_pand(t0v29, c9));\
    (D10) = _m_por(_m_pand(t0v20, c8), _m_psllwi(_m_pand(t0v21, c8), 1));\
    (D11) = _m_por(_m_psrlwi(_m_pand(t0v20, c9), 1), _m_pand(t0v21, c9));\
    (D12) = _m_por(_m_pand(t0v23, c8), _m_psllwi(_m_pand(t0v19, c8), 1));\
    (D13) = _m_por(_m_psrlwi(_m_pand(t0v23, c9), 1), _m_pand(t0v19, c9));\
    (D14) = _m_por(_m_pand(t0v17, c8), _m_psllwi(_m_pand(t0v16, c8), 1));\
    (D15) = _m_por(_m_psrlwi(_m_pand(t0v17, c9), 1), _m_pand(t0v16, c9));\
    (D16) = _m_por(_m_pand(t0v18, c8), _m_psllwi(_m_pand(t0v22, c8), 1));\
    (D17) = _m_por(_m_psrlwi(_m_pand(t0v18, c9), 1), _m_pand(t0v22, c9));\
    (D18) = _m_por(_m_pand(t0v12, c8), _m_psllwi(_m_pand(t0v13, c8), 1));\
    (D19) = _m_por(_m_psrlwi(_m_pand(t0v12, c9), 1), _m_pand(t0v13, c9));\
    (D20) = _m_por(_m_pand(t0v30, c8), _m_psllwi(_m_pand(t0v11, c8), 1));\
    (D21) = _m_por(_m_psrlwi(_m_pand(t0v30, c9), 1), _m_pand(t0v11, c9));\
    (D22) = _m_por(_m_pand(t0v9, c8), _m_psllwi(_m_pand(t0v8, c8), 1));\
    (D23) = _m_por(_m_psrlwi(_m_pand(t0v9, c9), 1), _m_pand(t0v8, c9));\
    (D24) = _m_por(_m_pand(t0v10, c8), _m_psllwi(_m_pand(t0v14, c8), 1));\
    (D25) = _m_por(_m_psrlwi(_m_pand(t0v10, c9), 1), _m_pand(t0v14, c9));\
    (D26) = _m_por(_m_pand(t0v5, c8), _m_psllwi(_m_pand(t0v4, c8), 1));\
    (D27) = _m_por(_m_psrlwi(_m_pand(t0v5, c9), 1), _m_pand(t0v4, c9));\
    (D28) = _m_por(_m_pand(t0v6, c8), _m_psllwi(_m_pand(t0v2, c8), 1));\
    (D29) = _m_por(_m_psrlwi(_m_pand(t0v6, c9), 1), _m_pand(t0v2, c9));\
    (D30) = _m_por(_m_pand(t0v0, c8), _m_psllwi(_m_pand(t0v1, c8), 1));\
    (D31) = _m_por(_m_psrlwi(_m_pand(t0v0, c9), 1), _m_pand(t0v1, c9));\
}
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    const __m64 c0 = (*(const __m64*)(transform_const2_tbl + 2*4));\
    const __m64 c7 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c8 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*5));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*6));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*7));\
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
    t0v0 = _m_por(_m_pand(t0v0, c0), _m_pslldi(t0v16, 16));\
    t0v1 = _m_por(_m_pand(t0v1, c0), _m_pslldi(t0v17, 16));\
    t0v2 = _m_por(_m_pand(t0v2, c0), _m_pslldi(t0v18, 16));\
    t0v3 = _m_por(_m_pand(t0v3, c0), _m_pslldi(t0v19, 16));\
    t0v4 = _m_por(_m_pand(t0v4, c0), _m_pslldi(t0v20, 16));\
    t0v5 = _m_por(_m_pand(t0v5, c0), _m_pslldi(t0v21, 16));\
    t0v6 = _m_por(_m_pand(t0v6, c0), _m_pslldi(t0v22, 16));\
    t0v7 = _m_por(_m_pand(t0v7, c0), _m_pslldi(t0v23, 16));\
    t0v8 = _m_por(_m_pand(t0v8, c0), _m_pslldi(t0v24, 16));\
    t0v9 = _m_por(_m_pand(t0v9, c0), _m_pslldi(t0v25, 16));\
    t0v10 = _m_por(_m_pand(t0v10, c0), _m_pslldi(t0v26, 16));\
    t0v11 = _m_por(_m_pand(t0v11, c0), _m_pslldi(t0v27, 16));\
    t0v12 = _m_por(_m_pand(t0v12, c0), _m_pslldi(t0v28, 16));\
    t0v13 = _m_por(_m_pand(t0v13, c0), _m_pslldi(t0v29, 16));\
    t0v14 = _m_por(_m_pand(t0v14, c0), _m_pslldi(t0v30, 16));\
    t0v15 = _m_por(_m_pand(t0v15, c0), _m_pslldi(t0v31, 16));\
    t0v16 = _m_por(_m_pand(t0v0, c1), _m_psllwi(t0v8, 8));\
    t0v0 = _m_por(_m_psrlwi(t0v0, 8), _m_pand(t0v8, c2));\
    t0v8 = _m_por(_m_pand(t0v1, c1), _m_psllwi(t0v9, 8));\
    t0v1 = _m_por(_m_psrlwi(t0v1, 8), _m_pand(t0v9, c2));\
    t0v9 = _m_por(_m_pand(t0v2, c1), _m_psllwi(t0v10, 8));\
    t0v2 = _m_por(_m_psrlwi(t0v2, 8), _m_pand(t0v10, c2));\
    t0v10 = _m_por(_m_pand(t0v3, c1), _m_psllwi(t0v11, 8));\
    t0v3 = _m_por(_m_psrlwi(t0v3, 8), _m_pand(t0v11, c2));\
    t0v11 = _m_por(_m_pand(t0v4, c1), _m_psllwi(t0v12, 8));\
    t0v4 = _m_por(_m_psrlwi(t0v4, 8), _m_pand(t0v12, c2));\
    t0v12 = _m_por(_m_pand(t0v5, c1), _m_psllwi(t0v13, 8));\
    t0v5 = _m_por(_m_psrlwi(t0v5, 8), _m_pand(t0v13, c2));\
    t0v13 = _m_por(_m_pand(t0v6, c1), _m_psllwi(t0v14, 8));\
    t0v6 = _m_por(_m_psrlwi(t0v6, 8), _m_pand(t0v14, c2));\
    t0v14 = _m_por(_m_pand(t0v7, c1), _m_psllwi(t0v15, 8));\
    t0v7 = _m_por(_m_psrlwi(t0v7, 8), _m_pand(t0v15, c2));\
    t0v15 = _m_por(_m_pand(t0v16, c3), _m_psllwi(_m_pand(t0v11, c3), 4));\
    t0v11 = _m_por(_m_psrlwi(_m_pand(t0v16, c4), 4), _m_pand(t0v11, c4));\
    t0v16 = _m_por(_m_pand(t0v8, c3), _m_psllwi(_m_pand(t0v12, c3), 4));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c4), 4), _m_pand(t0v12, c4));\
    t0v12 = _m_por(_m_pand(t0v9, c3), _m_psllwi(_m_pand(t0v13, c3), 4));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c4), 4), _m_pand(t0v13, c4));\
    t0v13 = _m_por(_m_pand(t0v10, c3), _m_psllwi(_m_pand(t0v14, c3), 4));\
    t0v10 = _m_por(_m_psrlwi(_m_pand(t0v10, c4), 4), _m_pand(t0v14, c4));\
    t0v14 = _m_por(_m_pand(t0v0, c3), _m_psllwi(_m_pand(t0v4, c3), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c4), 4), _m_pand(t0v4, c4));\
    t0v4 = _m_por(_m_pand(t0v1, c3), _m_psllwi(_m_pand(t0v5, c3), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c4), 4), _m_pand(t0v5, c4));\
    t0v5 = _m_por(_m_pand(t0v2, c3), _m_psllwi(_m_pand(t0v6, c3), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c4), 4), _m_pand(t0v6, c4));\
    t0v6 = _m_por(_m_pand(t0v3, c3), _m_psllwi(_m_pand(t0v7, c3), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v3, c4), 4), _m_pand(t0v7, c4));\
    t0v7 = _m_por(_m_pand(t0v15, c5), _m_psllwi(_m_pand(t0v12, c5), 2));\
    t0v12 = _m_por(_m_psrlwi(_m_pand(t0v15, c6), 2), _m_pand(t0v12, c6));\
    t0v15 = _m_por(_m_pand(t0v16, c5), _m_psllwi(_m_pand(t0v13, c5), 2));\
    t0v13 = _m_por(_m_psrlwi(_m_pand(t0v16, c6), 2), _m_pand(t0v13, c6));\
    t0v16 = _m_por(_m_pand(t0v11, c5), _m_psllwi(_m_pand(t0v9, c5), 2));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v11, c6), 2), _m_pand(t0v9, c6));\
    t0v11 = _m_por(_m_pand(t0v8, c5), _m_psllwi(_m_pand(t0v10, c5), 2));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c6), 2), _m_pand(t0v10, c6));\
    t0v10 = _m_por(_m_pand(t0v14, c5), _m_psllwi(_m_pand(t0v5, c5), 2));\
    t0v5 = _m_por(_m_psrlwi(_m_pand(t0v14, c6), 2), _m_pand(t0v5, c6));\
    t0v14 = _m_por(_m_pand(t0v4, c5), _m_psllwi(_m_pand(t0v6, c5), 2));\
    t0v4 = _m_por(_m_psrlwi(_m_pand(t0v4, c6), 2), _m_pand(t0v6, c6));\
    t0v6 = _m_por(_m_pand(t0v0, c5), _m_psllwi(_m_pand(t0v2, c5), 2));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c6), 2), _m_pand(t0v2, c6));\
    t0v2 = _m_por(_m_pand(t0v1, c5), _m_psllwi(_m_pand(t0v3, c5), 2));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c6), 2), _m_pand(t0v3, c6));\
    (D0) = _m_por(_m_pand(t0v7, c7), _m_psllwi(_m_pand(t0v15, c7), 1));\
    (D1) = _m_por(_m_psrlwi(_m_pand(t0v7, c8), 1), _m_pand(t0v15, c8));\
    (D2) = _m_por(_m_pand(t0v12, c7), _m_psllwi(_m_pand(t0v13, c7), 1));\
    (D3) = _m_por(_m_psrlwi(_m_pand(t0v12, c8), 1), _m_pand(t0v13, c8));\
    (D4) = _m_por(_m_pand(t0v16, c7), _m_psllwi(_m_pand(t0v11, c7), 1));\
    (D5) = _m_por(_m_psrlwi(_m_pand(t0v16, c8), 1), _m_pand(t0v11, c8));\
    (D6) = _m_por(_m_pand(t0v9, c7), _m_psllwi(_m_pand(t0v8, c7), 1));\
    (D7) = _m_por(_m_psrlwi(_m_pand(t0v9, c8), 1), _m_pand(t0v8, c8));\
    (D8) = _m_por(_m_pand(t0v10, c7), _m_psllwi(_m_pand(t0v14, c7), 1));\
    (D9) = _m_por(_m_psrlwi(_m_pand(t0v10, c8), 1), _m_pand(t0v14, c8));\
    (D10) = _m_por(_m_pand(t0v5, c7), _m_psllwi(_m_pand(t0v4, c7), 1));\
    (D11) = _m_por(_m_psrlwi(_m_pand(t0v5, c8), 1), _m_pand(t0v4, c8));\
    (D12) = _m_por(_m_pand(t0v6, c7), _m_psllwi(_m_pand(t0v2, c7), 1));\
    (D13) = _m_por(_m_psrlwi(_m_pand(t0v6, c8), 1), _m_pand(t0v2, c8));\
    (D14) = _m_por(_m_pand(t0v0, c7), _m_psllwi(_m_pand(t0v1, c7), 1));\
    (D15) = _m_por(_m_psrlwi(_m_pand(t0v0, c8), 1), _m_pand(t0v1, c8));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
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
    t0v0 = _m_por(_m_por(_m_por(_m_pand(t0v0, c0), _m_pslldi(_m_pand(t0v8, c0), 8)), _m_pslldi(_m_pand(t0v16, c0), 16)), _m_pslldi(t0v24, 24));\
    t0v1 = _m_por(_m_por(_m_por(_m_pand(t0v1, c0), _m_pslldi(_m_pand(t0v9, c0), 8)), _m_pslldi(_m_pand(t0v17, c0), 16)), _m_pslldi(t0v25, 24));\
    t0v2 = _m_por(_m_por(_m_por(_m_pand(t0v2, c0), _m_pslldi(_m_pand(t0v10, c0), 8)), _m_pslldi(_m_pand(t0v18, c0), 16)), _m_pslldi(t0v26, 24));\
    t0v3 = _m_por(_m_por(_m_por(_m_pand(t0v3, c0), _m_pslldi(_m_pand(t0v11, c0), 8)), _m_pslldi(_m_pand(t0v19, c0), 16)), _m_pslldi(t0v27, 24));\
    t0v4 = _m_por(_m_por(_m_por(_m_pand(t0v4, c0), _m_pslldi(_m_pand(t0v12, c0), 8)), _m_pslldi(_m_pand(t0v20, c0), 16)), _m_pslldi(t0v28, 24));\
    t0v5 = _m_por(_m_por(_m_por(_m_pand(t0v5, c0), _m_pslldi(_m_pand(t0v13, c0), 8)), _m_pslldi(_m_pand(t0v21, c0), 16)), _m_pslldi(t0v29, 24));\
    t0v6 = _m_por(_m_por(_m_por(_m_pand(t0v6, c0), _m_pslldi(_m_pand(t0v14, c0), 8)), _m_pslldi(_m_pand(t0v22, c0), 16)), _m_pslldi(t0v30, 24));\
    t0v7 = _m_por(_m_por(_m_por(_m_pand(t0v7, c0), _m_pslldi(_m_pand(t0v15, c0), 8)), _m_pslldi(_m_pand(t0v23, c0), 16)), _m_pslldi(t0v31, 24));\
    t0v8 = _m_por(_m_pand(t0v0, c1), _m_psllwi(_m_pand(t0v4, c1), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c2), 4), _m_pand(t0v4, c2));\
    t0v4 = _m_por(_m_pand(t0v1, c1), _m_psllwi(_m_pand(t0v5, c1), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c2), 4), _m_pand(t0v5, c2));\
    t0v5 = _m_por(_m_pand(t0v2, c1), _m_psllwi(_m_pand(t0v6, c1), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c2), 4), _m_pand(t0v6, c2));\
    t0v6 = _m_por(_m_pand(t0v3, c1), _m_psllwi(_m_pand(t0v7, c1), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v3, c2), 4), _m_pand(t0v7, c2));\
    t0v7 = _m_por(_m_pand(t0v8, c3), _m_psllwi(_m_pand(t0v5, c3), 2));\
    t0v5 = _m_por(_m_psrlwi(_m_pand(t0v8, c4), 2), _m_pand(t0v5, c4));\
    t0v8 = _m_por(_m_pand(t0v4, c3), _m_psllwi(_m_pand(t0v6, c3), 2));\
    t0v4 = _m_por(_m_psrlwi(_m_pand(t0v4, c4), 2), _m_pand(t0v6, c4));\
    t0v0 = _m_por(_m_pand(t0v0, c3), _m_psllwi(_m_pand(t0v2, c3), 2));\
    t0v1 = _m_por(_m_pand(t0v1, c3), _m_psllwi(_m_pand(t0v3, c3), 2));\
    (D0) = _m_por(_m_pand(t0v7, c5), _m_psllwi(_m_pand(t0v8, c5), 1));\
    (D1) = _m_por(_m_psrlwi(_m_pand(t0v7, c6), 1), _m_pand(t0v8, c6));\
    (D2) = _m_por(_m_pand(t0v5, c5), _m_psllwi(_m_pand(t0v4, c5), 1));\
    (D3) = _m_por(_m_psrlwi(_m_pand(t0v5, c6), 1), _m_pand(t0v4, c6));\
    (D4) = _m_por(_m_pand(t0v0, c5), _m_psllwi(_m_pand(t0v1, c5), 1));\
    (D5) = _m_por(_m_psrlwi(_m_pand(t0v0, c6), 1), _m_pand(t0v1, c6));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    const __m64 c0 = (*(const __m64*)(transform_const2_tbl + 2*0));\
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
    (D0) = _m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_por(_m_pand(t0v0, c0), _m_pslldi(_m_pand(t0v1, c0), 1)), _m_pslldi(_m_pand(t0v2, c0), 2)), _m_pslldi(_m_pand(t0v3, c0), 3)), _m_pslldi(_m_pand(t0v4, c0), 4)), _m_pslldi(_m_pand(t0v5, c0), 5)), _m_pslldi(_m_pand(t0v6, c0), 6)), _m_pslldi(_m_pand(t0v7, c0), 7)), _m_pslldi(_m_pand(t0v8, c0), 8)), _m_pslldi(_m_pand(t0v9, c0), 9)), _m_pslldi(_m_pand(t0v10, c0), 10)), _m_pslldi(_m_pand(t0v11, c0), 11)), _m_pslldi(_m_pand(t0v12, c0), 12)), _m_pslldi(_m_pand(t0v13, c0), 13)), _m_pslldi(_m_pand(t0v14, c0), 14)), _m_pslldi(_m_pand(t0v15, c0), 15)), _m_pslldi(_m_pand(t0v16, c0), 16)), _m_pslldi(_m_pand(t0v17, c0), 17)), _m_pslldi(_m_pand(t0v18, c0), 18)), _m_pslldi(_m_pand(t0v19, c0), 19)), _m_pslldi(_m_pand(t0v20, c0), 20)), _m_pslldi(_m_pand(t0v21, c0), 21)), _m_pslldi(_m_pand(t0v22, c0), 22)), _m_pslldi(_m_pand(t0v23, c0), 23)), _m_pslldi(_m_pand(t0v24, c0), 24)), _m_pslldi(_m_pand(t0v25, c0), 25)), _m_pslldi(_m_pand(t0v26, c0), 26)), _m_pslldi(_m_pand(t0v27, c0), 27)), _m_pslldi(_m_pand(t0v28, c0), 28)), _m_pslldi(_m_pand(t0v29, c0), 29)), _m_pslldi(_m_pand(t0v30, c0), 30)), _m_pslldi(t0v31, 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_SSE2.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    const __m128i c10 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c11 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c8 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c9 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c6 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*5));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*6));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*7));\
    const __m128i c2 = (*(const __m128i*)(transform_const_tbl + 4*8));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*9));\
    __m128i t0v0;\
    __m128i t0v1;\
    __m128i t0v2;\
    __m128i t0v3;\
    __m128i t0v4;\
    __m128i t0v5;\
    __m128i t0v6;\
    __m128i t0v7;\
    __m128i t0v8;\
    __m128i t0v9;\
    __m128i t0v10;\
    __m128i t0v11;\
    __m128i t0v12;\
    __m128i t0v13;\
    __m128i t0v14;\
    __m128i t0v15;\
    __m128i t0v16;\
    __m128i t0v17;\
    __m128i t0v18;\
    __m128i t0v19;\
    __m128i t0v20;\
    __m128i t0v21;\
    __m128i t0v22;\
    __m128i t0v23;\
    __m128i t0v24;\
    __m128i t0v25;\
    __m128i t0v26;\
    __m128i t0v27;\
    __m128i t0v28;\
    __m128i t0v29;\
    __m128i t0v30;\
    __m128i t0v31;\
    __m128i t0v32;\
    t0v0 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v1 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v2 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v3 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v4 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v5 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v6 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v7 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v8 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v9 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v10 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v11 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v12 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v13 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v14 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v15 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v16 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v17 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v18 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v19 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v20 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v21 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v22 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v23 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v24 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v25 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v26 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v27 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v28 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v29 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v30 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v31 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c0), _mm_slli_epi64(t0v16, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v16, c1));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v1, c0), _mm_slli_epi64(t0v17, 32));\
    t0v1 = _mm_or_si128(_mm_srli_epi64(t0v1, 32), _mm_and_si128(t0v17, c1));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v2, c0), _mm_slli_epi64(t0v18, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v2, 32), _mm_and_si128(t0v18, c1));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v3, c0), _mm_slli_epi64(t0v19, 32));\
    t0v3 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v19, c1));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v4, c0), _mm_slli_epi64(t0v20, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v4, 32), _mm_and_si128(t0v20, c1));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v5, c0), _mm_slli_epi64(t0v21, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v21, c1));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v6, c0), _mm_slli_epi64(t0v22, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v22, c1));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v7, c0), _mm_slli_epi64(t0v23, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v7, 32), _mm_and_si128(t0v23, c1));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v8, c0), _mm_slli_epi64(t0v24, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v24, c1));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v9, c0), _mm_slli_epi64(t0v25, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v25, c1));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v10, c0), _mm_slli_epi64(t0v26, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v26, c1));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v11, c0), _mm_slli_epi64(t0v27, 32));\
    t0v11 = _mm_or_si128(_mm_srli_epi64(t0v11, 32), _mm_and_si128(t0v27, c1));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v12, c0), _mm_slli_epi64(t0v28, 32));\
    t0v12 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v28, c1));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v13, c0), _mm_slli_epi64(t0v29, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v13, 32), _mm_and_si128(t0v29, c1));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v14, c0), _mm_slli_epi64(t0v30, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v30, c1));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v15, c0), _mm_slli_epi64(t0v31, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v15, 32), _mm_and_si128(t0v31, c1));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c2), _mm_slli_epi32(t0v23, 16));\
    t0v23 = _mm_or_si128(_mm_srli_epi32(t0v32, 16), _mm_and_si128(t0v23, c3));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi32(t0v8, 16));\
    t0v0 = _mm_or_si128(_mm_srli_epi32(t0v0, 16), _mm_and_si128(t0v8, c3));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v16, c2), _mm_slli_epi32(t0v24, 16));\
    t0v16 = _mm_or_si128(_mm_srli_epi32(t0v16, 16), _mm_and_si128(t0v24, c3));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi32(t0v9, 16));\
    t0v1 = _mm_or_si128(_mm_srli_epi32(t0v1, 16), _mm_and_si128(t0v9, c3));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v17, c2), _mm_slli_epi32(t0v25, 16));\
    t0v17 = _mm_or_si128(_mm_srli_epi32(t0v17, 16), _mm_and_si128(t0v25, c3));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v2, c2), _mm_slli_epi32(t0v10, 16));\
    t0v2 = _mm_or_si128(_mm_srli_epi32(t0v2, 16), _mm_and_si128(t0v10, c3));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v18, c2), _mm_slli_epi32(t0v26, 16));\
    t0v18 = _mm_or_si128(_mm_srli_epi32(t0v18, 16), _mm_and_si128(t0v26, c3));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v3, c2), _mm_slli_epi32(t0v11, 16));\
    t0v3 = _mm_or_si128(_mm_srli_epi32(t0v3, 16), _mm_and_si128(t0v11, c3));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v19, c2), _mm_slli_epi32(t0v27, 16));\
    t0v19 = _mm_or_si128(_mm_srli_epi32(t0v19, 16), _mm_and_si128(t0v27, c3));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v4, c2), _mm_slli_epi32(t0v12, 16));\
    t0v4 = _mm_or_si128(_mm_srli_epi32(t0v4, 16), _mm_and_si128(t0v12, c3));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v20, c2), _mm_slli_epi32(t0v28, 16));\
    t0v20 = _mm_or_si128(_mm_srli_epi32(t0v20, 16), _mm_and_si128(t0v28, c3));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v5, c2), _mm_slli_epi32(t0v13, 16));\
    t0v5 = _mm_or_si128(_mm_srli_epi32(t0v5, 16), _mm_and_si128(t0v13, c3));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v21, c2), _mm_slli_epi32(t0v29, 16));\
    t0v21 = _mm_or_si128(_mm_srli_epi32(t0v21, 16), _mm_and_si128(t0v29, c3));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v6, c2), _mm_slli_epi32(t0v14, 16));\
    t0v6 = _mm_or_si128(_mm_srli_epi32(t0v6, 16), _mm_and_si128(t0v14, c3));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v22, c2), _mm_slli_epi32(t0v30, 16));\
    t0v22 = _mm_or_si128(_mm_srli_epi32(t0v22, 16), _mm_and_si128(t0v30, c3));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v7, c2), _mm_slli_epi32(t0v15, 16));\
    t0v7 = _mm_or_si128(_mm_srli_epi32(t0v7, 16), _mm_and_si128(t0v15, c3));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v31, c4), _mm_slli_epi16(t0v11, 8));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(t0v31, 8), _mm_and_si128(t0v11, c5));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c4), _mm_slli_epi16(t0v27, 8));\
    t0v27 = _mm_or_si128(_mm_srli_epi16(t0v32, 8), _mm_and_si128(t0v27, c5));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v8, c4), _mm_slli_epi16(t0v12, 8));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(t0v8, 8), _mm_and_si128(t0v12, c5));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v24, c4), _mm_slli_epi16(t0v28, 8));\
    t0v24 = _mm_or_si128(_mm_srli_epi16(t0v24, 8), _mm_and_si128(t0v28, c5));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v9, c4), _mm_slli_epi16(t0v13, 8));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(t0v9, 8), _mm_and_si128(t0v13, c5));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v25, c4), _mm_slli_epi16(t0v29, 8));\
    t0v25 = _mm_or_si128(_mm_srli_epi16(t0v25, 8), _mm_and_si128(t0v29, c5));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v10, c4), _mm_slli_epi16(t0v14, 8));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(t0v10, 8), _mm_and_si128(t0v14, c5));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v26, c4), _mm_slli_epi16(t0v30, 8));\
    t0v26 = _mm_or_si128(_mm_srli_epi16(t0v26, 8), _mm_and_si128(t0v30, c5));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v23, c4), _mm_slli_epi16(t0v19, 8));\
    t0v19 = _mm_or_si128(_mm_srli_epi16(t0v23, 8), _mm_and_si128(t0v19, c5));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v0, c4), _mm_slli_epi16(t0v4, 8));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(t0v0, 8), _mm_and_si128(t0v4, c5));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v16, c4), _mm_slli_epi16(t0v20, 8));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(t0v16, 8), _mm_and_si128(t0v20, c5));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v1, c4), _mm_slli_epi16(t0v5, 8));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(t0v1, 8), _mm_and_si128(t0v5, c5));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v17, c4), _mm_slli_epi16(t0v21, 8));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(t0v17, 8), _mm_and_si128(t0v21, c5));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v2, c4), _mm_slli_epi16(t0v6, 8));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(t0v2, 8), _mm_and_si128(t0v6, c5));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v18, c4), _mm_slli_epi16(t0v22, 8));\
    t0v18 = _mm_or_si128(_mm_srli_epi16(t0v18, 8), _mm_and_si128(t0v22, c5));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v3, c4), _mm_slli_epi16(t0v7, 8));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(t0v3, 8), _mm_and_si128(t0v7, c5));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v15, c6), _mm_slli_epi16(_mm_and_si128(t0v28, c6), 4));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c7), 4), _mm_and_si128(t0v28, c7));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v31, c6), _mm_slli_epi16(_mm_and_si128(t0v13, c6), 4));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v31, c7), 4), _mm_and_si128(t0v13, c7));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c6), _mm_slli_epi16(_mm_and_si128(t0v29, c6), 4));\
    t0v29 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v32, c7), 4), _mm_and_si128(t0v29, c7));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v12, c6), _mm_slli_epi16(_mm_and_si128(t0v14, c6), 4));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v12, c7), 4), _mm_and_si128(t0v14, c7));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v11, c6), _mm_slli_epi16(_mm_and_si128(t0v9, c6), 4));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c7), 4), _mm_and_si128(t0v9, c7));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v27, c6), _mm_slli_epi16(_mm_and_si128(t0v25, c6), 4));\
    t0v25 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v27, c7), 4), _mm_and_si128(t0v25, c7));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v8, c6), _mm_slli_epi16(_mm_and_si128(t0v10, c6), 4));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c7), 4), _mm_and_si128(t0v10, c7));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v24, c6), _mm_slli_epi16(_mm_and_si128(t0v26, c6), 4));\
    t0v24 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v24, c7), 4), _mm_and_si128(t0v26, c7));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v30, c6), _mm_slli_epi16(_mm_and_si128(t0v5, c6), 4));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v30, c7), 4), _mm_and_si128(t0v5, c7));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v23, c6), _mm_slli_epi16(_mm_and_si128(t0v21, c6), 4));\
    t0v21 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v23, c7), 4), _mm_and_si128(t0v21, c7));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v4, c6), _mm_slli_epi16(_mm_and_si128(t0v6, c6), 4));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c7), 4), _mm_and_si128(t0v6, c7));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v20, c6), _mm_slli_epi16(_mm_and_si128(t0v22, c6), 4));\
    t0v20 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v20, c7), 4), _mm_and_si128(t0v22, c7));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v19, c6), _mm_slli_epi16(_mm_and_si128(t0v17, c6), 4));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v19, c7), 4), _mm_and_si128(t0v17, c7));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v0, c6), _mm_slli_epi16(_mm_and_si128(t0v2, c6), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c7), 4), _mm_and_si128(t0v2, c7));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v16, c6), _mm_slli_epi16(_mm_and_si128(t0v18, c6), 4));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c7), 4), _mm_and_si128(t0v18, c7));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v1, c6), _mm_slli_epi16(_mm_and_si128(t0v3, c6), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c7), 4), _mm_and_si128(t0v3, c7));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v7, c8), _mm_slli_epi16(_mm_and_si128(t0v31, c8), 2));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c9), 2), _mm_and_si128(t0v31, c9));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v28, c8), _mm_slli_epi16(_mm_and_si128(t0v32, c8), 2));\
    t0v28 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v28, c9), 2), _mm_and_si128(t0v32, c9));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v15, c8), _mm_slli_epi16(_mm_and_si128(t0v29, c8), 2));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c9), 2), _mm_and_si128(t0v29, c9));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v13, c8), _mm_slli_epi16(_mm_and_si128(t0v12, c8), 2));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c9), 2), _mm_and_si128(t0v12, c9));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v14, c8), _mm_slli_epi16(_mm_and_si128(t0v27, c8), 2));\
    t0v14 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v14, c9), 2), _mm_and_si128(t0v27, c9));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v11, c8), _mm_slli_epi16(_mm_and_si128(t0v10, c8), 2));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c9), 2), _mm_and_si128(t0v10, c9));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v9, c8), _mm_slli_epi16(_mm_and_si128(t0v8, c8), 2));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c9), 2), _mm_and_si128(t0v8, c9));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v25, c8), _mm_slli_epi16(_mm_and_si128(t0v24, c8), 2));\
    t0v24 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v25, c9), 2), _mm_and_si128(t0v24, c9));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v26, c8), _mm_slli_epi16(_mm_and_si128(t0v23, c8), 2));\
    t0v23 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v26, c9), 2), _mm_and_si128(t0v23, c9));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v30, c8), _mm_slli_epi16(_mm_and_si128(t0v6, c8), 2));\
    t0v6 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v30, c9), 2), _mm_and_si128(t0v6, c9));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v5, c8), _mm_slli_epi16(_mm_and_si128(t0v4, c8), 2));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v5, c9), 2), _mm_and_si128(t0v4, c9));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v21, c8), _mm_slli_epi16(_mm_and_si128(t0v20, c8), 2));\
    t0v20 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v21, c9), 2), _mm_and_si128(t0v20, c9));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v22, c8), _mm_slli_epi16(_mm_and_si128(t0v2, c8), 2));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v22, c9), 2), _mm_and_si128(t0v2, c9));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v19, c8), _mm_slli_epi16(_mm_and_si128(t0v18, c8), 2));\
    t0v18 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v19, c9), 2), _mm_and_si128(t0v18, c9));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v17, c8), _mm_slli_epi16(_mm_and_si128(t0v16, c8), 2));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v17, c9), 2), _mm_and_si128(t0v16, c9));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v0, c8), _mm_slli_epi16(_mm_and_si128(t0v1, c8), 2));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c9), 2), _mm_and_si128(t0v1, c9));\
    (D0) = _mm_or_si128(_mm_and_si128(t0v3, c10), _mm_slli_epi16(_mm_and_si128(t0v31, c10), 1));\
    (D1) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v3, c11), 1), _mm_and_si128(t0v31, c11));\
    (D2) = _mm_or_si128(_mm_and_si128(t0v7, c10), _mm_slli_epi16(_mm_and_si128(t0v28, c10), 1));\
    (D3) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c11), 1), _mm_and_si128(t0v28, c11));\
    (D4) = _mm_or_si128(_mm_and_si128(t0v32, c10), _mm_slli_epi16(_mm_and_si128(t0v29, c10), 1));\
    (D5) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v32, c11), 1), _mm_and_si128(t0v29, c11));\
    (D6) = _mm_or_si128(_mm_and_si128(t0v15, c10), _mm_slli_epi16(_mm_and_si128(t0v12, c10), 1));\
    (D7) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c11), 1), _mm_and_si128(t0v12, c11));\
    (D8) = _mm_or_si128(_mm_and_si128(t0v13, c10), _mm_slli_epi16(_mm_and_si128(t0v27, c10), 1));\
    (D9) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c11), 1), _mm_and_si128(t0v27, c11));\
    (D10) = _mm_or_si128(_mm_and_si128(t0v14, c10), _mm_slli_epi16(_mm_and_si128(t0v10, c10), 1));\
    (D11) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v14, c11), 1), _mm_and_si128(t0v10, c11));\
    (D12) = _mm_or_si128(_mm_and_si128(t0v11, c10), _mm_slli_epi16(_mm_and_si128(t0v9, c10), 1));\
    (D13) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c11), 1), _mm_and_si128(t0v9, c11));\
    (D14) = _mm_or_si128(_mm_and_si128(t0v8, c10), _mm_slli_epi16(_mm_and_si128(t0v24, c10), 1));\
    (D15) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c11), 1), _mm_and_si128(t0v24, c11));\
    (D16) = _mm_or_si128(_mm_and_si128(t0v25, c10), _mm_slli_epi16(_mm_and_si128(t0v26, c10), 1));\
    (D17) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v25, c11), 1), _mm_and_si128(t0v26, c11));\
    (D18) = _mm_or_si128(_mm_and_si128(t0v23, c10), _mm_slli_epi16(_mm_and_si128(t0v6, c10), 1));\
    (D19) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v23, c11), 1), _mm_and_si128(t0v6, c11));\
    (D20) = _mm_or_si128(_mm_and_si128(t0v30, c10), _mm_slli_epi16(_mm_and_si128(t0v5, c10), 1));\
    (D21) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v30, c11), 1), _mm_and_si128(t0v5, c11));\
    (D22) = _mm_or_si128(_mm_and_si128(t0v4, c10), _mm_slli_epi16(_mm_and_si128(t0v20, c10), 1));\
    (D23) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c11), 1), _mm_and_si128(t0v20, c11));\
    (D24) = _mm_or_si128(_mm_and_si128(t0v21, c10), _mm_slli_epi16(_mm_and_si128(t0v22, c10), 1));\
    (D25) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v21, c11), 1), _mm_and_si128(t0v22, c11));\
    (D26) = _mm_or_si128(_mm_and_si128(t0v2, c10), _mm_slli_epi16(_mm_and_si128(t0v18, c10), 1));\
    (D27) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c11), 1), _mm_and_si128(t0v18, c11));\
    (D28) = _mm_or_si128(_mm_and_si128(t0v19, c10), _mm_slli_epi16(_mm_and_si128(t0v17, c10), 1));\
    (D29) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v19, c11), 1), _mm_and_si128(t0v17, c11));\
    (D30) = _mm_or_si128(_mm_and_si128(t0v16, c10), _mm_slli_epi16(_mm_and_si128(t0v0, c10), 1));\
    (D31) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c11), 1), _mm_and_si128(t0v0, c11));\
}
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    const __m128i c2 = (*(const __m128i*)(transform_const2_tbl + 4*4));\
    const __m128i c9 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c10 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c8 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c6 = (*(const __m128i*)(transform_const_tbl + 4*5));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*6));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*7));\
    __m128i t0v0;\
    __m128i t0v1;\
    __m128i t0v2;\
    __m128i t0v3;\
    __m128i t0v4;\
    __m128i t0v5;\
    __m128i t0v6;\
    __m128i t0v7;\
    __m128i t0v8;\
    __m128i t0v9;\
    __m128i t0v10;\
    __m128i t0v11;\
    __m128i t0v12;\
    __m128i t0v13;\
    __m128i t0v14;\
    __m128i t0v15;\
    __m128i t0v16;\
    __m128i t0v17;\
    __m128i t0v18;\
    __m128i t0v19;\
    __m128i t0v20;\
    __m128i t0v21;\
    __m128i t0v22;\
    __m128i t0v23;\
    __m128i t0v24;\
    __m128i t0v25;\
    __m128i t0v26;\
    __m128i t0v27;\
    __m128i t0v28;\
    __m128i t0v29;\
    __m128i t0v30;\
    __m128i t0v31;\
    __m128i t0v32;\
    t0v0 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v1 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v2 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v3 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v4 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v5 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v6 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v7 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v8 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v9 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v10 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v11 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v12 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v13 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v14 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v15 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v16 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v17 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v18 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v19 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v20 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v21 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v22 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v23 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v24 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v25 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v26 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v27 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v28 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v29 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v30 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v31 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c0), _mm_slli_epi64(t0v16, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v16, c1));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v1, c0), _mm_slli_epi64(t0v17, 32));\
    t0v1 = _mm_or_si128(_mm_srli_epi64(t0v1, 32), _mm_and_si128(t0v17, c1));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v2, c0), _mm_slli_epi64(t0v18, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v2, 32), _mm_and_si128(t0v18, c1));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v3, c0), _mm_slli_epi64(t0v19, 32));\
    t0v3 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v19, c1));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v4, c0), _mm_slli_epi64(t0v20, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v4, 32), _mm_and_si128(t0v20, c1));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v5, c0), _mm_slli_epi64(t0v21, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v21, c1));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v6, c0), _mm_slli_epi64(t0v22, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v22, c1));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v7, c0), _mm_slli_epi64(t0v23, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v7, 32), _mm_and_si128(t0v23, c1));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v8, c0), _mm_slli_epi64(t0v24, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v24, c1));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v9, c0), _mm_slli_epi64(t0v25, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v25, c1));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v10, c0), _mm_slli_epi64(t0v26, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v26, c1));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v11, c0), _mm_slli_epi64(t0v27, 32));\
    t0v11 = _mm_or_si128(_mm_srli_epi64(t0v11, 32), _mm_and_si128(t0v27, c1));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v12, c0), _mm_slli_epi64(t0v28, 32));\
    t0v12 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v28, c1));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v13, c0), _mm_slli_epi64(t0v29, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v13, 32), _mm_and_si128(t0v29, c1));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v14, c0), _mm_slli_epi64(t0v30, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v30, c1));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v15, c0), _mm_slli_epi64(t0v31, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v15, 32), _mm_and_si128(t0v31, c1));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v32, c2), _mm_slli_epi32(t0v23, 16));\
    t0v0 = _mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi32(t0v8, 16));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v16, c2), _mm_slli_epi32(t0v24, 16));\
    t0v1 = _mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi32(t0v9, 16));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v17, c2), _mm_slli_epi32(t0v25, 16));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v2, c2), _mm_slli_epi32(t0v10, 16));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v18, c2), _mm_slli_epi32(t0v26, 16));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v3, c2), _mm_slli_epi32(t0v11, 16));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v19, c2), _mm_slli_epi32(t0v27, 16));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v4, c2), _mm_slli_epi32(t0v12, 16));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v20, c2), _mm_slli_epi32(t0v28, 16));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v5, c2), _mm_slli_epi32(t0v13, 16));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v21, c2), _mm_slli_epi32(t0v29, 16));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v6, c2), _mm_slli_epi32(t0v14, 16));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v22, c2), _mm_slli_epi32(t0v30, 16));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v7, c2), _mm_slli_epi32(t0v15, 16));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v23, c3), _mm_slli_epi16(t0v11, 8));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(t0v23, 8), _mm_and_si128(t0v11, c4));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v0, c3), _mm_slli_epi16(t0v4, 8));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(t0v0, 8), _mm_and_si128(t0v4, c4));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v8, c3), _mm_slli_epi16(t0v12, 8));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(t0v8, 8), _mm_and_si128(t0v12, c4));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v1, c3), _mm_slli_epi16(t0v5, 8));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(t0v1, 8), _mm_and_si128(t0v5, c4));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v9, c3), _mm_slli_epi16(t0v13, 8));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(t0v9, 8), _mm_and_si128(t0v13, c4));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v2, c3), _mm_slli_epi16(t0v6, 8));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(t0v2, 8), _mm_and_si128(t0v6, c4));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v10, c3), _mm_slli_epi16(t0v14, 8));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(t0v10, 8), _mm_and_si128(t0v14, c4));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v3, c3), _mm_slli_epi16(t0v7, 8));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(t0v3, 8), _mm_and_si128(t0v7, c4));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v15, c5), _mm_slli_epi16(_mm_and_si128(t0v5, c5), 4));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c6), 4), _mm_and_si128(t0v5, c6));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c5), _mm_slli_epi16(_mm_and_si128(t0v13, c5), 4));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c6), 4), _mm_and_si128(t0v13, c6));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v4, c5), _mm_slli_epi16(_mm_and_si128(t0v6, c5), 4));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c6), 4), _mm_and_si128(t0v6, c6));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v12, c5), _mm_slli_epi16(_mm_and_si128(t0v14, c5), 4));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v12, c6), 4), _mm_and_si128(t0v14, c6));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v11, c5), _mm_slli_epi16(_mm_and_si128(t0v9, c5), 4));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c6), 4), _mm_and_si128(t0v9, c6));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v0, c5), _mm_slli_epi16(_mm_and_si128(t0v2, c5), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c6), 4), _mm_and_si128(t0v2, c6));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v8, c5), _mm_slli_epi16(_mm_and_si128(t0v10, c5), 4));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c6), 4), _mm_and_si128(t0v10, c6));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v1, c5), _mm_slli_epi16(_mm_and_si128(t0v3, c5), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c6), 4), _mm_and_si128(t0v3, c6));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v7, c7), _mm_slli_epi16(_mm_and_si128(t0v16, c7), 2));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c8), 2), _mm_and_si128(t0v16, c8));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v15, c7), _mm_slli_epi16(_mm_and_si128(t0v6, c7), 2));\
    t0v6 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c8), 2), _mm_and_si128(t0v6, c8));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v5, c7), _mm_slli_epi16(_mm_and_si128(t0v4, c7), 2));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v5, c8), 2), _mm_and_si128(t0v4, c8));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v13, c7), _mm_slli_epi16(_mm_and_si128(t0v12, c7), 2));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c8), 2), _mm_and_si128(t0v12, c8));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v14, c7), _mm_slli_epi16(_mm_and_si128(t0v2, c7), 2));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v14, c8), 2), _mm_and_si128(t0v2, c8));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v11, c7), _mm_slli_epi16(_mm_and_si128(t0v10, c7), 2));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c8), 2), _mm_and_si128(t0v10, c8));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v9, c7), _mm_slli_epi16(_mm_and_si128(t0v8, c7), 2));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c8), 2), _mm_and_si128(t0v8, c8));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v0, c7), _mm_slli_epi16(_mm_and_si128(t0v1, c7), 2));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c8), 2), _mm_and_si128(t0v1, c8));\
    (D0) = _mm_or_si128(_mm_and_si128(t0v3, c9), _mm_slli_epi16(_mm_and_si128(t0v16, c9), 1));\
    (D1) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v3, c10), 1), _mm_and_si128(t0v16, c10));\
    (D2) = _mm_or_si128(_mm_and_si128(t0v7, c9), _mm_slli_epi16(_mm_and_si128(t0v6, c9), 1));\
    (D3) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c10), 1), _mm_and_si128(t0v6, c10));\
    (D4) = _mm_or_si128(_mm_and_si128(t0v15, c9), _mm_slli_epi16(_mm_and_si128(t0v5, c9), 1));\
    (D5) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c10), 1), _mm_and_si128(t0v5, c10));\
    (D6) = _mm_or_si128(_mm_and_si128(t0v4, c9), _mm_slli_epi16(_mm_and_si128(t0v12, c9), 1));\
    (D7) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c10), 1), _mm_and_si128(t0v12, c10));\
    (D8) = _mm_or_si128(_mm_and_si128(t0v13, c9), _mm_slli_epi16(_mm_and_si128(t0v14, c9), 1));\
    (D9) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c10), 1), _mm_and_si128(t0v14, c10));\
    (D10) = _mm_or_si128(_mm_and_si128(t0v2, c9), _mm_slli_epi16(_mm_and_si128(t0v10, c9), 1));\
    (D11) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c10), 1), _mm_and_si128(t0v10, c10));\
    (D12) = _mm_or_si128(_mm_and_si128(t0v11, c9), _mm_slli_epi16(_mm_and_si128(t0v9, c9), 1));\
    (D13) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v11, c10), 1), _mm_and_si128(t0v9, c10));\
    (D14) = _mm_or_si128(_mm_and_si128(t0v8, c9), _mm_slli_epi16(_mm_and_si128(t0v0, c9), 1));\
    (D15) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c10), 1), _mm_and_si128(t0v0, c10));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
{\
    const __m128i c2 = (*(const __m128i*)(transform_const2_tbl + 4*3));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c8 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c6 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*5));\
    __m128i t0v0;\
    __m128i t0v1;\
    __m128i t0v2;\
    __m128i t0v3;\
    __m128i t0v4;\
    __m128i t0v5;\
    __m128i t0v6;\
    __m128i t0v7;\
    __m128i t0v8;\
    __m128i t0v9;\
    __m128i t0v10;\
    __m128i t0v11;\
    __m128i t0v12;\
    __m128i t0v13;\
    __m128i t0v14;\
    __m128i t0v15;\
    __m128i t0v16;\
    __m128i t0v17;\
    __m128i t0v18;\
    __m128i t0v19;\
    __m128i t0v20;\
    __m128i t0v21;\
    __m128i t0v22;\
    __m128i t0v23;\
    __m128i t0v24;\
    __m128i t0v25;\
    __m128i t0v26;\
    __m128i t0v27;\
    __m128i t0v28;\
    __m128i t0v29;\
    __m128i t0v30;\
    __m128i t0v31;\
    __m128i t0v32;\
    t0v0 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v1 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v2 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v3 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v4 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v5 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v6 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v7 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v8 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v9 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v10 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v11 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v12 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v13 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v14 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v15 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v16 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v17 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v18 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v19 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v20 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v21 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v22 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v23 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v24 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v25 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v26 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v27 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v28 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v29 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v30 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v31 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c0), _mm_slli_epi64(t0v16, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v16, c1));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v1, c0), _mm_slli_epi64(t0v17, 32));\
    t0v1 = _mm_or_si128(_mm_srli_epi64(t0v1, 32), _mm_and_si128(t0v17, c1));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v2, c0), _mm_slli_epi64(t0v18, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v2, 32), _mm_and_si128(t0v18, c1));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v3, c0), _mm_slli_epi64(t0v19, 32));\
    t0v3 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v19, c1));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v4, c0), _mm_slli_epi64(t0v20, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v4, 32), _mm_and_si128(t0v20, c1));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v5, c0), _mm_slli_epi64(t0v21, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v21, c1));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v6, c0), _mm_slli_epi64(t0v22, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v22, c1));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v7, c0), _mm_slli_epi64(t0v23, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v7, 32), _mm_and_si128(t0v23, c1));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v8, c0), _mm_slli_epi64(t0v24, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v24, c1));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v9, c0), _mm_slli_epi64(t0v25, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v25, c1));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v10, c0), _mm_slli_epi64(t0v26, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v26, c1));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v11, c0), _mm_slli_epi64(t0v27, 32));\
    t0v11 = _mm_or_si128(_mm_srli_epi64(t0v11, 32), _mm_and_si128(t0v27, c1));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v12, c0), _mm_slli_epi64(t0v28, 32));\
    t0v12 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v28, c1));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v13, c0), _mm_slli_epi64(t0v29, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v13, 32), _mm_and_si128(t0v29, c1));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v14, c0), _mm_slli_epi64(t0v30, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v30, c1));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v15, c0), _mm_slli_epi64(t0v31, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v15, 32), _mm_and_si128(t0v31, c1));\
    t0v19 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v32, c2), _mm_slli_epi32(_mm_and_si128(t0v19, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v23, c2), 16)), _mm_slli_epi32(t0v27, 24));\
    t0v0 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi32(_mm_and_si128(t0v4, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v8, c2), 16)), _mm_slli_epi32(t0v12, 24));\
    t0v4 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v16, c2), _mm_slli_epi32(_mm_and_si128(t0v20, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v24, c2), 16)), _mm_slli_epi32(t0v28, 24));\
    t0v1 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi32(_mm_and_si128(t0v5, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v9, c2), 16)), _mm_slli_epi32(t0v13, 24));\
    t0v5 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v17, c2), _mm_slli_epi32(_mm_and_si128(t0v21, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v25, c2), 16)), _mm_slli_epi32(t0v29, 24));\
    t0v2 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v2, c2), _mm_slli_epi32(_mm_and_si128(t0v6, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v10, c2), 16)), _mm_slli_epi32(t0v14, 24));\
    t0v6 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v18, c2), _mm_slli_epi32(_mm_and_si128(t0v22, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v26, c2), 16)), _mm_slli_epi32(t0v30, 24));\
    t0v3 = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v3, c2), _mm_slli_epi32(_mm_and_si128(t0v7, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v11, c2), 16)), _mm_slli_epi32(t0v15, 24));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v19, c3), _mm_slli_epi16(_mm_and_si128(t0v5, c3), 4));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v19, c4), 4), _mm_and_si128(t0v5, c4));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v0, c3), _mm_slli_epi16(_mm_and_si128(t0v2, c3), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c4), 4), _mm_and_si128(t0v2, c4));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v4, c3), _mm_slli_epi16(_mm_and_si128(t0v6, c3), 4));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c4), 4), _mm_and_si128(t0v6, c4));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v1, c3), _mm_slli_epi16(_mm_and_si128(t0v3, c3), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c4), 4), _mm_and_si128(t0v3, c4));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v7, c5), _mm_slli_epi16(_mm_and_si128(t0v2, c5), 2));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c6), 2), _mm_and_si128(t0v2, c6));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v8, c5), _mm_slli_epi16(_mm_and_si128(t0v6, c5), 2));\
    t0v6 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c6), 2), _mm_and_si128(t0v6, c6));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v5, c5), _mm_slli_epi16(_mm_and_si128(t0v4, c5), 2));\
    t0v0 = _mm_or_si128(_mm_and_si128(t0v0, c5), _mm_slli_epi16(_mm_and_si128(t0v1, c5), 2));\
    (D0) = _mm_or_si128(_mm_and_si128(t0v3, c7), _mm_slli_epi16(_mm_and_si128(t0v7, c7), 1));\
    (D1) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v3, c8), 1), _mm_and_si128(t0v7, c8));\
    (D2) = _mm_or_si128(_mm_and_si128(t0v2, c7), _mm_slli_epi16(_mm_and_si128(t0v6, c7), 1));\
    (D3) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c8), 1), _mm_and_si128(t0v6, c8));\
    (D4) = _mm_or_si128(_mm_and_si128(t0v4, c7), _mm_slli_epi16(_mm_and_si128(t0v0, c7), 1));\
    (D5) = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c8), 1), _mm_and_si128(t0v0, c8));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    const __m128i c2 = (*(const __m128i*)(transform_const2_tbl + 4*0));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    __m128i t0v0;\
    __m128i t0v1;\
    __m128i t0v2;\
    __m128i t0v3;\
    __m128i t0v4;\
    __m128i t0v5;\
    __m128i t0v6;\
    __m128i t0v7;\
    __m128i t0v8;\
    __m128i t0v9;\
    __m128i t0v10;\
    __m128i t0v11;\
    __m128i t0v12;\
    __m128i t0v13;\
    __m128i t0v14;\
    __m128i t0v15;\
    __m128i t0v16;\
    __m128i t0v17;\
    __m128i t0v18;\
    __m128i t0v19;\
    __m128i t0v20;\
    __m128i t0v21;\
    __m128i t0v22;\
    __m128i t0v23;\
    __m128i t0v24;\
    __m128i t0v25;\
    __m128i t0v26;\
    __m128i t0v27;\
    __m128i t0v28;\
    __m128i t0v29;\
    __m128i t0v30;\
    __m128i t0v31;\
    __m128i t0v32;\
    t0v0 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v1 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 0)), _mm_loadu_si128((const __m128i*)((S) + 64)));\
    t0v2 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v3 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 4)), _mm_loadu_si128((const __m128i*)((S) + 68)));\
    t0v4 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v5 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 8)), _mm_loadu_si128((const __m128i*)((S) + 72)));\
    t0v6 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v7 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 12)), _mm_loadu_si128((const __m128i*)((S) + 76)));\
    t0v8 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v9 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 16)), _mm_loadu_si128((const __m128i*)((S) + 80)));\
    t0v10 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v11 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 20)), _mm_loadu_si128((const __m128i*)((S) + 84)));\
    t0v12 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v13 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 24)), _mm_loadu_si128((const __m128i*)((S) + 88)));\
    t0v14 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v15 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 28)), _mm_loadu_si128((const __m128i*)((S) + 92)));\
    t0v16 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v17 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 32)), _mm_loadu_si128((const __m128i*)((S) + 96)));\
    t0v18 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v19 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 36)), _mm_loadu_si128((const __m128i*)((S) + 100)));\
    t0v20 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v21 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 40)), _mm_loadu_si128((const __m128i*)((S) + 104)));\
    t0v22 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v23 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 44)), _mm_loadu_si128((const __m128i*)((S) + 108)));\
    t0v24 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v25 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 48)), _mm_loadu_si128((const __m128i*)((S) + 112)));\
    t0v26 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v27 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 52)), _mm_loadu_si128((const __m128i*)((S) + 116)));\
    t0v28 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v29 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 56)), _mm_loadu_si128((const __m128i*)((S) + 120)));\
    t0v30 = _mm_unpacklo_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v31 = _mm_unpackhi_epi64(_mm_loadu_si128((const __m128i*)((S) + 60)), _mm_loadu_si128((const __m128i*)((S) + 124)));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c0), _mm_slli_epi64(t0v16, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v16, c1));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v1, c0), _mm_slli_epi64(t0v17, 32));\
    t0v1 = _mm_or_si128(_mm_srli_epi64(t0v1, 32), _mm_and_si128(t0v17, c1));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v2, c0), _mm_slli_epi64(t0v18, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v2, 32), _mm_and_si128(t0v18, c1));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v3, c0), _mm_slli_epi64(t0v19, 32));\
    t0v3 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v19, c1));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v4, c0), _mm_slli_epi64(t0v20, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v4, 32), _mm_and_si128(t0v20, c1));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v5, c0), _mm_slli_epi64(t0v21, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v21, c1));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v6, c0), _mm_slli_epi64(t0v22, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v22, c1));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v7, c0), _mm_slli_epi64(t0v23, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v7, 32), _mm_and_si128(t0v23, c1));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v8, c0), _mm_slli_epi64(t0v24, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v24, c1));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v9, c0), _mm_slli_epi64(t0v25, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v25, c1));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v10, c0), _mm_slli_epi64(t0v26, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v26, c1));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v11, c0), _mm_slli_epi64(t0v27, 32));\
    t0v11 = _mm_or_si128(_mm_srli_epi64(t0v11, 32), _mm_and_si128(t0v27, c1));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v12, c0), _mm_slli_epi64(t0v28, 32));\
    t0v12 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v28, c1));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v13, c0), _mm_slli_epi64(t0v29, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v13, 32), _mm_and_si128(t0v29, c1));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v14, c0), _mm_slli_epi64(t0v30, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v30, c1));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v15, c0), _mm_slli_epi64(t0v31, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v15, 32), _mm_and_si128(t0v31, c1));\
    (D0) = _mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_or_si128(_mm_and_si128(t0v32, c2), _mm_slli_epi32(_mm_and_si128(t0v0, c2), 1)), _mm_slli_epi32(_mm_and_si128(t0v16, c2), 2)), _mm_slli_epi32(_mm_and_si128(t0v1, c2), 3)), _mm_slli_epi32(_mm_and_si128(t0v17, c2), 4)), _mm_slli_epi32(_mm_and_si128(t0v2, c2), 5)), _mm_slli_epi32(_mm_and_si128(t0v18, c2), 6)), _mm_slli_epi32(_mm_and_si128(t0v3, c2), 7)), _mm_slli_epi32(_mm_and_si128(t0v19, c2), 8)), _mm_slli_epi32(_mm_and_si128(t0v4, c2), 9)), _mm_slli_epi32(_mm_and_si128(t0v20, c2), 10)), _mm_slli_epi32(_mm_and_si128(t0v5, c2), 11)), _mm_slli_epi32(_mm_and_si128(t0v21, c2), 12)), _mm_slli_epi32(_mm_and_si128(t0v6, c2), 13)), _mm_slli_epi32(_mm_and_si128(t0v22, c2), 14)), _mm_slli_epi32(_mm_and_si128(t0v7, c2), 15)), _mm_slli_epi32(_mm_and_si128(t0v23, c2), 16)), _mm_slli_epi32(_mm_and_si128(t0v8, c2), 17)), _mm_slli_epi32(_mm_and_si128(t0v24, c2), 18)), _mm_slli_epi32(_mm_and_si128(t0v9, c2), 19)), _mm_slli_epi32(_mm_and_si128(t0v25, c2), 20)), _mm_slli_epi32(_mm_and_si128(t0v10, c2), 21)), _mm_slli_epi32(_mm_and_si128(t0v26, c2), 22)), _mm_slli_epi32(_mm_and_si128(t0v11, c2), 23)), _mm_slli_epi32(_mm_and_si128(t0v27, c2), 24)), _mm_slli_epi32(_mm_and_si128(t0v12, c2), 25)), _mm_slli_epi32(_mm_and_si128(t0v28, c2), 26)), _mm_slli_epi32(_mm_and_si128(t0v13, c2), 27)), _mm_slli_epi32(_mm_and_si128(t0v29, c2), 28)), _mm_slli_epi32(_mm_and_si128(t0v14, c2), 29)), _mm_slli_epi32(_mm_and_si128(t0v30, c2), 30)), _mm_slli_epi32(t0v15, 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_AVX2.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    const __m256i c8 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c9 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c6 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*5));\
    const __m256i c2 = (*(const __m256i*)(transform_const_tbl + 8*6));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*7));\
    __m256i t0v0;\
    __m256i t0v1;\
    __m256i t0v2;\
    __m256i t0v3;\
    __m256i t0v4;\
    __m256i t0v5;\
    __m256i t0v6;\
    __m256i t0v7;\
    __m256i t0v8;\
    __m256i t0v9;\
    __m256i t0v10;\
    __m256i t0v11;\
    __m256i t0v12;\
    __m256i t0v13;\
    __m256i t0v14;\
    __m256i t0v15;\
    __m256i t0v16;\
    __m256i t0v17;\
    __m256i t0v18;\
    __m256i t0v19;\
    __m256i t0v20;\
    __m256i t0v21;\
    __m256i t0v22;\
    __m256i t0v23;\
    __m256i t0v24;\
    __m256i t0v25;\
    __m256i t0v26;\
    __m256i t0v27;\
    __m256i t0v28;\
    __m256i t0v29;\
    __m256i t0v30;\
    __m256i t0v31;\
    __m256i t0v32;\
    t0v0 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x20);\
    t0v1 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x31);\
    t0v2 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x20);\
    t0v3 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x31);\
    t0v4 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x20);\
    t0v5 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x31);\
    t0v6 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x20);\
    t0v7 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x31);\
    t0v8 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x20);\
    t0v9 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x31);\
    t0v10 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x20);\
    t0v11 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x31);\
    t0v12 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x20);\
    t0v13 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x31);\
    t0v14 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x20);\
    t0v15 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x31);\
    t0v16 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x20);\
    t0v17 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x31);\
    t0v18 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x20);\
    t0v19 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x31);\
    t0v20 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x20);\
    t0v21 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x31);\
    t0v22 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x20);\
    t0v23 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x31);\
    t0v24 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x20);\
    t0v25 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x31);\
    t0v26 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x20);\
    t0v27 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x31);\
    t0v28 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x20);\
    t0v29 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x31);\
    t0v30 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x20);\
    t0v31 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x31);\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v0, c0), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v0, (64)>>3), _mm256_and_si256(t0v16, c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v1, c0), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v1 = _mm256_or_si256(_mm256_srli_si256(t0v1, (64)>>3), _mm256_and_si256(t0v17, c1));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v2, c0), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v2 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v18, c1));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v3, c0), _mm256_slli_si256(t0v19, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v3, (64)>>3), _mm256_and_si256(t0v19, c1));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v4, c0), _mm256_slli_si256(t0v20, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v4, (64)>>3), _mm256_and_si256(t0v20, c1));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v5, c0), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v5 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v21, c1));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v6, c0), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v6 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v22, c1));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v7, c0), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v23, c1));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v8, c0), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v8, (64)>>3), _mm256_and_si256(t0v24, c1));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v9, c0), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v25, c1));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v10, c0), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v26, c1));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v11, c0), _mm256_slli_si256(t0v27, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v11, (64)>>3), _mm256_and_si256(t0v27, c1));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v12, c0), _mm256_slli_si256(t0v28, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v12, (64)>>3), _mm256_and_si256(t0v28, c1));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v13, c0), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v29, c1));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v14, c0), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v14 = _mm256_or_si256(_mm256_srli_si256(t0v14, (64)>>3), _mm256_and_si256(t0v30, c1));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v15, c0), _mm256_slli_si256(t0v31, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v31, c1));\
    t0v31 = _mm256_blend_epi32(t0v32, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v23 = _mm256_blend_epi32(_mm256_srli_epi64(t0v32, 32), t0v23, 0xaa);\
    t0v32 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v8, 0xaa);\
    t0v8 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v24, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v1, _mm256_slli_epi64(t0v9, 32), 0xaa);\
    t0v1 = _mm256_blend_epi32(_mm256_srli_epi64(t0v1, 32), t0v9, 0xaa);\
    t0v9 = _mm256_blend_epi32(t0v17, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v17 = _mm256_blend_epi32(_mm256_srli_epi64(t0v17, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v2, _mm256_slli_epi64(t0v10, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v2, 32), t0v10, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v26, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v3 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v19, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v19 = _mm256_blend_epi32(_mm256_srli_epi64(t0v19, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v4, _mm256_slli_epi64(t0v12, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v4, 32), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v28, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v21, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v21 = _mm256_blend_epi32(_mm256_srli_epi64(t0v21, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v14, 0xaa);\
    t0v14 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v30, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v7, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v7, 32), t0v15, 0xaa);\
    t0v15 = _mm256_blend_epi16(t0v31, _mm256_slli_epi32(t0v11, 16), 0xaa);\
    t0v11 = _mm256_blend_epi16(_mm256_srli_epi32(t0v31, 16), t0v11, 0xaa);\
    t0v31 = _mm256_blend_epi16(t0v23, _mm256_slli_epi32(t0v19, 16), 0xaa);\
    t0v19 = _mm256_blend_epi16(_mm256_srli_epi32(t0v23, 16), t0v19, 0xaa);\
    t0v23 = _mm256_blend_epi16(t0v32, _mm256_slli_epi32(t0v27, 16), 0xaa);\
    t0v27 = _mm256_blend_epi16(_mm256_srli_epi32(t0v32, 16), t0v27, 0xaa);\
    t0v32 = _mm256_blend_epi16(t0v0, _mm256_slli_epi32(t0v4, 16), 0xaa);\
    t0v0 = _mm256_blend_epi16(_mm256_srli_epi32(t0v0, 16), t0v4, 0xaa);\
    t0v4 = _mm256_blend_epi16(t0v8, _mm256_slli_epi32(t0v12, 16), 0xaa);\
    t0v8 = _mm256_blend_epi16(_mm256_srli_epi32(t0v8, 16), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi16(t0v16, _mm256_slli_epi32(t0v20, 16), 0xaa);\
    t0v16 = _mm256_blend_epi16(_mm256_srli_epi32(t0v16, 16), t0v20, 0xaa);\
    t0v20 = _mm256_blend_epi16(t0v24, _mm256_slli_epi32(t0v28, 16), 0xaa);\
    t0v24 = _mm256_blend_epi16(_mm256_srli_epi32(t0v24, 16), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi16(t0v1, _mm256_slli_epi32(t0v5, 16), 0xaa);\
    t0v1 = _mm256_blend_epi16(_mm256_srli_epi32(t0v1, 16), t0v5, 0xaa);\
    t0v5 = _mm256_blend_epi16(t0v9, _mm256_slli_epi32(t0v13, 16), 0xaa);\
    t0v9 = _mm256_blend_epi16(_mm256_srli_epi32(t0v9, 16), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi16(t0v17, _mm256_slli_epi32(t0v21, 16), 0xaa);\
    t0v17 = _mm256_blend_epi16(_mm256_srli_epi32(t0v17, 16), t0v21, 0xaa);\
    t0v21 = _mm256_blend_epi16(t0v25, _mm256_slli_epi32(t0v29, 16), 0xaa);\
    t0v25 = _mm256_blend_epi16(_mm256_srli_epi32(t0v25, 16), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi16(t0v2, _mm256_slli_epi32(t0v6, 16), 0xaa);\
    t0v2 = _mm256_blend_epi16(_mm256_srli_epi32(t0v2, 16), t0v6, 0xaa);\
    t0v6 = _mm256_blend_epi16(t0v10, _mm256_slli_epi32(t0v14, 16), 0xaa);\
    t0v10 = _mm256_blend_epi16(_mm256_srli_epi32(t0v10, 16), t0v14, 0xaa);\
    t0v14 = _mm256_blend_epi16(t0v18, _mm256_slli_epi32(t0v22, 16), 0xaa);\
    t0v18 = _mm256_blend_epi16(_mm256_srli_epi32(t0v18, 16), t0v22, 0xaa);\
    t0v22 = _mm256_blend_epi16(t0v26, _mm256_slli_epi32(t0v30, 16), 0xaa);\
    t0v26 = _mm256_blend_epi16(_mm256_srli_epi32(t0v26, 16), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi16(t0v3, _mm256_slli_epi32(t0v7, 16), 0xaa);\
    t0v3 = _mm256_blend_epi16(_mm256_srli_epi32(t0v3, 16), t0v7, 0xaa);\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v15, c2), _mm256_slli_epi16(t0v5, 8));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(t0v15, 8), _mm256_and_si256(t0v5, c3));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v31, c2), _mm256_slli_epi16(t0v13, 8));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(t0v31, 8), _mm256_and_si256(t0v13, c3));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v23, c2), _mm256_slli_epi16(t0v21, 8));\
    t0v21 = _mm256_or_si256(_mm256_srli_epi16(t0v23, 8), _mm256_and_si256(t0v21, c3));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v32, c2), _mm256_slli_epi16(t0v29, 8));\
    t0v29 = _mm256_or_si256(_mm256_srli_epi16(t0v32, 8), _mm256_and_si256(t0v29, c3));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v4, c2), _mm256_slli_epi16(t0v6, 8));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(t0v4, 8), _mm256_and_si256(t0v6, c3));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v12, c2), _mm256_slli_epi16(t0v14, 8));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(t0v12, 8), _mm256_and_si256(t0v14, c3));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v20, c2), _mm256_slli_epi16(t0v22, 8));\
    t0v20 = _mm256_or_si256(_mm256_srli_epi16(t0v20, 8), _mm256_and_si256(t0v22, c3));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v28, c2), _mm256_slli_epi16(t0v30, 8));\
    t0v28 = _mm256_or_si256(_mm256_srli_epi16(t0v28, 8), _mm256_and_si256(t0v30, c3));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v11, c2), _mm256_slli_epi16(t0v9, 8));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(t0v11, 8), _mm256_and_si256(t0v9, c3));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v19, c2), _mm256_slli_epi16(t0v17, 8));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(t0v19, 8), _mm256_and_si256(t0v17, c3));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v27, c2), _mm256_slli_epi16(t0v25, 8));\
    t0v25 = _mm256_or_si256(_mm256_srli_epi16(t0v27, 8), _mm256_and_si256(t0v25, c3));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi16(t0v2, 8));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(t0v0, 8), _mm256_and_si256(t0v2, c3));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v8, c2), _mm256_slli_epi16(t0v10, 8));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(t0v8, 8), _mm256_and_si256(t0v10, c3));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v16, c2), _mm256_slli_epi16(t0v18, 8));\
    t0v16 = _mm256_or_si256(_mm256_srli_epi16(t0v16, 8), _mm256_and_si256(t0v18, c3));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v24, c2), _mm256_slli_epi16(t0v26, 8));\
    t0v24 = _mm256_or_si256(_mm256_srli_epi16(t0v24, 8), _mm256_and_si256(t0v26, c3));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi16(t0v3, 8));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(t0v1, 8), _mm256_and_si256(t0v3, c3));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v7, c4), _mm256_slli_epi16(_mm256_and_si256(t0v32, c4), 4));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c5), 4), _mm256_and_si256(t0v32, c5));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v15, c4), _mm256_slli_epi16(_mm256_and_si256(t0v6, c4), 4));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v15, c5), 4), _mm256_and_si256(t0v6, c5));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v31, c4), _mm256_slli_epi16(_mm256_and_si256(t0v14, c4), 4));\
    t0v14 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v31, c5), 4), _mm256_and_si256(t0v14, c5));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v23, c4), _mm256_slli_epi16(_mm256_and_si256(t0v22, c4), 4));\
    t0v22 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v23, c5), 4), _mm256_and_si256(t0v22, c5));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v5, c4), _mm256_slli_epi16(_mm256_and_si256(t0v4, c4), 4));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c5), 4), _mm256_and_si256(t0v4, c5));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v13, c4), _mm256_slli_epi16(_mm256_and_si256(t0v12, c4), 4));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c5), 4), _mm256_and_si256(t0v12, c5));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v21, c4), _mm256_slli_epi16(_mm256_and_si256(t0v20, c4), 4));\
    t0v20 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v21, c5), 4), _mm256_and_si256(t0v20, c5));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v29, c4), _mm256_slli_epi16(_mm256_and_si256(t0v28, c4), 4));\
    t0v28 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v29, c5), 4), _mm256_and_si256(t0v28, c5));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v30, c4), _mm256_slli_epi16(_mm256_and_si256(t0v2, c4), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v30, c5), 4), _mm256_and_si256(t0v2, c5));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v11, c4), _mm256_slli_epi16(_mm256_and_si256(t0v10, c4), 4));\
    t0v10 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v11, c5), 4), _mm256_and_si256(t0v10, c5));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v19, c4), _mm256_slli_epi16(_mm256_and_si256(t0v18, c4), 4));\
    t0v18 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v19, c5), 4), _mm256_and_si256(t0v18, c5));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v27, c4), _mm256_slli_epi16(_mm256_and_si256(t0v26, c4), 4));\
    t0v26 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v27, c5), 4), _mm256_and_si256(t0v26, c5));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v9, c4), _mm256_slli_epi16(_mm256_and_si256(t0v8, c4), 4));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c5), 4), _mm256_and_si256(t0v8, c5));\
    t0v9 = _mm256_or_si256(_mm256_and_si256(t0v17, c4), _mm256_slli_epi16(_mm256_and_si256(t0v16, c4), 4));\
    t0v16 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v17, c5), 4), _mm256_and_si256(t0v16, c5));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v25, c4), _mm256_slli_epi16(_mm256_and_si256(t0v24, c4), 4));\
    t0v24 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v25, c5), 4), _mm256_and_si256(t0v24, c5));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v0, c4), _mm256_slli_epi16(_mm256_and_si256(t0v1, c4), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c5), 4), _mm256_and_si256(t0v1, c5));\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v3, c6), _mm256_slli_epi16(_mm256_and_si256(t0v15, c6), 2));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c7), 2), _mm256_and_si256(t0v15, c7));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v32, c6), _mm256_slli_epi16(_mm256_and_si256(t0v31, c6), 2));\
    t0v31 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v32, c7), 2), _mm256_and_si256(t0v31, c7));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v7, c6), _mm256_slli_epi16(_mm256_and_si256(t0v14, c6), 2));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c7), 2), _mm256_and_si256(t0v14, c7));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v6, c6), _mm256_slli_epi16(_mm256_and_si256(t0v22, c6), 2));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v6, c7), 2), _mm256_and_si256(t0v22, c7));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v23, c6), _mm256_slli_epi16(_mm256_and_si256(t0v13, c6), 2));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v23, c7), 2), _mm256_and_si256(t0v13, c7));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v5, c6), _mm256_slli_epi16(_mm256_and_si256(t0v21, c6), 2));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c7), 2), _mm256_and_si256(t0v21, c7));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v4, c6), _mm256_slli_epi16(_mm256_and_si256(t0v20, c6), 2));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c7), 2), _mm256_and_si256(t0v20, c7));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v12, c6), _mm256_slli_epi16(_mm256_and_si256(t0v28, c6), 2));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v12, c7), 2), _mm256_and_si256(t0v28, c7));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v29, c6), _mm256_slli_epi16(_mm256_and_si256(t0v11, c6), 2));\
    t0v11 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v29, c7), 2), _mm256_and_si256(t0v11, c7));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v30, c6), _mm256_slli_epi16(_mm256_and_si256(t0v19, c6), 2));\
    t0v19 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v30, c7), 2), _mm256_and_si256(t0v19, c7));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v2, c6), _mm256_slli_epi16(_mm256_and_si256(t0v18, c6), 2));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c7), 2), _mm256_and_si256(t0v18, c7));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v10, c6), _mm256_slli_epi16(_mm256_and_si256(t0v26, c6), 2));\
    t0v10 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v10, c7), 2), _mm256_and_si256(t0v26, c7));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v27, c6), _mm256_slli_epi16(_mm256_and_si256(t0v17, c6), 2));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v27, c7), 2), _mm256_and_si256(t0v17, c7));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v9, c6), _mm256_slli_epi16(_mm256_and_si256(t0v25, c6), 2));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c7), 2), _mm256_and_si256(t0v25, c7));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v8, c6), _mm256_slli_epi16(_mm256_and_si256(t0v24, c6), 2));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c7), 2), _mm256_and_si256(t0v24, c7));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v16, c6), _mm256_slli_epi16(_mm256_and_si256(t0v0, c6), 2));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c7), 2), _mm256_and_si256(t0v0, c7));\
    (D0) = _mm256_or_si256(_mm256_and_si256(t0v1, c8), _mm256_slli_epi16(_mm256_and_si256(t0v15, c8), 1));\
    (D1) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c9), 1), _mm256_and_si256(t0v15, c9));\
    (D2) = _mm256_or_si256(_mm256_and_si256(t0v3, c8), _mm256_slli_epi16(_mm256_and_si256(t0v31, c8), 1));\
    (D3) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c9), 1), _mm256_and_si256(t0v31, c9));\
    (D4) = _mm256_or_si256(_mm256_and_si256(t0v32, c8), _mm256_slli_epi16(_mm256_and_si256(t0v14, c8), 1));\
    (D5) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v32, c9), 1), _mm256_and_si256(t0v14, c9));\
    (D6) = _mm256_or_si256(_mm256_and_si256(t0v7, c8), _mm256_slli_epi16(_mm256_and_si256(t0v6, c8), 1));\
    (D7) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c9), 1), _mm256_and_si256(t0v6, c9));\
    (D8) = _mm256_or_si256(_mm256_and_si256(t0v22, c8), _mm256_slli_epi16(_mm256_and_si256(t0v23, c8), 1));\
    (D9) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v22, c9), 1), _mm256_and_si256(t0v23, c9));\
    (D10) = _mm256_or_si256(_mm256_and_si256(t0v13, c8), _mm256_slli_epi16(_mm256_and_si256(t0v5, c8), 1));\
    (D11) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c9), 1), _mm256_and_si256(t0v5, c9));\
    (D12) = _mm256_or_si256(_mm256_and_si256(t0v21, c8), _mm256_slli_epi16(_mm256_and_si256(t0v20, c8), 1));\
    (D13) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v21, c9), 1), _mm256_and_si256(t0v20, c9));\
    (D14) = _mm256_or_si256(_mm256_and_si256(t0v4, c8), _mm256_slli_epi16(_mm256_and_si256(t0v12, c8), 1));\
    (D15) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c9), 1), _mm256_and_si256(t0v12, c9));\
    (D16) = _mm256_or_si256(_mm256_and_si256(t0v28, c8), _mm256_slli_epi16(_mm256_and_si256(t0v29, c8), 1));\
    (D17) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v28, c9), 1), _mm256_and_si256(t0v29, c9));\
    (D18) = _mm256_or_si256(_mm256_and_si256(t0v11, c8), _mm256_slli_epi16(_mm256_and_si256(t0v19, c8), 1));\
    (D19) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v11, c9), 1), _mm256_and_si256(t0v19, c9));\
    (D20) = _mm256_or_si256(_mm256_and_si256(t0v30, c8), _mm256_slli_epi16(_mm256_and_si256(t0v18, c8), 1));\
    (D21) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v30, c9), 1), _mm256_and_si256(t0v18, c9));\
    (D22) = _mm256_or_si256(_mm256_and_si256(t0v2, c8), _mm256_slli_epi16(_mm256_and_si256(t0v10, c8), 1));\
    (D23) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c9), 1), _mm256_and_si256(t0v10, c9));\
    (D24) = _mm256_or_si256(_mm256_and_si256(t0v26, c8), _mm256_slli_epi16(_mm256_and_si256(t0v27, c8), 1));\
    (D25) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v26, c9), 1), _mm256_and_si256(t0v27, c9));\
    (D26) = _mm256_or_si256(_mm256_and_si256(t0v17, c8), _mm256_slli_epi16(_mm256_and_si256(t0v9, c8), 1));\
    (D27) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v17, c9), 1), _mm256_and_si256(t0v9, c9));\
    (D28) = _mm256_or_si256(_mm256_and_si256(t0v25, c8), _mm256_slli_epi16(_mm256_and_si256(t0v24, c8), 1));\
    (D29) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v25, c9), 1), _mm256_and_si256(t0v24, c9));\
    (D30) = _mm256_or_si256(_mm256_and_si256(t0v8, c8), _mm256_slli_epi16(_mm256_and_si256(t0v0, c8), 1));\
    (D31) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c9), 1), _mm256_and_si256(t0v0, c9));\
}
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    const __m256i c2 = (*(const __m256i*)(transform_const2_tbl + 8*4));\
    const __m256i c9 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c10 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c8 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c6 = (*(const __m256i*)(transform_const_tbl + 8*5));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*6));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*7));\
    __m256i t0v0;\
    __m256i t0v1;\
    __m256i t0v2;\
    __m256i t0v3;\
    __m256i t0v4;\
    __m256i t0v5;\
    __m256i t0v6;\
    __m256i t0v7;\
    __m256i t0v8;\
    __m256i t0v9;\
    __m256i t0v10;\
    __m256i t0v11;\
    __m256i t0v12;\
    __m256i t0v13;\
    __m256i t0v14;\
    __m256i t0v15;\
    __m256i t0v16;\
    __m256i t0v17;\
    __m256i t0v18;\
    __m256i t0v19;\
    __m256i t0v20;\
    __m256i t0v21;\
    __m256i t0v22;\
    __m256i t0v23;\
    __m256i t0v24;\
    __m256i t0v25;\
    __m256i t0v26;\
    __m256i t0v27;\
    __m256i t0v28;\
    __m256i t0v29;\
    __m256i t0v30;\
    __m256i t0v31;\
    __m256i t0v32;\
    t0v0 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x20);\
    t0v1 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x31);\
    t0v2 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x20);\
    t0v3 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x31);\
    t0v4 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x20);\
    t0v5 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x31);\
    t0v6 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x20);\
    t0v7 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x31);\
    t0v8 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x20);\
    t0v9 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x31);\
    t0v10 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x20);\
    t0v11 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x31);\
    t0v12 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x20);\
    t0v13 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x31);\
    t0v14 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x20);\
    t0v15 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x31);\
    t0v16 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x20);\
    t0v17 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x31);\
    t0v18 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x20);\
    t0v19 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x31);\
    t0v20 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x20);\
    t0v21 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x31);\
    t0v22 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x20);\
    t0v23 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x31);\
    t0v24 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x20);\
    t0v25 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x31);\
    t0v26 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x20);\
    t0v27 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x31);\
    t0v28 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x20);\
    t0v29 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x31);\
    t0v30 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x20);\
    t0v31 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x31);\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v0, c0), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v0, (64)>>3), _mm256_and_si256(t0v16, c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v1, c0), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v1 = _mm256_or_si256(_mm256_srli_si256(t0v1, (64)>>3), _mm256_and_si256(t0v17, c1));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v2, c0), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v2 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v18, c1));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v3, c0), _mm256_slli_si256(t0v19, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v3, (64)>>3), _mm256_and_si256(t0v19, c1));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v4, c0), _mm256_slli_si256(t0v20, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v4, (64)>>3), _mm256_and_si256(t0v20, c1));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v5, c0), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v5 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v21, c1));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v6, c0), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v6 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v22, c1));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v7, c0), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v23, c1));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v8, c0), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v8, (64)>>3), _mm256_and_si256(t0v24, c1));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v9, c0), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v25, c1));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v10, c0), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v26, c1));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v11, c0), _mm256_slli_si256(t0v27, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v11, (64)>>3), _mm256_and_si256(t0v27, c1));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v12, c0), _mm256_slli_si256(t0v28, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v12, (64)>>3), _mm256_and_si256(t0v28, c1));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v13, c0), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v29, c1));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v14, c0), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v14 = _mm256_or_si256(_mm256_srli_si256(t0v14, (64)>>3), _mm256_and_si256(t0v30, c1));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v15, c0), _mm256_slli_si256(t0v31, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v31, c1));\
    t0v31 = _mm256_blend_epi32(t0v32, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v23 = _mm256_blend_epi32(_mm256_srli_epi64(t0v32, 32), t0v23, 0xaa);\
    t0v32 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v8, 0xaa);\
    t0v8 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v24, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v1, _mm256_slli_epi64(t0v9, 32), 0xaa);\
    t0v1 = _mm256_blend_epi32(_mm256_srli_epi64(t0v1, 32), t0v9, 0xaa);\
    t0v9 = _mm256_blend_epi32(t0v17, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v17 = _mm256_blend_epi32(_mm256_srli_epi64(t0v17, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v2, _mm256_slli_epi64(t0v10, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v2, 32), t0v10, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v26, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v3 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v19, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v19 = _mm256_blend_epi32(_mm256_srli_epi64(t0v19, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v4, _mm256_slli_epi64(t0v12, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v4, 32), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v28, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v21, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v21 = _mm256_blend_epi32(_mm256_srli_epi64(t0v21, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v14, 0xaa);\
    t0v14 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v30, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v7, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v7, 32), t0v15, 0xaa);\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v31, c2), _mm256_slli_epi32(t0v11, 16));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v23, c2), _mm256_slli_epi32(t0v19, 16));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v32, c2), _mm256_slli_epi32(t0v27, 16));\
    t0v0 = _mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi32(t0v4, 16));\
    t0v4 = _mm256_or_si256(_mm256_and_si256(t0v8, c2), _mm256_slli_epi32(t0v12, 16));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v16, c2), _mm256_slli_epi32(t0v20, 16));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v24, c2), _mm256_slli_epi32(t0v28, 16));\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi32(t0v5, 16));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v9, c2), _mm256_slli_epi32(t0v13, 16));\
    t0v9 = _mm256_or_si256(_mm256_and_si256(t0v17, c2), _mm256_slli_epi32(t0v21, 16));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v25, c2), _mm256_slli_epi32(t0v29, 16));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v2, c2), _mm256_slli_epi32(t0v6, 16));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v10, c2), _mm256_slli_epi32(t0v14, 16));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v18, c2), _mm256_slli_epi32(t0v22, 16));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v26, c2), _mm256_slli_epi32(t0v30, 16));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v3, c2), _mm256_slli_epi32(t0v7, 16));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v11, c3), _mm256_slli_epi16(t0v5, 8));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(t0v11, 8), _mm256_and_si256(t0v5, c4));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v15, c3), _mm256_slli_epi16(t0v9, 8));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(t0v15, 8), _mm256_and_si256(t0v9, c4));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v19, c3), _mm256_slli_epi16(t0v13, 8));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(t0v19, 8), _mm256_and_si256(t0v13, c4));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v0, c3), _mm256_slli_epi16(t0v2, 8));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(t0v0, 8), _mm256_and_si256(t0v2, c4));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v4, c3), _mm256_slli_epi16(t0v6, 8));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(t0v4, 8), _mm256_and_si256(t0v6, c4));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v8, c3), _mm256_slli_epi16(t0v10, 8));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(t0v8, 8), _mm256_and_si256(t0v10, c4));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v12, c3), _mm256_slli_epi16(t0v14, 8));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(t0v12, 8), _mm256_and_si256(t0v14, c4));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v1, c3), _mm256_slli_epi16(t0v3, 8));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(t0v1, 8), _mm256_and_si256(t0v3, c4));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v7, c5), _mm256_slli_epi16(_mm256_and_si256(t0v2, c5), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c6), 4), _mm256_and_si256(t0v2, c6));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v11, c5), _mm256_slli_epi16(_mm256_and_si256(t0v6, c5), 4));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v11, c6), 4), _mm256_and_si256(t0v6, c6));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v15, c5), _mm256_slli_epi16(_mm256_and_si256(t0v10, c5), 4));\
    t0v10 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v15, c6), 4), _mm256_and_si256(t0v10, c6));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v16, c5), _mm256_slli_epi16(_mm256_and_si256(t0v14, c5), 4));\
    t0v14 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c6), 4), _mm256_and_si256(t0v14, c6));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v5, c5), _mm256_slli_epi16(_mm256_and_si256(t0v4, c5), 4));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c6), 4), _mm256_and_si256(t0v4, c6));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v9, c5), _mm256_slli_epi16(_mm256_and_si256(t0v8, c5), 4));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c6), 4), _mm256_and_si256(t0v8, c6));\
    t0v9 = _mm256_or_si256(_mm256_and_si256(t0v13, c5), _mm256_slli_epi16(_mm256_and_si256(t0v12, c5), 4));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c6), 4), _mm256_and_si256(t0v12, c6));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v0, c5), _mm256_slli_epi16(_mm256_and_si256(t0v1, c5), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c6), 4), _mm256_and_si256(t0v1, c6));\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v3, c7), _mm256_slli_epi16(_mm256_and_si256(t0v11, c7), 2));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c8), 2), _mm256_and_si256(t0v11, c8));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v7, c7), _mm256_slli_epi16(_mm256_and_si256(t0v15, c7), 2));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c8), 2), _mm256_and_si256(t0v15, c8));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v2, c7), _mm256_slli_epi16(_mm256_and_si256(t0v10, c7), 2));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c8), 2), _mm256_and_si256(t0v10, c8));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v6, c7), _mm256_slli_epi16(_mm256_and_si256(t0v14, c7), 2));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v6, c8), 2), _mm256_and_si256(t0v14, c8));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v16, c7), _mm256_slli_epi16(_mm256_and_si256(t0v9, c7), 2));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c8), 2), _mm256_and_si256(t0v9, c8));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v5, c7), _mm256_slli_epi16(_mm256_and_si256(t0v13, c7), 2));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c8), 2), _mm256_and_si256(t0v13, c8));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v4, c7), _mm256_slli_epi16(_mm256_and_si256(t0v12, c7), 2));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c8), 2), _mm256_and_si256(t0v12, c8));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v8, c7), _mm256_slli_epi16(_mm256_and_si256(t0v0, c7), 2));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c8), 2), _mm256_and_si256(t0v0, c8));\
    (D0) = _mm256_or_si256(_mm256_and_si256(t0v1, c9), _mm256_slli_epi16(_mm256_and_si256(t0v11, c9), 1));\
    (D1) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c10), 1), _mm256_and_si256(t0v11, c10));\
    (D2) = _mm256_or_si256(_mm256_and_si256(t0v3, c9), _mm256_slli_epi16(_mm256_and_si256(t0v7, c9), 1));\
    (D3) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c10), 1), _mm256_and_si256(t0v7, c10));\
    (D4) = _mm256_or_si256(_mm256_and_si256(t0v15, c9), _mm256_slli_epi16(_mm256_and_si256(t0v10, c9), 1));\
    (D5) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v15, c10), 1), _mm256_and_si256(t0v10, c10));\
    (D6) = _mm256_or_si256(_mm256_and_si256(t0v2, c9), _mm256_slli_epi16(_mm256_and_si256(t0v6, c9), 1));\
    (D7) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c10), 1), _mm256_and_si256(t0v6, c10));\
    (D8) = _mm256_or_si256(_mm256_and_si256(t0v14, c9), _mm256_slli_epi16(_mm256_and_si256(t0v16, c9), 1));\
    (D9) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v14, c10), 1), _mm256_and_si256(t0v16, c10));\
    (D10) = _mm256_or_si256(_mm256_and_si256(t0v9, c9), _mm256_slli_epi16(_mm256_and_si256(t0v5, c9), 1));\
    (D11) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c10), 1), _mm256_and_si256(t0v5, c10));\
    (D12) = _mm256_or_si256(_mm256_and_si256(t0v13, c9), _mm256_slli_epi16(_mm256_and_si256(t0v12, c9), 1));\
    (D13) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c10), 1), _mm256_and_si256(t0v12, c10));\
    (D14) = _mm256_or_si256(_mm256_and_si256(t0v4, c9), _mm256_slli_epi16(_mm256_and_si256(t0v0, c9), 1));\
    (D15) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c10), 1), _mm256_and_si256(t0v0, c10));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
{\
    const __m256i c2 = (*(const __m256i*)(transform_const2_tbl + 8*3));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c8 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c6 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*5));\
    __m256i t0v0;\
    __m256i t0v1;\
    __m256i t0v2;\
    __m256i t0v3;\
    __m256i t0v4;\
    __m256i t0v5;\
    __m256i t0v6;\
    __m256i t0v7;\
    __m256i t0v8;\
    __m256i t0v9;\
    __m256i t0v10;\
    __m256i t0v11;\
    __m256i t0v12;\
    __m256i t0v13;\
    __m256i t0v14;\
    __m256i t0v15;\
    __m256i t0v16;\
    __m256i t0v17;\
    __m256i t0v18;\
    __m256i t0v19;\
    __m256i t0v20;\
    __m256i t0v21;\
    __m256i t0v22;\
    __m256i t0v23;\
    __m256i t0v24;\
    __m256i t0v25;\
    __m256i t0v26;\
    __m256i t0v27;\
    __m256i t0v28;\
    __m256i t0v29;\
    __m256i t0v30;\
    __m256i t0v31;\
    __m256i t0v32;\
    t0v0 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x20);\
    t0v1 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x31);\
    t0v2 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x20);\
    t0v3 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x31);\
    t0v4 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x20);\
    t0v5 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x31);\
    t0v6 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x20);\
    t0v7 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x31);\
    t0v8 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x20);\
    t0v9 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x31);\
    t0v10 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x20);\
    t0v11 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x31);\
    t0v12 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x20);\
    t0v13 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x31);\
    t0v14 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x20);\
    t0v15 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x31);\
    t0v16 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x20);\
    t0v17 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x31);\
    t0v18 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x20);\
    t0v19 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x31);\
    t0v20 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x20);\
    t0v21 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x31);\
    t0v22 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x20);\
    t0v23 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x31);\
    t0v24 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x20);\
    t0v25 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x31);\
    t0v26 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x20);\
    t0v27 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x31);\
    t0v28 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x20);\
    t0v29 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x31);\
    t0v30 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x20);\
    t0v31 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x31);\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v0, c0), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v0, (64)>>3), _mm256_and_si256(t0v16, c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v1, c0), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v1 = _mm256_or_si256(_mm256_srli_si256(t0v1, (64)>>3), _mm256_and_si256(t0v17, c1));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v2, c0), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v2 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v18, c1));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v3, c0), _mm256_slli_si256(t0v19, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v3, (64)>>3), _mm256_and_si256(t0v19, c1));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v4, c0), _mm256_slli_si256(t0v20, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v4, (64)>>3), _mm256_and_si256(t0v20, c1));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v5, c0), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v5 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v21, c1));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v6, c0), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v6 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v22, c1));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v7, c0), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v23, c1));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v8, c0), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v8, (64)>>3), _mm256_and_si256(t0v24, c1));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v9, c0), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v25, c1));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v10, c0), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v26, c1));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v11, c0), _mm256_slli_si256(t0v27, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v11, (64)>>3), _mm256_and_si256(t0v27, c1));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v12, c0), _mm256_slli_si256(t0v28, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v12, (64)>>3), _mm256_and_si256(t0v28, c1));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v13, c0), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v29, c1));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v14, c0), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v14 = _mm256_or_si256(_mm256_srli_si256(t0v14, (64)>>3), _mm256_and_si256(t0v30, c1));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v15, c0), _mm256_slli_si256(t0v31, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v31, c1));\
    t0v31 = _mm256_blend_epi32(t0v32, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v23 = _mm256_blend_epi32(_mm256_srli_epi64(t0v32, 32), t0v23, 0xaa);\
    t0v32 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v8, 0xaa);\
    t0v8 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v24, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v1, _mm256_slli_epi64(t0v9, 32), 0xaa);\
    t0v1 = _mm256_blend_epi32(_mm256_srli_epi64(t0v1, 32), t0v9, 0xaa);\
    t0v9 = _mm256_blend_epi32(t0v17, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v17 = _mm256_blend_epi32(_mm256_srli_epi64(t0v17, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v2, _mm256_slli_epi64(t0v10, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v2, 32), t0v10, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v26, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v3 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v19, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v19 = _mm256_blend_epi32(_mm256_srli_epi64(t0v19, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v4, _mm256_slli_epi64(t0v12, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v4, 32), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v28, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v21, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v21 = _mm256_blend_epi32(_mm256_srli_epi64(t0v21, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v14, 0xaa);\
    t0v14 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v30, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v7, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v7, 32), t0v15, 0xaa);\
    t0v9 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v31, c2), _mm256_slli_epi32(_mm256_and_si256(t0v9, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v11, c2), 16)), _mm256_slli_epi32(t0v13, 24));\
    t0v11 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v23, c2), _mm256_slli_epi32(_mm256_and_si256(t0v17, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v19, c2), 16)), _mm256_slli_epi32(t0v21, 24));\
    t0v13 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v32, c2), _mm256_slli_epi32(_mm256_and_si256(t0v25, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v27, c2), 16)), _mm256_slli_epi32(t0v29, 24));\
    t0v0 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi32(_mm256_and_si256(t0v2, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v4, c2), 16)), _mm256_slli_epi32(t0v6, 24));\
    t0v2 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v8, c2), _mm256_slli_epi32(_mm256_and_si256(t0v10, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v12, c2), 16)), _mm256_slli_epi32(t0v14, 24));\
    t0v4 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v16, c2), _mm256_slli_epi32(_mm256_and_si256(t0v18, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v20, c2), 16)), _mm256_slli_epi32(t0v22, 24));\
    t0v6 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v24, c2), _mm256_slli_epi32(_mm256_and_si256(t0v26, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v28, c2), 16)), _mm256_slli_epi32(t0v30, 24));\
    t0v1 = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi32(_mm256_and_si256(t0v3, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v5, c2), 16)), _mm256_slli_epi32(t0v7, 24));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v9, c3), _mm256_slli_epi16(_mm256_and_si256(t0v2, c3), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c4), 4), _mm256_and_si256(t0v2, c4));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v11, c3), _mm256_slli_epi16(_mm256_and_si256(t0v4, c3), 4));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v11, c4), 4), _mm256_and_si256(t0v4, c4));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v13, c3), _mm256_slli_epi16(_mm256_and_si256(t0v6, c3), 4));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c4), 4), _mm256_and_si256(t0v6, c4));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v0, c3), _mm256_slli_epi16(_mm256_and_si256(t0v1, c3), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c4), 4), _mm256_and_si256(t0v1, c4));\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v3, c5), _mm256_slli_epi16(_mm256_and_si256(t0v7, c5), 2));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c6), 2), _mm256_and_si256(t0v7, c6));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v5, c5), _mm256_slli_epi16(_mm256_and_si256(t0v8, c5), 2));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c6), 2), _mm256_and_si256(t0v8, c6));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v2, c5), _mm256_slli_epi16(_mm256_and_si256(t0v6, c5), 2));\
    t0v0 = _mm256_or_si256(_mm256_and_si256(t0v4, c5), _mm256_slli_epi16(_mm256_and_si256(t0v0, c5), 2));\
    (D0) = _mm256_or_si256(_mm256_and_si256(t0v1, c7), _mm256_slli_epi16(_mm256_and_si256(t0v7, c7), 1));\
    (D1) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c8), 1), _mm256_and_si256(t0v7, c8));\
    (D2) = _mm256_or_si256(_mm256_and_si256(t0v3, c7), _mm256_slli_epi16(_mm256_and_si256(t0v5, c7), 1));\
    (D3) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v3, c8), 1), _mm256_and_si256(t0v5, c8));\
    (D4) = _mm256_or_si256(_mm256_and_si256(t0v2, c7), _mm256_slli_epi16(_mm256_and_si256(t0v0, c7), 1));\
    (D5) = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c8), 1), _mm256_and_si256(t0v0, c8));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    const __m256i c2 = (*(const __m256i*)(transform_const2_tbl + 8*0));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    __m256i t0v0;\
    __m256i t0v1;\
    __m256i t0v2;\
    __m256i t0v3;\
    __m256i t0v4;\
    __m256i t0v5;\
    __m256i t0v6;\
    __m256i t0v7;\
    __m256i t0v8;\
    __m256i t0v9;\
    __m256i t0v10;\
    __m256i t0v11;\
    __m256i t0v12;\
    __m256i t0v13;\
    __m256i t0v14;\
    __m256i t0v15;\
    __m256i t0v16;\
    __m256i t0v17;\
    __m256i t0v18;\
    __m256i t0v19;\
    __m256i t0v20;\
    __m256i t0v21;\
    __m256i t0v22;\
    __m256i t0v23;\
    __m256i t0v24;\
    __m256i t0v25;\
    __m256i t0v26;\
    __m256i t0v27;\
    __m256i t0v28;\
    __m256i t0v29;\
    __m256i t0v30;\
    __m256i t0v31;\
    __m256i t0v32;\
    t0v0 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x20);\
    t0v1 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 0)), _mm256_loadu_si256((const __m256i*)((S) + 128)), 0x31);\
    t0v2 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x20);\
    t0v3 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 8)), _mm256_loadu_si256((const __m256i*)((S) + 136)), 0x31);\
    t0v4 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x20);\
    t0v5 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 16)), _mm256_loadu_si256((const __m256i*)((S) + 144)), 0x31);\
    t0v6 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x20);\
    t0v7 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 24)), _mm256_loadu_si256((const __m256i*)((S) + 152)), 0x31);\
    t0v8 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x20);\
    t0v9 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 32)), _mm256_loadu_si256((const __m256i*)((S) + 160)), 0x31);\
    t0v10 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x20);\
    t0v11 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 40)), _mm256_loadu_si256((const __m256i*)((S) + 168)), 0x31);\
    t0v12 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x20);\
    t0v13 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 48)), _mm256_loadu_si256((const __m256i*)((S) + 176)), 0x31);\
    t0v14 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x20);\
    t0v15 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 56)), _mm256_loadu_si256((const __m256i*)((S) + 184)), 0x31);\
    t0v16 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x20);\
    t0v17 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 64)), _mm256_loadu_si256((const __m256i*)((S) + 192)), 0x31);\
    t0v18 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x20);\
    t0v19 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 72)), _mm256_loadu_si256((const __m256i*)((S) + 200)), 0x31);\
    t0v20 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x20);\
    t0v21 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 80)), _mm256_loadu_si256((const __m256i*)((S) + 208)), 0x31);\
    t0v22 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x20);\
    t0v23 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 88)), _mm256_loadu_si256((const __m256i*)((S) + 216)), 0x31);\
    t0v24 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x20);\
    t0v25 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 96)), _mm256_loadu_si256((const __m256i*)((S) + 224)), 0x31);\
    t0v26 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x20);\
    t0v27 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 104)), _mm256_loadu_si256((const __m256i*)((S) + 232)), 0x31);\
    t0v28 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x20);\
    t0v29 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 112)), _mm256_loadu_si256((const __m256i*)((S) + 240)), 0x31);\
    t0v30 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x20);\
    t0v31 = _mm256_permute2x128_si256(_mm256_loadu_si256((const __m256i*)((S) + 120)), _mm256_loadu_si256((const __m256i*)((S) + 248)), 0x31);\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v0, c0), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v0, (64)>>3), _mm256_and_si256(t0v16, c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v1, c0), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v1 = _mm256_or_si256(_mm256_srli_si256(t0v1, (64)>>3), _mm256_and_si256(t0v17, c1));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v2, c0), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v2 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v18, c1));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v3, c0), _mm256_slli_si256(t0v19, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v3, (64)>>3), _mm256_and_si256(t0v19, c1));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v4, c0), _mm256_slli_si256(t0v20, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v4, (64)>>3), _mm256_and_si256(t0v20, c1));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v5, c0), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v5 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v21, c1));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v6, c0), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v6 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v22, c1));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v7, c0), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v23, c1));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v8, c0), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v8, (64)>>3), _mm256_and_si256(t0v24, c1));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v9, c0), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v25, c1));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v10, c0), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v26, c1));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v11, c0), _mm256_slli_si256(t0v27, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v11, (64)>>3), _mm256_and_si256(t0v27, c1));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v12, c0), _mm256_slli_si256(t0v28, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v12, (64)>>3), _mm256_and_si256(t0v28, c1));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v13, c0), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v29, c1));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v14, c0), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v14 = _mm256_or_si256(_mm256_srli_si256(t0v14, (64)>>3), _mm256_and_si256(t0v30, c1));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v15, c0), _mm256_slli_si256(t0v31, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v31, c1));\
    t0v31 = _mm256_blend_epi32(t0v32, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v23 = _mm256_blend_epi32(_mm256_srli_epi64(t0v32, 32), t0v23, 0xaa);\
    t0v32 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v8, 0xaa);\
    t0v8 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v24, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v1, _mm256_slli_epi64(t0v9, 32), 0xaa);\
    t0v1 = _mm256_blend_epi32(_mm256_srli_epi64(t0v1, 32), t0v9, 0xaa);\
    t0v9 = _mm256_blend_epi32(t0v17, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v17 = _mm256_blend_epi32(_mm256_srli_epi64(t0v17, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v2, _mm256_slli_epi64(t0v10, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v2, 32), t0v10, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v26, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v3 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v19, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v19 = _mm256_blend_epi32(_mm256_srli_epi64(t0v19, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v4, _mm256_slli_epi64(t0v12, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v4, 32), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v28, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v21, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v21 = _mm256_blend_epi32(_mm256_srli_epi64(t0v21, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v14, 0xaa);\
    t0v14 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v30, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v7, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v7, 32), t0v15, 0xaa);\
    (D0) = _mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_or_si256(_mm256_and_si256(t0v31, c2), _mm256_slli_epi32(_mm256_and_si256(t0v23, c2), 1)), _mm256_slli_epi32(_mm256_and_si256(t0v32, c2), 2)), _mm256_slli_epi32(_mm256_and_si256(t0v0, c2), 3)), _mm256_slli_epi32(_mm256_and_si256(t0v8, c2), 4)), _mm256_slli_epi32(_mm256_and_si256(t0v16, c2), 5)), _mm256_slli_epi32(_mm256_and_si256(t0v24, c2), 6)), _mm256_slli_epi32(_mm256_and_si256(t0v1, c2), 7)), _mm256_slli_epi32(_mm256_and_si256(t0v9, c2), 8)), _mm256_slli_epi32(_mm256_and_si256(t0v17, c2), 9)), _mm256_slli_epi32(_mm256_and_si256(t0v25, c2), 10)), _mm256_slli_epi32(_mm256_and_si256(t0v2, c2), 11)), _mm256_slli_epi32(_mm256_and_si256(t0v10, c2), 12)), _mm256_slli_epi32(_mm256_and_si256(t0v18, c2), 13)), _mm256_slli_epi32(_mm256_and_si256(t0v26, c2), 14)), _mm256_slli_epi32(_mm256_and_si256(t0v3, c2), 15)), _mm256_slli_epi32(_mm256_and_si256(t0v11, c2), 16)), _mm256_slli_epi32(_mm256_and_si256(t0v19, c2), 17)), _mm256_slli_epi32(_mm256_and_si256(t0v27, c2), 18)), _mm256_slli_epi32(_mm256_and_si256(t0v4, c2), 19)), _mm256_slli_epi32(_mm256_and_si256(t0v12, c2), 20)), _mm256_slli_epi32(_mm256_and_si256(t0v20, c2), 21)), _mm256_slli_epi32(_mm256_and_si256(t0v28, c2), 22)), _mm256_slli_epi32(_mm256_and_si256(t0v5, c2), 23)), _mm256_slli_epi32(_mm256_and_si256(t0v13, c2), 24)), _mm256_slli_epi32(_mm256_and_si256(t0v21, c2), 25)), _mm256_slli_epi32(_mm256_and_si256(t0v29, c2), 26)), _mm256_slli_epi32(_mm256_and_si256(t0v6, c2), 27)), _mm256_slli_epi32(_mm256_and_si256(t0v14, c2), 28)), _mm256_slli_epi32(_mm256_and_si256(t0v22, c2), 29)), _mm256_slli_epi32(_mm256_and_si256(t0v30, c2), 30)), _mm256_slli_epi32(t0v7, 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_AVX512.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    const __m512i c8 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c9 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c6 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c7 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*5));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*6));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*7));\
    const __m512i c0 = (*(const __m512i*)(transform_const_tbl + 16*8));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*9));\
    __m512i t0v0;\
    __m512i t0v1;\
    __m512i t0v2;\
    __m512i t0v3;\
    __m512i t0v4;\
    __m512i t0v5;\
    __m512i t0v6;\
    __m512i t0v7;\
    __m512i t0v8;\
    __m512i t0v9;\
    __m512i t0v10;\
    __m512i t0v11;\
    __m512i t0v12;\
    __m512i t0v13;\
    __m512i t0v14;\
    __m512i t0v15;\
    __m512i t0v16;\
    __m512i t0v17;\
    __m512i t0v18;\
    __m512i t0v19;\
    __m512i t0v20;\
    __m512i t0v21;\
    __m512i t0v22;\
    __m512i t0v23;\
    __m512i t0v24;\
    __m512i t0v25;\
    __m512i t0v26;\
    __m512i t0v27;\
    __m512i t0v28;\
    __m512i t0v29;\
    __m512i t0v30;\
    __m512i t0v31;\
    __m512i t0v32;\
    t0v0 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 256)));\
    t0v1 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 256)));\
    t0v2 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 272)));\
    t0v3 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 272)));\
    t0v4 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 288)));\
    t0v5 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 288)));\
    t0v6 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 304)));\
    t0v7 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 304)));\
    t0v8 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 320)));\
    t0v9 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 320)));\
    t0v10 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 336)));\
    t0v11 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 336)));\
    t0v12 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 352)));\
    t0v13 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 352)));\
    t0v14 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 368)));\
    t0v15 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 368)));\
    t0v16 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 384)));\
    t0v17 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 384)));\
    t0v18 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 400)));\
    t0v19 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 400)));\
    t0v20 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 416)));\
    t0v21 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 416)));\
    t0v22 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 432)));\
    t0v23 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 432)));\
    t0v24 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 448)));\
    t0v25 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 448)));\
    t0v26 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 464)));\
    t0v27 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 464)));\
    t0v28 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 480)));\
    t0v29 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 480)));\
    t0v30 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 496)));\
    t0v31 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 496)));\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v16);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v22);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v23);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v24);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v14 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v9);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v10);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v27);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v28);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v31 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v23 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v32 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v12);\
    t0v8 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v12);\
    t0v12 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v20);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v20);\
    t0v20 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v28);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v28);\
    t0v28 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 0)), t0v5);\
    t0v1 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 16)), t0v5);\
    t0v5 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v17 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v25 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 0)), t0v6);\
    t0v2 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 16)), t0v6);\
    t0v6 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v22);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v22);\
    t0v22 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v30);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v30);\
    t0v30 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v3 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c0), _mm512_slli_epi32(t0v5, 16));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(t0v15, 16), _mm512_and_epi64(t0v5, c1));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c0), _mm512_slli_epi32(t0v9, 16));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(t0v11, 16), _mm512_and_epi64(t0v9, c1));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c0), _mm512_slli_epi32(t0v13, 16));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(t0v31, 16), _mm512_and_epi64(t0v13, c1));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v19, c0), _mm512_slli_epi32(t0v17, 16));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(t0v19, 16), _mm512_and_epi64(t0v17, c1));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v23, c0), _mm512_slli_epi32(t0v21, 16));\
    t0v21 = _mm512_or_epi64(_mm512_srli_epi32(t0v23, 16), _mm512_and_epi64(t0v21, c1));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v27, c0), _mm512_slli_epi32(t0v25, 16));\
    t0v25 = _mm512_or_epi64(_mm512_srli_epi32(t0v27, 16), _mm512_and_epi64(t0v25, c1));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c0), _mm512_slli_epi32(t0v29, 16));\
    t0v29 = _mm512_or_epi64(_mm512_srli_epi32(t0v32, 16), _mm512_and_epi64(t0v29, c1));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c0), _mm512_slli_epi32(t0v2, 16));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(t0v0, 16), _mm512_and_epi64(t0v2, c1));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c0), _mm512_slli_epi32(t0v6, 16));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(t0v4, 16), _mm512_and_epi64(t0v6, c1));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c0), _mm512_slli_epi32(t0v10, 16));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(t0v8, 16), _mm512_and_epi64(t0v10, c1));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c0), _mm512_slli_epi32(t0v14, 16));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(t0v12, 16), _mm512_and_epi64(t0v14, c1));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c0), _mm512_slli_epi32(t0v18, 16));\
    t0v16 = _mm512_or_epi64(_mm512_srli_epi32(t0v16, 16), _mm512_and_epi64(t0v18, c1));\
    t0v18 = _mm512_or_epi64(_mm512_and_epi64(t0v20, c0), _mm512_slli_epi32(t0v22, 16));\
    t0v20 = _mm512_or_epi64(_mm512_srli_epi32(t0v20, 16), _mm512_and_epi64(t0v22, c1));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64(t0v24, c0), _mm512_slli_epi32(t0v26, 16));\
    t0v24 = _mm512_or_epi64(_mm512_srli_epi32(t0v24, 16), _mm512_and_epi64(t0v26, c1));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v28, c0), _mm512_slli_epi32(t0v30, 16));\
    t0v28 = _mm512_or_epi64(_mm512_srli_epi32(t0v28, 16), _mm512_and_epi64(t0v30, c1));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c0), _mm512_slli_epi32(t0v3, 16));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(t0v1, 16), _mm512_and_epi64(t0v3, c1));\
    t0v3 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c2), 8));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c3), 8), _mm512_and_epi64(t0v2, c3));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c2), 8));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c3), 8), _mm512_and_epi64(t0v6, c3));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c2), 8));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v11, c3), 8), _mm512_and_epi64(t0v10, c3));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c2), 8));\
    t0v14 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v31, c3), 8), _mm512_and_epi64(t0v14, c3));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v19, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v18, c2), 8));\
    t0v18 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v19, c3), 8), _mm512_and_epi64(t0v18, c3));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v23, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c2), 8));\
    t0v22 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v23, c3), 8), _mm512_and_epi64(t0v22, c3));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v27, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c2), 8));\
    t0v26 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v27, c3), 8), _mm512_and_epi64(t0v26, c3));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c2), 8));\
    t0v30 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c3), 8), _mm512_and_epi64(t0v30, c3));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c2), 8));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c3), 8), _mm512_and_epi64(t0v4, c3));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c2), 8));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c3), 8), _mm512_and_epi64(t0v8, c3));\
    t0v9 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c2), 8));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c3), 8), _mm512_and_epi64(t0v12, c3));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v16, c2), 8));\
    t0v16 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c3), 8), _mm512_and_epi64(t0v16, c3));\
    t0v17 = _mm512_or_epi64(_mm512_and_epi64(t0v21, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c2), 8));\
    t0v20 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v21, c3), 8), _mm512_and_epi64(t0v20, c3));\
    t0v21 = _mm512_or_epi64(_mm512_and_epi64(t0v25, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c2), 8));\
    t0v24 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v25, c3), 8), _mm512_and_epi64(t0v24, c3));\
    t0v25 = _mm512_or_epi64(_mm512_and_epi64(t0v29, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c2), 8));\
    t0v28 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v29, c3), 8), _mm512_and_epi64(t0v28, c3));\
    t0v29 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v1, c2), 8));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c3), 8), _mm512_and_epi64(t0v1, c3));\
    t0v1 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v31, c4), 4));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c5), 4), _mm512_and_epi64(t0v31, c5));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c4), 4));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c5), 4), _mm512_and_epi64(t0v19, c5));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c4), 4));\
    t0v15 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c5), 4), _mm512_and_epi64(t0v23, c5));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c4), 4));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v11, c5), 4), _mm512_and_epi64(t0v27, c5));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v18, c4), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c5), 4), _mm512_and_epi64(t0v18, c5));\
    t0v18 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c4), 4));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c5), 4), _mm512_and_epi64(t0v22, c5));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64(t0v10, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c4), 4));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v10, c5), 4), _mm512_and_epi64(t0v26, c5));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v14, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c4), 4));\
    t0v14 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v14, c5), 4), _mm512_and_epi64(t0v30, c5));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v17, c4), 4));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c5), 4), _mm512_and_epi64(t0v17, c5));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c4), 4));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c5), 4), _mm512_and_epi64(t0v21, c5));\
    t0v21 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c4), 4));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c5), 4), _mm512_and_epi64(t0v25, c5));\
    t0v25 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v29, c4), 4));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c5), 4), _mm512_and_epi64(t0v29, c5));\
    t0v29 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c4), 4));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c5), 4), _mm512_and_epi64(t0v20, c5));\
    t0v20 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c4), 4));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c5), 4), _mm512_and_epi64(t0v24, c5));\
    t0v24 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c4), 4));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v12, c5), 4), _mm512_and_epi64(t0v28, c5));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c4), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c5), 4), _mm512_and_epi64(t0v0, c5));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c6), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c7), 2), _mm512_and_epi64(t0v19, c7));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c6), 2));\
    t0v23 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v31, c7), 2), _mm512_and_epi64(t0v23, c7));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c6), 2));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c7), 2), _mm512_and_epi64(t0v15, c7));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c6), 2));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c7), 2), _mm512_and_epi64(t0v11, c7));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v27, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c6), 2));\
    t0v22 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v27, c7), 2), _mm512_and_epi64(t0v22, c7));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v18, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c6), 2));\
    t0v18 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v18, c7), 2), _mm512_and_epi64(t0v26, c7));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c6), 2));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c7), 2), _mm512_and_epi64(t0v10, c7));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c6), 2));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c7), 2), _mm512_and_epi64(t0v14, c7));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v30, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c6), 2));\
    t0v21 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v30, c7), 2), _mm512_and_epi64(t0v21, c7));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c6), 2));\
    t0v25 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c7), 2), _mm512_and_epi64(t0v25, c7));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c6), 2));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c7), 2), _mm512_and_epi64(t0v9, c7));\
    t0v17 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c6), 2));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c7), 2), _mm512_and_epi64(t0v13, c7));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v29, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c6), 2));\
    t0v24 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v29, c7), 2), _mm512_and_epi64(t0v24, c7));\
    t0v29 = _mm512_or_epi64(_mm512_and_epi64(t0v20, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c6), 2));\
    t0v20 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v20, c7), 2), _mm512_and_epi64(t0v28, c7));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c6), 2));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c7), 2), _mm512_and_epi64(t0v12, c7));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c6), 2));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c7), 2), _mm512_and_epi64(t0v0, c7));\
    (D0) = _mm512_or_epi64(_mm512_and_epi64(t0v16, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c8), 1));\
    (D1) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c9), 1), _mm512_and_epi64(t0v19, c9));\
    (D2) = _mm512_or_epi64(_mm512_and_epi64(t0v1, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c8), 1));\
    (D3) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c9), 1), _mm512_and_epi64(t0v23, c9));\
    (D4) = _mm512_or_epi64(_mm512_and_epi64(t0v31, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c8), 1));\
    (D5) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v31, c9), 1), _mm512_and_epi64(t0v15, c9));\
    (D6) = _mm512_or_epi64(_mm512_and_epi64(t0v3, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c8), 1));\
    (D7) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c9), 1), _mm512_and_epi64(t0v7, c9));\
    (D8) = _mm512_or_epi64(_mm512_and_epi64(t0v11, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c8), 1));\
    (D9) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v11, c9), 1), _mm512_and_epi64(t0v27, c9));\
    (D10) = _mm512_or_epi64(_mm512_and_epi64(t0v22, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v18, c8), 1));\
    (D11) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v22, c9), 1), _mm512_and_epi64(t0v18, c9));\
    (D12) = _mm512_or_epi64(_mm512_and_epi64(t0v26, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c8), 1));\
    (D13) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v26, c9), 1), _mm512_and_epi64(t0v10, c9));\
    (D14) = _mm512_or_epi64(_mm512_and_epi64(t0v2, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c8), 1));\
    (D15) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c9), 1), _mm512_and_epi64(t0v6, c9));\
    (D16) = _mm512_or_epi64(_mm512_and_epi64(t0v14, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c8), 1));\
    (D17) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v14, c9), 1), _mm512_and_epi64(t0v30, c9));\
    (D18) = _mm512_or_epi64(_mm512_and_epi64(t0v21, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c8), 1));\
    (D19) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v21, c9), 1), _mm512_and_epi64(t0v25, c9));\
    (D20) = _mm512_or_epi64(_mm512_and_epi64(t0v32, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v17, c8), 1));\
    (D21) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c9), 1), _mm512_and_epi64(t0v17, c9));\
    (D22) = _mm512_or_epi64(_mm512_and_epi64(t0v9, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c8), 1));\
    (D23) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c9), 1), _mm512_and_epi64(t0v5, c9));\
    (D24) = _mm512_or_epi64(_mm512_and_epi64(t0v13, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v29, c8), 1));\
    (D25) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c9), 1), _mm512_and_epi64(t0v29, c9));\
    (D26) = _mm512_or_epi64(_mm512_and_epi64(t0v24, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c8), 1));\
    (D27) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v24, c9), 1), _mm512_and_epi64(t0v20, c9));\
    (D28) = _mm512_or_epi64(_mm512_and_epi64(t0v28, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c8), 1));\
    (D29) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v28, c9), 1), _mm512_and_epi64(t0v12, c9));\
    (D30) = _mm512_or_epi64(_mm512_and_epi64(t0v4, c8), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c8), 1));\
    (D31) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c9), 1), _mm512_and_epi64(t0v0, c9));\
}
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    const __m512i c0 = (*(const __m512i*)(transform_const2_tbl + 16*4));\
    const __m512i c7 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c8 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c6 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*5));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*6));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*7));\
    __m512i t0v0;\
    __m512i t0v1;\
    __m512i t0v2;\
    __m512i t0v3;\
    __m512i t0v4;\
    __m512i t0v5;\
    __m512i t0v6;\
    __m512i t0v7;\
    __m512i t0v8;\
    __m512i t0v9;\
    __m512i t0v10;\
    __m512i t0v11;\
    __m512i t0v12;\
    __m512i t0v13;\
    __m512i t0v14;\
    __m512i t0v15;\
    __m512i t0v16;\
    __m512i t0v17;\
    __m512i t0v18;\
    __m512i t0v19;\
    __m512i t0v20;\
    __m512i t0v21;\
    __m512i t0v22;\
    __m512i t0v23;\
    __m512i t0v24;\
    __m512i t0v25;\
    __m512i t0v26;\
    __m512i t0v27;\
    __m512i t0v28;\
    __m512i t0v29;\
    __m512i t0v30;\
    __m512i t0v31;\
    __m512i t0v32;\
    t0v0 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 256)));\
    t0v1 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 256)));\
    t0v2 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 272)));\
    t0v3 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 272)));\
    t0v4 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 288)));\
    t0v5 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 288)));\
    t0v6 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 304)));\
    t0v7 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 304)));\
    t0v8 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 320)));\
    t0v9 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 320)));\
    t0v10 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 336)));\
    t0v11 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 336)));\
    t0v12 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 352)));\
    t0v13 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 352)));\
    t0v14 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 368)));\
    t0v15 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 368)));\
    t0v16 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 384)));\
    t0v17 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 384)));\
    t0v18 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 400)));\
    t0v19 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 400)));\
    t0v20 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 416)));\
    t0v21 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 416)));\
    t0v22 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 432)));\
    t0v23 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 432)));\
    t0v24 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 448)));\
    t0v25 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 448)));\
    t0v26 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 464)));\
    t0v27 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 464)));\
    t0v28 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 480)));\
    t0v29 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 480)));\
    t0v30 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 496)));\
    t0v31 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 496)));\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v16);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v22);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v23);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v24);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v14 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v9);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v10);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v27);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v28);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v31 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v23 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v32 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v12);\
    t0v8 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v12);\
    t0v12 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v20);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v20);\
    t0v20 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v28);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v28);\
    t0v28 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 0)), t0v5);\
    t0v1 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 16)), t0v5);\
    t0v5 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v17 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v25 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 0)), t0v6);\
    t0v2 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 16)), t0v6);\
    t0v6 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v22);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v22);\
    t0v22 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v30);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v30);\
    t0v30 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v3 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c0), _mm512_slli_epi32(t0v5, 16));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c0), _mm512_slli_epi32(t0v9, 16));\
    t0v9 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c0), _mm512_slli_epi32(t0v13, 16));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v19, c0), _mm512_slli_epi32(t0v17, 16));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v23, c0), _mm512_slli_epi32(t0v21, 16));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v27, c0), _mm512_slli_epi32(t0v25, 16));\
    t0v17 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c0), _mm512_slli_epi32(t0v29, 16));\
    t0v0 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c0), _mm512_slli_epi32(t0v2, 16));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c0), _mm512_slli_epi32(t0v6, 16));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c0), _mm512_slli_epi32(t0v10, 16));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c0), _mm512_slli_epi32(t0v14, 16));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c0), _mm512_slli_epi32(t0v18, 16));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v20, c0), _mm512_slli_epi32(t0v22, 16));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v24, c0), _mm512_slli_epi32(t0v26, 16));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v28, c0), _mm512_slli_epi32(t0v30, 16));\
    t0v1 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c0), _mm512_slli_epi32(t0v3, 16));\
    t0v3 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c1), 8));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c2), 8), _mm512_and_epi64(t0v2, c2));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c1), 8));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c2), 8), _mm512_and_epi64(t0v4, c2));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c1), 8));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c2), 8), _mm512_and_epi64(t0v6, c2));\
    t0v9 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c1), 8));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v11, c2), 8), _mm512_and_epi64(t0v8, c2));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c1), 8));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c2), 8), _mm512_and_epi64(t0v10, c2));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c1), 8));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c2), 8), _mm512_and_epi64(t0v12, c2));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c1), 8));\
    t0v14 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c2), 8), _mm512_and_epi64(t0v14, c2));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v1, c1), 8));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c2), 8), _mm512_and_epi64(t0v1, c2));\
    t0v1 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c3), 4));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c4), 4), _mm512_and_epi64(t0v11, c4));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c3), 4));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c4), 4), _mm512_and_epi64(t0v13, c4));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c3), 4));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c4), 4), _mm512_and_epi64(t0v15, c4));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v16, c3), 4));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c4), 4), _mm512_and_epi64(t0v16, c4));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c3), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c4), 4), _mm512_and_epi64(t0v10, c4));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c3), 4));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c4), 4), _mm512_and_epi64(t0v12, c4));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c3), 4));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c4), 4), _mm512_and_epi64(t0v14, c4));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c3), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c4), 4), _mm512_and_epi64(t0v0, c4));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c5), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c6), 2), _mm512_and_epi64(t0v13, c6));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c5), 2));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v11, c6), 2), _mm512_and_epi64(t0v15, c6));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c5), 2));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c6), 2), _mm512_and_epi64(t0v7, c6));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c5), 2));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c6), 2), _mm512_and_epi64(t0v9, c6));\
    t0v9 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c5), 2));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c6), 2), _mm512_and_epi64(t0v12, c6));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v10, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c5), 2));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v10, c6), 2), _mm512_and_epi64(t0v14, c6));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c5), 2));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c6), 2), _mm512_and_epi64(t0v6, c6));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c5), 2));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c6), 2), _mm512_and_epi64(t0v0, c6));\
    (D0) = _mm512_or_epi64(_mm512_and_epi64(t0v8, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c7), 1));\
    (D1) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c8), 1), _mm512_and_epi64(t0v13, c8));\
    (D2) = _mm512_or_epi64(_mm512_and_epi64(t0v1, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c7), 1));\
    (D3) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c8), 1), _mm512_and_epi64(t0v11, c8));\
    (D4) = _mm512_or_epi64(_mm512_and_epi64(t0v15, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c7), 1));\
    (D5) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c8), 1), _mm512_and_epi64(t0v7, c8));\
    (D6) = _mm512_or_epi64(_mm512_and_epi64(t0v3, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c7), 1));\
    (D7) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c8), 1), _mm512_and_epi64(t0v5, c8));\
    (D8) = _mm512_or_epi64(_mm512_and_epi64(t0v9, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v16, c7), 1));\
    (D9) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c8), 1), _mm512_and_epi64(t0v16, c8));\
    (D10) = _mm512_or_epi64(_mm512_and_epi64(t0v12, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c7), 1));\
    (D11) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v12, c8), 1), _mm512_and_epi64(t0v10, c8));\
    (D12) = _mm512_or_epi64(_mm512_and_epi64(t0v14, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c7), 1));\
    (D13) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v14, c8), 1), _mm512_and_epi64(t0v6, c8));\
    (D14) = _mm512_or_epi64(_mm512_and_epi64(t0v2, c7), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c7), 1));\
    (D15) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c8), 1), _mm512_and_epi64(t0v0, c8));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
{\
    const __m512i c0 = (*(const __m512i*)(transform_const2_tbl + 16*3));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c6 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*5));\
    __m512i t0v0;\
    __m512i t0v1;\
    __m512i t0v2;\
    __m512i t0v3;\
    __m512i t0v4;\
    __m512i t0v5;\
    __m512i t0v6;\
    __m512i t0v7;\
    __m512i t0v8;\
    __m512i t0v9;\
    __m512i t0v10;\
    __m512i t0v11;\
    __m512i t0v12;\
    __m512i t0v13;\
    __m512i t0v14;\
    __m512i t0v15;\
    __m512i t0v16;\
    __m512i t0v17;\
    __m512i t0v18;\
    __m512i t0v19;\
    __m512i t0v20;\
    __m512i t0v21;\
    __m512i t0v22;\
    __m512i t0v23;\
    __m512i t0v24;\
    __m512i t0v25;\
    __m512i t0v26;\
    __m512i t0v27;\
    __m512i t0v28;\
    __m512i t0v29;\
    __m512i t0v30;\
    __m512i t0v31;\
    __m512i t0v32;\
    t0v0 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 256)));\
    t0v1 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 256)));\
    t0v2 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 272)));\
    t0v3 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 272)));\
    t0v4 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 288)));\
    t0v5 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 288)));\
    t0v6 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 304)));\
    t0v7 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 304)));\
    t0v8 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 320)));\
    t0v9 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 320)));\
    t0v10 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 336)));\
    t0v11 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 336)));\
    t0v12 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 352)));\
    t0v13 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 352)));\
    t0v14 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 368)));\
    t0v15 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 368)));\
    t0v16 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 384)));\
    t0v17 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 384)));\
    t0v18 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 400)));\
    t0v19 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 400)));\
    t0v20 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 416)));\
    t0v21 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 416)));\
    t0v22 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 432)));\
    t0v23 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 432)));\
    t0v24 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 448)));\
    t0v25 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 448)));\
    t0v26 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 464)));\
    t0v27 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 464)));\
    t0v28 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 480)));\
    t0v29 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 480)));\
    t0v30 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 496)));\
    t0v31 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 496)));\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v16);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v22);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v23);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v24);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v14 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v9);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v10);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v27);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v28);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v31 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v23 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v32 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v12);\
    t0v8 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v12);\
    t0v12 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v20);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v20);\
    t0v20 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v28);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v28);\
    t0v28 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 0)), t0v5);\
    t0v1 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 16)), t0v5);\
    t0v5 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v17 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v25 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 0)), t0v6);\
    t0v2 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 16)), t0v6);\
    t0v6 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v22);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v22);\
    t0v22 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v30);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v30);\
    t0v30 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v3 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v4 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v15, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c0), 16)), _mm512_slli_epi32(t0v6, 24));\
    t0v5 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v11, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c0), 16)), _mm512_slli_epi32(t0v10, 24));\
    t0v6 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v31, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c0), 16)), _mm512_slli_epi32(t0v14, 24));\
    t0v7 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v19, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v16, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v17, c0), 16)), _mm512_slli_epi32(t0v18, 24));\
    t0v8 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v23, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c0), 16)), _mm512_slli_epi32(t0v22, 24));\
    t0v9 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v27, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c0), 16)), _mm512_slli_epi32(t0v26, 24));\
    t0v10 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v32, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v29, c0), 16)), _mm512_slli_epi32(t0v30, 24));\
    t0v0 = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v0, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v1, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c0), 16)), _mm512_slli_epi32(t0v3, 24));\
    t0v1 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c1), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c2), 4), _mm512_and_epi64(t0v8, c2));\
    t0v3 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c1), 4));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c2), 4), _mm512_and_epi64(t0v9, c2));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c1), 4));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c2), 4), _mm512_and_epi64(t0v10, c2));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c1), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c1), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c2), 4), _mm512_and_epi64(t0v0, c2));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c3), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c4), 2), _mm512_and_epi64(t0v5, c4));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c3), 2));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c4), 2), _mm512_and_epi64(t0v8, c4));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c3), 2));\
    t0v0 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c3), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c3), 2));\
    (D0) = _mm512_or_epi64(_mm512_and_epi64(t0v7, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c5), 1));\
    (D1) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c6), 1), _mm512_and_epi64(t0v5, c6));\
    (D2) = _mm512_or_epi64(_mm512_and_epi64(t0v1, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c5), 1));\
    (D3) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c6), 1), _mm512_and_epi64(t0v3, c6));\
    (D4) = _mm512_or_epi64(_mm512_and_epi64(t0v2, c5), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c5), 1));\
    (D5) = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c6), 1), _mm512_and_epi64(t0v0, c6));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    const __m512i c0 = (*(const __m512i*)(transform_const2_tbl + 16*0));\
    __m512i t0v0;\
    __m512i t0v1;\
    __m512i t0v2;\
    __m512i t0v3;\
    __m512i t0v4;\
    __m512i t0v5;\
    __m512i t0v6;\
    __m512i t0v7;\
    __m512i t0v8;\
    __m512i t0v9;\
    __m512i t0v10;\
    __m512i t0v11;\
    __m512i t0v12;\
    __m512i t0v13;\
    __m512i t0v14;\
    __m512i t0v15;\
    __m512i t0v16;\
    __m512i t0v17;\
    __m512i t0v18;\
    __m512i t0v19;\
    __m512i t0v20;\
    __m512i t0v21;\
    __m512i t0v22;\
    __m512i t0v23;\
    __m512i t0v24;\
    __m512i t0v25;\
    __m512i t0v26;\
    __m512i t0v27;\
    __m512i t0v28;\
    __m512i t0v29;\
    __m512i t0v30;\
    __m512i t0v31;\
    __m512i t0v32;\
    t0v0 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 256)));\
    t0v1 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 0)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 256)));\
    t0v2 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 272)));\
    t0v3 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 16)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 272)));\
    t0v4 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 288)));\
    t0v5 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 32)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 288)));\
    t0v6 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 304)));\
    t0v7 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 48)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 304)));\
    t0v8 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 320)));\
    t0v9 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 64)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 320)));\
    t0v10 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 336)));\
    t0v11 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 80)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 336)));\
    t0v12 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 352)));\
    t0v13 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 96)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 352)));\
    t0v14 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 368)));\
    t0v15 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 112)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 368)));\
    t0v16 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 384)));\
    t0v17 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 128)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 384)));\
    t0v18 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 400)));\
    t0v19 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 144)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 400)));\
    t0v20 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 416)));\
    t0v21 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 160)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 416)));\
    t0v22 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 432)));\
    t0v23 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 176)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 432)));\
    t0v24 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 448)));\
    t0v25 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 192)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 448)));\
    t0v26 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 464)));\
    t0v27 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 208)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 464)));\
    t0v28 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 480)));\
    t0v29 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 224)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 480)));\
    t0v30 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*4)), _mm512_loadu_epi64(((S) + 496)));\
    t0v31 = _mm512_permutex2var_epi64(_mm512_loadu_epi64(((S) + 240)), (*(const __m512i*)(transform_const3_tbl + 8*5)), _mm512_loadu_epi64(((S) + 496)));\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v16);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v22);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v23);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v24);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v14 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v32 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v9);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v10);\
    t0v2 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v27);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v28);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v31 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v23 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v32, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v32 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v12);\
    t0v8 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v12);\
    t0v12 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v20);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v20);\
    t0v20 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v28);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v28);\
    t0v28 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 0)), t0v5);\
    t0v1 = _mm512_permutex2var_epi32(t0v1, (*(const __m512i*)(transform_const4_tbl + 16)), t0v5);\
    t0v5 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v17 = _mm512_permutex2var_epi32(t0v17, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v25 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 0)), t0v6);\
    t0v2 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 16)), t0v6);\
    t0v6 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v22);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v22);\
    t0v22 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v30);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v30);\
    t0v30 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v3 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    (D0) = _mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_or_epi64(_mm512_and_epi64(t0v15, c0), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c0), 1)), _mm512_slli_epi32(_mm512_and_epi64(t0v31, c0), 2)), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c0), 3)), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c0), 4)), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c0), 5)), _mm512_slli_epi32(_mm512_and_epi64(t0v32, c0), 6)), _mm512_slli_epi32(_mm512_and_epi64(t0v0, c0), 7)), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c0), 8)), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c0), 9)), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c0), 10)), _mm512_slli_epi32(_mm512_and_epi64(t0v16, c0), 11)), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c0), 12)), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c0), 13)), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c0), 14)), _mm512_slli_epi32(_mm512_and_epi64(t0v1, c0), 15)), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c0), 16)), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c0), 17)), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c0), 18)), _mm512_slli_epi32(_mm512_and_epi64(t0v17, c0), 19)), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c0), 20)), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c0), 21)), _mm512_slli_epi32(_mm512_and_epi64(t0v29, c0), 22)), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c0), 23)), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c0), 24)), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c0), 25)), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c0), 26)), _mm512_slli_epi32(_mm512_and_epi64(t0v18, c0), 27)), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c0), 28)), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c0), 29)), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c0), 30)), _mm512_slli_epi32(t0v3, 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_ARM_NEON.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    uint32x4_t t0v0;\
    uint32x4_t t0v1;\
    uint32x4_t t0v2;\
    uint32x4_t t0v3;\
    uint32x4_t t0v4;\
    uint32x4_t t0v5;\
    uint32x4_t t0v6;\
    uint32x4_t t0v7;\
    uint32x4_t t0v8;\
    uint32x4_t t0v9;\
    uint32x4_t t0v10;\
    uint32x4_t t0v11;\
    uint32x4_t t0v12;\
    uint32x4_t t0v13;\
    uint32x4_t t0v14;\
    uint32x4_t t0v15;\
    uint32x4_t t0v16;\
    uint32x4_t t0v17;\
    uint32x4_t t0v18;\
    uint32x4_t t0v19;\
    uint32x4_t t0v20;\
    uint32x4_t t0v21;\
    uint32x4_t t0v22;\
    uint32x4_t t0v23;\
    uint32x4_t t0v24;\
    uint32x4_t t0v25;\
    uint32x4_t t0v26;\
    uint32x4_t t0v27;\
    uint32x4_t t0v28;\
    uint32x4_t t0v29;\
    uint32x4_t t0v30;\
    uint32x4_t t0v31;\
    uint32x4_t t0v32;\
    t0v0 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v1 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v2 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v3 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v4 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v5 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v6 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v7 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v8 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v9 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v10 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v11 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v12 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v13 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v14 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v15 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v16 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v17 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v18 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v19 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v20 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v21 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v22 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v23 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v24 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v25 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v26 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v27 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v28 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v29 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v30 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v31 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v16), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v16, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v1, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v1), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v2, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v18), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v2), 32)), vandq_u32(t0v18, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v4, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v20), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v4), 32)), vandq_u32(t0v20, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v20 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v22), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v22, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v7, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v7), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v24), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v24, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v26), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v26, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v11, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v11), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v28), 32)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v28, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v28 = vorrq_u32(vandq_u32(t0v13, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v13), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v30), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v30, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v15, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v15), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v23, 16));\
    t0v23 = vorrq_u32(vshrq_n_u32(t0v32, 16), vandq_u32(t0v23, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v8, 16));\
    t0v0 = vorrq_u32(vshrq_n_u32(t0v0, 16), vandq_u32(t0v8, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v8 = vorrq_u32(vandq_u32(t0v16, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v24, 16));\
    t0v16 = vorrq_u32(vshrq_n_u32(t0v16, 16), vandq_u32(t0v24, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v24 = vorrq_u32(vandq_u32(t0v1, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v9, 16));\
    t0v1 = vorrq_u32(vshrq_n_u32(t0v1, 16), vandq_u32(t0v9, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v9 = vorrq_u32(vandq_u32(t0v17, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v25, 16));\
    t0v17 = vorrq_u32(vshrq_n_u32(t0v17, 16), vandq_u32(t0v25, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v25 = vorrq_u32(vandq_u32(t0v2, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v10, 16));\
    t0v2 = vorrq_u32(vshrq_n_u32(t0v2, 16), vandq_u32(t0v10, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v10 = vorrq_u32(vandq_u32(t0v18, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v26, 16));\
    t0v18 = vorrq_u32(vshrq_n_u32(t0v18, 16), vandq_u32(t0v26, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v26 = vorrq_u32(vandq_u32(t0v3, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v11, 16));\
    t0v3 = vorrq_u32(vshrq_n_u32(t0v3, 16), vandq_u32(t0v11, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v11 = vorrq_u32(vandq_u32(t0v19, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v27, 16));\
    t0v19 = vorrq_u32(vshrq_n_u32(t0v19, 16), vandq_u32(t0v27, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v27 = vorrq_u32(vandq_u32(t0v4, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v12, 16));\
    t0v4 = vorrq_u32(vshrq_n_u32(t0v4, 16), vandq_u32(t0v12, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v20, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v28, 16));\
    t0v20 = vorrq_u32(vshrq_n_u32(t0v20, 16), vandq_u32(t0v28, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v5, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v13, 16));\
    t0v5 = vorrq_u32(vshrq_n_u32(t0v5, 16), vandq_u32(t0v13, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v21, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v29, 16));\
    t0v21 = vorrq_u32(vshrq_n_u32(t0v21, 16), vandq_u32(t0v29, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v29 = vorrq_u32(vandq_u32(t0v6, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v14, 16));\
    t0v6 = vorrq_u32(vshrq_n_u32(t0v6, 16), vandq_u32(t0v14, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v22, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v30, 16));\
    t0v22 = vorrq_u32(vshrq_n_u32(t0v22, 16), vandq_u32(t0v30, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v7, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v15, 16));\
    t0v7 = vorrq_u32(vshrq_n_u32(t0v7, 16), vandq_u32(t0v15, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v15 = vorrq_u32(vandq_u32(t0v31, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v11), 8)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v31), 8)), vandq_u32(t0v11, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v27), 8)));\
    t0v27 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v32), 8)), vandq_u32(t0v27, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v8, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v12), 8)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v8), 8)), vandq_u32(t0v12, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v24, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v28), 8)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v24), 8)), vandq_u32(t0v28, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v9, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v13), 8)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v9), 8)), vandq_u32(t0v13, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v25, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v29), 8)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v25), 8)), vandq_u32(t0v29, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v29 = vorrq_u32(vandq_u32(t0v10, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v14), 8)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v10), 8)), vandq_u32(t0v14, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v26, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v30), 8)));\
    t0v26 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v26), 8)), vandq_u32(t0v30, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v23, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v19), 8)));\
    t0v19 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v23), 8)), vandq_u32(t0v19, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v23 = vorrq_u32(vandq_u32(t0v0, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v4), 8)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v0), 8)), vandq_u32(t0v4, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v4 = vorrq_u32(vandq_u32(t0v16, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v20), 8)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v16), 8)), vandq_u32(t0v20, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v20 = vorrq_u32(vandq_u32(t0v1, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v5), 8)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v1), 8)), vandq_u32(t0v5, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v5 = vorrq_u32(vandq_u32(t0v17, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v21), 8)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v17), 8)), vandq_u32(t0v21, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v21 = vorrq_u32(vandq_u32(t0v2, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v6), 8)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v2), 8)), vandq_u32(t0v6, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v18, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v22), 8)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v18), 8)), vandq_u32(t0v22, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v22 = vorrq_u32(vandq_u32(t0v3, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v7), 8)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v3), 8)), vandq_u32(t0v7, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v7 = vorrq_u32(vandq_u32(t0v15, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v28), 4)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v15), 4)), vandq_u32(t0v28, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v31, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v13), 4)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v31), 4)), vandq_u32(t0v13, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v29), 4)));\
    t0v29 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v32), 4)), vandq_u32(t0v29, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v12, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v14), 4)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v12), 4)), vandq_u32(t0v14, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v11, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v9), 4)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v11), 4)), vandq_u32(t0v9, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v11 = vorrq_u32(vandq_u32(t0v27, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v25), 4)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v27), 4)), vandq_u32(t0v25, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v27 = vorrq_u32(vandq_u32(t0v8, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v10), 4)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v8), 4)), vandq_u32(t0v10, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v10 = vorrq_u32(vandq_u32(t0v24, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v26), 4)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v24), 4)), vandq_u32(t0v26, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v26 = vorrq_u32(vandq_u32(t0v30, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v30), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v23, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v21), 4)));\
    t0v21 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v23), 4)), vandq_u32(t0v21, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v23 = vorrq_u32(vandq_u32(t0v4, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v6), 4)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v4), 4)), vandq_u32(t0v6, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v20, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v22), 4)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v20), 4)), vandq_u32(t0v22, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v22 = vorrq_u32(vandq_u32(t0v19, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v17), 4)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v19), 4)), vandq_u32(t0v17, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v19 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v2), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v2, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v2 = vorrq_u32(vandq_u32(t0v16, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v18), 4)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v16), 4)), vandq_u32(t0v18, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v18 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v3 = vorrq_u32(vandq_u32(t0v7, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v31, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v31, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v31 = vorrq_u32(vandq_u32(t0v28, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v32, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v28, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v32, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v32 = vorrq_u32(vandq_u32(t0v15, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v29, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v29, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v13, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v12, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v13 = vorrq_u32(vandq_u32(t0v14, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v27, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v27, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v11, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v10, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v9, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v8, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v9 = vorrq_u32(vandq_u32(t0v25, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v24, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v25, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v24, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v26, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v23, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v23 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v26, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v23, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v30, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v30, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v6, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v4, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v5 = vorrq_u32(vandq_u32(t0v21, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v20, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v21, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v20, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v22, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v22, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v19, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v18, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v19, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v18, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v17, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v16, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v17, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v16, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v1, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    (D0) = vorrq_u32(vandq_u32(t0v3, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v31, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D1) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v31, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D2) = vorrq_u32(vandq_u32(t0v7, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v28, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D3) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v28, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D4) = vorrq_u32(vandq_u32(t0v32, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v29, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D5) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v32, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v29, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D6) = vorrq_u32(vandq_u32(t0v15, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D7) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v12, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D8) = vorrq_u32(vandq_u32(t0v13, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v27, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D9) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v27, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D10) = vorrq_u32(vandq_u32(t0v14, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D11) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v10, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D12) = vorrq_u32(vandq_u32(t0v11, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D13) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v9, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D14) = vorrq_u32(vandq_u32(t0v8, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v24, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D15) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v24, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D16) = vorrq_u32(vandq_u32(t0v25, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v26, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D17) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v25, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v26, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D18) = vorrq_u32(vandq_u32(t0v23, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D19) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v23, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v6, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D20) = vorrq_u32(vandq_u32(t0v30, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D21) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v30, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v5, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D22) = vorrq_u32(vandq_u32(t0v4, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v20, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D23) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v20, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D24) = vorrq_u32(vandq_u32(t0v21, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v22, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D25) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v21, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v22, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D26) = vorrq_u32(vandq_u32(t0v2, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v18, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D27) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v18, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D28) = vorrq_u32(vandq_u32(t0v19, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v17, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D29) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v19, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v17, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D30) = vorrq_u32(vandq_u32(t0v16, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D31) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v16, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v0, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
}
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    uint32x4_t t0v0;\
    uint32x4_t t0v1;\
    uint32x4_t t0v2;\
    uint32x4_t t0v3;\
    uint32x4_t t0v4;\
    uint32x4_t t0v5;\
    uint32x4_t t0v6;\
    uint32x4_t t0v7;\
    uint32x4_t t0v8;\
    uint32x4_t t0v9;\
    uint32x4_t t0v10;\
    uint32x4_t t0v11;\
    uint32x4_t t0v12;\
    uint32x4_t t0v13;\
    uint32x4_t t0v14;\
    uint32x4_t t0v15;\
    uint32x4_t t0v16;\
    uint32x4_t t0v17;\
    uint32x4_t t0v18;\
    uint32x4_t t0v19;\
    uint32x4_t t0v20;\
    uint32x4_t t0v21;\
    uint32x4_t t0v22;\
    uint32x4_t t0v23;\
    uint32x4_t t0v24;\
    uint32x4_t t0v25;\
    uint32x4_t t0v26;\
    uint32x4_t t0v27;\
    uint32x4_t t0v28;\
    uint32x4_t t0v29;\
    uint32x4_t t0v30;\
    uint32x4_t t0v31;\
    uint32x4_t t0v32;\
    t0v0 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v1 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v2 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v3 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v4 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v5 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v6 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v7 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v8 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v9 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v10 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v11 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v12 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v13 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v14 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v15 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v16 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v17 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v18 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v19 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v20 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v21 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v22 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v23 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v24 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v25 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v26 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v27 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v28 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v29 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v30 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v31 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v16), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v16, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v1, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v1), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v2, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v18), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v2), 32)), vandq_u32(t0v18, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v4, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v20), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v4), 32)), vandq_u32(t0v20, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v20 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v22), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v22, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v7, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v7), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v24), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v24, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v26), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v26, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v11, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v11), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v28), 32)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v28, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v28 = vorrq_u32(vandq_u32(t0v13, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v13), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v30), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v30, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v15, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v15), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v32, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v23, 16));\
    t0v0 = vorrq_u32(vandq_u32(t0v0, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v8, 16));\
    t0v8 = vorrq_u32(vandq_u32(t0v16, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v24, 16));\
    t0v1 = vorrq_u32(vandq_u32(t0v1, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v9, 16));\
    t0v9 = vorrq_u32(vandq_u32(t0v17, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v25, 16));\
    t0v2 = vorrq_u32(vandq_u32(t0v2, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v10, 16));\
    t0v10 = vorrq_u32(vandq_u32(t0v18, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v26, 16));\
    t0v3 = vorrq_u32(vandq_u32(t0v3, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v11, 16));\
    t0v11 = vorrq_u32(vandq_u32(t0v19, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v27, 16));\
    t0v4 = vorrq_u32(vandq_u32(t0v4, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v12, 16));\
    t0v12 = vorrq_u32(vandq_u32(t0v20, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v28, 16));\
    t0v5 = vorrq_u32(vandq_u32(t0v5, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v13, 16));\
    t0v13 = vorrq_u32(vandq_u32(t0v21, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v29, 16));\
    t0v6 = vorrq_u32(vandq_u32(t0v6, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v14, 16));\
    t0v14 = vorrq_u32(vandq_u32(t0v22, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v30, 16));\
    t0v7 = vorrq_u32(vandq_u32(t0v7, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v15, 16));\
    t0v15 = vorrq_u32(vandq_u32(t0v23, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v11), 8)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v23), 8)), vandq_u32(t0v11, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v16 = vorrq_u32(vandq_u32(t0v0, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v4), 8)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v0), 8)), vandq_u32(t0v4, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v4 = vorrq_u32(vandq_u32(t0v8, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v12), 8)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v8), 8)), vandq_u32(t0v12, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v1, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v5), 8)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v1), 8)), vandq_u32(t0v5, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v5 = vorrq_u32(vandq_u32(t0v9, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v13), 8)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v9), 8)), vandq_u32(t0v13, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v2, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v6), 8)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v2), 8)), vandq_u32(t0v6, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v10, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v14), 8)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v10), 8)), vandq_u32(t0v14, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v3, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v7), 8)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v3), 8)), vandq_u32(t0v7, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v7 = vorrq_u32(vandq_u32(t0v15, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v15), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v13), 4)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v16), 4)), vandq_u32(t0v13, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v16 = vorrq_u32(vandq_u32(t0v4, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v6), 4)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v4), 4)), vandq_u32(t0v6, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v12, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v14), 4)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v12), 4)), vandq_u32(t0v14, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v11, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v9), 4)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v11), 4)), vandq_u32(t0v9, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v11 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v2), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v2, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v2 = vorrq_u32(vandq_u32(t0v8, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v10), 4)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v8), 4)), vandq_u32(t0v10, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v10 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v3 = vorrq_u32(vandq_u32(t0v7, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v16, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v16, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v15, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v6, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v15 = vorrq_u32(vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v4, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v5 = vorrq_u32(vandq_u32(t0v13, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v12, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v13 = vorrq_u32(vandq_u32(t0v14, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v14 = vorrq_u32(vandq_u32(t0v11, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v10, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v9, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v8, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v9 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v1, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    (D0) = vorrq_u32(vandq_u32(t0v3, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v16, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D1) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v16, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D2) = vorrq_u32(vandq_u32(t0v7, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D3) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v6, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D4) = vorrq_u32(vandq_u32(t0v15, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D5) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v5, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D6) = vorrq_u32(vandq_u32(t0v4, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D7) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v12, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D8) = vorrq_u32(vandq_u32(t0v13, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D9) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v14, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D10) = vorrq_u32(vandq_u32(t0v2, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D11) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v10, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D12) = vorrq_u32(vandq_u32(t0v11, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D13) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v9, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D14) = vorrq_u32(vandq_u32(t0v8, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D15) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v0, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
}
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
{\
    uint32x4_t t0v0;\
    uint32x4_t t0v1;\
    uint32x4_t t0v2;\
    uint32x4_t t0v3;\
    uint32x4_t t0v4;\
    uint32x4_t t0v5;\
    uint32x4_t t0v6;\
    uint32x4_t t0v7;\
    uint32x4_t t0v8;\
    uint32x4_t t0v9;\
    uint32x4_t t0v10;\
    uint32x4_t t0v11;\
    uint32x4_t t0v12;\
    uint32x4_t t0v13;\
    uint32x4_t t0v14;\
    uint32x4_t t0v15;\
    uint32x4_t t0v16;\
    uint32x4_t t0v17;\
    uint32x4_t t0v18;\
    uint32x4_t t0v19;\
    uint32x4_t t0v20;\
    uint32x4_t t0v21;\
    uint32x4_t t0v22;\
    uint32x4_t t0v23;\
    uint32x4_t t0v24;\
    uint32x4_t t0v25;\
    uint32x4_t t0v26;\
    uint32x4_t t0v27;\
    uint32x4_t t0v28;\
    uint32x4_t t0v29;\
    uint32x4_t t0v30;\
    uint32x4_t t0v31;\
    uint32x4_t t0v32;\
    t0v0 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v1 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v2 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v3 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v4 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v5 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v6 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v7 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v8 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v9 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v10 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v11 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v12 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v13 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v14 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v15 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v16 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v17 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v18 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v19 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v20 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v21 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v22 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v23 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v24 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v25 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v26 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v27 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v28 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v29 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v30 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v31 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v16), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v16, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v1, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v1), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v2, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v18), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v2), 32)), vandq_u32(t0v18, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v4, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v20), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v4), 32)), vandq_u32(t0v20, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v20 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v22), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v22, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v7, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v7), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v24), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v24, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v26), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v26, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v11, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v11), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v28), 32)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v28, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v28 = vorrq_u32(vandq_u32(t0v13, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v13), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v30), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v30, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v15, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v15), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v32, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v19, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v23, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v27, 24));\
    t0v0 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v0, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v4, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v8, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v12, 24));\
    t0v4 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v16, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v20, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v24, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v28, 24));\
    t0v1 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v1, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v5, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v9, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v13, 24));\
    t0v5 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v17, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v21, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v25, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v29, 24));\
    t0v2 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v2, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v6, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v10, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v14, 24));\
    t0v6 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v18, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v22, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v26, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v30, 24));\
    t0v3 = vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v3, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), vshlq_n_u32(vandq_u32(t0v7, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 8)), vshlq_n_u32(vandq_u32(t0v11, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU }), 16)), vshlq_n_u32(t0v15, 24));\
    t0v7 = vorrq_u32(vandq_u32(t0v19, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v19), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v8 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v2), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v2, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v2 = vorrq_u32(vandq_u32(t0v4, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v6), 4)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v4), 4)), vandq_u32(t0v6, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v3 = vorrq_u32(vandq_u32(t0v7, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v7 = vorrq_u32(vandq_u32(t0v8, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v6, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v4 = vorrq_u32(vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    (D0) = vorrq_u32(vandq_u32(t0v3, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D1) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v7, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D2) = vorrq_u32(vandq_u32(t0v2, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D3) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v6, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    (D4) = vorrq_u32(vandq_u32(t0v4, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    (D5) = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32(t0v0, { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    uint32x4_t t0v0;\
    uint32x4_t t0v1;\
    uint32x4_t t0v2;\
    uint32x4_t t0v3;\
    uint32x4_t t0v4;\
    uint32x4_t t0v5;\
    uint32x4_t t0v6;\
    uint32x4_t t0v7;\
    uint32x4_t t0v8;\
    uint32x4_t t0v9;\
    uint32x4_t t0v10;\
    uint32x4_t t0v11;\
    uint32x4_t t0v12;\
    uint32x4_t t0v13;\
    uint32x4_t t0v14;\
    uint32x4_t t0v15;\
    uint32x4_t t0v16;\
    uint32x4_t t0v17;\
    uint32x4_t t0v18;\
    uint32x4_t t0v19;\
    uint32x4_t t0v20;\
    uint32x4_t t0v21;\
    uint32x4_t t0v22;\
    uint32x4_t t0v23;\
    uint32x4_t t0v24;\
    uint32x4_t t0v25;\
    uint32x4_t t0v26;\
    uint32x4_t t0v27;\
    uint32x4_t t0v28;\
    uint32x4_t t0v29;\
    uint32x4_t t0v30;\
    uint32x4_t t0v31;\
    uint32x4_t t0v32;\
    t0v0 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v1 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[0])), vreinterpretq_u64_u32(((S)[64]))));\
    t0v2 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v3 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[4])), vreinterpretq_u64_u32(((S)[68]))));\
    t0v4 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v5 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[8])), vreinterpretq_u64_u32(((S)[72]))));\
    t0v6 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v7 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[12])), vreinterpretq_u64_u32(((S)[76]))));\
    t0v8 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v9 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[16])), vreinterpretq_u64_u32(((S)[80]))));\
    t0v10 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v11 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[20])), vreinterpretq_u64_u32(((S)[84]))));\
    t0v12 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v13 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[24])), vreinterpretq_u64_u32(((S)[88]))));\
    t0v14 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v15 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[28])), vreinterpretq_u64_u32(((S)[92]))));\
    t0v16 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v17 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[32])), vreinterpretq_u64_u32(((S)[96]))));\
    t0v18 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v19 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[36])), vreinterpretq_u64_u32(((S)[100]))));\
    t0v20 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v21 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[40])), vreinterpretq_u64_u32(((S)[104]))));\
    t0v22 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v23 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[44])), vreinterpretq_u64_u32(((S)[108]))));\
    t0v24 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v25 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[48])), vreinterpretq_u64_u32(((S)[112]))));\
    t0v26 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v27 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[52])), vreinterpretq_u64_u32(((S)[116]))));\
    t0v28 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v29 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[56])), vreinterpretq_u64_u32(((S)[120]))));\
    t0v30 = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v31 = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(((S)[60])), vreinterpretq_u64_u32(((S)[124]))));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v16), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v16, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v1, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v1), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v2, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v18), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v2), 32)), vandq_u32(t0v18, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v4, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v20), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v4), 32)), vandq_u32(t0v20, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v20 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v22), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v22, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v7, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v7), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v24), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v24, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v26), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v26, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v11, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v11), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v28), 32)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v28, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v28 = vorrq_u32(vandq_u32(t0v13, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v13), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v30), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v30, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v15, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v15), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    (D0) = vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vorrq_u32(vandq_u32(t0v32, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), vshlq_n_u32(vandq_u32(t0v0, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 1)), vshlq_n_u32(vandq_u32(t0v16, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 2)), vshlq_n_u32(vandq_u32(t0v1, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 3)), vshlq_n_u32(vandq_u32(t0v17, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 4)), vshlq_n_u32(vandq_u32(t0v2, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 5)), vshlq_n_u32(vandq_u32(t0v18, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 6)), vshlq_n_u32(vandq_u32(t0v3, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 7)), vshlq_n_u32(vandq_u32(t0v19, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 8)), vshlq_n_u32(vandq_u32(t0v4, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 9)), vshlq_n_u32(vandq_u32(t0v20, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 10)), vshlq_n_u32(vandq_u32(t0v5, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 11)), vshlq_n_u32(vandq_u32(t0v21, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 12)), vshlq_n_u32(vandq_u32(t0v6, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 13)), vshlq_n_u32(vandq_u32(t0v22, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 14)), vshlq_n_u32(vandq_u32(t0v7, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 15)), vshlq_n_u32(vandq_u32(t0v23, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 16)), vshlq_n_u32(vandq_u32(t0v8, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 17)), vshlq_n_u32(vandq_u32(t0v24, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 18)), vshlq_n_u32(vandq_u32(t0v9, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 19)), vshlq_n_u32(vandq_u32(t0v25, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 20)), vshlq_n_u32(vandq_u32(t0v10, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 21)), vshlq_n_u32(vandq_u32(t0v26, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 22)), vshlq_n_u32(vandq_u32(t0v11, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 23)), vshlq_n_u32(vandq_u32(t0v27, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 24)), vshlq_n_u32(vandq_u32(t0v12, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 25)), vshlq_n_u32(vandq_u32(t0v28, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 26)), vshlq_n_u32(vandq_u32(t0v13, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 27)), vshlq_n_u32(vandq_u32(t0v29, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 28)), vshlq_n_u32(vandq_u32(t0v14, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 29)), vshlq_n_u32(vandq_u32(t0v30, { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U }), 30)), vshlq_n_u32(t0v15, 31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_OPENCL_U32.transform();
    transform.gen_input_transform(32);
    transform.gen_input_transform(16);
    transform.gen_input_transform(6);
    transform.gen_input_transform(1);
    assert_eq!(
        r##"#define INPUT_TRANSFORM_B32(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, S) \
{\
    uint t0v0;\
    uint t0v1;\
    uint t0v2;\
    uint t0v3;\
    uint t0v4;\
    uint t0v5;\
    uint t0v6;\
    uint t0v7;\
    uint t0v8;\
    uint t0v9;\
    uint t0v10;\
    uint t0v11;\
    uint t0v12;\
    uint t0v13;\
    uint t0v14;\
    uint t0v15;\
    uint t0v16;\
    uint t0v17;\
    uint t0v18;\
    uint t0v19;\
    uint t0v20;\
    uint t0v21;\
    uint t0v22;\
    uint t0v23;\
    uint t0v24;\
    uint t0v25;\
    uint t0v26;\
    uint t0v27;\
    uint t0v28;\
    uint t0v29;\
    uint t0v30;\
    uint t0v31;\
    uint t0v32;\
    t0v0 = ((((S)[0]) & 0x0000ffffU) | (((S)[16]) << 16));\
    t0v1 = ((((S)[0]) >> 16) | (((S)[16]) & 0xffff0000U));\
    t0v2 = ((((S)[1]) & 0x0000ffffU) | (((S)[17]) << 16));\
    t0v3 = ((((S)[1]) >> 16) | (((S)[17]) & 0xffff0000U));\
    t0v4 = ((((S)[2]) & 0x0000ffffU) | (((S)[18]) << 16));\
    t0v5 = ((((S)[2]) >> 16) | (((S)[18]) & 0xffff0000U));\
    t0v6 = ((((S)[3]) & 0x0000ffffU) | (((S)[19]) << 16));\
    t0v7 = ((((S)[3]) >> 16) | (((S)[19]) & 0xffff0000U));\
    t0v8 = ((((S)[4]) & 0x0000ffffU) | (((S)[20]) << 16));\
    t0v9 = ((((S)[4]) >> 16) | (((S)[20]) & 0xffff0000U));\
    t0v10 = ((((S)[5]) & 0x0000ffffU) | (((S)[21]) << 16));\
    t0v11 = ((((S)[5]) >> 16) | (((S)[21]) & 0xffff0000U));\
    t0v12 = ((((S)[6]) & 0x0000ffffU) | (((S)[22]) << 16));\
    t0v13 = ((((S)[6]) >> 16) | (((S)[22]) & 0xffff0000U));\
    t0v14 = ((((S)[7]) & 0x0000ffffU) | (((S)[23]) << 16));\
    t0v15 = ((((S)[7]) >> 16) | (((S)[23]) & 0xffff0000U));\
    t0v16 = ((((S)[8]) & 0x0000ffffU) | (((S)[24]) << 16));\
    t0v17 = ((((S)[8]) >> 16) | (((S)[24]) & 0xffff0000U));\
    t0v18 = ((((S)[9]) & 0x0000ffffU) | (((S)[25]) << 16));\
    t0v19 = ((((S)[9]) >> 16) | (((S)[25]) & 0xffff0000U));\
    t0v20 = ((((S)[10]) & 0x0000ffffU) | (((S)[26]) << 16));\
    t0v21 = ((((S)[10]) >> 16) | (((S)[26]) & 0xffff0000U));\
    t0v22 = ((((S)[11]) & 0x0000ffffU) | (((S)[27]) << 16));\
    t0v23 = ((((S)[11]) >> 16) | (((S)[27]) & 0xffff0000U));\
    t0v24 = ((((S)[12]) & 0x0000ffffU) | (((S)[28]) << 16));\
    t0v25 = ((((S)[12]) >> 16) | (((S)[28]) & 0xffff0000U));\
    t0v26 = ((((S)[13]) & 0x0000ffffU) | (((S)[29]) << 16));\
    t0v27 = ((((S)[13]) >> 16) | (((S)[29]) & 0xffff0000U));\
    t0v28 = ((((S)[14]) & 0x0000ffffU) | (((S)[30]) << 16));\
    t0v29 = ((((S)[14]) >> 16) | (((S)[30]) & 0xffff0000U));\
    t0v30 = ((((S)[15]) & 0x0000ffffU) | (((S)[31]) << 16));\
    t0v31 = ((((S)[15]) >> 16) | (((S)[31]) & 0xffff0000U));\
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
#define INPUT_TRANSFORM_B16(D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, S) \
{\
    uint t0v0;\
    uint t0v1;\
    uint t0v2;\
    uint t0v3;\
    uint t0v4;\
    uint t0v5;\
    uint t0v6;\
    uint t0v7;\
    uint t0v8;\
    uint t0v9;\
    uint t0v10;\
    uint t0v11;\
    uint t0v12;\
    uint t0v13;\
    uint t0v14;\
    uint t0v15;\
    uint t0v16;\
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
#define INPUT_TRANSFORM_B6(D0, D1, D2, D3, D4, D5, S) \
{\
    uint t0v0;\
    uint t0v1;\
    uint t0v2;\
    uint t0v3;\
    uint t0v4;\
    uint t0v5;\
    uint t0v6;\
    uint t0v7;\
    uint t0v8;\
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
    t0v0 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v1 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    (D0) = ((t0v7 & 0x55555555U) | ((t0v8 & 0x55555555U) << 1));\
    (D1) = (((t0v7 & 0xaaaaaaaaU) >> 1) | (t0v8 & 0xaaaaaaaaU));\
    (D2) = ((t0v5 & 0x55555555U) | ((t0v4 & 0x55555555U) << 1));\
    (D3) = (((t0v5 & 0xaaaaaaaaU) >> 1) | (t0v4 & 0xaaaaaaaaU));\
    (D4) = ((t0v0 & 0x55555555U) | ((t0v1 & 0x55555555U) << 1));\
    (D5) = (((t0v0 & 0xaaaaaaaaU) >> 1) | (t0v1 & 0xaaaaaaaaU));\
}
#define INPUT_TRANSFORM_B1(D0, S) \
{\
    (D0) = ((((((((((((((((((((((((((((((((((S)[0]) & 0x1U) | ((((S)[1]) & 0x1U) << 1)) | ((((S)[2]) & 0x1U) << 2)) | ((((S)[3]) & 0x1U) << 3)) | ((((S)[4]) & 0x1U) << 4)) | ((((S)[5]) & 0x1U) << 5)) | ((((S)[6]) & 0x1U) << 6)) | ((((S)[7]) & 0x1U) << 7)) | ((((S)[8]) & 0x1U) << 8)) | ((((S)[9]) & 0x1U) << 9)) | ((((S)[10]) & 0x1U) << 10)) | ((((S)[11]) & 0x1U) << 11)) | ((((S)[12]) & 0x1U) << 12)) | ((((S)[13]) & 0x1U) << 13)) | ((((S)[14]) & 0x1U) << 14)) | ((((S)[15]) & 0x1U) << 15)) | ((((S)[16]) & 0x1U) << 16)) | ((((S)[17]) & 0x1U) << 17)) | ((((S)[18]) & 0x1U) << 18)) | ((((S)[19]) & 0x1U) << 19)) | ((((S)[20]) & 0x1U) << 20)) | ((((S)[21]) & 0x1U) << 21)) | ((((S)[22]) & 0x1U) << 22)) | ((((S)[23]) & 0x1U) << 23)) | ((((S)[24]) & 0x1U) << 24)) | ((((S)[25]) & 0x1U) << 25)) | ((((S)[26]) & 0x1U) << 26)) | ((((S)[27]) & 0x1U) << 27)) | ((((S)[28]) & 0x1U) << 28)) | ((((S)[29]) & 0x1U) << 29)) | ((((S)[30]) & 0x1U) << 30)) | (((S)[31]) << 31));\
}
"##,
        transform.out()
    );
}
