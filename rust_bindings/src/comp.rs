use kalt::parser::{atoms::complex::Complex, pratt::pratt, validate, validator::matrix};
use sertyp::{
    Content, LocatingSequence, Sequence, TypedArray, TypedContent, TypstError, parse, typst_func,
};
use wasm_minimal_protocol::*;
initiate_protocol!();

#[typst_func()]
pub fn comp<'data>(TypedContent(seq): TypedContent<Sequence<'data>>) -> Content<'data> {
    let seq = LocatingSequence::from(&seq);
    let c = parse!(pratt(), &seq);
    TypstError::contentize(c, &seq)
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
