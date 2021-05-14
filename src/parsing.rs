// This file is part of CW2
// Copyright (C) 2021  Soni L.
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::convert::TryFrom;

/// Converts the given reason and content to a CW2.
///
/// A CW2 is a string of `[CW (reason)](content)`.
pub fn to_cw2(reason: &str, content: &str) -> String {
    let expected_size = reason.len() + content.len() + "[CW ]".len();
    let mut sb = String::with_capacity(expected_size);
    let mut count: isize = 0;
    let iter = reason.matches(|_| true);
    iter.for_each(|c| {
        match c {
            "\x10" => {
                sb += "\x10\x10";
            },
            "[" => {
                count += 1;
                sb += c;
            },
            "]" => {
                count -= 1;
                if count < 0 {
                    sb += "\x10";
                    count = 0;
                }
                sb += c;
            },
            c => {
                sb += c;
            },
        }
    });
    if count > 0 {
        let size = sb.len() + usize::try_from(count).unwrap();
        let mut sb2 = String::with_capacity(size);
        for c in sb.matches(|_| true).rev() {
            match c {
                "[" if count > 0 => {
                    count -= 1;
                    sb2 += c;
                    sb2 += "\x10";
                },
                c => {
                    sb2 += c;
                },
            }
        }
        sb = String::with_capacity(sb2.len());
        sb.extend(sb2.matches(|_| true).rev());
    }
    format!("[CW {}]{}", sb, content)
}

/// Attempts to convert the given message to a CW2.
///
/// The message is expected to be in the format `[(reason)](content)`. Returns
/// `None` if it isn't, otherwise returns `[CW (reason)](content)`.
pub fn try_to_cw2(message: &str) -> Option<String> {
    parse_cw_helper(message, false).map(|(r, c)| to_cw2(&r, c))
}

/// Parses an incoming message for CW.
///
/// Splits `[CW (reason)](content)` into `(reason)` and `(content)`, or
/// returns `None`.
pub fn try_parse_cw2(message: &str) -> Option<(String, &str)> {
    parse_cw_helper(message, true)
}

/// Parses a message for CW, with a flag to check for `CW` in the reason.
fn parse_cw_helper(message: &str, check_cw: bool) -> Option<(String, &str)> {
    let mut count = 0;
    let mut mquote = false;
    let mut last_size = 0;
    let mut skipped = if check_cw {
        message.starts_with("[CW ").then(|| 4)?
    } else {
        1
    };
    let cw_start = skipped;
    // figure out the last pos we need
    let pos = message.match_indices(|_| true).take_while(|&(_, c)| {
        match c {
            "[" if !mquote => count += 1,
            "]" if !mquote => count -= 1,
            "\x10" if !mquote => mquote = true,
            _ if mquote => {
                mquote = false;
                skipped += 1;
            },
            _ => {},
        }
        last_size = c.len();
        count != 0
    }).last()?.0;
    (count == 0).then(|| {
        // at this point we have `[CW foo]bar` as `[CW foo` and `bar`, but
        // `pos` is only the start of the last `o` in `foo`.
        let start_of_contents = pos + last_size + 1;
        // build the string
        let mut sb = String::with_capacity(pos - skipped);
        let mut mquote = false;
        let iter = message[cw_start..(start_of_contents-1)].matches(|_| true);
        iter.for_each(|c| {
            match c {
                "\x10" if !mquote => mquote = true,
                c => {
                    mquote = false;
                    sb += c;
                },
            }
        });
        let reason = sb;
        let contents = &message[start_of_contents..];
        (reason, contents)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_cw_parsing() {
        let tests = [
            ("[CW FOO] BAR", true, Some(("FOO".into(), " BAR"))),
            ("[CW FOO\x10]] BAR", true, Some(("FOO]".into(), " BAR"))),
            ("[CW FOO\x10[] BAR", true, Some(("FOO[".into(), " BAR"))),
            ("[CW FOO\x10] BAR", true, None),
            ("message", true, None),

            ("[CW FOO] BAR", false, Some(("CW FOO".into(), " BAR"))),
            ("[CW FOO\x10]] BAR", false, Some(("CW FOO]".into(), " BAR"))),
            ("[CW FOO\x10[] BAR", false, Some(("CW FOO[".into(), " BAR"))),
            ("[CW FOO\x10] BAR", false, None),
            ("message", false, None),
        ];
        for test in &tests {
            dbg!(test);
            assert_eq!(parse_cw_helper(test.0, test.1), test.2);
        }
    }

    #[test]
    fn test_correct_cw_building() {
        let tests = [
            ("", "", String::from("[CW ]")),
            ("[", "", "[CW \x10[]".into()),
            ("]", "", "[CW \x10]]".into()),
            ("[[]", "", "[CW [\x10[]]".into()),
            ("[]]", "", "[CW []\x10]]".into()),
        ];
        for test in &tests {
            dbg!(test);
            assert_eq!(to_cw2(test.0, test.1), test.2);
        }
    }
}
