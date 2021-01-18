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

pub fn timout(et: f64, pictur: &str, lenout: i32) -> SpiceResult<String> {
    let mut p = SString::new(pictur.as_bytes());
    if lenout >= 2 {
        let mut o = SString::with_size(lenout);
        s_err!(cspice_sys::timout_c(et, p.as_cs(), o.len(), o.as_cs()));
        use std::convert::TryInto;
        s_ok!(o.try_into().unwrap())
    } else {
        s_err!(cspice_sys::timout_c(
            et,
            p.as_cs(),
            lenout,
            std::ptr::null_mut()
        ));
        unreachable!()
    }
}
