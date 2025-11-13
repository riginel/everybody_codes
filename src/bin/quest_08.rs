use std::collections::BTreeMap;

ec::solution!(8);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let v = parse_string_sequence(notes);
    let string_len = *v.iter().max().unwrap();
    Some(count_passes_through_center(v.as_slice(), string_len).to_string())
}

fn count_passes_through_center(arr: &[usize], nails: usize) -> usize {
    let mut count = 0;

    for (&first, &second) in arr.iter().zip(arr.iter().skip(1)) {
        if first.abs_diff(second) == nails / 2 {
            count += 1;
        }
    }
    count
}
fn parse_string_sequence(s: &str) -> Vec<usize> {
    s.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    //This is more complicated, how can we count the knots efficiently?
    // a string  a->b produces a knot with a string that goes from a point c in [a+1,b-1] to a point d in [b+1,a-1]
    //we'll make one btree map and use the range function to find all knots
    let sequence = parse_string_sequence(notes);
    let &nails = sequence.iter().max().unwrap();
    let mut map: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    let mut knot_count: usize = 0;
    for (&first, &second) in sequence.iter().zip(sequence.iter().skip(1)) {
        let (first, second) = (first.min(second), first.max(second));
        insert_thread(first, second, &mut map);
        // println!("first {},second {}", first, second);
        let knots = find_knots(first, second, nails, &map);
        knot_count += knots;
    }
    Some(knot_count.to_string())
}
fn find_knots(
    start: usize,
    end: usize,
    nails: usize,
    map: &BTreeMap<usize, BTreeMap<usize, usize>>,
) -> usize {
    let mut sum = 0;

    let (mut iter_a, mut iter_b) = (map.range(1..start), map.range(start + 1..end));
    for (&c, map) in iter_a {
        let mut iter = map.range(start + 1..end);
        for (_d, count) in iter {
            sum += count;
        }
    }
    for (&c, map) in iter_b {
        let mut iter = map.range(end + 1..nails + 1);
        for (_d, count) in iter {
            sum += count;
        }
    }
    sum
}
fn insert_thread(start: usize, end: usize, map: &mut BTreeMap<usize, BTreeMap<usize, usize>>) {
    map.entry(start)
        .and_modify(|end_points| {
            end_points
                .entry(end)
                .and_modify(|end_point| *end_point += 1)
                .or_insert(1);
        })
        .or_insert(BTreeMap::from([(end, 1)]));
}
fn is_knot(first_pair: (usize, usize), second_pair: (usize, usize)) -> bool {
    let ((a, b), (c, d)) = (first_pair, second_pair);
    return (in_range_wrapping(c, a, b) && in_range_wrapping(d, b, a))
        || (in_range_wrapping(d, a, b) && in_range_wrapping(c, b, a));
}
fn in_range_wrapping(elem: usize, start: usize, end: usize) -> bool {
    if start < end {
        return (elem > start && elem < end);
    }
    return (elem > start || elem < end);
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let sequence = parse_string_sequence(notes);
    let &nails = sequence.iter().max().unwrap();
    let mut map: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    let mut max_so_far: usize = 0;
    for (&first, &second) in sequence.iter().zip(sequence.iter().skip(1)) {
        let (first, second) = (first.min(second), first.max(second));
        insert_thread(first, second, &mut map);
        // println!("first {},second {}", first, second);
    }
    for a in 1..nails - 1 {
        for b in a + 1..nails + 1 {
            max_so_far = usize::max(max_so_far, find_knots(a, b, nails, &map))
        }
    }
    Some((max_so_far).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(8, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(8, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(8, 3));
        assert_eq!(result, None);
    }
}
