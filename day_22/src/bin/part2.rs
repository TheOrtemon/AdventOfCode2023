use std::collections::{HashMap, HashSet};

use nom::{
    IResult, 
    sequence::{tuple, preceded}, 
    character::complete::u64 as uint, 
    bytes::complete::tag
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize
}

#[derive(Debug, Clone)]
struct BrickInfo {
    leans_on: HashSet<Brick>,
    supports: Vec<Brick>
}

fn parse_brick_end(input: &str) -> IResult<&str, [usize; 3]> {
    let (input, (x, y, z)) = tuple((uint, preceded(tag(","), uint), preceded(tag(","), uint)))(input)?;

    Ok((input, [x as usize, y as usize, z as usize]))
}

fn parse_line(line: &str) -> IResult<&str, Brick> {
    let (input, (p1, p2)) = tuple((parse_brick_end, preceded(tag("~"), parse_brick_end)))(line)?;
    let ([x1, y1, z1], [x2, y2, z2]) = (p1, p2);

    Ok((input, Brick { x1, y1, z1, x2, y2, z2, }))
}

fn drop_bricks(input: &str) -> usize {
    let mut bricks = input.split_whitespace()
        .map(|line| parse_line(line).unwrap().1)
        .collect::<Vec<_>>();
    bricks.sort_by(|a, b| a.z1.min(a.z2).cmp(&b.z1.min(b.z2)));
    
    let mut dropped_points: HashMap<[usize; 3], &Brick> = HashMap::new();
    let mut bricks_info: HashMap<Brick, BrickInfo> = HashMap::new();
    for brick in bricks.iter_mut() {
        let Brick { x1, y1, z1, x2, y2, z2 } = &brick;
        let mut leans_on = vec![];
        let mut height_drop = 1;
        let mut dropped = false;
        while *z1.min(z2) - height_drop > 0 {
            for x in *x1.min(x2)..=*x1.max(x2) {
                for y in *y1.min(y2)..=*y1.max(y2) {
                    for z in *z1.min(z2) - height_drop..=*z2.max(z2) - height_drop {
                        if let Some(adjacent) = dropped_points.get(&[x, y, z]) {
                            dropped = true;
                            if let Some(last) = leans_on.last_mut() {
                                if last == *adjacent {
                                    continue;
                                }
                            } 
                            leans_on.push(**adjacent);
                        }
                    }
                }
            }
            if dropped {
                break;
            } else {
                height_drop += 1;
            }
        }
        let new_heigh_drop = height_drop - 1;
        brick.z1 -= new_heigh_drop;
        brick.z2 -= new_heigh_drop;
        let brick_info = BrickInfo { leans_on: HashSet::from_iter(leans_on.clone()) , supports: vec![]};
        bricks_info.insert(*brick, brick_info);
        for leaned in leans_on.into_iter() {
            bricks_info.get_mut(&leaned).unwrap().supports.push(*brick);
        }
        let Brick { x1, y1, z1, x2, y2, z2 } = &brick;
        for x in *x1.min(x2)..=*x1.max(x2) {
            for y in *y1.min(y2)..=*y1.max(y2) {
                for z in *z1.min(z2)..=*z2.max(z2) { 
                    dropped_points.insert([x, y, z], brick);
                }
            }
        }
    }

    bricks.iter().map(|brick| {
        let mut parent_set = bricks_info.get(brick).unwrap().supports.iter().cloned()
            .filter(|child| bricks_info.get(child).unwrap().leans_on.len() <= 1)
            .collect::<HashSet<_>>();

        let mut result_set = HashSet::new();
        while !parent_set.is_subset(&result_set) {
            let mut child_set = HashSet::new();
            for dropped in parent_set.iter() {
                result_set.insert(*dropped);
                for dropped_child in bricks_info.get(dropped).unwrap().supports.iter() {
                    if bricks_info.get(dropped_child).unwrap().leans_on.is_subset(&result_set) {
                        child_set.insert(*dropped_child);
                    }
                }
            }
            parent_set = child_set; 
        }
        result_set.len()
    }).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res  = drop_bricks(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(drop_bricks(test_input), 7);
    }
}

