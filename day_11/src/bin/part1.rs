fn travel_galaxies(s: &str) -> usize {
    let mut table: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let (input_y, input_x) = (table.len(), table[0].len());
    for (y, line) in s.lines().enumerate() {
        if !line.contains('#') {
            table.insert(y + table.len() - input_y, line.chars().collect());
        }
    }
    for x in 0..input_x {
        let new_input_x = table[0].len();
        let mut contains = false;
        for line in table.iter() {
            if line[x + new_input_x - input_x] == '#' {
                contains = true;
                break;
            }
        }
        if !contains {
            for line in table.iter_mut() {
                line.insert(x + new_input_x - input_x, '.');
            }
        }
    }

    let galaxies: Vec<[usize; 2]> = table.iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter()
            .enumerate()
            .filter_map(move |(x, c)| if *c == '#' {Some([x, y])} else {None}))
        .collect();


    (0..galaxies.len() - 1).flat_map(|i| {
        let galaxies = &galaxies;
        (i + 1..galaxies.len()) .map(move |j| {
            let ([x1, y1], [x2, y2]) = (galaxies[i], galaxies[j]);
            x2.abs_diff(x1) + y2.abs_diff(y1)
        })
    }).sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let res = travel_galaxies(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(travel_galaxies(test_input), 374);
    }
}