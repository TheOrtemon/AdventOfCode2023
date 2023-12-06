fn combine_travel_ways(s: &str) -> usize {
    let mut lines = s.lines();
    let time: f64 = lines.next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: f64 = lines.last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

        let opt_time_max = (time + (time * time - 4.0 * distance).sqrt()) / 2.0;
        let opt_time_min = time - opt_time_max;
        let mut opt_time_max_floor = opt_time_max.floor() as usize;
        let mut opt_time_min_ceil = opt_time_min.ceil() as usize;
        if opt_time_max_floor * opt_time_min_ceil == distance as usize {
            opt_time_max_floor -= 1;
            opt_time_min_ceil += 1;
        }
        opt_time_max_floor - opt_time_min_ceil + 1
    
    // T = a + b; a, r are Z; => a = T - b
    // V = a; V_0 = 0;
    // L = a * b; => L = (T - b) * b; => b ** 2 - b * T + L = 0;
    // Discr = T * T - 4 * L
    // x1, 2 = T +- sqrt(Discr) / 2
    // optimal_b = (T + sqrt(T * T - 4 * L)) / 2;
    // optimal a = T - b;
}

fn main() {
    let input = include_str!("./input.txt");
    let res = combine_travel_ways(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = 
"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(combine_travel_ways(test_input), 71503);
    }
}