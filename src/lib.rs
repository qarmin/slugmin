extern crate deunicode;

use deunicode::deunicode_char;

/// Convert any unicode string to an ascii "slug" (useful for file names/url components)
///
/// The returned "slug" will consist of a-z, 0-9, and '-'. Furthermore, a slug will
/// never contain more than one '-' in a row and will never start or end with '-'.
///
/// ```rust
/// use self::slugmin::slugify;
///
/// assert_eq!(slugify("My Test String!!!1!1"), "my-test-string-1-1");
/// assert_eq!(slugify("test\nit   now!"), "test-it-now");
/// assert_eq!(slugify("  --test_-_cool"), "test-cool");
/// assert_eq!(slugify("Æúű--cool?"), "aeuu-cool");
/// assert_eq!(slugify("You & Me"), "you-me");
/// assert_eq!(slugify("user@example.com"), "user-example-com");
/// ```
pub fn slugify<S: AsRef<str>>(s: S) -> String {
    _slugify(s.as_ref())
}

// avoid unnecessary monomorphizations
fn _slugify(s: &str) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    // Manual lowercasing as Rust to_lowercase() is unicode
                    // aware and therefore much slower
                    slug.push(x - b'A' + b'a');
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(b'-');
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    if string.ends_with('-') {
        string.pop();
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}

/// Convert any unicode string to an ascii "slug" (useful for file names/url components)
/// In opposite to upper implementation, it removes also redundant whitespaces
/// Allows also to not change size of letters
///
/// The returned "slug" will consist of a-z, A-Z, 0-9, '-', ' ', '.', '_'. Furthermore, a slug will
/// never contain more than one '-' in a row and will never start or end with '-'.
///
/// ```rust
/// use slugmin::slugify_normal;
///
/// assert_eq!(slugify_normal("My Test String!!!1!1",false), "my test string-1-1");
/// assert_eq!(slugify_normal("test\nit   now!",false), "test-it now");
/// assert_eq!(slugify_normal("  --test_-_cool",false), "test_-_cool");
/// assert_eq!(slugify_normal("Æúű--cool?",false), "aeuu-cool");
/// assert_eq!(slugify_normal("You & Me",false), "you - me");
/// assert_eq!(slugify_normal("      user@example.com",false), "user-example.com");
/// assert_eq!(slugify_normal("RWR - - - - - - -",true), "RWR");
/// assert_eq!(slugify_normal(".Pliczek",true), ".Pliczek");
/// assert_eq!(slugify_normal("roman .txt",true), "roman .txt");
/// assert_eq!(slugify_normal("roman. txt",true), "roman. txt");
/// assert_eq!(slugify_normal("roman.  txt",true), "roman. txt");
/// ```
pub fn slugify_normal<S: AsRef<str>>(s: S, leave_size : bool) -> String {
    _slugify_normal(s.as_ref(),leave_size)
}

// avoid unnecessary monomorphizations
fn _slugify_normal(s: &str, leave_size : bool) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    let mut empty_space_was = true;
    let mut dot_was_before = false;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    dot_was_before = false;
                    empty_space_was = false;
                    slug.push(x);
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    dot_was_before = false;
                    empty_space_was = false;
                    if leave_size {
                        slug.push(x);
                    } else {
                        // Manual lowercasing as Rust to_lowercase() is unicode
                        // aware and therefore much slower
                        slug.push(x - b'A' + b'a');
                    }
                }
                b' ' | b'_' => {
                    if !empty_space_was {
                        slug.push(x);
                        prev_is_dash = false;
                        dot_was_before = false;
                        empty_space_was = true;
                    }
                }
                b'.' => {
                    if !dot_was_before  {
                        slug.push(x);
                        prev_is_dash = false;
                        dot_was_before = true;
                        empty_space_was = false;
                    }
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(b'-');
                        prev_is_dash = true;
                        dot_was_before = false;
                        empty_space_was = false;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    // Removes from the end `-` and ` `
    loop {
        if string.ends_with('-') {
            string.pop();
            continue;
        }
        if string.ends_with(' ') {
            string.pop();
            continue;
        }
        break;
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}
