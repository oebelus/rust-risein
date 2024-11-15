mod concatenate_strings;

fn main() {
    let string1: &str = "Hello, ";
    let string2: &str = "World!";

    let concatenated_string: String = concatenate_strings::concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}
