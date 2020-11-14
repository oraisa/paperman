use crate::string_cleaner;

pub fn match_values(value: &json::JsonValue, field_value: &json::JsonValue, clean: bool) -> bool {
    match value {
        json::JsonValue::String(str_val) => {
            match field_value {
                json::JsonValue::String(field_str) => match_string(str_val, &field_str, clean),
                json::JsonValue::Array(field_arr) => field_arr
                    .iter().any(|s| match_string(s.as_str().unwrap(), str_val, clean)),
                _ => false
            }
        },
        _ => false,
    }
}

fn match_string(needle: &str, haystack: &str, clean: bool) -> bool {
    if clean {
        let cleaned_haystack = string_cleaner::clean_and_decode(haystack).to_lowercase();
        let cleaned_needle = string_cleaner::clean_and_decode(needle).to_lowercase();
        return cleaned_haystack.contains(cleaned_needle.as_str())
    } else {
        return haystack.contains(needle)
    }
}
