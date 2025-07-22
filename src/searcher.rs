pub type SearchResult = Option<(usize, usize)>;
pub type SearchResults = Option<Vec<(usize, usize)>>;
pub type ReverseResult = bool;

pub trait Searcher {
    fn search_left(&self, pattern: &str, source: &str) -> SearchResult;

    fn search_right(&self, pattern: &str, source: &str) -> SearchResult;

    fn search_all(&self, pattern: &str, source: &str) -> SearchResults;

    fn reverse(&self, pattern: &str, source: &str) -> ReverseResult;
}
