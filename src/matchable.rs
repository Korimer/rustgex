pub trait Matchable {
    fn matchme(self, tomatch: &str) -> Option<usize>;
}