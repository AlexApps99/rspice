fn main() {
    #[cfg(feature = "error")]
    if let Err(e) = spice_e() {
        eprintln!("{}", e);
    }

    #[cfg(not(feature = "error"))]
    spice_n();
}

#[cfg(feature = "error")]
fn spice_e() -> Result<(), rspice::SpiceError> {
    rspice::erract("SET", 0, Some("RETURN"))?;
    rspice::errdev("SET", 0, Some("NULL"))?;
    rspice::furnsh("kernels/all.tm")?;
    let et = rspice::str2et("2021-01-01T00:00:00")?;
    let tdt = rspice::unitim(et, "ET", "TDT")?;
    let m = rspice::pxform("IAU_EARTH", "IAU_MOON", et)?;
    let m = nalgebra::Matrix3::from_row_slice(unsafe {
        &std::mem::transmute::<[[f64; 3]; 3], [f64; 9]>(m)
    });
    println!("ET: {}\nTDT: {}\n{}", et, tdt, m);
    rspice::unload("kernels/all.tm")?;
    Ok(())
}

#[cfg(not(feature = "error"))]
fn spice_n() {
    rspice::erract("SET", 0, Some("ABORT"));
    rspice::furnsh("kernels/all.tm");
    let et = rspice::str2et("2021-01-01T00:00:00");
    let tdt = rspice::unitim(et, "ET", "TDT");
    let m = rspice::pxform("IAU_EARTH", "IAU_MOON", et);
    let m = nalgebra::Matrix3::from_row_slice(unsafe {
        &std::mem::transmute::<[[f64; 3]; 3], [f64; 9]>(m)
    });
    println!("ET: {}\nTDT: {}\n{}", et, tdt, m);
    rspice::unload("kernels/all.tm");
}
