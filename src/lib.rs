#[allow(unused_imports)]
use js_sys::Promise;
use wasm_bindgen::JsValue;
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

pub fn is_valid_url(url: &str) -> bool {
    // http://www.example.com:80/path/to/myfile.html?key1=value1&key2=value2#SomewhereInTheDocument
    // Url contains:
    // scheme     => http
    // domain     => www.example.com
    // port       => 80
    // path       => /path/to/myfile.html
    // parameters => ?key1=value1&key2=value2
    // anchors    => #SomewhereInTheDocument
    //

    let url = url.trim();
    let mut chars = url.chars();
    let start = &chars.next().expect("Could Not Get the 1st Char");
    let end = &chars.last().expect("Could Not get the last Char");

    // TODO: implement better validation rules

    if !url.is_ascii()
        || !start.is_ascii_alphabetic()
        || end.eq(".".chars().next().as_ref().unwrap())
    {
        return false;
    }

    true
}

// save the text to the clipboard
#[cfg(web_sys_unstable_apis)]
pub fn save_to_clipboard(text: &str) -> Promise {
    if text.is_empty() {
        return Promise::reject(&JsValue::from_str("No Text to Save"));
    }

    let Some(window) = web_sys::window() else {
        return Promise::reject(&JsValue::from_str("Saving to Clipboard Failed!"));
    };

    let Some(clipboard) = window.navigator().clipboard() else {
        return Promise::reject(&JsValue::from_str("Could Not Get Clipboard Object"));
    };

    clipboard.write_text(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use wasm_bindgen::prelude::Closure;

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
    fn test_validate_url() {
        let urls = [
            "example.com",
            "http://awesome.things",
            ".http://example.net",
            "http://example.com.",
            "https://mythiongs.lol",
            "http::/hows.lod",
            "http:///kdsds..ijd",
        ];

        assert_eq!(is_valid_url(&urls[0]), true);
        assert_eq!(is_valid_url(&urls[1]), true);
        assert_eq!(is_valid_url(&urls[2]), false);
        assert_eq!(is_valid_url(&urls[3]), false);
        assert_eq!(is_valid_url(&urls[4]), true);
        assert_eq!(is_valid_url(&urls[5]), false);
        assert_eq!(is_valid_url(&urls[6]), false);
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

    #[test]
    #[ignore = "May not be run on non-wasm32"]
    // #[cfg(web_sys_unstable_apis)]
    fn test_saving_text_to_clipboard() {
        let success_promise = Promise::resolve(&JsValue::from_str("Saving successful"));
        let failure_promise = Promise::reject(&JsValue::from_str("Saving Failed"));

        let success_msg: String = String::default();
        let failure_msg: String = String::default();
        // let mut fail_empty_msg: String = String::default();

        {
            let mut success_msg = success_msg.clone();
            let _ = success_promise.then(&Closure::once(move |msg: JsValue| {
                let _ =
                    &success_msg.push_str(JsValue::as_string(&msg).unwrap_or_default().as_str());
            }));
        }
        {
            let mut failure_msg = failure_msg.clone();
            let _ = failure_promise.then(&Closure::once(move |msg: JsValue| {
                let _ =
                    &failure_msg.push_str(JsValue::as_string(&msg).unwrap_or_default().as_str());
            }));
        }
        // {
        // this test fail because the navigator function is not implemented on non-wasm targets
        // let mut fail_empty_msg = fail_empty_msg.clone();
        // let _ = save_to_clipboard("").then(&Closure::once(move |msg: JsValue| {
        // let _ =
        // &fail_empty_msg.push_str(JsValue::as_string(&msg).unwrap_or_default().as_str());
        // }));
        // }

        assert_eq!(success_msg, "Saving successful");
        assert_eq!(failure_msg, "Saving Failed")
        // assert_eq!(fail_empty_msg, "No Text to Save")
    }
}
