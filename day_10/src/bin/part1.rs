#[derive(Debug)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground
}

impl Pipe {
    fn from(pipe: char) -> Pipe {
        match pipe {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,      
            '7' => Pipe::SW,      
            'F' => Pipe::SE,     
            'S' => Pipe::Start,     
            '.' => Pipe::Ground,
            _ => unreachable!()
        }
    }

    fn is_pluggable(&self, other: &Pipe, x: isize, y: isize) -> bool {
        match (x, y) {
            (0, -1) => matches!(self, Pipe::Start | Pipe::Vertical | Pipe::NE | Pipe::NW) && 
                matches!(other, Pipe::Vertical | Pipe::SE | Pipe::SW),
            (0, 1) => matches!(self, Pipe::Start | Pipe::Vertical | Pipe::SE | Pipe::SW) &&
                matches!(other, Pipe::Vertical | Pipe::NE | Pipe::NW),
            (-1, 0) => matches!(self, Pipe::Start | Pipe::Horizontal | Pipe::NW | Pipe::SW) && 
                matches!(other, Pipe::Horizontal | Pipe::NE | Pipe::SE),
            (1, 0) => matches!(self, Pipe::Start | Pipe::Horizontal | Pipe::NE | Pipe::SE) &&
                matches!(other, Pipe::Horizontal | Pipe::NW | Pipe::SW),
            (_, _) => unreachable!()
        }
    }
}

fn get_pluggable_neighbours(
    pipe_table: &[Vec<Pipe>], 
    visited_table: &[Vec<bool>], 
    cur_point: [usize; 2], 
) -> Vec<[usize; 2]> {
    let directions: [[isize; 2]; 4] = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1]
    ];
    let [cur_x , cur_y] = cur_point;
    directions.iter()
        .filter_map(|[dx, dy]| {
            let cur = &pipe_table[cur_y][cur_x];
            let (neighbour_x, neighbour_y) = (
                cur_x.wrapping_add_signed(*dx), 
                cur_y.wrapping_add_signed(*dy)
            );
            if !(0..pipe_table.len()).contains(&neighbour_y) || !(0..pipe_table[0].len()).contains(&neighbour_x) {
                return None
            }
            let neighbour = &pipe_table[neighbour_y][neighbour_x];
            if cur.is_pluggable(neighbour, *dx, *dy) && !visited_table[neighbour_y][neighbour_x] {
                Some([neighbour_x, neighbour_y])
            } else {
                None
            }
        }).collect::<Vec<[usize; 2]>>()
}

fn extrapolate(s: &str) -> usize {
    let pipe_table: Vec<Vec<Pipe>> = s.lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect();
    let mut visited_table: Vec<Vec<bool>> = vec![vec![false; pipe_table[0].len()]; pipe_table.len()];
    let start: [usize; 2] = pipe_table.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter()
            .position(|pipe| matches!(pipe, Pipe::Start))
            .map(|x| [x, y]))
        .unwrap();
    let [start_x, start_y] = start;
    visited_table[start_y][start_x] = true;
    let mut paths = [start; 2];
    let mut counter = 0;
    loop {
        let neighbour_a = get_pluggable_neighbours(&pipe_table, &visited_table, paths[0]);
        let neighbour_b = get_pluggable_neighbours(&pipe_table, &visited_table, paths[1]);
        let neighbours = [neighbour_a.into_iter().next(), neighbour_b.into_iter().last()];
        for (path, neighbour) in paths.iter_mut().zip(neighbours) {
            if let Some(neighbour_point) = neighbour {
                let [x, y] = neighbour_point;
                visited_table[y][x] = true;
                *path = neighbour_point;
            } else {
                return counter
            }
        }
        counter += 1;
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let res = extrapolate(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(extrapolate(test_input), 4);
    }
}