use crate::latex_decoder;
use unidecode::unidecode;

pub fn clean_and_decode(s: &str) -> String {
    let cleaned = clean_string(s);
    return unidecode(&cleaned)
}

pub fn clean_string<'a>(s: &'a str) -> String {
    let decoded = latex_decoder::decode_latex(s);
    return remove_extra_whitespace(&decoded)
}

fn remove_extra_whitespace(s: &str) -> String {
    let split = s.split(char::is_whitespace);
    return split.map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().join(" ");
}

