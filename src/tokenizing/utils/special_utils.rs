mod classifier {
    use std::sync::LazyLock;
    use std::collections::{HashMap,HashSet};
    static SPECIAL_CHARS: LazyLock<HashSet<char>> = LazyLock::new(|| HashSet::from([
            '\\', '^', '$', '.', '|', '?', '*', '+', '(', ')', '[', ']', '{', '}', 
    ]));
    static BRACKETS: LazyLock<HashMap<char,char>> = LazyLock::new(|| HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
    ]));
    pub fn is_special(chr: &char) -> bool {
        (*SPECIAL_CHARS).contains(chr)
    }
    pub fn match_bracket(chr: &char) -> Option<char> {
        (*BRACKETS).get(chr).copied()
    }
}