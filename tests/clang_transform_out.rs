use gatenative::clang_transform::*;

#[test]
fn test_clang_transform_gen_output_transform() {
    let mut transform = CLANG_TRANSFORM_U32.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = (((S6) & 0x55555555U) | (((S7) & 0x55555555U) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaU) >> 1) | ((S7) & 0xaaaaaaaaU));\
    t0v8 = (((S8) & 0x55555555U) | (((S9) & 0x55555555U) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaU) >> 1) | ((S9) & 0xaaaaaaaaU));\
    t0v10 = (((S10) & 0x55555555U) | (((S11) & 0x55555555U) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaU) >> 1) | ((S11) & 0xaaaaaaaaU));\
    t0v12 = (((S12) & 0x55555555U) | (((S13) & 0x55555555U) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaU) >> 1) | ((S13) & 0xaaaaaaaaU));\
    t0v14 = (((S14) & 0x55555555U) | (((S15) & 0x55555555U) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaU) >> 1) | ((S15) & 0xaaaaaaaaU));\
    t0v16 = (((S16) & 0x55555555U) | (((S17) & 0x55555555U) << 1));\
    t0v17 = ((((S16) & 0xaaaaaaaaU) >> 1) | ((S17) & 0xaaaaaaaaU));\
    t0v18 = (((S18) & 0x55555555U) | (((S19) & 0x55555555U) << 1));\
    t0v19 = ((((S18) & 0xaaaaaaaaU) >> 1) | ((S19) & 0xaaaaaaaaU));\
    t0v20 = (((S20) & 0x55555555U) | (((S21) & 0x55555555U) << 1));\
    t0v21 = ((((S20) & 0xaaaaaaaaU) >> 1) | ((S21) & 0xaaaaaaaaU));\
    t0v22 = (((S22) & 0x55555555U) | (((S23) & 0x55555555U) << 1));\
    t0v23 = ((((S22) & 0xaaaaaaaaU) >> 1) | ((S23) & 0xaaaaaaaaU));\
    t0v24 = (((S24) & 0x55555555U) | (((S25) & 0x55555555U) << 1));\
    t0v25 = ((((S24) & 0xaaaaaaaaU) >> 1) | ((S25) & 0xaaaaaaaaU));\
    t0v26 = (((S26) & 0x55555555U) | (((S27) & 0x55555555U) << 1));\
    t0v27 = ((((S26) & 0xaaaaaaaaU) >> 1) | ((S27) & 0xaaaaaaaaU));\
    t0v28 = (((S28) & 0x55555555U) | (((S29) & 0x55555555U) << 1));\
    t0v29 = ((((S28) & 0xaaaaaaaaU) >> 1) | ((S29) & 0xaaaaaaaaU));\
    t0v30 = (((S30) & 0x55555555U) | (((S31) & 0x55555555U) << 1));\
    t0v31 = ((((S30) & 0xaaaaaaaaU) >> 1) | ((S31) & 0xaaaaaaaaU));\
    t0v32 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x33333333U) | ((t0v18 & 0x33333333U) << 2));\
    t0v16 = (((t0v16 & 0xccccccccU) >> 2) | (t0v18 & 0xccccccccU));\
    t0v18 = ((t0v17 & 0x33333333U) | ((t0v19 & 0x33333333U) << 2));\
    t0v17 = (((t0v17 & 0xccccccccU) >> 2) | (t0v19 & 0xccccccccU));\
    t0v19 = ((t0v20 & 0x33333333U) | ((t0v22 & 0x33333333U) << 2));\
    t0v20 = (((t0v20 & 0xccccccccU) >> 2) | (t0v22 & 0xccccccccU));\
    t0v22 = ((t0v21 & 0x33333333U) | ((t0v23 & 0x33333333U) << 2));\
    t0v21 = (((t0v21 & 0xccccccccU) >> 2) | (t0v23 & 0xccccccccU));\
    t0v23 = ((t0v24 & 0x33333333U) | ((t0v26 & 0x33333333U) << 2));\
    t0v24 = (((t0v24 & 0xccccccccU) >> 2) | (t0v26 & 0xccccccccU));\
    t0v26 = ((t0v25 & 0x33333333U) | ((t0v27 & 0x33333333U) << 2));\
    t0v25 = (((t0v25 & 0xccccccccU) >> 2) | (t0v27 & 0xccccccccU));\
    t0v27 = ((t0v28 & 0x33333333U) | ((t0v30 & 0x33333333U) << 2));\
    t0v28 = (((t0v28 & 0xccccccccU) >> 2) | (t0v30 & 0xccccccccU));\
    t0v30 = ((t0v29 & 0x33333333U) | ((t0v31 & 0x33333333U) << 2));\
    t0v29 = (((t0v29 & 0xccccccccU) >> 2) | (t0v31 & 0xccccccccU));\
    t0v31 = ((t0v32 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v32 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v32 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x0f0f0f0fU) | ((t0v19 & 0x0f0f0f0fU) << 4));\
    t0v15 = (((t0v15 & 0xf0f0f0f0U) >> 4) | (t0v19 & 0xf0f0f0f0U));\
    t0v19 = ((t0v18 & 0x0f0f0f0fU) | ((t0v22 & 0x0f0f0f0fU) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0U) >> 4) | (t0v22 & 0xf0f0f0f0U));\
    t0v22 = ((t0v16 & 0x0f0f0f0fU) | ((t0v20 & 0x0f0f0f0fU) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v20 & 0xf0f0f0f0U));\
    t0v20 = ((t0v17 & 0x0f0f0f0fU) | ((t0v21 & 0x0f0f0f0fU) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0U) >> 4) | (t0v21 & 0xf0f0f0f0U));\
    t0v21 = ((t0v23 & 0x0f0f0f0fU) | ((t0v27 & 0x0f0f0f0fU) << 4));\
    t0v23 = (((t0v23 & 0xf0f0f0f0U) >> 4) | (t0v27 & 0xf0f0f0f0U));\
    t0v27 = ((t0v26 & 0x0f0f0f0fU) | ((t0v30 & 0x0f0f0f0fU) << 4));\
    t0v26 = (((t0v26 & 0xf0f0f0f0U) >> 4) | (t0v30 & 0xf0f0f0f0U));\
    t0v30 = ((t0v24 & 0x0f0f0f0fU) | ((t0v28 & 0x0f0f0f0fU) << 4));\
    t0v24 = (((t0v24 & 0xf0f0f0f0U) >> 4) | (t0v28 & 0xf0f0f0f0U));\
    t0v28 = ((t0v25 & 0x0f0f0f0fU) | ((t0v29 & 0x0f0f0f0fU) << 4));\
    t0v25 = (((t0v25 & 0xf0f0f0f0U) >> 4) | (t0v29 & 0xf0f0f0f0U));\
    t0v29 = ((t0v31 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v31 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v31 = ((t0v32 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v32 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v32 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    t0v9 = ((t0v13 & 0x00ff00ffU) | ((t0v21 & 0x00ff00ffU) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00U) >> 8) | (t0v21 & 0xff00ff00U));\
    t0v21 = ((t0v19 & 0x00ff00ffU) | ((t0v27 & 0x00ff00ffU) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00U) >> 8) | (t0v27 & 0xff00ff00U));\
    t0v27 = ((t0v22 & 0x00ff00ffU) | ((t0v30 & 0x00ff00ffU) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00U) >> 8) | (t0v30 & 0xff00ff00U));\
    t0v30 = ((t0v20 & 0x00ff00ffU) | ((t0v28 & 0x00ff00ffU) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00U) >> 8) | (t0v28 & 0xff00ff00U));\
    t0v28 = ((t0v15 & 0x00ff00ffU) | ((t0v23 & 0x00ff00ffU) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v23 & 0xff00ff00U));\
    t0v23 = ((t0v18 & 0x00ff00ffU) | ((t0v26 & 0x00ff00ffU) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00U) >> 8) | (t0v26 & 0xff00ff00U));\
    t0v26 = ((t0v16 & 0x00ff00ffU) | ((t0v24 & 0x00ff00ffU) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v24 & 0xff00ff00U));\
    t0v24 = ((t0v17 & 0x00ff00ffU) | ((t0v25 & 0x00ff00ffU) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00U) >> 8) | (t0v25 & 0xff00ff00U));\
    ((D)[0]) = ((t0v29 & 0x0000ffffU) | (t0v9 << 16));\
    ((D)[16]) = ((t0v29 >> 16) | (t0v9 & 0xffff0000U));\
    ((D)[1]) = ((t0v31 & 0x0000ffffU) | (t0v21 << 16));\
    ((D)[17]) = ((t0v31 >> 16) | (t0v21 & 0xffff0000U));\
    ((D)[2]) = ((t0v32 & 0x0000ffffU) | (t0v27 << 16));\
    ((D)[18]) = ((t0v32 >> 16) | (t0v27 & 0xffff0000U));\
    ((D)[3]) = ((t0v14 & 0x0000ffffU) | (t0v30 << 16));\
    ((D)[19]) = ((t0v14 >> 16) | (t0v30 & 0xffff0000U));\
    ((D)[4]) = ((t0v12 & 0x0000ffffU) | (t0v28 << 16));\
    ((D)[20]) = ((t0v12 >> 16) | (t0v28 & 0xffff0000U));\
    ((D)[5]) = ((t0v7 & 0x0000ffffU) | (t0v23 << 16));\
    ((D)[21]) = ((t0v7 >> 16) | (t0v23 & 0xffff0000U));\
    ((D)[6]) = ((t0v10 & 0x0000ffffU) | (t0v26 << 16));\
    ((D)[22]) = ((t0v10 >> 16) | (t0v26 & 0xffff0000U));\
    ((D)[7]) = ((t0v8 & 0x0000ffffU) | (t0v24 << 16));\
    ((D)[23]) = ((t0v8 >> 16) | (t0v24 & 0xffff0000U));\
    ((D)[8]) = ((t0v5 & 0x0000ffffU) | (t0v13 << 16));\
    ((D)[24]) = ((t0v5 >> 16) | (t0v13 & 0xffff0000U));\
    ((D)[9]) = ((t0v11 & 0x0000ffffU) | (t0v19 << 16));\
    ((D)[25]) = ((t0v11 >> 16) | (t0v19 & 0xffff0000U));\
    ((D)[10]) = ((t0v6 & 0x0000ffffU) | (t0v22 << 16));\
    ((D)[26]) = ((t0v6 >> 16) | (t0v22 & 0xffff0000U));\
    ((D)[11]) = ((t0v4 & 0x0000ffffU) | (t0v20 << 16));\
    ((D)[27]) = ((t0v4 >> 16) | (t0v20 & 0xffff0000U));\
    ((D)[12]) = ((t0v3 & 0x0000ffffU) | (t0v15 << 16));\
    ((D)[28]) = ((t0v3 >> 16) | (t0v15 & 0xffff0000U));\
    ((D)[13]) = ((t0v2 & 0x0000ffffU) | (t0v18 << 16));\
    ((D)[29]) = ((t0v2 >> 16) | (t0v18 & 0xffff0000U));\
    ((D)[14]) = ((t0v0 & 0x0000ffffU) | (t0v16 << 16));\
    ((D)[30]) = ((t0v0 >> 16) | (t0v16 & 0xffff0000U));\
    ((D)[15]) = ((t0v1 & 0x0000ffffU) | (t0v17 << 16));\
    ((D)[31]) = ((t0v1 >> 16) | (t0v17 & 0xffff0000U));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = (((S6) & 0x55555555U) | (((S7) & 0x55555555U) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaU) >> 1) | ((S7) & 0xaaaaaaaaU));\
    t0v8 = (((S8) & 0x55555555U) | (((S9) & 0x55555555U) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaU) >> 1) | ((S9) & 0xaaaaaaaaU));\
    t0v10 = (((S10) & 0x55555555U) | (((S11) & 0x55555555U) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaU) >> 1) | ((S11) & 0xaaaaaaaaU));\
    t0v12 = (((S12) & 0x55555555U) | (((S13) & 0x55555555U) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaU) >> 1) | ((S13) & 0xaaaaaaaaU));\
    t0v14 = (((S14) & 0x55555555U) | (((S15) & 0x55555555U) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaU) >> 1) | ((S15) & 0xaaaaaaaaU));\
    t0v16 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v16 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v15 = ((t0v16 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v16 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    ((D)[0]) = (t0v13 & 0xffffU);\
    ((D)[16]) = (t0v13 >> 16);\
    ((D)[1]) = (t0v15 & 0xffffU);\
    ((D)[17]) = (t0v15 >> 16);\
    ((D)[2]) = (t0v16 & 0xffffU);\
    ((D)[18]) = (t0v16 >> 16);\
    ((D)[3]) = (t0v14 & 0xffffU);\
    ((D)[19]) = (t0v14 >> 16);\
    ((D)[4]) = (t0v12 & 0xffffU);\
    ((D)[20]) = (t0v12 >> 16);\
    ((D)[5]) = (t0v7 & 0xffffU);\
    ((D)[21]) = (t0v7 >> 16);\
    ((D)[6]) = (t0v10 & 0xffffU);\
    ((D)[22]) = (t0v10 >> 16);\
    ((D)[7]) = (t0v8 & 0xffffU);\
    ((D)[23]) = (t0v8 >> 16);\
    ((D)[8]) = (t0v5 & 0xffffU);\
    ((D)[24]) = (t0v5 >> 16);\
    ((D)[9]) = (t0v11 & 0xffffU);\
    ((D)[25]) = (t0v11 >> 16);\
    ((D)[10]) = (t0v6 & 0xffffU);\
    ((D)[26]) = (t0v6 >> 16);\
    ((D)[11]) = (t0v4 & 0xffffU);\
    ((D)[27]) = (t0v4 >> 16);\
    ((D)[12]) = (t0v3 & 0xffffU);\
    ((D)[28]) = (t0v3 >> 16);\
    ((D)[13]) = (t0v2 & 0xffffU);\
    ((D)[29]) = (t0v2 >> 16);\
    ((D)[14]) = (t0v0 & 0xffffU);\
    ((D)[30]) = (t0v0 >> 16);\
    ((D)[15]) = (t0v1 & 0xffffU);\
    ((D)[31]) = (t0v1 >> 16);\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = (t0v4 & 0x33333333U);\
    t0v4 = ((t0v4 & 0xccccccccU) >> 2);\
    t0v7 = (t0v5 & 0x33333333U);\
    t0v5 = ((t0v5 & 0xccccccccU) >> 2);\
    t0v8 = ((t0v6 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v6 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v6 = ((t0v2 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    ((D)[0]) = (t0v8 & 0xffU);\
    ((D)[8]) = ((t0v8 >> 8) & 0xffU);\
    ((D)[16]) = ((t0v8 >> 16) & 0xffU);\
    ((D)[24]) = (t0v8 >> 24);\
    ((D)[1]) = (t0v6 & 0xffU);\
    ((D)[9]) = ((t0v6 >> 8) & 0xffU);\
    ((D)[17]) = ((t0v6 >> 16) & 0xffU);\
    ((D)[25]) = (t0v6 >> 24);\
    ((D)[2]) = (t0v7 & 0xffU);\
    ((D)[10]) = ((t0v7 >> 8) & 0xffU);\
    ((D)[18]) = ((t0v7 >> 16) & 0xffU);\
    ((D)[26]) = (t0v7 >> 24);\
    ((D)[3]) = (t0v4 & 0xffU);\
    ((D)[11]) = ((t0v4 >> 8) & 0xffU);\
    ((D)[19]) = ((t0v4 >> 16) & 0xffU);\
    ((D)[27]) = (t0v4 >> 24);\
    ((D)[4]) = (t0v3 & 0xffU);\
    ((D)[12]) = ((t0v3 >> 8) & 0xffU);\
    ((D)[20]) = ((t0v3 >> 16) & 0xffU);\
    ((D)[28]) = (t0v3 >> 24);\
    ((D)[5]) = (t0v2 & 0xffU);\
    ((D)[13]) = ((t0v2 >> 8) & 0xffU);\
    ((D)[21]) = ((t0v2 >> 16) & 0xffU);\
    ((D)[29]) = (t0v2 >> 24);\
    ((D)[6]) = (t0v0 & 0xffU);\
    ((D)[14]) = ((t0v0 >> 8) & 0xffU);\
    ((D)[22]) = ((t0v0 >> 16) & 0xffU);\
    ((D)[30]) = (t0v0 >> 24);\
    ((D)[7]) = (t0v1 & 0xffU);\
    ((D)[15]) = ((t0v1 >> 8) & 0xffU);\
    ((D)[23]) = ((t0v1 >> 16) & 0xffU);\
    ((D)[31]) = (t0v1 >> 24);\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    ((D)[0]) = ((S0) & 0x1U);\
    ((D)[1]) = (((S0) >> 1) & 0x1U);\
    ((D)[2]) = (((S0) >> 2) & 0x1U);\
    ((D)[3]) = (((S0) >> 3) & 0x1U);\
    ((D)[4]) = (((S0) >> 4) & 0x1U);\
    ((D)[5]) = (((S0) >> 5) & 0x1U);\
    ((D)[6]) = (((S0) >> 6) & 0x1U);\
    ((D)[7]) = (((S0) >> 7) & 0x1U);\
    ((D)[8]) = (((S0) >> 8) & 0x1U);\
    ((D)[9]) = (((S0) >> 9) & 0x1U);\
    ((D)[10]) = (((S0) >> 10) & 0x1U);\
    ((D)[11]) = (((S0) >> 11) & 0x1U);\
    ((D)[12]) = (((S0) >> 12) & 0x1U);\
    ((D)[13]) = (((S0) >> 13) & 0x1U);\
    ((D)[14]) = (((S0) >> 14) & 0x1U);\
    ((D)[15]) = (((S0) >> 15) & 0x1U);\
    ((D)[16]) = (((S0) >> 16) & 0x1U);\
    ((D)[17]) = (((S0) >> 17) & 0x1U);\
    ((D)[18]) = (((S0) >> 18) & 0x1U);\
    ((D)[19]) = (((S0) >> 19) & 0x1U);\
    ((D)[20]) = (((S0) >> 20) & 0x1U);\
    ((D)[21]) = (((S0) >> 21) & 0x1U);\
    ((D)[22]) = (((S0) >> 22) & 0x1U);\
    ((D)[23]) = (((S0) >> 23) & 0x1U);\
    ((D)[24]) = (((S0) >> 24) & 0x1U);\
    ((D)[25]) = (((S0) >> 25) & 0x1U);\
    ((D)[26]) = (((S0) >> 26) & 0x1U);\
    ((D)[27]) = (((S0) >> 27) & 0x1U);\
    ((D)[28]) = (((S0) >> 28) & 0x1U);\
    ((D)[29]) = (((S0) >> 29) & 0x1U);\
    ((D)[30]) = (((S0) >> 30) & 0x1U);\
    ((D)[31]) = ((S0) >> 31);\
}
"##,
        transform.out(),
    );
    let mut transform = CLANG_TRANSFORM_U64.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    uint64_t* dest = (uint64_t*)(D);\
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
    t0v0 = (((S0) & 0x5555555555555555ULL) | (((S1) & 0x5555555555555555ULL) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S1) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v2 = (((S2) & 0x5555555555555555ULL) | (((S3) & 0x5555555555555555ULL) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S3) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v4 = (((S4) & 0x5555555555555555ULL) | (((S5) & 0x5555555555555555ULL) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S5) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v6 = (((S6) & 0x5555555555555555ULL) | (((S7) & 0x5555555555555555ULL) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S7) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v8 = (((S8) & 0x5555555555555555ULL) | (((S9) & 0x5555555555555555ULL) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S9) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v10 = (((S10) & 0x5555555555555555ULL) | (((S11) & 0x5555555555555555ULL) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S11) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v12 = (((S12) & 0x5555555555555555ULL) | (((S13) & 0x5555555555555555ULL) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S13) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v14 = (((S14) & 0x5555555555555555ULL) | (((S15) & 0x5555555555555555ULL) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S15) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v16 = (((S16) & 0x5555555555555555ULL) | (((S17) & 0x5555555555555555ULL) << 1));\
    t0v17 = ((((S16) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S17) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v18 = (((S18) & 0x5555555555555555ULL) | (((S19) & 0x5555555555555555ULL) << 1));\
    t0v19 = ((((S18) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S19) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v20 = (((S20) & 0x5555555555555555ULL) | (((S21) & 0x5555555555555555ULL) << 1));\
    t0v21 = ((((S20) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S21) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v22 = (((S22) & 0x5555555555555555ULL) | (((S23) & 0x5555555555555555ULL) << 1));\
    t0v23 = ((((S22) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S23) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v24 = (((S24) & 0x5555555555555555ULL) | (((S25) & 0x5555555555555555ULL) << 1));\
    t0v25 = ((((S24) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S25) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v26 = (((S26) & 0x5555555555555555ULL) | (((S27) & 0x5555555555555555ULL) << 1));\
    t0v27 = ((((S26) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S27) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v28 = (((S28) & 0x5555555555555555ULL) | (((S29) & 0x5555555555555555ULL) << 1));\
    t0v29 = ((((S28) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S29) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v30 = (((S30) & 0x5555555555555555ULL) | (((S31) & 0x5555555555555555ULL) << 1));\
    t0v31 = ((((S30) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S31) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v32 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    t0v2 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v1 & 0xccccccccccccccccULL) >> 2) | (t0v3 & 0xccccccccccccccccULL));\
    t0v3 = ((t0v4 & 0x3333333333333333ULL) | ((t0v6 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v4 & 0xccccccccccccccccULL) >> 2) | (t0v6 & 0xccccccccccccccccULL));\
    t0v6 = ((t0v5 & 0x3333333333333333ULL) | ((t0v7 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v5 & 0xccccccccccccccccULL) >> 2) | (t0v7 & 0xccccccccccccccccULL));\
    t0v7 = ((t0v8 & 0x3333333333333333ULL) | ((t0v10 & 0x3333333333333333ULL) << 2));\
    t0v8 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v10 & 0xccccccccccccccccULL));\
    t0v10 = ((t0v9 & 0x3333333333333333ULL) | ((t0v11 & 0x3333333333333333ULL) << 2));\
    t0v9 = (((t0v9 & 0xccccccccccccccccULL) >> 2) | (t0v11 & 0xccccccccccccccccULL));\
    t0v11 = ((t0v12 & 0x3333333333333333ULL) | ((t0v14 & 0x3333333333333333ULL) << 2));\
    t0v12 = (((t0v12 & 0xccccccccccccccccULL) >> 2) | (t0v14 & 0xccccccccccccccccULL));\
    t0v14 = ((t0v13 & 0x3333333333333333ULL) | ((t0v15 & 0x3333333333333333ULL) << 2));\
    t0v13 = (((t0v13 & 0xccccccccccccccccULL) >> 2) | (t0v15 & 0xccccccccccccccccULL));\
    t0v15 = ((t0v16 & 0x3333333333333333ULL) | ((t0v18 & 0x3333333333333333ULL) << 2));\
    t0v16 = (((t0v16 & 0xccccccccccccccccULL) >> 2) | (t0v18 & 0xccccccccccccccccULL));\
    t0v18 = ((t0v17 & 0x3333333333333333ULL) | ((t0v19 & 0x3333333333333333ULL) << 2));\
    t0v17 = (((t0v17 & 0xccccccccccccccccULL) >> 2) | (t0v19 & 0xccccccccccccccccULL));\
    t0v19 = ((t0v20 & 0x3333333333333333ULL) | ((t0v22 & 0x3333333333333333ULL) << 2));\
    t0v20 = (((t0v20 & 0xccccccccccccccccULL) >> 2) | (t0v22 & 0xccccccccccccccccULL));\
    t0v22 = ((t0v21 & 0x3333333333333333ULL) | ((t0v23 & 0x3333333333333333ULL) << 2));\
    t0v21 = (((t0v21 & 0xccccccccccccccccULL) >> 2) | (t0v23 & 0xccccccccccccccccULL));\
    t0v23 = ((t0v24 & 0x3333333333333333ULL) | ((t0v26 & 0x3333333333333333ULL) << 2));\
    t0v24 = (((t0v24 & 0xccccccccccccccccULL) >> 2) | (t0v26 & 0xccccccccccccccccULL));\
    t0v26 = ((t0v25 & 0x3333333333333333ULL) | ((t0v27 & 0x3333333333333333ULL) << 2));\
    t0v25 = (((t0v25 & 0xccccccccccccccccULL) >> 2) | (t0v27 & 0xccccccccccccccccULL));\
    t0v27 = ((t0v28 & 0x3333333333333333ULL) | ((t0v30 & 0x3333333333333333ULL) << 2));\
    t0v28 = (((t0v28 & 0xccccccccccccccccULL) >> 2) | (t0v30 & 0xccccccccccccccccULL));\
    t0v30 = ((t0v29 & 0x3333333333333333ULL) | ((t0v31 & 0x3333333333333333ULL) << 2));\
    t0v29 = (((t0v29 & 0xccccccccccccccccULL) >> 2) | (t0v31 & 0xccccccccccccccccULL));\
    t0v31 = ((t0v32 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v32 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v3 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v32 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v11 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v11 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v11 = ((t0v10 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v14 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v14 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v14 = ((t0v8 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v12 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v12 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v12 = ((t0v9 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v13 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v13 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v13 = ((t0v15 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v19 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v15 = (((t0v15 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v19 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v19 = ((t0v18 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v22 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v22 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v22 = ((t0v16 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v20 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v20 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v20 = ((t0v17 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v21 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v21 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v21 = ((t0v23 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v27 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v23 = (((t0v23 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v27 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v27 = ((t0v26 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v30 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v26 = (((t0v26 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v30 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v30 = ((t0v24 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v28 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v24 = (((t0v24 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v28 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v28 = ((t0v25 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v29 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v25 = (((t0v25 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v29 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v29 = ((t0v31 & 0x00ff00ff00ff00ffULL) | ((t0v5 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v5 = (((t0v31 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v5 & 0xff00ff00ff00ff00ULL));\
    t0v31 = ((t0v32 & 0x00ff00ff00ff00ffULL) | ((t0v11 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v11 = (((t0v32 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v11 & 0xff00ff00ff00ff00ULL));\
    t0v32 = ((t0v6 & 0x00ff00ff00ff00ffULL) | ((t0v14 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v14 & 0xff00ff00ff00ff00ULL));\
    t0v14 = ((t0v4 & 0x00ff00ff00ff00ffULL) | ((t0v12 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v12 & 0xff00ff00ff00ff00ULL));\
    t0v12 = ((t0v3 & 0x00ff00ff00ff00ffULL) | ((t0v7 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v7 & 0xff00ff00ff00ff00ULL));\
    t0v7 = ((t0v2 & 0x00ff00ff00ff00ffULL) | ((t0v10 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v10 & 0xff00ff00ff00ff00ULL));\
    t0v10 = ((t0v0 & 0x00ff00ff00ff00ffULL) | ((t0v8 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v8 & 0xff00ff00ff00ff00ULL));\
    t0v8 = ((t0v1 & 0x00ff00ff00ff00ffULL) | ((t0v9 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v9 & 0xff00ff00ff00ff00ULL));\
    t0v9 = ((t0v13 & 0x00ff00ff00ff00ffULL) | ((t0v21 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v21 & 0xff00ff00ff00ff00ULL));\
    t0v21 = ((t0v19 & 0x00ff00ff00ff00ffULL) | ((t0v27 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v27 & 0xff00ff00ff00ff00ULL));\
    t0v27 = ((t0v22 & 0x00ff00ff00ff00ffULL) | ((t0v30 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v30 & 0xff00ff00ff00ff00ULL));\
    t0v30 = ((t0v20 & 0x00ff00ff00ff00ffULL) | ((t0v28 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v28 & 0xff00ff00ff00ff00ULL));\
    t0v28 = ((t0v15 & 0x00ff00ff00ff00ffULL) | ((t0v23 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v23 & 0xff00ff00ff00ff00ULL));\
    t0v23 = ((t0v18 & 0x00ff00ff00ff00ffULL) | ((t0v26 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v26 & 0xff00ff00ff00ff00ULL));\
    t0v26 = ((t0v16 & 0x00ff00ff00ff00ffULL) | ((t0v24 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v24 & 0xff00ff00ff00ff00ULL));\
    t0v24 = ((t0v17 & 0x00ff00ff00ff00ffULL) | ((t0v25 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v25 & 0xff00ff00ff00ff00ULL));\
    t0v25 = ((t0v29 & 0x0000ffff0000ffffULL) | ((t0v9 & 0x0000ffff0000ffffULL) << 16));\
    t0v9 = (((t0v29 & 0xffff0000ffff0000ULL) >> 16) | (t0v9 & 0xffff0000ffff0000ULL));\
    t0v29 = ((t0v31 & 0x0000ffff0000ffffULL) | ((t0v21 & 0x0000ffff0000ffffULL) << 16));\
    t0v21 = (((t0v31 & 0xffff0000ffff0000ULL) >> 16) | (t0v21 & 0xffff0000ffff0000ULL));\
    t0v31 = ((t0v32 & 0x0000ffff0000ffffULL) | ((t0v27 & 0x0000ffff0000ffffULL) << 16));\
    t0v27 = (((t0v32 & 0xffff0000ffff0000ULL) >> 16) | (t0v27 & 0xffff0000ffff0000ULL));\
    t0v32 = ((t0v14 & 0x0000ffff0000ffffULL) | ((t0v30 & 0x0000ffff0000ffffULL) << 16));\
    t0v14 = (((t0v14 & 0xffff0000ffff0000ULL) >> 16) | (t0v30 & 0xffff0000ffff0000ULL));\
    t0v30 = ((t0v12 & 0x0000ffff0000ffffULL) | ((t0v28 & 0x0000ffff0000ffffULL) << 16));\
    t0v12 = (((t0v12 & 0xffff0000ffff0000ULL) >> 16) | (t0v28 & 0xffff0000ffff0000ULL));\
    t0v28 = ((t0v7 & 0x0000ffff0000ffffULL) | ((t0v23 & 0x0000ffff0000ffffULL) << 16));\
    t0v7 = (((t0v7 & 0xffff0000ffff0000ULL) >> 16) | (t0v23 & 0xffff0000ffff0000ULL));\
    t0v23 = ((t0v10 & 0x0000ffff0000ffffULL) | ((t0v26 & 0x0000ffff0000ffffULL) << 16));\
    t0v10 = (((t0v10 & 0xffff0000ffff0000ULL) >> 16) | (t0v26 & 0xffff0000ffff0000ULL));\
    t0v26 = ((t0v8 & 0x0000ffff0000ffffULL) | ((t0v24 & 0x0000ffff0000ffffULL) << 16));\
    t0v8 = (((t0v8 & 0xffff0000ffff0000ULL) >> 16) | (t0v24 & 0xffff0000ffff0000ULL));\
    t0v24 = ((t0v5 & 0x0000ffff0000ffffULL) | ((t0v13 & 0x0000ffff0000ffffULL) << 16));\
    t0v5 = (((t0v5 & 0xffff0000ffff0000ULL) >> 16) | (t0v13 & 0xffff0000ffff0000ULL));\
    t0v13 = ((t0v11 & 0x0000ffff0000ffffULL) | ((t0v19 & 0x0000ffff0000ffffULL) << 16));\
    t0v11 = (((t0v11 & 0xffff0000ffff0000ULL) >> 16) | (t0v19 & 0xffff0000ffff0000ULL));\
    t0v19 = ((t0v6 & 0x0000ffff0000ffffULL) | ((t0v22 & 0x0000ffff0000ffffULL) << 16));\
    t0v6 = (((t0v6 & 0xffff0000ffff0000ULL) >> 16) | (t0v22 & 0xffff0000ffff0000ULL));\
    t0v22 = ((t0v4 & 0x0000ffff0000ffffULL) | ((t0v20 & 0x0000ffff0000ffffULL) << 16));\
    t0v4 = (((t0v4 & 0xffff0000ffff0000ULL) >> 16) | (t0v20 & 0xffff0000ffff0000ULL));\
    t0v20 = ((t0v3 & 0x0000ffff0000ffffULL) | ((t0v15 & 0x0000ffff0000ffffULL) << 16));\
    t0v3 = (((t0v3 & 0xffff0000ffff0000ULL) >> 16) | (t0v15 & 0xffff0000ffff0000ULL));\
    t0v15 = ((t0v2 & 0x0000ffff0000ffffULL) | ((t0v18 & 0x0000ffff0000ffffULL) << 16));\
    t0v2 = (((t0v2 & 0xffff0000ffff0000ULL) >> 16) | (t0v18 & 0xffff0000ffff0000ULL));\
    t0v18 = ((t0v0 & 0x0000ffff0000ffffULL) | ((t0v16 & 0x0000ffff0000ffffULL) << 16));\
    t0v0 = (((t0v0 & 0xffff0000ffff0000ULL) >> 16) | (t0v16 & 0xffff0000ffff0000ULL));\
    t0v16 = ((t0v1 & 0x0000ffff0000ffffULL) | ((t0v17 & 0x0000ffff0000ffffULL) << 16));\
    t0v1 = (((t0v1 & 0xffff0000ffff0000ULL) >> 16) | (t0v17 & 0xffff0000ffff0000ULL));\
    *((uint64_t*)(&((dest[0])))) = ((t0v25 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    *((uint64_t*)(&((dest[16])))) = ((t0v25 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[1])))) = ((t0v31 & 0x00000000ffffffffULL) | (t0v32 << 32));\
    *((uint64_t*)(&((dest[17])))) = ((t0v31 >> 32) | (t0v32 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[2])))) = ((t0v30 & 0x00000000ffffffffULL) | (t0v28 << 32));\
    *((uint64_t*)(&((dest[18])))) = ((t0v30 >> 32) | (t0v28 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[3])))) = ((t0v23 & 0x00000000ffffffffULL) | (t0v26 << 32));\
    *((uint64_t*)(&((dest[19])))) = ((t0v23 >> 32) | (t0v26 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[4])))) = ((t0v24 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    *((uint64_t*)(&((dest[20])))) = ((t0v24 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[5])))) = ((t0v19 & 0x00000000ffffffffULL) | (t0v22 << 32));\
    *((uint64_t*)(&((dest[21])))) = ((t0v19 >> 32) | (t0v22 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[6])))) = ((t0v20 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    *((uint64_t*)(&((dest[22])))) = ((t0v20 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[7])))) = ((t0v18 & 0x00000000ffffffffULL) | (t0v16 << 32));\
    *((uint64_t*)(&((dest[23])))) = ((t0v18 >> 32) | (t0v16 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[8])))) = ((t0v9 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    *((uint64_t*)(&((dest[24])))) = ((t0v9 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[9])))) = ((t0v27 & 0x00000000ffffffffULL) | (t0v14 << 32));\
    *((uint64_t*)(&((dest[25])))) = ((t0v27 >> 32) | (t0v14 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[10])))) = ((t0v12 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    *((uint64_t*)(&((dest[26])))) = ((t0v12 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[11])))) = ((t0v10 & 0x00000000ffffffffULL) | (t0v8 << 32));\
    *((uint64_t*)(&((dest[27])))) = ((t0v10 >> 32) | (t0v8 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[12])))) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    *((uint64_t*)(&((dest[28])))) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[13])))) = ((t0v6 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    *((uint64_t*)(&((dest[29])))) = ((t0v6 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[14])))) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    *((uint64_t*)(&((dest[30])))) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[15])))) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    *((uint64_t*)(&((dest[31])))) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    uint64_t* dest = (uint64_t*)(D);\
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
    t0v0 = (((S0) & 0x5555555555555555ULL) | (((S1) & 0x5555555555555555ULL) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S1) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v2 = (((S2) & 0x5555555555555555ULL) | (((S3) & 0x5555555555555555ULL) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S3) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v4 = (((S4) & 0x5555555555555555ULL) | (((S5) & 0x5555555555555555ULL) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S5) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v6 = (((S6) & 0x5555555555555555ULL) | (((S7) & 0x5555555555555555ULL) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S7) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v8 = (((S8) & 0x5555555555555555ULL) | (((S9) & 0x5555555555555555ULL) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S9) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v10 = (((S10) & 0x5555555555555555ULL) | (((S11) & 0x5555555555555555ULL) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S11) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v12 = (((S12) & 0x5555555555555555ULL) | (((S13) & 0x5555555555555555ULL) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S13) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v14 = (((S14) & 0x5555555555555555ULL) | (((S15) & 0x5555555555555555ULL) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S15) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v16 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    t0v2 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v1 & 0xccccccccccccccccULL) >> 2) | (t0v3 & 0xccccccccccccccccULL));\
    t0v3 = ((t0v4 & 0x3333333333333333ULL) | ((t0v6 & 0x3333333333333333ULL) << 2));\
    t0v4 = (((t0v4 & 0xccccccccccccccccULL) >> 2) | (t0v6 & 0xccccccccccccccccULL));\
    t0v6 = ((t0v5 & 0x3333333333333333ULL) | ((t0v7 & 0x3333333333333333ULL) << 2));\
    t0v5 = (((t0v5 & 0xccccccccccccccccULL) >> 2) | (t0v7 & 0xccccccccccccccccULL));\
    t0v7 = ((t0v8 & 0x3333333333333333ULL) | ((t0v10 & 0x3333333333333333ULL) << 2));\
    t0v8 = (((t0v8 & 0xccccccccccccccccULL) >> 2) | (t0v10 & 0xccccccccccccccccULL));\
    t0v10 = ((t0v9 & 0x3333333333333333ULL) | ((t0v11 & 0x3333333333333333ULL) << 2));\
    t0v9 = (((t0v9 & 0xccccccccccccccccULL) >> 2) | (t0v11 & 0xccccccccccccccccULL));\
    t0v11 = ((t0v12 & 0x3333333333333333ULL) | ((t0v14 & 0x3333333333333333ULL) << 2));\
    t0v12 = (((t0v12 & 0xccccccccccccccccULL) >> 2) | (t0v14 & 0xccccccccccccccccULL));\
    t0v14 = ((t0v13 & 0x3333333333333333ULL) | ((t0v15 & 0x3333333333333333ULL) << 2));\
    t0v13 = (((t0v13 & 0xccccccccccccccccULL) >> 2) | (t0v15 & 0xccccccccccccccccULL));\
    t0v15 = ((t0v16 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v16 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v3 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v16 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v6 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v11 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v11 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v11 = ((t0v10 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v14 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v14 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v14 = ((t0v8 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v12 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v12 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v12 = ((t0v9 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v13 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v13 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v13 = ((t0v15 & 0x00ff00ff00ff00ffULL) | ((t0v5 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v5 = (((t0v15 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v5 & 0xff00ff00ff00ff00ULL));\
    t0v15 = ((t0v16 & 0x00ff00ff00ff00ffULL) | ((t0v11 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v11 = (((t0v16 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v11 & 0xff00ff00ff00ff00ULL));\
    t0v16 = ((t0v6 & 0x00ff00ff00ff00ffULL) | ((t0v14 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v14 & 0xff00ff00ff00ff00ULL));\
    t0v14 = ((t0v4 & 0x00ff00ff00ff00ffULL) | ((t0v12 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v12 & 0xff00ff00ff00ff00ULL));\
    t0v12 = ((t0v3 & 0x00ff00ff00ff00ffULL) | ((t0v7 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v7 & 0xff00ff00ff00ff00ULL));\
    t0v7 = ((t0v2 & 0x00ff00ff00ff00ffULL) | ((t0v10 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v10 & 0xff00ff00ff00ff00ULL));\
    t0v10 = ((t0v0 & 0x00ff00ff00ff00ffULL) | ((t0v8 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v8 & 0xff00ff00ff00ff00ULL));\
    t0v8 = ((t0v1 & 0x00ff00ff00ff00ffULL) | ((t0v9 & 0x00ff00ff00ff00ffULL) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00ff00ff00ULL) >> 8) | (t0v9 & 0xff00ff00ff00ff00ULL));\
    t0v9 = (t0v13 & 0x0000ffff0000ffffULL);\
    t0v13 = ((t0v13 >> 16) & 0x0000ffff0000ffffULL);\
    t0v17 = (t0v15 & 0x0000ffff0000ffffULL);\
    t0v15 = ((t0v15 >> 16) & 0x0000ffff0000ffffULL);\
    t0v18 = (t0v16 & 0x0000ffff0000ffffULL);\
    t0v16 = ((t0v16 >> 16) & 0x0000ffff0000ffffULL);\
    t0v19 = (t0v14 & 0x0000ffff0000ffffULL);\
    t0v14 = ((t0v14 >> 16) & 0x0000ffff0000ffffULL);\
    t0v20 = (t0v12 & 0x0000ffff0000ffffULL);\
    t0v12 = ((t0v12 >> 16) & 0x0000ffff0000ffffULL);\
    t0v21 = (t0v7 & 0x0000ffff0000ffffULL);\
    t0v7 = ((t0v7 >> 16) & 0x0000ffff0000ffffULL);\
    t0v22 = (t0v10 & 0x0000ffff0000ffffULL);\
    t0v10 = ((t0v10 >> 16) & 0x0000ffff0000ffffULL);\
    t0v23 = (t0v8 & 0x0000ffff0000ffffULL);\
    t0v8 = ((t0v8 >> 16) & 0x0000ffff0000ffffULL);\
    t0v24 = (t0v5 & 0x0000ffff0000ffffULL);\
    t0v5 = ((t0v5 >> 16) & 0x0000ffff0000ffffULL);\
    t0v25 = (t0v11 & 0x0000ffff0000ffffULL);\
    t0v11 = ((t0v11 >> 16) & 0x0000ffff0000ffffULL);\
    t0v26 = (t0v6 & 0x0000ffff0000ffffULL);\
    t0v6 = ((t0v6 >> 16) & 0x0000ffff0000ffffULL);\
    t0v27 = (t0v4 & 0x0000ffff0000ffffULL);\
    t0v4 = ((t0v4 >> 16) & 0x0000ffff0000ffffULL);\
    t0v28 = (t0v3 & 0x0000ffff0000ffffULL);\
    t0v3 = ((t0v3 >> 16) & 0x0000ffff0000ffffULL);\
    t0v29 = (t0v2 & 0x0000ffff0000ffffULL);\
    t0v2 = ((t0v2 >> 16) & 0x0000ffff0000ffffULL);\
    t0v30 = (t0v0 & 0x0000ffff0000ffffULL);\
    t0v0 = ((t0v0 >> 16) & 0x0000ffff0000ffffULL);\
    t0v31 = (t0v1 & 0x0000ffff0000ffffULL);\
    t0v1 = ((t0v1 >> 16) & 0x0000ffff0000ffffULL);\
    *((uint64_t*)(&((dest[0])))) = ((t0v9 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    *((uint64_t*)(&((dest[16])))) = ((t0v9 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[1])))) = ((t0v18 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    *((uint64_t*)(&((dest[17])))) = ((t0v18 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[2])))) = ((t0v20 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    *((uint64_t*)(&((dest[18])))) = ((t0v20 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[3])))) = ((t0v22 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    *((uint64_t*)(&((dest[19])))) = ((t0v22 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[4])))) = ((t0v24 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    *((uint64_t*)(&((dest[20])))) = ((t0v24 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[5])))) = ((t0v26 & 0x00000000ffffffffULL) | (t0v27 << 32));\
    *((uint64_t*)(&((dest[21])))) = ((t0v26 >> 32) | (t0v27 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[6])))) = ((t0v28 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    *((uint64_t*)(&((dest[22])))) = ((t0v28 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[7])))) = ((t0v30 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    *((uint64_t*)(&((dest[23])))) = ((t0v30 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[8])))) = ((t0v13 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    *((uint64_t*)(&((dest[24])))) = ((t0v13 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[9])))) = ((t0v16 & 0x00000000ffffffffULL) | (t0v14 << 32));\
    *((uint64_t*)(&((dest[25])))) = ((t0v16 >> 32) | (t0v14 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[10])))) = ((t0v12 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    *((uint64_t*)(&((dest[26])))) = ((t0v12 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[11])))) = ((t0v10 & 0x00000000ffffffffULL) | (t0v8 << 32));\
    *((uint64_t*)(&((dest[27])))) = ((t0v10 >> 32) | (t0v8 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[12])))) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    *((uint64_t*)(&((dest[28])))) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[13])))) = ((t0v6 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    *((uint64_t*)(&((dest[29])))) = ((t0v6 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[14])))) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    *((uint64_t*)(&((dest[30])))) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[15])))) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    *((uint64_t*)(&((dest[31])))) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    uint64_t* dest = (uint64_t*)(D);\
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
    t0v0 = (((S0) & 0x5555555555555555ULL) | (((S1) & 0x5555555555555555ULL) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S1) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v2 = (((S2) & 0x5555555555555555ULL) | (((S3) & 0x5555555555555555ULL) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S3) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v4 = (((S4) & 0x5555555555555555ULL) | (((S5) & 0x5555555555555555ULL) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaaaaaaaaaULL) >> 1) | ((S5) & 0xaaaaaaaaaaaaaaaaULL));\
    t0v6 = ((t0v0 & 0x3333333333333333ULL) | ((t0v2 & 0x3333333333333333ULL) << 2));\
    t0v0 = (((t0v0 & 0xccccccccccccccccULL) >> 2) | (t0v2 & 0xccccccccccccccccULL));\
    t0v2 = ((t0v1 & 0x3333333333333333ULL) | ((t0v3 & 0x3333333333333333ULL) << 2));\
    t0v1 = (((t0v1 & 0xccccccccccccccccULL) >> 2) | (t0v3 & 0xccccccccccccccccULL));\
    t0v3 = (t0v4 & 0x3333333333333333ULL);\
    t0v4 = ((t0v4 & 0xccccccccccccccccULL) >> 2);\
    t0v7 = (t0v5 & 0x3333333333333333ULL);\
    t0v5 = ((t0v5 & 0xccccccccccccccccULL) >> 2);\
    t0v8 = ((t0v6 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v3 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v3 = (((t0v6 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v3 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v6 = ((t0v2 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v7 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v7 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v7 = ((t0v0 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v4 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v4 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v4 = ((t0v1 & 0x0f0f0f0f0f0f0f0fULL) | ((t0v5 & 0x0f0f0f0f0f0f0f0fULL) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0f0f0f0f0ULL) >> 4) | (t0v5 & 0xf0f0f0f0f0f0f0f0ULL));\
    t0v5 = (t0v8 & 0x000000ff000000ffULL);\
    t0v9 = ((t0v8 >> 8) & 0x000000ff000000ffULL);\
    t0v10 = ((t0v8 >> 16) & 0x000000ff000000ffULL);\
    t0v8 = ((t0v8 >> 24) & 0x000000ff000000ffULL);\
    t0v11 = (t0v6 & 0x000000ff000000ffULL);\
    t0v12 = ((t0v6 >> 8) & 0x000000ff000000ffULL);\
    t0v13 = ((t0v6 >> 16) & 0x000000ff000000ffULL);\
    t0v6 = ((t0v6 >> 24) & 0x000000ff000000ffULL);\
    t0v14 = (t0v7 & 0x000000ff000000ffULL);\
    t0v15 = ((t0v7 >> 8) & 0x000000ff000000ffULL);\
    t0v16 = ((t0v7 >> 16) & 0x000000ff000000ffULL);\
    t0v7 = ((t0v7 >> 24) & 0x000000ff000000ffULL);\
    t0v17 = (t0v4 & 0x000000ff000000ffULL);\
    t0v18 = ((t0v4 >> 8) & 0x000000ff000000ffULL);\
    t0v19 = ((t0v4 >> 16) & 0x000000ff000000ffULL);\
    t0v4 = ((t0v4 >> 24) & 0x000000ff000000ffULL);\
    t0v20 = (t0v3 & 0x000000ff000000ffULL);\
    t0v21 = ((t0v3 >> 8) & 0x000000ff000000ffULL);\
    t0v22 = ((t0v3 >> 16) & 0x000000ff000000ffULL);\
    t0v3 = ((t0v3 >> 24) & 0x000000ff000000ffULL);\
    t0v23 = (t0v2 & 0x000000ff000000ffULL);\
    t0v24 = ((t0v2 >> 8) & 0x000000ff000000ffULL);\
    t0v25 = ((t0v2 >> 16) & 0x000000ff000000ffULL);\
    t0v2 = ((t0v2 >> 24) & 0x000000ff000000ffULL);\
    t0v26 = (t0v0 & 0x000000ff000000ffULL);\
    t0v27 = ((t0v0 >> 8) & 0x000000ff000000ffULL);\
    t0v28 = ((t0v0 >> 16) & 0x000000ff000000ffULL);\
    t0v0 = ((t0v0 >> 24) & 0x000000ff000000ffULL);\
    t0v29 = (t0v1 & 0x000000ff000000ffULL);\
    t0v30 = ((t0v1 >> 8) & 0x000000ff000000ffULL);\
    t0v31 = ((t0v1 >> 16) & 0x000000ff000000ffULL);\
    t0v1 = ((t0v1 >> 24) & 0x000000ff000000ffULL);\
    *((uint64_t*)(&((dest[0])))) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    *((uint64_t*)(&((dest[16])))) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[1])))) = ((t0v14 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    *((uint64_t*)(&((dest[17])))) = ((t0v14 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[2])))) = ((t0v20 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    *((uint64_t*)(&((dest[18])))) = ((t0v20 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[3])))) = ((t0v26 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    *((uint64_t*)(&((dest[19])))) = ((t0v26 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[4])))) = ((t0v9 & 0x00000000ffffffffULL) | (t0v12 << 32));\
    *((uint64_t*)(&((dest[20])))) = ((t0v9 >> 32) | (t0v12 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[5])))) = ((t0v15 & 0x00000000ffffffffULL) | (t0v18 << 32));\
    *((uint64_t*)(&((dest[21])))) = ((t0v15 >> 32) | (t0v18 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[6])))) = ((t0v21 & 0x00000000ffffffffULL) | (t0v24 << 32));\
    *((uint64_t*)(&((dest[22])))) = ((t0v21 >> 32) | (t0v24 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[7])))) = ((t0v27 & 0x00000000ffffffffULL) | (t0v30 << 32));\
    *((uint64_t*)(&((dest[23])))) = ((t0v27 >> 32) | (t0v30 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[8])))) = ((t0v10 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    *((uint64_t*)(&((dest[24])))) = ((t0v10 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[9])))) = ((t0v16 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    *((uint64_t*)(&((dest[25])))) = ((t0v16 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[10])))) = ((t0v22 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    *((uint64_t*)(&((dest[26])))) = ((t0v22 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[11])))) = ((t0v28 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    *((uint64_t*)(&((dest[27])))) = ((t0v28 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[12])))) = ((t0v8 & 0x00000000ffffffffULL) | (t0v6 << 32));\
    *((uint64_t*)(&((dest[28])))) = ((t0v8 >> 32) | (t0v6 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[13])))) = ((t0v7 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    *((uint64_t*)(&((dest[29])))) = ((t0v7 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[14])))) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    *((uint64_t*)(&((dest[30])))) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[15])))) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    *((uint64_t*)(&((dest[31])))) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    uint64_t* dest = (uint64_t*)(D);\
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
    t0v0 = ((S0) & 0x0000000100000001ULL);\
    t0v1 = (((S0) >> 1) & 0x0000000100000001ULL);\
    t0v2 = (((S0) >> 2) & 0x0000000100000001ULL);\
    t0v3 = (((S0) >> 3) & 0x0000000100000001ULL);\
    t0v4 = (((S0) >> 4) & 0x0000000100000001ULL);\
    t0v5 = (((S0) >> 5) & 0x0000000100000001ULL);\
    t0v6 = (((S0) >> 6) & 0x0000000100000001ULL);\
    t0v7 = (((S0) >> 7) & 0x0000000100000001ULL);\
    t0v8 = (((S0) >> 8) & 0x0000000100000001ULL);\
    t0v9 = (((S0) >> 9) & 0x0000000100000001ULL);\
    t0v10 = (((S0) >> 10) & 0x0000000100000001ULL);\
    t0v11 = (((S0) >> 11) & 0x0000000100000001ULL);\
    t0v12 = (((S0) >> 12) & 0x0000000100000001ULL);\
    t0v13 = (((S0) >> 13) & 0x0000000100000001ULL);\
    t0v14 = (((S0) >> 14) & 0x0000000100000001ULL);\
    t0v15 = (((S0) >> 15) & 0x0000000100000001ULL);\
    t0v16 = (((S0) >> 16) & 0x0000000100000001ULL);\
    t0v17 = (((S0) >> 17) & 0x0000000100000001ULL);\
    t0v18 = (((S0) >> 18) & 0x0000000100000001ULL);\
    t0v19 = (((S0) >> 19) & 0x0000000100000001ULL);\
    t0v20 = (((S0) >> 20) & 0x0000000100000001ULL);\
    t0v21 = (((S0) >> 21) & 0x0000000100000001ULL);\
    t0v22 = (((S0) >> 22) & 0x0000000100000001ULL);\
    t0v23 = (((S0) >> 23) & 0x0000000100000001ULL);\
    t0v24 = (((S0) >> 24) & 0x0000000100000001ULL);\
    t0v25 = (((S0) >> 25) & 0x0000000100000001ULL);\
    t0v26 = (((S0) >> 26) & 0x0000000100000001ULL);\
    t0v27 = (((S0) >> 27) & 0x0000000100000001ULL);\
    t0v28 = (((S0) >> 28) & 0x0000000100000001ULL);\
    t0v29 = (((S0) >> 29) & 0x0000000100000001ULL);\
    t0v30 = (((S0) >> 30) & 0x0000000100000001ULL);\
    t0v31 = (((S0) >> 31) & 0x0000000100000001ULL);\
    *((uint64_t*)(&((dest[0])))) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    *((uint64_t*)(&((dest[16])))) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[1])))) = ((t0v2 & 0x00000000ffffffffULL) | (t0v3 << 32));\
    *((uint64_t*)(&((dest[17])))) = ((t0v2 >> 32) | (t0v3 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[2])))) = ((t0v4 & 0x00000000ffffffffULL) | (t0v5 << 32));\
    *((uint64_t*)(&((dest[18])))) = ((t0v4 >> 32) | (t0v5 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[3])))) = ((t0v6 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    *((uint64_t*)(&((dest[19])))) = ((t0v6 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[4])))) = ((t0v8 & 0x00000000ffffffffULL) | (t0v9 << 32));\
    *((uint64_t*)(&((dest[20])))) = ((t0v8 >> 32) | (t0v9 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[5])))) = ((t0v10 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    *((uint64_t*)(&((dest[21])))) = ((t0v10 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[6])))) = ((t0v12 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    *((uint64_t*)(&((dest[22])))) = ((t0v12 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[7])))) = ((t0v14 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    *((uint64_t*)(&((dest[23])))) = ((t0v14 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[8])))) = ((t0v16 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    *((uint64_t*)(&((dest[24])))) = ((t0v16 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[9])))) = ((t0v18 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    *((uint64_t*)(&((dest[25])))) = ((t0v18 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[10])))) = ((t0v20 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    *((uint64_t*)(&((dest[26])))) = ((t0v20 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[11])))) = ((t0v22 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    *((uint64_t*)(&((dest[27])))) = ((t0v22 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[12])))) = ((t0v24 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    *((uint64_t*)(&((dest[28])))) = ((t0v24 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[13])))) = ((t0v26 & 0x00000000ffffffffULL) | (t0v27 << 32));\
    *((uint64_t*)(&((dest[29])))) = ((t0v26 >> 32) | (t0v27 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[14])))) = ((t0v28 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    *((uint64_t*)(&((dest[30])))) = ((t0v28 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    *((uint64_t*)(&((dest[15])))) = ((t0v30 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    *((uint64_t*)(&((dest[31])))) = ((t0v30 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_MMX.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    __m64* dest = (__m64*)(D);\
    const __m64 c0 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*5));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*6));\
    const __m64 c7 = (*(const __m64*)(transform_const_tbl + 2*7));\
    const __m64 c8 = (*(const __m64*)(transform_const_tbl + 2*8));\
    const __m64 c9 = (*(const __m64*)(transform_const_tbl + 2*9));\
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
    t0v0 = _m_por(_m_pand((S0), c0), _m_psllwi(_m_pand((S1), c0), 1));\
    t0v1 = _m_por(_m_psrlwi(_m_pand((S0), c1), 1), _m_pand((S1), c1));\
    t0v2 = _m_por(_m_pand((S2), c0), _m_psllwi(_m_pand((S3), c0), 1));\
    t0v3 = _m_por(_m_psrlwi(_m_pand((S2), c1), 1), _m_pand((S3), c1));\
    t0v4 = _m_por(_m_pand((S4), c0), _m_psllwi(_m_pand((S5), c0), 1));\
    t0v5 = _m_por(_m_psrlwi(_m_pand((S4), c1), 1), _m_pand((S5), c1));\
    t0v6 = _m_por(_m_pand((S6), c0), _m_psllwi(_m_pand((S7), c0), 1));\
    t0v7 = _m_por(_m_psrlwi(_m_pand((S6), c1), 1), _m_pand((S7), c1));\
    t0v8 = _m_por(_m_pand((S8), c0), _m_psllwi(_m_pand((S9), c0), 1));\
    t0v9 = _m_por(_m_psrlwi(_m_pand((S8), c1), 1), _m_pand((S9), c1));\
    t0v10 = _m_por(_m_pand((S10), c0), _m_psllwi(_m_pand((S11), c0), 1));\
    t0v11 = _m_por(_m_psrlwi(_m_pand((S10), c1), 1), _m_pand((S11), c1));\
    t0v12 = _m_por(_m_pand((S12), c0), _m_psllwi(_m_pand((S13), c0), 1));\
    t0v13 = _m_por(_m_psrlwi(_m_pand((S12), c1), 1), _m_pand((S13), c1));\
    t0v14 = _m_por(_m_pand((S14), c0), _m_psllwi(_m_pand((S15), c0), 1));\
    t0v15 = _m_por(_m_psrlwi(_m_pand((S14), c1), 1), _m_pand((S15), c1));\
    t0v16 = _m_por(_m_pand((S16), c0), _m_psllwi(_m_pand((S17), c0), 1));\
    t0v17 = _m_por(_m_psrlwi(_m_pand((S16), c1), 1), _m_pand((S17), c1));\
    t0v18 = _m_por(_m_pand((S18), c0), _m_psllwi(_m_pand((S19), c0), 1));\
    t0v19 = _m_por(_m_psrlwi(_m_pand((S18), c1), 1), _m_pand((S19), c1));\
    t0v20 = _m_por(_m_pand((S20), c0), _m_psllwi(_m_pand((S21), c0), 1));\
    t0v21 = _m_por(_m_psrlwi(_m_pand((S20), c1), 1), _m_pand((S21), c1));\
    t0v22 = _m_por(_m_pand((S22), c0), _m_psllwi(_m_pand((S23), c0), 1));\
    t0v23 = _m_por(_m_psrlwi(_m_pand((S22), c1), 1), _m_pand((S23), c1));\
    t0v24 = _m_por(_m_pand((S24), c0), _m_psllwi(_m_pand((S25), c0), 1));\
    t0v25 = _m_por(_m_psrlwi(_m_pand((S24), c1), 1), _m_pand((S25), c1));\
    t0v26 = _m_por(_m_pand((S26), c0), _m_psllwi(_m_pand((S27), c0), 1));\
    t0v27 = _m_por(_m_psrlwi(_m_pand((S26), c1), 1), _m_pand((S27), c1));\
    t0v28 = _m_por(_m_pand((S28), c0), _m_psllwi(_m_pand((S29), c0), 1));\
    t0v29 = _m_por(_m_psrlwi(_m_pand((S28), c1), 1), _m_pand((S29), c1));\
    t0v30 = _m_por(_m_pand((S30), c0), _m_psllwi(_m_pand((S31), c0), 1));\
    t0v31 = _m_por(_m_psrlwi(_m_pand((S30), c1), 1), _m_pand((S31), c1));\
    t0v32 = _m_por(_m_pand(t0v0, c2), _m_psllwi(_m_pand(t0v2, c2), 2));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c3), 2), _m_pand(t0v2, c3));\
    t0v2 = _m_por(_m_pand(t0v1, c2), _m_psllwi(_m_pand(t0v3, c2), 2));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c3), 2), _m_pand(t0v3, c3));\
    t0v3 = _m_por(_m_pand(t0v4, c2), _m_psllwi(_m_pand(t0v6, c2), 2));\
    t0v4 = _m_por(_m_psrlwi(_m_pand(t0v4, c3), 2), _m_pand(t0v6, c3));\
    t0v6 = _m_por(_m_pand(t0v5, c2), _m_psllwi(_m_pand(t0v7, c2), 2));\
    t0v5 = _m_por(_m_psrlwi(_m_pand(t0v5, c3), 2), _m_pand(t0v7, c3));\
    t0v7 = _m_por(_m_pand(t0v8, c2), _m_psllwi(_m_pand(t0v10, c2), 2));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c3), 2), _m_pand(t0v10, c3));\
    t0v10 = _m_por(_m_pand(t0v9, c2), _m_psllwi(_m_pand(t0v11, c2), 2));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c3), 2), _m_pand(t0v11, c3));\
    t0v11 = _m_por(_m_pand(t0v12, c2), _m_psllwi(_m_pand(t0v14, c2), 2));\
    t0v12 = _m_por(_m_psrlwi(_m_pand(t0v12, c3), 2), _m_pand(t0v14, c3));\
    t0v14 = _m_por(_m_pand(t0v13, c2), _m_psllwi(_m_pand(t0v15, c2), 2));\
    t0v13 = _m_por(_m_psrlwi(_m_pand(t0v13, c3), 2), _m_pand(t0v15, c3));\
    t0v15 = _m_por(_m_pand(t0v16, c2), _m_psllwi(_m_pand(t0v18, c2), 2));\
    t0v16 = _m_por(_m_psrlwi(_m_pand(t0v16, c3), 2), _m_pand(t0v18, c3));\
    t0v18 = _m_por(_m_pand(t0v17, c2), _m_psllwi(_m_pand(t0v19, c2), 2));\
    t0v17 = _m_por(_m_psrlwi(_m_pand(t0v17, c3), 2), _m_pand(t0v19, c3));\
    t0v19 = _m_por(_m_pand(t0v20, c2), _m_psllwi(_m_pand(t0v22, c2), 2));\
    t0v20 = _m_por(_m_psrlwi(_m_pand(t0v20, c3), 2), _m_pand(t0v22, c3));\
    t0v22 = _m_por(_m_pand(t0v21, c2), _m_psllwi(_m_pand(t0v23, c2), 2));\
    t0v21 = _m_por(_m_psrlwi(_m_pand(t0v21, c3), 2), _m_pand(t0v23, c3));\
    t0v23 = _m_por(_m_pand(t0v24, c2), _m_psllwi(_m_pand(t0v26, c2), 2));\
    t0v24 = _m_por(_m_psrlwi(_m_pand(t0v24, c3), 2), _m_pand(t0v26, c3));\
    t0v26 = _m_por(_m_pand(t0v25, c2), _m_psllwi(_m_pand(t0v27, c2), 2));\
    t0v25 = _m_por(_m_psrlwi(_m_pand(t0v25, c3), 2), _m_pand(t0v27, c3));\
    t0v27 = _m_por(_m_pand(t0v28, c2), _m_psllwi(_m_pand(t0v30, c2), 2));\
    t0v28 = _m_por(_m_psrlwi(_m_pand(t0v28, c3), 2), _m_pand(t0v30, c3));\
    t0v30 = _m_por(_m_pand(t0v29, c2), _m_psllwi(_m_pand(t0v31, c2), 2));\
    t0v29 = _m_por(_m_psrlwi(_m_pand(t0v29, c3), 2), _m_pand(t0v31, c3));\
    t0v31 = _m_por(_m_pand(t0v32, c4), _m_psllwi(_m_pand(t0v3, c4), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v32, c5), 4), _m_pand(t0v3, c5));\
    t0v32 = _m_por(_m_pand(t0v2, c4), _m_psllwi(_m_pand(t0v6, c4), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c5), 4), _m_pand(t0v6, c5));\
    t0v6 = _m_por(_m_pand(t0v0, c4), _m_psllwi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_psllwi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_por(_m_pand(t0v7, c4), _m_psllwi(_m_pand(t0v11, c4), 4));\
    t0v7 = _m_por(_m_psrlwi(_m_pand(t0v7, c5), 4), _m_pand(t0v11, c5));\
    t0v11 = _m_por(_m_pand(t0v10, c4), _m_psllwi(_m_pand(t0v14, c4), 4));\
    t0v10 = _m_por(_m_psrlwi(_m_pand(t0v10, c5), 4), _m_pand(t0v14, c5));\
    t0v14 = _m_por(_m_pand(t0v8, c4), _m_psllwi(_m_pand(t0v12, c4), 4));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c5), 4), _m_pand(t0v12, c5));\
    t0v12 = _m_por(_m_pand(t0v9, c4), _m_psllwi(_m_pand(t0v13, c4), 4));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c5), 4), _m_pand(t0v13, c5));\
    t0v13 = _m_por(_m_pand(t0v15, c4), _m_psllwi(_m_pand(t0v19, c4), 4));\
    t0v15 = _m_por(_m_psrlwi(_m_pand(t0v15, c5), 4), _m_pand(t0v19, c5));\
    t0v19 = _m_por(_m_pand(t0v18, c4), _m_psllwi(_m_pand(t0v22, c4), 4));\
    t0v18 = _m_por(_m_psrlwi(_m_pand(t0v18, c5), 4), _m_pand(t0v22, c5));\
    t0v22 = _m_por(_m_pand(t0v16, c4), _m_psllwi(_m_pand(t0v20, c4), 4));\
    t0v16 = _m_por(_m_psrlwi(_m_pand(t0v16, c5), 4), _m_pand(t0v20, c5));\
    t0v20 = _m_por(_m_pand(t0v17, c4), _m_psllwi(_m_pand(t0v21, c4), 4));\
    t0v17 = _m_por(_m_psrlwi(_m_pand(t0v17, c5), 4), _m_pand(t0v21, c5));\
    t0v21 = _m_por(_m_pand(t0v23, c4), _m_psllwi(_m_pand(t0v27, c4), 4));\
    t0v23 = _m_por(_m_psrlwi(_m_pand(t0v23, c5), 4), _m_pand(t0v27, c5));\
    t0v27 = _m_por(_m_pand(t0v26, c4), _m_psllwi(_m_pand(t0v30, c4), 4));\
    t0v26 = _m_por(_m_psrlwi(_m_pand(t0v26, c5), 4), _m_pand(t0v30, c5));\
    t0v30 = _m_por(_m_pand(t0v24, c4), _m_psllwi(_m_pand(t0v28, c4), 4));\
    t0v24 = _m_por(_m_psrlwi(_m_pand(t0v24, c5), 4), _m_pand(t0v28, c5));\
    t0v28 = _m_por(_m_pand(t0v25, c4), _m_psllwi(_m_pand(t0v29, c4), 4));\
    t0v25 = _m_por(_m_psrlwi(_m_pand(t0v25, c5), 4), _m_pand(t0v29, c5));\
    t0v29 = _m_por(_m_pand(t0v31, c6), _m_psllwi(t0v5, 8));\
    t0v5 = _m_por(_m_psrlwi(t0v31, 8), _m_pand(t0v5, c7));\
    t0v31 = _m_por(_m_pand(t0v32, c6), _m_psllwi(t0v11, 8));\
    t0v11 = _m_por(_m_psrlwi(t0v32, 8), _m_pand(t0v11, c7));\
    t0v32 = _m_por(_m_pand(t0v6, c6), _m_psllwi(t0v14, 8));\
    t0v6 = _m_por(_m_psrlwi(t0v6, 8), _m_pand(t0v14, c7));\
    t0v14 = _m_por(_m_pand(t0v4, c6), _m_psllwi(t0v12, 8));\
    t0v4 = _m_por(_m_psrlwi(t0v4, 8), _m_pand(t0v12, c7));\
    t0v12 = _m_por(_m_pand(t0v3, c6), _m_psllwi(t0v7, 8));\
    t0v3 = _m_por(_m_psrlwi(t0v3, 8), _m_pand(t0v7, c7));\
    t0v7 = _m_por(_m_pand(t0v2, c6), _m_psllwi(t0v10, 8));\
    t0v2 = _m_por(_m_psrlwi(t0v2, 8), _m_pand(t0v10, c7));\
    t0v10 = _m_por(_m_pand(t0v0, c6), _m_psllwi(t0v8, 8));\
    t0v0 = _m_por(_m_psrlwi(t0v0, 8), _m_pand(t0v8, c7));\
    t0v8 = _m_por(_m_pand(t0v1, c6), _m_psllwi(t0v9, 8));\
    t0v1 = _m_por(_m_psrlwi(t0v1, 8), _m_pand(t0v9, c7));\
    t0v9 = _m_por(_m_pand(t0v13, c6), _m_psllwi(t0v21, 8));\
    t0v13 = _m_por(_m_psrlwi(t0v13, 8), _m_pand(t0v21, c7));\
    t0v21 = _m_por(_m_pand(t0v19, c6), _m_psllwi(t0v27, 8));\
    t0v19 = _m_por(_m_psrlwi(t0v19, 8), _m_pand(t0v27, c7));\
    t0v27 = _m_por(_m_pand(t0v22, c6), _m_psllwi(t0v30, 8));\
    t0v22 = _m_por(_m_psrlwi(t0v22, 8), _m_pand(t0v30, c7));\
    t0v30 = _m_por(_m_pand(t0v20, c6), _m_psllwi(t0v28, 8));\
    t0v20 = _m_por(_m_psrlwi(t0v20, 8), _m_pand(t0v28, c7));\
    t0v28 = _m_por(_m_pand(t0v15, c6), _m_psllwi(t0v23, 8));\
    t0v15 = _m_por(_m_psrlwi(t0v15, 8), _m_pand(t0v23, c7));\
    t0v23 = _m_por(_m_pand(t0v18, c6), _m_psllwi(t0v26, 8));\
    t0v18 = _m_por(_m_psrlwi(t0v18, 8), _m_pand(t0v26, c7));\
    t0v26 = _m_por(_m_pand(t0v16, c6), _m_psllwi(t0v24, 8));\
    t0v16 = _m_por(_m_psrlwi(t0v16, 8), _m_pand(t0v24, c7));\
    t0v24 = _m_por(_m_pand(t0v17, c6), _m_psllwi(t0v25, 8));\
    t0v17 = _m_por(_m_psrlwi(t0v17, 8), _m_pand(t0v25, c7));\
    t0v25 = _m_por(_m_pand(t0v29, c8), _m_pslldi(t0v9, 16));\
    t0v9 = _m_por(_m_psrldi(t0v29, 16), _m_pand(t0v9, c9));\
    t0v29 = _m_por(_m_pand(t0v31, c8), _m_pslldi(t0v21, 16));\
    t0v21 = _m_por(_m_psrldi(t0v31, 16), _m_pand(t0v21, c9));\
    t0v31 = _m_por(_m_pand(t0v32, c8), _m_pslldi(t0v27, 16));\
    t0v27 = _m_por(_m_psrldi(t0v32, 16), _m_pand(t0v27, c9));\
    t0v32 = _m_por(_m_pand(t0v14, c8), _m_pslldi(t0v30, 16));\
    t0v14 = _m_por(_m_psrldi(t0v14, 16), _m_pand(t0v30, c9));\
    t0v30 = _m_por(_m_pand(t0v12, c8), _m_pslldi(t0v28, 16));\
    t0v12 = _m_por(_m_psrldi(t0v12, 16), _m_pand(t0v28, c9));\
    t0v28 = _m_por(_m_pand(t0v7, c8), _m_pslldi(t0v23, 16));\
    t0v7 = _m_por(_m_psrldi(t0v7, 16), _m_pand(t0v23, c9));\
    t0v23 = _m_por(_m_pand(t0v10, c8), _m_pslldi(t0v26, 16));\
    t0v10 = _m_por(_m_psrldi(t0v10, 16), _m_pand(t0v26, c9));\
    t0v26 = _m_por(_m_pand(t0v8, c8), _m_pslldi(t0v24, 16));\
    t0v8 = _m_por(_m_psrldi(t0v8, 16), _m_pand(t0v24, c9));\
    t0v24 = _m_por(_m_pand(t0v5, c8), _m_pslldi(t0v13, 16));\
    t0v5 = _m_por(_m_psrldi(t0v5, 16), _m_pand(t0v13, c9));\
    t0v13 = _m_por(_m_pand(t0v11, c8), _m_pslldi(t0v19, 16));\
    t0v11 = _m_por(_m_psrldi(t0v11, 16), _m_pand(t0v19, c9));\
    t0v19 = _m_por(_m_pand(t0v6, c8), _m_pslldi(t0v22, 16));\
    t0v6 = _m_por(_m_psrldi(t0v6, 16), _m_pand(t0v22, c9));\
    t0v22 = _m_por(_m_pand(t0v4, c8), _m_pslldi(t0v20, 16));\
    t0v4 = _m_por(_m_psrldi(t0v4, 16), _m_pand(t0v20, c9));\
    t0v20 = _m_por(_m_pand(t0v3, c8), _m_pslldi(t0v15, 16));\
    t0v3 = _m_por(_m_psrldi(t0v3, 16), _m_pand(t0v15, c9));\
    t0v15 = _m_por(_m_pand(t0v2, c8), _m_pslldi(t0v18, 16));\
    t0v2 = _m_por(_m_psrldi(t0v2, 16), _m_pand(t0v18, c9));\
    t0v18 = _m_por(_m_pand(t0v0, c8), _m_pslldi(t0v16, 16));\
    t0v0 = _m_por(_m_psrldi(t0v0, 16), _m_pand(t0v16, c9));\
    t0v16 = _m_por(_m_pand(t0v1, c8), _m_pslldi(t0v17, 16));\
    t0v1 = _m_por(_m_psrldi(t0v1, 16), _m_pand(t0v17, c9));\
    *((__m64*)(&((dest[0])))) = _m_punpckldq(t0v25, t0v29);\
    *((__m64*)(&((dest[16])))) = _m_punpckhdq(t0v25, t0v29);\
    *((__m64*)(&((dest[1])))) = _m_punpckldq(t0v31, t0v32);\
    *((__m64*)(&((dest[17])))) = _m_punpckhdq(t0v31, t0v32);\
    *((__m64*)(&((dest[2])))) = _m_punpckldq(t0v30, t0v28);\
    *((__m64*)(&((dest[18])))) = _m_punpckhdq(t0v30, t0v28);\
    *((__m64*)(&((dest[3])))) = _m_punpckldq(t0v23, t0v26);\
    *((__m64*)(&((dest[19])))) = _m_punpckhdq(t0v23, t0v26);\
    *((__m64*)(&((dest[4])))) = _m_punpckldq(t0v24, t0v13);\
    *((__m64*)(&((dest[20])))) = _m_punpckhdq(t0v24, t0v13);\
    *((__m64*)(&((dest[5])))) = _m_punpckldq(t0v19, t0v22);\
    *((__m64*)(&((dest[21])))) = _m_punpckhdq(t0v19, t0v22);\
    *((__m64*)(&((dest[6])))) = _m_punpckldq(t0v20, t0v15);\
    *((__m64*)(&((dest[22])))) = _m_punpckhdq(t0v20, t0v15);\
    *((__m64*)(&((dest[7])))) = _m_punpckldq(t0v18, t0v16);\
    *((__m64*)(&((dest[23])))) = _m_punpckhdq(t0v18, t0v16);\
    *((__m64*)(&((dest[8])))) = _m_punpckldq(t0v9, t0v21);\
    *((__m64*)(&((dest[24])))) = _m_punpckhdq(t0v9, t0v21);\
    *((__m64*)(&((dest[9])))) = _m_punpckldq(t0v27, t0v14);\
    *((__m64*)(&((dest[25])))) = _m_punpckhdq(t0v27, t0v14);\
    *((__m64*)(&((dest[10])))) = _m_punpckldq(t0v12, t0v7);\
    *((__m64*)(&((dest[26])))) = _m_punpckhdq(t0v12, t0v7);\
    *((__m64*)(&((dest[11])))) = _m_punpckldq(t0v10, t0v8);\
    *((__m64*)(&((dest[27])))) = _m_punpckhdq(t0v10, t0v8);\
    *((__m64*)(&((dest[12])))) = _m_punpckldq(t0v5, t0v11);\
    *((__m64*)(&((dest[28])))) = _m_punpckhdq(t0v5, t0v11);\
    *((__m64*)(&((dest[13])))) = _m_punpckldq(t0v6, t0v4);\
    *((__m64*)(&((dest[29])))) = _m_punpckhdq(t0v6, t0v4);\
    *((__m64*)(&((dest[14])))) = _m_punpckldq(t0v3, t0v2);\
    *((__m64*)(&((dest[30])))) = _m_punpckhdq(t0v3, t0v2);\
    *((__m64*)(&((dest[15])))) = _m_punpckldq(t0v0, t0v1);\
    *((__m64*)(&((dest[31])))) = _m_punpckhdq(t0v0, t0v1);\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    __m64* dest = (__m64*)(D);\
    const __m64 c8 = (*(const __m64*)(transform_const2_tbl + 2*4));\
    const __m64 c0 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*5));\
    const __m64 c6 = (*(const __m64*)(transform_const_tbl + 2*6));\
    const __m64 c7 = (*(const __m64*)(transform_const_tbl + 2*7));\
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
    t0v0 = _m_por(_m_pand((S0), c0), _m_psllwi(_m_pand((S1), c0), 1));\
    t0v1 = _m_por(_m_psrlwi(_m_pand((S0), c1), 1), _m_pand((S1), c1));\
    t0v2 = _m_por(_m_pand((S2), c0), _m_psllwi(_m_pand((S3), c0), 1));\
    t0v3 = _m_por(_m_psrlwi(_m_pand((S2), c1), 1), _m_pand((S3), c1));\
    t0v4 = _m_por(_m_pand((S4), c0), _m_psllwi(_m_pand((S5), c0), 1));\
    t0v5 = _m_por(_m_psrlwi(_m_pand((S4), c1), 1), _m_pand((S5), c1));\
    t0v6 = _m_por(_m_pand((S6), c0), _m_psllwi(_m_pand((S7), c0), 1));\
    t0v7 = _m_por(_m_psrlwi(_m_pand((S6), c1), 1), _m_pand((S7), c1));\
    t0v8 = _m_por(_m_pand((S8), c0), _m_psllwi(_m_pand((S9), c0), 1));\
    t0v9 = _m_por(_m_psrlwi(_m_pand((S8), c1), 1), _m_pand((S9), c1));\
    t0v10 = _m_por(_m_pand((S10), c0), _m_psllwi(_m_pand((S11), c0), 1));\
    t0v11 = _m_por(_m_psrlwi(_m_pand((S10), c1), 1), _m_pand((S11), c1));\
    t0v12 = _m_por(_m_pand((S12), c0), _m_psllwi(_m_pand((S13), c0), 1));\
    t0v13 = _m_por(_m_psrlwi(_m_pand((S12), c1), 1), _m_pand((S13), c1));\
    t0v14 = _m_por(_m_pand((S14), c0), _m_psllwi(_m_pand((S15), c0), 1));\
    t0v15 = _m_por(_m_psrlwi(_m_pand((S14), c1), 1), _m_pand((S15), c1));\
    t0v16 = _m_por(_m_pand(t0v0, c2), _m_psllwi(_m_pand(t0v2, c2), 2));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c3), 2), _m_pand(t0v2, c3));\
    t0v2 = _m_por(_m_pand(t0v1, c2), _m_psllwi(_m_pand(t0v3, c2), 2));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c3), 2), _m_pand(t0v3, c3));\
    t0v3 = _m_por(_m_pand(t0v4, c2), _m_psllwi(_m_pand(t0v6, c2), 2));\
    t0v4 = _m_por(_m_psrlwi(_m_pand(t0v4, c3), 2), _m_pand(t0v6, c3));\
    t0v6 = _m_por(_m_pand(t0v5, c2), _m_psllwi(_m_pand(t0v7, c2), 2));\
    t0v5 = _m_por(_m_psrlwi(_m_pand(t0v5, c3), 2), _m_pand(t0v7, c3));\
    t0v7 = _m_por(_m_pand(t0v8, c2), _m_psllwi(_m_pand(t0v10, c2), 2));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c3), 2), _m_pand(t0v10, c3));\
    t0v10 = _m_por(_m_pand(t0v9, c2), _m_psllwi(_m_pand(t0v11, c2), 2));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c3), 2), _m_pand(t0v11, c3));\
    t0v11 = _m_por(_m_pand(t0v12, c2), _m_psllwi(_m_pand(t0v14, c2), 2));\
    t0v12 = _m_por(_m_psrlwi(_m_pand(t0v12, c3), 2), _m_pand(t0v14, c3));\
    t0v14 = _m_por(_m_pand(t0v13, c2), _m_psllwi(_m_pand(t0v15, c2), 2));\
    t0v13 = _m_por(_m_psrlwi(_m_pand(t0v13, c3), 2), _m_pand(t0v15, c3));\
    t0v15 = _m_por(_m_pand(t0v16, c4), _m_psllwi(_m_pand(t0v3, c4), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v16, c5), 4), _m_pand(t0v3, c5));\
    t0v16 = _m_por(_m_pand(t0v2, c4), _m_psllwi(_m_pand(t0v6, c4), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c5), 4), _m_pand(t0v6, c5));\
    t0v6 = _m_por(_m_pand(t0v0, c4), _m_psllwi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_psllwi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_por(_m_pand(t0v7, c4), _m_psllwi(_m_pand(t0v11, c4), 4));\
    t0v7 = _m_por(_m_psrlwi(_m_pand(t0v7, c5), 4), _m_pand(t0v11, c5));\
    t0v11 = _m_por(_m_pand(t0v10, c4), _m_psllwi(_m_pand(t0v14, c4), 4));\
    t0v10 = _m_por(_m_psrlwi(_m_pand(t0v10, c5), 4), _m_pand(t0v14, c5));\
    t0v14 = _m_por(_m_pand(t0v8, c4), _m_psllwi(_m_pand(t0v12, c4), 4));\
    t0v8 = _m_por(_m_psrlwi(_m_pand(t0v8, c5), 4), _m_pand(t0v12, c5));\
    t0v12 = _m_por(_m_pand(t0v9, c4), _m_psllwi(_m_pand(t0v13, c4), 4));\
    t0v9 = _m_por(_m_psrlwi(_m_pand(t0v9, c5), 4), _m_pand(t0v13, c5));\
    t0v13 = _m_por(_m_pand(t0v15, c6), _m_psllwi(t0v5, 8));\
    t0v5 = _m_por(_m_psrlwi(t0v15, 8), _m_pand(t0v5, c7));\
    t0v15 = _m_por(_m_pand(t0v16, c6), _m_psllwi(t0v11, 8));\
    t0v11 = _m_por(_m_psrlwi(t0v16, 8), _m_pand(t0v11, c7));\
    t0v16 = _m_por(_m_pand(t0v6, c6), _m_psllwi(t0v14, 8));\
    t0v6 = _m_por(_m_psrlwi(t0v6, 8), _m_pand(t0v14, c7));\
    t0v14 = _m_por(_m_pand(t0v4, c6), _m_psllwi(t0v12, 8));\
    t0v4 = _m_por(_m_psrlwi(t0v4, 8), _m_pand(t0v12, c7));\
    t0v12 = _m_por(_m_pand(t0v3, c6), _m_psllwi(t0v7, 8));\
    t0v3 = _m_por(_m_psrlwi(t0v3, 8), _m_pand(t0v7, c7));\
    t0v7 = _m_por(_m_pand(t0v2, c6), _m_psllwi(t0v10, 8));\
    t0v2 = _m_por(_m_psrlwi(t0v2, 8), _m_pand(t0v10, c7));\
    t0v10 = _m_por(_m_pand(t0v0, c6), _m_psllwi(t0v8, 8));\
    t0v0 = _m_por(_m_psrlwi(t0v0, 8), _m_pand(t0v8, c7));\
    t0v8 = _m_por(_m_pand(t0v1, c6), _m_psllwi(t0v9, 8));\
    t0v1 = _m_por(_m_psrlwi(t0v1, 8), _m_pand(t0v9, c7));\
    t0v9 = _m_pand(t0v13, c8);\
    t0v13 = _m_psrldi(t0v13, 16);\
    t0v17 = _m_pand(t0v15, c8);\
    t0v15 = _m_psrldi(t0v15, 16);\
    t0v18 = _m_pand(t0v16, c8);\
    t0v16 = _m_psrldi(t0v16, 16);\
    t0v19 = _m_pand(t0v14, c8);\
    t0v14 = _m_psrldi(t0v14, 16);\
    t0v20 = _m_pand(t0v12, c8);\
    t0v12 = _m_psrldi(t0v12, 16);\
    t0v21 = _m_pand(t0v7, c8);\
    t0v7 = _m_psrldi(t0v7, 16);\
    t0v22 = _m_pand(t0v10, c8);\
    t0v10 = _m_psrldi(t0v10, 16);\
    t0v23 = _m_pand(t0v8, c8);\
    t0v8 = _m_psrldi(t0v8, 16);\
    t0v24 = _m_pand(t0v5, c8);\
    t0v5 = _m_psrldi(t0v5, 16);\
    t0v25 = _m_pand(t0v11, c8);\
    t0v11 = _m_psrldi(t0v11, 16);\
    t0v26 = _m_pand(t0v6, c8);\
    t0v6 = _m_psrldi(t0v6, 16);\
    t0v27 = _m_pand(t0v4, c8);\
    t0v4 = _m_psrldi(t0v4, 16);\
    t0v28 = _m_pand(t0v3, c8);\
    t0v3 = _m_psrldi(t0v3, 16);\
    t0v29 = _m_pand(t0v2, c8);\
    t0v2 = _m_psrldi(t0v2, 16);\
    t0v30 = _m_pand(t0v0, c8);\
    t0v0 = _m_psrldi(t0v0, 16);\
    t0v31 = _m_pand(t0v1, c8);\
    t0v1 = _m_psrldi(t0v1, 16);\
    *((__m64*)(&((dest[0])))) = _m_punpckldq(t0v9, t0v17);\
    *((__m64*)(&((dest[16])))) = _m_punpckhdq(t0v9, t0v17);\
    *((__m64*)(&((dest[1])))) = _m_punpckldq(t0v18, t0v19);\
    *((__m64*)(&((dest[17])))) = _m_punpckhdq(t0v18, t0v19);\
    *((__m64*)(&((dest[2])))) = _m_punpckldq(t0v20, t0v21);\
    *((__m64*)(&((dest[18])))) = _m_punpckhdq(t0v20, t0v21);\
    *((__m64*)(&((dest[3])))) = _m_punpckldq(t0v22, t0v23);\
    *((__m64*)(&((dest[19])))) = _m_punpckhdq(t0v22, t0v23);\
    *((__m64*)(&((dest[4])))) = _m_punpckldq(t0v24, t0v25);\
    *((__m64*)(&((dest[20])))) = _m_punpckhdq(t0v24, t0v25);\
    *((__m64*)(&((dest[5])))) = _m_punpckldq(t0v26, t0v27);\
    *((__m64*)(&((dest[21])))) = _m_punpckhdq(t0v26, t0v27);\
    *((__m64*)(&((dest[6])))) = _m_punpckldq(t0v28, t0v29);\
    *((__m64*)(&((dest[22])))) = _m_punpckhdq(t0v28, t0v29);\
    *((__m64*)(&((dest[7])))) = _m_punpckldq(t0v30, t0v31);\
    *((__m64*)(&((dest[23])))) = _m_punpckhdq(t0v30, t0v31);\
    *((__m64*)(&((dest[8])))) = _m_punpckldq(t0v13, t0v15);\
    *((__m64*)(&((dest[24])))) = _m_punpckhdq(t0v13, t0v15);\
    *((__m64*)(&((dest[9])))) = _m_punpckldq(t0v16, t0v14);\
    *((__m64*)(&((dest[25])))) = _m_punpckhdq(t0v16, t0v14);\
    *((__m64*)(&((dest[10])))) = _m_punpckldq(t0v12, t0v7);\
    *((__m64*)(&((dest[26])))) = _m_punpckhdq(t0v12, t0v7);\
    *((__m64*)(&((dest[11])))) = _m_punpckldq(t0v10, t0v8);\
    *((__m64*)(&((dest[27])))) = _m_punpckhdq(t0v10, t0v8);\
    *((__m64*)(&((dest[12])))) = _m_punpckldq(t0v5, t0v11);\
    *((__m64*)(&((dest[28])))) = _m_punpckhdq(t0v5, t0v11);\
    *((__m64*)(&((dest[13])))) = _m_punpckldq(t0v6, t0v4);\
    *((__m64*)(&((dest[29])))) = _m_punpckhdq(t0v6, t0v4);\
    *((__m64*)(&((dest[14])))) = _m_punpckldq(t0v3, t0v2);\
    *((__m64*)(&((dest[30])))) = _m_punpckhdq(t0v3, t0v2);\
    *((__m64*)(&((dest[15])))) = _m_punpckldq(t0v0, t0v1);\
    *((__m64*)(&((dest[31])))) = _m_punpckhdq(t0v0, t0v1);\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    __m64* dest = (__m64*)(D);\
    const __m64 c6 = (*(const __m64*)(transform_const2_tbl + 2*3));\
    const __m64 c0 = (*(const __m64*)(transform_const_tbl + 2*0));\
    const __m64 c1 = (*(const __m64*)(transform_const_tbl + 2*1));\
    const __m64 c2 = (*(const __m64*)(transform_const_tbl + 2*2));\
    const __m64 c3 = (*(const __m64*)(transform_const_tbl + 2*3));\
    const __m64 c4 = (*(const __m64*)(transform_const_tbl + 2*4));\
    const __m64 c5 = (*(const __m64*)(transform_const_tbl + 2*5));\
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
    t0v0 = _m_por(_m_pand((S0), c0), _m_psllwi(_m_pand((S1), c0), 1));\
    t0v1 = _m_por(_m_psrlwi(_m_pand((S0), c1), 1), _m_pand((S1), c1));\
    t0v2 = _m_por(_m_pand((S2), c0), _m_psllwi(_m_pand((S3), c0), 1));\
    t0v3 = _m_por(_m_psrlwi(_m_pand((S2), c1), 1), _m_pand((S3), c1));\
    t0v4 = _m_por(_m_pand((S4), c0), _m_psllwi(_m_pand((S5), c0), 1));\
    t0v5 = _m_por(_m_psrlwi(_m_pand((S4), c1), 1), _m_pand((S5), c1));\
    t0v6 = _m_por(_m_pand(t0v0, c2), _m_psllwi(_m_pand(t0v2, c2), 2));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c3), 2), _m_pand(t0v2, c3));\
    t0v2 = _m_por(_m_pand(t0v1, c2), _m_psllwi(_m_pand(t0v3, c2), 2));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c3), 2), _m_pand(t0v3, c3));\
    t0v3 = _m_pand(t0v4, c2);\
    t0v4 = _m_psrlwi(_m_pand(t0v4, c3), 2);\
    t0v7 = _m_pand(t0v5, c2);\
    t0v5 = _m_psrlwi(_m_pand(t0v5, c3), 2);\
    t0v8 = _m_por(_m_pand(t0v6, c4), _m_psllwi(_m_pand(t0v3, c4), 4));\
    t0v3 = _m_por(_m_psrlwi(_m_pand(t0v6, c5), 4), _m_pand(t0v3, c5));\
    t0v6 = _m_por(_m_pand(t0v2, c4), _m_psllwi(_m_pand(t0v7, c4), 4));\
    t0v2 = _m_por(_m_psrlwi(_m_pand(t0v2, c5), 4), _m_pand(t0v7, c5));\
    t0v7 = _m_por(_m_pand(t0v0, c4), _m_psllwi(_m_pand(t0v4, c4), 4));\
    t0v0 = _m_por(_m_psrlwi(_m_pand(t0v0, c5), 4), _m_pand(t0v4, c5));\
    t0v4 = _m_por(_m_pand(t0v1, c4), _m_psllwi(_m_pand(t0v5, c4), 4));\
    t0v1 = _m_por(_m_psrlwi(_m_pand(t0v1, c5), 4), _m_pand(t0v5, c5));\
    t0v5 = _m_pand(t0v8, c6);\
    t0v9 = _m_pand(_m_psrldi(t0v8, 8), c6);\
    t0v10 = _m_pand(_m_psrldi(t0v8, 16), c6);\
    t0v8 = _m_psrldi(t0v8, 24);\
    t0v11 = _m_pand(t0v6, c6);\
    t0v12 = _m_pand(_m_psrldi(t0v6, 8), c6);\
    t0v13 = _m_pand(_m_psrldi(t0v6, 16), c6);\
    t0v6 = _m_psrldi(t0v6, 24);\
    t0v14 = _m_pand(t0v7, c6);\
    t0v15 = _m_pand(_m_psrldi(t0v7, 8), c6);\
    t0v16 = _m_pand(_m_psrldi(t0v7, 16), c6);\
    t0v7 = _m_psrldi(t0v7, 24);\
    t0v17 = _m_pand(t0v4, c6);\
    t0v18 = _m_pand(_m_psrldi(t0v4, 8), c6);\
    t0v19 = _m_pand(_m_psrldi(t0v4, 16), c6);\
    t0v4 = _m_psrldi(t0v4, 24);\
    t0v20 = _m_pand(t0v3, c6);\
    t0v21 = _m_pand(_m_psrldi(t0v3, 8), c6);\
    t0v22 = _m_pand(_m_psrldi(t0v3, 16), c6);\
    t0v3 = _m_psrldi(t0v3, 24);\
    t0v23 = _m_pand(t0v2, c6);\
    t0v24 = _m_pand(_m_psrldi(t0v2, 8), c6);\
    t0v25 = _m_pand(_m_psrldi(t0v2, 16), c6);\
    t0v2 = _m_psrldi(t0v2, 24);\
    t0v26 = _m_pand(t0v0, c6);\
    t0v27 = _m_pand(_m_psrldi(t0v0, 8), c6);\
    t0v28 = _m_pand(_m_psrldi(t0v0, 16), c6);\
    t0v0 = _m_psrldi(t0v0, 24);\
    t0v29 = _m_pand(t0v1, c6);\
    t0v30 = _m_pand(_m_psrldi(t0v1, 8), c6);\
    t0v31 = _m_pand(_m_psrldi(t0v1, 16), c6);\
    t0v1 = _m_psrldi(t0v1, 24);\
    *((__m64*)(&((dest[0])))) = _m_punpckldq(t0v5, t0v11);\
    *((__m64*)(&((dest[16])))) = _m_punpckhdq(t0v5, t0v11);\
    *((__m64*)(&((dest[1])))) = _m_punpckldq(t0v14, t0v17);\
    *((__m64*)(&((dest[17])))) = _m_punpckhdq(t0v14, t0v17);\
    *((__m64*)(&((dest[2])))) = _m_punpckldq(t0v20, t0v23);\
    *((__m64*)(&((dest[18])))) = _m_punpckhdq(t0v20, t0v23);\
    *((__m64*)(&((dest[3])))) = _m_punpckldq(t0v26, t0v29);\
    *((__m64*)(&((dest[19])))) = _m_punpckhdq(t0v26, t0v29);\
    *((__m64*)(&((dest[4])))) = _m_punpckldq(t0v9, t0v12);\
    *((__m64*)(&((dest[20])))) = _m_punpckhdq(t0v9, t0v12);\
    *((__m64*)(&((dest[5])))) = _m_punpckldq(t0v15, t0v18);\
    *((__m64*)(&((dest[21])))) = _m_punpckhdq(t0v15, t0v18);\
    *((__m64*)(&((dest[6])))) = _m_punpckldq(t0v21, t0v24);\
    *((__m64*)(&((dest[22])))) = _m_punpckhdq(t0v21, t0v24);\
    *((__m64*)(&((dest[7])))) = _m_punpckldq(t0v27, t0v30);\
    *((__m64*)(&((dest[23])))) = _m_punpckhdq(t0v27, t0v30);\
    *((__m64*)(&((dest[8])))) = _m_punpckldq(t0v10, t0v13);\
    *((__m64*)(&((dest[24])))) = _m_punpckhdq(t0v10, t0v13);\
    *((__m64*)(&((dest[9])))) = _m_punpckldq(t0v16, t0v19);\
    *((__m64*)(&((dest[25])))) = _m_punpckhdq(t0v16, t0v19);\
    *((__m64*)(&((dest[10])))) = _m_punpckldq(t0v22, t0v25);\
    *((__m64*)(&((dest[26])))) = _m_punpckhdq(t0v22, t0v25);\
    *((__m64*)(&((dest[11])))) = _m_punpckldq(t0v28, t0v31);\
    *((__m64*)(&((dest[27])))) = _m_punpckhdq(t0v28, t0v31);\
    *((__m64*)(&((dest[12])))) = _m_punpckldq(t0v8, t0v6);\
    *((__m64*)(&((dest[28])))) = _m_punpckhdq(t0v8, t0v6);\
    *((__m64*)(&((dest[13])))) = _m_punpckldq(t0v7, t0v4);\
    *((__m64*)(&((dest[29])))) = _m_punpckhdq(t0v7, t0v4);\
    *((__m64*)(&((dest[14])))) = _m_punpckldq(t0v3, t0v2);\
    *((__m64*)(&((dest[30])))) = _m_punpckhdq(t0v3, t0v2);\
    *((__m64*)(&((dest[15])))) = _m_punpckldq(t0v0, t0v1);\
    *((__m64*)(&((dest[31])))) = _m_punpckhdq(t0v0, t0v1);\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    __m64* dest = (__m64*)(D);\
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
    t0v0 = _m_pand((S0), c0);\
    t0v1 = _m_pand(_m_psrldi((S0), 1), c0);\
    t0v2 = _m_pand(_m_psrldi((S0), 2), c0);\
    t0v3 = _m_pand(_m_psrldi((S0), 3), c0);\
    t0v4 = _m_pand(_m_psrldi((S0), 4), c0);\
    t0v5 = _m_pand(_m_psrldi((S0), 5), c0);\
    t0v6 = _m_pand(_m_psrldi((S0), 6), c0);\
    t0v7 = _m_pand(_m_psrldi((S0), 7), c0);\
    t0v8 = _m_pand(_m_psrldi((S0), 8), c0);\
    t0v9 = _m_pand(_m_psrldi((S0), 9), c0);\
    t0v10 = _m_pand(_m_psrldi((S0), 10), c0);\
    t0v11 = _m_pand(_m_psrldi((S0), 11), c0);\
    t0v12 = _m_pand(_m_psrldi((S0), 12), c0);\
    t0v13 = _m_pand(_m_psrldi((S0), 13), c0);\
    t0v14 = _m_pand(_m_psrldi((S0), 14), c0);\
    t0v15 = _m_pand(_m_psrldi((S0), 15), c0);\
    t0v16 = _m_pand(_m_psrldi((S0), 16), c0);\
    t0v17 = _m_pand(_m_psrldi((S0), 17), c0);\
    t0v18 = _m_pand(_m_psrldi((S0), 18), c0);\
    t0v19 = _m_pand(_m_psrldi((S0), 19), c0);\
    t0v20 = _m_pand(_m_psrldi((S0), 20), c0);\
    t0v21 = _m_pand(_m_psrldi((S0), 21), c0);\
    t0v22 = _m_pand(_m_psrldi((S0), 22), c0);\
    t0v23 = _m_pand(_m_psrldi((S0), 23), c0);\
    t0v24 = _m_pand(_m_psrldi((S0), 24), c0);\
    t0v25 = _m_pand(_m_psrldi((S0), 25), c0);\
    t0v26 = _m_pand(_m_psrldi((S0), 26), c0);\
    t0v27 = _m_pand(_m_psrldi((S0), 27), c0);\
    t0v28 = _m_pand(_m_psrldi((S0), 28), c0);\
    t0v29 = _m_pand(_m_psrldi((S0), 29), c0);\
    t0v30 = _m_pand(_m_psrldi((S0), 30), c0);\
    t0v31 = _m_psrldi((S0), 31);\
    *((__m64*)(&((dest[0])))) = _m_punpckldq(t0v0, t0v1);\
    *((__m64*)(&((dest[16])))) = _m_punpckhdq(t0v0, t0v1);\
    *((__m64*)(&((dest[1])))) = _m_punpckldq(t0v2, t0v3);\
    *((__m64*)(&((dest[17])))) = _m_punpckhdq(t0v2, t0v3);\
    *((__m64*)(&((dest[2])))) = _m_punpckldq(t0v4, t0v5);\
    *((__m64*)(&((dest[18])))) = _m_punpckhdq(t0v4, t0v5);\
    *((__m64*)(&((dest[3])))) = _m_punpckldq(t0v6, t0v7);\
    *((__m64*)(&((dest[19])))) = _m_punpckhdq(t0v6, t0v7);\
    *((__m64*)(&((dest[4])))) = _m_punpckldq(t0v8, t0v9);\
    *((__m64*)(&((dest[20])))) = _m_punpckhdq(t0v8, t0v9);\
    *((__m64*)(&((dest[5])))) = _m_punpckldq(t0v10, t0v11);\
    *((__m64*)(&((dest[21])))) = _m_punpckhdq(t0v10, t0v11);\
    *((__m64*)(&((dest[6])))) = _m_punpckldq(t0v12, t0v13);\
    *((__m64*)(&((dest[22])))) = _m_punpckhdq(t0v12, t0v13);\
    *((__m64*)(&((dest[7])))) = _m_punpckldq(t0v14, t0v15);\
    *((__m64*)(&((dest[23])))) = _m_punpckhdq(t0v14, t0v15);\
    *((__m64*)(&((dest[8])))) = _m_punpckldq(t0v16, t0v17);\
    *((__m64*)(&((dest[24])))) = _m_punpckhdq(t0v16, t0v17);\
    *((__m64*)(&((dest[9])))) = _m_punpckldq(t0v18, t0v19);\
    *((__m64*)(&((dest[25])))) = _m_punpckhdq(t0v18, t0v19);\
    *((__m64*)(&((dest[10])))) = _m_punpckldq(t0v20, t0v21);\
    *((__m64*)(&((dest[26])))) = _m_punpckhdq(t0v20, t0v21);\
    *((__m64*)(&((dest[11])))) = _m_punpckldq(t0v22, t0v23);\
    *((__m64*)(&((dest[27])))) = _m_punpckhdq(t0v22, t0v23);\
    *((__m64*)(&((dest[12])))) = _m_punpckldq(t0v24, t0v25);\
    *((__m64*)(&((dest[28])))) = _m_punpckhdq(t0v24, t0v25);\
    *((__m64*)(&((dest[13])))) = _m_punpckldq(t0v26, t0v27);\
    *((__m64*)(&((dest[29])))) = _m_punpckhdq(t0v26, t0v27);\
    *((__m64*)(&((dest[14])))) = _m_punpckldq(t0v28, t0v29);\
    *((__m64*)(&((dest[30])))) = _m_punpckhdq(t0v28, t0v29);\
    *((__m64*)(&((dest[15])))) = _m_punpckldq(t0v30, t0v31);\
    *((__m64*)(&((dest[31])))) = _m_punpckhdq(t0v30, t0v31);\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_SSE.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    uint32_t temps[128];\
    unsigned int i;\
    _mm_storeu_ps((float*)(temps + 0), (S0));\
    _mm_storeu_ps((float*)(temps + 4), (S1));\
    _mm_storeu_ps((float*)(temps + 8), (S2));\
    _mm_storeu_ps((float*)(temps + 12), (S3));\
    _mm_storeu_ps((float*)(temps + 16), (S4));\
    _mm_storeu_ps((float*)(temps + 20), (S5));\
    _mm_storeu_ps((float*)(temps + 24), (S6));\
    _mm_storeu_ps((float*)(temps + 28), (S7));\
    _mm_storeu_ps((float*)(temps + 32), (S8));\
    _mm_storeu_ps((float*)(temps + 36), (S9));\
    _mm_storeu_ps((float*)(temps + 40), (S10));\
    _mm_storeu_ps((float*)(temps + 44), (S11));\
    _mm_storeu_ps((float*)(temps + 48), (S12));\
    _mm_storeu_ps((float*)(temps + 52), (S13));\
    _mm_storeu_ps((float*)(temps + 56), (S14));\
    _mm_storeu_ps((float*)(temps + 60), (S15));\
    _mm_storeu_ps((float*)(temps + 64), (S16));\
    _mm_storeu_ps((float*)(temps + 68), (S17));\
    _mm_storeu_ps((float*)(temps + 72), (S18));\
    _mm_storeu_ps((float*)(temps + 76), (S19));\
    _mm_storeu_ps((float*)(temps + 80), (S20));\
    _mm_storeu_ps((float*)(temps + 84), (S21));\
    _mm_storeu_ps((float*)(temps + 88), (S22));\
    _mm_storeu_ps((float*)(temps + 92), (S23));\
    _mm_storeu_ps((float*)(temps + 96), (S24));\
    _mm_storeu_ps((float*)(temps + 100), (S25));\
    _mm_storeu_ps((float*)(temps + 104), (S26));\
    _mm_storeu_ps((float*)(temps + 108), (S27));\
    _mm_storeu_ps((float*)(temps + 112), (S28));\
    _mm_storeu_ps((float*)(temps + 116), (S29));\
    _mm_storeu_ps((float*)(temps + 120), (S30));\
    _mm_storeu_ps((float*)(temps + 124), (S31));\
    for (i = 0; i < 4; i++) {\
    const unsigned int ib = i * 32;\
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
    t0v0 = ((temps[0 + i] & 0x55555555U) | ((temps[4 + i] & 0x55555555U) << 1));\
    t0v1 = (((temps[0 + i] & 0xaaaaaaaaU) >> 1) | (temps[4 + i] & 0xaaaaaaaaU));\
    t0v2 = ((temps[8 + i] & 0x55555555U) | ((temps[12 + i] & 0x55555555U) << 1));\
    t0v3 = (((temps[8 + i] & 0xaaaaaaaaU) >> 1) | (temps[12 + i] & 0xaaaaaaaaU));\
    t0v4 = ((temps[16 + i] & 0x55555555U) | ((temps[20 + i] & 0x55555555U) << 1));\
    t0v5 = (((temps[16 + i] & 0xaaaaaaaaU) >> 1) | (temps[20 + i] & 0xaaaaaaaaU));\
    t0v6 = ((temps[24 + i] & 0x55555555U) | ((temps[28 + i] & 0x55555555U) << 1));\
    t0v7 = (((temps[24 + i] & 0xaaaaaaaaU) >> 1) | (temps[28 + i] & 0xaaaaaaaaU));\
    t0v8 = ((temps[32 + i] & 0x55555555U) | ((temps[36 + i] & 0x55555555U) << 1));\
    t0v9 = (((temps[32 + i] & 0xaaaaaaaaU) >> 1) | (temps[36 + i] & 0xaaaaaaaaU));\
    t0v10 = ((temps[40 + i] & 0x55555555U) | ((temps[44 + i] & 0x55555555U) << 1));\
    t0v11 = (((temps[40 + i] & 0xaaaaaaaaU) >> 1) | (temps[44 + i] & 0xaaaaaaaaU));\
    t0v12 = ((temps[48 + i] & 0x55555555U) | ((temps[52 + i] & 0x55555555U) << 1));\
    t0v13 = (((temps[48 + i] & 0xaaaaaaaaU) >> 1) | (temps[52 + i] & 0xaaaaaaaaU));\
    t0v14 = ((temps[56 + i] & 0x55555555U) | ((temps[60 + i] & 0x55555555U) << 1));\
    t0v15 = (((temps[56 + i] & 0xaaaaaaaaU) >> 1) | (temps[60 + i] & 0xaaaaaaaaU));\
    t0v16 = ((temps[64 + i] & 0x55555555U) | ((temps[68 + i] & 0x55555555U) << 1));\
    t0v17 = (((temps[64 + i] & 0xaaaaaaaaU) >> 1) | (temps[68 + i] & 0xaaaaaaaaU));\
    t0v18 = ((temps[72 + i] & 0x55555555U) | ((temps[76 + i] & 0x55555555U) << 1));\
    t0v19 = (((temps[72 + i] & 0xaaaaaaaaU) >> 1) | (temps[76 + i] & 0xaaaaaaaaU));\
    t0v20 = ((temps[80 + i] & 0x55555555U) | ((temps[84 + i] & 0x55555555U) << 1));\
    t0v21 = (((temps[80 + i] & 0xaaaaaaaaU) >> 1) | (temps[84 + i] & 0xaaaaaaaaU));\
    t0v22 = ((temps[88 + i] & 0x55555555U) | ((temps[92 + i] & 0x55555555U) << 1));\
    t0v23 = (((temps[88 + i] & 0xaaaaaaaaU) >> 1) | (temps[92 + i] & 0xaaaaaaaaU));\
    t0v24 = ((temps[96 + i] & 0x55555555U) | ((temps[100 + i] & 0x55555555U) << 1));\
    t0v25 = (((temps[96 + i] & 0xaaaaaaaaU) >> 1) | (temps[100 + i] & 0xaaaaaaaaU));\
    t0v26 = ((temps[104 + i] & 0x55555555U) | ((temps[108 + i] & 0x55555555U) << 1));\
    t0v27 = (((temps[104 + i] & 0xaaaaaaaaU) >> 1) | (temps[108 + i] & 0xaaaaaaaaU));\
    t0v28 = ((temps[112 + i] & 0x55555555U) | ((temps[116 + i] & 0x55555555U) << 1));\
    t0v29 = (((temps[112 + i] & 0xaaaaaaaaU) >> 1) | (temps[116 + i] & 0xaaaaaaaaU));\
    t0v30 = ((temps[120 + i] & 0x55555555U) | ((temps[124 + i] & 0x55555555U) << 1));\
    t0v31 = (((temps[120 + i] & 0xaaaaaaaaU) >> 1) | (temps[124 + i] & 0xaaaaaaaaU));\
    t0v32 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x33333333U) | ((t0v18 & 0x33333333U) << 2));\
    t0v16 = (((t0v16 & 0xccccccccU) >> 2) | (t0v18 & 0xccccccccU));\
    t0v18 = ((t0v17 & 0x33333333U) | ((t0v19 & 0x33333333U) << 2));\
    t0v17 = (((t0v17 & 0xccccccccU) >> 2) | (t0v19 & 0xccccccccU));\
    t0v19 = ((t0v20 & 0x33333333U) | ((t0v22 & 0x33333333U) << 2));\
    t0v20 = (((t0v20 & 0xccccccccU) >> 2) | (t0v22 & 0xccccccccU));\
    t0v22 = ((t0v21 & 0x33333333U) | ((t0v23 & 0x33333333U) << 2));\
    t0v21 = (((t0v21 & 0xccccccccU) >> 2) | (t0v23 & 0xccccccccU));\
    t0v23 = ((t0v24 & 0x33333333U) | ((t0v26 & 0x33333333U) << 2));\
    t0v24 = (((t0v24 & 0xccccccccU) >> 2) | (t0v26 & 0xccccccccU));\
    t0v26 = ((t0v25 & 0x33333333U) | ((t0v27 & 0x33333333U) << 2));\
    t0v25 = (((t0v25 & 0xccccccccU) >> 2) | (t0v27 & 0xccccccccU));\
    t0v27 = ((t0v28 & 0x33333333U) | ((t0v30 & 0x33333333U) << 2));\
    t0v28 = (((t0v28 & 0xccccccccU) >> 2) | (t0v30 & 0xccccccccU));\
    t0v30 = ((t0v29 & 0x33333333U) | ((t0v31 & 0x33333333U) << 2));\
    t0v29 = (((t0v29 & 0xccccccccU) >> 2) | (t0v31 & 0xccccccccU));\
    t0v31 = ((t0v32 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v32 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v32 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x0f0f0f0fU) | ((t0v19 & 0x0f0f0f0fU) << 4));\
    t0v15 = (((t0v15 & 0xf0f0f0f0U) >> 4) | (t0v19 & 0xf0f0f0f0U));\
    t0v19 = ((t0v18 & 0x0f0f0f0fU) | ((t0v22 & 0x0f0f0f0fU) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0U) >> 4) | (t0v22 & 0xf0f0f0f0U));\
    t0v22 = ((t0v16 & 0x0f0f0f0fU) | ((t0v20 & 0x0f0f0f0fU) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v20 & 0xf0f0f0f0U));\
    t0v20 = ((t0v17 & 0x0f0f0f0fU) | ((t0v21 & 0x0f0f0f0fU) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0U) >> 4) | (t0v21 & 0xf0f0f0f0U));\
    t0v21 = ((t0v23 & 0x0f0f0f0fU) | ((t0v27 & 0x0f0f0f0fU) << 4));\
    t0v23 = (((t0v23 & 0xf0f0f0f0U) >> 4) | (t0v27 & 0xf0f0f0f0U));\
    t0v27 = ((t0v26 & 0x0f0f0f0fU) | ((t0v30 & 0x0f0f0f0fU) << 4));\
    t0v26 = (((t0v26 & 0xf0f0f0f0U) >> 4) | (t0v30 & 0xf0f0f0f0U));\
    t0v30 = ((t0v24 & 0x0f0f0f0fU) | ((t0v28 & 0x0f0f0f0fU) << 4));\
    t0v24 = (((t0v24 & 0xf0f0f0f0U) >> 4) | (t0v28 & 0xf0f0f0f0U));\
    t0v28 = ((t0v25 & 0x0f0f0f0fU) | ((t0v29 & 0x0f0f0f0fU) << 4));\
    t0v25 = (((t0v25 & 0xf0f0f0f0U) >> 4) | (t0v29 & 0xf0f0f0f0U));\
    t0v29 = ((t0v31 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v31 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v31 = ((t0v32 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v32 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v32 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    t0v9 = ((t0v13 & 0x00ff00ffU) | ((t0v21 & 0x00ff00ffU) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00U) >> 8) | (t0v21 & 0xff00ff00U));\
    t0v21 = ((t0v19 & 0x00ff00ffU) | ((t0v27 & 0x00ff00ffU) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00U) >> 8) | (t0v27 & 0xff00ff00U));\
    t0v27 = ((t0v22 & 0x00ff00ffU) | ((t0v30 & 0x00ff00ffU) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00U) >> 8) | (t0v30 & 0xff00ff00U));\
    t0v30 = ((t0v20 & 0x00ff00ffU) | ((t0v28 & 0x00ff00ffU) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00U) >> 8) | (t0v28 & 0xff00ff00U));\
    t0v28 = ((t0v15 & 0x00ff00ffU) | ((t0v23 & 0x00ff00ffU) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v23 & 0xff00ff00U));\
    t0v23 = ((t0v18 & 0x00ff00ffU) | ((t0v26 & 0x00ff00ffU) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00U) >> 8) | (t0v26 & 0xff00ff00U));\
    t0v26 = ((t0v16 & 0x00ff00ffU) | ((t0v24 & 0x00ff00ffU) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v24 & 0xff00ff00U));\
    t0v24 = ((t0v17 & 0x00ff00ffU) | ((t0v25 & 0x00ff00ffU) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00U) >> 8) | (t0v25 & 0xff00ff00U));\
    ((D)[0 + ib]) = ((t0v29 & 0x0000ffffU) | (t0v9 << 16));\
    ((D)[16 + ib]) = ((t0v29 >> 16) | (t0v9 & 0xffff0000U));\
    ((D)[1 + ib]) = ((t0v31 & 0x0000ffffU) | (t0v21 << 16));\
    ((D)[17 + ib]) = ((t0v31 >> 16) | (t0v21 & 0xffff0000U));\
    ((D)[2 + ib]) = ((t0v32 & 0x0000ffffU) | (t0v27 << 16));\
    ((D)[18 + ib]) = ((t0v32 >> 16) | (t0v27 & 0xffff0000U));\
    ((D)[3 + ib]) = ((t0v14 & 0x0000ffffU) | (t0v30 << 16));\
    ((D)[19 + ib]) = ((t0v14 >> 16) | (t0v30 & 0xffff0000U));\
    ((D)[4 + ib]) = ((t0v12 & 0x0000ffffU) | (t0v28 << 16));\
    ((D)[20 + ib]) = ((t0v12 >> 16) | (t0v28 & 0xffff0000U));\
    ((D)[5 + ib]) = ((t0v7 & 0x0000ffffU) | (t0v23 << 16));\
    ((D)[21 + ib]) = ((t0v7 >> 16) | (t0v23 & 0xffff0000U));\
    ((D)[6 + ib]) = ((t0v10 & 0x0000ffffU) | (t0v26 << 16));\
    ((D)[22 + ib]) = ((t0v10 >> 16) | (t0v26 & 0xffff0000U));\
    ((D)[7 + ib]) = ((t0v8 & 0x0000ffffU) | (t0v24 << 16));\
    ((D)[23 + ib]) = ((t0v8 >> 16) | (t0v24 & 0xffff0000U));\
    ((D)[8 + ib]) = ((t0v5 & 0x0000ffffU) | (t0v13 << 16));\
    ((D)[24 + ib]) = ((t0v5 >> 16) | (t0v13 & 0xffff0000U));\
    ((D)[9 + ib]) = ((t0v11 & 0x0000ffffU) | (t0v19 << 16));\
    ((D)[25 + ib]) = ((t0v11 >> 16) | (t0v19 & 0xffff0000U));\
    ((D)[10 + ib]) = ((t0v6 & 0x0000ffffU) | (t0v22 << 16));\
    ((D)[26 + ib]) = ((t0v6 >> 16) | (t0v22 & 0xffff0000U));\
    ((D)[11 + ib]) = ((t0v4 & 0x0000ffffU) | (t0v20 << 16));\
    ((D)[27 + ib]) = ((t0v4 >> 16) | (t0v20 & 0xffff0000U));\
    ((D)[12 + ib]) = ((t0v3 & 0x0000ffffU) | (t0v15 << 16));\
    ((D)[28 + ib]) = ((t0v3 >> 16) | (t0v15 & 0xffff0000U));\
    ((D)[13 + ib]) = ((t0v2 & 0x0000ffffU) | (t0v18 << 16));\
    ((D)[29 + ib]) = ((t0v2 >> 16) | (t0v18 & 0xffff0000U));\
    ((D)[14 + ib]) = ((t0v0 & 0x0000ffffU) | (t0v16 << 16));\
    ((D)[30 + ib]) = ((t0v0 >> 16) | (t0v16 & 0xffff0000U));\
    ((D)[15 + ib]) = ((t0v1 & 0x0000ffffU) | (t0v17 << 16));\
    ((D)[31 + ib]) = ((t0v1 >> 16) | (t0v17 & 0xffff0000U));\
    }\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    uint32_t temps[64];\
    unsigned int i;\
    _mm_storeu_ps((float*)(temps + 0), (S0));\
    _mm_storeu_ps((float*)(temps + 4), (S1));\
    _mm_storeu_ps((float*)(temps + 8), (S2));\
    _mm_storeu_ps((float*)(temps + 12), (S3));\
    _mm_storeu_ps((float*)(temps + 16), (S4));\
    _mm_storeu_ps((float*)(temps + 20), (S5));\
    _mm_storeu_ps((float*)(temps + 24), (S6));\
    _mm_storeu_ps((float*)(temps + 28), (S7));\
    _mm_storeu_ps((float*)(temps + 32), (S8));\
    _mm_storeu_ps((float*)(temps + 36), (S9));\
    _mm_storeu_ps((float*)(temps + 40), (S10));\
    _mm_storeu_ps((float*)(temps + 44), (S11));\
    _mm_storeu_ps((float*)(temps + 48), (S12));\
    _mm_storeu_ps((float*)(temps + 52), (S13));\
    _mm_storeu_ps((float*)(temps + 56), (S14));\
    _mm_storeu_ps((float*)(temps + 60), (S15));\
    for (i = 0; i < 4; i++) {\
    const unsigned int ib = i * 32;\
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
    t0v0 = ((temps[0 + i] & 0x55555555U) | ((temps[4 + i] & 0x55555555U) << 1));\
    t0v1 = (((temps[0 + i] & 0xaaaaaaaaU) >> 1) | (temps[4 + i] & 0xaaaaaaaaU));\
    t0v2 = ((temps[8 + i] & 0x55555555U) | ((temps[12 + i] & 0x55555555U) << 1));\
    t0v3 = (((temps[8 + i] & 0xaaaaaaaaU) >> 1) | (temps[12 + i] & 0xaaaaaaaaU));\
    t0v4 = ((temps[16 + i] & 0x55555555U) | ((temps[20 + i] & 0x55555555U) << 1));\
    t0v5 = (((temps[16 + i] & 0xaaaaaaaaU) >> 1) | (temps[20 + i] & 0xaaaaaaaaU));\
    t0v6 = ((temps[24 + i] & 0x55555555U) | ((temps[28 + i] & 0x55555555U) << 1));\
    t0v7 = (((temps[24 + i] & 0xaaaaaaaaU) >> 1) | (temps[28 + i] & 0xaaaaaaaaU));\
    t0v8 = ((temps[32 + i] & 0x55555555U) | ((temps[36 + i] & 0x55555555U) << 1));\
    t0v9 = (((temps[32 + i] & 0xaaaaaaaaU) >> 1) | (temps[36 + i] & 0xaaaaaaaaU));\
    t0v10 = ((temps[40 + i] & 0x55555555U) | ((temps[44 + i] & 0x55555555U) << 1));\
    t0v11 = (((temps[40 + i] & 0xaaaaaaaaU) >> 1) | (temps[44 + i] & 0xaaaaaaaaU));\
    t0v12 = ((temps[48 + i] & 0x55555555U) | ((temps[52 + i] & 0x55555555U) << 1));\
    t0v13 = (((temps[48 + i] & 0xaaaaaaaaU) >> 1) | (temps[52 + i] & 0xaaaaaaaaU));\
    t0v14 = ((temps[56 + i] & 0x55555555U) | ((temps[60 + i] & 0x55555555U) << 1));\
    t0v15 = (((temps[56 + i] & 0xaaaaaaaaU) >> 1) | (temps[60 + i] & 0xaaaaaaaaU));\
    t0v16 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v16 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v15 = ((t0v16 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v16 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    ((D)[0 + ib]) = (t0v13 & 0xffffU);\
    ((D)[16 + ib]) = (t0v13 >> 16);\
    ((D)[1 + ib]) = (t0v15 & 0xffffU);\
    ((D)[17 + ib]) = (t0v15 >> 16);\
    ((D)[2 + ib]) = (t0v16 & 0xffffU);\
    ((D)[18 + ib]) = (t0v16 >> 16);\
    ((D)[3 + ib]) = (t0v14 & 0xffffU);\
    ((D)[19 + ib]) = (t0v14 >> 16);\
    ((D)[4 + ib]) = (t0v12 & 0xffffU);\
    ((D)[20 + ib]) = (t0v12 >> 16);\
    ((D)[5 + ib]) = (t0v7 & 0xffffU);\
    ((D)[21 + ib]) = (t0v7 >> 16);\
    ((D)[6 + ib]) = (t0v10 & 0xffffU);\
    ((D)[22 + ib]) = (t0v10 >> 16);\
    ((D)[7 + ib]) = (t0v8 & 0xffffU);\
    ((D)[23 + ib]) = (t0v8 >> 16);\
    ((D)[8 + ib]) = (t0v5 & 0xffffU);\
    ((D)[24 + ib]) = (t0v5 >> 16);\
    ((D)[9 + ib]) = (t0v11 & 0xffffU);\
    ((D)[25 + ib]) = (t0v11 >> 16);\
    ((D)[10 + ib]) = (t0v6 & 0xffffU);\
    ((D)[26 + ib]) = (t0v6 >> 16);\
    ((D)[11 + ib]) = (t0v4 & 0xffffU);\
    ((D)[27 + ib]) = (t0v4 >> 16);\
    ((D)[12 + ib]) = (t0v3 & 0xffffU);\
    ((D)[28 + ib]) = (t0v3 >> 16);\
    ((D)[13 + ib]) = (t0v2 & 0xffffU);\
    ((D)[29 + ib]) = (t0v2 >> 16);\
    ((D)[14 + ib]) = (t0v0 & 0xffffU);\
    ((D)[30 + ib]) = (t0v0 >> 16);\
    ((D)[15 + ib]) = (t0v1 & 0xffffU);\
    ((D)[31 + ib]) = (t0v1 >> 16);\
    }\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    uint32_t temps[24];\
    unsigned int i;\
    _mm_storeu_ps((float*)(temps + 0), (S0));\
    _mm_storeu_ps((float*)(temps + 4), (S1));\
    _mm_storeu_ps((float*)(temps + 8), (S2));\
    _mm_storeu_ps((float*)(temps + 12), (S3));\
    _mm_storeu_ps((float*)(temps + 16), (S4));\
    _mm_storeu_ps((float*)(temps + 20), (S5));\
    for (i = 0; i < 4; i++) {\
    const unsigned int ib = i * 32;\
    uint32_t t0v0;\
    uint32_t t0v1;\
    uint32_t t0v2;\
    uint32_t t0v3;\
    uint32_t t0v4;\
    uint32_t t0v5;\
    uint32_t t0v6;\
    uint32_t t0v7;\
    uint32_t t0v8;\
    t0v0 = ((temps[0 + i] & 0x55555555U) | ((temps[4 + i] & 0x55555555U) << 1));\
    t0v1 = (((temps[0 + i] & 0xaaaaaaaaU) >> 1) | (temps[4 + i] & 0xaaaaaaaaU));\
    t0v2 = ((temps[8 + i] & 0x55555555U) | ((temps[12 + i] & 0x55555555U) << 1));\
    t0v3 = (((temps[8 + i] & 0xaaaaaaaaU) >> 1) | (temps[12 + i] & 0xaaaaaaaaU));\
    t0v4 = ((temps[16 + i] & 0x55555555U) | ((temps[20 + i] & 0x55555555U) << 1));\
    t0v5 = (((temps[16 + i] & 0xaaaaaaaaU) >> 1) | (temps[20 + i] & 0xaaaaaaaaU));\
    t0v6 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = (t0v4 & 0x33333333U);\
    t0v4 = ((t0v4 & 0xccccccccU) >> 2);\
    t0v7 = (t0v5 & 0x33333333U);\
    t0v5 = ((t0v5 & 0xccccccccU) >> 2);\
    t0v8 = ((t0v6 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v6 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v6 = ((t0v2 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    ((D)[0 + ib]) = (t0v8 & 0xffU);\
    ((D)[8 + ib]) = ((t0v8 >> 8) & 0xffU);\
    ((D)[16 + ib]) = ((t0v8 >> 16) & 0xffU);\
    ((D)[24 + ib]) = (t0v8 >> 24);\
    ((D)[1 + ib]) = (t0v6 & 0xffU);\
    ((D)[9 + ib]) = ((t0v6 >> 8) & 0xffU);\
    ((D)[17 + ib]) = ((t0v6 >> 16) & 0xffU);\
    ((D)[25 + ib]) = (t0v6 >> 24);\
    ((D)[2 + ib]) = (t0v7 & 0xffU);\
    ((D)[10 + ib]) = ((t0v7 >> 8) & 0xffU);\
    ((D)[18 + ib]) = ((t0v7 >> 16) & 0xffU);\
    ((D)[26 + ib]) = (t0v7 >> 24);\
    ((D)[3 + ib]) = (t0v4 & 0xffU);\
    ((D)[11 + ib]) = ((t0v4 >> 8) & 0xffU);\
    ((D)[19 + ib]) = ((t0v4 >> 16) & 0xffU);\
    ((D)[27 + ib]) = (t0v4 >> 24);\
    ((D)[4 + ib]) = (t0v3 & 0xffU);\
    ((D)[12 + ib]) = ((t0v3 >> 8) & 0xffU);\
    ((D)[20 + ib]) = ((t0v3 >> 16) & 0xffU);\
    ((D)[28 + ib]) = (t0v3 >> 24);\
    ((D)[5 + ib]) = (t0v2 & 0xffU);\
    ((D)[13 + ib]) = ((t0v2 >> 8) & 0xffU);\
    ((D)[21 + ib]) = ((t0v2 >> 16) & 0xffU);\
    ((D)[29 + ib]) = (t0v2 >> 24);\
    ((D)[6 + ib]) = (t0v0 & 0xffU);\
    ((D)[14 + ib]) = ((t0v0 >> 8) & 0xffU);\
    ((D)[22 + ib]) = ((t0v0 >> 16) & 0xffU);\
    ((D)[30 + ib]) = (t0v0 >> 24);\
    ((D)[7 + ib]) = (t0v1 & 0xffU);\
    ((D)[15 + ib]) = ((t0v1 >> 8) & 0xffU);\
    ((D)[23 + ib]) = ((t0v1 >> 16) & 0xffU);\
    ((D)[31 + ib]) = (t0v1 >> 24);\
    }\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    uint32_t temps[4];\
    unsigned int i;\
    _mm_storeu_ps((float*)(temps + 0), (S0));\
    for (i = 0; i < 4; i++) {\
    const unsigned int ib = i * 32;\
    ((D)[0 + ib]) = (temps[0 + i] & 0x1U);\
    ((D)[1 + ib]) = ((temps[0 + i] >> 1) & 0x1U);\
    ((D)[2 + ib]) = ((temps[0 + i] >> 2) & 0x1U);\
    ((D)[3 + ib]) = ((temps[0 + i] >> 3) & 0x1U);\
    ((D)[4 + ib]) = ((temps[0 + i] >> 4) & 0x1U);\
    ((D)[5 + ib]) = ((temps[0 + i] >> 5) & 0x1U);\
    ((D)[6 + ib]) = ((temps[0 + i] >> 6) & 0x1U);\
    ((D)[7 + ib]) = ((temps[0 + i] >> 7) & 0x1U);\
    ((D)[8 + ib]) = ((temps[0 + i] >> 8) & 0x1U);\
    ((D)[9 + ib]) = ((temps[0 + i] >> 9) & 0x1U);\
    ((D)[10 + ib]) = ((temps[0 + i] >> 10) & 0x1U);\
    ((D)[11 + ib]) = ((temps[0 + i] >> 11) & 0x1U);\
    ((D)[12 + ib]) = ((temps[0 + i] >> 12) & 0x1U);\
    ((D)[13 + ib]) = ((temps[0 + i] >> 13) & 0x1U);\
    ((D)[14 + ib]) = ((temps[0 + i] >> 14) & 0x1U);\
    ((D)[15 + ib]) = ((temps[0 + i] >> 15) & 0x1U);\
    ((D)[16 + ib]) = ((temps[0 + i] >> 16) & 0x1U);\
    ((D)[17 + ib]) = ((temps[0 + i] >> 17) & 0x1U);\
    ((D)[18 + ib]) = ((temps[0 + i] >> 18) & 0x1U);\
    ((D)[19 + ib]) = ((temps[0 + i] >> 19) & 0x1U);\
    ((D)[20 + ib]) = ((temps[0 + i] >> 20) & 0x1U);\
    ((D)[21 + ib]) = ((temps[0 + i] >> 21) & 0x1U);\
    ((D)[22 + ib]) = ((temps[0 + i] >> 22) & 0x1U);\
    ((D)[23 + ib]) = ((temps[0 + i] >> 23) & 0x1U);\
    ((D)[24 + ib]) = ((temps[0 + i] >> 24) & 0x1U);\
    ((D)[25 + ib]) = ((temps[0 + i] >> 25) & 0x1U);\
    ((D)[26 + ib]) = ((temps[0 + i] >> 26) & 0x1U);\
    ((D)[27 + ib]) = ((temps[0 + i] >> 27) & 0x1U);\
    ((D)[28 + ib]) = ((temps[0 + i] >> 28) & 0x1U);\
    ((D)[29 + ib]) = ((temps[0 + i] >> 29) & 0x1U);\
    ((D)[30 + ib]) = ((temps[0 + i] >> 30) & 0x1U);\
    ((D)[31 + ib]) = (temps[0 + i] >> 31);\
    }\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_SSE2.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    __m128i* dest = (__m128i*)(D);\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c10 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c11 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c2 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*5));\
    const __m128i c6 = (*(const __m128i*)(transform_const_tbl + 4*6));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*7));\
    const __m128i c8 = (*(const __m128i*)(transform_const_tbl + 4*8));\
    const __m128i c9 = (*(const __m128i*)(transform_const_tbl + 4*9));\
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
    t0v0 = _mm_or_si128(_mm_and_si128((S0), c0), _mm_slli_epi16(_mm_and_si128((S1), c0), 1));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S0), c1), 1), _mm_and_si128((S1), c1));\
    t0v2 = _mm_or_si128(_mm_and_si128((S2), c0), _mm_slli_epi16(_mm_and_si128((S3), c0), 1));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S2), c1), 1), _mm_and_si128((S3), c1));\
    t0v4 = _mm_or_si128(_mm_and_si128((S4), c0), _mm_slli_epi16(_mm_and_si128((S5), c0), 1));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S4), c1), 1), _mm_and_si128((S5), c1));\
    t0v6 = _mm_or_si128(_mm_and_si128((S6), c0), _mm_slli_epi16(_mm_and_si128((S7), c0), 1));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S6), c1), 1), _mm_and_si128((S7), c1));\
    t0v8 = _mm_or_si128(_mm_and_si128((S8), c0), _mm_slli_epi16(_mm_and_si128((S9), c0), 1));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S8), c1), 1), _mm_and_si128((S9), c1));\
    t0v10 = _mm_or_si128(_mm_and_si128((S10), c0), _mm_slli_epi16(_mm_and_si128((S11), c0), 1));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S10), c1), 1), _mm_and_si128((S11), c1));\
    t0v12 = _mm_or_si128(_mm_and_si128((S12), c0), _mm_slli_epi16(_mm_and_si128((S13), c0), 1));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S12), c1), 1), _mm_and_si128((S13), c1));\
    t0v14 = _mm_or_si128(_mm_and_si128((S14), c0), _mm_slli_epi16(_mm_and_si128((S15), c0), 1));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S14), c1), 1), _mm_and_si128((S15), c1));\
    t0v16 = _mm_or_si128(_mm_and_si128((S16), c0), _mm_slli_epi16(_mm_and_si128((S17), c0), 1));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S16), c1), 1), _mm_and_si128((S17), c1));\
    t0v18 = _mm_or_si128(_mm_and_si128((S18), c0), _mm_slli_epi16(_mm_and_si128((S19), c0), 1));\
    t0v19 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S18), c1), 1), _mm_and_si128((S19), c1));\
    t0v20 = _mm_or_si128(_mm_and_si128((S20), c0), _mm_slli_epi16(_mm_and_si128((S21), c0), 1));\
    t0v21 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S20), c1), 1), _mm_and_si128((S21), c1));\
    t0v22 = _mm_or_si128(_mm_and_si128((S22), c0), _mm_slli_epi16(_mm_and_si128((S23), c0), 1));\
    t0v23 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S22), c1), 1), _mm_and_si128((S23), c1));\
    t0v24 = _mm_or_si128(_mm_and_si128((S24), c0), _mm_slli_epi16(_mm_and_si128((S25), c0), 1));\
    t0v25 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S24), c1), 1), _mm_and_si128((S25), c1));\
    t0v26 = _mm_or_si128(_mm_and_si128((S26), c0), _mm_slli_epi16(_mm_and_si128((S27), c0), 1));\
    t0v27 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S26), c1), 1), _mm_and_si128((S27), c1));\
    t0v28 = _mm_or_si128(_mm_and_si128((S28), c0), _mm_slli_epi16(_mm_and_si128((S29), c0), 1));\
    t0v29 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S28), c1), 1), _mm_and_si128((S29), c1));\
    t0v30 = _mm_or_si128(_mm_and_si128((S30), c0), _mm_slli_epi16(_mm_and_si128((S31), c0), 1));\
    t0v31 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S30), c1), 1), _mm_and_si128((S31), c1));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi16(_mm_and_si128(t0v2, c2), 2));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c3), 2), _mm_and_si128(t0v2, c3));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi16(_mm_and_si128(t0v3, c2), 2));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c3), 2), _mm_and_si128(t0v3, c3));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v4, c2), _mm_slli_epi16(_mm_and_si128(t0v6, c2), 2));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c3), 2), _mm_and_si128(t0v6, c3));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v5, c2), _mm_slli_epi16(_mm_and_si128(t0v7, c2), 2));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v5, c3), 2), _mm_and_si128(t0v7, c3));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v8, c2), _mm_slli_epi16(_mm_and_si128(t0v10, c2), 2));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c3), 2), _mm_and_si128(t0v10, c3));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v9, c2), _mm_slli_epi16(_mm_and_si128(t0v11, c2), 2));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c3), 2), _mm_and_si128(t0v11, c3));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v12, c2), _mm_slli_epi16(_mm_and_si128(t0v14, c2), 2));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v12, c3), 2), _mm_and_si128(t0v14, c3));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v13, c2), _mm_slli_epi16(_mm_and_si128(t0v15, c2), 2));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c3), 2), _mm_and_si128(t0v15, c3));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c2), _mm_slli_epi16(_mm_and_si128(t0v18, c2), 2));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c3), 2), _mm_and_si128(t0v18, c3));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v17, c2), _mm_slli_epi16(_mm_and_si128(t0v19, c2), 2));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v17, c3), 2), _mm_and_si128(t0v19, c3));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v20, c2), _mm_slli_epi16(_mm_and_si128(t0v22, c2), 2));\
    t0v20 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v20, c3), 2), _mm_and_si128(t0v22, c3));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v21, c2), _mm_slli_epi16(_mm_and_si128(t0v23, c2), 2));\
    t0v21 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v21, c3), 2), _mm_and_si128(t0v23, c3));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v24, c2), _mm_slli_epi16(_mm_and_si128(t0v26, c2), 2));\
    t0v24 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v24, c3), 2), _mm_and_si128(t0v26, c3));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v25, c2), _mm_slli_epi16(_mm_and_si128(t0v27, c2), 2));\
    t0v25 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v25, c3), 2), _mm_and_si128(t0v27, c3));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v28, c2), _mm_slli_epi16(_mm_and_si128(t0v30, c2), 2));\
    t0v28 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v28, c3), 2), _mm_and_si128(t0v30, c3));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v29, c2), _mm_slli_epi16(_mm_and_si128(t0v31, c2), 2));\
    t0v29 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v29, c3), 2), _mm_and_si128(t0v31, c3));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c4), _mm_slli_epi16(_mm_and_si128(t0v3, c4), 4));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v32, c5), 4), _mm_and_si128(t0v3, c5));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v2, c4), _mm_slli_epi16(_mm_and_si128(t0v6, c4), 4));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c5), 4), _mm_and_si128(t0v6, c5));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v0, c4), _mm_slli_epi16(_mm_and_si128(t0v4, c4), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c5), 4), _mm_and_si128(t0v4, c5));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v1, c4), _mm_slli_epi16(_mm_and_si128(t0v5, c4), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c5), 4), _mm_and_si128(t0v5, c5));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v7, c4), _mm_slli_epi16(_mm_and_si128(t0v11, c4), 4));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c5), 4), _mm_and_si128(t0v11, c5));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v10, c4), _mm_slli_epi16(_mm_and_si128(t0v14, c4), 4));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v10, c5), 4), _mm_and_si128(t0v14, c5));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v8, c4), _mm_slli_epi16(_mm_and_si128(t0v12, c4), 4));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c5), 4), _mm_and_si128(t0v12, c5));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v9, c4), _mm_slli_epi16(_mm_and_si128(t0v13, c4), 4));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c5), 4), _mm_and_si128(t0v13, c5));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v15, c4), _mm_slli_epi16(_mm_and_si128(t0v19, c4), 4));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v15, c5), 4), _mm_and_si128(t0v19, c5));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v18, c4), _mm_slli_epi16(_mm_and_si128(t0v22, c4), 4));\
    t0v18 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v18, c5), 4), _mm_and_si128(t0v22, c5));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v16, c4), _mm_slli_epi16(_mm_and_si128(t0v20, c4), 4));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c5), 4), _mm_and_si128(t0v20, c5));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v17, c4), _mm_slli_epi16(_mm_and_si128(t0v21, c4), 4));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v17, c5), 4), _mm_and_si128(t0v21, c5));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v23, c4), _mm_slli_epi16(_mm_and_si128(t0v27, c4), 4));\
    t0v23 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v23, c5), 4), _mm_and_si128(t0v27, c5));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v26, c4), _mm_slli_epi16(_mm_and_si128(t0v30, c4), 4));\
    t0v26 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v26, c5), 4), _mm_and_si128(t0v30, c5));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v24, c4), _mm_slli_epi16(_mm_and_si128(t0v28, c4), 4));\
    t0v24 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v24, c5), 4), _mm_and_si128(t0v28, c5));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v25, c4), _mm_slli_epi16(_mm_and_si128(t0v29, c4), 4));\
    t0v25 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v25, c5), 4), _mm_and_si128(t0v29, c5));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v31, c6), _mm_slli_epi16(t0v5, 8));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(t0v31, 8), _mm_and_si128(t0v5, c7));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c6), _mm_slli_epi16(t0v11, 8));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(t0v32, 8), _mm_and_si128(t0v11, c7));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v6, c6), _mm_slli_epi16(t0v14, 8));\
    t0v6 = _mm_or_si128(_mm_srli_epi16(t0v6, 8), _mm_and_si128(t0v14, c7));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v4, c6), _mm_slli_epi16(t0v12, 8));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(t0v4, 8), _mm_and_si128(t0v12, c7));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v3, c6), _mm_slli_epi16(t0v7, 8));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(t0v3, 8), _mm_and_si128(t0v7, c7));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v2, c6), _mm_slli_epi16(t0v10, 8));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(t0v2, 8), _mm_and_si128(t0v10, c7));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v0, c6), _mm_slli_epi16(t0v8, 8));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(t0v0, 8), _mm_and_si128(t0v8, c7));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v1, c6), _mm_slli_epi16(t0v9, 8));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(t0v1, 8), _mm_and_si128(t0v9, c7));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v13, c6), _mm_slli_epi16(t0v21, 8));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(t0v13, 8), _mm_and_si128(t0v21, c7));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v19, c6), _mm_slli_epi16(t0v27, 8));\
    t0v19 = _mm_or_si128(_mm_srli_epi16(t0v19, 8), _mm_and_si128(t0v27, c7));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v22, c6), _mm_slli_epi16(t0v30, 8));\
    t0v22 = _mm_or_si128(_mm_srli_epi16(t0v22, 8), _mm_and_si128(t0v30, c7));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v20, c6), _mm_slli_epi16(t0v28, 8));\
    t0v20 = _mm_or_si128(_mm_srli_epi16(t0v20, 8), _mm_and_si128(t0v28, c7));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v15, c6), _mm_slli_epi16(t0v23, 8));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(t0v15, 8), _mm_and_si128(t0v23, c7));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v18, c6), _mm_slli_epi16(t0v26, 8));\
    t0v18 = _mm_or_si128(_mm_srli_epi16(t0v18, 8), _mm_and_si128(t0v26, c7));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v16, c6), _mm_slli_epi16(t0v24, 8));\
    t0v16 = _mm_or_si128(_mm_srli_epi16(t0v16, 8), _mm_and_si128(t0v24, c7));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v17, c6), _mm_slli_epi16(t0v25, 8));\
    t0v17 = _mm_or_si128(_mm_srli_epi16(t0v17, 8), _mm_and_si128(t0v25, c7));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v29, c8), _mm_slli_epi32(t0v9, 16));\
    t0v9 = _mm_or_si128(_mm_srli_epi32(t0v29, 16), _mm_and_si128(t0v9, c9));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v31, c8), _mm_slli_epi32(t0v21, 16));\
    t0v21 = _mm_or_si128(_mm_srli_epi32(t0v31, 16), _mm_and_si128(t0v21, c9));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v32, c8), _mm_slli_epi32(t0v27, 16));\
    t0v27 = _mm_or_si128(_mm_srli_epi32(t0v32, 16), _mm_and_si128(t0v27, c9));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v14, c8), _mm_slli_epi32(t0v30, 16));\
    t0v14 = _mm_or_si128(_mm_srli_epi32(t0v14, 16), _mm_and_si128(t0v30, c9));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v12, c8), _mm_slli_epi32(t0v28, 16));\
    t0v12 = _mm_or_si128(_mm_srli_epi32(t0v12, 16), _mm_and_si128(t0v28, c9));\
    t0v28 = _mm_or_si128(_mm_and_si128(t0v7, c8), _mm_slli_epi32(t0v23, 16));\
    t0v7 = _mm_or_si128(_mm_srli_epi32(t0v7, 16), _mm_and_si128(t0v23, c9));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v10, c8), _mm_slli_epi32(t0v26, 16));\
    t0v10 = _mm_or_si128(_mm_srli_epi32(t0v10, 16), _mm_and_si128(t0v26, c9));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v8, c8), _mm_slli_epi32(t0v24, 16));\
    t0v8 = _mm_or_si128(_mm_srli_epi32(t0v8, 16), _mm_and_si128(t0v24, c9));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v5, c8), _mm_slli_epi32(t0v13, 16));\
    t0v5 = _mm_or_si128(_mm_srli_epi32(t0v5, 16), _mm_and_si128(t0v13, c9));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v11, c8), _mm_slli_epi32(t0v19, 16));\
    t0v11 = _mm_or_si128(_mm_srli_epi32(t0v11, 16), _mm_and_si128(t0v19, c9));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v6, c8), _mm_slli_epi32(t0v22, 16));\
    t0v6 = _mm_or_si128(_mm_srli_epi32(t0v6, 16), _mm_and_si128(t0v22, c9));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v4, c8), _mm_slli_epi32(t0v20, 16));\
    t0v4 = _mm_or_si128(_mm_srli_epi32(t0v4, 16), _mm_and_si128(t0v20, c9));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v3, c8), _mm_slli_epi32(t0v15, 16));\
    t0v3 = _mm_or_si128(_mm_srli_epi32(t0v3, 16), _mm_and_si128(t0v15, c9));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v2, c8), _mm_slli_epi32(t0v18, 16));\
    t0v2 = _mm_or_si128(_mm_srli_epi32(t0v2, 16), _mm_and_si128(t0v18, c9));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v0, c8), _mm_slli_epi32(t0v16, 16));\
    t0v0 = _mm_or_si128(_mm_srli_epi32(t0v0, 16), _mm_and_si128(t0v16, c9));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v1, c8), _mm_slli_epi32(t0v17, 16));\
    t0v1 = _mm_or_si128(_mm_srli_epi32(t0v1, 16), _mm_and_si128(t0v17, c9));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v25, c10), _mm_slli_epi64(t0v29, 32));\
    t0v25 = _mm_or_si128(_mm_srli_epi64(t0v25, 32), _mm_and_si128(t0v29, c11));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v31, c10), _mm_slli_epi64(t0v32, 32));\
    t0v31 = _mm_or_si128(_mm_srli_epi64(t0v31, 32), _mm_and_si128(t0v32, c11));\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v30, c10), _mm_slli_epi64(t0v28, 32));\
    t0v28 = _mm_or_si128(_mm_srli_epi64(t0v30, 32), _mm_and_si128(t0v28, c11));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v23, c10), _mm_slli_epi64(t0v26, 32));\
    t0v23 = _mm_or_si128(_mm_srli_epi64(t0v23, 32), _mm_and_si128(t0v26, c11));\
    t0v26 = _mm_or_si128(_mm_and_si128(t0v24, c10), _mm_slli_epi64(t0v13, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v24, 32), _mm_and_si128(t0v13, c11));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v19, c10), _mm_slli_epi64(t0v22, 32));\
    t0v19 = _mm_or_si128(_mm_srli_epi64(t0v19, 32), _mm_and_si128(t0v22, c11));\
    t0v22 = _mm_or_si128(_mm_and_si128(t0v20, c10), _mm_slli_epi64(t0v15, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v20, 32), _mm_and_si128(t0v15, c11));\
    t0v20 = _mm_or_si128(_mm_and_si128(t0v18, c10), _mm_slli_epi64(t0v16, 32));\
    t0v16 = _mm_or_si128(_mm_srli_epi64(t0v18, 32), _mm_and_si128(t0v16, c11));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v9, c10), _mm_slli_epi64(t0v21, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v21, c11));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v27, c10), _mm_slli_epi64(t0v14, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v27, 32), _mm_and_si128(t0v14, c11));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v12, c10), _mm_slli_epi64(t0v7, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v7, c11));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v10, c10), _mm_slli_epi64(t0v8, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v8, c11));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v5, c10), _mm_slli_epi64(t0v11, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v11, c11));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v6, c10), _mm_slli_epi64(t0v4, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v4, c11));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v3, c10), _mm_slli_epi64(t0v2, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v2, c11));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v0, c10), _mm_slli_epi64(t0v1, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v1, c11));\
    _mm_storeu_si128((__m128i*)&((dest[0])), _mm_unpacklo_epi64(t0v17, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[16])), _mm_unpackhi_epi64(t0v17, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[1])), _mm_unpacklo_epi64(t0v32, t0v30));\
    _mm_storeu_si128((__m128i*)&((dest[17])), _mm_unpackhi_epi64(t0v32, t0v30));\
    _mm_storeu_si128((__m128i*)&((dest[2])), _mm_unpacklo_epi64(t0v26, t0v24));\
    _mm_storeu_si128((__m128i*)&((dest[18])), _mm_unpackhi_epi64(t0v26, t0v24));\
    _mm_storeu_si128((__m128i*)&((dest[3])), _mm_unpacklo_epi64(t0v22, t0v20));\
    _mm_storeu_si128((__m128i*)&((dest[19])), _mm_unpackhi_epi64(t0v22, t0v20));\
    _mm_storeu_si128((__m128i*)&((dest[4])), _mm_unpacklo_epi64(t0v18, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[20])), _mm_unpackhi_epi64(t0v18, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[5])), _mm_unpacklo_epi64(t0v27, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[21])), _mm_unpackhi_epi64(t0v27, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[6])), _mm_unpacklo_epi64(t0v10, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[22])), _mm_unpackhi_epi64(t0v10, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[7])), _mm_unpacklo_epi64(t0v6, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[23])), _mm_unpackhi_epi64(t0v6, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[8])), _mm_unpacklo_epi64(t0v25, t0v31));\
    _mm_storeu_si128((__m128i*)&((dest[24])), _mm_unpackhi_epi64(t0v25, t0v31));\
    _mm_storeu_si128((__m128i*)&((dest[9])), _mm_unpacklo_epi64(t0v28, t0v23));\
    _mm_storeu_si128((__m128i*)&((dest[25])), _mm_unpackhi_epi64(t0v28, t0v23));\
    _mm_storeu_si128((__m128i*)&((dest[10])), _mm_unpacklo_epi64(t0v13, t0v19));\
    _mm_storeu_si128((__m128i*)&((dest[26])), _mm_unpackhi_epi64(t0v13, t0v19));\
    _mm_storeu_si128((__m128i*)&((dest[11])), _mm_unpacklo_epi64(t0v15, t0v16));\
    _mm_storeu_si128((__m128i*)&((dest[27])), _mm_unpackhi_epi64(t0v15, t0v16));\
    _mm_storeu_si128((__m128i*)&((dest[12])), _mm_unpacklo_epi64(t0v9, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[28])), _mm_unpackhi_epi64(t0v9, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[13])), _mm_unpacklo_epi64(t0v7, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[29])), _mm_unpackhi_epi64(t0v7, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[14])), _mm_unpacklo_epi64(t0v5, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[30])), _mm_unpackhi_epi64(t0v5, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[15])), _mm_unpacklo_epi64(t0v2, t0v0));\
    _mm_storeu_si128((__m128i*)&((dest[31])), _mm_unpackhi_epi64(t0v2, t0v0));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    __m128i* dest = (__m128i*)(D);\
    const __m128i c8 = (*(const __m128i*)(transform_const2_tbl + 4*4));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c9 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c10 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c2 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*5));\
    const __m128i c6 = (*(const __m128i*)(transform_const_tbl + 4*6));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*7));\
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
    t0v0 = _mm_or_si128(_mm_and_si128((S0), c0), _mm_slli_epi16(_mm_and_si128((S1), c0), 1));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S0), c1), 1), _mm_and_si128((S1), c1));\
    t0v2 = _mm_or_si128(_mm_and_si128((S2), c0), _mm_slli_epi16(_mm_and_si128((S3), c0), 1));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S2), c1), 1), _mm_and_si128((S3), c1));\
    t0v4 = _mm_or_si128(_mm_and_si128((S4), c0), _mm_slli_epi16(_mm_and_si128((S5), c0), 1));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S4), c1), 1), _mm_and_si128((S5), c1));\
    t0v6 = _mm_or_si128(_mm_and_si128((S6), c0), _mm_slli_epi16(_mm_and_si128((S7), c0), 1));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S6), c1), 1), _mm_and_si128((S7), c1));\
    t0v8 = _mm_or_si128(_mm_and_si128((S8), c0), _mm_slli_epi16(_mm_and_si128((S9), c0), 1));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S8), c1), 1), _mm_and_si128((S9), c1));\
    t0v10 = _mm_or_si128(_mm_and_si128((S10), c0), _mm_slli_epi16(_mm_and_si128((S11), c0), 1));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S10), c1), 1), _mm_and_si128((S11), c1));\
    t0v12 = _mm_or_si128(_mm_and_si128((S12), c0), _mm_slli_epi16(_mm_and_si128((S13), c0), 1));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S12), c1), 1), _mm_and_si128((S13), c1));\
    t0v14 = _mm_or_si128(_mm_and_si128((S14), c0), _mm_slli_epi16(_mm_and_si128((S15), c0), 1));\
    t0v15 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S14), c1), 1), _mm_and_si128((S15), c1));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi16(_mm_and_si128(t0v2, c2), 2));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c3), 2), _mm_and_si128(t0v2, c3));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi16(_mm_and_si128(t0v3, c2), 2));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c3), 2), _mm_and_si128(t0v3, c3));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v4, c2), _mm_slli_epi16(_mm_and_si128(t0v6, c2), 2));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v4, c3), 2), _mm_and_si128(t0v6, c3));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v5, c2), _mm_slli_epi16(_mm_and_si128(t0v7, c2), 2));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v5, c3), 2), _mm_and_si128(t0v7, c3));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v8, c2), _mm_slli_epi16(_mm_and_si128(t0v10, c2), 2));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c3), 2), _mm_and_si128(t0v10, c3));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v9, c2), _mm_slli_epi16(_mm_and_si128(t0v11, c2), 2));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c3), 2), _mm_and_si128(t0v11, c3));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v12, c2), _mm_slli_epi16(_mm_and_si128(t0v14, c2), 2));\
    t0v12 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v12, c3), 2), _mm_and_si128(t0v14, c3));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v13, c2), _mm_slli_epi16(_mm_and_si128(t0v15, c2), 2));\
    t0v13 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v13, c3), 2), _mm_and_si128(t0v15, c3));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c4), _mm_slli_epi16(_mm_and_si128(t0v3, c4), 4));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v16, c5), 4), _mm_and_si128(t0v3, c5));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v2, c4), _mm_slli_epi16(_mm_and_si128(t0v6, c4), 4));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c5), 4), _mm_and_si128(t0v6, c5));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v0, c4), _mm_slli_epi16(_mm_and_si128(t0v4, c4), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c5), 4), _mm_and_si128(t0v4, c5));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v1, c4), _mm_slli_epi16(_mm_and_si128(t0v5, c4), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c5), 4), _mm_and_si128(t0v5, c5));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v7, c4), _mm_slli_epi16(_mm_and_si128(t0v11, c4), 4));\
    t0v7 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v7, c5), 4), _mm_and_si128(t0v11, c5));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v10, c4), _mm_slli_epi16(_mm_and_si128(t0v14, c4), 4));\
    t0v10 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v10, c5), 4), _mm_and_si128(t0v14, c5));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v8, c4), _mm_slli_epi16(_mm_and_si128(t0v12, c4), 4));\
    t0v8 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v8, c5), 4), _mm_and_si128(t0v12, c5));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v9, c4), _mm_slli_epi16(_mm_and_si128(t0v13, c4), 4));\
    t0v9 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v9, c5), 4), _mm_and_si128(t0v13, c5));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v15, c6), _mm_slli_epi16(t0v5, 8));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(t0v15, 8), _mm_and_si128(t0v5, c7));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c6), _mm_slli_epi16(t0v11, 8));\
    t0v11 = _mm_or_si128(_mm_srli_epi16(t0v16, 8), _mm_and_si128(t0v11, c7));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v6, c6), _mm_slli_epi16(t0v14, 8));\
    t0v6 = _mm_or_si128(_mm_srli_epi16(t0v6, 8), _mm_and_si128(t0v14, c7));\
    t0v14 = _mm_or_si128(_mm_and_si128(t0v4, c6), _mm_slli_epi16(t0v12, 8));\
    t0v4 = _mm_or_si128(_mm_srli_epi16(t0v4, 8), _mm_and_si128(t0v12, c7));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v3, c6), _mm_slli_epi16(t0v7, 8));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(t0v3, 8), _mm_and_si128(t0v7, c7));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v2, c6), _mm_slli_epi16(t0v10, 8));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(t0v2, 8), _mm_and_si128(t0v10, c7));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v0, c6), _mm_slli_epi16(t0v8, 8));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(t0v0, 8), _mm_and_si128(t0v8, c7));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v1, c6), _mm_slli_epi16(t0v9, 8));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(t0v1, 8), _mm_and_si128(t0v9, c7));\
    t0v9 = _mm_and_si128(t0v13, c8);\
    t0v13 = _mm_srli_epi32(t0v13, 16);\
    t0v17 = _mm_and_si128(t0v15, c8);\
    t0v15 = _mm_srli_epi32(t0v15, 16);\
    t0v18 = _mm_and_si128(t0v16, c8);\
    t0v16 = _mm_srli_epi32(t0v16, 16);\
    t0v19 = _mm_and_si128(t0v14, c8);\
    t0v14 = _mm_srli_epi32(t0v14, 16);\
    t0v20 = _mm_and_si128(t0v12, c8);\
    t0v12 = _mm_srli_epi32(t0v12, 16);\
    t0v21 = _mm_and_si128(t0v7, c8);\
    t0v7 = _mm_srli_epi32(t0v7, 16);\
    t0v22 = _mm_and_si128(t0v10, c8);\
    t0v10 = _mm_srli_epi32(t0v10, 16);\
    t0v23 = _mm_and_si128(t0v8, c8);\
    t0v8 = _mm_srli_epi32(t0v8, 16);\
    t0v24 = _mm_and_si128(t0v5, c8);\
    t0v5 = _mm_srli_epi32(t0v5, 16);\
    t0v25 = _mm_and_si128(t0v11, c8);\
    t0v11 = _mm_srli_epi32(t0v11, 16);\
    t0v26 = _mm_and_si128(t0v6, c8);\
    t0v6 = _mm_srli_epi32(t0v6, 16);\
    t0v27 = _mm_and_si128(t0v4, c8);\
    t0v4 = _mm_srli_epi32(t0v4, 16);\
    t0v28 = _mm_and_si128(t0v3, c8);\
    t0v3 = _mm_srli_epi32(t0v3, 16);\
    t0v29 = _mm_and_si128(t0v2, c8);\
    t0v2 = _mm_srli_epi32(t0v2, 16);\
    t0v30 = _mm_and_si128(t0v0, c8);\
    t0v0 = _mm_srli_epi32(t0v0, 16);\
    t0v31 = _mm_and_si128(t0v1, c8);\
    t0v1 = _mm_srli_epi32(t0v1, 16);\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v9, c9), _mm_slli_epi64(t0v17, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v17, c10));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v18, c9), _mm_slli_epi64(t0v19, 32));\
    t0v18 = _mm_or_si128(_mm_srli_epi64(t0v18, 32), _mm_and_si128(t0v19, c10));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v20, c9), _mm_slli_epi64(t0v21, 32));\
    t0v20 = _mm_or_si128(_mm_srli_epi64(t0v20, 32), _mm_and_si128(t0v21, c10));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v22, c9), _mm_slli_epi64(t0v23, 32));\
    t0v22 = _mm_or_si128(_mm_srli_epi64(t0v22, 32), _mm_and_si128(t0v23, c10));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v24, c9), _mm_slli_epi64(t0v25, 32));\
    t0v24 = _mm_or_si128(_mm_srli_epi64(t0v24, 32), _mm_and_si128(t0v25, c10));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v26, c9), _mm_slli_epi64(t0v27, 32));\
    t0v26 = _mm_or_si128(_mm_srli_epi64(t0v26, 32), _mm_and_si128(t0v27, c10));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v28, c9), _mm_slli_epi64(t0v29, 32));\
    t0v28 = _mm_or_si128(_mm_srli_epi64(t0v28, 32), _mm_and_si128(t0v29, c10));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v30, c9), _mm_slli_epi64(t0v31, 32));\
    t0v30 = _mm_or_si128(_mm_srli_epi64(t0v30, 32), _mm_and_si128(t0v31, c10));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v13, c9), _mm_slli_epi64(t0v15, 32));\
    t0v13 = _mm_or_si128(_mm_srli_epi64(t0v13, 32), _mm_and_si128(t0v15, c10));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c9), _mm_slli_epi64(t0v14, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v16, 32), _mm_and_si128(t0v14, c10));\
    t0v16 = _mm_or_si128(_mm_and_si128(t0v12, c9), _mm_slli_epi64(t0v7, 32));\
    t0v7 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v7, c10));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v10, c9), _mm_slli_epi64(t0v8, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v8, c10));\
    t0v10 = _mm_or_si128(_mm_and_si128(t0v5, c9), _mm_slli_epi64(t0v11, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v11, c10));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v6, c9), _mm_slli_epi64(t0v4, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v4, c10));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v3, c9), _mm_slli_epi64(t0v2, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v2, c10));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v0, c9), _mm_slli_epi64(t0v1, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v1, c10));\
    _mm_storeu_si128((__m128i*)&((dest[0])), _mm_unpacklo_epi64(t0v32, t0v17));\
    _mm_storeu_si128((__m128i*)&((dest[16])), _mm_unpackhi_epi64(t0v32, t0v17));\
    _mm_storeu_si128((__m128i*)&((dest[1])), _mm_unpacklo_epi64(t0v19, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[17])), _mm_unpackhi_epi64(t0v19, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[2])), _mm_unpacklo_epi64(t0v23, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[18])), _mm_unpackhi_epi64(t0v23, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[3])), _mm_unpacklo_epi64(t0v27, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[19])), _mm_unpackhi_epi64(t0v27, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[4])), _mm_unpacklo_epi64(t0v31, t0v15));\
    _mm_storeu_si128((__m128i*)&((dest[20])), _mm_unpackhi_epi64(t0v31, t0v15));\
    _mm_storeu_si128((__m128i*)&((dest[5])), _mm_unpacklo_epi64(t0v16, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[21])), _mm_unpackhi_epi64(t0v16, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[6])), _mm_unpacklo_epi64(t0v10, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[22])), _mm_unpackhi_epi64(t0v10, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[7])), _mm_unpacklo_epi64(t0v6, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[23])), _mm_unpackhi_epi64(t0v6, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[8])), _mm_unpacklo_epi64(t0v9, t0v18));\
    _mm_storeu_si128((__m128i*)&((dest[24])), _mm_unpackhi_epi64(t0v9, t0v18));\
    _mm_storeu_si128((__m128i*)&((dest[9])), _mm_unpacklo_epi64(t0v20, t0v22));\
    _mm_storeu_si128((__m128i*)&((dest[25])), _mm_unpackhi_epi64(t0v20, t0v22));\
    _mm_storeu_si128((__m128i*)&((dest[10])), _mm_unpacklo_epi64(t0v24, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[26])), _mm_unpackhi_epi64(t0v24, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[11])), _mm_unpacklo_epi64(t0v28, t0v30));\
    _mm_storeu_si128((__m128i*)&((dest[27])), _mm_unpackhi_epi64(t0v28, t0v30));\
    _mm_storeu_si128((__m128i*)&((dest[12])), _mm_unpacklo_epi64(t0v13, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[28])), _mm_unpackhi_epi64(t0v13, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[13])), _mm_unpacklo_epi64(t0v7, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[29])), _mm_unpackhi_epi64(t0v7, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[14])), _mm_unpacklo_epi64(t0v5, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[30])), _mm_unpackhi_epi64(t0v5, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[15])), _mm_unpacklo_epi64(t0v2, t0v0));\
    _mm_storeu_si128((__m128i*)&((dest[31])), _mm_unpackhi_epi64(t0v2, t0v0));\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    __m128i* dest = (__m128i*)(D);\
    const __m128i c6 = (*(const __m128i*)(transform_const2_tbl + 4*3));\
    const __m128i c0 = (*(const __m128i*)(transform_const_tbl + 4*0));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*1));\
    const __m128i c7 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c8 = (*(const __m128i*)(transform_const_tbl + 4*11));\
    const __m128i c2 = (*(const __m128i*)(transform_const_tbl + 4*2));\
    const __m128i c3 = (*(const __m128i*)(transform_const_tbl + 4*3));\
    const __m128i c4 = (*(const __m128i*)(transform_const_tbl + 4*4));\
    const __m128i c5 = (*(const __m128i*)(transform_const_tbl + 4*5));\
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
    t0v0 = _mm_or_si128(_mm_and_si128((S0), c0), _mm_slli_epi16(_mm_and_si128((S1), c0), 1));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S0), c1), 1), _mm_and_si128((S1), c1));\
    t0v2 = _mm_or_si128(_mm_and_si128((S2), c0), _mm_slli_epi16(_mm_and_si128((S3), c0), 1));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S2), c1), 1), _mm_and_si128((S3), c1));\
    t0v4 = _mm_or_si128(_mm_and_si128((S4), c0), _mm_slli_epi16(_mm_and_si128((S5), c0), 1));\
    t0v5 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128((S4), c1), 1), _mm_and_si128((S5), c1));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v0, c2), _mm_slli_epi16(_mm_and_si128(t0v2, c2), 2));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c3), 2), _mm_and_si128(t0v2, c3));\
    t0v2 = _mm_or_si128(_mm_and_si128(t0v1, c2), _mm_slli_epi16(_mm_and_si128(t0v3, c2), 2));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c3), 2), _mm_and_si128(t0v3, c3));\
    t0v3 = _mm_and_si128(t0v4, c2);\
    t0v4 = _mm_srli_epi16(_mm_and_si128(t0v4, c3), 2);\
    t0v7 = _mm_and_si128(t0v5, c2);\
    t0v5 = _mm_srli_epi16(_mm_and_si128(t0v5, c3), 2);\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v6, c4), _mm_slli_epi16(_mm_and_si128(t0v3, c4), 4));\
    t0v3 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v6, c5), 4), _mm_and_si128(t0v3, c5));\
    t0v6 = _mm_or_si128(_mm_and_si128(t0v2, c4), _mm_slli_epi16(_mm_and_si128(t0v7, c4), 4));\
    t0v2 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v2, c5), 4), _mm_and_si128(t0v7, c5));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v0, c4), _mm_slli_epi16(_mm_and_si128(t0v4, c4), 4));\
    t0v0 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v0, c5), 4), _mm_and_si128(t0v4, c5));\
    t0v4 = _mm_or_si128(_mm_and_si128(t0v1, c4), _mm_slli_epi16(_mm_and_si128(t0v5, c4), 4));\
    t0v1 = _mm_or_si128(_mm_srli_epi16(_mm_and_si128(t0v1, c5), 4), _mm_and_si128(t0v5, c5));\
    t0v5 = _mm_and_si128(t0v8, c6);\
    t0v9 = _mm_and_si128(_mm_srli_epi32(t0v8, 8), c6);\
    t0v10 = _mm_and_si128(_mm_srli_epi32(t0v8, 16), c6);\
    t0v8 = _mm_srli_epi32(t0v8, 24);\
    t0v11 = _mm_and_si128(t0v6, c6);\
    t0v12 = _mm_and_si128(_mm_srli_epi32(t0v6, 8), c6);\
    t0v13 = _mm_and_si128(_mm_srli_epi32(t0v6, 16), c6);\
    t0v6 = _mm_srli_epi32(t0v6, 24);\
    t0v14 = _mm_and_si128(t0v7, c6);\
    t0v15 = _mm_and_si128(_mm_srli_epi32(t0v7, 8), c6);\
    t0v16 = _mm_and_si128(_mm_srli_epi32(t0v7, 16), c6);\
    t0v7 = _mm_srli_epi32(t0v7, 24);\
    t0v17 = _mm_and_si128(t0v4, c6);\
    t0v18 = _mm_and_si128(_mm_srli_epi32(t0v4, 8), c6);\
    t0v19 = _mm_and_si128(_mm_srli_epi32(t0v4, 16), c6);\
    t0v4 = _mm_srli_epi32(t0v4, 24);\
    t0v20 = _mm_and_si128(t0v3, c6);\
    t0v21 = _mm_and_si128(_mm_srli_epi32(t0v3, 8), c6);\
    t0v22 = _mm_and_si128(_mm_srli_epi32(t0v3, 16), c6);\
    t0v3 = _mm_srli_epi32(t0v3, 24);\
    t0v23 = _mm_and_si128(t0v2, c6);\
    t0v24 = _mm_and_si128(_mm_srli_epi32(t0v2, 8), c6);\
    t0v25 = _mm_and_si128(_mm_srli_epi32(t0v2, 16), c6);\
    t0v2 = _mm_srli_epi32(t0v2, 24);\
    t0v26 = _mm_and_si128(t0v0, c6);\
    t0v27 = _mm_and_si128(_mm_srli_epi32(t0v0, 8), c6);\
    t0v28 = _mm_and_si128(_mm_srli_epi32(t0v0, 16), c6);\
    t0v0 = _mm_srli_epi32(t0v0, 24);\
    t0v29 = _mm_and_si128(t0v1, c6);\
    t0v30 = _mm_and_si128(_mm_srli_epi32(t0v1, 8), c6);\
    t0v31 = _mm_and_si128(_mm_srli_epi32(t0v1, 16), c6);\
    t0v1 = _mm_srli_epi32(t0v1, 24);\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v5, c7), _mm_slli_epi64(t0v11, 32));\
    t0v5 = _mm_or_si128(_mm_srli_epi64(t0v5, 32), _mm_and_si128(t0v11, c8));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v14, c7), _mm_slli_epi64(t0v17, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v17, c8));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v20, c7), _mm_slli_epi64(t0v23, 32));\
    t0v20 = _mm_or_si128(_mm_srli_epi64(t0v20, 32), _mm_and_si128(t0v23, c8));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v26, c7), _mm_slli_epi64(t0v29, 32));\
    t0v26 = _mm_or_si128(_mm_srli_epi64(t0v26, 32), _mm_and_si128(t0v29, c8));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v9, c7), _mm_slli_epi64(t0v12, 32));\
    t0v9 = _mm_or_si128(_mm_srli_epi64(t0v9, 32), _mm_and_si128(t0v12, c8));\
    t0v12 = _mm_or_si128(_mm_and_si128(t0v15, c7), _mm_slli_epi64(t0v18, 32));\
    t0v15 = _mm_or_si128(_mm_srli_epi64(t0v15, 32), _mm_and_si128(t0v18, c8));\
    t0v18 = _mm_or_si128(_mm_and_si128(t0v21, c7), _mm_slli_epi64(t0v24, 32));\
    t0v21 = _mm_or_si128(_mm_srli_epi64(t0v21, 32), _mm_and_si128(t0v24, c8));\
    t0v24 = _mm_or_si128(_mm_and_si128(t0v27, c7), _mm_slli_epi64(t0v30, 32));\
    t0v27 = _mm_or_si128(_mm_srli_epi64(t0v27, 32), _mm_and_si128(t0v30, c8));\
    t0v30 = _mm_or_si128(_mm_and_si128(t0v10, c7), _mm_slli_epi64(t0v13, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v13, c8));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v16, c7), _mm_slli_epi64(t0v19, 32));\
    t0v16 = _mm_or_si128(_mm_srli_epi64(t0v16, 32), _mm_and_si128(t0v19, c8));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v22, c7), _mm_slli_epi64(t0v25, 32));\
    t0v22 = _mm_or_si128(_mm_srli_epi64(t0v22, 32), _mm_and_si128(t0v25, c8));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v28, c7), _mm_slli_epi64(t0v31, 32));\
    t0v28 = _mm_or_si128(_mm_srli_epi64(t0v28, 32), _mm_and_si128(t0v31, c8));\
    t0v31 = _mm_or_si128(_mm_and_si128(t0v8, c7), _mm_slli_epi64(t0v6, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v6, c8));\
    t0v8 = _mm_or_si128(_mm_and_si128(t0v7, c7), _mm_slli_epi64(t0v4, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v7, 32), _mm_and_si128(t0v4, c8));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v3, c7), _mm_slli_epi64(t0v2, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v3, 32), _mm_and_si128(t0v2, c8));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v0, c7), _mm_slli_epi64(t0v1, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v1, c8));\
    _mm_storeu_si128((__m128i*)&((dest[0])), _mm_unpacklo_epi64(t0v32, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[16])), _mm_unpackhi_epi64(t0v32, t0v11));\
    _mm_storeu_si128((__m128i*)&((dest[1])), _mm_unpacklo_epi64(t0v17, t0v23));\
    _mm_storeu_si128((__m128i*)&((dest[17])), _mm_unpackhi_epi64(t0v17, t0v23));\
    _mm_storeu_si128((__m128i*)&((dest[2])), _mm_unpacklo_epi64(t0v29, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[18])), _mm_unpackhi_epi64(t0v29, t0v12));\
    _mm_storeu_si128((__m128i*)&((dest[3])), _mm_unpacklo_epi64(t0v18, t0v24));\
    _mm_storeu_si128((__m128i*)&((dest[19])), _mm_unpackhi_epi64(t0v18, t0v24));\
    _mm_storeu_si128((__m128i*)&((dest[4])), _mm_unpacklo_epi64(t0v30, t0v13));\
    _mm_storeu_si128((__m128i*)&((dest[20])), _mm_unpackhi_epi64(t0v30, t0v13));\
    _mm_storeu_si128((__m128i*)&((dest[5])), _mm_unpacklo_epi64(t0v19, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[21])), _mm_unpackhi_epi64(t0v19, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[6])), _mm_unpacklo_epi64(t0v31, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[22])), _mm_unpackhi_epi64(t0v31, t0v8));\
    _mm_storeu_si128((__m128i*)&((dest[7])), _mm_unpacklo_epi64(t0v7, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[23])), _mm_unpackhi_epi64(t0v7, t0v3));\
    _mm_storeu_si128((__m128i*)&((dest[8])), _mm_unpacklo_epi64(t0v5, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[24])), _mm_unpackhi_epi64(t0v5, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[9])), _mm_unpacklo_epi64(t0v20, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[25])), _mm_unpackhi_epi64(t0v20, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[10])), _mm_unpacklo_epi64(t0v9, t0v15));\
    _mm_storeu_si128((__m128i*)&((dest[26])), _mm_unpackhi_epi64(t0v9, t0v15));\
    _mm_storeu_si128((__m128i*)&((dest[11])), _mm_unpacklo_epi64(t0v21, t0v27));\
    _mm_storeu_si128((__m128i*)&((dest[27])), _mm_unpackhi_epi64(t0v21, t0v27));\
    _mm_storeu_si128((__m128i*)&((dest[12])), _mm_unpacklo_epi64(t0v10, t0v16));\
    _mm_storeu_si128((__m128i*)&((dest[28])), _mm_unpackhi_epi64(t0v10, t0v16));\
    _mm_storeu_si128((__m128i*)&((dest[13])), _mm_unpacklo_epi64(t0v22, t0v28));\
    _mm_storeu_si128((__m128i*)&((dest[29])), _mm_unpackhi_epi64(t0v22, t0v28));\
    _mm_storeu_si128((__m128i*)&((dest[14])), _mm_unpacklo_epi64(t0v6, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[30])), _mm_unpackhi_epi64(t0v6, t0v4));\
    _mm_storeu_si128((__m128i*)&((dest[15])), _mm_unpacklo_epi64(t0v2, t0v0));\
    _mm_storeu_si128((__m128i*)&((dest[31])), _mm_unpackhi_epi64(t0v2, t0v0));\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    __m128i* dest = (__m128i*)(D);\
    const __m128i c0 = (*(const __m128i*)(transform_const2_tbl + 4*0));\
    const __m128i c1 = (*(const __m128i*)(transform_const_tbl + 4*10));\
    const __m128i c2 = (*(const __m128i*)(transform_const_tbl + 4*11));\
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
    t0v0 = _mm_and_si128((S0), c0);\
    t0v1 = _mm_and_si128(_mm_srli_epi32((S0), 1), c0);\
    t0v2 = _mm_and_si128(_mm_srli_epi32((S0), 2), c0);\
    t0v3 = _mm_and_si128(_mm_srli_epi32((S0), 3), c0);\
    t0v4 = _mm_and_si128(_mm_srli_epi32((S0), 4), c0);\
    t0v5 = _mm_and_si128(_mm_srli_epi32((S0), 5), c0);\
    t0v6 = _mm_and_si128(_mm_srli_epi32((S0), 6), c0);\
    t0v7 = _mm_and_si128(_mm_srli_epi32((S0), 7), c0);\
    t0v8 = _mm_and_si128(_mm_srli_epi32((S0), 8), c0);\
    t0v9 = _mm_and_si128(_mm_srli_epi32((S0), 9), c0);\
    t0v10 = _mm_and_si128(_mm_srli_epi32((S0), 10), c0);\
    t0v11 = _mm_and_si128(_mm_srli_epi32((S0), 11), c0);\
    t0v12 = _mm_and_si128(_mm_srli_epi32((S0), 12), c0);\
    t0v13 = _mm_and_si128(_mm_srli_epi32((S0), 13), c0);\
    t0v14 = _mm_and_si128(_mm_srli_epi32((S0), 14), c0);\
    t0v15 = _mm_and_si128(_mm_srli_epi32((S0), 15), c0);\
    t0v16 = _mm_and_si128(_mm_srli_epi32((S0), 16), c0);\
    t0v17 = _mm_and_si128(_mm_srli_epi32((S0), 17), c0);\
    t0v18 = _mm_and_si128(_mm_srli_epi32((S0), 18), c0);\
    t0v19 = _mm_and_si128(_mm_srli_epi32((S0), 19), c0);\
    t0v20 = _mm_and_si128(_mm_srli_epi32((S0), 20), c0);\
    t0v21 = _mm_and_si128(_mm_srli_epi32((S0), 21), c0);\
    t0v22 = _mm_and_si128(_mm_srli_epi32((S0), 22), c0);\
    t0v23 = _mm_and_si128(_mm_srli_epi32((S0), 23), c0);\
    t0v24 = _mm_and_si128(_mm_srli_epi32((S0), 24), c0);\
    t0v25 = _mm_and_si128(_mm_srli_epi32((S0), 25), c0);\
    t0v26 = _mm_and_si128(_mm_srli_epi32((S0), 26), c0);\
    t0v27 = _mm_and_si128(_mm_srli_epi32((S0), 27), c0);\
    t0v28 = _mm_and_si128(_mm_srli_epi32((S0), 28), c0);\
    t0v29 = _mm_and_si128(_mm_srli_epi32((S0), 29), c0);\
    t0v30 = _mm_and_si128(_mm_srli_epi32((S0), 30), c0);\
    t0v31 = _mm_srli_epi32((S0), 31);\
    t0v32 = _mm_or_si128(_mm_and_si128(t0v0, c1), _mm_slli_epi64(t0v1, 32));\
    t0v0 = _mm_or_si128(_mm_srli_epi64(t0v0, 32), _mm_and_si128(t0v1, c2));\
    t0v1 = _mm_or_si128(_mm_and_si128(t0v2, c1), _mm_slli_epi64(t0v3, 32));\
    t0v2 = _mm_or_si128(_mm_srli_epi64(t0v2, 32), _mm_and_si128(t0v3, c2));\
    t0v3 = _mm_or_si128(_mm_and_si128(t0v4, c1), _mm_slli_epi64(t0v5, 32));\
    t0v4 = _mm_or_si128(_mm_srli_epi64(t0v4, 32), _mm_and_si128(t0v5, c2));\
    t0v5 = _mm_or_si128(_mm_and_si128(t0v6, c1), _mm_slli_epi64(t0v7, 32));\
    t0v6 = _mm_or_si128(_mm_srli_epi64(t0v6, 32), _mm_and_si128(t0v7, c2));\
    t0v7 = _mm_or_si128(_mm_and_si128(t0v8, c1), _mm_slli_epi64(t0v9, 32));\
    t0v8 = _mm_or_si128(_mm_srli_epi64(t0v8, 32), _mm_and_si128(t0v9, c2));\
    t0v9 = _mm_or_si128(_mm_and_si128(t0v10, c1), _mm_slli_epi64(t0v11, 32));\
    t0v10 = _mm_or_si128(_mm_srli_epi64(t0v10, 32), _mm_and_si128(t0v11, c2));\
    t0v11 = _mm_or_si128(_mm_and_si128(t0v12, c1), _mm_slli_epi64(t0v13, 32));\
    t0v12 = _mm_or_si128(_mm_srli_epi64(t0v12, 32), _mm_and_si128(t0v13, c2));\
    t0v13 = _mm_or_si128(_mm_and_si128(t0v14, c1), _mm_slli_epi64(t0v15, 32));\
    t0v14 = _mm_or_si128(_mm_srli_epi64(t0v14, 32), _mm_and_si128(t0v15, c2));\
    t0v15 = _mm_or_si128(_mm_and_si128(t0v16, c1), _mm_slli_epi64(t0v17, 32));\
    t0v16 = _mm_or_si128(_mm_srli_epi64(t0v16, 32), _mm_and_si128(t0v17, c2));\
    t0v17 = _mm_or_si128(_mm_and_si128(t0v18, c1), _mm_slli_epi64(t0v19, 32));\
    t0v18 = _mm_or_si128(_mm_srli_epi64(t0v18, 32), _mm_and_si128(t0v19, c2));\
    t0v19 = _mm_or_si128(_mm_and_si128(t0v20, c1), _mm_slli_epi64(t0v21, 32));\
    t0v20 = _mm_or_si128(_mm_srli_epi64(t0v20, 32), _mm_and_si128(t0v21, c2));\
    t0v21 = _mm_or_si128(_mm_and_si128(t0v22, c1), _mm_slli_epi64(t0v23, 32));\
    t0v22 = _mm_or_si128(_mm_srli_epi64(t0v22, 32), _mm_and_si128(t0v23, c2));\
    t0v23 = _mm_or_si128(_mm_and_si128(t0v24, c1), _mm_slli_epi64(t0v25, 32));\
    t0v24 = _mm_or_si128(_mm_srli_epi64(t0v24, 32), _mm_and_si128(t0v25, c2));\
    t0v25 = _mm_or_si128(_mm_and_si128(t0v26, c1), _mm_slli_epi64(t0v27, 32));\
    t0v26 = _mm_or_si128(_mm_srli_epi64(t0v26, 32), _mm_and_si128(t0v27, c2));\
    t0v27 = _mm_or_si128(_mm_and_si128(t0v28, c1), _mm_slli_epi64(t0v29, 32));\
    t0v28 = _mm_or_si128(_mm_srli_epi64(t0v28, 32), _mm_and_si128(t0v29, c2));\
    t0v29 = _mm_or_si128(_mm_and_si128(t0v30, c1), _mm_slli_epi64(t0v31, 32));\
    t0v30 = _mm_or_si128(_mm_srli_epi64(t0v30, 32), _mm_and_si128(t0v31, c2));\
    _mm_storeu_si128((__m128i*)&((dest[0])), _mm_unpacklo_epi64(t0v32, t0v1));\
    _mm_storeu_si128((__m128i*)&((dest[16])), _mm_unpackhi_epi64(t0v32, t0v1));\
    _mm_storeu_si128((__m128i*)&((dest[1])), _mm_unpacklo_epi64(t0v3, t0v5));\
    _mm_storeu_si128((__m128i*)&((dest[17])), _mm_unpackhi_epi64(t0v3, t0v5));\
    _mm_storeu_si128((__m128i*)&((dest[2])), _mm_unpacklo_epi64(t0v7, t0v9));\
    _mm_storeu_si128((__m128i*)&((dest[18])), _mm_unpackhi_epi64(t0v7, t0v9));\
    _mm_storeu_si128((__m128i*)&((dest[3])), _mm_unpacklo_epi64(t0v11, t0v13));\
    _mm_storeu_si128((__m128i*)&((dest[19])), _mm_unpackhi_epi64(t0v11, t0v13));\
    _mm_storeu_si128((__m128i*)&((dest[4])), _mm_unpacklo_epi64(t0v15, t0v17));\
    _mm_storeu_si128((__m128i*)&((dest[20])), _mm_unpackhi_epi64(t0v15, t0v17));\
    _mm_storeu_si128((__m128i*)&((dest[5])), _mm_unpacklo_epi64(t0v19, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[21])), _mm_unpackhi_epi64(t0v19, t0v21));\
    _mm_storeu_si128((__m128i*)&((dest[6])), _mm_unpacklo_epi64(t0v23, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[22])), _mm_unpackhi_epi64(t0v23, t0v25));\
    _mm_storeu_si128((__m128i*)&((dest[7])), _mm_unpacklo_epi64(t0v27, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[23])), _mm_unpackhi_epi64(t0v27, t0v29));\
    _mm_storeu_si128((__m128i*)&((dest[8])), _mm_unpacklo_epi64(t0v0, t0v2));\
    _mm_storeu_si128((__m128i*)&((dest[24])), _mm_unpackhi_epi64(t0v0, t0v2));\
    _mm_storeu_si128((__m128i*)&((dest[9])), _mm_unpacklo_epi64(t0v4, t0v6));\
    _mm_storeu_si128((__m128i*)&((dest[25])), _mm_unpackhi_epi64(t0v4, t0v6));\
    _mm_storeu_si128((__m128i*)&((dest[10])), _mm_unpacklo_epi64(t0v8, t0v10));\
    _mm_storeu_si128((__m128i*)&((dest[26])), _mm_unpackhi_epi64(t0v8, t0v10));\
    _mm_storeu_si128((__m128i*)&((dest[11])), _mm_unpacklo_epi64(t0v12, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[27])), _mm_unpackhi_epi64(t0v12, t0v14));\
    _mm_storeu_si128((__m128i*)&((dest[12])), _mm_unpacklo_epi64(t0v16, t0v18));\
    _mm_storeu_si128((__m128i*)&((dest[28])), _mm_unpackhi_epi64(t0v16, t0v18));\
    _mm_storeu_si128((__m128i*)&((dest[13])), _mm_unpacklo_epi64(t0v20, t0v22));\
    _mm_storeu_si128((__m128i*)&((dest[29])), _mm_unpackhi_epi64(t0v20, t0v22));\
    _mm_storeu_si128((__m128i*)&((dest[14])), _mm_unpacklo_epi64(t0v24, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[30])), _mm_unpackhi_epi64(t0v24, t0v26));\
    _mm_storeu_si128((__m128i*)&((dest[15])), _mm_unpacklo_epi64(t0v28, t0v30));\
    _mm_storeu_si128((__m128i*)&((dest[31])), _mm_unpackhi_epi64(t0v28, t0v30));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_AVX2.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    __m256i* dest = (__m256i*)(D);\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c8 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c9 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c2 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*5));\
    const __m256i c6 = (*(const __m256i*)(transform_const_tbl + 8*6));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*7));\
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
    t0v0 = _mm256_or_si256(_mm256_and_si256((S0), c0), _mm256_slli_epi16(_mm256_and_si256((S1), c0), 1));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S0), c1), 1), _mm256_and_si256((S1), c1));\
    t0v2 = _mm256_or_si256(_mm256_and_si256((S2), c0), _mm256_slli_epi16(_mm256_and_si256((S3), c0), 1));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S2), c1), 1), _mm256_and_si256((S3), c1));\
    t0v4 = _mm256_or_si256(_mm256_and_si256((S4), c0), _mm256_slli_epi16(_mm256_and_si256((S5), c0), 1));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S4), c1), 1), _mm256_and_si256((S5), c1));\
    t0v6 = _mm256_or_si256(_mm256_and_si256((S6), c0), _mm256_slli_epi16(_mm256_and_si256((S7), c0), 1));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S6), c1), 1), _mm256_and_si256((S7), c1));\
    t0v8 = _mm256_or_si256(_mm256_and_si256((S8), c0), _mm256_slli_epi16(_mm256_and_si256((S9), c0), 1));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S8), c1), 1), _mm256_and_si256((S9), c1));\
    t0v10 = _mm256_or_si256(_mm256_and_si256((S10), c0), _mm256_slli_epi16(_mm256_and_si256((S11), c0), 1));\
    t0v11 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S10), c1), 1), _mm256_and_si256((S11), c1));\
    t0v12 = _mm256_or_si256(_mm256_and_si256((S12), c0), _mm256_slli_epi16(_mm256_and_si256((S13), c0), 1));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S12), c1), 1), _mm256_and_si256((S13), c1));\
    t0v14 = _mm256_or_si256(_mm256_and_si256((S14), c0), _mm256_slli_epi16(_mm256_and_si256((S15), c0), 1));\
    t0v15 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S14), c1), 1), _mm256_and_si256((S15), c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256((S16), c0), _mm256_slli_epi16(_mm256_and_si256((S17), c0), 1));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S16), c1), 1), _mm256_and_si256((S17), c1));\
    t0v18 = _mm256_or_si256(_mm256_and_si256((S18), c0), _mm256_slli_epi16(_mm256_and_si256((S19), c0), 1));\
    t0v19 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S18), c1), 1), _mm256_and_si256((S19), c1));\
    t0v20 = _mm256_or_si256(_mm256_and_si256((S20), c0), _mm256_slli_epi16(_mm256_and_si256((S21), c0), 1));\
    t0v21 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S20), c1), 1), _mm256_and_si256((S21), c1));\
    t0v22 = _mm256_or_si256(_mm256_and_si256((S22), c0), _mm256_slli_epi16(_mm256_and_si256((S23), c0), 1));\
    t0v23 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S22), c1), 1), _mm256_and_si256((S23), c1));\
    t0v24 = _mm256_or_si256(_mm256_and_si256((S24), c0), _mm256_slli_epi16(_mm256_and_si256((S25), c0), 1));\
    t0v25 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S24), c1), 1), _mm256_and_si256((S25), c1));\
    t0v26 = _mm256_or_si256(_mm256_and_si256((S26), c0), _mm256_slli_epi16(_mm256_and_si256((S27), c0), 1));\
    t0v27 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S26), c1), 1), _mm256_and_si256((S27), c1));\
    t0v28 = _mm256_or_si256(_mm256_and_si256((S28), c0), _mm256_slli_epi16(_mm256_and_si256((S29), c0), 1));\
    t0v29 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S28), c1), 1), _mm256_and_si256((S29), c1));\
    t0v30 = _mm256_or_si256(_mm256_and_si256((S30), c0), _mm256_slli_epi16(_mm256_and_si256((S31), c0), 1));\
    t0v31 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S30), c1), 1), _mm256_and_si256((S31), c1));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi16(_mm256_and_si256(t0v2, c2), 2));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c3), 2), _mm256_and_si256(t0v2, c3));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi16(_mm256_and_si256(t0v3, c2), 2));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c3), 2), _mm256_and_si256(t0v3, c3));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v4, c2), _mm256_slli_epi16(_mm256_and_si256(t0v6, c2), 2));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c3), 2), _mm256_and_si256(t0v6, c3));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v5, c2), _mm256_slli_epi16(_mm256_and_si256(t0v7, c2), 2));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c3), 2), _mm256_and_si256(t0v7, c3));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v8, c2), _mm256_slli_epi16(_mm256_and_si256(t0v10, c2), 2));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c3), 2), _mm256_and_si256(t0v10, c3));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v9, c2), _mm256_slli_epi16(_mm256_and_si256(t0v11, c2), 2));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c3), 2), _mm256_and_si256(t0v11, c3));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v12, c2), _mm256_slli_epi16(_mm256_and_si256(t0v14, c2), 2));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v12, c3), 2), _mm256_and_si256(t0v14, c3));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v13, c2), _mm256_slli_epi16(_mm256_and_si256(t0v15, c2), 2));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c3), 2), _mm256_and_si256(t0v15, c3));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v16, c2), _mm256_slli_epi16(_mm256_and_si256(t0v18, c2), 2));\
    t0v16 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c3), 2), _mm256_and_si256(t0v18, c3));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v17, c2), _mm256_slli_epi16(_mm256_and_si256(t0v19, c2), 2));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v17, c3), 2), _mm256_and_si256(t0v19, c3));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v20, c2), _mm256_slli_epi16(_mm256_and_si256(t0v22, c2), 2));\
    t0v20 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v20, c3), 2), _mm256_and_si256(t0v22, c3));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v21, c2), _mm256_slli_epi16(_mm256_and_si256(t0v23, c2), 2));\
    t0v21 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v21, c3), 2), _mm256_and_si256(t0v23, c3));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v24, c2), _mm256_slli_epi16(_mm256_and_si256(t0v26, c2), 2));\
    t0v24 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v24, c3), 2), _mm256_and_si256(t0v26, c3));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v25, c2), _mm256_slli_epi16(_mm256_and_si256(t0v27, c2), 2));\
    t0v25 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v25, c3), 2), _mm256_and_si256(t0v27, c3));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v28, c2), _mm256_slli_epi16(_mm256_and_si256(t0v30, c2), 2));\
    t0v28 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v28, c3), 2), _mm256_and_si256(t0v30, c3));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v29, c2), _mm256_slli_epi16(_mm256_and_si256(t0v31, c2), 2));\
    t0v29 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v29, c3), 2), _mm256_and_si256(t0v31, c3));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v32, c4), _mm256_slli_epi16(_mm256_and_si256(t0v3, c4), 4));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v32, c5), 4), _mm256_and_si256(t0v3, c5));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v2, c4), _mm256_slli_epi16(_mm256_and_si256(t0v6, c4), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c5), 4), _mm256_and_si256(t0v6, c5));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v0, c4), _mm256_slli_epi16(_mm256_and_si256(t0v4, c4), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c5), 4), _mm256_and_si256(t0v4, c5));\
    t0v4 = _mm256_or_si256(_mm256_and_si256(t0v1, c4), _mm256_slli_epi16(_mm256_and_si256(t0v5, c4), 4));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c5), 4), _mm256_and_si256(t0v5, c5));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v7, c4), _mm256_slli_epi16(_mm256_and_si256(t0v11, c4), 4));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c5), 4), _mm256_and_si256(t0v11, c5));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v10, c4), _mm256_slli_epi16(_mm256_and_si256(t0v14, c4), 4));\
    t0v10 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v10, c5), 4), _mm256_and_si256(t0v14, c5));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v8, c4), _mm256_slli_epi16(_mm256_and_si256(t0v12, c4), 4));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c5), 4), _mm256_and_si256(t0v12, c5));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v9, c4), _mm256_slli_epi16(_mm256_and_si256(t0v13, c4), 4));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c5), 4), _mm256_and_si256(t0v13, c5));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v15, c4), _mm256_slli_epi16(_mm256_and_si256(t0v19, c4), 4));\
    t0v15 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v15, c5), 4), _mm256_and_si256(t0v19, c5));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v18, c4), _mm256_slli_epi16(_mm256_and_si256(t0v22, c4), 4));\
    t0v18 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v18, c5), 4), _mm256_and_si256(t0v22, c5));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v16, c4), _mm256_slli_epi16(_mm256_and_si256(t0v20, c4), 4));\
    t0v16 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c5), 4), _mm256_and_si256(t0v20, c5));\
    t0v20 = _mm256_or_si256(_mm256_and_si256(t0v17, c4), _mm256_slli_epi16(_mm256_and_si256(t0v21, c4), 4));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v17, c5), 4), _mm256_and_si256(t0v21, c5));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v23, c4), _mm256_slli_epi16(_mm256_and_si256(t0v27, c4), 4));\
    t0v23 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v23, c5), 4), _mm256_and_si256(t0v27, c5));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v26, c4), _mm256_slli_epi16(_mm256_and_si256(t0v30, c4), 4));\
    t0v26 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v26, c5), 4), _mm256_and_si256(t0v30, c5));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v24, c4), _mm256_slli_epi16(_mm256_and_si256(t0v28, c4), 4));\
    t0v24 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v24, c5), 4), _mm256_and_si256(t0v28, c5));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v25, c4), _mm256_slli_epi16(_mm256_and_si256(t0v29, c4), 4));\
    t0v25 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v25, c5), 4), _mm256_and_si256(t0v29, c5));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v31, c6), _mm256_slli_epi16(t0v5, 8));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(t0v31, 8), _mm256_and_si256(t0v5, c7));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v32, c6), _mm256_slli_epi16(t0v11, 8));\
    t0v11 = _mm256_or_si256(_mm256_srli_epi16(t0v32, 8), _mm256_and_si256(t0v11, c7));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v6, c6), _mm256_slli_epi16(t0v14, 8));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(t0v6, 8), _mm256_and_si256(t0v14, c7));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v4, c6), _mm256_slli_epi16(t0v12, 8));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(t0v4, 8), _mm256_and_si256(t0v12, c7));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v3, c6), _mm256_slli_epi16(t0v7, 8));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(t0v3, 8), _mm256_and_si256(t0v7, c7));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v2, c6), _mm256_slli_epi16(t0v10, 8));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(t0v2, 8), _mm256_and_si256(t0v10, c7));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v0, c6), _mm256_slli_epi16(t0v8, 8));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(t0v0, 8), _mm256_and_si256(t0v8, c7));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v1, c6), _mm256_slli_epi16(t0v9, 8));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(t0v1, 8), _mm256_and_si256(t0v9, c7));\
    t0v9 = _mm256_or_si256(_mm256_and_si256(t0v13, c6), _mm256_slli_epi16(t0v21, 8));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(t0v13, 8), _mm256_and_si256(t0v21, c7));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v19, c6), _mm256_slli_epi16(t0v27, 8));\
    t0v19 = _mm256_or_si256(_mm256_srli_epi16(t0v19, 8), _mm256_and_si256(t0v27, c7));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v22, c6), _mm256_slli_epi16(t0v30, 8));\
    t0v22 = _mm256_or_si256(_mm256_srli_epi16(t0v22, 8), _mm256_and_si256(t0v30, c7));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v20, c6), _mm256_slli_epi16(t0v28, 8));\
    t0v20 = _mm256_or_si256(_mm256_srli_epi16(t0v20, 8), _mm256_and_si256(t0v28, c7));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v15, c6), _mm256_slli_epi16(t0v23, 8));\
    t0v15 = _mm256_or_si256(_mm256_srli_epi16(t0v15, 8), _mm256_and_si256(t0v23, c7));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v18, c6), _mm256_slli_epi16(t0v26, 8));\
    t0v18 = _mm256_or_si256(_mm256_srli_epi16(t0v18, 8), _mm256_and_si256(t0v26, c7));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v16, c6), _mm256_slli_epi16(t0v24, 8));\
    t0v16 = _mm256_or_si256(_mm256_srli_epi16(t0v16, 8), _mm256_and_si256(t0v24, c7));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v17, c6), _mm256_slli_epi16(t0v25, 8));\
    t0v17 = _mm256_or_si256(_mm256_srli_epi16(t0v17, 8), _mm256_and_si256(t0v25, c7));\
    t0v25 = _mm256_blend_epi16(t0v29, _mm256_slli_epi32(t0v9, 16), 0xaa);\
    t0v9 = _mm256_blend_epi16(_mm256_srli_epi32(t0v29, 16), t0v9, 0xaa);\
    t0v29 = _mm256_blend_epi16(t0v31, _mm256_slli_epi32(t0v21, 16), 0xaa);\
    t0v21 = _mm256_blend_epi16(_mm256_srli_epi32(t0v31, 16), t0v21, 0xaa);\
    t0v31 = _mm256_blend_epi16(t0v32, _mm256_slli_epi32(t0v27, 16), 0xaa);\
    t0v27 = _mm256_blend_epi16(_mm256_srli_epi32(t0v32, 16), t0v27, 0xaa);\
    t0v32 = _mm256_blend_epi16(t0v14, _mm256_slli_epi32(t0v30, 16), 0xaa);\
    t0v14 = _mm256_blend_epi16(_mm256_srli_epi32(t0v14, 16), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi16(t0v12, _mm256_slli_epi32(t0v28, 16), 0xaa);\
    t0v12 = _mm256_blend_epi16(_mm256_srli_epi32(t0v12, 16), t0v28, 0xaa);\
    t0v28 = _mm256_blend_epi16(t0v7, _mm256_slli_epi32(t0v23, 16), 0xaa);\
    t0v7 = _mm256_blend_epi16(_mm256_srli_epi32(t0v7, 16), t0v23, 0xaa);\
    t0v23 = _mm256_blend_epi16(t0v10, _mm256_slli_epi32(t0v26, 16), 0xaa);\
    t0v10 = _mm256_blend_epi16(_mm256_srli_epi32(t0v10, 16), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi16(t0v8, _mm256_slli_epi32(t0v24, 16), 0xaa);\
    t0v8 = _mm256_blend_epi16(_mm256_srli_epi32(t0v8, 16), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi16(t0v5, _mm256_slli_epi32(t0v13, 16), 0xaa);\
    t0v5 = _mm256_blend_epi16(_mm256_srli_epi32(t0v5, 16), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi16(t0v11, _mm256_slli_epi32(t0v19, 16), 0xaa);\
    t0v11 = _mm256_blend_epi16(_mm256_srli_epi32(t0v11, 16), t0v19, 0xaa);\
    t0v19 = _mm256_blend_epi16(t0v6, _mm256_slli_epi32(t0v22, 16), 0xaa);\
    t0v6 = _mm256_blend_epi16(_mm256_srli_epi32(t0v6, 16), t0v22, 0xaa);\
    t0v22 = _mm256_blend_epi16(t0v4, _mm256_slli_epi32(t0v20, 16), 0xaa);\
    t0v4 = _mm256_blend_epi16(_mm256_srli_epi32(t0v4, 16), t0v20, 0xaa);\
    t0v20 = _mm256_blend_epi16(t0v3, _mm256_slli_epi32(t0v15, 16), 0xaa);\
    t0v3 = _mm256_blend_epi16(_mm256_srli_epi32(t0v3, 16), t0v15, 0xaa);\
    t0v15 = _mm256_blend_epi16(t0v2, _mm256_slli_epi32(t0v18, 16), 0xaa);\
    t0v2 = _mm256_blend_epi16(_mm256_srli_epi32(t0v2, 16), t0v18, 0xaa);\
    t0v18 = _mm256_blend_epi16(t0v0, _mm256_slli_epi32(t0v16, 16), 0xaa);\
    t0v0 = _mm256_blend_epi16(_mm256_srli_epi32(t0v0, 16), t0v16, 0xaa);\
    t0v16 = _mm256_blend_epi16(t0v1, _mm256_slli_epi32(t0v17, 16), 0xaa);\
    t0v1 = _mm256_blend_epi16(_mm256_srli_epi32(t0v1, 16), t0v17, 0xaa);\
    t0v17 = _mm256_blend_epi32(t0v25, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v25 = _mm256_blend_epi32(_mm256_srli_epi64(t0v25, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v31, _mm256_slli_epi64(t0v32, 32), 0xaa);\
    t0v31 = _mm256_blend_epi32(_mm256_srli_epi64(t0v31, 32), t0v32, 0xaa);\
    t0v32 = _mm256_blend_epi32(t0v30, _mm256_slli_epi64(t0v28, 32), 0xaa);\
    t0v28 = _mm256_blend_epi32(_mm256_srli_epi64(t0v30, 32), t0v28, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v23, _mm256_slli_epi64(t0v26, 32), 0xaa);\
    t0v23 = _mm256_blend_epi32(_mm256_srli_epi64(t0v23, 32), t0v26, 0xaa);\
    t0v26 = _mm256_blend_epi32(t0v24, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v13 = _mm256_blend_epi32(_mm256_srli_epi64(t0v24, 32), t0v13, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v19, _mm256_slli_epi64(t0v22, 32), 0xaa);\
    t0v19 = _mm256_blend_epi32(_mm256_srli_epi64(t0v19, 32), t0v22, 0xaa);\
    t0v22 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v15 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v15, 0xaa);\
    t0v20 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v16, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v16, 0xaa);\
    t0v18 = _mm256_blend_epi32(t0v9, _mm256_slli_epi64(t0v21, 32), 0xaa);\
    t0v9 = _mm256_blend_epi32(_mm256_srli_epi64(t0v9, 32), t0v21, 0xaa);\
    t0v21 = _mm256_blend_epi32(t0v27, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v14 = _mm256_blend_epi32(_mm256_srli_epi64(t0v27, 32), t0v14, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v12, _mm256_slli_epi64(t0v7, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v12, 32), t0v7, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v10, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v8 = _mm256_blend_epi32(_mm256_srli_epi64(t0v10, 32), t0v8, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v4, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v4, 0xaa);\
    t0v6 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v2, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v2, 0xaa);\
    t0v3 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v1, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v1, 0xaa);\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v17, c8), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v17 = _mm256_or_si256(_mm256_srli_si256(t0v17, (64)>>3), _mm256_and_si256(t0v29, c9));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v32, c8), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v30 = _mm256_or_si256(_mm256_srli_si256(t0v32, (64)>>3), _mm256_and_si256(t0v30, c9));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v26, c8), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v24 = _mm256_or_si256(_mm256_srli_si256(t0v26, (64)>>3), _mm256_and_si256(t0v24, c9));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v22, c8), _mm256_slli_si256(t0v20, (64)>>3));\
    t0v20 = _mm256_or_si256(_mm256_srli_si256(t0v22, (64)>>3), _mm256_and_si256(t0v20, c9));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v18, c8), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v18 = _mm256_or_si256(_mm256_srli_si256(t0v18, (64)>>3), _mm256_and_si256(t0v21, c9));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v27, c8), _mm256_slli_si256(t0v12, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v27, (64)>>3), _mm256_and_si256(t0v12, c9));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v10, c8), _mm256_slli_si256(t0v11, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v11, c9));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v6, c8), _mm256_slli_si256(t0v3, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v3, c9));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v25, c8), _mm256_slli_si256(t0v31, (64)>>3));\
    t0v25 = _mm256_or_si256(_mm256_srli_si256(t0v25, (64)>>3), _mm256_and_si256(t0v31, c9));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v28, c8), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v23 = _mm256_or_si256(_mm256_srli_si256(t0v28, (64)>>3), _mm256_and_si256(t0v23, c9));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v13, c8), _mm256_slli_si256(t0v19, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v19, c9));\
    t0v19 = _mm256_or_si256(_mm256_and_si256(t0v15, c8), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v16, c9));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v9, c8), _mm256_slli_si256(t0v14, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v14, c9));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v7, c8), _mm256_slli_si256(t0v8, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v8, c9));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v5, c8), _mm256_slli_si256(t0v4, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v4, c9));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v2, c8), _mm256_slli_si256(t0v0, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v0, c9));\
    _mm256_storeu_si256((__m256i*)&((dest[0])), _mm256_permute2x128_si256(t0v1, t0v29, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[16])), _mm256_permute2x128_si256(t0v1, t0v29, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[1])), _mm256_permute2x128_si256(t0v32, t0v26, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[17])), _mm256_permute2x128_si256(t0v32, t0v26, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[2])), _mm256_permute2x128_si256(t0v22, t0v21, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[18])), _mm256_permute2x128_si256(t0v22, t0v21, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[3])), _mm256_permute2x128_si256(t0v27, t0v11, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[19])), _mm256_permute2x128_si256(t0v27, t0v11, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[4])), _mm256_permute2x128_si256(t0v6, t0v31, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[20])), _mm256_permute2x128_si256(t0v6, t0v31, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[5])), _mm256_permute2x128_si256(t0v28, t0v19, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[21])), _mm256_permute2x128_si256(t0v28, t0v19, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[6])), _mm256_permute2x128_si256(t0v16, t0v14, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[22])), _mm256_permute2x128_si256(t0v16, t0v14, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[7])), _mm256_permute2x128_si256(t0v8, t0v5, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[23])), _mm256_permute2x128_si256(t0v8, t0v5, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[8])), _mm256_permute2x128_si256(t0v17, t0v30, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[24])), _mm256_permute2x128_si256(t0v17, t0v30, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[9])), _mm256_permute2x128_si256(t0v24, t0v20, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[25])), _mm256_permute2x128_si256(t0v24, t0v20, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[10])), _mm256_permute2x128_si256(t0v18, t0v12, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[26])), _mm256_permute2x128_si256(t0v18, t0v12, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[11])), _mm256_permute2x128_si256(t0v10, t0v3, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[27])), _mm256_permute2x128_si256(t0v10, t0v3, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[12])), _mm256_permute2x128_si256(t0v25, t0v23, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[28])), _mm256_permute2x128_si256(t0v25, t0v23, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[13])), _mm256_permute2x128_si256(t0v13, t0v15, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[29])), _mm256_permute2x128_si256(t0v13, t0v15, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[14])), _mm256_permute2x128_si256(t0v9, t0v7, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[30])), _mm256_permute2x128_si256(t0v9, t0v7, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[15])), _mm256_permute2x128_si256(t0v4, t0v0, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[31])), _mm256_permute2x128_si256(t0v4, t0v0, 0x31));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    __m256i* dest = (__m256i*)(D);\
    const __m256i c8 = (*(const __m256i*)(transform_const2_tbl + 8*4));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c9 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c10 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c2 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*5));\
    const __m256i c6 = (*(const __m256i*)(transform_const_tbl + 8*6));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*7));\
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
    t0v0 = _mm256_or_si256(_mm256_and_si256((S0), c0), _mm256_slli_epi16(_mm256_and_si256((S1), c0), 1));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S0), c1), 1), _mm256_and_si256((S1), c1));\
    t0v2 = _mm256_or_si256(_mm256_and_si256((S2), c0), _mm256_slli_epi16(_mm256_and_si256((S3), c0), 1));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S2), c1), 1), _mm256_and_si256((S3), c1));\
    t0v4 = _mm256_or_si256(_mm256_and_si256((S4), c0), _mm256_slli_epi16(_mm256_and_si256((S5), c0), 1));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S4), c1), 1), _mm256_and_si256((S5), c1));\
    t0v6 = _mm256_or_si256(_mm256_and_si256((S6), c0), _mm256_slli_epi16(_mm256_and_si256((S7), c0), 1));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S6), c1), 1), _mm256_and_si256((S7), c1));\
    t0v8 = _mm256_or_si256(_mm256_and_si256((S8), c0), _mm256_slli_epi16(_mm256_and_si256((S9), c0), 1));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S8), c1), 1), _mm256_and_si256((S9), c1));\
    t0v10 = _mm256_or_si256(_mm256_and_si256((S10), c0), _mm256_slli_epi16(_mm256_and_si256((S11), c0), 1));\
    t0v11 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S10), c1), 1), _mm256_and_si256((S11), c1));\
    t0v12 = _mm256_or_si256(_mm256_and_si256((S12), c0), _mm256_slli_epi16(_mm256_and_si256((S13), c0), 1));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S12), c1), 1), _mm256_and_si256((S13), c1));\
    t0v14 = _mm256_or_si256(_mm256_and_si256((S14), c0), _mm256_slli_epi16(_mm256_and_si256((S15), c0), 1));\
    t0v15 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S14), c1), 1), _mm256_and_si256((S15), c1));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi16(_mm256_and_si256(t0v2, c2), 2));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c3), 2), _mm256_and_si256(t0v2, c3));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi16(_mm256_and_si256(t0v3, c2), 2));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c3), 2), _mm256_and_si256(t0v3, c3));\
    t0v3 = _mm256_or_si256(_mm256_and_si256(t0v4, c2), _mm256_slli_epi16(_mm256_and_si256(t0v6, c2), 2));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v4, c3), 2), _mm256_and_si256(t0v6, c3));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v5, c2), _mm256_slli_epi16(_mm256_and_si256(t0v7, c2), 2));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v5, c3), 2), _mm256_and_si256(t0v7, c3));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v8, c2), _mm256_slli_epi16(_mm256_and_si256(t0v10, c2), 2));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c3), 2), _mm256_and_si256(t0v10, c3));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v9, c2), _mm256_slli_epi16(_mm256_and_si256(t0v11, c2), 2));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c3), 2), _mm256_and_si256(t0v11, c3));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v12, c2), _mm256_slli_epi16(_mm256_and_si256(t0v14, c2), 2));\
    t0v12 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v12, c3), 2), _mm256_and_si256(t0v14, c3));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v13, c2), _mm256_slli_epi16(_mm256_and_si256(t0v15, c2), 2));\
    t0v13 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v13, c3), 2), _mm256_and_si256(t0v15, c3));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v16, c4), _mm256_slli_epi16(_mm256_and_si256(t0v3, c4), 4));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v16, c5), 4), _mm256_and_si256(t0v3, c5));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v2, c4), _mm256_slli_epi16(_mm256_and_si256(t0v6, c4), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c5), 4), _mm256_and_si256(t0v6, c5));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v0, c4), _mm256_slli_epi16(_mm256_and_si256(t0v4, c4), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c5), 4), _mm256_and_si256(t0v4, c5));\
    t0v4 = _mm256_or_si256(_mm256_and_si256(t0v1, c4), _mm256_slli_epi16(_mm256_and_si256(t0v5, c4), 4));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c5), 4), _mm256_and_si256(t0v5, c5));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v7, c4), _mm256_slli_epi16(_mm256_and_si256(t0v11, c4), 4));\
    t0v7 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v7, c5), 4), _mm256_and_si256(t0v11, c5));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v10, c4), _mm256_slli_epi16(_mm256_and_si256(t0v14, c4), 4));\
    t0v10 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v10, c5), 4), _mm256_and_si256(t0v14, c5));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v8, c4), _mm256_slli_epi16(_mm256_and_si256(t0v12, c4), 4));\
    t0v8 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v8, c5), 4), _mm256_and_si256(t0v12, c5));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v9, c4), _mm256_slli_epi16(_mm256_and_si256(t0v13, c4), 4));\
    t0v9 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v9, c5), 4), _mm256_and_si256(t0v13, c5));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v15, c6), _mm256_slli_epi16(t0v5, 8));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(t0v15, 8), _mm256_and_si256(t0v5, c7));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v16, c6), _mm256_slli_epi16(t0v11, 8));\
    t0v11 = _mm256_or_si256(_mm256_srli_epi16(t0v16, 8), _mm256_and_si256(t0v11, c7));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v6, c6), _mm256_slli_epi16(t0v14, 8));\
    t0v6 = _mm256_or_si256(_mm256_srli_epi16(t0v6, 8), _mm256_and_si256(t0v14, c7));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v4, c6), _mm256_slli_epi16(t0v12, 8));\
    t0v4 = _mm256_or_si256(_mm256_srli_epi16(t0v4, 8), _mm256_and_si256(t0v12, c7));\
    t0v12 = _mm256_or_si256(_mm256_and_si256(t0v3, c6), _mm256_slli_epi16(t0v7, 8));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(t0v3, 8), _mm256_and_si256(t0v7, c7));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v2, c6), _mm256_slli_epi16(t0v10, 8));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(t0v2, 8), _mm256_and_si256(t0v10, c7));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v0, c6), _mm256_slli_epi16(t0v8, 8));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(t0v0, 8), _mm256_and_si256(t0v8, c7));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v1, c6), _mm256_slli_epi16(t0v9, 8));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(t0v1, 8), _mm256_and_si256(t0v9, c7));\
    t0v9 = _mm256_and_si256(t0v13, c8);\
    t0v13 = _mm256_srli_epi32(t0v13, 16);\
    t0v17 = _mm256_and_si256(t0v15, c8);\
    t0v15 = _mm256_srli_epi32(t0v15, 16);\
    t0v18 = _mm256_and_si256(t0v16, c8);\
    t0v16 = _mm256_srli_epi32(t0v16, 16);\
    t0v19 = _mm256_and_si256(t0v14, c8);\
    t0v14 = _mm256_srli_epi32(t0v14, 16);\
    t0v20 = _mm256_and_si256(t0v12, c8);\
    t0v12 = _mm256_srli_epi32(t0v12, 16);\
    t0v21 = _mm256_and_si256(t0v7, c8);\
    t0v7 = _mm256_srli_epi32(t0v7, 16);\
    t0v22 = _mm256_and_si256(t0v10, c8);\
    t0v10 = _mm256_srli_epi32(t0v10, 16);\
    t0v23 = _mm256_and_si256(t0v8, c8);\
    t0v8 = _mm256_srli_epi32(t0v8, 16);\
    t0v24 = _mm256_and_si256(t0v5, c8);\
    t0v5 = _mm256_srli_epi32(t0v5, 16);\
    t0v25 = _mm256_and_si256(t0v11, c8);\
    t0v11 = _mm256_srli_epi32(t0v11, 16);\
    t0v26 = _mm256_and_si256(t0v6, c8);\
    t0v6 = _mm256_srli_epi32(t0v6, 16);\
    t0v27 = _mm256_and_si256(t0v4, c8);\
    t0v4 = _mm256_srli_epi32(t0v4, 16);\
    t0v28 = _mm256_and_si256(t0v3, c8);\
    t0v3 = _mm256_srli_epi32(t0v3, 16);\
    t0v29 = _mm256_and_si256(t0v2, c8);\
    t0v2 = _mm256_srli_epi32(t0v2, 16);\
    t0v30 = _mm256_and_si256(t0v0, c8);\
    t0v0 = _mm256_srli_epi32(t0v0, 16);\
    t0v31 = _mm256_and_si256(t0v1, c8);\
    t0v1 = _mm256_srli_epi32(t0v1, 16);\
    t0v32 = _mm256_blend_epi32(t0v9, _mm256_slli_epi64(t0v17, 32), 0xaa);\
    t0v9 = _mm256_blend_epi32(_mm256_srli_epi64(t0v9, 32), t0v17, 0xaa);\
    t0v17 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v19, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v19, 0xaa);\
    t0v19 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v21, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v21, 0xaa);\
    t0v21 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v23, 0xaa);\
    t0v23 = _mm256_blend_epi32(t0v24, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v24 = _mm256_blend_epi32(_mm256_srli_epi64(t0v24, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v26, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v26 = _mm256_blend_epi32(_mm256_srli_epi64(t0v26, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v28, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v28 = _mm256_blend_epi32(_mm256_srli_epi64(t0v28, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v30, _mm256_slli_epi64(t0v31, 32), 0xaa);\
    t0v30 = _mm256_blend_epi32(_mm256_srli_epi64(t0v30, 32), t0v31, 0xaa);\
    t0v31 = _mm256_blend_epi32(t0v13, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v13 = _mm256_blend_epi32(_mm256_srli_epi64(t0v13, 32), t0v15, 0xaa);\
    t0v15 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v14, 32), 0xaa);\
    t0v14 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v14, 0xaa);\
    t0v16 = _mm256_blend_epi32(t0v12, _mm256_slli_epi64(t0v7, 32), 0xaa);\
    t0v7 = _mm256_blend_epi32(_mm256_srli_epi64(t0v12, 32), t0v7, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v10, _mm256_slli_epi64(t0v8, 32), 0xaa);\
    t0v8 = _mm256_blend_epi32(_mm256_srli_epi64(t0v10, 32), t0v8, 0xaa);\
    t0v10 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v4, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v4, 0xaa);\
    t0v6 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v2, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v2, 0xaa);\
    t0v3 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v1, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v1, 0xaa);\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v32, c9), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v17 = _mm256_or_si256(_mm256_srli_si256(t0v32, (64)>>3), _mm256_and_si256(t0v17, c10));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v19, c9), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v19 = _mm256_or_si256(_mm256_srli_si256(t0v19, (64)>>3), _mm256_and_si256(t0v21, c10));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v23, c9), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v23 = _mm256_or_si256(_mm256_srli_si256(t0v23, (64)>>3), _mm256_and_si256(t0v25, c10));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v27, c9), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v27 = _mm256_or_si256(_mm256_srli_si256(t0v27, (64)>>3), _mm256_and_si256(t0v29, c10));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v31, c9), _mm256_slli_si256(t0v15, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v31, (64)>>3), _mm256_and_si256(t0v15, c10));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v16, c9), _mm256_slli_si256(t0v12, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v16, (64)>>3), _mm256_and_si256(t0v12, c10));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v10, c9), _mm256_slli_si256(t0v11, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v11, c10));\
    t0v11 = _mm256_or_si256(_mm256_and_si256(t0v6, c9), _mm256_slli_si256(t0v3, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v3, c10));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v9, c9), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v18, c10));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v20, c9), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v20 = _mm256_or_si256(_mm256_srli_si256(t0v20, (64)>>3), _mm256_and_si256(t0v22, c10));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v24, c9), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v24 = _mm256_or_si256(_mm256_srli_si256(t0v24, (64)>>3), _mm256_and_si256(t0v26, c10));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v28, c9), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v28 = _mm256_or_si256(_mm256_srli_si256(t0v28, (64)>>3), _mm256_and_si256(t0v30, c10));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v13, c9), _mm256_slli_si256(t0v14, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v13, (64)>>3), _mm256_and_si256(t0v14, c10));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v7, c9), _mm256_slli_si256(t0v8, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v8, c10));\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v5, c9), _mm256_slli_si256(t0v4, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v4, c10));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v2, c9), _mm256_slli_si256(t0v0, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v0, c10));\
    _mm256_storeu_si256((__m256i*)&((dest[0])), _mm256_permute2x128_si256(t0v1, t0v32, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[16])), _mm256_permute2x128_si256(t0v1, t0v32, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[1])), _mm256_permute2x128_si256(t0v21, t0v25, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[17])), _mm256_permute2x128_si256(t0v21, t0v25, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[2])), _mm256_permute2x128_si256(t0v29, t0v31, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[18])), _mm256_permute2x128_si256(t0v29, t0v31, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[3])), _mm256_permute2x128_si256(t0v16, t0v11, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[19])), _mm256_permute2x128_si256(t0v16, t0v11, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[4])), _mm256_permute2x128_si256(t0v6, t0v18, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[20])), _mm256_permute2x128_si256(t0v6, t0v18, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[5])), _mm256_permute2x128_si256(t0v22, t0v26, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[21])), _mm256_permute2x128_si256(t0v22, t0v26, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[6])), _mm256_permute2x128_si256(t0v30, t0v14, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[22])), _mm256_permute2x128_si256(t0v30, t0v14, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[7])), _mm256_permute2x128_si256(t0v8, t0v5, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[23])), _mm256_permute2x128_si256(t0v8, t0v5, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[8])), _mm256_permute2x128_si256(t0v17, t0v19, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[24])), _mm256_permute2x128_si256(t0v17, t0v19, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[9])), _mm256_permute2x128_si256(t0v23, t0v27, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[25])), _mm256_permute2x128_si256(t0v23, t0v27, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[10])), _mm256_permute2x128_si256(t0v15, t0v12, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[26])), _mm256_permute2x128_si256(t0v15, t0v12, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[11])), _mm256_permute2x128_si256(t0v10, t0v3, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[27])), _mm256_permute2x128_si256(t0v10, t0v3, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[12])), _mm256_permute2x128_si256(t0v9, t0v20, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[28])), _mm256_permute2x128_si256(t0v9, t0v20, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[13])), _mm256_permute2x128_si256(t0v24, t0v28, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[29])), _mm256_permute2x128_si256(t0v24, t0v28, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[14])), _mm256_permute2x128_si256(t0v13, t0v7, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[30])), _mm256_permute2x128_si256(t0v13, t0v7, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[15])), _mm256_permute2x128_si256(t0v4, t0v0, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[31])), _mm256_permute2x128_si256(t0v4, t0v0, 0x31));\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    __m256i* dest = (__m256i*)(D);\
    const __m256i c6 = (*(const __m256i*)(transform_const2_tbl + 8*3));\
    const __m256i c0 = (*(const __m256i*)(transform_const_tbl + 8*0));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*1));\
    const __m256i c7 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c8 = (*(const __m256i*)(transform_const_tbl + 8*13));\
    const __m256i c2 = (*(const __m256i*)(transform_const_tbl + 8*2));\
    const __m256i c3 = (*(const __m256i*)(transform_const_tbl + 8*3));\
    const __m256i c4 = (*(const __m256i*)(transform_const_tbl + 8*4));\
    const __m256i c5 = (*(const __m256i*)(transform_const_tbl + 8*5));\
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
    t0v0 = _mm256_or_si256(_mm256_and_si256((S0), c0), _mm256_slli_epi16(_mm256_and_si256((S1), c0), 1));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S0), c1), 1), _mm256_and_si256((S1), c1));\
    t0v2 = _mm256_or_si256(_mm256_and_si256((S2), c0), _mm256_slli_epi16(_mm256_and_si256((S3), c0), 1));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S2), c1), 1), _mm256_and_si256((S3), c1));\
    t0v4 = _mm256_or_si256(_mm256_and_si256((S4), c0), _mm256_slli_epi16(_mm256_and_si256((S5), c0), 1));\
    t0v5 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256((S4), c1), 1), _mm256_and_si256((S5), c1));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v0, c2), _mm256_slli_epi16(_mm256_and_si256(t0v2, c2), 2));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c3), 2), _mm256_and_si256(t0v2, c3));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v1, c2), _mm256_slli_epi16(_mm256_and_si256(t0v3, c2), 2));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c3), 2), _mm256_and_si256(t0v3, c3));\
    t0v3 = _mm256_and_si256(t0v4, c2);\
    t0v4 = _mm256_srli_epi16(_mm256_and_si256(t0v4, c3), 2);\
    t0v7 = _mm256_and_si256(t0v5, c2);\
    t0v5 = _mm256_srli_epi16(_mm256_and_si256(t0v5, c3), 2);\
    t0v8 = _mm256_or_si256(_mm256_and_si256(t0v6, c4), _mm256_slli_epi16(_mm256_and_si256(t0v3, c4), 4));\
    t0v3 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v6, c5), 4), _mm256_and_si256(t0v3, c5));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v2, c4), _mm256_slli_epi16(_mm256_and_si256(t0v7, c4), 4));\
    t0v2 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v2, c5), 4), _mm256_and_si256(t0v7, c5));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v0, c4), _mm256_slli_epi16(_mm256_and_si256(t0v4, c4), 4));\
    t0v0 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v0, c5), 4), _mm256_and_si256(t0v4, c5));\
    t0v4 = _mm256_or_si256(_mm256_and_si256(t0v1, c4), _mm256_slli_epi16(_mm256_and_si256(t0v5, c4), 4));\
    t0v1 = _mm256_or_si256(_mm256_srli_epi16(_mm256_and_si256(t0v1, c5), 4), _mm256_and_si256(t0v5, c5));\
    t0v5 = _mm256_and_si256(t0v8, c6);\
    t0v9 = _mm256_and_si256(_mm256_srli_epi32(t0v8, 8), c6);\
    t0v10 = _mm256_and_si256(_mm256_srli_epi32(t0v8, 16), c6);\
    t0v8 = _mm256_srli_epi32(t0v8, 24);\
    t0v11 = _mm256_and_si256(t0v6, c6);\
    t0v12 = _mm256_and_si256(_mm256_srli_epi32(t0v6, 8), c6);\
    t0v13 = _mm256_and_si256(_mm256_srli_epi32(t0v6, 16), c6);\
    t0v6 = _mm256_srli_epi32(t0v6, 24);\
    t0v14 = _mm256_and_si256(t0v7, c6);\
    t0v15 = _mm256_and_si256(_mm256_srli_epi32(t0v7, 8), c6);\
    t0v16 = _mm256_and_si256(_mm256_srli_epi32(t0v7, 16), c6);\
    t0v7 = _mm256_srli_epi32(t0v7, 24);\
    t0v17 = _mm256_and_si256(t0v4, c6);\
    t0v18 = _mm256_and_si256(_mm256_srli_epi32(t0v4, 8), c6);\
    t0v19 = _mm256_and_si256(_mm256_srli_epi32(t0v4, 16), c6);\
    t0v4 = _mm256_srli_epi32(t0v4, 24);\
    t0v20 = _mm256_and_si256(t0v3, c6);\
    t0v21 = _mm256_and_si256(_mm256_srli_epi32(t0v3, 8), c6);\
    t0v22 = _mm256_and_si256(_mm256_srli_epi32(t0v3, 16), c6);\
    t0v3 = _mm256_srli_epi32(t0v3, 24);\
    t0v23 = _mm256_and_si256(t0v2, c6);\
    t0v24 = _mm256_and_si256(_mm256_srli_epi32(t0v2, 8), c6);\
    t0v25 = _mm256_and_si256(_mm256_srli_epi32(t0v2, 16), c6);\
    t0v2 = _mm256_srli_epi32(t0v2, 24);\
    t0v26 = _mm256_and_si256(t0v0, c6);\
    t0v27 = _mm256_and_si256(_mm256_srli_epi32(t0v0, 8), c6);\
    t0v28 = _mm256_and_si256(_mm256_srli_epi32(t0v0, 16), c6);\
    t0v0 = _mm256_srli_epi32(t0v0, 24);\
    t0v29 = _mm256_and_si256(t0v1, c6);\
    t0v30 = _mm256_and_si256(_mm256_srli_epi32(t0v1, 8), c6);\
    t0v31 = _mm256_and_si256(_mm256_srli_epi32(t0v1, 16), c6);\
    t0v1 = _mm256_srli_epi32(t0v1, 24);\
    t0v32 = _mm256_blend_epi32(t0v5, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v5 = _mm256_blend_epi32(_mm256_srli_epi64(t0v5, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v14, _mm256_slli_epi64(t0v17, 32), 0xaa);\
    t0v14 = _mm256_blend_epi32(_mm256_srli_epi64(t0v14, 32), t0v17, 0xaa);\
    t0v17 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v23, 0xaa);\
    t0v23 = _mm256_blend_epi32(t0v26, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v26 = _mm256_blend_epi32(_mm256_srli_epi64(t0v26, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v9, _mm256_slli_epi64(t0v12, 32), 0xaa);\
    t0v9 = _mm256_blend_epi32(_mm256_srli_epi64(t0v9, 32), t0v12, 0xaa);\
    t0v12 = _mm256_blend_epi32(t0v15, _mm256_slli_epi64(t0v18, 32), 0xaa);\
    t0v15 = _mm256_blend_epi32(_mm256_srli_epi64(t0v15, 32), t0v18, 0xaa);\
    t0v18 = _mm256_blend_epi32(t0v21, _mm256_slli_epi64(t0v24, 32), 0xaa);\
    t0v21 = _mm256_blend_epi32(_mm256_srli_epi64(t0v21, 32), t0v24, 0xaa);\
    t0v24 = _mm256_blend_epi32(t0v27, _mm256_slli_epi64(t0v30, 32), 0xaa);\
    t0v27 = _mm256_blend_epi32(_mm256_srli_epi64(t0v27, 32), t0v30, 0xaa);\
    t0v30 = _mm256_blend_epi32(t0v10, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v10 = _mm256_blend_epi32(_mm256_srli_epi64(t0v10, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v19, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v19, 0xaa);\
    t0v19 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v28, _mm256_slli_epi64(t0v31, 32), 0xaa);\
    t0v28 = _mm256_blend_epi32(_mm256_srli_epi64(t0v28, 32), t0v31, 0xaa);\
    t0v31 = _mm256_blend_epi32(t0v8, _mm256_slli_epi64(t0v6, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v8, 32), t0v6, 0xaa);\
    t0v8 = _mm256_blend_epi32(t0v7, _mm256_slli_epi64(t0v4, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v7, 32), t0v4, 0xaa);\
    t0v7 = _mm256_blend_epi32(t0v3, _mm256_slli_epi64(t0v2, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v3, 32), t0v2, 0xaa);\
    t0v3 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v1, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v1, 0xaa);\
    t0v1 = _mm256_or_si256(_mm256_and_si256(t0v32, c7), _mm256_slli_si256(t0v11, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v32, (64)>>3), _mm256_and_si256(t0v11, c8));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v17, c7), _mm256_slli_si256(t0v23, (64)>>3));\
    t0v17 = _mm256_or_si256(_mm256_srli_si256(t0v17, (64)>>3), _mm256_and_si256(t0v23, c8));\
    t0v23 = _mm256_or_si256(_mm256_and_si256(t0v29, c7), _mm256_slli_si256(t0v12, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v29, (64)>>3), _mm256_and_si256(t0v12, c8));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v18, c7), _mm256_slli_si256(t0v24, (64)>>3));\
    t0v18 = _mm256_or_si256(_mm256_srli_si256(t0v18, (64)>>3), _mm256_and_si256(t0v24, c8));\
    t0v24 = _mm256_or_si256(_mm256_and_si256(t0v30, c7), _mm256_slli_si256(t0v13, (64)>>3));\
    t0v13 = _mm256_or_si256(_mm256_srli_si256(t0v30, (64)>>3), _mm256_and_si256(t0v13, c8));\
    t0v30 = _mm256_or_si256(_mm256_and_si256(t0v19, c7), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v19 = _mm256_or_si256(_mm256_srli_si256(t0v19, (64)>>3), _mm256_and_si256(t0v25, c8));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v31, c7), _mm256_slli_si256(t0v8, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v31, (64)>>3), _mm256_and_si256(t0v8, c8));\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v7, c7), _mm256_slli_si256(t0v3, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v3, c8));\
    t0v7 = _mm256_or_si256(_mm256_and_si256(t0v5, c7), _mm256_slli_si256(t0v14, (64)>>3));\
    t0v5 = _mm256_or_si256(_mm256_srli_si256(t0v5, (64)>>3), _mm256_and_si256(t0v14, c8));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v20, c7), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v20 = _mm256_or_si256(_mm256_srli_si256(t0v20, (64)>>3), _mm256_and_si256(t0v26, c8));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v9, c7), _mm256_slli_si256(t0v15, (64)>>3));\
    t0v9 = _mm256_or_si256(_mm256_srli_si256(t0v9, (64)>>3), _mm256_and_si256(t0v15, c8));\
    t0v15 = _mm256_or_si256(_mm256_and_si256(t0v21, c7), _mm256_slli_si256(t0v27, (64)>>3));\
    t0v21 = _mm256_or_si256(_mm256_srli_si256(t0v21, (64)>>3), _mm256_and_si256(t0v27, c8));\
    t0v27 = _mm256_or_si256(_mm256_and_si256(t0v10, c7), _mm256_slli_si256(t0v16, (64)>>3));\
    t0v10 = _mm256_or_si256(_mm256_srli_si256(t0v10, (64)>>3), _mm256_and_si256(t0v16, c8));\
    t0v16 = _mm256_or_si256(_mm256_and_si256(t0v22, c7), _mm256_slli_si256(t0v28, (64)>>3));\
    t0v22 = _mm256_or_si256(_mm256_srli_si256(t0v22, (64)>>3), _mm256_and_si256(t0v28, c8));\
    t0v28 = _mm256_or_si256(_mm256_and_si256(t0v6, c7), _mm256_slli_si256(t0v4, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v6, (64)>>3), _mm256_and_si256(t0v4, c8));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v2, c7), _mm256_slli_si256(t0v0, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v2, (64)>>3), _mm256_and_si256(t0v0, c8));\
    _mm256_storeu_si256((__m256i*)&((dest[0])), _mm256_permute2x128_si256(t0v1, t0v32, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[16])), _mm256_permute2x128_si256(t0v1, t0v32, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[1])), _mm256_permute2x128_si256(t0v23, t0v29, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[17])), _mm256_permute2x128_si256(t0v23, t0v29, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[2])), _mm256_permute2x128_si256(t0v24, t0v30, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[18])), _mm256_permute2x128_si256(t0v24, t0v30, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[3])), _mm256_permute2x128_si256(t0v25, t0v31, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[19])), _mm256_permute2x128_si256(t0v25, t0v31, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[4])), _mm256_permute2x128_si256(t0v7, t0v14, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[20])), _mm256_permute2x128_si256(t0v7, t0v14, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[5])), _mm256_permute2x128_si256(t0v26, t0v15, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[21])), _mm256_permute2x128_si256(t0v26, t0v15, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[6])), _mm256_permute2x128_si256(t0v27, t0v16, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[22])), _mm256_permute2x128_si256(t0v27, t0v16, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[7])), _mm256_permute2x128_si256(t0v28, t0v6, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[23])), _mm256_permute2x128_si256(t0v28, t0v6, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[8])), _mm256_permute2x128_si256(t0v11, t0v17, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[24])), _mm256_permute2x128_si256(t0v11, t0v17, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[9])), _mm256_permute2x128_si256(t0v12, t0v18, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[25])), _mm256_permute2x128_si256(t0v12, t0v18, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[10])), _mm256_permute2x128_si256(t0v13, t0v19, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[26])), _mm256_permute2x128_si256(t0v13, t0v19, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[11])), _mm256_permute2x128_si256(t0v8, t0v3, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[27])), _mm256_permute2x128_si256(t0v8, t0v3, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[12])), _mm256_permute2x128_si256(t0v5, t0v20, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[28])), _mm256_permute2x128_si256(t0v5, t0v20, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[13])), _mm256_permute2x128_si256(t0v9, t0v21, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[29])), _mm256_permute2x128_si256(t0v9, t0v21, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[14])), _mm256_permute2x128_si256(t0v10, t0v22, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[30])), _mm256_permute2x128_si256(t0v10, t0v22, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[15])), _mm256_permute2x128_si256(t0v4, t0v0, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[31])), _mm256_permute2x128_si256(t0v4, t0v0, 0x31));\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    __m256i* dest = (__m256i*)(D);\
    const __m256i c0 = (*(const __m256i*)(transform_const2_tbl + 8*0));\
    const __m256i c1 = (*(const __m256i*)(transform_const_tbl + 8*12));\
    const __m256i c2 = (*(const __m256i*)(transform_const_tbl + 8*13));\
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
    t0v0 = _mm256_and_si256((S0), c0);\
    t0v1 = _mm256_and_si256(_mm256_srli_epi32((S0), 1), c0);\
    t0v2 = _mm256_and_si256(_mm256_srli_epi32((S0), 2), c0);\
    t0v3 = _mm256_and_si256(_mm256_srli_epi32((S0), 3), c0);\
    t0v4 = _mm256_and_si256(_mm256_srli_epi32((S0), 4), c0);\
    t0v5 = _mm256_and_si256(_mm256_srli_epi32((S0), 5), c0);\
    t0v6 = _mm256_and_si256(_mm256_srli_epi32((S0), 6), c0);\
    t0v7 = _mm256_and_si256(_mm256_srli_epi32((S0), 7), c0);\
    t0v8 = _mm256_and_si256(_mm256_srli_epi32((S0), 8), c0);\
    t0v9 = _mm256_and_si256(_mm256_srli_epi32((S0), 9), c0);\
    t0v10 = _mm256_and_si256(_mm256_srli_epi32((S0), 10), c0);\
    t0v11 = _mm256_and_si256(_mm256_srli_epi32((S0), 11), c0);\
    t0v12 = _mm256_and_si256(_mm256_srli_epi32((S0), 12), c0);\
    t0v13 = _mm256_and_si256(_mm256_srli_epi32((S0), 13), c0);\
    t0v14 = _mm256_and_si256(_mm256_srli_epi32((S0), 14), c0);\
    t0v15 = _mm256_and_si256(_mm256_srli_epi32((S0), 15), c0);\
    t0v16 = _mm256_and_si256(_mm256_srli_epi32((S0), 16), c0);\
    t0v17 = _mm256_and_si256(_mm256_srli_epi32((S0), 17), c0);\
    t0v18 = _mm256_and_si256(_mm256_srli_epi32((S0), 18), c0);\
    t0v19 = _mm256_and_si256(_mm256_srli_epi32((S0), 19), c0);\
    t0v20 = _mm256_and_si256(_mm256_srli_epi32((S0), 20), c0);\
    t0v21 = _mm256_and_si256(_mm256_srli_epi32((S0), 21), c0);\
    t0v22 = _mm256_and_si256(_mm256_srli_epi32((S0), 22), c0);\
    t0v23 = _mm256_and_si256(_mm256_srli_epi32((S0), 23), c0);\
    t0v24 = _mm256_and_si256(_mm256_srli_epi32((S0), 24), c0);\
    t0v25 = _mm256_and_si256(_mm256_srli_epi32((S0), 25), c0);\
    t0v26 = _mm256_and_si256(_mm256_srli_epi32((S0), 26), c0);\
    t0v27 = _mm256_and_si256(_mm256_srli_epi32((S0), 27), c0);\
    t0v28 = _mm256_and_si256(_mm256_srli_epi32((S0), 28), c0);\
    t0v29 = _mm256_and_si256(_mm256_srli_epi32((S0), 29), c0);\
    t0v30 = _mm256_and_si256(_mm256_srli_epi32((S0), 30), c0);\
    t0v31 = _mm256_srli_epi32((S0), 31);\
    t0v32 = _mm256_blend_epi32(t0v0, _mm256_slli_epi64(t0v1, 32), 0xaa);\
    t0v0 = _mm256_blend_epi32(_mm256_srli_epi64(t0v0, 32), t0v1, 0xaa);\
    t0v1 = _mm256_blend_epi32(t0v2, _mm256_slli_epi64(t0v3, 32), 0xaa);\
    t0v2 = _mm256_blend_epi32(_mm256_srli_epi64(t0v2, 32), t0v3, 0xaa);\
    t0v3 = _mm256_blend_epi32(t0v4, _mm256_slli_epi64(t0v5, 32), 0xaa);\
    t0v4 = _mm256_blend_epi32(_mm256_srli_epi64(t0v4, 32), t0v5, 0xaa);\
    t0v5 = _mm256_blend_epi32(t0v6, _mm256_slli_epi64(t0v7, 32), 0xaa);\
    t0v6 = _mm256_blend_epi32(_mm256_srli_epi64(t0v6, 32), t0v7, 0xaa);\
    t0v7 = _mm256_blend_epi32(t0v8, _mm256_slli_epi64(t0v9, 32), 0xaa);\
    t0v8 = _mm256_blend_epi32(_mm256_srli_epi64(t0v8, 32), t0v9, 0xaa);\
    t0v9 = _mm256_blend_epi32(t0v10, _mm256_slli_epi64(t0v11, 32), 0xaa);\
    t0v10 = _mm256_blend_epi32(_mm256_srli_epi64(t0v10, 32), t0v11, 0xaa);\
    t0v11 = _mm256_blend_epi32(t0v12, _mm256_slli_epi64(t0v13, 32), 0xaa);\
    t0v12 = _mm256_blend_epi32(_mm256_srli_epi64(t0v12, 32), t0v13, 0xaa);\
    t0v13 = _mm256_blend_epi32(t0v14, _mm256_slli_epi64(t0v15, 32), 0xaa);\
    t0v14 = _mm256_blend_epi32(_mm256_srli_epi64(t0v14, 32), t0v15, 0xaa);\
    t0v15 = _mm256_blend_epi32(t0v16, _mm256_slli_epi64(t0v17, 32), 0xaa);\
    t0v16 = _mm256_blend_epi32(_mm256_srli_epi64(t0v16, 32), t0v17, 0xaa);\
    t0v17 = _mm256_blend_epi32(t0v18, _mm256_slli_epi64(t0v19, 32), 0xaa);\
    t0v18 = _mm256_blend_epi32(_mm256_srli_epi64(t0v18, 32), t0v19, 0xaa);\
    t0v19 = _mm256_blend_epi32(t0v20, _mm256_slli_epi64(t0v21, 32), 0xaa);\
    t0v20 = _mm256_blend_epi32(_mm256_srli_epi64(t0v20, 32), t0v21, 0xaa);\
    t0v21 = _mm256_blend_epi32(t0v22, _mm256_slli_epi64(t0v23, 32), 0xaa);\
    t0v22 = _mm256_blend_epi32(_mm256_srli_epi64(t0v22, 32), t0v23, 0xaa);\
    t0v23 = _mm256_blend_epi32(t0v24, _mm256_slli_epi64(t0v25, 32), 0xaa);\
    t0v24 = _mm256_blend_epi32(_mm256_srli_epi64(t0v24, 32), t0v25, 0xaa);\
    t0v25 = _mm256_blend_epi32(t0v26, _mm256_slli_epi64(t0v27, 32), 0xaa);\
    t0v26 = _mm256_blend_epi32(_mm256_srli_epi64(t0v26, 32), t0v27, 0xaa);\
    t0v27 = _mm256_blend_epi32(t0v28, _mm256_slli_epi64(t0v29, 32), 0xaa);\
    t0v28 = _mm256_blend_epi32(_mm256_srli_epi64(t0v28, 32), t0v29, 0xaa);\
    t0v29 = _mm256_blend_epi32(t0v30, _mm256_slli_epi64(t0v31, 32), 0xaa);\
    t0v30 = _mm256_blend_epi32(_mm256_srli_epi64(t0v30, 32), t0v31, 0xaa);\
    t0v31 = _mm256_or_si256(_mm256_and_si256(t0v32, c1), _mm256_slli_si256(t0v1, (64)>>3));\
    t0v1 = _mm256_or_si256(_mm256_srli_si256(t0v32, (64)>>3), _mm256_and_si256(t0v1, c2));\
    t0v32 = _mm256_or_si256(_mm256_and_si256(t0v3, c1), _mm256_slli_si256(t0v5, (64)>>3));\
    t0v3 = _mm256_or_si256(_mm256_srli_si256(t0v3, (64)>>3), _mm256_and_si256(t0v5, c2));\
    t0v5 = _mm256_or_si256(_mm256_and_si256(t0v7, c1), _mm256_slli_si256(t0v9, (64)>>3));\
    t0v7 = _mm256_or_si256(_mm256_srli_si256(t0v7, (64)>>3), _mm256_and_si256(t0v9, c2));\
    t0v9 = _mm256_or_si256(_mm256_and_si256(t0v11, c1), _mm256_slli_si256(t0v13, (64)>>3));\
    t0v11 = _mm256_or_si256(_mm256_srli_si256(t0v11, (64)>>3), _mm256_and_si256(t0v13, c2));\
    t0v13 = _mm256_or_si256(_mm256_and_si256(t0v15, c1), _mm256_slli_si256(t0v17, (64)>>3));\
    t0v15 = _mm256_or_si256(_mm256_srli_si256(t0v15, (64)>>3), _mm256_and_si256(t0v17, c2));\
    t0v17 = _mm256_or_si256(_mm256_and_si256(t0v19, c1), _mm256_slli_si256(t0v21, (64)>>3));\
    t0v19 = _mm256_or_si256(_mm256_srli_si256(t0v19, (64)>>3), _mm256_and_si256(t0v21, c2));\
    t0v21 = _mm256_or_si256(_mm256_and_si256(t0v23, c1), _mm256_slli_si256(t0v25, (64)>>3));\
    t0v23 = _mm256_or_si256(_mm256_srli_si256(t0v23, (64)>>3), _mm256_and_si256(t0v25, c2));\
    t0v25 = _mm256_or_si256(_mm256_and_si256(t0v27, c1), _mm256_slli_si256(t0v29, (64)>>3));\
    t0v27 = _mm256_or_si256(_mm256_srli_si256(t0v27, (64)>>3), _mm256_and_si256(t0v29, c2));\
    t0v29 = _mm256_or_si256(_mm256_and_si256(t0v0, c1), _mm256_slli_si256(t0v2, (64)>>3));\
    t0v0 = _mm256_or_si256(_mm256_srli_si256(t0v0, (64)>>3), _mm256_and_si256(t0v2, c2));\
    t0v2 = _mm256_or_si256(_mm256_and_si256(t0v4, c1), _mm256_slli_si256(t0v6, (64)>>3));\
    t0v4 = _mm256_or_si256(_mm256_srli_si256(t0v4, (64)>>3), _mm256_and_si256(t0v6, c2));\
    t0v6 = _mm256_or_si256(_mm256_and_si256(t0v8, c1), _mm256_slli_si256(t0v10, (64)>>3));\
    t0v8 = _mm256_or_si256(_mm256_srli_si256(t0v8, (64)>>3), _mm256_and_si256(t0v10, c2));\
    t0v10 = _mm256_or_si256(_mm256_and_si256(t0v12, c1), _mm256_slli_si256(t0v14, (64)>>3));\
    t0v12 = _mm256_or_si256(_mm256_srli_si256(t0v12, (64)>>3), _mm256_and_si256(t0v14, c2));\
    t0v14 = _mm256_or_si256(_mm256_and_si256(t0v16, c1), _mm256_slli_si256(t0v18, (64)>>3));\
    t0v16 = _mm256_or_si256(_mm256_srli_si256(t0v16, (64)>>3), _mm256_and_si256(t0v18, c2));\
    t0v18 = _mm256_or_si256(_mm256_and_si256(t0v20, c1), _mm256_slli_si256(t0v22, (64)>>3));\
    t0v20 = _mm256_or_si256(_mm256_srli_si256(t0v20, (64)>>3), _mm256_and_si256(t0v22, c2));\
    t0v22 = _mm256_or_si256(_mm256_and_si256(t0v24, c1), _mm256_slli_si256(t0v26, (64)>>3));\
    t0v24 = _mm256_or_si256(_mm256_srli_si256(t0v24, (64)>>3), _mm256_and_si256(t0v26, c2));\
    t0v26 = _mm256_or_si256(_mm256_and_si256(t0v28, c1), _mm256_slli_si256(t0v30, (64)>>3));\
    t0v28 = _mm256_or_si256(_mm256_srli_si256(t0v28, (64)>>3), _mm256_and_si256(t0v30, c2));\
    _mm256_storeu_si256((__m256i*)&((dest[0])), _mm256_permute2x128_si256(t0v31, t0v32, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[16])), _mm256_permute2x128_si256(t0v31, t0v32, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[1])), _mm256_permute2x128_si256(t0v5, t0v9, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[17])), _mm256_permute2x128_si256(t0v5, t0v9, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[2])), _mm256_permute2x128_si256(t0v13, t0v17, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[18])), _mm256_permute2x128_si256(t0v13, t0v17, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[3])), _mm256_permute2x128_si256(t0v21, t0v25, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[19])), _mm256_permute2x128_si256(t0v21, t0v25, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[4])), _mm256_permute2x128_si256(t0v29, t0v2, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[20])), _mm256_permute2x128_si256(t0v29, t0v2, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[5])), _mm256_permute2x128_si256(t0v6, t0v10, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[21])), _mm256_permute2x128_si256(t0v6, t0v10, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[6])), _mm256_permute2x128_si256(t0v14, t0v18, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[22])), _mm256_permute2x128_si256(t0v14, t0v18, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[7])), _mm256_permute2x128_si256(t0v22, t0v26, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[23])), _mm256_permute2x128_si256(t0v22, t0v26, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[8])), _mm256_permute2x128_si256(t0v1, t0v3, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[24])), _mm256_permute2x128_si256(t0v1, t0v3, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[9])), _mm256_permute2x128_si256(t0v7, t0v11, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[25])), _mm256_permute2x128_si256(t0v7, t0v11, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[10])), _mm256_permute2x128_si256(t0v15, t0v19, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[26])), _mm256_permute2x128_si256(t0v15, t0v19, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[11])), _mm256_permute2x128_si256(t0v23, t0v27, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[27])), _mm256_permute2x128_si256(t0v23, t0v27, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[12])), _mm256_permute2x128_si256(t0v0, t0v4, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[28])), _mm256_permute2x128_si256(t0v0, t0v4, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[13])), _mm256_permute2x128_si256(t0v8, t0v12, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[29])), _mm256_permute2x128_si256(t0v8, t0v12, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[14])), _mm256_permute2x128_si256(t0v16, t0v20, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[30])), _mm256_permute2x128_si256(t0v16, t0v20, 0x31));\
    _mm256_storeu_si256((__m256i*)&((dest[15])), _mm256_permute2x128_si256(t0v24, t0v28, 0x20));\
    _mm256_storeu_si256((__m256i*)&((dest[31])), _mm256_permute2x128_si256(t0v24, t0v28, 0x31));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_INTEL_AVX512.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    __m512i* dest = (__m512i*)(D);\
    const __m512i c0 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*5));\
    const __m512i c6 = (*(const __m512i*)(transform_const_tbl + 16*6));\
    const __m512i c7 = (*(const __m512i*)(transform_const_tbl + 16*7));\
    const __m512i c8 = (*(const __m512i*)(transform_const_tbl + 16*8));\
    const __m512i c9 = (*(const __m512i*)(transform_const_tbl + 16*9));\
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
    t0v0 = _mm512_or_epi64(_mm512_and_epi64((S0), c0), _mm512_slli_epi32(_mm512_and_epi64((S1), c0), 1));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S0), c1), 1), _mm512_and_epi64((S1), c1));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64((S2), c0), _mm512_slli_epi32(_mm512_and_epi64((S3), c0), 1));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S2), c1), 1), _mm512_and_epi64((S3), c1));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64((S4), c0), _mm512_slli_epi32(_mm512_and_epi64((S5), c0), 1));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S4), c1), 1), _mm512_and_epi64((S5), c1));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64((S6), c0), _mm512_slli_epi32(_mm512_and_epi64((S7), c0), 1));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S6), c1), 1), _mm512_and_epi64((S7), c1));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64((S8), c0), _mm512_slli_epi32(_mm512_and_epi64((S9), c0), 1));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S8), c1), 1), _mm512_and_epi64((S9), c1));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64((S10), c0), _mm512_slli_epi32(_mm512_and_epi64((S11), c0), 1));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S10), c1), 1), _mm512_and_epi64((S11), c1));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64((S12), c0), _mm512_slli_epi32(_mm512_and_epi64((S13), c0), 1));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S12), c1), 1), _mm512_and_epi64((S13), c1));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64((S14), c0), _mm512_slli_epi32(_mm512_and_epi64((S15), c0), 1));\
    t0v15 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S14), c1), 1), _mm512_and_epi64((S15), c1));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64((S16), c0), _mm512_slli_epi32(_mm512_and_epi64((S17), c0), 1));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S16), c1), 1), _mm512_and_epi64((S17), c1));\
    t0v18 = _mm512_or_epi64(_mm512_and_epi64((S18), c0), _mm512_slli_epi32(_mm512_and_epi64((S19), c0), 1));\
    t0v19 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S18), c1), 1), _mm512_and_epi64((S19), c1));\
    t0v20 = _mm512_or_epi64(_mm512_and_epi64((S20), c0), _mm512_slli_epi32(_mm512_and_epi64((S21), c0), 1));\
    t0v21 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S20), c1), 1), _mm512_and_epi64((S21), c1));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64((S22), c0), _mm512_slli_epi32(_mm512_and_epi64((S23), c0), 1));\
    t0v23 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S22), c1), 1), _mm512_and_epi64((S23), c1));\
    t0v24 = _mm512_or_epi64(_mm512_and_epi64((S24), c0), _mm512_slli_epi32(_mm512_and_epi64((S25), c0), 1));\
    t0v25 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S24), c1), 1), _mm512_and_epi64((S25), c1));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64((S26), c0), _mm512_slli_epi32(_mm512_and_epi64((S27), c0), 1));\
    t0v27 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S26), c1), 1), _mm512_and_epi64((S27), c1));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64((S28), c0), _mm512_slli_epi32(_mm512_and_epi64((S29), c0), 1));\
    t0v29 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S28), c1), 1), _mm512_and_epi64((S29), c1));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64((S30), c0), _mm512_slli_epi32(_mm512_and_epi64((S31), c0), 1));\
    t0v31 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S30), c1), 1), _mm512_and_epi64((S31), c1));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c2), 2));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c3), 2), _mm512_and_epi64(t0v2, c3));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c2), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c3), 2), _mm512_and_epi64(t0v3, c3));\
    t0v3 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c2), 2));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c3), 2), _mm512_and_epi64(t0v6, c3));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c2), 2));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c3), 2), _mm512_and_epi64(t0v7, c3));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c2), 2));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c3), 2), _mm512_and_epi64(t0v10, c3));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c2), 2));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c3), 2), _mm512_and_epi64(t0v11, c3));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c2), 2));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v12, c3), 2), _mm512_and_epi64(t0v14, c3));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c2), 2));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c3), 2), _mm512_and_epi64(t0v15, c3));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v18, c2), 2));\
    t0v16 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c3), 2), _mm512_and_epi64(t0v18, c3));\
    t0v18 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c2), 2));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c3), 2), _mm512_and_epi64(t0v19, c3));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v20, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c2), 2));\
    t0v20 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v20, c3), 2), _mm512_and_epi64(t0v22, c3));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64(t0v21, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c2), 2));\
    t0v21 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v21, c3), 2), _mm512_and_epi64(t0v23, c3));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v24, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c2), 2));\
    t0v24 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v24, c3), 2), _mm512_and_epi64(t0v26, c3));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v25, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c2), 2));\
    t0v25 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v25, c3), 2), _mm512_and_epi64(t0v27, c3));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v28, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c2), 2));\
    t0v28 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v28, c3), 2), _mm512_and_epi64(t0v30, c3));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v29, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v31, c2), 2));\
    t0v29 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v29, c3), 2), _mm512_and_epi64(t0v31, c3));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c4), 4));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c5), 4), _mm512_and_epi64(t0v3, c5));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c4), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c5), 4), _mm512_and_epi64(t0v6, c5));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c4), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c5), 4), _mm512_and_epi64(t0v4, c5));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c4), 4));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c5), 4), _mm512_and_epi64(t0v5, c5));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c4), 4));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c5), 4), _mm512_and_epi64(t0v11, c5));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v10, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c4), 4));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v10, c5), 4), _mm512_and_epi64(t0v14, c5));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c4), 4));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c5), 4), _mm512_and_epi64(t0v12, c5));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c4), 4));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c5), 4), _mm512_and_epi64(t0v13, c5));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v19, c4), 4));\
    t0v15 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c5), 4), _mm512_and_epi64(t0v19, c5));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v18, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v22, c4), 4));\
    t0v18 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v18, c5), 4), _mm512_and_epi64(t0v22, c5));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v20, c4), 4));\
    t0v16 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c5), 4), _mm512_and_epi64(t0v20, c5));\
    t0v20 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c4), 4));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c5), 4), _mm512_and_epi64(t0v21, c5));\
    t0v21 = _mm512_or_epi64(_mm512_and_epi64(t0v23, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c4), 4));\
    t0v23 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v23, c5), 4), _mm512_and_epi64(t0v27, c5));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v26, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c4), 4));\
    t0v26 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v26, c5), 4), _mm512_and_epi64(t0v30, c5));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v24, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c4), 4));\
    t0v24 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v24, c5), 4), _mm512_and_epi64(t0v28, c5));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64(t0v25, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v29, c4), 4));\
    t0v25 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v25, c5), 4), _mm512_and_epi64(t0v29, c5));\
    t0v29 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c6), 8));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v31, c7), 8), _mm512_and_epi64(t0v5, c7));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c6), 8));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v32, c7), 8), _mm512_and_epi64(t0v11, c7));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c6), 8));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c7), 8), _mm512_and_epi64(t0v14, c7));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c6), 8));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c7), 8), _mm512_and_epi64(t0v12, c7));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c6), 8));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c7), 8), _mm512_and_epi64(t0v7, c7));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c6), 8));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c7), 8), _mm512_and_epi64(t0v10, c7));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c6), 8));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c7), 8), _mm512_and_epi64(t0v8, c7));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c6), 8));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c7), 8), _mm512_and_epi64(t0v9, c7));\
    t0v9 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v21, c6), 8));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c7), 8), _mm512_and_epi64(t0v21, c7));\
    t0v21 = _mm512_or_epi64(_mm512_and_epi64(t0v19, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v27, c6), 8));\
    t0v19 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v19, c7), 8), _mm512_and_epi64(t0v27, c7));\
    t0v27 = _mm512_or_epi64(_mm512_and_epi64(t0v22, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v30, c6), 8));\
    t0v22 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v22, c7), 8), _mm512_and_epi64(t0v30, c7));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v20, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v28, c6), 8));\
    t0v20 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v20, c7), 8), _mm512_and_epi64(t0v28, c7));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v23, c6), 8));\
    t0v15 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c7), 8), _mm512_and_epi64(t0v23, c7));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v18, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v26, c6), 8));\
    t0v18 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v18, c7), 8), _mm512_and_epi64(t0v26, c7));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v24, c6), 8));\
    t0v16 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c7), 8), _mm512_and_epi64(t0v24, c7));\
    t0v24 = _mm512_or_epi64(_mm512_and_epi64(t0v17, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v25, c6), 8));\
    t0v17 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v17, c7), 8), _mm512_and_epi64(t0v25, c7));\
    t0v25 = _mm512_or_epi64(_mm512_and_epi64(t0v29, c8), _mm512_slli_epi32(t0v9, 16));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(t0v29, 16), _mm512_and_epi64(t0v9, c9));\
    t0v29 = _mm512_or_epi64(_mm512_and_epi64(t0v31, c8), _mm512_slli_epi32(t0v21, 16));\
    t0v21 = _mm512_or_epi64(_mm512_srli_epi32(t0v31, 16), _mm512_and_epi64(t0v21, c9));\
    t0v31 = _mm512_or_epi64(_mm512_and_epi64(t0v32, c8), _mm512_slli_epi32(t0v27, 16));\
    t0v27 = _mm512_or_epi64(_mm512_srli_epi32(t0v32, 16), _mm512_and_epi64(t0v27, c9));\
    t0v32 = _mm512_or_epi64(_mm512_and_epi64(t0v14, c8), _mm512_slli_epi32(t0v30, 16));\
    t0v14 = _mm512_or_epi64(_mm512_srli_epi32(t0v14, 16), _mm512_and_epi64(t0v30, c9));\
    t0v30 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c8), _mm512_slli_epi32(t0v28, 16));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(t0v12, 16), _mm512_and_epi64(t0v28, c9));\
    t0v28 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c8), _mm512_slli_epi32(t0v23, 16));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(t0v7, 16), _mm512_and_epi64(t0v23, c9));\
    t0v23 = _mm512_or_epi64(_mm512_and_epi64(t0v10, c8), _mm512_slli_epi32(t0v26, 16));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(t0v10, 16), _mm512_and_epi64(t0v26, c9));\
    t0v26 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c8), _mm512_slli_epi32(t0v24, 16));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(t0v8, 16), _mm512_and_epi64(t0v24, c9));\
    t0v24 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c8), _mm512_slli_epi32(t0v13, 16));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(t0v5, 16), _mm512_and_epi64(t0v13, c9));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v11, c8), _mm512_slli_epi32(t0v19, 16));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(t0v11, 16), _mm512_and_epi64(t0v19, c9));\
    t0v19 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c8), _mm512_slli_epi32(t0v22, 16));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(t0v6, 16), _mm512_and_epi64(t0v22, c9));\
    t0v22 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c8), _mm512_slli_epi32(t0v20, 16));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(t0v4, 16), _mm512_and_epi64(t0v20, c9));\
    t0v20 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c8), _mm512_slli_epi32(t0v15, 16));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(t0v3, 16), _mm512_and_epi64(t0v15, c9));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c8), _mm512_slli_epi32(t0v18, 16));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(t0v2, 16), _mm512_and_epi64(t0v18, c9));\
    t0v18 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c8), _mm512_slli_epi32(t0v16, 16));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(t0v0, 16), _mm512_and_epi64(t0v16, c9));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c8), _mm512_slli_epi32(t0v17, 16));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(t0v1, 16), _mm512_and_epi64(t0v17, c9));\
    t0v17 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v25 = _mm512_permutex2var_epi32(t0v25, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 0)), t0v32);\
    t0v31 = _mm512_permutex2var_epi32(t0v31, (*(const __m512i*)(transform_const4_tbl + 16)), t0v32);\
    t0v32 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 0)), t0v28);\
    t0v28 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 16)), t0v28);\
    t0v30 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 0)), t0v26);\
    t0v23 = _mm512_permutex2var_epi32(t0v23, (*(const __m512i*)(transform_const4_tbl + 16)), t0v26);\
    t0v26 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v24 = _mm512_permutex2var_epi32(t0v19, (*(const __m512i*)(transform_const4_tbl + 0)), t0v22);\
    t0v19 = _mm512_permutex2var_epi32(t0v19, (*(const __m512i*)(transform_const4_tbl + 16)), t0v22);\
    t0v22 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 0)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 16)), t0v15);\
    t0v20 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v16);\
    t0v16 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v16);\
    t0v18 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v27, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v27, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v27 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v7 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v12 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v8);\
    t0v8 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v8);\
    t0v10 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v5 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v6 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v2);\
    t0v2 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v2);\
    t0v3 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v1);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v1);\
    t0v1 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v32 = _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v26 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v20);\
    t0v22 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v21);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v27 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v3);\
    t0v6 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v31);\
    t0v25 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v19);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v16);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v4);\
    t0v5 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v0);\
    t0v2 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v32 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v22 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v11);\
    t0v27 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v28 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v14);\
    t0v16 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v5);\
    t0v5 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v5);\
    t0v8 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v24 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v12);\
    t0v18 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v3);\
    t0v10 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v23);\
    t0v25 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v15);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v15);\
    t0v15 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v7);\
    t0v7 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v7);\
    t0v9 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v0);\
    _mm512_storeu_epi64(&((dest[0])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v29));\
    _mm512_storeu_epi64(&((dest[16])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v29));\
    _mm512_storeu_epi64(&((dest[1])), _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v22));\
    _mm512_storeu_epi64(&((dest[17])), _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v22));\
    _mm512_storeu_epi64(&((dest[2])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v31));\
    _mm512_storeu_epi64(&((dest[18])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v31));\
    _mm512_storeu_epi64(&((dest[3])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v16));\
    _mm512_storeu_epi64(&((dest[19])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v16));\
    _mm512_storeu_epi64(&((dest[4])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v30));\
    _mm512_storeu_epi64(&((dest[20])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v30));\
    _mm512_storeu_epi64(&((dest[5])), _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v18));\
    _mm512_storeu_epi64(&((dest[21])), _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v18));\
    _mm512_storeu_epi64(&((dest[6])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v25));\
    _mm512_storeu_epi64(&((dest[22])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v25));\
    _mm512_storeu_epi64(&((dest[7])), _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v9));\
    _mm512_storeu_epi64(&((dest[23])), _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v9));\
    _mm512_storeu_epi64(&((dest[8])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v26));\
    _mm512_storeu_epi64(&((dest[24])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v26));\
    _mm512_storeu_epi64(&((dest[9])), _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v11));\
    _mm512_storeu_epi64(&((dest[25])), _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v11));\
    _mm512_storeu_epi64(&((dest[10])), _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v19));\
    _mm512_storeu_epi64(&((dest[26])), _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v19));\
    _mm512_storeu_epi64(&((dest[11])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v5));\
    _mm512_storeu_epi64(&((dest[27])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v5));\
    _mm512_storeu_epi64(&((dest[12])), _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v20));\
    _mm512_storeu_epi64(&((dest[28])), _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v20));\
    _mm512_storeu_epi64(&((dest[13])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v3));\
    _mm512_storeu_epi64(&((dest[29])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v3));\
    _mm512_storeu_epi64(&((dest[14])), _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v13));\
    _mm512_storeu_epi64(&((dest[30])), _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v13));\
    _mm512_storeu_epi64(&((dest[15])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v0));\
    _mm512_storeu_epi64(&((dest[31])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v0));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    __m512i* dest = (__m512i*)(D);\
    const __m512i c8 = (*(const __m512i*)(transform_const2_tbl + 16*4));\
    const __m512i c0 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*5));\
    const __m512i c6 = (*(const __m512i*)(transform_const_tbl + 16*6));\
    const __m512i c7 = (*(const __m512i*)(transform_const_tbl + 16*7));\
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
    t0v0 = _mm512_or_epi64(_mm512_and_epi64((S0), c0), _mm512_slli_epi32(_mm512_and_epi64((S1), c0), 1));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S0), c1), 1), _mm512_and_epi64((S1), c1));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64((S2), c0), _mm512_slli_epi32(_mm512_and_epi64((S3), c0), 1));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S2), c1), 1), _mm512_and_epi64((S3), c1));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64((S4), c0), _mm512_slli_epi32(_mm512_and_epi64((S5), c0), 1));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S4), c1), 1), _mm512_and_epi64((S5), c1));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64((S6), c0), _mm512_slli_epi32(_mm512_and_epi64((S7), c0), 1));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S6), c1), 1), _mm512_and_epi64((S7), c1));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64((S8), c0), _mm512_slli_epi32(_mm512_and_epi64((S9), c0), 1));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S8), c1), 1), _mm512_and_epi64((S9), c1));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64((S10), c0), _mm512_slli_epi32(_mm512_and_epi64((S11), c0), 1));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S10), c1), 1), _mm512_and_epi64((S11), c1));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64((S12), c0), _mm512_slli_epi32(_mm512_and_epi64((S13), c0), 1));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S12), c1), 1), _mm512_and_epi64((S13), c1));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64((S14), c0), _mm512_slli_epi32(_mm512_and_epi64((S15), c0), 1));\
    t0v15 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S14), c1), 1), _mm512_and_epi64((S15), c1));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c2), 2));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c3), 2), _mm512_and_epi64(t0v2, c3));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c2), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c3), 2), _mm512_and_epi64(t0v3, c3));\
    t0v3 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c2), 2));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c3), 2), _mm512_and_epi64(t0v6, c3));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v5, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c2), 2));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v5, c3), 2), _mm512_and_epi64(t0v7, c3));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c2), 2));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c3), 2), _mm512_and_epi64(t0v10, c3));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c2), 2));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c3), 2), _mm512_and_epi64(t0v11, c3));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v12, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c2), 2));\
    t0v12 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v12, c3), 2), _mm512_and_epi64(t0v14, c3));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v13, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v15, c2), 2));\
    t0v13 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v13, c3), 2), _mm512_and_epi64(t0v15, c3));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c4), 4));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c5), 4), _mm512_and_epi64(t0v3, c5));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v6, c4), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c5), 4), _mm512_and_epi64(t0v6, c5));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c4), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c5), 4), _mm512_and_epi64(t0v4, c5));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c4), 4));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c5), 4), _mm512_and_epi64(t0v5, c5));\
    t0v5 = _mm512_or_epi64(_mm512_and_epi64(t0v7, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c4), 4));\
    t0v7 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v7, c5), 4), _mm512_and_epi64(t0v11, c5));\
    t0v11 = _mm512_or_epi64(_mm512_and_epi64(t0v10, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c4), 4));\
    t0v10 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v10, c5), 4), _mm512_and_epi64(t0v14, c5));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v8, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c4), 4));\
    t0v8 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v8, c5), 4), _mm512_and_epi64(t0v12, c5));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v9, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v13, c4), 4));\
    t0v9 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v9, c5), 4), _mm512_and_epi64(t0v13, c5));\
    t0v13 = _mm512_or_epi64(_mm512_and_epi64(t0v15, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c6), 8));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v15, c7), 8), _mm512_and_epi64(t0v5, c7));\
    t0v15 = _mm512_or_epi64(_mm512_and_epi64(t0v16, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v11, c6), 8));\
    t0v11 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v16, c7), 8), _mm512_and_epi64(t0v11, c7));\
    t0v16 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v14, c6), 8));\
    t0v6 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c7), 8), _mm512_and_epi64(t0v14, c7));\
    t0v14 = _mm512_or_epi64(_mm512_and_epi64(t0v4, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v12, c6), 8));\
    t0v4 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v4, c7), 8), _mm512_and_epi64(t0v12, c7));\
    t0v12 = _mm512_or_epi64(_mm512_and_epi64(t0v3, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c6), 8));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v3, c7), 8), _mm512_and_epi64(t0v7, c7));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v10, c6), 8));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c7), 8), _mm512_and_epi64(t0v10, c7));\
    t0v10 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v8, c6), 8));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c7), 8), _mm512_and_epi64(t0v8, c7));\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c6), _mm512_slli_epi32(_mm512_and_epi64(t0v9, c6), 8));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c7), 8), _mm512_and_epi64(t0v9, c7));\
    t0v9 = _mm512_and_epi64(t0v13, c8);\
    t0v13 = _mm512_srli_epi32(t0v13, 16);\
    t0v17 = _mm512_and_epi64(t0v15, c8);\
    t0v15 = _mm512_srli_epi32(t0v15, 16);\
    t0v18 = _mm512_and_epi64(t0v16, c8);\
    t0v16 = _mm512_srli_epi32(t0v16, 16);\
    t0v19 = _mm512_and_epi64(t0v14, c8);\
    t0v14 = _mm512_srli_epi32(t0v14, 16);\
    t0v20 = _mm512_and_epi64(t0v12, c8);\
    t0v12 = _mm512_srli_epi32(t0v12, 16);\
    t0v21 = _mm512_and_epi64(t0v7, c8);\
    t0v7 = _mm512_srli_epi32(t0v7, 16);\
    t0v22 = _mm512_and_epi64(t0v10, c8);\
    t0v10 = _mm512_srli_epi32(t0v10, 16);\
    t0v23 = _mm512_and_epi64(t0v8, c8);\
    t0v8 = _mm512_srli_epi32(t0v8, 16);\
    t0v24 = _mm512_and_epi64(t0v5, c8);\
    t0v5 = _mm512_srli_epi32(t0v5, 16);\
    t0v25 = _mm512_and_epi64(t0v11, c8);\
    t0v11 = _mm512_srli_epi32(t0v11, 16);\
    t0v26 = _mm512_and_epi64(t0v6, c8);\
    t0v6 = _mm512_srli_epi32(t0v6, 16);\
    t0v27 = _mm512_and_epi64(t0v4, c8);\
    t0v4 = _mm512_srli_epi32(t0v4, 16);\
    t0v28 = _mm512_and_epi64(t0v3, c8);\
    t0v3 = _mm512_srli_epi32(t0v3, 16);\
    t0v29 = _mm512_and_epi64(t0v2, c8);\
    t0v2 = _mm512_srli_epi32(t0v2, 16);\
    t0v30 = _mm512_and_epi64(t0v0, c8);\
    t0v0 = _mm512_srli_epi32(t0v0, 16);\
    t0v31 = _mm512_and_epi64(t0v1, c8);\
    t0v1 = _mm512_srli_epi32(t0v1, 16);\
    t0v32 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v17);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v17);\
    t0v17 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v20 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 0)), t0v23);\
    t0v22 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 16)), t0v23);\
    t0v23 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v25);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v25);\
    t0v25 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v28 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 0)), t0v31);\
    t0v30 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 16)), t0v31);\
    t0v31 = _mm512_permutex2var_epi32(t0v13, (*(const __m512i*)(transform_const4_tbl + 0)), t0v15);\
    t0v13 = _mm512_permutex2var_epi32(t0v13, (*(const __m512i*)(transform_const4_tbl + 16)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v14);\
    t0v14 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v14);\
    t0v16 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v7 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v12 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v8);\
    t0v8 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v8);\
    t0v10 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v5 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v6 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v2);\
    t0v2 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v2);\
    t0v3 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v1);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v1);\
    t0v1 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v17);\
    t0v32 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v21);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v23 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v27 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v15 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v31 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v16 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v3);\
    t0v6 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v18);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v22);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v24 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v28 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v4);\
    t0v5 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v0);\
    t0v2 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v32);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v32);\
    t0v32 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v29 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v11);\
    t0v16 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v14);\
    t0v30 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v5);\
    t0v5 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v5);\
    t0v8 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v23 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v12);\
    t0v15 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v3);\
    t0v10 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v24 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v7);\
    t0v7 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v7);\
    t0v13 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v0);\
    _mm512_storeu_epi64(&((dest[0])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v32));\
    _mm512_storeu_epi64(&((dest[16])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v32));\
    _mm512_storeu_epi64(&((dest[1])), _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v31));\
    _mm512_storeu_epi64(&((dest[17])), _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v31));\
    _mm512_storeu_epi64(&((dest[2])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v18));\
    _mm512_storeu_epi64(&((dest[18])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v18));\
    _mm512_storeu_epi64(&((dest[3])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v30));\
    _mm512_storeu_epi64(&((dest[19])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v30));\
    _mm512_storeu_epi64(&((dest[4])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v19));\
    _mm512_storeu_epi64(&((dest[20])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v19));\
    _mm512_storeu_epi64(&((dest[5])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v15));\
    _mm512_storeu_epi64(&((dest[21])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v15));\
    _mm512_storeu_epi64(&((dest[6])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v20));\
    _mm512_storeu_epi64(&((dest[22])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v20));\
    _mm512_storeu_epi64(&((dest[7])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v13));\
    _mm512_storeu_epi64(&((dest[23])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v13));\
    _mm512_storeu_epi64(&((dest[8])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v21));\
    _mm512_storeu_epi64(&((dest[24])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v21));\
    _mm512_storeu_epi64(&((dest[9])), _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v11));\
    _mm512_storeu_epi64(&((dest[25])), _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v11));\
    _mm512_storeu_epi64(&((dest[10])), _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v22));\
    _mm512_storeu_epi64(&((dest[26])), _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v22));\
    _mm512_storeu_epi64(&((dest[11])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v5));\
    _mm512_storeu_epi64(&((dest[27])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v5));\
    _mm512_storeu_epi64(&((dest[12])), _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v23));\
    _mm512_storeu_epi64(&((dest[28])), _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v23));\
    _mm512_storeu_epi64(&((dest[13])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v3));\
    _mm512_storeu_epi64(&((dest[29])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v3));\
    _mm512_storeu_epi64(&((dest[14])), _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v24));\
    _mm512_storeu_epi64(&((dest[30])), _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v24));\
    _mm512_storeu_epi64(&((dest[15])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v0));\
    _mm512_storeu_epi64(&((dest[31])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v0));\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    __m512i* dest = (__m512i*)(D);\
    const __m512i c6 = (*(const __m512i*)(transform_const2_tbl + 16*3));\
    const __m512i c0 = (*(const __m512i*)(transform_const_tbl + 16*0));\
    const __m512i c1 = (*(const __m512i*)(transform_const_tbl + 16*1));\
    const __m512i c2 = (*(const __m512i*)(transform_const_tbl + 16*2));\
    const __m512i c3 = (*(const __m512i*)(transform_const_tbl + 16*3));\
    const __m512i c4 = (*(const __m512i*)(transform_const_tbl + 16*4));\
    const __m512i c5 = (*(const __m512i*)(transform_const_tbl + 16*5));\
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
    t0v0 = _mm512_or_epi64(_mm512_and_epi64((S0), c0), _mm512_slli_epi32(_mm512_and_epi64((S1), c0), 1));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S0), c1), 1), _mm512_and_epi64((S1), c1));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64((S2), c0), _mm512_slli_epi32(_mm512_and_epi64((S3), c0), 1));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S2), c1), 1), _mm512_and_epi64((S3), c1));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64((S4), c0), _mm512_slli_epi32(_mm512_and_epi64((S5), c0), 1));\
    t0v5 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64((S4), c1), 1), _mm512_and_epi64((S5), c1));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v2, c2), 2));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c3), 2), _mm512_and_epi64(t0v2, c3));\
    t0v2 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c2), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c2), 2));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c3), 2), _mm512_and_epi64(t0v3, c3));\
    t0v3 = _mm512_and_epi64(t0v4, c2);\
    t0v4 = _mm512_srli_epi32(_mm512_and_epi64(t0v4, c3), 2);\
    t0v7 = _mm512_and_epi64(t0v5, c2);\
    t0v5 = _mm512_srli_epi32(_mm512_and_epi64(t0v5, c3), 2);\
    t0v8 = _mm512_or_epi64(_mm512_and_epi64(t0v6, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v3, c4), 4));\
    t0v3 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v6, c5), 4), _mm512_and_epi64(t0v3, c5));\
    t0v6 = _mm512_or_epi64(_mm512_and_epi64(t0v2, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v7, c4), 4));\
    t0v2 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v2, c5), 4), _mm512_and_epi64(t0v7, c5));\
    t0v7 = _mm512_or_epi64(_mm512_and_epi64(t0v0, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v4, c4), 4));\
    t0v0 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v0, c5), 4), _mm512_and_epi64(t0v4, c5));\
    t0v4 = _mm512_or_epi64(_mm512_and_epi64(t0v1, c4), _mm512_slli_epi32(_mm512_and_epi64(t0v5, c4), 4));\
    t0v1 = _mm512_or_epi64(_mm512_srli_epi32(_mm512_and_epi64(t0v1, c5), 4), _mm512_and_epi64(t0v5, c5));\
    t0v5 = _mm512_and_epi64(t0v8, c6);\
    t0v9 = _mm512_and_epi64(_mm512_srli_epi32(t0v8, 8), c6);\
    t0v10 = _mm512_and_epi64(_mm512_srli_epi32(t0v8, 16), c6);\
    t0v8 = _mm512_srli_epi32(t0v8, 24);\
    t0v11 = _mm512_and_epi64(t0v6, c6);\
    t0v12 = _mm512_and_epi64(_mm512_srli_epi32(t0v6, 8), c6);\
    t0v13 = _mm512_and_epi64(_mm512_srli_epi32(t0v6, 16), c6);\
    t0v6 = _mm512_srli_epi32(t0v6, 24);\
    t0v14 = _mm512_and_epi64(t0v7, c6);\
    t0v15 = _mm512_and_epi64(_mm512_srli_epi32(t0v7, 8), c6);\
    t0v16 = _mm512_and_epi64(_mm512_srli_epi32(t0v7, 16), c6);\
    t0v7 = _mm512_srli_epi32(t0v7, 24);\
    t0v17 = _mm512_and_epi64(t0v4, c6);\
    t0v18 = _mm512_and_epi64(_mm512_srli_epi32(t0v4, 8), c6);\
    t0v19 = _mm512_and_epi64(_mm512_srli_epi32(t0v4, 16), c6);\
    t0v4 = _mm512_srli_epi32(t0v4, 24);\
    t0v20 = _mm512_and_epi64(t0v3, c6);\
    t0v21 = _mm512_and_epi64(_mm512_srli_epi32(t0v3, 8), c6);\
    t0v22 = _mm512_and_epi64(_mm512_srli_epi32(t0v3, 16), c6);\
    t0v3 = _mm512_srli_epi32(t0v3, 24);\
    t0v23 = _mm512_and_epi64(t0v2, c6);\
    t0v24 = _mm512_and_epi64(_mm512_srli_epi32(t0v2, 8), c6);\
    t0v25 = _mm512_and_epi64(_mm512_srli_epi32(t0v2, 16), c6);\
    t0v2 = _mm512_srli_epi32(t0v2, 24);\
    t0v26 = _mm512_and_epi64(t0v0, c6);\
    t0v27 = _mm512_and_epi64(_mm512_srli_epi32(t0v0, 8), c6);\
    t0v28 = _mm512_and_epi64(_mm512_srli_epi32(t0v0, 16), c6);\
    t0v0 = _mm512_srli_epi32(t0v0, 24);\
    t0v29 = _mm512_and_epi64(t0v1, c6);\
    t0v30 = _mm512_and_epi64(_mm512_srli_epi32(t0v1, 8), c6);\
    t0v31 = _mm512_and_epi64(_mm512_srli_epi32(t0v1, 16), c6);\
    t0v1 = _mm512_srli_epi32(t0v1, 24);\
    t0v32 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v5 = _mm512_permutex2var_epi32(t0v5, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v14, (*(const __m512i*)(transform_const4_tbl + 0)), t0v17);\
    t0v14 = _mm512_permutex2var_epi32(t0v14, (*(const __m512i*)(transform_const4_tbl + 16)), t0v17);\
    t0v17 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 0)), t0v23);\
    t0v20 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 16)), t0v23);\
    t0v23 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 0)), t0v12);\
    t0v9 = _mm512_permutex2var_epi32(t0v9, (*(const __m512i*)(transform_const4_tbl + 16)), t0v12);\
    t0v12 = _mm512_permutex2var_epi32(t0v15, (*(const __m512i*)(transform_const4_tbl + 0)), t0v18);\
    t0v15 = _mm512_permutex2var_epi32(t0v15, (*(const __m512i*)(transform_const4_tbl + 16)), t0v18);\
    t0v18 = _mm512_permutex2var_epi32(t0v21, (*(const __m512i*)(transform_const4_tbl + 0)), t0v24);\
    t0v21 = _mm512_permutex2var_epi32(t0v21, (*(const __m512i*)(transform_const4_tbl + 16)), t0v24);\
    t0v24 = _mm512_permutex2var_epi32(t0v27, (*(const __m512i*)(transform_const4_tbl + 0)), t0v30);\
    t0v27 = _mm512_permutex2var_epi32(t0v27, (*(const __m512i*)(transform_const4_tbl + 16)), t0v30);\
    t0v30 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 0)), t0v25);\
    t0v22 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 16)), t0v25);\
    t0v25 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 0)), t0v31);\
    t0v28 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 16)), t0v31);\
    t0v31 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v6);\
    t0v6 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v6);\
    t0v8 = _mm512_permutex2var_epi32(t0v7, (*(const __m512i*)(transform_const4_tbl + 0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi32(t0v7, (*(const __m512i*)(transform_const4_tbl + 16)), t0v4);\
    t0v7 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 0)), t0v2);\
    t0v2 = _mm512_permutex2var_epi32(t0v3, (*(const __m512i*)(transform_const4_tbl + 16)), t0v2);\
    t0v3 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v1);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v1);\
    t0v1 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v11);\
    t0v32 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v23);\
    t0v17 = _mm512_permutex2var_epi64(t0v17, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v23);\
    t0v23 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v12);\
    t0v29 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v24);\
    t0v18 = _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v24);\
    t0v24 = _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v30 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v8);\
    t0v8 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v8);\
    t0v31 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v3);\
    t0v7 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v15);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v15);\
    t0v15 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v27);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v16);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v28);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v28);\
    t0v28 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v4);\
    t0v4 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v4);\
    t0v6 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v0);\
    t0v2 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v32);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v32);\
    t0v32 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v29);\
    t0v23 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v30);\
    t0v24 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v31);\
    t0v25 = _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v14);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v15);\
    t0v15 = _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v15);\
    t0v26 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v16);\
    t0v16 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v16);\
    t0v27 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v6);\
    t0v6 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v6);\
    t0v28 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v3);\
    t0v8 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v21);\
    t0v9 = _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v22);\
    t0v10 = _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v0);\
    t0v0 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v0);\
    _mm512_storeu_epi64(&((dest[0])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v32));\
    _mm512_storeu_epi64(&((dest[16])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v32));\
    _mm512_storeu_epi64(&((dest[1])), _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v30));\
    _mm512_storeu_epi64(&((dest[17])), _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v30));\
    _mm512_storeu_epi64(&((dest[2])), _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v14));\
    _mm512_storeu_epi64(&((dest[18])), _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v14));\
    _mm512_storeu_epi64(&((dest[3])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v27));\
    _mm512_storeu_epi64(&((dest[19])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v27));\
    _mm512_storeu_epi64(&((dest[4])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v17));\
    _mm512_storeu_epi64(&((dest[20])), _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v17));\
    _mm512_storeu_epi64(&((dest[5])), _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v19));\
    _mm512_storeu_epi64(&((dest[21])), _mm512_permutex2var_epi64(t0v18, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v19));\
    _mm512_storeu_epi64(&((dest[6])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v20));\
    _mm512_storeu_epi64(&((dest[22])), _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v20));\
    _mm512_storeu_epi64(&((dest[7])), _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v22));\
    _mm512_storeu_epi64(&((dest[23])), _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v22));\
    _mm512_storeu_epi64(&((dest[8])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v23));\
    _mm512_storeu_epi64(&((dest[24])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v23));\
    _mm512_storeu_epi64(&((dest[9])), _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v25));\
    _mm512_storeu_epi64(&((dest[25])), _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v25));\
    _mm512_storeu_epi64(&((dest[10])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v15));\
    _mm512_storeu_epi64(&((dest[26])), _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v15));\
    _mm512_storeu_epi64(&((dest[11])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v6));\
    _mm512_storeu_epi64(&((dest[27])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v6));\
    _mm512_storeu_epi64(&((dest[12])), _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v12));\
    _mm512_storeu_epi64(&((dest[28])), _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v12));\
    _mm512_storeu_epi64(&((dest[13])), _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v3));\
    _mm512_storeu_epi64(&((dest[29])), _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v3));\
    _mm512_storeu_epi64(&((dest[14])), _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v9));\
    _mm512_storeu_epi64(&((dest[30])), _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v9));\
    _mm512_storeu_epi64(&((dest[15])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v0));\
    _mm512_storeu_epi64(&((dest[31])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v0));\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    __m512i* dest = (__m512i*)(D);\
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
    t0v0 = _mm512_and_epi64((S0), c0);\
    t0v1 = _mm512_and_epi64(_mm512_srli_epi32((S0), 1), c0);\
    t0v2 = _mm512_and_epi64(_mm512_srli_epi32((S0), 2), c0);\
    t0v3 = _mm512_and_epi64(_mm512_srli_epi32((S0), 3), c0);\
    t0v4 = _mm512_and_epi64(_mm512_srli_epi32((S0), 4), c0);\
    t0v5 = _mm512_and_epi64(_mm512_srli_epi32((S0), 5), c0);\
    t0v6 = _mm512_and_epi64(_mm512_srli_epi32((S0), 6), c0);\
    t0v7 = _mm512_and_epi64(_mm512_srli_epi32((S0), 7), c0);\
    t0v8 = _mm512_and_epi64(_mm512_srli_epi32((S0), 8), c0);\
    t0v9 = _mm512_and_epi64(_mm512_srli_epi32((S0), 9), c0);\
    t0v10 = _mm512_and_epi64(_mm512_srli_epi32((S0), 10), c0);\
    t0v11 = _mm512_and_epi64(_mm512_srli_epi32((S0), 11), c0);\
    t0v12 = _mm512_and_epi64(_mm512_srli_epi32((S0), 12), c0);\
    t0v13 = _mm512_and_epi64(_mm512_srli_epi32((S0), 13), c0);\
    t0v14 = _mm512_and_epi64(_mm512_srli_epi32((S0), 14), c0);\
    t0v15 = _mm512_and_epi64(_mm512_srli_epi32((S0), 15), c0);\
    t0v16 = _mm512_and_epi64(_mm512_srli_epi32((S0), 16), c0);\
    t0v17 = _mm512_and_epi64(_mm512_srli_epi32((S0), 17), c0);\
    t0v18 = _mm512_and_epi64(_mm512_srli_epi32((S0), 18), c0);\
    t0v19 = _mm512_and_epi64(_mm512_srli_epi32((S0), 19), c0);\
    t0v20 = _mm512_and_epi64(_mm512_srli_epi32((S0), 20), c0);\
    t0v21 = _mm512_and_epi64(_mm512_srli_epi32((S0), 21), c0);\
    t0v22 = _mm512_and_epi64(_mm512_srli_epi32((S0), 22), c0);\
    t0v23 = _mm512_and_epi64(_mm512_srli_epi32((S0), 23), c0);\
    t0v24 = _mm512_and_epi64(_mm512_srli_epi32((S0), 24), c0);\
    t0v25 = _mm512_and_epi64(_mm512_srli_epi32((S0), 25), c0);\
    t0v26 = _mm512_and_epi64(_mm512_srli_epi32((S0), 26), c0);\
    t0v27 = _mm512_and_epi64(_mm512_srli_epi32((S0), 27), c0);\
    t0v28 = _mm512_and_epi64(_mm512_srli_epi32((S0), 28), c0);\
    t0v29 = _mm512_and_epi64(_mm512_srli_epi32((S0), 29), c0);\
    t0v30 = _mm512_and_epi64(_mm512_srli_epi32((S0), 30), c0);\
    t0v31 = _mm512_srli_epi32((S0), 31);\
    t0v32 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 0)), t0v1);\
    t0v0 = _mm512_permutex2var_epi32(t0v0, (*(const __m512i*)(transform_const4_tbl + 16)), t0v1);\
    t0v1 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 0)), t0v3);\
    t0v2 = _mm512_permutex2var_epi32(t0v2, (*(const __m512i*)(transform_const4_tbl + 16)), t0v3);\
    t0v3 = _mm512_permutex2var_epi32(t0v4, (*(const __m512i*)(transform_const4_tbl + 0)), t0v5);\
    t0v4 = _mm512_permutex2var_epi32(t0v4, (*(const __m512i*)(transform_const4_tbl + 16)), t0v5);\
    t0v5 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 0)), t0v7);\
    t0v6 = _mm512_permutex2var_epi32(t0v6, (*(const __m512i*)(transform_const4_tbl + 16)), t0v7);\
    t0v7 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 0)), t0v9);\
    t0v8 = _mm512_permutex2var_epi32(t0v8, (*(const __m512i*)(transform_const4_tbl + 16)), t0v9);\
    t0v9 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 0)), t0v11);\
    t0v10 = _mm512_permutex2var_epi32(t0v10, (*(const __m512i*)(transform_const4_tbl + 16)), t0v11);\
    t0v11 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 0)), t0v13);\
    t0v12 = _mm512_permutex2var_epi32(t0v12, (*(const __m512i*)(transform_const4_tbl + 16)), t0v13);\
    t0v13 = _mm512_permutex2var_epi32(t0v14, (*(const __m512i*)(transform_const4_tbl + 0)), t0v15);\
    t0v14 = _mm512_permutex2var_epi32(t0v14, (*(const __m512i*)(transform_const4_tbl + 16)), t0v15);\
    t0v15 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 0)), t0v17);\
    t0v16 = _mm512_permutex2var_epi32(t0v16, (*(const __m512i*)(transform_const4_tbl + 16)), t0v17);\
    t0v17 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 0)), t0v19);\
    t0v18 = _mm512_permutex2var_epi32(t0v18, (*(const __m512i*)(transform_const4_tbl + 16)), t0v19);\
    t0v19 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 0)), t0v21);\
    t0v20 = _mm512_permutex2var_epi32(t0v20, (*(const __m512i*)(transform_const4_tbl + 16)), t0v21);\
    t0v21 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 0)), t0v23);\
    t0v22 = _mm512_permutex2var_epi32(t0v22, (*(const __m512i*)(transform_const4_tbl + 16)), t0v23);\
    t0v23 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 0)), t0v25);\
    t0v24 = _mm512_permutex2var_epi32(t0v24, (*(const __m512i*)(transform_const4_tbl + 16)), t0v25);\
    t0v25 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 0)), t0v27);\
    t0v26 = _mm512_permutex2var_epi32(t0v26, (*(const __m512i*)(transform_const4_tbl + 16)), t0v27);\
    t0v27 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 0)), t0v29);\
    t0v28 = _mm512_permutex2var_epi32(t0v28, (*(const __m512i*)(transform_const4_tbl + 16)), t0v29);\
    t0v29 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 0)), t0v31);\
    t0v30 = _mm512_permutex2var_epi32(t0v30, (*(const __m512i*)(transform_const4_tbl + 16)), t0v31);\
    t0v31 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v1);\
    t0v1 = _mm512_permutex2var_epi64(t0v32, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v1);\
    t0v32 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v5);\
    t0v3 = _mm512_permutex2var_epi64(t0v3, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v5);\
    t0v5 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v9);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v13);\
    t0v11 = _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v13);\
    t0v13 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v17);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v21);\
    t0v19 = _mm512_permutex2var_epi64(t0v19, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v21);\
    t0v21 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v25);\
    t0v23 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v29);\
    t0v27 = _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v29);\
    t0v29 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v2);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v2);\
    t0v2 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v6);\
    t0v4 = _mm512_permutex2var_epi64(t0v4, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v6);\
    t0v6 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v10);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v14);\
    t0v12 = _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v14);\
    t0v14 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v18);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v22);\
    t0v20 = _mm512_permutex2var_epi64(t0v20, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v22);\
    t0v22 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v26);\
    t0v24 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*0)), t0v30);\
    t0v28 = _mm512_permutex2var_epi64(t0v28, (*(const __m512i*)(transform_const3_tbl + 8*1)), t0v30);\
    t0v30 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v32);\
    t0v31 = _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v32);\
    t0v32 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v9);\
    t0v5 = _mm512_permutex2var_epi64(t0v5, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v9);\
    t0v9 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v17);\
    t0v13 = _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v17);\
    t0v17 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v25);\
    t0v21 = _mm512_permutex2var_epi64(t0v21, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v25);\
    t0v25 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v2);\
    t0v2 = _mm512_permutex2var_epi64(t0v29, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v2);\
    t0v29 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v10);\
    t0v6 = _mm512_permutex2var_epi64(t0v6, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v10);\
    t0v10 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v18);\
    t0v14 = _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v18);\
    t0v18 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v26);\
    t0v22 = _mm512_permutex2var_epi64(t0v22, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v26);\
    t0v26 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v3);\
    t0v1 = _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v3);\
    t0v3 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v11);\
    t0v7 = _mm512_permutex2var_epi64(t0v7, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v11);\
    t0v11 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v19);\
    t0v15 = _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v19);\
    t0v19 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v27);\
    t0v23 = _mm512_permutex2var_epi64(t0v23, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v27);\
    t0v27 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v4);\
    t0v0 = _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v4);\
    t0v4 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v12);\
    t0v8 = _mm512_permutex2var_epi64(t0v8, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v12);\
    t0v12 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v20);\
    t0v16 = _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v20);\
    t0v20 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*2)), t0v28);\
    t0v24 = _mm512_permutex2var_epi64(t0v24, (*(const __m512i*)(transform_const3_tbl + 8*3)), t0v28);\
    _mm512_storeu_epi64(&((dest[0])), _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v32));\
    _mm512_storeu_epi64(&((dest[16])), _mm512_permutex2var_epi64(t0v30, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v32));\
    _mm512_storeu_epi64(&((dest[1])), _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v17));\
    _mm512_storeu_epi64(&((dest[17])), _mm512_permutex2var_epi64(t0v9, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v17));\
    _mm512_storeu_epi64(&((dest[2])), _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v29));\
    _mm512_storeu_epi64(&((dest[18])), _mm512_permutex2var_epi64(t0v25, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v29));\
    _mm512_storeu_epi64(&((dest[3])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v18));\
    _mm512_storeu_epi64(&((dest[19])), _mm512_permutex2var_epi64(t0v10, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v18));\
    _mm512_storeu_epi64(&((dest[4])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v3));\
    _mm512_storeu_epi64(&((dest[20])), _mm512_permutex2var_epi64(t0v26, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v3));\
    _mm512_storeu_epi64(&((dest[5])), _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v19));\
    _mm512_storeu_epi64(&((dest[21])), _mm512_permutex2var_epi64(t0v11, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v19));\
    _mm512_storeu_epi64(&((dest[6])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v4));\
    _mm512_storeu_epi64(&((dest[22])), _mm512_permutex2var_epi64(t0v27, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v4));\
    _mm512_storeu_epi64(&((dest[7])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v20));\
    _mm512_storeu_epi64(&((dest[23])), _mm512_permutex2var_epi64(t0v12, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v20));\
    _mm512_storeu_epi64(&((dest[8])), _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v5));\
    _mm512_storeu_epi64(&((dest[24])), _mm512_permutex2var_epi64(t0v31, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v5));\
    _mm512_storeu_epi64(&((dest[9])), _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v21));\
    _mm512_storeu_epi64(&((dest[25])), _mm512_permutex2var_epi64(t0v13, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v21));\
    _mm512_storeu_epi64(&((dest[10])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v6));\
    _mm512_storeu_epi64(&((dest[26])), _mm512_permutex2var_epi64(t0v2, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v6));\
    _mm512_storeu_epi64(&((dest[11])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v22));\
    _mm512_storeu_epi64(&((dest[27])), _mm512_permutex2var_epi64(t0v14, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v22));\
    _mm512_storeu_epi64(&((dest[12])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v7));\
    _mm512_storeu_epi64(&((dest[28])), _mm512_permutex2var_epi64(t0v1, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v7));\
    _mm512_storeu_epi64(&((dest[13])), _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v23));\
    _mm512_storeu_epi64(&((dest[29])), _mm512_permutex2var_epi64(t0v15, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v23));\
    _mm512_storeu_epi64(&((dest[14])), _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v8));\
    _mm512_storeu_epi64(&((dest[30])), _mm512_permutex2var_epi64(t0v0, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v8));\
    _mm512_storeu_epi64(&((dest[15])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*4)), t0v24));\
    _mm512_storeu_epi64(&((dest[31])), _mm512_permutex2var_epi64(t0v16, (*(const __m512i*)(transform_const3_tbl + 8*5)), t0v24));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_ARM_NEON.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
{\
    uint32x4_t* dest = (uint32x4_t*)(D);\
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
    t0v0 = vorrq_u32(vandq_u32((S0), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S1), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S0), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S1), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v2 = vorrq_u32(vandq_u32((S2), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S3), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S2), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S3), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v4 = vorrq_u32(vandq_u32((S4), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S5), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S4), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S5), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v6 = vorrq_u32(vandq_u32((S6), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S7), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S6), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S7), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v8 = vorrq_u32(vandq_u32((S8), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S9), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S8), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S9), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v10 = vorrq_u32(vandq_u32((S10), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S11), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S10), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S11), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v12 = vorrq_u32(vandq_u32((S12), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S13), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S12), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S13), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v14 = vorrq_u32(vandq_u32((S14), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S15), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S14), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S15), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v16 = vorrq_u32(vandq_u32((S16), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S17), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S16), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S17), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v18 = vorrq_u32(vandq_u32((S18), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S19), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v19 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S18), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S19), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v20 = vorrq_u32(vandq_u32((S20), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S21), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v21 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S20), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S21), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v22 = vorrq_u32(vandq_u32((S22), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S23), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v23 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S22), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S23), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v24 = vorrq_u32(vandq_u32((S24), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S25), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S24), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S25), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v26 = vorrq_u32(vandq_u32((S26), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S27), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v27 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S26), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S27), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v28 = vorrq_u32(vandq_u32((S28), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S29), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v29 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S28), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S29), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v30 = vorrq_u32(vandq_u32((S30), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S31), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v31 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S30), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S31), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v2 = vorrq_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v3, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v6, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v6 = vorrq_u32(vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v7, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v7 = vorrq_u32(vandq_u32(t0v8, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v10, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v10 = vorrq_u32(vandq_u32(t0v9, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v11, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v12, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v14, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v14 = vorrq_u32(vandq_u32(t0v13, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v15, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v18, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v16, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v18, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v17, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v19, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v17, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v19, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v20, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v22, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v20, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v22, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v21, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v23, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v21 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v21, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v23, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v24, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v26, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v24, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v26, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v25, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v27, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v25, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v27, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v28, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v30, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v28, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v30, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v29, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v31, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v29 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v29, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v31, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v32), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v2, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v6), 4)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v2), 4)), vandq_u32(t0v6, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v4), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v4, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v4 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v5 = vorrq_u32(vandq_u32(t0v7, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v11), 4)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v7), 4)), vandq_u32(t0v11, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v11 = vorrq_u32(vandq_u32(t0v10, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v14), 4)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v10), 4)), vandq_u32(t0v14, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v8, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v12), 4)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v8), 4)), vandq_u32(t0v12, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v9, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v13), 4)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v9), 4)), vandq_u32(t0v13, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v15, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v19), 4)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v15), 4)), vandq_u32(t0v19, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v19 = vorrq_u32(vandq_u32(t0v18, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v22), 4)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v18), 4)), vandq_u32(t0v22, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v22 = vorrq_u32(vandq_u32(t0v16, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v20), 4)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v16), 4)), vandq_u32(t0v20, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v20 = vorrq_u32(vandq_u32(t0v17, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v21), 4)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v17), 4)), vandq_u32(t0v21, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v21 = vorrq_u32(vandq_u32(t0v23, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v27), 4)));\
    t0v23 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v23), 4)), vandq_u32(t0v27, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v27 = vorrq_u32(vandq_u32(t0v26, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v30), 4)));\
    t0v26 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v26), 4)), vandq_u32(t0v30, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v24, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v28), 4)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v24), 4)), vandq_u32(t0v28, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v25, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v29), 4)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v25), 4)), vandq_u32(t0v29, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v29 = vorrq_u32(vandq_u32(t0v31, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v5), 8)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v31), 8)), vandq_u32(t0v5, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v11), 8)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v32), 8)), vandq_u32(t0v11, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v6, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v14), 8)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v6), 8)), vandq_u32(t0v14, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v4, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v12), 8)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v4), 8)), vandq_u32(t0v12, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v3, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v7), 8)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v3), 8)), vandq_u32(t0v7, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v7 = vorrq_u32(vandq_u32(t0v2, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v10), 8)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v2), 8)), vandq_u32(t0v10, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v10 = vorrq_u32(vandq_u32(t0v0, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v8), 8)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v0), 8)), vandq_u32(t0v8, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v8 = vorrq_u32(vandq_u32(t0v1, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v9), 8)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v1), 8)), vandq_u32(t0v9, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v9 = vorrq_u32(vandq_u32(t0v13, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v21), 8)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v13), 8)), vandq_u32(t0v21, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v21 = vorrq_u32(vandq_u32(t0v19, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v27), 8)));\
    t0v19 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v19), 8)), vandq_u32(t0v27, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v27 = vorrq_u32(vandq_u32(t0v22, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v30), 8)));\
    t0v22 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v22), 8)), vandq_u32(t0v30, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v20, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v28), 8)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v20), 8)), vandq_u32(t0v28, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v15, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v23), 8)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v15), 8)), vandq_u32(t0v23, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v23 = vorrq_u32(vandq_u32(t0v18, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v26), 8)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v18), 8)), vandq_u32(t0v26, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v26 = vorrq_u32(vandq_u32(t0v16, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v24), 8)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v16), 8)), vandq_u32(t0v24, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v24 = vorrq_u32(vandq_u32(t0v17, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v25), 8)));\
    t0v17 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v17), 8)), vandq_u32(t0v25, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v25 = vorrq_u32(vandq_u32(t0v29, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v9, 16));\
    t0v9 = vorrq_u32(vshrq_n_u32(t0v29, 16), vandq_u32(t0v9, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v29 = vorrq_u32(vandq_u32(t0v31, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v21, 16));\
    t0v21 = vorrq_u32(vshrq_n_u32(t0v31, 16), vandq_u32(t0v21, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v31 = vorrq_u32(vandq_u32(t0v32, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v27, 16));\
    t0v27 = vorrq_u32(vshrq_n_u32(t0v32, 16), vandq_u32(t0v27, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v32 = vorrq_u32(vandq_u32(t0v14, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v30, 16));\
    t0v14 = vorrq_u32(vshrq_n_u32(t0v14, 16), vandq_u32(t0v30, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v30 = vorrq_u32(vandq_u32(t0v12, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v28, 16));\
    t0v12 = vorrq_u32(vshrq_n_u32(t0v12, 16), vandq_u32(t0v28, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v28 = vorrq_u32(vandq_u32(t0v7, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v23, 16));\
    t0v7 = vorrq_u32(vshrq_n_u32(t0v7, 16), vandq_u32(t0v23, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v23 = vorrq_u32(vandq_u32(t0v10, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v26, 16));\
    t0v10 = vorrq_u32(vshrq_n_u32(t0v10, 16), vandq_u32(t0v26, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v26 = vorrq_u32(vandq_u32(t0v8, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v24, 16));\
    t0v8 = vorrq_u32(vshrq_n_u32(t0v8, 16), vandq_u32(t0v24, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v24 = vorrq_u32(vandq_u32(t0v5, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v13, 16));\
    t0v5 = vorrq_u32(vshrq_n_u32(t0v5, 16), vandq_u32(t0v13, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v11, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v19, 16));\
    t0v11 = vorrq_u32(vshrq_n_u32(t0v11, 16), vandq_u32(t0v19, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v19 = vorrq_u32(vandq_u32(t0v6, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v22, 16));\
    t0v6 = vorrq_u32(vshrq_n_u32(t0v6, 16), vandq_u32(t0v22, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v22 = vorrq_u32(vandq_u32(t0v4, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v20, 16));\
    t0v4 = vorrq_u32(vshrq_n_u32(t0v4, 16), vandq_u32(t0v20, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v20 = vorrq_u32(vandq_u32(t0v3, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v15, 16));\
    t0v3 = vorrq_u32(vshrq_n_u32(t0v3, 16), vandq_u32(t0v15, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v15 = vorrq_u32(vandq_u32(t0v2, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v18, 16));\
    t0v2 = vorrq_u32(vshrq_n_u32(t0v2, 16), vandq_u32(t0v18, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v18 = vorrq_u32(vandq_u32(t0v0, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v16, 16));\
    t0v0 = vorrq_u32(vshrq_n_u32(t0v0, 16), vandq_u32(t0v16, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v16 = vorrq_u32(vandq_u32(t0v1, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU }), vshlq_n_u32(t0v17, 16));\
    t0v1 = vorrq_u32(vshrq_n_u32(t0v1, 16), vandq_u32(t0v17, { 0xffff0000U, 0xffff0000U, 0xffff0000U, 0xffff0000U }));\
    t0v17 = vorrq_u32(vandq_u32(t0v25, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v25 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v25), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v31, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v32), 32)));\
    t0v31 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v31), 32)), vandq_u32(t0v32, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v32 = vorrq_u32(vandq_u32(t0v30, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v28), 32)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v30), 32)), vandq_u32(t0v28, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v23, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v26), 32)));\
    t0v23 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v23), 32)), vandq_u32(t0v26, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v26 = vorrq_u32(vandq_u32(t0v24, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v13), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v24), 32)), vandq_u32(t0v13, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v19, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v22), 32)));\
    t0v19 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v19), 32)), vandq_u32(t0v22, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v22 = vorrq_u32(vandq_u32(t0v20, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v15), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v20), 32)), vandq_u32(t0v15, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v20 = vorrq_u32(vandq_u32(t0v18, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v16), 32)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v18), 32)), vandq_u32(t0v16, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v27, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v14), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v27), 32)), vandq_u32(t0v14, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v7), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v7, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v12 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v8), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v8, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v10 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v11), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v11, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v4), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v4, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v6 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v2), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v2, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v1), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v1, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    (dest[0]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v17), vreinterpretq_u64_u32(t0v29)));\
    (dest[16]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v17), vreinterpretq_u64_u32(t0v29)));\
    (dest[1]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v30)));\
    (dest[17]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v30)));\
    (dest[2]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v26), vreinterpretq_u64_u32(t0v24)));\
    (dest[18]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v26), vreinterpretq_u64_u32(t0v24)));\
    (dest[3]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v22), vreinterpretq_u64_u32(t0v20)));\
    (dest[19]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v22), vreinterpretq_u64_u32(t0v20)));\
    (dest[4]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v18), vreinterpretq_u64_u32(t0v21)));\
    (dest[20]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v18), vreinterpretq_u64_u32(t0v21)));\
    (dest[5]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v12)));\
    (dest[21]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v12)));\
    (dest[6]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v11)));\
    (dest[22]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v11)));\
    (dest[7]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v3)));\
    (dest[23]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v3)));\
    (dest[8]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v25), vreinterpretq_u64_u32(t0v31)));\
    (dest[24]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v25), vreinterpretq_u64_u32(t0v31)));\
    (dest[9]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v23)));\
    (dest[25]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v23)));\
    (dest[10]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v13), vreinterpretq_u64_u32(t0v19)));\
    (dest[26]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v13), vreinterpretq_u64_u32(t0v19)));\
    (dest[11]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v15), vreinterpretq_u64_u32(t0v16)));\
    (dest[27]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v15), vreinterpretq_u64_u32(t0v16)));\
    (dest[12]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v14)));\
    (dest[28]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v14)));\
    (dest[13]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v8)));\
    (dest[29]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v8)));\
    (dest[14]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v4)));\
    (dest[30]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v4)));\
    (dest[15]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
    (dest[31]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
{\
    uint32x4_t* dest = (uint32x4_t*)(D);\
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
    t0v0 = vorrq_u32(vandq_u32((S0), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S1), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S0), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S1), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v2 = vorrq_u32(vandq_u32((S2), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S3), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S2), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S3), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v4 = vorrq_u32(vandq_u32((S4), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S5), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S4), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S5), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v6 = vorrq_u32(vandq_u32((S6), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S7), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S6), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S7), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v8 = vorrq_u32(vandq_u32((S8), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S9), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S8), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S9), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v10 = vorrq_u32(vandq_u32((S10), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S11), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S10), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S11), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v12 = vorrq_u32(vandq_u32((S12), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S13), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S12), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S13), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v14 = vorrq_u32(vandq_u32((S14), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S15), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S14), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S15), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v2 = vorrq_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v3, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v6, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v6, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v6 = vorrq_u32(vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v7, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v7, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v7 = vorrq_u32(vandq_u32(t0v8, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v10, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v8, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v10, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v10 = vorrq_u32(vandq_u32(t0v9, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v11, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v9, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v11, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v12, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v14, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v12, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v14, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v14 = vorrq_u32(vandq_u32(t0v13, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v15, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v13, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v15, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v16), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v16 = vorrq_u32(vandq_u32(t0v2, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v6), 4)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v2), 4)), vandq_u32(t0v6, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v4), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v4, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v4 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v5 = vorrq_u32(vandq_u32(t0v7, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v11), 4)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v7), 4)), vandq_u32(t0v11, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v11 = vorrq_u32(vandq_u32(t0v10, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v14), 4)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v10), 4)), vandq_u32(t0v14, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v8, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v12), 4)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v8), 4)), vandq_u32(t0v12, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v9, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v13), 4)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v9), 4)), vandq_u32(t0v13, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v13 = vorrq_u32(vandq_u32(t0v15, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v5), 8)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v15), 8)), vandq_u32(t0v5, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v11), 8)));\
    t0v11 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v16), 8)), vandq_u32(t0v11, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v16 = vorrq_u32(vandq_u32(t0v6, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v14), 8)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v6), 8)), vandq_u32(t0v14, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v14 = vorrq_u32(vandq_u32(t0v4, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v12), 8)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v4), 8)), vandq_u32(t0v12, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v12 = vorrq_u32(vandq_u32(t0v3, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v7), 8)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v3), 8)), vandq_u32(t0v7, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v7 = vorrq_u32(vandq_u32(t0v2, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v10), 8)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v2), 8)), vandq_u32(t0v10, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v10 = vorrq_u32(vandq_u32(t0v0, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v8), 8)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v0), 8)), vandq_u32(t0v8, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v8 = vorrq_u32(vandq_u32(t0v1, { 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU, 0x00ff00ffU }), vreinterpretq_u32_u16(vshlq_n_u16(vreinterpretq_u16_u32(t0v9), 8)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u16(vshrq_n_u16(vreinterpretq_u16_u32(t0v1), 8)), vandq_u32(t0v9, { 0xff00ff00U, 0xff00ff00U, 0xff00ff00U, 0xff00ff00U }));\
    t0v9 = vandq_u32(t0v13, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v13 = vshrq_n_u32(t0v13, 16);\
    t0v17 = vandq_u32(t0v15, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v15 = vshrq_n_u32(t0v15, 16);\
    t0v18 = vandq_u32(t0v16, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v16 = vshrq_n_u32(t0v16, 16);\
    t0v19 = vandq_u32(t0v14, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v14 = vshrq_n_u32(t0v14, 16);\
    t0v20 = vandq_u32(t0v12, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v12 = vshrq_n_u32(t0v12, 16);\
    t0v21 = vandq_u32(t0v7, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v7 = vshrq_n_u32(t0v7, 16);\
    t0v22 = vandq_u32(t0v10, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v10 = vshrq_n_u32(t0v10, 16);\
    t0v23 = vandq_u32(t0v8, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v8 = vshrq_n_u32(t0v8, 16);\
    t0v24 = vandq_u32(t0v5, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v5 = vshrq_n_u32(t0v5, 16);\
    t0v25 = vandq_u32(t0v11, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v11 = vshrq_n_u32(t0v11, 16);\
    t0v26 = vandq_u32(t0v6, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v6 = vshrq_n_u32(t0v6, 16);\
    t0v27 = vandq_u32(t0v4, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v4 = vshrq_n_u32(t0v4, 16);\
    t0v28 = vandq_u32(t0v3, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v3 = vshrq_n_u32(t0v3, 16);\
    t0v29 = vandq_u32(t0v2, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v2 = vshrq_n_u32(t0v2, 16);\
    t0v30 = vandq_u32(t0v0, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v0 = vshrq_n_u32(t0v0, 16);\
    t0v31 = vandq_u32(t0v1, { 0x0000ffffU, 0x0000ffffU, 0x0000ffffU, 0x0000ffffU });\
    t0v1 = vshrq_n_u32(t0v1, 16);\
    t0v32 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v18, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v18), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v20, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v20), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v22, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v22 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v22), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v24, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v24), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v26, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v26 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v26), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v28, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v28), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v30, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v30 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v30), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v31 = vorrq_u32(vandq_u32(t0v13, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v15), 32)));\
    t0v13 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v13), 32)), vandq_u32(t0v15, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v14), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v16), 32)), vandq_u32(t0v14, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v16 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v7), 32)));\
    t0v7 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v7, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v12 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v8), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v8, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v10 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v11), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v11, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v4), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v4, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v6 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v2), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v2, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v1), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v1, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    (dest[0]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v17)));\
    (dest[16]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v17)));\
    (dest[1]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v21)));\
    (dest[17]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v21)));\
    (dest[2]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v23), vreinterpretq_u64_u32(t0v25)));\
    (dest[18]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v23), vreinterpretq_u64_u32(t0v25)));\
    (dest[3]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v29)));\
    (dest[19]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v29)));\
    (dest[4]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v31), vreinterpretq_u64_u32(t0v15)));\
    (dest[20]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v31), vreinterpretq_u64_u32(t0v15)));\
    (dest[5]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v16), vreinterpretq_u64_u32(t0v12)));\
    (dest[21]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v16), vreinterpretq_u64_u32(t0v12)));\
    (dest[6]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v11)));\
    (dest[22]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v11)));\
    (dest[7]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v3)));\
    (dest[23]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v3)));\
    (dest[8]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v18)));\
    (dest[24]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v18)));\
    (dest[9]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v22)));\
    (dest[25]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v22)));\
    (dest[10]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v24), vreinterpretq_u64_u32(t0v26)));\
    (dest[26]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v24), vreinterpretq_u64_u32(t0v26)));\
    (dest[11]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v30)));\
    (dest[27]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v30)));\
    (dest[12]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v13), vreinterpretq_u64_u32(t0v14)));\
    (dest[28]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v13), vreinterpretq_u64_u32(t0v14)));\
    (dest[13]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v8)));\
    (dest[29]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v8)));\
    (dest[14]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v4)));\
    (dest[30]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v4)));\
    (dest[15]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
    (dest[31]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
{\
    uint32x4_t* dest = (uint32x4_t*)(D);\
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
    t0v0 = vorrq_u32(vandq_u32((S0), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S1), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S0), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S1), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v2 = vorrq_u32(vandq_u32((S2), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S3), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S2), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S3), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v4 = vorrq_u32(vandq_u32((S4), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32((S5), { 0x55555555U, 0x55555555U, 0x55555555U, 0x55555555U })), 1)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32((S4), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU })), 1)), vandq_u32((S5), { 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU, 0xaaaaaaaaU }));\
    t0v6 = vorrq_u32(vandq_u32(t0v0, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v2, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v0, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v2, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v2 = vorrq_u32(vandq_u32(t0v1, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v3, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U })), 2)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v1, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2)), vandq_u32(t0v3, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU }));\
    t0v3 = vandq_u32(t0v4, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U });\
    t0v4 = vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v4, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2));\
    t0v7 = vandq_u32(t0v5, { 0x33333333U, 0x33333333U, 0x33333333U, 0x33333333U });\
    t0v5 = vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(vandq_u32(t0v5, { 0xccccccccU, 0xccccccccU, 0xccccccccU, 0xccccccccU })), 2));\
    t0v8 = vorrq_u32(vandq_u32(t0v6, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v3), 4)));\
    t0v3 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v6), 4)), vandq_u32(t0v3, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v6 = vorrq_u32(vandq_u32(t0v2, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v7), 4)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v2), 4)), vandq_u32(t0v7, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v7 = vorrq_u32(vandq_u32(t0v0, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v4), 4)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v0), 4)), vandq_u32(t0v4, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v4 = vorrq_u32(vandq_u32(t0v1, { 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU, 0x0f0f0f0fU }), vreinterpretq_u32_u8(vshlq_n_u8(vreinterpretq_u8_u32(t0v5), 4)));\
    t0v1 = vorrq_u32(vreinterpretq_u32_u8(vshrq_n_u8(vreinterpretq_u8_u32(t0v1), 4)), vandq_u32(t0v5, { 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U, 0xf0f0f0f0U }));\
    t0v5 = vandq_u32(t0v8, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v9 = vandq_u32(vshrq_n_u32(t0v8, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v10 = vandq_u32(vshrq_n_u32(t0v8, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v8 = vshrq_n_u32(t0v8, 24);\
    t0v11 = vandq_u32(t0v6, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v12 = vandq_u32(vshrq_n_u32(t0v6, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v13 = vandq_u32(vshrq_n_u32(t0v6, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v6 = vshrq_n_u32(t0v6, 24);\
    t0v14 = vandq_u32(t0v7, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v15 = vandq_u32(vshrq_n_u32(t0v7, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v16 = vandq_u32(vshrq_n_u32(t0v7, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v7 = vshrq_n_u32(t0v7, 24);\
    t0v17 = vandq_u32(t0v4, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v18 = vandq_u32(vshrq_n_u32(t0v4, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v19 = vandq_u32(vshrq_n_u32(t0v4, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v4 = vshrq_n_u32(t0v4, 24);\
    t0v20 = vandq_u32(t0v3, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v21 = vandq_u32(vshrq_n_u32(t0v3, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v22 = vandq_u32(vshrq_n_u32(t0v3, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v3 = vshrq_n_u32(t0v3, 24);\
    t0v23 = vandq_u32(t0v2, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v24 = vandq_u32(vshrq_n_u32(t0v2, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v25 = vandq_u32(vshrq_n_u32(t0v2, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v2 = vshrq_n_u32(t0v2, 24);\
    t0v26 = vandq_u32(t0v0, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v27 = vandq_u32(vshrq_n_u32(t0v0, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v28 = vandq_u32(vshrq_n_u32(t0v0, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v0 = vshrq_n_u32(t0v0, 24);\
    t0v29 = vandq_u32(t0v1, { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v30 = vandq_u32(vshrq_n_u32(t0v1, 8), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v31 = vandq_u32(vshrq_n_u32(t0v1, 16), { 0x000000ffU, 0x000000ffU, 0x000000ffU, 0x000000ffU });\
    t0v1 = vshrq_n_u32(t0v1, 24);\
    t0v32 = vorrq_u32(vandq_u32(t0v5, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v11), 32)));\
    t0v5 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v5), 32)), vandq_u32(t0v11, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v20, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v20), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v26, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v26 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v26), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v9, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v12), 32)));\
    t0v9 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v9), 32)), vandq_u32(t0v12, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v12 = vorrq_u32(vandq_u32(t0v15, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v18), 32)));\
    t0v15 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v15), 32)), vandq_u32(t0v18, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v18 = vorrq_u32(vandq_u32(t0v21, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v24), 32)));\
    t0v21 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v21), 32)), vandq_u32(t0v24, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v24 = vorrq_u32(vandq_u32(t0v27, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v30), 32)));\
    t0v27 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v27), 32)), vandq_u32(t0v30, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v30 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v13), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v13, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v13 = vorrq_u32(vandq_u32(t0v16, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v16), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v22, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v22 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v22), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v28, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v28), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v31 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v6), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v6, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v8 = vorrq_u32(vandq_u32(t0v7, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v4), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v7), 32)), vandq_u32(t0v4, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v7 = vorrq_u32(vandq_u32(t0v3, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v2), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v3), 32)), vandq_u32(t0v2, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v1), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v1, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    (dest[0]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v11)));\
    (dest[16]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v11)));\
    (dest[1]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v17), vreinterpretq_u64_u32(t0v23)));\
    (dest[17]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v17), vreinterpretq_u64_u32(t0v23)));\
    (dest[2]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v29), vreinterpretq_u64_u32(t0v12)));\
    (dest[18]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v29), vreinterpretq_u64_u32(t0v12)));\
    (dest[3]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v18), vreinterpretq_u64_u32(t0v24)));\
    (dest[19]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v18), vreinterpretq_u64_u32(t0v24)));\
    (dest[4]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v30), vreinterpretq_u64_u32(t0v13)));\
    (dest[20]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v30), vreinterpretq_u64_u32(t0v13)));\
    (dest[5]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v25)));\
    (dest[21]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v25)));\
    (dest[6]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v31), vreinterpretq_u64_u32(t0v8)));\
    (dest[22]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v31), vreinterpretq_u64_u32(t0v8)));\
    (dest[7]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v3)));\
    (dest[23]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v3)));\
    (dest[8]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v14)));\
    (dest[24]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v5), vreinterpretq_u64_u32(t0v14)));\
    (dest[9]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v26)));\
    (dest[25]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v26)));\
    (dest[10]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v15)));\
    (dest[26]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v9), vreinterpretq_u64_u32(t0v15)));\
    (dest[11]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v21), vreinterpretq_u64_u32(t0v27)));\
    (dest[27]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v21), vreinterpretq_u64_u32(t0v27)));\
    (dest[12]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v16)));\
    (dest[28]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v10), vreinterpretq_u64_u32(t0v16)));\
    (dest[13]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v22), vreinterpretq_u64_u32(t0v28)));\
    (dest[29]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v22), vreinterpretq_u64_u32(t0v28)));\
    (dest[14]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v4)));\
    (dest[30]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v6), vreinterpretq_u64_u32(t0v4)));\
    (dest[15]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
    (dest[31]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v2), vreinterpretq_u64_u32(t0v0)));\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    uint32x4_t* dest = (uint32x4_t*)(D);\
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
    t0v0 = vandq_u32((S0), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v1 = vandq_u32(vshrq_n_u32((S0), 1), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v2 = vandq_u32(vshrq_n_u32((S0), 2), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v3 = vandq_u32(vshrq_n_u32((S0), 3), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v4 = vandq_u32(vshrq_n_u32((S0), 4), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v5 = vandq_u32(vshrq_n_u32((S0), 5), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v6 = vandq_u32(vshrq_n_u32((S0), 6), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v7 = vandq_u32(vshrq_n_u32((S0), 7), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v8 = vandq_u32(vshrq_n_u32((S0), 8), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v9 = vandq_u32(vshrq_n_u32((S0), 9), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v10 = vandq_u32(vshrq_n_u32((S0), 10), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v11 = vandq_u32(vshrq_n_u32((S0), 11), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v12 = vandq_u32(vshrq_n_u32((S0), 12), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v13 = vandq_u32(vshrq_n_u32((S0), 13), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v14 = vandq_u32(vshrq_n_u32((S0), 14), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v15 = vandq_u32(vshrq_n_u32((S0), 15), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v16 = vandq_u32(vshrq_n_u32((S0), 16), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v17 = vandq_u32(vshrq_n_u32((S0), 17), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v18 = vandq_u32(vshrq_n_u32((S0), 18), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v19 = vandq_u32(vshrq_n_u32((S0), 19), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v20 = vandq_u32(vshrq_n_u32((S0), 20), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v21 = vandq_u32(vshrq_n_u32((S0), 21), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v22 = vandq_u32(vshrq_n_u32((S0), 22), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v23 = vandq_u32(vshrq_n_u32((S0), 23), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v24 = vandq_u32(vshrq_n_u32((S0), 24), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v25 = vandq_u32(vshrq_n_u32((S0), 25), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v26 = vandq_u32(vshrq_n_u32((S0), 26), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v27 = vandq_u32(vshrq_n_u32((S0), 27), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v28 = vandq_u32(vshrq_n_u32((S0), 28), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v29 = vandq_u32(vshrq_n_u32((S0), 29), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v30 = vandq_u32(vshrq_n_u32((S0), 30), { 0x00000001U, 0x00000001U, 0x00000001U, 0x00000001U });\
    t0v31 = vshrq_n_u32((S0), 31);\
    t0v32 = vorrq_u32(vandq_u32(t0v0, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v1), 32)));\
    t0v0 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v0), 32)), vandq_u32(t0v1, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v1 = vorrq_u32(vandq_u32(t0v2, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v3), 32)));\
    t0v2 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v2), 32)), vandq_u32(t0v3, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v3 = vorrq_u32(vandq_u32(t0v4, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v5), 32)));\
    t0v4 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v4), 32)), vandq_u32(t0v5, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v5 = vorrq_u32(vandq_u32(t0v6, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v7), 32)));\
    t0v6 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v6), 32)), vandq_u32(t0v7, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v7 = vorrq_u32(vandq_u32(t0v8, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v9), 32)));\
    t0v8 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v8), 32)), vandq_u32(t0v9, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v9 = vorrq_u32(vandq_u32(t0v10, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v11), 32)));\
    t0v10 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v10), 32)), vandq_u32(t0v11, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v11 = vorrq_u32(vandq_u32(t0v12, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v13), 32)));\
    t0v12 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v12), 32)), vandq_u32(t0v13, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v13 = vorrq_u32(vandq_u32(t0v14, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v15), 32)));\
    t0v14 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v14), 32)), vandq_u32(t0v15, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v15 = vorrq_u32(vandq_u32(t0v16, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v17), 32)));\
    t0v16 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v16), 32)), vandq_u32(t0v17, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v17 = vorrq_u32(vandq_u32(t0v18, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v19), 32)));\
    t0v18 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v18), 32)), vandq_u32(t0v19, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v19 = vorrq_u32(vandq_u32(t0v20, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v21), 32)));\
    t0v20 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v20), 32)), vandq_u32(t0v21, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v21 = vorrq_u32(vandq_u32(t0v22, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v23), 32)));\
    t0v22 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v22), 32)), vandq_u32(t0v23, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v23 = vorrq_u32(vandq_u32(t0v24, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v25), 32)));\
    t0v24 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v24), 32)), vandq_u32(t0v25, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v25 = vorrq_u32(vandq_u32(t0v26, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v27), 32)));\
    t0v26 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v26), 32)), vandq_u32(t0v27, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v27 = vorrq_u32(vandq_u32(t0v28, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v29), 32)));\
    t0v28 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v28), 32)), vandq_u32(t0v29, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    t0v29 = vorrq_u32(vandq_u32(t0v30, { 0xffffffffU, 0x00000000U, 0xffffffffU, 0x00000000U }), vreinterpretq_u32_u64(vshlq_n_u64(vreinterpretq_u64_u32(t0v31), 32)));\
    t0v30 = vorrq_u32(vreinterpretq_u32_u64(vshrq_n_u64(vreinterpretq_u64_u32(t0v30), 32)), vandq_u32(t0v31, { 0x00000000U, 0xffffffffU, 0x00000000U, 0xffffffffU }));\
    (dest[0]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v1)));\
    (dest[16]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v32), vreinterpretq_u64_u32(t0v1)));\
    (dest[1]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v3), vreinterpretq_u64_u32(t0v5)));\
    (dest[17]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v3), vreinterpretq_u64_u32(t0v5)));\
    (dest[2]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v9)));\
    (dest[18]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v7), vreinterpretq_u64_u32(t0v9)));\
    (dest[3]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v11), vreinterpretq_u64_u32(t0v13)));\
    (dest[19]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v11), vreinterpretq_u64_u32(t0v13)));\
    (dest[4]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v15), vreinterpretq_u64_u32(t0v17)));\
    (dest[20]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v15), vreinterpretq_u64_u32(t0v17)));\
    (dest[5]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v21)));\
    (dest[21]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v19), vreinterpretq_u64_u32(t0v21)));\
    (dest[6]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v23), vreinterpretq_u64_u32(t0v25)));\
    (dest[22]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v23), vreinterpretq_u64_u32(t0v25)));\
    (dest[7]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v29)));\
    (dest[23]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v27), vreinterpretq_u64_u32(t0v29)));\
    (dest[8]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v0), vreinterpretq_u64_u32(t0v2)));\
    (dest[24]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v0), vreinterpretq_u64_u32(t0v2)));\
    (dest[9]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v4), vreinterpretq_u64_u32(t0v6)));\
    (dest[25]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v4), vreinterpretq_u64_u32(t0v6)));\
    (dest[10]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v8), vreinterpretq_u64_u32(t0v10)));\
    (dest[26]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v8), vreinterpretq_u64_u32(t0v10)));\
    (dest[11]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v12), vreinterpretq_u64_u32(t0v14)));\
    (dest[27]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v12), vreinterpretq_u64_u32(t0v14)));\
    (dest[12]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v16), vreinterpretq_u64_u32(t0v18)));\
    (dest[28]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v16), vreinterpretq_u64_u32(t0v18)));\
    (dest[13]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v22)));\
    (dest[29]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v20), vreinterpretq_u64_u32(t0v22)));\
    (dest[14]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v24), vreinterpretq_u64_u32(t0v26)));\
    (dest[30]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v24), vreinterpretq_u64_u32(t0v26)));\
    (dest[15]) = vreinterpretq_u32_u64(vzip1q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v30)));\
    (dest[31]) = vreinterpretq_u32_u64(vzip2q_u64(
            vreinterpretq_u64_u32(t0v28), vreinterpretq_u64_u32(t0v30)));\
}
"##,
        transform.out()
    );
    let mut transform = CLANG_TRANSFORM_OPENCL_U32.transform();
    transform.gen_output_transform(32);
    transform.gen_output_transform(16);
    transform.gen_output_transform(6);
    transform.gen_output_transform(1);
    assert_eq!(
        r##"#define OUTPUT_TRANSFORM_B32(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S25, S26, S27, S28, S29, S30, S31) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = (((S6) & 0x55555555U) | (((S7) & 0x55555555U) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaU) >> 1) | ((S7) & 0xaaaaaaaaU));\
    t0v8 = (((S8) & 0x55555555U) | (((S9) & 0x55555555U) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaU) >> 1) | ((S9) & 0xaaaaaaaaU));\
    t0v10 = (((S10) & 0x55555555U) | (((S11) & 0x55555555U) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaU) >> 1) | ((S11) & 0xaaaaaaaaU));\
    t0v12 = (((S12) & 0x55555555U) | (((S13) & 0x55555555U) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaU) >> 1) | ((S13) & 0xaaaaaaaaU));\
    t0v14 = (((S14) & 0x55555555U) | (((S15) & 0x55555555U) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaU) >> 1) | ((S15) & 0xaaaaaaaaU));\
    t0v16 = (((S16) & 0x55555555U) | (((S17) & 0x55555555U) << 1));\
    t0v17 = ((((S16) & 0xaaaaaaaaU) >> 1) | ((S17) & 0xaaaaaaaaU));\
    t0v18 = (((S18) & 0x55555555U) | (((S19) & 0x55555555U) << 1));\
    t0v19 = ((((S18) & 0xaaaaaaaaU) >> 1) | ((S19) & 0xaaaaaaaaU));\
    t0v20 = (((S20) & 0x55555555U) | (((S21) & 0x55555555U) << 1));\
    t0v21 = ((((S20) & 0xaaaaaaaaU) >> 1) | ((S21) & 0xaaaaaaaaU));\
    t0v22 = (((S22) & 0x55555555U) | (((S23) & 0x55555555U) << 1));\
    t0v23 = ((((S22) & 0xaaaaaaaaU) >> 1) | ((S23) & 0xaaaaaaaaU));\
    t0v24 = (((S24) & 0x55555555U) | (((S25) & 0x55555555U) << 1));\
    t0v25 = ((((S24) & 0xaaaaaaaaU) >> 1) | ((S25) & 0xaaaaaaaaU));\
    t0v26 = (((S26) & 0x55555555U) | (((S27) & 0x55555555U) << 1));\
    t0v27 = ((((S26) & 0xaaaaaaaaU) >> 1) | ((S27) & 0xaaaaaaaaU));\
    t0v28 = (((S28) & 0x55555555U) | (((S29) & 0x55555555U) << 1));\
    t0v29 = ((((S28) & 0xaaaaaaaaU) >> 1) | ((S29) & 0xaaaaaaaaU));\
    t0v30 = (((S30) & 0x55555555U) | (((S31) & 0x55555555U) << 1));\
    t0v31 = ((((S30) & 0xaaaaaaaaU) >> 1) | ((S31) & 0xaaaaaaaaU));\
    t0v32 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x33333333U) | ((t0v18 & 0x33333333U) << 2));\
    t0v16 = (((t0v16 & 0xccccccccU) >> 2) | (t0v18 & 0xccccccccU));\
    t0v18 = ((t0v17 & 0x33333333U) | ((t0v19 & 0x33333333U) << 2));\
    t0v17 = (((t0v17 & 0xccccccccU) >> 2) | (t0v19 & 0xccccccccU));\
    t0v19 = ((t0v20 & 0x33333333U) | ((t0v22 & 0x33333333U) << 2));\
    t0v20 = (((t0v20 & 0xccccccccU) >> 2) | (t0v22 & 0xccccccccU));\
    t0v22 = ((t0v21 & 0x33333333U) | ((t0v23 & 0x33333333U) << 2));\
    t0v21 = (((t0v21 & 0xccccccccU) >> 2) | (t0v23 & 0xccccccccU));\
    t0v23 = ((t0v24 & 0x33333333U) | ((t0v26 & 0x33333333U) << 2));\
    t0v24 = (((t0v24 & 0xccccccccU) >> 2) | (t0v26 & 0xccccccccU));\
    t0v26 = ((t0v25 & 0x33333333U) | ((t0v27 & 0x33333333U) << 2));\
    t0v25 = (((t0v25 & 0xccccccccU) >> 2) | (t0v27 & 0xccccccccU));\
    t0v27 = ((t0v28 & 0x33333333U) | ((t0v30 & 0x33333333U) << 2));\
    t0v28 = (((t0v28 & 0xccccccccU) >> 2) | (t0v30 & 0xccccccccU));\
    t0v30 = ((t0v29 & 0x33333333U) | ((t0v31 & 0x33333333U) << 2));\
    t0v29 = (((t0v29 & 0xccccccccU) >> 2) | (t0v31 & 0xccccccccU));\
    t0v31 = ((t0v32 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v32 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v32 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x0f0f0f0fU) | ((t0v19 & 0x0f0f0f0fU) << 4));\
    t0v15 = (((t0v15 & 0xf0f0f0f0U) >> 4) | (t0v19 & 0xf0f0f0f0U));\
    t0v19 = ((t0v18 & 0x0f0f0f0fU) | ((t0v22 & 0x0f0f0f0fU) << 4));\
    t0v18 = (((t0v18 & 0xf0f0f0f0U) >> 4) | (t0v22 & 0xf0f0f0f0U));\
    t0v22 = ((t0v16 & 0x0f0f0f0fU) | ((t0v20 & 0x0f0f0f0fU) << 4));\
    t0v16 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v20 & 0xf0f0f0f0U));\
    t0v20 = ((t0v17 & 0x0f0f0f0fU) | ((t0v21 & 0x0f0f0f0fU) << 4));\
    t0v17 = (((t0v17 & 0xf0f0f0f0U) >> 4) | (t0v21 & 0xf0f0f0f0U));\
    t0v21 = ((t0v23 & 0x0f0f0f0fU) | ((t0v27 & 0x0f0f0f0fU) << 4));\
    t0v23 = (((t0v23 & 0xf0f0f0f0U) >> 4) | (t0v27 & 0xf0f0f0f0U));\
    t0v27 = ((t0v26 & 0x0f0f0f0fU) | ((t0v30 & 0x0f0f0f0fU) << 4));\
    t0v26 = (((t0v26 & 0xf0f0f0f0U) >> 4) | (t0v30 & 0xf0f0f0f0U));\
    t0v30 = ((t0v24 & 0x0f0f0f0fU) | ((t0v28 & 0x0f0f0f0fU) << 4));\
    t0v24 = (((t0v24 & 0xf0f0f0f0U) >> 4) | (t0v28 & 0xf0f0f0f0U));\
    t0v28 = ((t0v25 & 0x0f0f0f0fU) | ((t0v29 & 0x0f0f0f0fU) << 4));\
    t0v25 = (((t0v25 & 0xf0f0f0f0U) >> 4) | (t0v29 & 0xf0f0f0f0U));\
    t0v29 = ((t0v31 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v31 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v31 = ((t0v32 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v32 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v32 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    t0v9 = ((t0v13 & 0x00ff00ffU) | ((t0v21 & 0x00ff00ffU) << 8));\
    t0v13 = (((t0v13 & 0xff00ff00U) >> 8) | (t0v21 & 0xff00ff00U));\
    t0v21 = ((t0v19 & 0x00ff00ffU) | ((t0v27 & 0x00ff00ffU) << 8));\
    t0v19 = (((t0v19 & 0xff00ff00U) >> 8) | (t0v27 & 0xff00ff00U));\
    t0v27 = ((t0v22 & 0x00ff00ffU) | ((t0v30 & 0x00ff00ffU) << 8));\
    t0v22 = (((t0v22 & 0xff00ff00U) >> 8) | (t0v30 & 0xff00ff00U));\
    t0v30 = ((t0v20 & 0x00ff00ffU) | ((t0v28 & 0x00ff00ffU) << 8));\
    t0v20 = (((t0v20 & 0xff00ff00U) >> 8) | (t0v28 & 0xff00ff00U));\
    t0v28 = ((t0v15 & 0x00ff00ffU) | ((t0v23 & 0x00ff00ffU) << 8));\
    t0v15 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v23 & 0xff00ff00U));\
    t0v23 = ((t0v18 & 0x00ff00ffU) | ((t0v26 & 0x00ff00ffU) << 8));\
    t0v18 = (((t0v18 & 0xff00ff00U) >> 8) | (t0v26 & 0xff00ff00U));\
    t0v26 = ((t0v16 & 0x00ff00ffU) | ((t0v24 & 0x00ff00ffU) << 8));\
    t0v16 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v24 & 0xff00ff00U));\
    t0v24 = ((t0v17 & 0x00ff00ffU) | ((t0v25 & 0x00ff00ffU) << 8));\
    t0v17 = (((t0v17 & 0xff00ff00U) >> 8) | (t0v25 & 0xff00ff00U));\
    ((D)[0]) = ((t0v29 & 0x0000ffffU) | (t0v9 << 16));\
    ((D)[16]) = ((t0v29 >> 16) | (t0v9 & 0xffff0000U));\
    ((D)[1]) = ((t0v31 & 0x0000ffffU) | (t0v21 << 16));\
    ((D)[17]) = ((t0v31 >> 16) | (t0v21 & 0xffff0000U));\
    ((D)[2]) = ((t0v32 & 0x0000ffffU) | (t0v27 << 16));\
    ((D)[18]) = ((t0v32 >> 16) | (t0v27 & 0xffff0000U));\
    ((D)[3]) = ((t0v14 & 0x0000ffffU) | (t0v30 << 16));\
    ((D)[19]) = ((t0v14 >> 16) | (t0v30 & 0xffff0000U));\
    ((D)[4]) = ((t0v12 & 0x0000ffffU) | (t0v28 << 16));\
    ((D)[20]) = ((t0v12 >> 16) | (t0v28 & 0xffff0000U));\
    ((D)[5]) = ((t0v7 & 0x0000ffffU) | (t0v23 << 16));\
    ((D)[21]) = ((t0v7 >> 16) | (t0v23 & 0xffff0000U));\
    ((D)[6]) = ((t0v10 & 0x0000ffffU) | (t0v26 << 16));\
    ((D)[22]) = ((t0v10 >> 16) | (t0v26 & 0xffff0000U));\
    ((D)[7]) = ((t0v8 & 0x0000ffffU) | (t0v24 << 16));\
    ((D)[23]) = ((t0v8 >> 16) | (t0v24 & 0xffff0000U));\
    ((D)[8]) = ((t0v5 & 0x0000ffffU) | (t0v13 << 16));\
    ((D)[24]) = ((t0v5 >> 16) | (t0v13 & 0xffff0000U));\
    ((D)[9]) = ((t0v11 & 0x0000ffffU) | (t0v19 << 16));\
    ((D)[25]) = ((t0v11 >> 16) | (t0v19 & 0xffff0000U));\
    ((D)[10]) = ((t0v6 & 0x0000ffffU) | (t0v22 << 16));\
    ((D)[26]) = ((t0v6 >> 16) | (t0v22 & 0xffff0000U));\
    ((D)[11]) = ((t0v4 & 0x0000ffffU) | (t0v20 << 16));\
    ((D)[27]) = ((t0v4 >> 16) | (t0v20 & 0xffff0000U));\
    ((D)[12]) = ((t0v3 & 0x0000ffffU) | (t0v15 << 16));\
    ((D)[28]) = ((t0v3 >> 16) | (t0v15 & 0xffff0000U));\
    ((D)[13]) = ((t0v2 & 0x0000ffffU) | (t0v18 << 16));\
    ((D)[29]) = ((t0v2 >> 16) | (t0v18 & 0xffff0000U));\
    ((D)[14]) = ((t0v0 & 0x0000ffffU) | (t0v16 << 16));\
    ((D)[30]) = ((t0v0 >> 16) | (t0v16 & 0xffff0000U));\
    ((D)[15]) = ((t0v1 & 0x0000ffffU) | (t0v17 << 16));\
    ((D)[31]) = ((t0v1 >> 16) | (t0v17 & 0xffff0000U));\
}
#define OUTPUT_TRANSFORM_B16(D, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = (((S6) & 0x55555555U) | (((S7) & 0x55555555U) << 1));\
    t0v7 = ((((S6) & 0xaaaaaaaaU) >> 1) | ((S7) & 0xaaaaaaaaU));\
    t0v8 = (((S8) & 0x55555555U) | (((S9) & 0x55555555U) << 1));\
    t0v9 = ((((S8) & 0xaaaaaaaaU) >> 1) | ((S9) & 0xaaaaaaaaU));\
    t0v10 = (((S10) & 0x55555555U) | (((S11) & 0x55555555U) << 1));\
    t0v11 = ((((S10) & 0xaaaaaaaaU) >> 1) | ((S11) & 0xaaaaaaaaU));\
    t0v12 = (((S12) & 0x55555555U) | (((S13) & 0x55555555U) << 1));\
    t0v13 = ((((S12) & 0xaaaaaaaaU) >> 1) | ((S13) & 0xaaaaaaaaU));\
    t0v14 = (((S14) & 0x55555555U) | (((S15) & 0x55555555U) << 1));\
    t0v15 = ((((S14) & 0xaaaaaaaaU) >> 1) | ((S15) & 0xaaaaaaaaU));\
    t0v16 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = ((t0v4 & 0x33333333U) | ((t0v6 & 0x33333333U) << 2));\
    t0v4 = (((t0v4 & 0xccccccccU) >> 2) | (t0v6 & 0xccccccccU));\
    t0v6 = ((t0v5 & 0x33333333U) | ((t0v7 & 0x33333333U) << 2));\
    t0v5 = (((t0v5 & 0xccccccccU) >> 2) | (t0v7 & 0xccccccccU));\
    t0v7 = ((t0v8 & 0x33333333U) | ((t0v10 & 0x33333333U) << 2));\
    t0v8 = (((t0v8 & 0xccccccccU) >> 2) | (t0v10 & 0xccccccccU));\
    t0v10 = ((t0v9 & 0x33333333U) | ((t0v11 & 0x33333333U) << 2));\
    t0v9 = (((t0v9 & 0xccccccccU) >> 2) | (t0v11 & 0xccccccccU));\
    t0v11 = ((t0v12 & 0x33333333U) | ((t0v14 & 0x33333333U) << 2));\
    t0v12 = (((t0v12 & 0xccccccccU) >> 2) | (t0v14 & 0xccccccccU));\
    t0v14 = ((t0v13 & 0x33333333U) | ((t0v15 & 0x33333333U) << 2));\
    t0v13 = (((t0v13 & 0xccccccccU) >> 2) | (t0v15 & 0xccccccccU));\
    t0v15 = ((t0v16 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v16 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v16 = ((t0v2 & 0x0f0f0f0fU) | ((t0v6 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v6 & 0xf0f0f0f0U));\
    t0v6 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    t0v5 = ((t0v7 & 0x0f0f0f0fU) | ((t0v11 & 0x0f0f0f0fU) << 4));\
    t0v7 = (((t0v7 & 0xf0f0f0f0U) >> 4) | (t0v11 & 0xf0f0f0f0U));\
    t0v11 = ((t0v10 & 0x0f0f0f0fU) | ((t0v14 & 0x0f0f0f0fU) << 4));\
    t0v10 = (((t0v10 & 0xf0f0f0f0U) >> 4) | (t0v14 & 0xf0f0f0f0U));\
    t0v14 = ((t0v8 & 0x0f0f0f0fU) | ((t0v12 & 0x0f0f0f0fU) << 4));\
    t0v8 = (((t0v8 & 0xf0f0f0f0U) >> 4) | (t0v12 & 0xf0f0f0f0U));\
    t0v12 = ((t0v9 & 0x0f0f0f0fU) | ((t0v13 & 0x0f0f0f0fU) << 4));\
    t0v9 = (((t0v9 & 0xf0f0f0f0U) >> 4) | (t0v13 & 0xf0f0f0f0U));\
    t0v13 = ((t0v15 & 0x00ff00ffU) | ((t0v5 & 0x00ff00ffU) << 8));\
    t0v5 = (((t0v15 & 0xff00ff00U) >> 8) | (t0v5 & 0xff00ff00U));\
    t0v15 = ((t0v16 & 0x00ff00ffU) | ((t0v11 & 0x00ff00ffU) << 8));\
    t0v11 = (((t0v16 & 0xff00ff00U) >> 8) | (t0v11 & 0xff00ff00U));\
    t0v16 = ((t0v6 & 0x00ff00ffU) | ((t0v14 & 0x00ff00ffU) << 8));\
    t0v6 = (((t0v6 & 0xff00ff00U) >> 8) | (t0v14 & 0xff00ff00U));\
    t0v14 = ((t0v4 & 0x00ff00ffU) | ((t0v12 & 0x00ff00ffU) << 8));\
    t0v4 = (((t0v4 & 0xff00ff00U) >> 8) | (t0v12 & 0xff00ff00U));\
    t0v12 = ((t0v3 & 0x00ff00ffU) | ((t0v7 & 0x00ff00ffU) << 8));\
    t0v3 = (((t0v3 & 0xff00ff00U) >> 8) | (t0v7 & 0xff00ff00U));\
    t0v7 = ((t0v2 & 0x00ff00ffU) | ((t0v10 & 0x00ff00ffU) << 8));\
    t0v2 = (((t0v2 & 0xff00ff00U) >> 8) | (t0v10 & 0xff00ff00U));\
    t0v10 = ((t0v0 & 0x00ff00ffU) | ((t0v8 & 0x00ff00ffU) << 8));\
    t0v0 = (((t0v0 & 0xff00ff00U) >> 8) | (t0v8 & 0xff00ff00U));\
    t0v8 = ((t0v1 & 0x00ff00ffU) | ((t0v9 & 0x00ff00ffU) << 8));\
    t0v1 = (((t0v1 & 0xff00ff00U) >> 8) | (t0v9 & 0xff00ff00U));\
    ((D)[0]) = (t0v13 & 0xffffU);\
    ((D)[16]) = (t0v13 >> 16);\
    ((D)[1]) = (t0v15 & 0xffffU);\
    ((D)[17]) = (t0v15 >> 16);\
    ((D)[2]) = (t0v16 & 0xffffU);\
    ((D)[18]) = (t0v16 >> 16);\
    ((D)[3]) = (t0v14 & 0xffffU);\
    ((D)[19]) = (t0v14 >> 16);\
    ((D)[4]) = (t0v12 & 0xffffU);\
    ((D)[20]) = (t0v12 >> 16);\
    ((D)[5]) = (t0v7 & 0xffffU);\
    ((D)[21]) = (t0v7 >> 16);\
    ((D)[6]) = (t0v10 & 0xffffU);\
    ((D)[22]) = (t0v10 >> 16);\
    ((D)[7]) = (t0v8 & 0xffffU);\
    ((D)[23]) = (t0v8 >> 16);\
    ((D)[8]) = (t0v5 & 0xffffU);\
    ((D)[24]) = (t0v5 >> 16);\
    ((D)[9]) = (t0v11 & 0xffffU);\
    ((D)[25]) = (t0v11 >> 16);\
    ((D)[10]) = (t0v6 & 0xffffU);\
    ((D)[26]) = (t0v6 >> 16);\
    ((D)[11]) = (t0v4 & 0xffffU);\
    ((D)[27]) = (t0v4 >> 16);\
    ((D)[12]) = (t0v3 & 0xffffU);\
    ((D)[28]) = (t0v3 >> 16);\
    ((D)[13]) = (t0v2 & 0xffffU);\
    ((D)[29]) = (t0v2 >> 16);\
    ((D)[14]) = (t0v0 & 0xffffU);\
    ((D)[30]) = (t0v0 >> 16);\
    ((D)[15]) = (t0v1 & 0xffffU);\
    ((D)[31]) = (t0v1 >> 16);\
}
#define OUTPUT_TRANSFORM_B6(D, S0, S1, S2, S3, S4, S5) \
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
    t0v0 = (((S0) & 0x55555555U) | (((S1) & 0x55555555U) << 1));\
    t0v1 = ((((S0) & 0xaaaaaaaaU) >> 1) | ((S1) & 0xaaaaaaaaU));\
    t0v2 = (((S2) & 0x55555555U) | (((S3) & 0x55555555U) << 1));\
    t0v3 = ((((S2) & 0xaaaaaaaaU) >> 1) | ((S3) & 0xaaaaaaaaU));\
    t0v4 = (((S4) & 0x55555555U) | (((S5) & 0x55555555U) << 1));\
    t0v5 = ((((S4) & 0xaaaaaaaaU) >> 1) | ((S5) & 0xaaaaaaaaU));\
    t0v6 = ((t0v0 & 0x33333333U) | ((t0v2 & 0x33333333U) << 2));\
    t0v0 = (((t0v0 & 0xccccccccU) >> 2) | (t0v2 & 0xccccccccU));\
    t0v2 = ((t0v1 & 0x33333333U) | ((t0v3 & 0x33333333U) << 2));\
    t0v1 = (((t0v1 & 0xccccccccU) >> 2) | (t0v3 & 0xccccccccU));\
    t0v3 = (t0v4 & 0x33333333U);\
    t0v4 = ((t0v4 & 0xccccccccU) >> 2);\
    t0v7 = (t0v5 & 0x33333333U);\
    t0v5 = ((t0v5 & 0xccccccccU) >> 2);\
    t0v8 = ((t0v6 & 0x0f0f0f0fU) | ((t0v3 & 0x0f0f0f0fU) << 4));\
    t0v3 = (((t0v6 & 0xf0f0f0f0U) >> 4) | (t0v3 & 0xf0f0f0f0U));\
    t0v6 = ((t0v2 & 0x0f0f0f0fU) | ((t0v7 & 0x0f0f0f0fU) << 4));\
    t0v2 = (((t0v2 & 0xf0f0f0f0U) >> 4) | (t0v7 & 0xf0f0f0f0U));\
    t0v7 = ((t0v0 & 0x0f0f0f0fU) | ((t0v4 & 0x0f0f0f0fU) << 4));\
    t0v0 = (((t0v0 & 0xf0f0f0f0U) >> 4) | (t0v4 & 0xf0f0f0f0U));\
    t0v4 = ((t0v1 & 0x0f0f0f0fU) | ((t0v5 & 0x0f0f0f0fU) << 4));\
    t0v1 = (((t0v1 & 0xf0f0f0f0U) >> 4) | (t0v5 & 0xf0f0f0f0U));\
    ((D)[0]) = (t0v8 & 0xffU);\
    ((D)[8]) = ((t0v8 >> 8) & 0xffU);\
    ((D)[16]) = ((t0v8 >> 16) & 0xffU);\
    ((D)[24]) = (t0v8 >> 24);\
    ((D)[1]) = (t0v6 & 0xffU);\
    ((D)[9]) = ((t0v6 >> 8) & 0xffU);\
    ((D)[17]) = ((t0v6 >> 16) & 0xffU);\
    ((D)[25]) = (t0v6 >> 24);\
    ((D)[2]) = (t0v7 & 0xffU);\
    ((D)[10]) = ((t0v7 >> 8) & 0xffU);\
    ((D)[18]) = ((t0v7 >> 16) & 0xffU);\
    ((D)[26]) = (t0v7 >> 24);\
    ((D)[3]) = (t0v4 & 0xffU);\
    ((D)[11]) = ((t0v4 >> 8) & 0xffU);\
    ((D)[19]) = ((t0v4 >> 16) & 0xffU);\
    ((D)[27]) = (t0v4 >> 24);\
    ((D)[4]) = (t0v3 & 0xffU);\
    ((D)[12]) = ((t0v3 >> 8) & 0xffU);\
    ((D)[20]) = ((t0v3 >> 16) & 0xffU);\
    ((D)[28]) = (t0v3 >> 24);\
    ((D)[5]) = (t0v2 & 0xffU);\
    ((D)[13]) = ((t0v2 >> 8) & 0xffU);\
    ((D)[21]) = ((t0v2 >> 16) & 0xffU);\
    ((D)[29]) = (t0v2 >> 24);\
    ((D)[6]) = (t0v0 & 0xffU);\
    ((D)[14]) = ((t0v0 >> 8) & 0xffU);\
    ((D)[22]) = ((t0v0 >> 16) & 0xffU);\
    ((D)[30]) = (t0v0 >> 24);\
    ((D)[7]) = (t0v1 & 0xffU);\
    ((D)[15]) = ((t0v1 >> 8) & 0xffU);\
    ((D)[23]) = ((t0v1 >> 16) & 0xffU);\
    ((D)[31]) = (t0v1 >> 24);\
}
#define OUTPUT_TRANSFORM_B1(D, S0) \
{\
    ((D)[0]) = ((S0) & 0x1U);\
    ((D)[1]) = (((S0) >> 1) & 0x1U);\
    ((D)[2]) = (((S0) >> 2) & 0x1U);\
    ((D)[3]) = (((S0) >> 3) & 0x1U);\
    ((D)[4]) = (((S0) >> 4) & 0x1U);\
    ((D)[5]) = (((S0) >> 5) & 0x1U);\
    ((D)[6]) = (((S0) >> 6) & 0x1U);\
    ((D)[7]) = (((S0) >> 7) & 0x1U);\
    ((D)[8]) = (((S0) >> 8) & 0x1U);\
    ((D)[9]) = (((S0) >> 9) & 0x1U);\
    ((D)[10]) = (((S0) >> 10) & 0x1U);\
    ((D)[11]) = (((S0) >> 11) & 0x1U);\
    ((D)[12]) = (((S0) >> 12) & 0x1U);\
    ((D)[13]) = (((S0) >> 13) & 0x1U);\
    ((D)[14]) = (((S0) >> 14) & 0x1U);\
    ((D)[15]) = (((S0) >> 15) & 0x1U);\
    ((D)[16]) = (((S0) >> 16) & 0x1U);\
    ((D)[17]) = (((S0) >> 17) & 0x1U);\
    ((D)[18]) = (((S0) >> 18) & 0x1U);\
    ((D)[19]) = (((S0) >> 19) & 0x1U);\
    ((D)[20]) = (((S0) >> 20) & 0x1U);\
    ((D)[21]) = (((S0) >> 21) & 0x1U);\
    ((D)[22]) = (((S0) >> 22) & 0x1U);\
    ((D)[23]) = (((S0) >> 23) & 0x1U);\
    ((D)[24]) = (((S0) >> 24) & 0x1U);\
    ((D)[25]) = (((S0) >> 25) & 0x1U);\
    ((D)[26]) = (((S0) >> 26) & 0x1U);\
    ((D)[27]) = (((S0) >> 27) & 0x1U);\
    ((D)[28]) = (((S0) >> 28) & 0x1U);\
    ((D)[29]) = (((S0) >> 29) & 0x1U);\
    ((D)[30]) = (((S0) >> 30) & 0x1U);\
    ((D)[31]) = ((S0) >> 31);\
}
"##,
        transform.out()
    );
    // let mut transform = CLANG_TRANSFORM_INTEL_AVX2.transform();
    // for i in (1..=32).rev() {
    //     transform.gen_output_transform(i);
    // }
    // println!("{}", transform.out());
}
