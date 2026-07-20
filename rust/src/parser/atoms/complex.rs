use chumsky::primitive::choice;
use derive_more::{Deref, DerefMut, From, Into};
use sertyp::{
    Content, Item,
    chumsky::parser::{auto_radix, character, unsigned_float_no_radix},
    content, equation,
};

/// Wrapper for num::Complex<T>
#[derive(Debug, Clone, Copy, Hash, From, Into, Deref, DerefMut)]
pub struct Complex<T>(num::Complex<T>);

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex(num::Complex::new(re, im))
    }
}

impl<'data> From<Complex<f64>> for Content<'data> {
    fn from(val: Complex<f64>) -> Self {
        fn format_float(f: f64) -> String {
            if f.is_infinite() {
                "∞".into()
            } else if f.fract() == 0.0 {
                format!("{:.1}", f)
            } else {
                format!("{:?}", f)
            }
        }

        let mut seq = vec![];
        if val.re != 0.0 || val.im == 0.0 {
            seq.push(format_float(val.re).into())
        }
        if val.re != 0.0 && val.im > 0.0 {
            seq.push('+'.into())
        }
        if val.im != 0.0 {
            seq.push(format!("{}i", format_float(val.im)).into())
        }
        content!(equation!(seq))
    }
}

impl<'data> From<Complex<f64>> for Item<'data> {
    fn from(val: Complex<f64>) -> Self {
        Content::from(val).into()
    }
}

/// parses a complex number
///
/// # EBNF
/// <float> ["i"] | "i"
#[kalt_macros::parser]
pub fn complex() -> num::Complex<f64> {
    fn real(f: f64) -> num::Complex<f64> {
        num::Complex::new(f, 0.0)
    }
    fn imag(f: f64) -> num::Complex<f64> {
        num::Complex::new(0.0, f)
    }

    choice((
        // <float> ["i"]
        auto_radix(unsigned_float_no_radix, 10)
            .then(character('i').or_not())
            .map(|(f, i)| match i {
                Some(_) => imag(f),
                None => real(f),
            }),
        // "i"
        character('i').map(|_| imag(1.0)),
    ))
}
