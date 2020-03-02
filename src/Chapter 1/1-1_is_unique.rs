use std::collections::HashSet;

fn check_unique_with_data_structure(word: &str) -> bool {
    let mut chs = HashSet::new();
    for ch in word.chars() {
        if chs.contains(&ch) {
            return true;
        } else {
            chs.insert(ch);
        }
    }
    false
}

fn check_unique_without_data_structure(word: &str) -> bool {
    let mut bit_vec: u128 = 0;
    let mut ch_num: u128;
    for ch in word.to_lowercase().chars() {
        ch_num = ch as u128 - 'a' as u128;
        if bit_vec & (1 << ch_num) > 0 {
            return true;
        }
        bit_vec |= 1 << ch_num;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_data_structure() {
        assert_eq!(check_unique_with_data_structure("yellow"), true);
        assert_eq!(check_unique_with_data_structure("brown"), false);
    }

    #[test]
    fn test_without_data_structure() {
        assert_eq!(check_unique_without_data_structure("purple"), true);
        assert_eq!(check_unique_without_data_structure("blue"), false);
    }
}

fn main() {
    check_unique_with_data_structure("hello");
    check_unique_without_data_structure("hello");
}
