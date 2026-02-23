use super::skeleton::{MapRef, ProgRef};

pub enum AppItemKindMut<'a> {
    Map(&'a mut MapRef),
    Prog(&'a mut ProgRef),
}

pub enum AppItemKind<'a> {
    Map(&'a MapRef),
    Prog(&'a ProgRef),
}

pub trait AppItem {
    const MAP: usize;
    const PROG: usize;

    fn named(name: &'static str) -> Self;
    fn kind_mut(&mut self) -> AppItemKindMut<'_>;
    fn kind(&self) -> AppItemKind<'_>;
}
