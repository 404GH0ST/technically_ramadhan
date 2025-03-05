use std::collections::HashMap;

fn words_to_marks(s: &str) -> u32 {
    let mut sum = 0;
    let charset: HashMap<char, u32> = "abcdefghijklmnopqrstuvwxyz".chars().zip(1..=26).collect();
    for c in s.chars() {
        sum += charset.get(&c).unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
