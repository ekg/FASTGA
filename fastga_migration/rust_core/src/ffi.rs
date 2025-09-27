use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_double};
use std::slice;

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Safe wrappers around C functions

pub unsafe fn encode_2bit_wrapper(sequence: &str) -> Vec<u8> {
    let c_string = CString::new(sequence).expect("CString::new failed");
    let mut encoded_len: usize = 0;

    let encoded_ptr = fastga_encode_2bit(
        c_string.as_ptr(),
        sequence.len(),
        &mut encoded_len as *mut usize
    );

    let result = slice::from_raw_parts(encoded_ptr, encoded_len).to_vec();
    libc::free(encoded_ptr as *mut libc::c_void);
    result
}

pub unsafe fn decode_2bit_wrapper(encoded: &[u8], seq_len: usize) -> String {
    let decoded_ptr = fastga_decode_2bit(
        encoded.as_ptr(),
        encoded.len(),
        seq_len
    );

    let c_str = CStr::from_ptr(decoded_ptr);
    let result = c_str.to_string_lossy().to_string();
    libc::free(decoded_ptr as *mut libc::c_void);
    result
}

pub unsafe fn encode_kmer_wrapper(seq: &[u8], k: usize) -> u64 {
    fastga_encode_kmer(seq.as_ptr(), k as c_int)
}

pub unsafe fn decode_kmer_wrapper(kmer: u64, k: usize) -> Vec<u8> {
    let mut out = vec![0u8; k];
    fastga_decode_kmer(kmer, k as c_int, out.as_mut_ptr());
    out
}

pub unsafe fn kmer_reverse_complement_wrapper(kmer: u64, k: usize) -> u64 {
    fastga_kmer_reverse_complement(kmer, k as c_int)
}

pub unsafe fn hash_kmer_wrapper(kmer: u64) -> u64 {
    fastga_hash_kmer(kmer)
}

pub unsafe fn score_match_wrapper(a: u8, b: u8) -> i32 {
    fastga_score_match(a, b) as i32
}

pub unsafe fn edit_distance_wrapper(seq1: &[u8], seq2: &[u8]) -> usize {
    fastga_edit_distance(
        seq1.as_ptr(),
        seq1.len(),
        seq2.as_ptr(),
        seq2.len()
    ) as usize
}

pub unsafe fn gc_content_wrapper(seq: &[u8]) -> f64 {
    fastga_gc_content(seq.as_ptr(), seq.len())
}

pub unsafe fn count_bases_wrapper(seq: &[u8]) -> (u64, u64, u64, u64) {
    let mut count_a: u64 = 0;
    let mut count_c: u64 = 0;
    let mut count_g: u64 = 0;
    let mut count_t: u64 = 0;

    fastga_count_bases(
        seq.as_ptr(),
        seq.len(),
        &mut count_a as *mut u64,
        &mut count_c as *mut u64,
        &mut count_g as *mut u64,
        &mut count_t as *mut u64
    );

    (count_a, count_c, count_g, count_t)
}

pub unsafe fn reverse_complement_seq_wrapper(seq: &mut [u8]) {
    fastga_reverse_complement_seq(seq.as_mut_ptr(), seq.len());
}