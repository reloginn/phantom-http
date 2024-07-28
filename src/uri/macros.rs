macro_rules! match_start_and_end {
    ($start:expr, $end:expr, $parser:expr) => {
        match ($start, $end) {
            (0, 0) => None,
            (start, 0) if start != 0 => {
                let value = unsafe { $parser.get_unchecked(start..) };
                Some(Self::new(value))
            }
            (start, end) => {
                let value = unsafe { $parser.get_unchecked(start..end) };
                Some(Self::new(value))
            }
        }
    };
}

macro_rules! to_compact {
    ($value:expr) => {{
        let s = unsafe { std::str::from_utf8_unchecked($value.as_ref()) };
        compact_str::CompactString::new(s)
    }};
}

macro_rules! get_unchecked {
    ($parser:expr, $from:expr) => {{
        let value = unsafe { $parser.get_unchecked($from..) };
        Self::new(value)
    }};
    ($parser:expr, $to:expr, $never:ty) => {{
        let value = unsafe { $parser.get_unchecked(..$to) };
        Self::new(value)
    }};
    ($parser:expr, $from:expr, $to:expr) => {{
        let value = unsafe { $parser.get_unchecked($from..$to) };
        Self::new(value)
    }};
}

pub(crate) use get_unchecked;
pub(crate) use match_start_and_end;
pub(crate) use to_compact;
