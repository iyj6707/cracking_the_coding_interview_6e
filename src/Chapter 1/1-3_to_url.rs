fn to_url(s: &str, _len: i32) -> String {
    let mut url = String::from("");
    for ch in s.chars() {
        if ch == ' ' {
            url += "%20";
        } else {
            url += ch.to_string().as_str();
        }
    }
    url
}

fn to_url_2(s: &str, _len: i32) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_url() {
        assert_eq!(
            to_url("Mr John Smith", 13),
            String::from("Mr%20John%20Smith")
        );
        assert_eq!(to_url_2("hi im fine", 10), String::from("hi%20im%20fine"));
    }
}

fn main() {
    to_url("Hi Im abc", 9);
    to_url_2("a b c", 5);
}
