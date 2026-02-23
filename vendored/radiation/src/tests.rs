// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use alloc::{boxed::Box, vec::Vec};

use super::{Absorb, AbsorbExt, DynSized, Emit, Limit, ParseError, ParseErrorKind};

#[derive(Debug, PartialEq, Eq, Absorb, Emit)]
struct SomeStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

#[test]
fn trivial_struct() {
    let foo = SomeStruct::absorb_ext(b"\x12\x23\x34\x45\x56\x67\x78").unwrap();
    assert_eq!(foo.chain(vec![]), b"\x12\x23\x34\x45\x56\x67\x78");
    assert_eq!(
        foo,
        SomeStruct {
            a: 0x12,
            b: 0x2334,
            c: 0x45566778,
        }
    );
}

#[derive(Debug, PartialEq, Eq, Absorb, Emit)]
#[tag(u8)]
enum SomeEnum {
    #[tag(1)]
    A {
        one: u8,
        two: u8,
        // specify a function to parse the field with
        #[custom_absorb(absorb)]
        #[custom_emit(emit)]
        three: u16,
    },
    // use `FromStr` implementation to parse the value from `str`
    B(#[as_str] u16),
    C(u32),
}

// custom parser, parse u8 and square the result
fn absorb(input: &[u8]) -> nom::IResult<&[u8], u16, ParseError<&[u8]>> {
    crate::nom::combinator::map(u8::absorb::<()>, |a| a as u16 * a as u16)(input)
}

fn emit<W>(value: &u16, buffer: &mut W)
where
    W: for<'a> Extend<&'a u8>,
{
    let a = (*value as f32).sqrt() as u8;
    a.emit(buffer);
}

#[test]
fn trivial_enum() {
    let a = SomeEnum::absorb_ext(b"\x01\xcc\xdd\x12").unwrap();
    assert_eq!(a.chain(vec![]), b"\x01\xcc\xdd\x12");
    assert_eq!(
        a,
        SomeEnum::A {
            one: 0xcc,
            two: 0xdd,
            three: 0x12 * 0x12,
        }
    );

    let b = SomeEnum::absorb_ext(b"\x02\x00\x00\x00\x0512345").unwrap();
    assert_eq!(b.chain(vec![]), b"\x02\x00\x00\x00\x0512345");
    assert_eq!(b, SomeEnum::B(12345));

    let c = SomeEnum::absorb_ext(b"\x03\x12\x34\xab\xcd").unwrap();
    assert_eq!(c.chain(vec![]), b"\x03\x12\x34\xab\xcd");
    assert_eq!(c, SomeEnum::C(0x1234abcd));

    let err = SomeEnum::absorb_ext(b"\x04").unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::UnknownTag { hint, .. } = &err.kind {
            if *hint == stringify!(SomeEnum) {
                return;
            }
        }
    }
    panic!("unexpected error {}", err);
}

#[derive(Limit)]
#[limit(upper = 36, inner = LimitOne)]
struct LimitBig;

#[derive(Limit)]
#[limit(lower = 16, upper = 24, next = LimitTwo)]
struct LimitOne;

#[derive(Limit)]
#[limit(lower = 0, upper = 8)]
struct LimitTwo;

#[derive(Absorb, Emit, Debug)]
struct Limited {
    small: u16,
    // `DynSized` specifies that the bytes will be prefixed with length
    // makes the size of the whole type known
    #[limit(LimitBig)]
    big: DynSized<LimitedInner>,
}

#[derive(Absorb, Emit, Debug)]
struct LimitedInner {
    // `Vec` is prefixed with length
    one: Vec<u32>,
    // Box<[T]> is not prefixed with length, don't need it,
    // because the size of the whole type is known
    two: Box<[u16]>,
}

#[test]
fn test_limits_fail_0() {
    let limited = Limited {
        small: 321,
        // this is 4 + 6 * 4 + 5 * 2 == 38 bytes long, which is over limit 36
        big: DynSized(LimitedInner {
            one: vec![0x12345; 6],
            two: Box::new([0; 5]),
        }),
    };
    let bytes = limited.chain(vec![]);
    let err = <Limited as AbsorbExt>::absorb_ext(&bytes).unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::Limit(_, hint) = &err.kind {
            if *hint == stringify!(LimitBig) {
                return;
            }
        }
    }
    panic!("wrong error {err}");
}

#[test]
fn test_limits_fail_1() {
    let limited = Limited {
        small: 321,
        big: DynSized(LimitedInner {
            // this is 7 * 4 == 28 bytes long, which is over limit 24
            one: vec![0x12345; 7],
            two: Box::new([0; 1]),
        }),
    };
    let bytes = limited.chain(vec![]);
    let err = <Limited as AbsorbExt>::absorb_ext(&bytes).unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::Limit(_, hint) = &err.kind {
            if *hint == stringify!(LimitOne) {
                return;
            }
        }
    }
    panic!("wrong error {err}");
}

#[test]
fn test_limits_fail_2() {
    let limited = Limited {
        small: 321,
        big: DynSized(LimitedInner {
            // this is 2 * 4 == 8 bytes long, which is under limit 16
            one: vec![0x12345; 2],
            two: Box::new([0; 1]),
        }),
    };
    let bytes = limited.chain(vec![]);
    let err = <Limited as AbsorbExt>::absorb_ext(&bytes).unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::Limit(_, hint) = &err.kind {
            if *hint == stringify!(LimitOne) {
                return;
            }
        }
    }
    panic!("wrong error {err}");
}

#[test]
fn test_limits_fail_3() {
    let limited = Limited {
        small: 321,
        big: DynSized(LimitedInner {
            one: vec![0x12345; 4],
            // this is 5 * 2 == 10 bytes long, which is over limit 8
            two: Box::new([0; 5]),
        }),
    };
    let bytes = limited.chain(vec![]);
    let err = <Limited as AbsorbExt>::absorb_ext(&bytes).unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::Limit(_, hint) = &err.kind {
            if *hint == stringify!(LimitTwo) {
                return;
            }
        }
    }
    panic!("wrong error {err}");
}
