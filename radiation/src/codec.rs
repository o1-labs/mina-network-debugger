use core::marker::PhantomData;
use std::io;

use tokio_util::codec::{Encoder, Decoder};
use bytes::BytesMut;

use super::{Absorb, ParseError, Emit, RadiationBuffer};

impl RadiationBuffer for BytesMut {
    fn pos(&self) -> usize {
        self.len()
    }

    fn write_at(&mut self, pos: usize, data: &[u8]) {
        self.as_mut()[pos..(pos + data.len())].clone_from_slice(data);
    }
}

#[derive(Clone)]
pub struct Codec<T>(PhantomData<T>);

impl<T> Default for Codec<T> {
    fn default() -> Self {
        Codec(PhantomData)
    }
}

impl<T> Decoder for Codec<T>
where
    T: for<'pa> Absorb<'pa>,
{
    type Error = io::Error;

    type Item = T;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let pos = src.as_ptr() as usize;
        match T::absorb::<()>(&*src).map_err(|err| err.map(ParseError::into_vec)) {
            Ok((remaining, v)) => {
                let len = remaining.as_ptr() as usize - pos;
                let _ = src.split_to(len);
                Ok(Some(v))
            }
            Err(nom::Err::Incomplete(_)) => Ok(None),
            Err(nom::Err::Error(err)) if err.kind.is_eof() => Ok(None),
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }
}

impl<T> Encoder<T> for Codec<T>
where
    T: Emit<BytesMut>,
{
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.emit(dst);

        Ok(())
    }
}
