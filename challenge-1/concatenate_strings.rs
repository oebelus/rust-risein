pub fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result: String = a.to_string();
    result.push_str(b);
    result
}
