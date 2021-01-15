#[macro_use]
pub(crate) mod helper;

cspice_sgfn!(erract, cspice_sys::erract_c, action);
cspice_sgfn!(errdev, cspice_sys::errdev_c, device);
cspice_sgfn!(errprt, cspice_sys::errprt_c, list);
