#[allow(unused_imports)]
use js_sys::Promise;
#[allow(unused_imports)]
use web_sys::Navigator;

///
/// ```rust
/// use url_defanger::defange_url;
/// use pretty_assertions::assert_eq;
/// ```
/// assert_eq!(defange_url(""),None)
///
// defange url and return it
#[allow(unused)]
pub fn defange_url(url: &str) -> Option<String> {
    if url.is_empty() {
        return None;
    }

    let str_splits: Vec<_> = url.split('.').collect();
    let tmp = str_splits.join("[.]");

    Some(tmp.to_owned())
}

/// Check if a particular character is repeated successively.
pub fn is_char_doubled(str: &str, char: char, succession_len: Option<u32>) -> bool {
    const DEFAULT_SUCCESSION_LENGTH: u32 = 2;

    if str.len() < DEFAULT_SUCCESSION_LENGTH as usize
        || str.len() < succession_len.unwrap_or(DEFAULT_SUCCESSION_LENGTH) as usize
    {
        return false;
    }

    let succession_len: u32 = succession_len.unwrap_or(DEFAULT_SUCCESSION_LENGTH);

    let pattern = std::iter::repeat(char).take(succession_len as usize);

    // str.find(String::from_iter(pattern).as_str()).is_some()
    str.contains(String::from_iter(pattern).as_str())
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_defange_url() {
        assert_eq!(defange_url("url"), Some("url".to_string()));
        assert_eq!(
            defange_url("http://example.com"),
            Some("http://example[.]com".to_string())
        );
        assert_eq!(defange_url("url"), Some("url".to_string()));
        assert_eq!(defange_url(""), None)
    }

    #[test]
    fn test_is_char_doubled() {
        let patterns = [
            "Hello",
            "hhelo",
            "hhhhh",
            "abccdd",
            "sssssssssssssss",
            "aabccnccccc",
            "unique",
            "strindgd",
        ];

        assert_eq!(is_char_doubled(&patterns[0], 'l', None), true);
        assert_eq!(is_char_doubled(&patterns[1], 'h', None), true);
        assert_eq!(is_char_doubled(&patterns[2], 'h', None), true);
        assert_eq!(is_char_doubled(&patterns[3], 'a', None), false);
        assert_eq!(is_char_doubled(&patterns[4], 's', Some(5)), true);
        assert_eq!(is_char_doubled(&patterns[5], 'n', None), false);
        assert_eq!(is_char_doubled(&patterns[6], 'u', None), false);
        assert_eq!(is_char_doubled(&patterns[7], 'g', None), false)
    }
}
