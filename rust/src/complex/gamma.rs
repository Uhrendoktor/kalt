/// Taken and adjusted from `spfunc` crate.
/// All credits go to the original author of the crate.
///
/// https://crates.io/crates/spfunc
use num::{
    FromPrimitive, One, ToPrimitive,
    complex::{Complex64, ComplexFloat},
};

pub const SQRT_2_PI: f64 = 2.5066282746310005;

/// Coefficients for calculating the gamma function.
const G_COF: [f64; 7] = [
    1.000000000190015,
    76.18009172947146,
    -86.50532032941677,
    24.01409824083091,
    -1.231739572450155,
    0.1208650973866179e-2,
    -0.5395239384953e-5,
];

/// Calculate $\ln{\Gamma(z)}$.
pub fn gamma_ln(z: Complex64) -> Complex64 {
    if z.re().to_f64().unwrap() < 0.5 {
        let pi = Complex64::from_f64(std::f64::consts::PI).unwrap();
        return pi.ln() - (pi * z).sin().ln() - gamma_ln(Complex64::one() - z);
    }

    let g_cof = G_COF
        .iter()
        .map(|&c| Complex64::from_f64(c).unwrap())
        .collect::<Vec<Complex64>>();
    let sqrt_2_pi = Complex64::from_f64(SQRT_2_PI).unwrap();
    let tmp = z + Complex64::from_f64(5.5).unwrap();
    let mut ser = g_cof[0];
    for i in 1..7 {
        ser += g_cof[i] / (z + Complex64::from_usize(i).unwrap());
    }
    -tmp + (z + Complex64::from_f64(0.5).unwrap()) * tmp.ln() + (sqrt_2_pi * ser / z).ln()
}

/// Calculate $\Gamma(z)$.
///
/// The result is given as $\exp(\ln{\Gamma(z)})$.
pub fn gamma(z: Complex64) -> Complex64 {
    gamma_ln(z).exp()
}
