use std::collections::{HashMap, HashSet};

ec::solution!(7);

fn parse_input_one(notes: &str) -> (Vec<&str>, HashMap<u8, HashSet<u8>>) {
    let mut lines = notes.lines();
    let mut map: HashMap<u8, HashSet<u8>> = HashMap::new();
    let names: Vec<&str> = lines.next().unwrap().split(',').collect();
    let _whitespace = lines.next();
    for i in lines {
        let (idx, list) = i.split_once('>').unwrap();
        let (idx, list) = (
            idx.trim().bytes().next().unwrap(),
            list.trim().split(',').map(|c| c.bytes().next().unwrap()),
        );
        map.entry(idx)
            .and_modify(|set| {
                list.clone().for_each(|c| {
                    set.insert(c);
                })
            })
            .or_insert(list.collect());
    }
    (names, map)
}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, map) = parse_input_one(notes);
    let iter = names.iter().zip(
        names
            .iter()
            .map(|&s| s.as_bytes())
            .map(|bytes| bytes.iter().zip(bytes.iter().skip(1))),
    );
    for (&name, mut name_char_iter) in iter {
        if name_char_iter
            .all(|(first, second)| map.get(first).and_then(|set| set.get(second)).is_some())
        {
            return Some(name.to_string());
        }
    }
    None
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, map) = parse_input_one(notes);
    let iter = names.iter().enumerate().zip(
        names
            .iter()
            .map(|&s| s.as_bytes())
            .map(|bytes| bytes.iter().zip(bytes.iter().skip(1))),
    );
    let mut name_sum = 0;
    for ((index, name), mut name_char_iter) in iter {
        if name_char_iter
            .all(|(first, second)| map.get(first).and_then(|set| set.get(second)).is_some())
        {
            name_sum += index + 1
        }
    }
    Some(name_sum.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (names, map) = parse_input_one(notes);
    let names = filter_prefixes(list_of_names_that_fit(names, &map));
    /*println!("{:?}, \n {:?}", names, map);*/
    let sum: usize = names
        .iter()
        .map(|x| x.as_bytes())
        .map(|name| num_of_names(*name.last().unwrap(), name.len(), &map))
        .sum();
    Some(sum.to_string())
}
fn list_of_names_that_fit<'a>(
    names: Vec<&'a str>,
    char_map: &HashMap<u8, HashSet<u8>>,
) -> Vec<&'a str> {
    names
        .iter()
        .zip(
            names
                .iter()
                .map(|&s| s.as_bytes())
                .map(|bytes| bytes.iter().zip(bytes.iter().skip(1))),
        )
        .filter(|(_name, name_char_iter)| {
            name_char_iter.clone().all(|(first, second)| {
                char_map
                    .get(first)
                    .and_then(|set| set.get(second))
                    .is_some()
            })
        })
        .map(|(&name, _)| name)
        .collect()
}
fn num_of_names(last_letter: u8, curr_len: usize, char_map: &HashMap<u8, HashSet<u8>>) -> usize {
    if curr_len == 11 {
        return 1;
    }
    let add_this_one = if curr_len >= 7 { 1 } else { 0 };

    let mut next_sum = 0;
    for i in char_map.get(&last_letter).unwrap().iter() {
        next_sum += num_of_names(*i, curr_len + 1, char_map);
    }
    next_sum + add_this_one
}
//return only the prefixes from the original list
fn filter_prefixes(names: Vec<&str>) -> Vec<&str> {
    let mut filtered_out_set = HashSet::new();
    for &i in names.iter() {
        for &j in names.iter() {
            if i == j {
                continue;
            }
            if j.starts_with(i) {
                filtered_out_set.insert(j);
            }
        }
    }
    return names
        .into_iter()
        .filter(|&name| !filtered_out_set.contains(name))
        .collect();
}
#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(7, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(7, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(7, 3));
        assert_eq!(result, None);
    }
}
