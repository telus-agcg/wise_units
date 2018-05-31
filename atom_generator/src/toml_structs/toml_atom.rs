use heck::CamelCase;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"^\d+").unwrap();
}

pub(crate) trait TomlAtom {
    fn primary_code(&self) -> String;
    fn names(&self) -> Vec<String>;

    fn type_name(&self) -> String {
        let names = self.names();
        let name = names.first().unwrap().clone();

        let name = sanitize_type_name_segment(name);
        let primary_code = sanitize_primary_code(&self.primary_code());

        finally(&name, &primary_code)
    }
}

fn sanitize_primary_code(string: &str) -> String {
    if string.ends_with("_i]") {
        "International".to_string()
    } else if string.ends_with("_us]") {
        "US".to_string()
    } else if string.ends_with("_br]") {
        "British".to_string()
    } else if string.ends_with("_m]") {
        // All but one name of [*_m] atoms have "metric" in the name already. For the
        // one that doesn't ("mean british thermal unit"), there is no other
        // atom by the same name so there's no need to make the distinction.
        String::new()
    } else if string.ends_with("_av]") {
        "Avoirdupois".to_string()
    } else if string.ends_with("_tr]") {
        "Troy".to_string()
    } else if string.ends_with("_ap]") {
        "Apothecaries".to_string()
    } else {
        match string {
            "'" | "''" => "Angle".to_string(),
            "10*" => "Star".to_string(),
            "10^" => "Caret".to_string(),
            "bit_s" => "LogarithmusDualis".to_string(),
            "L" | "[IU]" => "Secondary".to_string(),
            _ => String::new(),
        }
    }
}

fn sanitize_type_name_segment(mut string: String) -> String {
    if !string.is_ascii() {
        string = remove_non_latin_chars(&string);
    };

    let s = remove_non_letter_chars(&string);

    s.to_camel_case()
}

fn remove_non_latin_chars(string: &str) -> String {
    string
        .replace("è", "e")
        .replace("é", "e")
        .replace("Å", "A")
        .replace("ö", "o")
}

fn remove_non_letter_chars(string: &str) -> String {
    string
        .replace("'", "")
        .replace("*", " Star")
        .replace("^", " Caret")
}

fn finally(name: &str, primary_code: &str) -> String {
    let type_name = format!("{}{}", name, primary_code);

    let mut type_name = NUMBER_REGEX.replace(&type_name, "").into_owned();
    type_name.truncate(64);

    type_name
}
