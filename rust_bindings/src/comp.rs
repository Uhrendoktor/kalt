use kalt::parser::{
    atoms::{complex::Complex, tensor::Tensor},
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

fn integer_to_float(value: Integer) -> Float {
    match value {
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
    }
}

fn string_sequence<'data>(value: sertyp::String<'data>) -> Sequence<'data> {
    let mut contents = Vec::new();
    let mut text = std::string::String::new();

    for character in value.chars() {
        if character.is_whitespace() {
            if !text.is_empty() {
                contents.push(Content::from(std::mem::take(&mut text)));
            }
            contents.push(sertyp::Space.into());
        } else {
            text.push(character);
        }
    }

    if !text.is_empty() {
        contents.push(Content::from(text));
    }

    Sequence::from(contents)
}

fn scalar_sequence<'data>(value: f64) -> Sequence<'data> {
    let content: Content<'data> = Complex::new(value, 0.0).into();
    Sequence::from(content)
}

#[typst_func()]
pub fn comp<'data>(body: Item<'data>) -> Item<'data> {
    let seq = match body {
        Item::Boolean(Boolean(value)) => scalar_sequence(if value { 1.0 } else { 0.0 }),
        Item::Integer(value) => scalar_sequence(integer_to_float(value).into()),
        Item::Float(Float::f64(value)) => scalar_sequence(value),
        Item::Decimal(value) => scalar_sequence(value.into()),
        Item::String(value) => string_sequence(value),
        Item::Content(content) => Sequence::from(*content),
        Item::Panic(panic) => return panic.into(),
        _ => {
            return Panic::from(sertyp::String::from(
                "comp expects a Boolean, integer, float, decimal, string, or math content",
            ))
            .into();
        }
    };
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
