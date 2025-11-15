use std::{collections::HashSet, fmt::Display};

ec::solution!(10);
type Position = [i64; 2];
struct Board {
    size: usize,
    dragon_pos: Position,
    sheeps_pos: HashSet<Position>,
}
impl Board {
    fn valid_pos(&self, elem: Position) -> bool {
        return elem.iter().all(|x| (0..self.size as i64).contains(x));
    }
}

fn add(first: Position, second: Position) -> Position {
    [first[0] + second[0], first[1] + second[1]]
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const MOVES_REACH: usize = 4;
    let board = parse_board(notes);
    //DFS approach
    let size = dragon_dfs(&board, MOVES_REACH).len();
    Some(size.to_string())
}

fn parse_board(notes: &str) -> Board {
    let mut size = 0;
    let mut dragon_pos: Position = [0, 0];
    let mut sheeps_pos: HashSet<Position> = HashSet::new();
    for (y, row) in notes.lines().map(|line| line.bytes()).enumerate() {
        size = y + 1;
        for (x, elem) in row.enumerate() {
            if elem == b'D' {
                dragon_pos = [x as i64, y as i64];
            }
            if elem == b'S' {
                sheeps_pos.insert([x as i64, y as i64]);
            }
        }
    }
    println!("{}", size);
    return Board {
        size,
        dragon_pos,
        sheeps_pos,
    };
}
fn dragon_dfs(board: &Board, reach: usize) -> HashSet<Position> {
    let mut stack: Vec<(Position, usize)> = Vec::with_capacity(8 * reach);
    //let mut visited: HashSet<Position> = HashSet::new();
    let mut taken_sheep: HashSet<Position> = HashSet::new();
    stack.push((board.dragon_pos, 0));
    while let Some((pos, steps)) = stack.pop() {
        //println!("steps:{steps}");
        if board.sheeps_pos.contains(&pos) {
            taken_sheep.insert(pos);
        }

        if steps == 4 {
            continue;
        }
        let next_steps = [
            [2, 1], //
            [2, -1],
            [-2, 1],
            [-2, -1],
            [1, 2],
            [-1, 2],
            [1, -2],
            [-1, -2],
        ]
        .iter()
        .map(|&x| add(pos, x))
        .filter(|&pos| board.valid_pos(pos))
        .map(|pos| (pos, steps + 1));
        stack.extend(next_steps);
    }
    taken_sheep
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
        let result = part_one(&read_example_file(10, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(10, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(10, 3));
        assert_eq!(result, None);
    }
}
