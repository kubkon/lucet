//! Hostcalls that implement
//! [WASI](https://github.com/CraneStation/wasmtime-wasi/blob/wasi/docs/WASI-overview.md).
//!
//! This code borrows heavily from [wasmtime-wasi](https://github.com/CraneStation/wasmtime-wasi),
//! which in turn borrows from cloudabi-utils. See `LICENSE.wasmtime-wasi` for license information.
//!
//! This is currently a very incomplete prototype, only supporting the hostcalls required to run
//! `/examples/hello.c`, and using a bare-bones translation of the capabilities system rather than
//! something nice.

#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]
use crate::memory;

use lucet_runtime::vmctx::Vmctx;
use lucet_runtime::{lucet_hostcall_terminate, lucet_hostcalls};
use wasi_common::ctx::{VmContext, WasiCtx};
use wasi_common::{host, wasm32};

struct VmCtx {
    vmctx: *mut Vmctx,
}

impl VmContext for VmCtx {
    fn as_wasi_ctx(&self) -> *const WasiCtx {
        unsafe { &*(&*self.vmctx).get_embed_ctx::<WasiCtx>() }
    }

    fn as_wasi_ctx_mut(&mut self) -> *mut WasiCtx {
        unsafe { &mut *(&mut *self.vmctx).get_embed_ctx_mut::<WasiCtx>() }
    }

    unsafe fn dec_ptr(
        &mut self,
        ptr: wasm32::uintptr_t,
        len: usize,
    ) -> Result<*mut u8, host::__wasi_errno_t> {
        unsafe { memory::dec_ptr(&*self.vmctx, ptr, len) }
    }
}

lucet_hostcalls! {
    #[no_mangle]
    pub unsafe extern "C" fn __wasi_proc_exit(
        &mut _vmctx,
        rval: wasm32::__wasi_exitcode_t,
    ) -> ! {
        lucet_hostcall_terminate!(wasi_common::memory::dec_exitcode(rval));
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_args_get(
        &mut vmctx,
        argv_ptr: wasm32::uintptr_t,
        argv_buf: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::args_get(&mut vmctx, argv_ptr, argv_buf)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_args_sizes_get(
        &mut vmctx,
        argc_ptr: wasm32::uintptr_t,
        argv_buf_size_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::args_sizes_get(&mut vmctx, argc_ptr, argv_buf_size_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_clock_res_get(
        &mut vmctx,
        clock_id: wasm32::__wasi_clockid_t,
        resolution_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::clock_res_get(&mut vmctx, clock_id, resolution_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_clock_time_get(
        &mut vmctx,
        clock_id: wasm32::__wasi_clockid_t,
        // ignored for now, but will be useful once we put optional limits on precision to reduce side
        // channels
        precision: wasm32::__wasi_timestamp_t,
        time_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::clock_time_get(&mut vmctx, clock_id, precision, time_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_environ_get(
        &mut vmctx,
        environ_ptr: wasm32::uintptr_t,
        environ_buf: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::environ_get(&mut vmctx, environ_ptr, environ_buf)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_environ_sizes_get(
        &mut vmctx,
        environ_count_ptr: wasm32::uintptr_t,
        environ_size_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::environ_sizes_get(&mut vmctx, environ_count_ptr, environ_size_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_close(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_close(&mut vmctx, fd)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_fdstat_get(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        fdstat_ptr: wasm32::uintptr_t, // *mut wasm32::__wasi_fdstat_t
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_fdstat_get(&mut vmctx, fd, fdstat_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_fdstat_set_flags(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        fdflags: wasm32::__wasi_fdflags_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_fdstat_set_flags(&mut vmctx, fd, fdflags)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_seek(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        offset: wasm32::__wasi_filedelta_t,
        whence: wasm32::__wasi_whence_t,
        newoffset: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_seek(&mut vmctx, fd, offset, whence, newoffset)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_prestat_get(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        prestat_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_prestat_get(&mut vmctx, fd, prestat_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_prestat_dir_name(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        path_ptr: wasm32::uintptr_t,
        path_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_prestat_dir_name(&mut vmctx, fd, path_ptr, path_len)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_read(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        iovs_ptr: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        nread: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_read(&mut vmctx, fd, iovs_ptr, iovs_len, nread)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_write(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        iovs_ptr: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        nwritten: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_write(&mut vmctx, fd, iovs_ptr, iovs_len, nwritten)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_path_open(
        &mut vmctx,
        dirfd: wasm32::__wasi_fd_t,
        dirflags: wasm32::__wasi_lookupflags_t,
        path_ptr: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        oflags: wasm32::__wasi_oflags_t,
        fs_rights_base: wasm32::__wasi_rights_t,
        fs_rights_inheriting: wasm32::__wasi_rights_t,
        fs_flags: wasm32::__wasi_fdflags_t,
        fd_out_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::path_open(
            &mut vmctx,
            dirfd,
            dirflags,
            path_ptr,
            path_len,
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            fd_out_ptr,
        )
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_random_get(
        &mut vmctx,
        buf_ptr: wasm32::uintptr_t,
        buf_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::random_get(&mut vmctx, buf_ptr, buf_len)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_poll_oneoff(
        &mut vmctx,
        input: wasm32::uintptr_t,
        output: wasm32::uintptr_t,
        nsubscriptions: wasm32::size_t,
        nevents: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::poll_oneoff(&mut vmctx, input, output, nsubscriptions, nevents)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_fd_filestat_get(
        &mut vmctx,
        fd: wasm32::__wasi_fd_t,
        filestat_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::fd_filestat_get(&mut vmctx, fd, filestat_ptr)
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_path_filestat_get(
        &mut vmctx,
        dirfd: wasm32::__wasi_fd_t,
        dirflags: wasm32::__wasi_lookupflags_t,
        path_ptr: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        filestat_ptr: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::path_filestat_get(
            &mut vmctx,
            dirfd,
            dirflags,
            path_ptr,
            path_len,
            filestat_ptr,
        )
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_path_create_directory(
        &mut vmctx,
        dirfd: wasm32::__wasi_fd_t,
        path_ptr: wasm32::uintptr_t,
        path_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::path_create_directory(
            &mut vmctx,
            dirfd,
            path_ptr,
            path_len,
        )
    }

    #[no_mangle]
    pub unsafe extern "C" fn __wasi_path_unlink_file(
        &mut vmctx,
        dirfd: wasm32::__wasi_fd_t,
        path_ptr: wasm32::uintptr_t,
        path_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        let mut vmctx = VmCtx { vmctx };
        wasi_common::hostcalls::path_unlink_file(
            &mut vmctx,
            dirfd,
            path_ptr,
            path_len,
        )
    }
}

#[doc(hidden)]
pub fn ensure_linked() {
    unsafe {
        std::ptr::read_volatile(__wasi_proc_exit as *const extern "C" fn());
    }
}
