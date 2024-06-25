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
    const unsigned int* inelem = input + 64*idx;
    const size_t tpidx = 0;
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
    let mut dt = CLANG_DATA_TRANSFORM_U32.data_transform(160);
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
    const unsigned int* inelem = input + 320*idx;
    const size_t tpidx = idx % 5;
    uint32_t* outelem = output + 10*(idx - tpidx) + tpidx;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    outelem[35] = v0;
    outelem[10] = v1;
    outelem[45] = v2;
    outelem[25] = v3;
    outelem[40] = v4;
    outelem[5] = v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    outelem[20] = v0;
    outelem[30] = v1;
    outelem[0] = v2;
    outelem[15] = v3;
    }
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
    let mut dt = CLANG_DATA_TRANSFORM_U32.data_transform(256);
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
    const unsigned int* inelem = input + 512*idx;
    const size_t tpidx = idx & 7;
    uint32_t* outelem = output + 10*(idx & ~(size_t)7) + tpidx;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    outelem[56] = v0;
    outelem[16] = v1;
    outelem[72] = v2;
    outelem[40] = v3;
    outelem[64] = v4;
    outelem[8] = v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    outelem[32] = v0;
    outelem[48] = v1;
    outelem[0] = v2;
    outelem[24] = v3;
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
    const unsigned int* inelem = input + 512*idx;
    const size_t tpidx = 0;
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
    let mut dt = CLANG_DATA_TRANSFORM_INTEL_AVX.data_transform(640);
    dt.input_transform(
        "blable",
        64,
        10,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"void blable(unsigned long n, const unsigned int* input, __m128i* output) {
    const __m128i zero = *((const __m128i*)zero_value);
    __m128i unused;
    size_t k;
    size_t idx;
    for (idx = 0; idx < n; idx++) {
    unsigned int temp[128];
    const unsigned int* inelem = input + 1280*idx;
    const size_t tpidx = idx % 5;
    __m128i* outelem = output + 10*(idx - tpidx) + tpidx;
    __m128i v0;
    __m128i v1;
    __m128i v2;
    __m128i v3;
    __m128i v4;
    __m128i v5;
    for (k = 0; k < 128; k++)
        temp[k] = inelem[256*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    _mm_storeu_si128((__m128i*)&outelem[35], v0);
    _mm_storeu_si128((__m128i*)&outelem[10], v1);
    _mm_storeu_si128((__m128i*)&outelem[45], v2);
    _mm_storeu_si128((__m128i*)&outelem[25], v3);
    _mm_storeu_si128((__m128i*)&outelem[40], v4);
    _mm_storeu_si128((__m128i*)&outelem[5], v5);
    for (k = 0; k < 128; k++)
        temp[k] = inelem[256*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    _mm_storeu_si128((__m128i*)&outelem[20], v0);
    _mm_storeu_si128((__m128i*)&outelem[30], v1);
    _mm_storeu_si128((__m128i*)&outelem[0], v2);
    _mm_storeu_si128((__m128i*)&outelem[15], v3);
    }
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
    // OpenCL
    let mut dt = CLANG_DATA_TRANSFORM_OPENCL_U32.data_transform(32);
    dt.input_transform(
        "blable",
        64,
        10,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"kernel void blable(unsigned long n, const global unsigned int* input, global uint* output) {
    const uint zero = 0;
    uint unused;
    size_t k;
    const size_t idx = get_local_id(0);
    if (idx >= n) return;
    unsigned int temp[32];
    const global unsigned int* inelem = input + 64*idx;
    const size_t tpidx = 0;
    global uint* outelem = output + 10*idx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
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
"##,
        String::from_utf8(dt.out()).unwrap()
    );
    let mut dt = CLANG_DATA_TRANSFORM_OPENCL_U32.data_transform(192);
    dt.input_transform(
        "blable",
        64,
        10,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"kernel void blable(unsigned long n, const global unsigned int* input, global uint* output) {
    const uint zero = 0;
    uint unused;
    size_t k;
    const size_t idx = get_local_id(0);
    if (idx >= n) return;
    unsigned int temp[32];
    const global unsigned int* inelem = input + 384*idx;
    const size_t tpidx = idx % 6;
    global uint* outelem = output + 10*(idx - tpidx) + tpidx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 0];
    INPUT_TRANSFORM_B7(v0,v1,v2,v3,unused,v4,v5, temp);
    outelem[42] = v0;
    outelem[12] = v1;
    outelem[54] = v2;
    outelem[30] = v3;
    outelem[48] = v4;
    outelem[6] = v5;
    for (k = 0; k < 32; k++)
        temp[k] = inelem[64*tpidx + 2*k + 1];
    INPUT_TRANSFORM_B6(v0,v1,v2,unused,unused,v3, temp);
    outelem[24] = v0;
    outelem[36] = v1;
    outelem[0] = v2;
    outelem[18] = v3;
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
}

#[test]
fn test_clang_data_transform_output() {
    let mut dt = CLANG_DATA_TRANSFORM_U32.data_transform(32);
    dt.output_transform(
        "blable",
        10,
        64,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"void blable(unsigned long n, const uint32_t* input, unsigned int* output) {
    const uint32_t zero = 0;
    uint32_t unused;
    size_t k;
    size_t idx;
    for (idx = 0; idx < n; idx++) {
    unsigned int temp[32];
    const size_t tpidx = 0;
    const uint32_t* inelem = input + 10*idx;
    unsigned int* outelem = output + 64*idx;
    uint32_t v0;
    uint32_t v1;
    uint32_t v2;
    uint32_t v3;
    uint32_t v4;
    uint32_t v5;
    v0 = inelem[7];
    v1 = inelem[2];
    v2 = inelem[9];
    v3 = inelem[5];
    v4 = inelem[8];
    v5 = inelem[1];
    OUTPUT_TRANSFORM_B7(temp, v0,v1,v2,v3,zero,v4,v5);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 0] = temp[k];
    v0 = inelem[4];
    v1 = inelem[6];
    v2 = inelem[0];
    v3 = inelem[3];
    OUTPUT_TRANSFORM_B6(temp, v0,v1,v2,zero,zero,v3);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 1] = temp[k];
    }
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
    // OpenCL
    let mut dt = CLANG_DATA_TRANSFORM_OPENCL_U32.data_transform(32);
    dt.output_transform(
        "blable",
        10,
        64,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"kernel void blable(unsigned long n, const global uint* input, global unsigned int* output) {
    const uint zero = 0;
    uint unused;
    size_t k;
    const size_t idx = get_local_id(0);
    if (idx >= n) return;
    unsigned int temp[32];
    const size_t tpidx = 0;
    const global uint* inelem = input + 10*idx;
    global unsigned int* outelem = output + 64*idx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    v0 = inelem[7];
    v1 = inelem[2];
    v2 = inelem[9];
    v3 = inelem[5];
    v4 = inelem[8];
    v5 = inelem[1];
    OUTPUT_TRANSFORM_B7(temp, v0,v1,v2,v3,zero,v4,v5);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 0] = temp[k];
    v0 = inelem[4];
    v1 = inelem[6];
    v2 = inelem[0];
    v3 = inelem[3];
    OUTPUT_TRANSFORM_B6(temp, v0,v1,v2,zero,zero,v3);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 1] = temp[k];
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
    let mut dt = CLANG_DATA_TRANSFORM_OPENCL_U32.data_transform(192);
    dt.output_transform(
        "blable",
        10,
        64,
        &[32 + 2, 6, 1, 32 + 5, 32, 3, 32 + 1, 0, 5, 2],
    );
    assert_eq!(
        r##"kernel void blable(unsigned long n, const global uint* input, global unsigned int* output) {
    const uint zero = 0;
    uint unused;
    size_t k;
    const size_t idx = get_local_id(0);
    if (idx >= n) return;
    unsigned int temp[32];
    const size_t tpidx = idx % 6;
    const global uint* inelem = input + 10*(idx - tpidx) + tpidx;
    global unsigned int* outelem = output + 384*idx;
    uint v0;
    uint v1;
    uint v2;
    uint v3;
    uint v4;
    uint v5;
    v0 = inelem[42];
    v1 = inelem[12];
    v2 = inelem[54];
    v3 = inelem[30];
    v4 = inelem[48];
    v5 = inelem[6];
    OUTPUT_TRANSFORM_B7(temp, v0,v1,v2,v3,zero,v4,v5);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 0] = temp[k];
    v0 = inelem[24];
    v1 = inelem[36];
    v2 = inelem[0];
    v3 = inelem[18];
    OUTPUT_TRANSFORM_B6(temp, v0,v1,v2,zero,zero,v3);
    for (k = 0; k < 32; k++)
        outelem[64*tpidx + 2*k + 1] = temp[k];
}
"##,
        String::from_utf8(dt.out()).unwrap()
    );
}
