use std::ops::{Add, Div, Mul, Sub};

ec::solution!(2);
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
struct Complex {
    real: i64,
    imaginary: i64,
}

impl Complex {
    fn new(real: i64, imaginary: i64) -> Self {
        Self { real, imaginary }
    }
    fn from_arr(arr: &[i64; 2]) -> Self {
        Self {
            real: arr[0],
            imaginary: arr[1],
        }
    }
    fn to_output_string(&self) -> String {
        format!("[{},{}]", self.real, self.imaginary)
    }
}
impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}
impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        // (a +bi) *(c +di) = ac + adi + bic - bd = (ac-bd) +(adi + bic)
        let (a, b, c, d) = (self.real, self.imaginary, rhs.real, rhs.imaginary);
        Complex {
            real: (a * c - b * d),
            imaginary: (a * d + b * c),
        }
    }
}
impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real / rhs.real,
            imaginary: self.imaginary / rhs.imaginary,
        }
    }
}
fn parse_complex_num(to_parse: &str) -> Option<Complex> {
    let (real, imaginary) = to_parse
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .and_then(|s| s.split_once(','))?;
    Some(Complex {
        real: real.parse().ok()?,
        imaginary: imaginary.parse().ok()?,
    })
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (_var_name, number) = notes.split_once("=")?;
    let number = parse_complex_num(number)?;
    let mut result = Complex::default();
    for _i in 0..3 {
        result = result * result;
        result = result / Complex::new(10, 10);
        result = result + number;
    }
    return Some(result.to_output_string());
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (_var_name, number) = notes.split_once("=")?;
    let top_left = parse_complex_num(number)?;
    //println!("top_left: {}",top_left.to_output_string());
    let grid_iter = GridIter::new(top_left, Complex::new(1000, 1000), Complex::new(101, 101));
    let mut count: usize = 0;
    let mut num_total_points = 0;
    for c in grid_iter {
        num_total_points += 1;
        let mut result = Complex::default();
        let mut valid = true;
        for _i in 0..100 {
            result = result * result;
            result = result / Complex::new(100000, 100000);
            result = result + c;

            if !((-1000000..=1000000).contains(&result.real)
                && (-1000000..=1000000).contains(&result.imaginary))
            {
                valid = false;
                //println!("c: {}",c.to_output_string());
                //println!("result:  {}", result.to_output_string());
                break;
            }
        }
        if valid {
            count += 1
        };
    }
    //println!("Total Points: {num_total_points}");

    return Some(count.to_string());
}

struct GridIter {
    finished: bool,
    top_left: Complex,
    size: Complex,
    division: Complex,
    current: Complex,
    offset: Complex,
}
impl GridIter {
    fn new(top_left: Complex, size: Complex, division: Complex) -> Self {
        let offset = size / (division - Complex::new(1, 1));
        Self {
            finished: false,
            top_left,
            size,
            division,
            current: Complex::new(0, 0),
            offset,
        }
    }
    fn next_index(&mut self) -> Complex {
        match (self.current.real, self.current.imaginary) {
            (real, imaginary)
                if real == self.division.real - 1 && imaginary == self.division.imaginary - 1 =>
            {
                self.finished = true;
                self.division
            }
            (real, imaginary) if imaginary == self.division.imaginary - 1 => {
                Complex::new(real + 1, 0)
            }
            (real, imaginary) => Complex::new(real, imaginary + 1),
        }
    }
    fn compute_coords(&self) -> Complex {
        self.top_left
            + Complex::new(
                self.current.real * self.offset.real,
                self.current.imaginary * self.offset.imaginary,
            )
    }
}
impl Iterator for GridIter {
    type Item = Complex;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.finished) {
            return None;
        }
        let coords = self.compute_coords();
        self.current = self.next_index();
        Some(coords)
    }
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (_var_name, number) = notes.split_once("=")?;
    let top_left = parse_complex_num(number)?;
    //println!("top_left: {}",top_left.to_output_string());
    let grid_iter = GridIter::new(top_left, Complex::new(1000 ,1000),Complex::new(1001, 1001));
    let mut count: usize = 0;
    let mut num_total_points = 0;
    for c in grid_iter {
        num_total_points += 1;
        let mut result = Complex::default();
        let mut valid = true;
        for _i in 0..100 {
            result = result * result;
            result = result / Complex::new(100000, 100000);
            result = result + c;

            if !((-1000000..=1000000).contains(&result.real)
                && (-1000000..=1000000).contains(&result.imaginary))
            {
                valid = false;
                //println!("c: {}",c.to_output_string());
                //println!("result:  {}", result.to_output_string());
                break;
            }
        }
        if valid {
            count += 1
        };
    }
    //println!("Total Points: {num_total_points}");

    return Some(count.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(2, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(2, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(2, 3));
        assert_eq!(result, None);
    }
}
