//! Functions to go back and forth between WASI types in host and wasm32 representations.
//!
//! This module is an adaptation of the `wasmtime-wasi` module
//! [`translate.rs`](https://github.com/CraneStation/wasmtime-wasi/blob/1a6ecf3a0378d71f3fc1ba25ce76a2b43e4166b8/lib/wasi/src/translate.rs);
//! its license file `LICENSE.wasmtime-wasi` is included in this project.
//!
//! Any of these functions that take a `Vmctx` argument are only meant to be called from within a
//! hostcall.
//!
//! This sort of manual encoding will hopefully be obsolete once the IDL is developed.

use lucet_runtime::vmctx::Vmctx;
use wasi_common::{host, wasm32};

macro_rules! bail_errno {
    ( $errno:ident ) => {
        return Err(host::$errno as host::__wasi_errno_t);
    };
}

pub unsafe fn dec_ptr(
    vmctx: &Vmctx,
    ptr: wasm32::uintptr_t,
    len: usize,
) -> Result<*mut u8, host::__wasi_errno_t> {
    let mut heap = vmctx.heap_mut();

    // check that `len` fits in the wasm32 address space
    if len > wasm32::UINTPTR_MAX as usize {
        bail_errno!(__WASI_EOVERFLOW);
    }

    // check that `ptr` and `ptr + len` are both within the guest heap
    if ptr as usize > heap.len() || ptr as usize + len > heap.len() {
        bail_errno!(__WASI_EFAULT);
    }

    // translate the pointer
    Ok(heap.as_mut_ptr().offset(ptr as isize))
}
