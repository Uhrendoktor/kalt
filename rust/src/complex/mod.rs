use std::fmt::Debug;

use derive_more::{Deref, DerefMut, From, Into};
use num::Complex as _Complex;
use sertyp::{Content, Item, content, equation};

pub mod gamma;

#[derive(Debug, Clone, Copy, Hash, From, Into, Deref, DerefMut)]
pub struct Complex<T>(_Complex<T>);

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
