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
    let mut crate_list = parse_crates(notes);
    crate_list.sort();
    
    let mut prev = crate_list[0];
    let mut smallest_set = prev;
    let mut count = 1;
    for i in crate_list{
        if i > prev{
            smallest_set+=i;
            prev = i;
            count+=1;
            if count == 20 {break;}


        }
        
    }
    Some(smallest_set.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let mut crate_list = parse_crates(notes);
    //println!("{}",crate_list.len());
    crate_list.sort(); crate_list.reverse();

    let mut set_vec: Vec<Set> = vec![Set{prev: crate_list[0],amount: crate_list[0]}];
    for i in crate_list{
        let mut  found_a_spot = false;
        for j  in 0..set_vec.len() {
            if i< set_vec[j].prev {
                set_vec[j].amount+=i;
                set_vec[j].prev = i;
                found_a_spot = true;
                break;
            }
        }
        if found_a_spot {continue};

        set_vec.push(Set{prev:i,amount:i});
        
            
    }
    Some(set_vec.len().to_string())
}

struct Set {
    prev: i64,
    amount: i64
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
