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

pub fn pxfrm2(from: &str, to: &str, etfrom: f64, etto: f64) -> SpiceResult<[[f64; 3]; 3]> {
    let mut f = SString::new(from.as_bytes());
    let mut t = SString::new(to.as_bytes());
    let mut o = [[0.0_f64; 3]; 3];

    s_err!(cspice_sys::pxfrm2_c(
        f.as_cs(),
        t.as_cs(),
        etfrom,
        etto,
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

pub fn bodfnd(body: i32, item: &str) -> SpiceResult<bool> {
    let mut i = SString::new(item.as_bytes());
    s_ok!(s_err!(cspice_sys::bodfnd_c(body, i.as_cs())) != 0)
}

pub fn bodvrd(bodynm: &str, item: &str, maxn: i32) -> SpiceResult<Vec<f64>> {
    let mut b = SString::new(bodynm.as_bytes());
    let mut i = SString::new(item.as_bytes());
    use std::convert::TryInto;
    if let Ok(sz) = TryInto::<usize>::try_into(maxn) {
        if sz > 0 {
            let mut values = vec![0_f64; sz];
            let mut n = 0_i32;
            s_err!(cspice_sys::bodvrd_c(
                b.as_cs(),
                i.as_cs(),
                maxn,
                &mut n,
                values.as_mut_ptr()
            ));
            values.truncate(n as usize);
            return s_ok!(values);
        }
    }
    s_err!(cspice_sys::bodvrd_c(
        b.as_cs(),
        i.as_cs(),
        maxn,
        std::ptr::null_mut(),
        std::ptr::null_mut()
    ));
    unreachable!()
}
