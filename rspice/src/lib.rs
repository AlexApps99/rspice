#[macro_use]
pub(crate) mod helper;
use helper::SString;
use std::convert::TryInto;

#[cfg(feature = "error")]
#[derive(Debug, Clone)]
pub struct SpiceError {
    pub short: String,
    pub explain: String,
    pub long: String,
    pub trace: String,
}

#[cfg(feature = "error")]
type SpiceResult<T> = Result<T, SpiceError>;

#[cfg(not(feature = "error"))]
type SpiceResult<T> = T;

#[cfg(feature = "error")]
impl SpiceError {
    pub(crate) unsafe fn get_unchecked() -> Self {
        let mut s = SString::new(b"SHORT" as &[u8]);
        let mut e = SString::new(b"EXPLAIN" as &[u8]);
        let mut l = SString::new(b"LONG" as &[u8]);
        let mut s_o = SString::with_size(26);
        let mut e_o = SString::with_size(321);
        let mut l_o = SString::with_size(321);
        cspice_sys::getmsg_c(s.as_cs(), s_o.len(), s_o.as_cs());
        cspice_sys::getmsg_c(e.as_cs(), e_o.len(), e_o.as_cs());
        cspice_sys::getmsg_c(l.as_cs(), l_o.len(), l_o.as_cs());

        let mut t_o = SString::with_size(cspice_sys::SPICE_ERROR_TRCLEN as i32); // TODO Should be i32 hmm
        cspice_sys::qcktrc_c(cspice_sys::SPICE_ERROR_TRCLEN as i32, t_o.as_cs());

        Self {
            short: s_o.try_into().unwrap(),
            explain: e_o.try_into().unwrap(),
            long: l_o.try_into().unwrap(),
            trace: t_o.try_into().unwrap(),
        }
    }
}

#[cfg(feature = "error")]
impl std::error::Error for SpiceError {}

#[cfg(feature = "error")]
impl std::fmt::Display for SpiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tk = SString::new(b"TOOLKIT" as &[u8]);
        write!(
            f,
            "\n\
            ================================================================================\n\
            \n\
            Toolkit version: {}\n\
            \n\
            {} --\n\
            {}\n\
            {}\n\
            \n\
            A traceback follows.  The name of the highest level module is first.\n\
            {}\n\
            \n\
            ================================================================================",
            unsafe {
                std::ffi::CStr::from_ptr(cspice_sys::tkvrsn_c(tk.as_cs()).offset(7))
                    .to_str()
                    .unwrap()
            },
            self.short,
            self.explain,
            self.long,
            self.trace,
        )
    }
}

s_sgfn!(erract, cspice_sys::erract_c, action);
s_sgfn!(errdev, cspice_sys::errdev_c, device);
s_sgfn!(errprt, cspice_sys::errprt_c, list);

pub fn failed() -> bool {
    unsafe { cspice_sys::failed_c() != 0 }
}

pub fn reset() {
    unsafe { cspice_sys::reset_c() }
}

pub fn getmsg(option: &str, lenout: i32) -> SpiceResult<String> {
    let mut o = SString::new(option.as_bytes());
    let (b, mut c) = if lenout < 2 {
        (2, SString::with_size(2))
    } else {
        (lenout, SString::with_size(lenout))
    };
    s_err!(cspice_sys::getmsg_c(o.as_cs(), b, c.as_cs()));
    s_ok!(c.try_into().unwrap())
}

pub fn furnsh(file: &str) -> SpiceResult<()> {
    let mut f = SString::new(file.as_bytes());
    s_errn!(cspice_sys::furnsh_c(f.as_cs()))
}

pub fn unload(file: &str) -> SpiceResult<()> {
    let mut f = SString::new(file.as_bytes());
    s_errn!(cspice_sys::unload_c(f.as_cs()))
}

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

pub fn unitim(epoch: f64, insys: &str, outsys: &str) -> SpiceResult<f64> {
    let mut i = SString::new(insys.as_bytes());
    let mut o = SString::new(outsys.as_bytes());
    s_errn!(cspice_sys::unitim_c(epoch, i.as_cs(), o.as_cs()))
}

pub fn utc2et(utcstr: &str) -> SpiceResult<f64> {
    let mut u = SString::new(utcstr.as_bytes());
    let mut o = 0.0_f64;
    s_err!(cspice_sys::utc2et_c(u.as_cs(), &mut o));
    s_ok!(o)
}
