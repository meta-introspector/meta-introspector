// 🔥 GENERATED LD_PRELOAD HOOKS
// Auto-generated from real library symbols

use redhook::{hook, real};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::os::raw::{c_char, c_int, c_void};
use libc::{size_t, FILE};

static BROTLIDECODERCREATEINSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIDECODERDECOMPRESSSTREAM_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIDECODERDESTROYINSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIDECODERISFINISHED_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIENCODERCOMPRESSSTREAM_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIENCODERCREATEINSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIENCODERDESTROYINSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BROTLIENCODERISFINISHED_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERR_ERROR_STRING_N_COUNT: AtomicUsize = AtomicUsize::new(0);
static ERR_GET_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_COPY_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_CTRL_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_NEW_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_RESET_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_CIPHER_CTX_SET_PADDING_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DECRYPTFINAL_EX_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DECRYPTINIT_EX_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DECRYPTUPDATE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DIGESTFINAL_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DIGESTINIT_EX_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_DIGESTUPDATE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_ENCRYPTFINAL_EX_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_ENCRYPTINIT_EX_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_ENCRYPTUPDATE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_CTX_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_CTX_NEW_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_FETCH_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_FINAL_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MAC_UPDATE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MD_CTX_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MD_CTX_NEW_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MD_CTX_SET_FLAGS_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MD_GET_SIZE_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_AES_256_CBC_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_AES_256_CTR_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_AES_256_ECB_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_AES_256_GCM_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_MD5_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_SHA1_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVP_SHA256_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_ADD_ROOTS_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_EXPAND_HP_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_GCOLLECT_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_GET_BYTES_SINCE_GC_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_GET_HEAP_USAGE_SAFE_COUNT: AtomicUsize = AtomicUsize::new(0);
static GC_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

hook! {
    unsafe fn malloc(size: size_t) -> *mut c_void => my_malloc {
        let count = MALLOC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        eprintln!("HOOK[{}]: malloc({}) called", count, size);
        real!(malloc)(size)
    }
}

hook! {
    unsafe fn printf(format: *const c_char) -> c_int => my_printf {
        let count = PRINTF_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        eprintln!("HOOK[{}]: printf called", count);
        real!(printf)(format)
    }
}


