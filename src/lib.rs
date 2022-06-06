use libc::{c_char, c_float, c_int, c_uint, size_t};
use std::ffi::CString;
use std::path::Path;
use std::slice::from_raw_parts;

pub enum AnnoyIndexInterface {}

pub mod ffi {
    use super::*;

    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn annoy_index_angular(f: c_int) -> *mut AnnoyIndexInterface;
        pub fn annoy_delete_index(index: *mut AnnoyIndexInterface);
        pub fn annoy_add_item(
            index: *mut AnnoyIndexInterface,
            item: c_uint,
            w: *const c_float,
        ) -> bool;
        pub fn annoy_build(index: *mut AnnoyIndexInterface, q: c_int) -> bool;

        pub fn annoy_load(index: *mut AnnoyIndexInterface, p: *const c_char) -> bool;

        pub fn annoy_unload(index: *mut AnnoyIndexInterface);

        pub fn annoy_save(index: *mut AnnoyIndexInterface, p: *const c_char) -> bool;

        pub fn annoy_get_item(
            index: *mut AnnoyIndexInterface,
            item: c_uint,
            result: *mut c_float,
        ) -> bool;

        pub fn annoy_get_n_items(index: *mut AnnoyIndexInterface) -> c_uint;

        pub fn annoy_get_nns_by_item(
            index: *mut AnnoyIndexInterface,
            item: c_uint,
            n: size_t,
            search_k: c_int,
            result: *mut c_uint,
            distances: *mut c_float,
        ) -> size_t;

        pub fn annoy_get_nns_by_vector(
            index: *mut AnnoyIndexInterface,
            w: *const c_float,
            n: size_t,
            search_k: c_int,
            result: *mut c_uint,
            distances: *mut c_float,
        ) -> size_t;
    }
}

pub struct Rannoy(usize, *mut AnnoyIndexInterface);

impl Rannoy {
    pub fn new(n: usize) -> Self {
        let index = unsafe { ffi::annoy_index_angular(n as i32) };

        Rannoy(n, index)
    }

    pub fn add_item(&self, item: u32, w: &Vec<f32>) -> bool {
        unsafe { ffi::annoy_add_item(self.1, item, w.as_ptr()) }
    }

    pub fn build(&self, n: i32) -> bool {
        unsafe { ffi::annoy_build(self.1, n) }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> bool {
        unsafe {
            path.as_ref()
                .to_str()
                .map(CString::new)
                .and_then(Result::ok)
                .map(|x| ffi::annoy_save(self.1, x.as_ptr()))
                .unwrap_or_default()
        }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> bool {
        unsafe {
            path.as_ref()
                .to_str()
                .map(CString::new)
                .and_then(Result::ok)
                .map(|x| ffi::annoy_load(self.1, x.as_ptr()))
                .unwrap_or_default()
        }
    }

    pub fn unload(&self) {
        unsafe {
            ffi::annoy_unload(self.1);
        }
    }

    pub fn get_nns_by_item(&self, item: u32, n: usize, search_k: i32) -> (Vec<u32>, Vec<f32>) {
        let mut result = Vec::with_capacity(n);
        let result_ptr = result.as_mut_ptr();

        let mut distance = Vec::with_capacity(n);
        let distance_ptr = distance.as_mut_ptr();

        unsafe {
            let size =
                ffi::annoy_get_nns_by_item(self.1, item, n, search_k, result_ptr, distance_ptr);

            let a = from_raw_parts(result_ptr, size);
            let b = from_raw_parts(distance_ptr, size);

            (a.to_vec(), b.to_vec())
        }
    }

    pub fn get_nns_by_vector(&self, w: &[f32], n: usize, search_k: i32) -> (Vec<u32>, Vec<f32>) {
        let mut result = Vec::with_capacity(n);
        let result_ptr = result.as_mut_ptr();

        let mut distance = Vec::with_capacity(n);
        let distance_ptr = distance.as_mut_ptr();

        unsafe {
            let size = ffi::annoy_get_nns_by_vector(
                self.1,
                w.as_ptr(),
                n,
                search_k,
                result_ptr,
                distance_ptr,
            );

            let a = from_raw_parts(result_ptr, size);
            let b = from_raw_parts(distance_ptr, size);

            (a.to_vec(), b.to_vec())
        }
    }

    pub fn get_n_items(&self) -> u32 {
        unsafe { ffi::annoy_get_n_items(self.1) }
    }

    pub fn get_item(&self, item: u32) -> Vec<f32> {
        let mut vct = vec![0f32; self.0];
        let vct_ptr = vct.as_mut_ptr();
        unsafe {
            if ffi::annoy_get_item(self.1, item, vct_ptr) {
                vct
            } else {
                vec![]
            }
        }
    }
}

impl Drop for Rannoy {
    fn drop(&mut self) {
        unsafe {
            ffi::annoy_delete_index(self.1);
        }
    }
}
