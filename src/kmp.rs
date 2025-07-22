use crate::searcher::{ReverseResult, SearchResult, SearchResults, Searcher};

#[derive(Debug, Default)]
pub struct KnuthMorrisPratt {}

enum Side {
    Left,
    Right,
}

impl KnuthMorrisPratt {
    fn search_core(&self, pattern: &str, source: &str, side: Side) -> SearchResult {
        let v = pre_calc(pattern, source)?;

        // Copy-paste
        match side {
            Side::Left => v[pattern.len()..]
                .iter()
                .enumerate()
                .find(|(_, el)| **el == pattern.len())
                .map(|(pos, _)| (pos - pattern.len(), pos)),
            Side::Right => v[pattern.len()..]
                .iter()
                .enumerate()
                .rev()
                .find(|(_, el)| **el == pattern.len())
                .map(|(pos, _)| (pos - pattern.len(), pos)),
        }
    }
}

impl Searcher for KnuthMorrisPratt {
    fn search_left(&self, pattern: &str, source: &str) -> SearchResult {
        self.search_core(pattern, source, Side::Left)
    }

    fn search_right(&self, pattern: &str, source: &str) -> SearchResult {
        self.search_core(pattern, source, Side::Right)
    }

    fn search_all(&self, pattern: &str, source: &str) -> SearchResults {
        let prefix = pre_calc(pattern, source)?;
        let chars: Vec<char> = pattern.chars().collect();
        let mut result = Vec::new();

        prefix[chars.len()..]
            .iter()
            .enumerate()
            .filter(|(_, val)| **val == chars.len())
            .for_each(|(ind, _)| result.push((ind - chars.len(), ind)));

        Some(result)
    }

    fn reverse(&self, pattern: &str, source: &str) -> ReverseResult {
        match pre_calc(pattern, source) {
            Some(prefix) => !prefix.contains(&pattern.len()),
            None => false,
        }
    }
}

fn pre_calc(pattern: &str, source: &str) -> Option<Vec<usize>> {
    let form = format!("{pattern}🤡{source}");
    prefix(&form)
}

fn prefix(source: &str) -> Option<Vec<usize>> {
    if source.len() <= 1 {
        return None;
    }

    let chars = source.chars().collect::<Vec<char>>();

    println!("Chars = {:?}", chars);

    let mut result = vec![0; chars.len()];

    for i in 1..chars.len() {
        let mut k = result[i - 1];

        while k > 0 && chars[k] != chars[i] {
            k = result[k as usize - 1];
        }

        if chars[k] == chars[i] {
            k += 1;
        }

        result[i] = k
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_prefix() {
        let a = prefix("");
        assert!(a.is_none());
    }

    #[test]
    fn one_char_prefix() {
        let a = prefix("a");
        assert!(a.is_none());
    }

    #[test]
    fn simple_prefix0() {
        let result = prefix("abbab");

        assert_eq!(result.unwrap(), [0, 0, 0, 1, 2]);
    }

    #[test]
    fn simple_prefix1() {
        let result = prefix("abbababb");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), [0, 0, 0, 1, 2, 1, 2, 3]);
    }

    #[test]
    fn left_aba_in_abacaba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_left("aba", "abacaba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), (0, 3));
    }

    #[test]
    fn left_aba_in_aba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_left("aba", "aba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), (0, 3));
    }

    #[test]
    fn right_aba_in_abacaba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_right("aba", "abacaba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), (4, 7));
    }

    #[test]
    fn right_aba_in_aba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_right("aba", "aba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), (0, 3));
    }

    #[test]
    fn all_aba_in_abacaba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_all("aba", "abacaba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), [(0, 3), (4, 7)]);
    }

    #[test]
    fn all_aba_in_aba() {
        let kmp = KnuthMorrisPratt::default();
        let result = kmp.search_all("aba", "aba");

        assert!(result.is_some());
        assert_eq!(result.unwrap(), [(0, 3)]);
    }
}
