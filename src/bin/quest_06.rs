use std::collections::HashMap;

ec::solution!(6);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut elems_before: HashMap<u8, Vec<u32>> = HashMap::new();
    for i in notes.bytes() {
        if i.is_ascii_uppercase() {
            //println!("Uppercase {}", i);
            elems_before
                .entry(i)
                .and_modify(|x| {
                    x.push(0);
                })
                .or_insert(vec![0]);
        } else {
            elems_before.entry(i.to_ascii_uppercase()).and_modify(|v| {
                v.iter_mut().for_each(|elem| {
                    *elem += 1;
                })
            });
        }
    }
    //println!("{:?}", elems_before);
    /* I thought it meant all pairs, instead it wanted only letter A
    let sum = elems_before
        .values()
        .fold(0 as u32, |acc, elem| acc + elem.iter().sum::<u32>());
    */
    let sum = elems_before
        .get(&b'A')
        .map(|v| v.iter().sum::<u32>().to_string());
    return sum;
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut sum: u32 = 0;
    let mut elems_before: HashMap<u8, u32> = HashMap::new();
    for i in notes.bytes() {
        if i.is_ascii_uppercase() {
            //println!("Uppercase {}", i);
            elems_before
                .entry(i)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        } else {
            elems_before
                .get(&i.to_ascii_uppercase())
                .inspect(|&&num| sum += num);
        }
    }
    /*
    let sum = elems_before
        .values()
        .fold(0 as u32, |acc, elem| acc + elem.iter().sum::<u32>());
    Some(sum.to_string())
    */
    Some(sum.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    /*
    you have a segment s of length l(s)
    segment repeated 1000 times
    novice can be mentored by knights which are at most 1000 tents to left or right

    this time we should do d
     */
    const CYCLES: u32 = 1000;
    const DISTANCE: u32 = 1000;
    let (mut low, mut hi): (u32, u32) = (0, 0);

    let mut sum: u32 = 0;
    let mut count_down = notes.len() * CYCLES as usize;
    let mut elems_before: HashMap<u8, u32> = HashMap::new();
    let (mut low_cycle, hi_cycle) = (notes.bytes().cycle(), notes.bytes().cycle());
    let mut low_char = low_cycle.next();
    for i in hi_cycle {
        if count_down == 0 {
            break;
        }

        if hi - low > DISTANCE {
            //time to remove a count for low char
            low_char.inspect(|&x| {
                elems_before.entry(x).and_modify(|count| *count -= 1);
            });
            low_char = low_cycle.next();
            low += 1;
        }
        elems_before
            .entry(i)
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);
        if i.is_ascii_uppercase() {
            //println!("Uppercase {}", i);
            elems_before
                .get(&i.to_ascii_lowercase())
                .inspect(|&&num| sum += num);
        } else {
            elems_before
                .get(&i.to_ascii_uppercase())
                .inspect(|&&num| sum += num);
        }
        hi += 1;
        count_down -= 1;
    }
    Some(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(6, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(6, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(6, 3));
        assert_eq!(result, None);
    }
}
