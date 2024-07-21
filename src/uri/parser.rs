use std::ops::AddAssign;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum State {
    Start,
    Increment,
    Eof,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Parser<'a> {
    slice: Slice<'a>,
    position: usize,
    state: State,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            slice: Slice::new(data),
            position: 0,
            state: State::Start,
        }
    }
    pub(crate) fn increment(&mut self) {
        if self.position >= self.slice.len().wrapping_sub(1) {
            self.state = State::Eof;
            return;
        }
        self.position.add_assign(1);
        self.state = State::Increment
    }
    pub(crate) fn get_byte(&self) -> u8 {
        let &byte = unsafe { self.get_unchecked(self.position) };
        byte
    }
    pub(crate) fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.increment()
        }
    }
    pub(crate) unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>,
    {
        self.slice.get_unchecked(index)
    }
    pub(crate) const fn position(&self) -> usize {
        self.position
    }
    pub(crate) const fn state(&self) -> State {
        self.state
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Slice<'a> {
    data: &'a [u8],
}

impl<'a> Slice<'a> {
    pub(crate) const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
    pub(crate) unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>,
    {
        self.data.get_unchecked(index)
    }
    pub(crate) const fn len(&self) -> usize {
        self.data.len()
    }
}