#[no_mangle]
pub extern "C" fn print_hook_usage() {
    eprintln!("📊 HOOK USAGE SUMMARY:");
    let BrotliDecoderCreateInstance = BROTLIDECODERCREATEINSTANCE_COUNT.load(Ordering::SeqCst);
    if BrotliDecoderCreateInstance > 0 { eprintln!("  BrotliDecoderCreateInstance: {} calls", BrotliDecoderCreateInstance); }
    let BrotliDecoderDecompressStream = BROTLIDECODERDECOMPRESSSTREAM_COUNT.load(Ordering::SeqCst);
    if BrotliDecoderDecompressStream > 0 { eprintln!("  BrotliDecoderDecompressStream: {} calls", BrotliDecoderDecompressStream); }
    let BrotliDecoderDestroyInstance = BROTLIDECODERDESTROYINSTANCE_COUNT.load(Ordering::SeqCst);
    if BrotliDecoderDestroyInstance > 0 { eprintln!("  BrotliDecoderDestroyInstance: {} calls", BrotliDecoderDestroyInstance); }
    let BrotliDecoderIsFinished = BROTLIDECODERISFINISHED_COUNT.load(Ordering::SeqCst);
    if BrotliDecoderIsFinished > 0 { eprintln!("  BrotliDecoderIsFinished: {} calls", BrotliDecoderIsFinished); }
    let BrotliEncoderCompressStream = BROTLIENCODERCOMPRESSSTREAM_COUNT.load(Ordering::SeqCst);
    if BrotliEncoderCompressStream > 0 { eprintln!("  BrotliEncoderCompressStream: {} calls", BrotliEncoderCompressStream); }
    let BrotliEncoderCreateInstance = BROTLIENCODERCREATEINSTANCE_COUNT.load(Ordering::SeqCst);
    if BrotliEncoderCreateInstance > 0 { eprintln!("  BrotliEncoderCreateInstance: {} calls", BrotliEncoderCreateInstance); }
    let BrotliEncoderDestroyInstance = BROTLIENCODERDESTROYINSTANCE_COUNT.load(Ordering::SeqCst);
    if BrotliEncoderDestroyInstance > 0 { eprintln!("  BrotliEncoderDestroyInstance: {} calls", BrotliEncoderDestroyInstance); }
    let BrotliEncoderIsFinished = BROTLIENCODERISFINISHED_COUNT.load(Ordering::SeqCst);
    if BrotliEncoderIsFinished > 0 { eprintln!("  BrotliEncoderIsFinished: {} calls", BrotliEncoderIsFinished); }
    let ERR_error_string_n = ERR_ERROR_STRING_N_COUNT.load(Ordering::SeqCst);
    if ERR_error_string_n > 0 { eprintln!("  ERR_error_string_n: {} calls", ERR_error_string_n); }
    let ERR_get_error = ERR_GET_ERROR_COUNT.load(Ordering::SeqCst);
    if ERR_get_error > 0 { eprintln!("  ERR_get_error: {} calls", ERR_get_error); }
    let EVP_CIPHER_CTX_copy = EVP_CIPHER_CTX_COPY_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_copy > 0 { eprintln!("  EVP_CIPHER_CTX_copy: {} calls", EVP_CIPHER_CTX_copy); }
    let EVP_CIPHER_CTX_ctrl = EVP_CIPHER_CTX_CTRL_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_ctrl > 0 { eprintln!("  EVP_CIPHER_CTX_ctrl: {} calls", EVP_CIPHER_CTX_ctrl); }
    let EVP_CIPHER_CTX_free = EVP_CIPHER_CTX_FREE_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_free > 0 { eprintln!("  EVP_CIPHER_CTX_free: {} calls", EVP_CIPHER_CTX_free); }
    let EVP_CIPHER_CTX_new = EVP_CIPHER_CTX_NEW_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_new > 0 { eprintln!("  EVP_CIPHER_CTX_new: {} calls", EVP_CIPHER_CTX_new); }
    let EVP_CIPHER_CTX_reset = EVP_CIPHER_CTX_RESET_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_reset > 0 { eprintln!("  EVP_CIPHER_CTX_reset: {} calls", EVP_CIPHER_CTX_reset); }
    let EVP_CIPHER_CTX_set_padding = EVP_CIPHER_CTX_SET_PADDING_COUNT.load(Ordering::SeqCst);
    if EVP_CIPHER_CTX_set_padding > 0 { eprintln!("  EVP_CIPHER_CTX_set_padding: {} calls", EVP_CIPHER_CTX_set_padding); }
    let EVP_DecryptFinal_ex = EVP_DECRYPTFINAL_EX_COUNT.load(Ordering::SeqCst);
    if EVP_DecryptFinal_ex > 0 { eprintln!("  EVP_DecryptFinal_ex: {} calls", EVP_DecryptFinal_ex); }
    let EVP_DecryptInit_ex = EVP_DECRYPTINIT_EX_COUNT.load(Ordering::SeqCst);
    if EVP_DecryptInit_ex > 0 { eprintln!("  EVP_DecryptInit_ex: {} calls", EVP_DecryptInit_ex); }
    let EVP_DecryptUpdate = EVP_DECRYPTUPDATE_COUNT.load(Ordering::SeqCst);
    if EVP_DecryptUpdate > 0 { eprintln!("  EVP_DecryptUpdate: {} calls", EVP_DecryptUpdate); }
    let EVP_DigestFinal = EVP_DIGESTFINAL_COUNT.load(Ordering::SeqCst);
    if EVP_DigestFinal > 0 { eprintln!("  EVP_DigestFinal: {} calls", EVP_DigestFinal); }
    let EVP_DigestInit_ex = EVP_DIGESTINIT_EX_COUNT.load(Ordering::SeqCst);
    if EVP_DigestInit_ex > 0 { eprintln!("  EVP_DigestInit_ex: {} calls", EVP_DigestInit_ex); }
    let EVP_DigestUpdate = EVP_DIGESTUPDATE_COUNT.load(Ordering::SeqCst);
    if EVP_DigestUpdate > 0 { eprintln!("  EVP_DigestUpdate: {} calls", EVP_DigestUpdate); }
    let EVP_EncryptFinal_ex = EVP_ENCRYPTFINAL_EX_COUNT.load(Ordering::SeqCst);
    if EVP_EncryptFinal_ex > 0 { eprintln!("  EVP_EncryptFinal_ex: {} calls", EVP_EncryptFinal_ex); }
    let EVP_EncryptInit_ex = EVP_ENCRYPTINIT_EX_COUNT.load(Ordering::SeqCst);
    if EVP_EncryptInit_ex > 0 { eprintln!("  EVP_EncryptInit_ex: {} calls", EVP_EncryptInit_ex); }
    let EVP_EncryptUpdate = EVP_ENCRYPTUPDATE_COUNT.load(Ordering::SeqCst);
    if EVP_EncryptUpdate > 0 { eprintln!("  EVP_EncryptUpdate: {} calls", EVP_EncryptUpdate); }
    let EVP_MAC_CTX_free = EVP_MAC_CTX_FREE_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_CTX_free > 0 { eprintln!("  EVP_MAC_CTX_free: {} calls", EVP_MAC_CTX_free); }
    let EVP_MAC_CTX_new = EVP_MAC_CTX_NEW_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_CTX_new > 0 { eprintln!("  EVP_MAC_CTX_new: {} calls", EVP_MAC_CTX_new); }
    let EVP_MAC_fetch = EVP_MAC_FETCH_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_fetch > 0 { eprintln!("  EVP_MAC_fetch: {} calls", EVP_MAC_fetch); }
    let EVP_MAC_final = EVP_MAC_FINAL_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_final > 0 { eprintln!("  EVP_MAC_final: {} calls", EVP_MAC_final); }
    let EVP_MAC_free = EVP_MAC_FREE_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_free > 0 { eprintln!("  EVP_MAC_free: {} calls", EVP_MAC_free); }
    let EVP_MAC_init = EVP_MAC_INIT_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_init > 0 { eprintln!("  EVP_MAC_init: {} calls", EVP_MAC_init); }
    let EVP_MAC_update = EVP_MAC_UPDATE_COUNT.load(Ordering::SeqCst);
    if EVP_MAC_update > 0 { eprintln!("  EVP_MAC_update: {} calls", EVP_MAC_update); }
    let EVP_MD_CTX_free = EVP_MD_CTX_FREE_COUNT.load(Ordering::SeqCst);
    if EVP_MD_CTX_free > 0 { eprintln!("  EVP_MD_CTX_free: {} calls", EVP_MD_CTX_free); }
    let EVP_MD_CTX_new = EVP_MD_CTX_NEW_COUNT.load(Ordering::SeqCst);
    if EVP_MD_CTX_new > 0 { eprintln!("  EVP_MD_CTX_new: {} calls", EVP_MD_CTX_new); }
    let EVP_MD_CTX_set_flags = EVP_MD_CTX_SET_FLAGS_COUNT.load(Ordering::SeqCst);
    if EVP_MD_CTX_set_flags > 0 { eprintln!("  EVP_MD_CTX_set_flags: {} calls", EVP_MD_CTX_set_flags); }
    let EVP_MD_get_size = EVP_MD_GET_SIZE_COUNT.load(Ordering::SeqCst);
    if EVP_MD_get_size > 0 { eprintln!("  EVP_MD_get_size: {} calls", EVP_MD_get_size); }
    let EVP_aes_256_cbc = EVP_AES_256_CBC_COUNT.load(Ordering::SeqCst);
    if EVP_aes_256_cbc > 0 { eprintln!("  EVP_aes_256_cbc: {} calls", EVP_aes_256_cbc); }
    let EVP_aes_256_ctr = EVP_AES_256_CTR_COUNT.load(Ordering::SeqCst);
    if EVP_aes_256_ctr > 0 { eprintln!("  EVP_aes_256_ctr: {} calls", EVP_aes_256_ctr); }
    let EVP_aes_256_ecb = EVP_AES_256_ECB_COUNT.load(Ordering::SeqCst);
    if EVP_aes_256_ecb > 0 { eprintln!("  EVP_aes_256_ecb: {} calls", EVP_aes_256_ecb); }
    let EVP_aes_256_gcm = EVP_AES_256_GCM_COUNT.load(Ordering::SeqCst);
    if EVP_aes_256_gcm > 0 { eprintln!("  EVP_aes_256_gcm: {} calls", EVP_aes_256_gcm); }
    let EVP_md5 = EVP_MD5_COUNT.load(Ordering::SeqCst);
    if EVP_md5 > 0 { eprintln!("  EVP_md5: {} calls", EVP_md5); }
    let EVP_sha1 = EVP_SHA1_COUNT.load(Ordering::SeqCst);
    if EVP_sha1 > 0 { eprintln!("  EVP_sha1: {} calls", EVP_sha1); }
    let EVP_sha256 = EVP_SHA256_COUNT.load(Ordering::SeqCst);
    if EVP_sha256 > 0 { eprintln!("  EVP_sha256: {} calls", EVP_sha256); }
    let GC_add_roots = GC_ADD_ROOTS_COUNT.load(Ordering::SeqCst);
    if GC_add_roots > 0 { eprintln!("  GC_add_roots: {} calls", GC_add_roots); }
    let GC_expand_hp = GC_EXPAND_HP_COUNT.load(Ordering::SeqCst);
    if GC_expand_hp > 0 { eprintln!("  GC_expand_hp: {} calls", GC_expand_hp); }
    let GC_free = GC_FREE_COUNT.load(Ordering::SeqCst);
    if GC_free > 0 { eprintln!("  GC_free: {} calls", GC_free); }
    let GC_gcollect = GC_GCOLLECT_COUNT.load(Ordering::SeqCst);
    if GC_gcollect > 0 { eprintln!("  GC_gcollect: {} calls", GC_gcollect); }
    let GC_get_bytes_since_gc = GC_GET_BYTES_SINCE_GC_COUNT.load(Ordering::SeqCst);
    if GC_get_bytes_since_gc > 0 { eprintln!("  GC_get_bytes_since_gc: {} calls", GC_get_bytes_since_gc); }
    let GC_get_heap_usage_safe = GC_GET_HEAP_USAGE_SAFE_COUNT.load(Ordering::SeqCst);
    if GC_get_heap_usage_safe > 0 { eprintln!("  GC_get_heap_usage_safe: {} calls", GC_get_heap_usage_safe); }
    let GC_init = GC_INIT_COUNT.load(Ordering::SeqCst);
    if GC_init > 0 { eprintln!("  GC_init: {} calls", GC_init); }
}
