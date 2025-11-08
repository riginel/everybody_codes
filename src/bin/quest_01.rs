ec::solution!(1);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut lines = notes.lines();
let (name_row,_space,instruction_row) = (lines.next()?,lines.next()?,lines.next()?);
let (name_list, instruction_list) : (Vec<&str>,_)= (name_row.split(',').collect(),instruction_row.split(','));
let mut idx = 0;
for i in instruction_list {
    idx = move_idx(idx, i, name_list.len());
}
name_list.get(idx).map(|s| s.to_string())
}

fn move_idx(curr_idx:usize, next_instruction: &str, len:usize)-> usize{
    if next_instruction.len()<2 {
        return curr_idx
    } 
    let (instruction,how_much) = next_instruction.split_at(1);
    let how_much:usize = how_much.parse().unwrap();
    match instruction {
        "R" => (curr_idx+ how_much).min(len-1),
        "L" => curr_idx.saturating_sub(how_much),
        _ => curr_idx
    }
    
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut lines = notes.lines();
    let (name_row,_space,instruction_row) = (lines.next()?,lines.next()?,lines.next()?);
    let (mut name_list, instruction_list) : (Vec<&str>,_)= (name_row.split(',').collect(),instruction_row.split(','));
    let mut idx = 0;
    for i in instruction_list {
        idx = move_idx_circular(idx, i, name_list.len());
        
    }
    name_list.get(idx).map(|s| s.to_string())
}

fn move_idx_circular(curr_idx:usize,next_instruction: &str,len:usize) ->usize {
    if next_instruction.len()<2 {
        return curr_idx
    } 
    let (instruction,how_much) = next_instruction.split_at(1);
    let how_much:usize = how_much.parse().unwrap();
    match instruction {
        "R" => (curr_idx + how_much )% len,
        "L" => match (curr_idx as i32 - how_much as i32) % len as i32 {
            x if x< 0 => len- (x.abs() as usize),
            x => x as usize            
        },
        _ => curr_idx
    }
}
#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let mut lines = notes.lines();
    let (name_row,_space,instruction_row) = (lines.next()?,lines.next()?,lines.next()?);
    let (mut name_list, instruction_list) : (Vec<&str>,_)= (name_row.split(',').collect(),instruction_row.split(','));
    let idx = 0;
    for i in instruction_list {
        let swap_idx = move_idx_circular_swap(idx, i, name_list.len());
        name_list.swap(0,swap_idx);

    }
    name_list.get(idx).map(|s| s.to_string())
}
fn move_idx_circular_swap(curr_idx:usize,next_instruction: &str,len:usize) ->usize {
    if next_instruction.len()<2 {
        return curr_idx
    } 
    let (instruction,how_much) = next_instruction.split_at(1);
    let how_much:usize = how_much.parse().unwrap();
    match instruction {
        "R" => (curr_idx + how_much )% len,
        "L" => match (curr_idx as i32 - how_much as i32) % len as i32 {
            x if x< 0 => len- (x.abs() as usize),
            x => x as usize            
        },
        _ => curr_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(1, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(1, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(1, 3));
        assert_eq!(result, None);
    }
}
