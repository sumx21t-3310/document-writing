use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

const DEFAULT_TOP: usize = 10;

/// Counts how many times each whitespace-separated word appears in `text`.
fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// Sorts by count (descending), then by word (ascending) so the output is stable.
fn rank<'a>(counts: HashMap<&'a str, usize>) -> Vec<(&'a str, usize)> {
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    entries
}

fn usage() -> ! {
    eprintln!("usage: wordfreq <FILE> [--top N]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (path, top) = match args.as_slice() {
        [path] => (path, DEFAULT_TOP),
        [path, flag, n] if flag == "--top" => match n.parse() {
            Ok(n) => (path, n),
            Err(_) => usage(),
        },
        _ => usage(),
    };

    let text = fs::read_to_string(path).unwrap();
    let counts = count_words(&text);

    println!("lines: {}", text.lines().count());
    for (word, n) in rank(counts).iter().take(top) {
        println!("{n:>6} {word}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_repeated_words() {
        let counts = count_words("a b a\nc a b");
        assert_eq!(counts["a"], 3);
        assert_eq!(counts["b"], 2);
        assert_eq!(counts["c"], 1);
    }

    #[test]
    fn empty_text_has_no_words() {
        assert!(count_words("").is_empty());
    }

    #[test]
    fn rank_orders_by_count_then_word() {
        let ranked = rank(count_words("b a b a c"));
        assert_eq!(ranked, vec![("a", 2), ("b", 2), ("c", 1)]);
    }
}
