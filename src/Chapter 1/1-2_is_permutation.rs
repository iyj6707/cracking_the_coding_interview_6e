use std::collections::HashMap;

fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_letters = HashMap::new();
    let mut s2_letters = HashMap::new();
    if s1.len() == s2.len() {
        for idx in 0..s1.len() {
            let counter1 = s1_letters.entry(s1.chars().nth(idx).unwrap()).or_insert(0);
            let counter2 = s2_letters.entry(s2.chars().nth(idx).unwrap()).or_insert(0);
            *counter1 += 1;
            *counter2 += 1;
        }
    } else {
        return false;
    }

    if s1_letters != s2_letters {
        return false;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_permutation() {
        assert_eq!(is_permutation("abcd", "acbd"), true);
        assert_eq!(is_permutation("abcde", "aebcc"), false);
        assert_eq!(is_permutation("abbcde", "bcbdea"), true);
        assert_eq!(is_permutation("abcde", "abcd"), false);
    }
}

fn main() {
    is_permutation("abcd", "dcba");
}
