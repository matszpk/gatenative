use crate::gencode::generate_code_with_config;
use gatenative::clang_writer::*;
use gatenative::*;
use gatesim::*;

#[test]
fn test_clang_writer_elem_input() {
    let circuit = Circuit::new(
        30,
        (0..15).map(|i| Gate::new_xor(i, 15 + i)),
        (0..15).map(|i| (30 + i, false)),
    )
    .unwrap();

    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint32_t v0;
    uint32_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[9];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[10];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[12];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[13];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[15];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_U64.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint64_t* input,
    uint64_t* output, size_t idx) {
    const uint64_t zero = 0ULL;
    const uint64_t one = 0xffffffffffffffffULL;
    const uint64_t elem_low_bit0 = 0xaaaaaaaaaaaaaaaaULL;
    const uint64_t elem_low_bit1 = 0xccccccccccccccccULL;
    const uint64_t elem_low_bit2 = 0xf0f0f0f0f0f0f0f0ULL;
    const uint64_t elem_low_bit3 = 0xff00ff00ff00ff00ULL;
    const uint64_t elem_low_bit4 = 0xffff0000ffff0000ULL;
    const uint64_t elem_low_bit5 = 0xffffffff00000000ULL;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint64_t v0;
    uint64_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[9];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[10];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[12];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[13];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[15];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_MMX.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m64* input,
    __m64* output, size_t idx) {
    const __m64 zero = *((const __m64*)zero_value);
    const __m64 one = *((const __m64*)one_value);
    const __m64 elem_low_bit0 = *((const __m64*)elem_index_low_tbl);
    const __m64 elem_low_bit1 = *((const __m64*)(elem_index_low_tbl + 2));
    const __m64 elem_low_bit2 = *((const __m64*)(elem_index_low_tbl + 4));
    const __m64 elem_low_bit3 = *((const __m64*)(elem_index_low_tbl + 6));
    const __m64 elem_low_bit4 = *((const __m64*)(elem_index_low_tbl + 8));
    const __m64 elem_low_bit5 = *((const __m64*)(elem_index_low_tbl + 10));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m64 v0;
    __m64 v1;
    v0 = input[0];
    v1 = input[2];
    v0 = _m_pxor(v0, v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = _m_pxor(v0, v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = _m_pxor(v0, v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = _m_pxor(v0, v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = _m_pxor(v0, v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = _m_pxor(v0, v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = _m_pxor(v0, v1);
    output[6] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[9];
    v0 = _m_pxor(v0, v1);
    output[7] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[10];
    v0 = _m_pxor(v0, v1);
    output[8] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[11];
    v0 = _m_pxor(v0, v1);
    output[9] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[12];
    v0 = _m_pxor(v0, v1);
    output[10] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[13];
    v0 = _m_pxor(v0, v1);
    output[11] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[14];
    v0 = _m_pxor(v0, v1);
    output[12] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[15];
    v0 = _m_pxor(v0, v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = _m_pxor(v0, v1);
    output[14] = v0;
    _m_empty();
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_SSE.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m128* input,
    __m128* output, size_t idx) {
    const __m128 zero = *((const __m128*)zero_value);
    const __m128 one = *((const __m128*)one_value);
    const __m128 elem_low_bit0 = *((const __m128*)elem_index_low_tbl);
    const __m128 elem_low_bit1 = *((const __m128*)(elem_index_low_tbl + 4));
    const __m128 elem_low_bit2 = *((const __m128*)(elem_index_low_tbl + 8));
    const __m128 elem_low_bit3 = *((const __m128*)(elem_index_low_tbl + 12));
    const __m128 elem_low_bit4 = *((const __m128*)(elem_index_low_tbl + 16));
    const __m128 elem_low_bit5 = *((const __m128*)(elem_index_low_tbl + 20));
    const __m128 elem_low_bit6 = *((const __m128*)(elem_index_low_tbl + 24));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m128 v0;
    __m128 v1;
    v0 = _mm_loadu_ps((const float*)&input[0]);
    v1 = _mm_loadu_ps((const float*)&input[2]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[0], v0);
    v0 = elem_low_bit0;
    v1 = _mm_loadu_ps((const float*)&input[3]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[1], v0);
    v0 = elem_low_bit1;
    v1 = _mm_loadu_ps((const float*)&input[4]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[2], v0);
    v0 = elem_low_bit2;
    v1 = _mm_loadu_ps((const float*)&input[5]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[3], v0);
    v0 = elem_low_bit3;
    v1 = _mm_loadu_ps((const float*)&input[6]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[4], v0);
    v0 = elem_low_bit4;
    v1 = _mm_loadu_ps((const float*)&input[7]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[5], v0);
    v0 = elem_low_bit5;
    v1 = _mm_loadu_ps((const float*)&input[8]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[6], v0);
    v0 = elem_low_bit6;
    v1 = _mm_loadu_ps((const float*)&input[9]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[7], v0);
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[10]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[8], v0);
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[11]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[9], v0);
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[12]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[10], v0);
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[13]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[11], v0);
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[14]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[12], v0);
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = _mm_loadu_ps((const float*)&input[15]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[13], v0);
    v0 = _mm_loadu_ps((const float*)&input[1]);
    v1 = _mm_loadu_ps((const float*)&input[16]);
    v0 = _mm_xor_ps(v0, v1);
    _mm_storeu_ps((float*)&output[14], v0);
}
"##
    );

    let mut writer = CLANG_WRITER_INTEL_AVX.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m256* input,
    __m256* output, size_t idx) {
    const __m256 zero = *((const __m256*)zero_value);
    const __m256 one = *((const __m256*)one_value);
    const __m256 elem_low_bit0 = *((const __m256*)elem_index_low_tbl);
    const __m256 elem_low_bit1 = *((const __m256*)(elem_index_low_tbl + 8));
    const __m256 elem_low_bit2 = *((const __m256*)(elem_index_low_tbl + 16));
    const __m256 elem_low_bit3 = *((const __m256*)(elem_index_low_tbl + 24));
    const __m256 elem_low_bit4 = *((const __m256*)(elem_index_low_tbl + 32));
    const __m256 elem_low_bit5 = *((const __m256*)(elem_index_low_tbl + 40));
    const __m256 elem_low_bit6 = *((const __m256*)(elem_index_low_tbl + 48));
    const __m256 elem_low_bit7 = *((const __m256*)(elem_index_low_tbl + 56));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m256 v0;
    __m256 v1;
    v0 = _mm256_loadu_ps((const float*)&input[0]);
    v1 = _mm256_loadu_ps((const float*)&input[2]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[0], v0);
    v0 = elem_low_bit0;
    v1 = _mm256_loadu_ps((const float*)&input[3]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[1], v0);
    v0 = elem_low_bit1;
    v1 = _mm256_loadu_ps((const float*)&input[4]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[2], v0);
    v0 = elem_low_bit2;
    v1 = _mm256_loadu_ps((const float*)&input[5]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[3], v0);
    v0 = elem_low_bit3;
    v1 = _mm256_loadu_ps((const float*)&input[6]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[4], v0);
    v0 = elem_low_bit4;
    v1 = _mm256_loadu_ps((const float*)&input[7]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[5], v0);
    v0 = elem_low_bit5;
    v1 = _mm256_loadu_ps((const float*)&input[8]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[6], v0);
    v0 = elem_low_bit6;
    v1 = _mm256_loadu_ps((const float*)&input[9]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[7], v0);
    v0 = elem_low_bit7;
    v1 = _mm256_loadu_ps((const float*)&input[10]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[8], v0);
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[11]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[9], v0);
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[12]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[10], v0);
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[13]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[11], v0);
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[14]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[12], v0);
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = _mm256_loadu_ps((const float*)&input[15]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[13], v0);
    v0 = _mm256_loadu_ps((const float*)&input[1]);
    v1 = _mm256_loadu_ps((const float*)&input[16]);
    v0 = _mm256_xor_ps(v0, v1);
    _mm256_storeu_ps((float*)&output[14], v0);
}
"##
    );
    let mut writer = CLANG_WRITER_INTEL_AVX512.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const __m512i* input,
    __m512i* output, size_t idx) {
    const __m512i zero = *((const __m512i*)zero_value);
    const __m512i one = *((const __m512i*)one_value);
    const __m512i elem_low_bit0 = *((const __m512i*)elem_index_low_tbl);
    const __m512i elem_low_bit1 = *((const __m512i*)(elem_index_low_tbl + 16));
    const __m512i elem_low_bit2 = *((const __m512i*)(elem_index_low_tbl + 32));
    const __m512i elem_low_bit3 = *((const __m512i*)(elem_index_low_tbl + 48));
    const __m512i elem_low_bit4 = *((const __m512i*)(elem_index_low_tbl + 64));
    const __m512i elem_low_bit5 = *((const __m512i*)(elem_index_low_tbl + 80));
    const __m512i elem_low_bit6 = *((const __m512i*)(elem_index_low_tbl + 96));
    const __m512i elem_low_bit7 = *((const __m512i*)(elem_index_low_tbl + 112));
    const __m512i elem_low_bit8 = *((const __m512i*)(elem_index_low_tbl + 128));
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    __m512i v0;
    __m512i v1;
    v0 = _mm512_loadu_epi64(&input[0]);
    v1 = _mm512_loadu_epi64(&input[2]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[0], v0);
    v0 = elem_low_bit0;
    v1 = _mm512_loadu_epi64(&input[3]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[1], v0);
    v0 = elem_low_bit1;
    v1 = _mm512_loadu_epi64(&input[4]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[2], v0);
    v0 = elem_low_bit2;
    v1 = _mm512_loadu_epi64(&input[5]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[3], v0);
    v0 = elem_low_bit3;
    v1 = _mm512_loadu_epi64(&input[6]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[4], v0);
    v0 = elem_low_bit4;
    v1 = _mm512_loadu_epi64(&input[7]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[5], v0);
    v0 = elem_low_bit5;
    v1 = _mm512_loadu_epi64(&input[8]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[6], v0);
    v0 = elem_low_bit6;
    v1 = _mm512_loadu_epi64(&input[9]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[7], v0);
    v0 = elem_low_bit7;
    v1 = _mm512_loadu_epi64(&input[10]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[8], v0);
    v0 = elem_low_bit8;
    v1 = _mm512_loadu_epi64(&input[11]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[9], v0);
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[12]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[10], v0);
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[13]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[11], v0);
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[14]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[12], v0);
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = _mm512_loadu_epi64(&input[15]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[13], v0);
    v0 = _mm512_loadu_epi64(&input[1]);
    v1 = _mm512_loadu_epi64(&input[16]);
    v0 = _mm512_xor_epi64(v0, v1);
    _mm512_storeu_epi64(&output[14], v0);
}
"##
    );

    let mut writer = CLANG_WRITER_ARM_NEON.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32x4_t* input,
    uint32x4_t* output, size_t idx) {
    const uint32x4_t zero = { 0, 0, 0, 0 };
    const uint32x4_t one = { 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff };
    const uint32x4_t elem_low_bit0 = { 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa, 0xaaaaaaaa };
    const uint32x4_t elem_low_bit1 = { 0xcccccccc, 0xcccccccc, 0xcccccccc, 0xcccccccc };
    const uint32x4_t elem_low_bit2 = { 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0, 0xf0f0f0f0 };
    const uint32x4_t elem_low_bit3 = { 0xff00ff00, 0xff00ff00, 0xff00ff00, 0xff00ff00 };
    const uint32x4_t elem_low_bit4 = { 0xffff0000, 0xffff0000, 0xffff0000, 0xffff0000 };
    const uint32x4_t elem_low_bit5 = { 0x00000000, 0xffffffff, 0x00000000, 0xffffffff };
    const uint32x4_t elem_low_bit6 = { 0x00000000, 0x00000000, 0xffffffff, 0xffffffff };
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint32x4_t v0;
    uint32x4_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = veorq_u32(v0, v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = veorq_u32(v0, v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[4];
    v0 = veorq_u32(v0, v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[5];
    v0 = veorq_u32(v0, v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[6];
    v0 = veorq_u32(v0, v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[7];
    v0 = veorq_u32(v0, v1);
    output[5] = v0;
    v0 = elem_low_bit5;
    v1 = input[8];
    v0 = veorq_u32(v0, v1);
    output[6] = v0;
    v0 = elem_low_bit6;
    v1 = input[9];
    v0 = veorq_u32(v0, v1);
    output[7] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[10];
    v0 = veorq_u32(v0, v1);
    output[8] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[11];
    v0 = veorq_u32(v0, v1);
    output[9] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[12];
    v0 = veorq_u32(v0, v1);
    output[10] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[13];
    v0 = veorq_u32(v0, v1);
    output[11] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[14];
    v0 = veorq_u32(v0, v1);
    output[12] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[15];
    v0 = veorq_u32(v0, v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[16];
    v0 = veorq_u32(v0, v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 17 * idx + input_shift;
    const size_t ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = input[ivn + 2];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 3];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = input[ivn + 4];
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = input[ivn + 5];
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = input[ivn + 6];
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = input[ivn + 7];
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[ivn + 8];
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[ivn + 9];
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[ivn + 10];
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[ivn + 11];
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[ivn + 12];
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[ivn + 13];
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[ivn + 14];
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[ivn + 15];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 1];
    v1 = input[ivn + 16];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new().elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (17 * idx) + input_shift;
    const size_t ovn = llen * (15 * idx) + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*2 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*0 + lidx] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + llen*3 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*1 + lidx] = v0;
    v0 = elem_low_bit1;
    v1 = input[ivn + llen*4 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*2 + lidx] = v0;
    v0 = elem_low_bit2;
    v1 = input[ivn + llen*5 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*3 + lidx] = v0;
    v0 = elem_low_bit3;
    v1 = input[ivn + llen*6 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*4 + lidx] = v0;
    v0 = elem_low_bit4;
    v1 = input[ivn + llen*7 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*5 + lidx] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[ivn + llen*8 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*6 + lidx] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[ivn + llen*9 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*7 + lidx] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[ivn + llen*10 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*8 + lidx] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[ivn + llen*11 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*9 + lidx] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[ivn + llen*12 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*10 + lidx] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[ivn + llen*13 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*11 + lidx] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[ivn + llen*14 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*12 + lidx] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[ivn + llen*15 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*13 + lidx] = v0;
    v0 = input[ivn + llen*1 + lidx];
    v1 = input[ivn + llen*16 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*14 + lidx] = v0;
}
"##
    );

    // with input_placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((
                &(0..30 - 13).map(|i| 2 + 3 * i).collect::<Vec<_>>(),
                100,
            )))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint32_t v0;
    uint32_t v1;
    v0 = input[2];
    v1 = input[8];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[11];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = input[14];
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = input[17];
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = input[20];
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = input[23];
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = input[26];
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = input[29];
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = input[32];
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = input[35];
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = input[38];
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = input[41];
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = input[44];
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[47];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[5];
    v1 = input[50];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    // with arg_input
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&(17..28).collect::<Vec<_>>()))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint32_t v0;
    uint32_t v1;
    v0 = input[0];
    v1 = input[2];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[3];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[4];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[1];
    v1 = input[5];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&(17..28).collect::<Vec<_>>()))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 6 * idx + input_shift;
    const size_t ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 0];
    v1 = input[ivn + 2];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 3];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[ivn + 4];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 1];
    v1 = input[ivn + 5];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32_GROUP_VEC.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .arg_inputs(Some(&(17..28).collect::<Vec<_>>()))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_group_id(0);
    const uint lidx = get_local_id(0);
    const uint llen = get_local_size(0);
    const size_t ivn = llen * (6 * idx) + input_shift;
    const size_t ovn = llen * (15 * idx) + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + llen*0 + lidx];
    v1 = input[ivn + llen*2 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*0 + lidx] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + llen*3 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*1 + lidx] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*2 + lidx] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*3 + lidx] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*4 + lidx] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*5 + lidx] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*6 + lidx] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*7 + lidx] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*8 + lidx] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*9 + lidx] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*10 + lidx] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*11 + lidx] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + llen*12 + lidx] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[ivn + llen*4 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*13 + lidx] = v0;
    v0 = input[ivn + llen*1 + lidx];
    v1 = input[ivn + llen*5 + lidx];
    v0 = (v0 ^ v1);
    output[ovn + llen*14 + lidx] = v0;
}
"##
    );

    // with arg_input and input_placement
    let mut writer = CLANG_WRITER_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((
                &(0..30 - 13 - 11).map(|i| 9 + 11 * i).collect::<Vec<_>>(),
                100,
            )))
            .arg_inputs(Some(&(17..28).collect::<Vec<_>>()))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"void gate_sys_xor(const uint32_t* input,
    uint32_t* output, unsigned int arg, unsigned int arg2, size_t idx) {
    const uint32_t zero = 0;
    const uint32_t one = 0xffffffff;
    const uint32_t elem_low_bit0 = 0xaaaaaaaa;
    const uint32_t elem_low_bit1 = 0xcccccccc;
    const uint32_t elem_low_bit2 = 0xf0f0f0f0;
    const uint32_t elem_low_bit3 = 0xff00ff00;
    const uint32_t elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint32_t v0;
    uint32_t v1;
    v0 = input[9];
    v1 = input[31];
    v0 = (v0 ^ v1);
    output[0] = v0;
    v0 = elem_low_bit0;
    v1 = input[42];
    v0 = (v0 ^ v1);
    output[1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[53];
    v0 = (v0 ^ v1);
    output[13] = v0;
    v0 = input[20];
    v1 = input[64];
    v0 = (v0 ^ v1);
    output[14] = v0;
}
"##
    );

    let mut writer = CLANG_WRITER_OPENCL_U32.writer();
    generate_code_with_config(
        &mut writer,
        "xor",
        circuit.clone(),
        false,
        CodeConfig::new()
            .input_placement(Some((
                &(0..30 - 13 - 11).map(|i| 9 + 11 * i).collect::<Vec<_>>(),
                100,
            )))
            .arg_inputs(Some(&(17..28).collect::<Vec<_>>()))
            .elem_inputs(Some(&(1..14).collect::<Vec<_>>())),
    );
    assert_eq!(
        &String::from_utf8(writer.out()).unwrap(),
        r##"kernel void gate_sys_xor(unsigned long n, 
    unsigned long input_shift, unsigned long output_shift,
    const global uint* input,
    global uint* output, unsigned int arg, unsigned int arg2) {
    const size_t idx = get_global_id(0);
    const size_t ivn = 100 * idx + input_shift;
    const size_t ovn = 15 * idx + output_shift;
    const uint zero = 0;
    const uint one = 0xffffffff;
    const uint elem_low_bit0 = 0xaaaaaaaa;
    const uint elem_low_bit1 = 0xcccccccc;
    const uint elem_low_bit2 = 0xf0f0f0f0;
    const uint elem_low_bit3 = 0xff00ff00;
    const uint elem_low_bit4 = 0xffff0000;
    const unsigned int idxl = idx & 0xffffffff;
    const unsigned int idxh = idx >> 32;
    uint v0;
    uint v1;
    if (idx >= n) return;
    v0 = input[ivn + 9];
    v1 = input[ivn + 31];
    v0 = (v0 ^ v1);
    output[ovn + 0] = v0;
    v0 = elem_low_bit0;
    v1 = input[ivn + 42];
    v0 = (v0 ^ v1);
    output[ovn + 1] = v0;
    v0 = elem_low_bit1;
    v1 = ((arg & 1) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 2] = v0;
    v0 = elem_low_bit2;
    v1 = ((arg & 2) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 3] = v0;
    v0 = elem_low_bit3;
    v1 = ((arg & 4) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 4] = v0;
    v0 = elem_low_bit4;
    v1 = ((arg & 8) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 5] = v0;
    v0 = ((idxl & 1) != 0) ? one : zero;
    v1 = ((arg & 16) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 6] = v0;
    v0 = ((idxl & 2) != 0) ? one : zero;
    v1 = ((arg & 32) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 7] = v0;
    v0 = ((idxl & 4) != 0) ? one : zero;
    v1 = ((arg & 64) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 8] = v0;
    v0 = ((idxl & 8) != 0) ? one : zero;
    v1 = ((arg & 128) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 9] = v0;
    v0 = ((idxl & 16) != 0) ? one : zero;
    v1 = ((arg & 256) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 10] = v0;
    v0 = ((idxl & 32) != 0) ? one : zero;
    v1 = ((arg & 512) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 11] = v0;
    v0 = ((idxl & 64) != 0) ? one : zero;
    v1 = ((arg & 1024) != 0) ? one : zero;
    v0 = (v0 ^ v1);
    output[ovn + 12] = v0;
    v0 = ((idxl & 128) != 0) ? one : zero;
    v1 = input[ivn + 53];
    v0 = (v0 ^ v1);
    output[ovn + 13] = v0;
    v0 = input[ivn + 20];
    v1 = input[ivn + 64];
    v0 = (v0 ^ v1);
    output[ovn + 14] = v0;
}
"##
    );
}
