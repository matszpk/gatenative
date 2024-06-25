use gatenative::clang_data_transform::*;

#[test]
fn test_clang_data_transform_input() {
    let mut dt = CLANG_DATA_TRANSFORM_U32.data_transform(32);
    dt.input_transform(
        "blable",
        64,
        10,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"void blable(unsigned long n, const unsigned int* input, uint32_t* output) {
    const uint32_t zero = 0;
    uint32_t unused;
    size_t k;
    size_t idx;
    for (idx = 0; idx < n; idx++) {
    unsigned int temp[32];
    const unsigned int* inelem = input + 64*idx
    size_t tpidx = 0;
    uint32_t* outelem = output + 10*idx;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    outelem[7] = v0;
    outelem[2] = v1;
    outelem[9] = v2;
    outelem[5] = v3;
    outelem[8] = v4;
    outelem[1] = v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    outelem[4] = v0;
    outelem[6] = v1;
    outelem[0] = v2;
    outelem[3] = v3;
    }
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );

    let mut dt = CLANG_DATA_TRANSFORM_INTEL_AVX.data_transform(256);
    dt.input_transform(
        "blable",
        64,
        10,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"void blable(unsigned long n, const unsigned int* input, __m256* output) {
    const __m256 zero = *((const __m256*)zero_value);
    __m256 unused;
    size_t k;
    size_t idx;
    for (idx = 0; idx < n; idx++) {
    unsigned int temp[256];
    const unsigned int* inelem = input + 512*idx
    size_t tpidx = 0;
    __m256* outelem = output + 10*idx;
    __m256 v0;
    __m256 v1;
    __m256 v2;
    __m256 v3;
    __m256 v4;
    __m256 v5;
    for (k = 0; k < 256; k++)
        temp[k] = inelem[512*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    _mm256_storeu_ps((float*)&outelem[7], v0);
    _mm256_storeu_ps((float*)&outelem[2], v1);
    _mm256_storeu_ps((float*)&outelem[9], v2);
    _mm256_storeu_ps((float*)&outelem[5], v3);
    _mm256_storeu_ps((float*)&outelem[8], v4);
    _mm256_storeu_ps((float*)&outelem[1], v5);
    for (k = 0; k < 256; k++)
        temp[k] = inelem[512*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    _mm256_storeu_ps((float*)&outelem[4], v0);
    _mm256_storeu_ps((float*)&outelem[6], v1);
    _mm256_storeu_ps((float*)&outelem[0], v2);
    _mm256_storeu_ps((float*)&outelem[3], v3);
    }
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
}
