pub struct SString(Vec<i8>);

impl SString {
    pub fn new<'a, T: Into<&'a [u8]>>(t: T) -> Self {
        let v = Vec::from(unsafe { &*(t.into() as *const [u8] as *const [i8]) });

        let mut s = Self(v);
        s.replace_null();
        s
    }

    pub fn remove_null(v: &mut Vec<i8>) {
        if let Some(n) = v.iter().position(|&x| x == 0) {
            v.truncate(n);
        }
    }

    pub fn replace_null(&mut self) {
        Self::remove_null(&mut self.0);
        self.0.push(0);
    }

    pub fn with_size(s: i32) -> Self {
        Self(vec![0; s as usize])
    }

    pub fn as_cs(&mut self) -> *mut i8 {
        self.0.as_mut_ptr()
    }

    pub fn len(&self) -> i32 {
        self.0.len() as i32
    }
}

impl std::convert::TryInto<String> for SString {
    type Error = std::string::FromUtf8Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let mut v = self.0;
        Self::remove_null(&mut v);
        let mut v = std::mem::ManuallyDrop::new(v);
        String::from_utf8(unsafe {
            Vec::from_raw_parts(v.as_mut_ptr() as *mut u8, v.len(), v.capacity())
        })
    }
}

#[cfg(feature = "error")]
macro_rules! s_err {
    ($i:expr) => {
        s_errn!($i)?
    };
}

#[cfg(not(feature = "error"))]
macro_rules! s_err {
    ($i:expr) => {
        unsafe { $i }
    };
}

#[cfg(feature = "error")]
macro_rules! s_errn {
    ($i:expr) => {{
        let r = unsafe { $i };
        if $crate::failed() {
            let err = unsafe { $crate::SpiceError::get_unchecked() };
            $crate::reset();
            Err(err)
        } else {
            Ok(r)
        }
    }};
}

#[cfg(not(feature = "error"))]
macro_rules! s_errn {
    ($i:expr) => {
        unsafe { $i }
    };
}

#[cfg(feature = "error")]
macro_rules! s_ok {
    ($i:expr) => {
        Ok($i)
    };
}

#[cfg(not(feature = "error"))]
macro_rules! s_ok {
    ($i:expr) => {
        $i
    };
}

// Macro to help wrap get/set functions
// $n: Name of function to be defined
// $f: cspice_sys function to be called
// $c: thing to get/set
macro_rules! s_sgfn {
    ($n:ident, $f:path, $c:ident) => {
        pub fn $n(op: &str, lenout: i32, $c: Option<&str>) -> $crate::SpiceResult<String> {
            let mut a = $crate::helper::SString::new(op.as_bytes());

            let (b, mut c) = if let Some(cc) = $c {
                let s = $crate::helper::SString::new(cc.as_bytes());
                (s.len(), s)
            } else {
                if lenout < 2 {
                    (2, $crate::helper::SString::with_size(2))
                } else {
                    (lenout, $crate::helper::SString::with_size(lenout))
                }
            };

            s_err!($f(a.as_cs(), b, c.as_cs()));
            use std::convert::TryInto;
            s_ok!(c.try_into().unwrap())
        }
    };
}
