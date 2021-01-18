use crate::helper::SString;
use crate::SpiceResult;

pub fn furnsh(file: &str) -> SpiceResult<()> {
    let mut f = SString::new(file.as_bytes());
    s_errn!(cspice_sys::furnsh_c(f.as_cs()))
}

pub fn unload(file: &str) -> SpiceResult<()> {
    let mut f = SString::new(file.as_bytes());
    s_errn!(cspice_sys::unload_c(f.as_cs()))
}
