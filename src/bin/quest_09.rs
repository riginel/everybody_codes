use std::collections::{HashMap, HashSet};

use disjoint::DisjointSet;

ec::solution!(9);
const NULL_SEQ: [u8; 1] = [0];
fn parse_dna_sequences(sequence: &str) -> Vec<&[u8]> {
    let mut map = vec![NULL_SEQ.as_slice()];

    for line in sequence.lines() {
        let (idx, seq) = line.split_once(":").unwrap();
        map.push(seq.as_bytes())
    }
    map
}
fn find_similarities(first: &[u8], second: &[u8]) -> u32 {
    first
        .iter()
        .zip(second.iter())
        .map(|(&f, &s)| (f == s) as u32)
        .sum()
}
fn has_parents(child: usize, parent_1: usize, parent_2: usize, map: &[&[u8]]) -> bool {
    let (child, parent_1, parent_2) = (map[child], map[parent_1], map[parent_2]);
    child
        .iter()
        .zip(parent_1.iter().zip(parent_2.iter()))
        .all(|(c, (p_1, p_2))| c == p_1 || c == p_2)
}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let sequences = parse_dna_sequences(notes);
    let (mut child, mut parent_1, mut parent_2) = (1, 2, 3);
    if has_parents(2, 1, 3, &sequences) {
        (child, parent_1, parent_2) = (2, 1, 3);
    }
    if has_parents(3, 1, 2, &sequences) {
        (child, parent_1, parent_2) = (3, 1, 2);
    }

    let degree_of_similarity = find_similarities(sequences[child], sequences[parent_1])
        * find_similarities(sequences[child], sequences[parent_2]);
    Some(degree_of_similarity.to_string())
}

fn dna_to_index(val: u8) -> usize {
    match val as char {
        'T' => 0,
        'C' => 1,
        'A' => 2,
        'G' => 3,
        _ => unreachable!(),
    }
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let sequences = parse_dna_sequences(notes);
    let v = naif_find_parents(sequences.as_slice());
    println!("{:?}", v);
    Some(
        v.iter()
            .map(|&(child, parent_1, parent_2)| -> u32 {
                find_similarities(sequences[child], sequences[parent_1])
                    * find_similarities(sequences[child], sequences[parent_2])
            })
            .sum::<u32>()
            .to_string(),
    )
}
fn naif_find_parents(seq: &[&[u8]]) -> Vec<(usize, usize, usize)> {
    let mut v = vec![];
    for a in 1..seq.len() - 2 {
        for b in a + 1..seq.len() - 1 {
            for c in b + 1..seq.len() {
                if (has_parents(a, b, c, seq)) {
                    v.push((a, b, c));
                }
                if (has_parents(b, a, c, seq)) {
                    v.push((b, a, c));
                }
                if (has_parents(c, b, a, seq)) {
                    v.push((c, b, a));
                }
            }
        }
    }
    v
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let sequences = parse_dna_sequences(notes);
    let v = naif_find_parents(sequences.as_slice());
    let mut set = DisjointSet::with_len(sequences.len());
    for (child, p_1, p_2) in v {
        set.join(child, p_1);
        set.join(child, p_2);
    }
    set.sets()
        .iter()
        .max_by(|set_1, set_2| set_1.len().cmp(&set_2.len()))
        .map(|max| max.iter().sum::<usize>())
        .map(|max| max.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(9, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(9, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(9, 3));
        assert_eq!(result, None);
    }
}
