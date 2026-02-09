use std::net::SocketAddr;

use nom::IResult;

use super::{
    core::Absorb,
    error::{ParseError, ParseErrorKind},
    limit::Limit,
};

impl<'pa> Absorb<'pa> for SocketAddr {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        let original_input = <&[u8]>::clone(&input);
        let (input, tag) = u8::absorb::<()>(input)?;
        match tag {
            4 => nom::combinator::map(<([u8; 4], u16)>::absorb::<()>, |(ip, port)| {
                SocketAddr::new(ip.into(), port)
            })(input),
            6 => nom::combinator::map(<([u8; 16], u16)>::absorb::<()>, |(ip, port)| {
                SocketAddr::new(ip.into(), port)
            })(input),
            tag => {
                let kind = ParseErrorKind::UnknownTag {
                    tag: tag.to_string(),
                    hint: "",
                };
                Err(kind.error(original_input))
            }
        }
    }
}
