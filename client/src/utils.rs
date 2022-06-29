pub fn f() {}

pub fn parse(string_to_parse: String) -> String {
    let string_without_slash = string_to_parse.replace("\\", "");

    let string_without_quotes: String = (&string_without_slash[1..string_without_slash.len() - 1]).parse().unwrap();

    return string_without_quotes;
}