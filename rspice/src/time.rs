use crate::helper::SString;
use crate::SpiceResult;

pub fn unitim(epoch: f64, insys: &str, outsys: &str) -> SpiceResult<f64> {
    let mut i = SString::new(insys.as_bytes());
    let mut o = SString::new(outsys.as_bytes());
    s_errn!(cspice_sys::unitim_c(epoch, i.as_cs(), o.as_cs()))
}

pub fn str2et(date: &str) -> SpiceResult<f64> {
    let mut u = SString::new(date.as_bytes());
    let mut o = 0.0_f64;
    s_err!(cspice_sys::str2et_c(u.as_cs(), &mut o));
    s_ok!(o)
}
