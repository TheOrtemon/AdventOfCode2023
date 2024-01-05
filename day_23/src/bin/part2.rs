use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn coords(&self) -> [isize; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Left => [-1, 0],
            Direction::Down => [0, 1],
            Direction::Right => [1, 0],
        }
    }
}

type Node = [usize; 2];
type MeasuredNode = (Node, usize);
type NodeNeighbours = [Option<MeasuredNode>; 4];

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];


fn traverse_map(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut neigbour_map: HashMap<Node, NodeNeighbours> = HashMap::new();
    let (x_len, y_len) = (map[0].len(), map.len());
    
    for x in 0..x_len {
        for y in 0..y_len {
            let c = map[y][x];
            if c != '#' {
                let node = [x, y];
                let neighbourds: NodeNeighbours = DIRECTIONS.map(|dir|{
                    let [dx, dy] = dir.coords();
                    let [xi, yi] = [x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)];
                    if let Some(cur) = map.get(yi).and_then(|row| row.get(xi)) {
                        if *cur != '#' {
                            Some(([xi, yi], 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                neigbour_map.insert(node, neighbourds);
            }
        }
    }
    neigbour_map.clone().into_keys().for_each(|node| {
        let neighbours = *neigbour_map.get(&node).unwrap();
        let real_neighbours = neighbours.into_iter()
            .flatten()
            .collect::<Vec<MeasuredNode>>();
        if real_neighbours.len() == 2 {
            let [(left, left_dist), (right, right_dist)] = real_neighbours[..2] else {unreachable!()};
            let l_neigh = neigbour_map.remove(&left).unwrap();
            let r_neigh = neigbour_map.remove(&right).unwrap();
            let l_neigh = l_neigh.map(|op_neigh| {
                match op_neigh {
                    Some((neigh, neight_dist)) => {
                        match neigh == node {
                            true => Some((right, right_dist + left_dist)),
                            false => Some((neigh, neight_dist)),
                        }
                    }
                    None => None,
                }
            });
            let r_neigh = r_neigh.map(|op_neigh| {
                match op_neigh {
                    Some((neigh, neight_dist)) => {
                        match neigh == node {
                            true => Some((left, left_dist + right_dist)),
                            false => Some((neigh, neight_dist)),
                        }
                    }
                    None => None,
                }
            });
            neigbour_map.insert(left, l_neigh);
            neigbour_map.insert(right, r_neigh);
        }
    });

    let [start, finish] = [[1, 0], [x_len - 2, y_len - 1]];

    let mut paths = Vec::from([(start, HashSet::new(), 0)]);
    let mut res = 0;

    while let Some((node, trajectory, len)) = paths.pop() {
        if node == finish {
            res = res.max(len);
            continue;
        }
        for (neigh, neigh_dist) in neigbour_map.get(&node).unwrap().iter().flatten() {
            let mut new_trajectory = trajectory.clone();
            let viable = new_trajectory.insert(neigh);
            if viable  {
                let new_len = len + neigh_dist;
                paths.push((*neigh, new_trajectory, new_len));
            }
        }
    }
    res
}

fn main() {
    let input = include_str!("./input.txt");
    let res = traverse_map(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(traverse_map(test_input), 154);
    }
}

