use std::ops::{AddAssign, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum State {
    Start,
    Increment,
    Decrement,
    EOF
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Parser<'a> {
    slice: Slice<'a>,
    position: usize,
    state: State
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            slice: Slice {
                data
            },
            position: 0,
            state: State::Start
        }
    }
    pub fn increment(&mut self) {
        if self.position >= self.slice.len() {
            self.state = State::EOF;
            return;
        }
        self.position.add_assign(1);
        self.state = State::Increment
    }
    pub fn decrement(&mut self) {
        self.position.sub_assign(1);
        self.state = State::Decrement
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>
    {
        self.slice.get_unchecked(index)
    }
    pub const fn position(&self) -> usize {
        self.position
    }
    pub const fn state(&self) -> State { self.state }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Slice<'a> {
    data: &'a [u8]
}

impl<'a> Slice<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data
        }
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>
    {
        self.data.get_unchecked(index)
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
    pub const fn len(&self) -> usize {
        self.data.len()
    }
}