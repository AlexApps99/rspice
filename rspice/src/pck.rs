use crate::helper::SString;
use crate::SpiceResult;

pub fn pxform(from: &str, to: &str, et: f64) -> SpiceResult<[[f64; 3]; 3]> {
    let mut f = SString::new(from.as_bytes());
    let mut t = SString::new(to.as_bytes());
    let mut o = [[0.0_f64; 3]; 3];

    s_err!(cspice_sys::pxform_c(
        f.as_cs(),
        t.as_cs(),
        et,
        o.as_mut_ptr()
    ));
    s_ok!(o)
}

pub fn sxform(from: &str, to: &str, et: f64) -> SpiceResult<[[f64; 6]; 6]> {
    let mut f = SString::new(from.as_bytes());
    let mut t = SString::new(to.as_bytes());
    let mut o = [[0.0_f64; 6]; 6];

    s_err!(cspice_sys::sxform_c(
        f.as_cs(),
        t.as_cs(),
        et,
        o.as_mut_ptr()
    ));
    s_ok!(o)
}
