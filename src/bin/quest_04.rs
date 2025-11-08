ec::solution!(4);
fn parse_gears(s: &str) -> Vec<f64> {
    s.lines().map(|x| x.parse::<f64>().unwrap()).collect()
}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let gears = parse_gears(notes);
    let mut  turns:f64 = 1.0;
    let pairs = gears.iter().zip(gears.iter().skip(1));
    for (first,second) in pairs {
        turns = turns * (first/second);
    }
    let turns:i64 = (turns *2025.0) as i64;
    return Some(turns.to_string())

}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let gears = parse_gears(notes);
    let mut  turns:f64 = 1.0;
    let pairs = gears.iter().zip(gears.iter().skip(1));
    for (first,second) in pairs {
        turns = turns * (first/second);
    }
    let min_turns = (10000000000000.0 / turns).ceil() as i64;
    return Some(min_turns.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let gears = parse_three(notes);
    let mut turns = 1.0;

    let pairs = gears.iter().zip(gears.iter().skip(1));
    for(first,second) in pairs {
        match (first,second) {
            (WheelType::Double(a,b), WheelType::Single(c)) => {
                turns = turns * (b/c);

            },
            (WheelType::Single(a),WheelType::Double(b,c)) => {
                turns = turns * (a/b);
            },
            (WheelType::Double(a,b ), WheelType::Double(c,d)) => {
                turns = turns * (b/c);
            },
            (WheelType::Single(a), WheelType::Single(b)) => {
                turns = turns * (a/b);
            }

            
        }
    }
    let turns =( turns * 100.0 ) as i64;
    return Some(turns.to_string())

    
}
enum WheelType {
    Single(f64),
    Double(f64,f64)
}
fn parse_three(s: &str)-> Vec<WheelType> {

    s.lines().filter(|x| !x.trim().is_empty()).map(|x| { 
        match x.split_once('|') {
        Some((a,b)) => {
           
            WheelType::Double(a.trim().parse::<f64>().unwrap(), b.trim().parse::<f64>().unwrap())},
        None => {
            
            WheelType::Single(x.trim().parse::<f64>().unwrap())}
    } }
    ).collect()
} 
#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(4, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(4, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(4, 3));
        assert_eq!(result, None);
    }
}
