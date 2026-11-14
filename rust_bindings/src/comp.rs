use kalt::parser::{
    atoms::{complex::Complex, tensor::Tensor},
    ops::logic::boolean_indicator,
    pratt::pratt,
    validate,
    validator::matrix,
};
use sertyp::{
    Boolean, Content, Float, Integer, Item, LocatingSequence, Panic, Sequence, TypedArray,
    TypedContent, TypedSequence, TypstError, parse, typst_func,
};
use wasm_minimal_protocol::*;
initiate_protocol!();

#[typst_func()]
pub fn comp<'data>(body: Item<'data>) -> Item<'data> {
    let content = match body {
        Item::Boolean(Boolean(value)) => Complex::from(boolean_indicator(value)).into()
        Item::Integer(value) => {
            let value: Float = match value {
                Integer::i8(value) => value.into(),
                Integer::i16(value) => value.into(),
                Integer::i32(value) => value.into(),
                Integer::i64(value) => value.into(),
                Integer::i128(value) => value.into(),
                Integer::u8(value) => value.into(),
                Integer::u16(value) => value.into(),
                Integer::u32(value) => value.into(),
                Integer::u64(value) => value.into(),
                Integer::u128(value) => value.into(),
                Integer::isize(value) => value.into(),
                Integer::usize(value) => value.into(),
            };
            Complex::new(value.into(), 0.0).into()
        }
        Item::Float(Float::f64(value)) => Complex::new(value, 0.0).into(),
        Item::Decimal(value) => Complex::new(value.into(), 0.0).into(),
        Item::String(value) => Content::from(value),
        Item::Content(content) => *content,
        // Item inputs receive panics from SerTyp's #[typst_func] wrapper.
        Item::Panic(panic) => return panic.into(),
        _ => {
            return Panic::from(sertyp::String::from(
                "comp expects a Boolean, integer, float, decimal, string, or math content",
            ))
            .into();
        }
    };
    let seq = Sequence::from(content);
    let seq = LocatingSequence::from(&seq);
    let c = parse!(pratt(), &seq);

    // Output exception 1: unwrap a 1×1 matrix because it contains one scalar,
    // not a useful matrix structure to expose from comp.
    let c = c.map(|tensor| match tensor {
        Tensor::Matrix(matrix) if matrix.nrows() == 1 && matrix.ncols() == 1 => {
            Tensor::Scalar(matrix[[0, 0]])
        }
        other => other,
    });

    // Output exception 2: represent real scalar results as Typst floats so
    // callers can use them directly in Typst code outside math mode.
    if let Ok(Tensor::Scalar(value)) = &c {
        if value.im == 0.0 {
            return Float::from(value.re).into();
        }
    }

    TypstError::contentize(c, &seq).into()
}

#[typst_func()]
pub fn to_elements<'data>(
    TypedContent(seq): TypedContent<Sequence<'data>>,
) -> Result<TypedArray<TypedArray<Complex<f64>>>, Box<Content<'data>>> {
    let seq = LocatingSequence::from(&seq);
    use chumsky::Parser;
    let arr = match (validate(pratt(), matrix)).parse(&seq).into_result() {
        Ok(arr) => arr,
        Err(e) => {
            let err = e
                .into_iter()
                .map(|e: sertyp::TypstError| e.render(&seq).into())
                .collect::<Vec<_>>();
            return Err(Box::new(sertyp::Sequence::from(err).into()));
        }
    };
    let arr: Result<TypedArray<TypedArray<Complex<f64>>>, _> = arr.map(|arr| {
        arr.rows()
            .into_iter()
            .map(|r| r.iter().map(|v| (*v).into()).collect::<Vec<_>>().into())
            .collect::<Vec<_>>()
            .into()
    });
    match arr {
        Ok(arr) => Ok(arr),
        Err(e) => Err(Box::new(e.render(&seq).into())),
    }
}

#[typst_func()]
fn tensor_variant<'data>(
    TypedContent(TypedSequence(content)): TypedContent<TypedSequence<Tensor>>,
) -> sertyp::String<'data> {
    match content {
        Tensor::Scalar(_) => "scalar".into(),
        Tensor::Matrix(_) => "matrix".into(),
    }
}
