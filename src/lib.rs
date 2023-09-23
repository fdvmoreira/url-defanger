///
/// ```rust
/// use url_defanger::defange_url;
/// use pretty_assertions::assert_eq;
/// assert_eq!(defange_url(""),None)
/// ```
///
// defange url and return it
#[allow(unused)]
pub fn defange_url(url: &str) -> Option<String>{

    if url.is_empty() {
        return None;
    }

    let str_splits:Vec<_> = url.split('.').collect();
    let tmp = str_splits.join("[.]");

    Some(tmp.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_defange_url() {
        assert_eq!(defange_url("url"), Some("url".to_string()));
        assert_eq!(defange_url("http://example.com"), Some("http://example[.]com".to_string()));
        assert_eq!(defange_url("url"), Some("url".to_string()));
        assert_eq!(defange_url(""), None)
    }

}
