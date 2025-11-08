ec::solution!(5);
struct Spine {
    center: i64,
    left: Option<i64>,
    right: Option<i64>
}
struct FishBone {
    identifier: i64,
    bones: Vec<Spine>
}

fn parse_fishbone(notes: &str) -> FishBone {
    let (id, list) = notes.split_once(':').unwrap();
    let id = id.parse::<i64>().unwrap();
    let list = list.split(",").map(|x| x.parse::<i64>().unwrap());
    let mut bone_vec: Vec<Spine> = vec![];
    for i in list {
        let mut found_a_place = false;
        for j in bone_vec.iter_mut() {
            if j.center > i && j.left.is_none(){
                j.left = Some(i);
                found_a_place = true; break;
            }
            if j.center < i && j.right.is_none(){
                j.right = Some(i);
                found_a_place = true; break;
            }
        }
        if(!found_a_place) {
            bone_vec.push(Spine{center:i,left:None,right: None});

        }
    };
    FishBone { identifier: id, bones: bone_vec }

}
#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let bone = parse_fishbone(notes);
    let v = bone.bones.iter().map(|x| x.center).fold(String::new(),|mut acc,elem| {acc.push_str(&elem.to_string()); acc} );
    Some(v)
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut bone_list = notes.lines().map(|notes| parse_fishbone(notes));
    let v = bone_list.map(|bone |bone.bones.iter().map(|x| x.center).fold(String::new(),|mut acc,elem| {acc.push_str(&elem.to_string()); acc} )).map(|s| s.parse::<i64>().unwrap());
    let diff = v.clone().min().unwrap().abs_diff(v.max().unwrap());
    
    Some(
        diff.to_string()
    ) 
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let mut bone_list = notes.lines().map(|notes| parse_fishbone(notes));
    let v = bone_list.map(|bone |bone.bones.iter().map(|x| x.center).fold(String::new(),|mut acc,elem| {acc.push_str(&elem.to_string()); acc} )).map(|s| s.parse::<i64>().unwrap());
    let diff = v.clone().min().unwrap().abs_diff(v.max().unwrap());
    
    Some(
        diff.to_string()
    ) 
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
