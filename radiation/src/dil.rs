use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};
use pqcrypto_traits::sign::{PublicKey, SecretKey};

use crate::{Absorb, Emit, Limit, ParseError};

macro_rules! impl_pk {
    ($t:ty, $f:expr) => {
        impl<'a> Absorb<'a> for $t {
            fn absorb<L>(input: &'a [u8]) -> nom::IResult<&'a [u8], Self, ParseError<&'a [u8]>>
            where
                L: Limit,
            {
                nom::combinator::map_res(<[u8; $f]>::absorb::<L>, |x| <$t>::from_bytes(&x))(input)
            }
        }

        impl<W> Emit<W> for $t
        where
            W: for<'a> Extend<&'a u8>,
        {
            fn emit(&self, buffer: &mut W) {
                buffer.extend(self.as_bytes());
            }
        }
    };
}

impl_pk!(dilithium2::PublicKey, dilithium2::public_key_bytes());

impl_pk!(dilithium2::SecretKey, dilithium2::secret_key_bytes());

impl_pk!(dilithium3::PublicKey, dilithium3::public_key_bytes());

impl_pk!(dilithium3::SecretKey, dilithium3::secret_key_bytes());

impl_pk!(dilithium5::PublicKey, dilithium5::public_key_bytes());

impl_pk!(dilithium5::SecretKey, dilithium5::secret_key_bytes());
