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
    (dest[0]) = ((t0v25 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    (dest[16]) = ((t0v25 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    (dest[1]) = ((t0v31 & 0x00000000ffffffffULL) | (t0v32 << 32));\
    (dest[17]) = ((t0v31 >> 32) | (t0v32 & 0xffffffff00000000ULL));\
    (dest[2]) = ((t0v30 & 0x00000000ffffffffULL) | (t0v28 << 32));\
    (dest[18]) = ((t0v30 >> 32) | (t0v28 & 0xffffffff00000000ULL));\
    (dest[3]) = ((t0v23 & 0x00000000ffffffffULL) | (t0v26 << 32));\
    (dest[19]) = ((t0v23 >> 32) | (t0v26 & 0xffffffff00000000ULL));\
    (dest[4]) = ((t0v24 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    (dest[20]) = ((t0v24 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    (dest[5]) = ((t0v19 & 0x00000000ffffffffULL) | (t0v22 << 32));\
    (dest[21]) = ((t0v19 >> 32) | (t0v22 & 0xffffffff00000000ULL));\
    (dest[6]) = ((t0v20 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    (dest[22]) = ((t0v20 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    (dest[7]) = ((t0v18 & 0x00000000ffffffffULL) | (t0v16 << 32));\
    (dest[23]) = ((t0v18 >> 32) | (t0v16 & 0xffffffff00000000ULL));\
    (dest[8]) = ((t0v9 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    (dest[24]) = ((t0v9 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    (dest[9]) = ((t0v27 & 0x00000000ffffffffULL) | (t0v14 << 32));\
    (dest[25]) = ((t0v27 >> 32) | (t0v14 & 0xffffffff00000000ULL));\
    (dest[10]) = ((t0v12 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    (dest[26]) = ((t0v12 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    (dest[11]) = ((t0v10 & 0x00000000ffffffffULL) | (t0v8 << 32));\
    (dest[27]) = ((t0v10 >> 32) | (t0v8 & 0xffffffff00000000ULL));\
    (dest[12]) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    (dest[28]) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    (dest[13]) = ((t0v6 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    (dest[29]) = ((t0v6 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    (dest[14]) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    (dest[30]) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    (dest[15]) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    (dest[31]) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
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
    (dest[0]) = ((t0v9 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    (dest[16]) = ((t0v9 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    (dest[1]) = ((t0v18 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    (dest[17]) = ((t0v18 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    (dest[2]) = ((t0v20 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    (dest[18]) = ((t0v20 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    (dest[3]) = ((t0v22 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    (dest[19]) = ((t0v22 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    (dest[4]) = ((t0v24 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    (dest[20]) = ((t0v24 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    (dest[5]) = ((t0v26 & 0x00000000ffffffffULL) | (t0v27 << 32));\
    (dest[21]) = ((t0v26 >> 32) | (t0v27 & 0xffffffff00000000ULL));\
    (dest[6]) = ((t0v28 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    (dest[22]) = ((t0v28 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    (dest[7]) = ((t0v30 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    (dest[23]) = ((t0v30 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
    (dest[8]) = ((t0v13 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    (dest[24]) = ((t0v13 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    (dest[9]) = ((t0v16 & 0x00000000ffffffffULL) | (t0v14 << 32));\
    (dest[25]) = ((t0v16 >> 32) | (t0v14 & 0xffffffff00000000ULL));\
    (dest[10]) = ((t0v12 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    (dest[26]) = ((t0v12 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    (dest[11]) = ((t0v10 & 0x00000000ffffffffULL) | (t0v8 << 32));\
    (dest[27]) = ((t0v10 >> 32) | (t0v8 & 0xffffffff00000000ULL));\
    (dest[12]) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    (dest[28]) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    (dest[13]) = ((t0v6 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    (dest[29]) = ((t0v6 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    (dest[14]) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    (dest[30]) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    (dest[15]) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    (dest[31]) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
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
    (dest[0]) = ((t0v5 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    (dest[16]) = ((t0v5 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    (dest[1]) = ((t0v14 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    (dest[17]) = ((t0v14 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    (dest[2]) = ((t0v20 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    (dest[18]) = ((t0v20 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    (dest[3]) = ((t0v26 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    (dest[19]) = ((t0v26 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    (dest[4]) = ((t0v9 & 0x00000000ffffffffULL) | (t0v12 << 32));\
    (dest[20]) = ((t0v9 >> 32) | (t0v12 & 0xffffffff00000000ULL));\
    (dest[5]) = ((t0v15 & 0x00000000ffffffffULL) | (t0v18 << 32));\
    (dest[21]) = ((t0v15 >> 32) | (t0v18 & 0xffffffff00000000ULL));\
    (dest[6]) = ((t0v21 & 0x00000000ffffffffULL) | (t0v24 << 32));\
    (dest[22]) = ((t0v21 >> 32) | (t0v24 & 0xffffffff00000000ULL));\
    (dest[7]) = ((t0v27 & 0x00000000ffffffffULL) | (t0v30 << 32));\
    (dest[23]) = ((t0v27 >> 32) | (t0v30 & 0xffffffff00000000ULL));\
    (dest[8]) = ((t0v10 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    (dest[24]) = ((t0v10 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    (dest[9]) = ((t0v16 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    (dest[25]) = ((t0v16 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    (dest[10]) = ((t0v22 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    (dest[26]) = ((t0v22 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    (dest[11]) = ((t0v28 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    (dest[27]) = ((t0v28 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
    (dest[12]) = ((t0v8 & 0x00000000ffffffffULL) | (t0v6 << 32));\
    (dest[28]) = ((t0v8 >> 32) | (t0v6 & 0xffffffff00000000ULL));\
    (dest[13]) = ((t0v7 & 0x00000000ffffffffULL) | (t0v4 << 32));\
    (dest[29]) = ((t0v7 >> 32) | (t0v4 & 0xffffffff00000000ULL));\
    (dest[14]) = ((t0v3 & 0x00000000ffffffffULL) | (t0v2 << 32));\
    (dest[30]) = ((t0v3 >> 32) | (t0v2 & 0xffffffff00000000ULL));\
    (dest[15]) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    (dest[31]) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
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
    (dest[0]) = ((t0v0 & 0x00000000ffffffffULL) | (t0v1 << 32));\
    (dest[16]) = ((t0v0 >> 32) | (t0v1 & 0xffffffff00000000ULL));\
    (dest[1]) = ((t0v2 & 0x00000000ffffffffULL) | (t0v3 << 32));\
    (dest[17]) = ((t0v2 >> 32) | (t0v3 & 0xffffffff00000000ULL));\
    (dest[2]) = ((t0v4 & 0x00000000ffffffffULL) | (t0v5 << 32));\
    (dest[18]) = ((t0v4 >> 32) | (t0v5 & 0xffffffff00000000ULL));\
    (dest[3]) = ((t0v6 & 0x00000000ffffffffULL) | (t0v7 << 32));\
    (dest[19]) = ((t0v6 >> 32) | (t0v7 & 0xffffffff00000000ULL));\
    (dest[4]) = ((t0v8 & 0x00000000ffffffffULL) | (t0v9 << 32));\
    (dest[20]) = ((t0v8 >> 32) | (t0v9 & 0xffffffff00000000ULL));\
    (dest[5]) = ((t0v10 & 0x00000000ffffffffULL) | (t0v11 << 32));\
    (dest[21]) = ((t0v10 >> 32) | (t0v11 & 0xffffffff00000000ULL));\
    (dest[6]) = ((t0v12 & 0x00000000ffffffffULL) | (t0v13 << 32));\
    (dest[22]) = ((t0v12 >> 32) | (t0v13 & 0xffffffff00000000ULL));\
    (dest[7]) = ((t0v14 & 0x00000000ffffffffULL) | (t0v15 << 32));\
    (dest[23]) = ((t0v14 >> 32) | (t0v15 & 0xffffffff00000000ULL));\
    (dest[8]) = ((t0v16 & 0x00000000ffffffffULL) | (t0v17 << 32));\
    (dest[24]) = ((t0v16 >> 32) | (t0v17 & 0xffffffff00000000ULL));\
    (dest[9]) = ((t0v18 & 0x00000000ffffffffULL) | (t0v19 << 32));\
    (dest[25]) = ((t0v18 >> 32) | (t0v19 & 0xffffffff00000000ULL));\
    (dest[10]) = ((t0v20 & 0x00000000ffffffffULL) | (t0v21 << 32));\
    (dest[26]) = ((t0v20 >> 32) | (t0v21 & 0xffffffff00000000ULL));\
    (dest[11]) = ((t0v22 & 0x00000000ffffffffULL) | (t0v23 << 32));\
    (dest[27]) = ((t0v22 >> 32) | (t0v23 & 0xffffffff00000000ULL));\
    (dest[12]) = ((t0v24 & 0x00000000ffffffffULL) | (t0v25 << 32));\
    (dest[28]) = ((t0v24 >> 32) | (t0v25 & 0xffffffff00000000ULL));\
    (dest[13]) = ((t0v26 & 0x00000000ffffffffULL) | (t0v27 << 32));\
    (dest[29]) = ((t0v26 >> 32) | (t0v27 & 0xffffffff00000000ULL));\
    (dest[14]) = ((t0v28 & 0x00000000ffffffffULL) | (t0v29 << 32));\
    (dest[30]) = ((t0v28 >> 32) | (t0v29 & 0xffffffff00000000ULL));\
    (dest[15]) = ((t0v30 & 0x00000000ffffffffULL) | (t0v31 << 32));\
    (dest[31]) = ((t0v30 >> 32) | (t0v31 & 0xffffffff00000000ULL));\
}
"##,
        transform.out());
    // let mut transform = CLANG_TRANSFORM_U32.transform();
    // for i in (1..=32).rev() {
    //     transform.gen_output_transform(i);
    // }
    // println!("{}", transform.out());
}
