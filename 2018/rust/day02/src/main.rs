use std::collections::HashMap;
use std::io::{stdin, BufRead};

use itertools::Itertools;

#[derive(PartialEq)]
struct WordCounts {
    has_pair: bool,
    has_triplet: bool,
}

impl WordCounts {
    fn new(has_pair: bool, has_triplet: bool) -> WordCounts {
        WordCounts {
            has_pair,
            has_triplet,
        }
    }
}

fn score_word(word: &str) -> WordCounts {
    let mut counts = HashMap::new();
    let mut stats = WordCounts::new(false, false);

    for word_char in word.chars() {
        *counts.entry(word_char).or_insert(0) += 1;
    }

    for (_, val) in counts.iter() {
        match val {
            2 => {
                stats.has_pair = true;
            }
            3 => {
                stats.has_triplet = true;
            }
            _ => (),
        };
    }

    stats
}

fn part_one(words: &Vec<String>) -> u32 {
    let mut pair_count = 0;
    let mut triplet_count = 0;

    for score in words.iter().map(|x| score_word(x)) {
        match score {
            WordCounts {
                has_pair: true,
                has_triplet: true,
            } => {
                pair_count += 1;
                triplet_count += 1;
            }
            WordCounts {
                has_pair: true,
                has_triplet: false,
            } => {
                pair_count += 1;
            }
            WordCounts {
                has_pair: false,
                has_triplet: true,
            } => {
                triplet_count += 1;
            }
            _ => (),
        };
    }

    return pair_count * triplet_count;
}

fn differs_by_one_char(word_a: &str, word_b: &str) -> bool {
    if word_a.len() != word_b.len() || word_a == word_b {
        return false;
    }

    word_a
        .chars()
        .zip(word_b.chars())
        .try_fold(0, |count, (a, b)| {
            let mut add = 0;

            if a != b {
                add = 1;

                if count == 1 {
                    return None;
                }
            }

            Some(count + add)
        }).is_some()
}

fn find_matching_chars(word_a: &str, word_b: &str) -> String {
    word_a
        .chars()
        .zip(word_b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _b)| a)
        .collect()
}

fn part_two(words: &Vec<String>) -> Option<String> {
    match words
        .iter()
        .tuple_combinations()
        .skip_while(|(a, b)| !differs_by_one_char(a, b))
        .next()
    {
        Some((a, b)) => Some(find_matching_chars(a, b)),
        None => None,
    }
}

fn main() {
    let words: Vec<String> = stdin().lock().lines().filter_map(|x| x.ok()).collect();

    println!("Part one: {:?}", part_one(&words));
    println!("Part two: {:?}", part_two(&words));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_word() {
        let words = vec![
            ("abcdef", WordCounts::new(false, false)),
            ("bababc", WordCounts::new(true, true)),
            ("abbcde", WordCounts::new(true, false)),
            ("abcccd", WordCounts::new(false, true)),
            ("aabcdd", WordCounts::new(true, false)),
            ("abcdee", WordCounts::new(true, false)),
            ("ababab", WordCounts::new(false, true)),
        ];

        for (word, expected_score) in words.iter() {
            assert!(score_word(*word) == *expected_score);
        }
    }

    #[test]
    fn test_part_one() {
        let words: Vec<String> = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ].iter()
        .map(|x| (*x).into())
        .collect();

        assert!(part_one(&words) == 12);
    }

    #[test]
    fn test_differs_by_one_char() {
        assert!(differs_by_one_char("fguij", "fguik") == true);
        assert!(differs_by_one_char("fghij", "fguij") == true);
        assert!(differs_by_one_char("bguij", "cguij") == true);
        assert!(differs_by_one_char("bguij", "cguija") == false);
        assert!(differs_by_one_char("cguij", "cguija") == false);
        assert!(differs_by_one_char("bguxj", "cguij") == false);
        assert!(differs_by_one_char("bguxj", "bguxj") == false);
    }

    #[test]
    fn test_find_matching_chars() {
        assert!(find_matching_chars("fguij", "fguik") == "fgui");
        assert!(find_matching_chars("fghij", "fguij") == "fgij");
        assert!(find_matching_chars("bguij", "cguij") == "guij");
    }

    #[test]
    fn test_part_two() {
        let words: Vec<String> = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ].iter()
        .map(|x| (*x).into())
        .collect();

        assert!(part_two(&words) == Some("fgij".into()));
    }
}
