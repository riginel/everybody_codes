ec::solution!(3);

fn parse_crates(list: &str) -> Vec<i64> {
    list.split(",").map(|x| x.parse::<i64>().unwrap()).collect()
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut crate_list = parse_crates(notes);
    crate_list.sort();
    crate_list.reverse();
    let mut prev = crate_list[0];
    let mut biggest_set = prev;
    for i in crate_list{
        if i < prev{
            biggest_set+=i;
            prev = i;
        }
        
    }
    Some(biggest_set.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    None
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(3, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(3, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(3, 3));
        assert_eq!(result, None);
    }
}
