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
        let v = pre_calc(pattern, source)?;

        let mut result = Vec::new();

        v[pattern.len()..]
            .iter()
            .enumerate()
            .filter(|(_, val)| **val == pattern.len())
            .for_each(|(ind, _)| result.push((ind - pattern.len(), ind)));
        
        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }

    fn reverse(&self, pattern: &str, source: &str) -> ReverseResult {

        match pre_calc(pattern, source) {
            Some(prefix) => {
                for val in prefix {
                    if val == pattern.len() {
                        return false;
                    }
                }

                true
            },
            None => false 
        }


        /*let prefix = pre_calc(pattern, source)?;

        for val in prefix {
            if val == pattern.len() {
                return false;
            }
        }

        true*/
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

    let mut result = vec![0; source.len()];
    result[0] = 0;

    for i in 1..source.len() {
        let mut k = result[i - 1];

        while k > 0 && source.chars().nth(k as usize) != source.chars().nth(i) {
            k = result[k as usize - 1];
        }

        if source.chars().nth(k as usize) == source.chars().nth(i) {
            k += 1;
        }

        result[i] = k
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod prefix_function {
        use super::*;

        #[test]
        fn prefix0() {
            let a = prefix("");
            assert!(a.is_none());
        }

        #[test]
        fn prefix1() {
            let a = prefix("a");
            assert!(a.is_none());
        }

        #[test]
        fn prefix2() {
            let result = prefix("abbab");

            assert_eq!(result.unwrap(), [0, 0, 0, 1, 2]);
        }

        #[test]
        fn prefix3() {
            let result = prefix("abbababb");

            assert_eq!(result.unwrap(), [0, 0, 0, 1, 2, 1, 2, 3]);
        }
    }

    mod kmp_test_left {
        use super::*;

        #[test]
        fn test1() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_left("aba", "abacaba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (0, 3));
        }

        #[test]
        fn test2() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_left("aba", "aba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (0, 3));
        }

        #[test]
        #[ignore = "UTF-8 not working.."]
        fn test3() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_left("русский", "Ого, русский язык");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (5, 12));
        }
    }

    mod kmp_test_right {
        use super::*;

        #[test]
        fn test1() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_right("aba", "abacaba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (4, 7));
        }

        #[test]
        fn test2() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_right("aba", "aba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (0, 3));
        }

        #[test]
        #[ignore = "UTF-8 not working.."]
        fn test3() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_right("русский", "Ого, русский язык");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), (5, 12));
        }
    }

    mod kmp_test_search_all {

        use super::*;

        #[test]
        fn test1() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_all("aba", "abacaba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), [(0, 3), (4, 7)]);
        }

        #[test]
        fn test2() {
            let kmp = KnuthMorrisPratt::default();
            let result = kmp.search_all("aba", "aba");

            assert!(result.is_some());
            assert_eq!(result.unwrap(), [(0, 3)]);
        }
    }
}
