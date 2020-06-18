//! Bindings to [PARDISO].
//! 
//! [PARDISO]: https://www.pardiso-project.org/

#![allow(non_camel_case_types)]
#![no_std]

extern crate libc;

use libc::{c_char, c_double, c_int, c_longlong, c_void};
pub type mkl_int = c_int;
pub type _mkl_dss_handle_t = c_void;

// === Deprecated API ===
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// #[repr(C)]
// pub enum PARDISO_ENV_PARAM {
//     PARDISO_OOC_FILE_NAME = 1,
// }

extern "C" {
    pub fn pardiso(
        pt: *mut _mkl_dss_handle_t,
        maxfct: *const mkl_int,
        mnum: *const mkl_int,
        mtype: *const mkl_int,
        phase: *const mkl_int,
        n: *const mkl_int,
        a: *const c_void,
        ia: *const mkl_int,
        ja: *const mkl_int,
        perm: *mut mkl_int,
        nrhs: *const mkl_int,
        iparm: *mut mkl_int,
        msglvl: *const mkl_int,
        b: *mut c_void,
        x: *mut c_void,
        error: *mut mkl_int,
    );
    pub fn pardisoinit(
        pt: *mut _mkl_dss_handle_t,
        mtype: *const mkl_int,
        iparm: *mut mkl_int,
    );
    pub fn pardiso_64(
        pt: *mut _mkl_dss_handle_t,
        maxfct: *const c_longlong,
        mnum: *const c_longlong,
        mtype: *const c_longlong,
        phase: *const c_longlong,
        n: *const c_longlong,
        a: *const c_void,
        ia: *const c_longlong,
        ja: *const c_longlong,
        perm: *mut c_longlong,
        nrhs: *const c_longlong,
        iparm: *mut c_longlong,
        msglvl: *const c_longlong,
        b: *mut c_void,
        x: *mut c_void,
        error: *mut c_longlong,
    );
    pub fn pardiso_handle_store_64(
        pt: *mut _mkl_dss_handle_t,
        dirname: *const c_char,
        error: *mut c_longlong,
    );
    pub fn pardiso_handle_restore_64(
        pt: *mut _mkl_dss_handle_t,
        dirname: *const c_char,
        error: *mut c_longlong,
    );
    pub fn pardiso_handle_delete_64(
        dirname: *const c_char,
        error: *mut c_longlong,
    );
    pub fn pardiso_handle_store(
        pt: *mut _mkl_dss_handle_t,
        dirname: *const c_char,
        error: *mut mkl_int,
    );
    pub fn pardiso_handle_restore(
        pt: *mut _mkl_dss_handle_t,
        dirname: *const c_char,
        error: *mut mkl_int,
    );
    pub fn pardiso_handle_delete(
        dirname: *const c_char,
        error: *mut mkl_int,
    );
    pub fn mkl_pardiso_pivot(
        aii: *const c_void,
        bii: *mut c_void,
        eps: *const c_void,
    ) -> c_int;
    pub fn pardiso_getdiag(
        pt: *const _mkl_dss_handle_t,
        df: *mut c_void,
        da: *mut c_void,
        mnum: *const mkl_int,
        error: *mut mkl_int,
    );
    pub fn pardiso_export(
        pt: *mut _mkl_dss_handle_t,
        values: *mut c_void,
        rows: *mut mkl_int,
        columns: *mut mkl_int,
        step: *const mkl_int,
        iparm: *const mkl_int,
        error: *mut mkl_int,
    );
}