use std::cmp::Ordering;
ec::solution!(5);

struct Spine {
    center: i64,
    left: Option<i64>,
    right: Option<i64>,
}
impl Spine {
    fn to_checksum(&self) -> i64 {
        let mut s = String::new();
        if let Some(x) = self.left {
            s.push_str(&x.to_string());
        }
        s.push_str(&self.center.to_string());
        if let Some(x) = self.right {
            s.push_str(&x.to_string());
        }
        s.parse().unwrap()
    }
}
impl PartialEq<Spine> for Spine {
    fn eq(&self, other: &Spine) -> bool {
        let (c_a, c_b) = (self.to_checksum(), other.to_checksum());
        c_a.eq(&c_b)
    }
}
impl PartialOrd<Spine> for Spine {
    fn partial_cmp(&self, other: &Spine) -> Option<std::cmp::Ordering> {
        let (c_a, c_b) = (self.to_checksum(), other.to_checksum());
        c_a.partial_cmp(&c_b)
    }
}
struct FishBone {
    identifier: i64,
    bones: Vec<Spine>,
}
impl PartialEq<FishBone> for FishBone {
    fn eq(&self, other: &FishBone) -> bool {
        self.identifier == other.identifier && self.bones == other.bones
    }
}
impl PartialOrd<FishBone> for FishBone {
    fn partial_cmp(&self, other: &FishBone) -> Option<Ordering> {
        let (quality_a, quality_b) = (self.get_quality(), other.get_quality());
        if !(quality_a == quality_b) {
            return quality_a.partial_cmp(&quality_b);
        }
        let second_comparison = self
            .bones
            .iter()
            .map(|x| x.to_checksum())
            .partial_cmp(other.bones.iter().map(|x| x.to_checksum()));
        if let Some(Ordering::Equal) = second_comparison {
            return self.identifier.partial_cmp(&other.identifier);
        }
        return second_comparison;
    }
}
impl FishBone {
    fn get_quality_string(&self) -> String {
        self.bones
            .iter()
            .map(|x| x.center)
            .fold(String::new(), |mut acc, elem| {
                acc.push_str(&elem.to_string());
                acc
            })
    }
    fn get_quality(&self) -> i64 {
        self.get_quality_string().parse().unwrap()
    }
}
fn parse_fishbone(notes: &str) -> FishBone {
    let (id, list) = notes.split_once(':').unwrap();
    let id = id.parse::<i64>().unwrap();
    let list = list.split(",").map(|x| x.parse::<i64>().unwrap());
    let mut bone_vec: Vec<Spine> = vec![];
    for i in list {
        let mut found_a_place = false;
        for j in bone_vec.iter_mut() {
            if j.center > i && j.left.is_none() {
                j.left = Some(i);
                found_a_place = true;
                break;
            }
            if j.center < i && j.right.is_none() {
                j.right = Some(i);
                found_a_place = true;
                break;
            }
        }
        if (!found_a_place) {
            bone_vec.push(Spine {
                center: i,
                left: None,
                right: None,
            });
        }
    }
    FishBone {
        identifier: id,
        bones: bone_vec,
    }
}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let bone = parse_fishbone(notes);
    let v = bone.get_quality_string();
    Some(v)
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut bone_list = notes.lines().map(|notes| parse_fishbone(notes));
    let v = bone_list.map(|bone| bone.get_quality());
    let diff = v.clone().min().unwrap().abs_diff(v.max().unwrap());

    Some(diff.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let mut bone_list: Vec<FishBone> = notes.lines().map(|notes| parse_fishbone(notes)).collect();
    bone_list.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    bone_list.reverse();
    let ret = bone_list
        .iter()
        .enumerate()
        .map(|(idx, elem)| (idx + 1, elem))
        .fold(0, |acc: i64, (idx, elem)| {
            acc + (idx as i64 * elem.identifier)
        })
        .to_string();
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(5, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(5, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(5, 3));
        assert_eq!(result, None);
    }
}
