use super::algorithm::Algorithm;

pub struct Hamming {}

impl Hamming {
    fn calculate(&self, s1: &str, s2: &str) -> usize {
        let mut result = 0;
        for (s_char, t_char) in s1.chars().zip(s2.chars()) {
            if s_char != t_char {
                result += 1
            }
        }
        let s_len = s1.chars().count();
        let t_len = s2.chars().count();
        result + s_len.abs_diff(t_len)
    }
}

impl Algorithm for Hamming {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        self.calculate(s1, s2)
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.chars().count().max(s2.chars().count())
    }
}

const DEFAULT: Hamming = Hamming {};

pub fn hamming(s1: &str, s2: &str) -> usize {
    DEFAULT.calculate(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function() {
        let f = hamming;
        assert_eq!(f("sitting", "sitting"), 0);
        assert_eq!(f("abcdefg", "hijklmn"), 7);
        assert_eq!(f("karolin", "kathrin"), 3);
        assert_eq!(f("hello", "world"), 4);
        assert_eq!(f("Rust", "rust"), 1);
        assert_eq!(f("hi mark", "hi markus"), 2);
    }

    #[test]
    fn default_struct() {
        assert_eq!(DEFAULT.distance("Rust", "rust"), 1);
        assert_eq!(DEFAULT.similarity("Rust", "rust"), 3);
        assert_eq!(DEFAULT.maximum("Rust", "rust"), 4);
    }
}
