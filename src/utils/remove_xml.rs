use regex::Regex;

pub fn remove_xml(string: String) -> crate::Result<String> {
    let re = Regex::new(r"</?[^>]+(>|$)").unwrap();
    Ok(re.replace_all(string.as_str(), "").to_string())
}