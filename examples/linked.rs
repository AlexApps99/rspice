use cspice_sys::*;

fn main() {
    unsafe {
        let name = std::ffi::CString::new("kernels/all.tm").unwrap().into_raw();
        furnsh_c(name);
        unload_c(name);
        std::ffi::CString::from_raw(name);
    }
}
