#![cfg_attr(not(test), no_std)]

//! # RFC9839-rs
//!
//! A rust implementation of RFC9839 to test for problematic Unicode code points


/// Check if the value is either a low or high surrogate
/// these characters should not be encoded as part of a UTF-8 stream.
pub const fn is_unicode_surrotate(c: u32) -> bool {
    matches!(c, 0xd800..=0xdbff | 0xdc00..=0xdfff)
}

pub mod control {
    //! Characters which are part of the ASCII control character range or extended ASCII

    /// Checks for `b'\n'`
    pub const fn is_newline(c: u32) -> bool {
        c == 0xa
    }

    /// Checks for `b'\r'`
    pub const fn is_carriage_return(c: u32) -> bool {
        c == 0xd
    }

    /// Checks for `b'\t'`
    pub const fn is_horizontal_tab(c: u32) -> bool {
        c == 0x9
    }


    /// Checks for either `b'\n'`, `b'\r'` or `b\t`
    pub const fn is_useful_control(c: u32) -> bool {
        is_newline(c)
            || is_carriage_return(c)
            || is_horizontal_tab(c)
    }

    /// Checks if the value falls into the ASCII control character range
    pub const fn is_c0_control(c: u32) -> bool {
        matches!(c, 0x0..=0x1f)
    }

    /// Checks if the value falls into extended ASCII range
    pub const fn is_c1_control(c: u32) -> bool {
        matches!(c, 0x80..=0x9f)
    }

    /// Checks if the value falls into the ASCII control character range
    /// and isn't one of `b'\n'`, `b'\r'` or `b\t`
    pub const fn is_legacy_control(c: u32) -> bool {
        !is_useful_control(c)
            && (is_c0_control(c)
                || is_c1_control(c))
    }
}




/// Checks if the value is outside the range of Unicode code points
pub const fn is_noncharacter(c: u32) -> bool {
    matches!(c,
        0xfdd0..=0xfdef
        | 0xfffe..=0xffff
        | 0x1fffe..=0x1ffff
        | 0x2fffe..=0x2ffff
        | 0x3fffe..=0x3ffff
        | 0x4fffe..=0x4ffff
        | 0x5fffe..=0x5ffff
        | 0x6fffe..=0x6ffff
        | 0x7fffe..=0x7ffff
        | 0x8fffe..=0x8ffff
        | 0x9fffe..=0x9ffff
        | 0xafffe..=0xaffff
        | 0xbfffe..=0xbffff
        | 0xcfffe..=0xcffff
        | 0xdfffe..=0xdffff
        | 0xefffe..=0xeffff
        | 0xffffe..=0xfffff
        | 0x10fffe..=0x10ffff
    )
}

/// Any Unicode code point except high-surrogate and low-surrogate code points.
/// As specified by Unicode 16
pub struct UnicodeScalars {}

impl UnicodeScalars {
    pub const fn contains(c: u32) -> bool {
        !is_unicode_surrotate(c)
    }
}

/// Unicode code points that excludes surrogates, legacy C0 controls, and the
/// noncharacters U+FFFE and U+FFFF. As specified by the XML 1.0 specification.
pub struct XmlCharacters {}

impl XmlCharacters {
    pub const fn contains(c: u32) -> bool {
        !(control::is_c0_control(c)
            && !control::is_useful_control(c))
        && !is_unicode_surrotate(c)
        && !matches!(c, 0xfffe..=0xffff)
    }
}

/// Unicode code points that are not problematic. As specified by RFC9839.
pub struct UnicodeAssignables {}

impl UnicodeAssignables {
    pub const fn contains(c: u32) -> bool {
        c != 0x7f // del
        && !(control::is_c0_control(c)
            && !control::is_useful_control(c))
        && !control::is_c1_control(c)
        && !is_unicode_surrotate(c)
        && !is_noncharacter(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::ops::RangeInclusive;

    #[track_caller]
    fn assert_predicate(p: fn(u32) -> bool, ranges: &[RangeInclusive<u32>]) {
        let mut last = 0;
        for range in ranges {
            for i in last..*range.start() {
                assert!(
                    p(i) == false,
                    "{}: {:x} should not be included but is",
                    core::panic::Location::caller(),
                    i);
            }
            last = *range.end() + 1;
            for u in range.clone() {
                assert!(
                    p(u),
                    "{}: {:x} should be included but isn't",
                    core::panic::Location::caller(),
                    u);
            }
        }
        for i in last..=(char::MAX as u32) {
            assert!(
                p(i) == false,
                "{}: {:x} should not be included but is",
                core::panic::Location::caller(),
                i);
        }
    }

    #[test]
    fn test_scalars() {
        let ranges = [
            0x0..=0xd7ff,
            0xe000..=0x10ffff
        ];
        assert_predicate(UnicodeScalars::contains, &ranges);
    }


    #[test]
    fn test_xml() {
        let ranges = [
            0x9_u32..=0x9,
            0xa..=0xa,
            0xd..=0xd,
            0x20..=0xd7ff,
            0xe000..=0xfffd,
            0x10000..=0x10ffff
        ];
        assert_predicate(XmlCharacters::contains, &ranges);
    }

    #[test]
    fn test_assignable() {
        let ranges = [
            0x9_u32..=0x9,
            0xa..=0xa,
            0xd..=0xd,
            0x20..=0x7e,
            0xa0..=0xd7ff,
            0xe000..=0xfdcf,
            0xfdf0..=0xfffd,
            0x10000..=0x1fffd,
            0x20000..=0x2fffd,
            0x30000..=0x3fffd,
            0x40000..=0x4fffd,
            0x50000..=0x5fffd,
            0x60000..=0x6fffd,
            0x70000..=0x7fffd,
            0x80000..=0x8fffd,
            0x90000..=0x9fffd,
            0xa0000..=0xafffd,
            0xb0000..=0xbfffd,
            0xc0000..=0xcfffd,
            0xd0000..=0xdfffd,
            0xe0000..=0xefffd,
            0xf0000..=0xffffd,
            0x100000..=0x10fffd,
        ];
        assert_predicate(UnicodeAssignables::contains, &ranges);
    }
}
