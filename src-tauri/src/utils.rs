use regex::Regex;

pub(crate) fn remove_main_function(code: &str) -> String {
    let re = Regex::new(r"(?s)int\s+main\s*\([^)]*\)\s*\{.*?\}").unwrap();

    let result = re.replace_all(code, "");

    result.into_owned()
}
