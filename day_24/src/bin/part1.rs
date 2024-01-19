use nom::{
    IResult, 
    sequence::{tuple, preceded},
    character::complete::{i64 as nom_int, multispace1}, bytes::complete::tag
};

fn delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(",")(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, ()))
}

fn parse_matrix(line: &str) -> IResult<&str, [f64; 3]> {
    let (input, (x, y, z)) = tuple((
        nom_int, 
        preceded(delimiter, nom_int), 
        preceded(delimiter, nom_int)
    ))(line)?;

    Ok((input, [x, y, z].map(|i| i as f64)))
}

fn matrix_delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, ()))
}

fn parse_line(line: &str) -> IResult<&str, [[f64; 3]; 2]> {
    let (input, (pos, speed)) = tuple((parse_matrix, preceded(matrix_delimiter, parse_matrix)))(line)?;

    Ok((input, [pos, speed]))
}


fn move_hails(input: &str, lower: usize, higher: usize) -> usize {
    let in_desired_range = |n: f64| (lower as f64..=higher as f64).contains(&n);
    let hails = input.lines().map(|line| parse_line(line).unwrap().1).collect::<Vec<_>>();
    let hails_len = hails.len();
    let mut res = 0;
    for i in 0..hails_len - 1 {
        for j in i..hails_len {
            let [[x0_1, y0_1, _], [vx1, vy1, _]] = hails[i];
            let [[x0_2, y0_2, _], [vx2, vy2, _]] = hails[j];
            /* 
            x = x0 + vx*t
            y = y0 + vy*t
            t = (x - x0)/vx
            y = y0 + vy/vx*(x - x0)
            y0_1 + vy_1/vx_1*(x - x0_1) = y0_2 + vy_2/vx_2*(x - x0_2)
            vy_1/vx_1*x - vy_2/vx_2*x = y0_2 - y0_1 + vy_1/vx_1*x0_1 - vy_2/vx_2*x0_2 
            dy = y2 - y1
            dv = vy / vx
            x = (dy + dv1*x0_1 - dv2*x0_2)/(dv1 - dv2)
            TODO: check for parallel lines
            */
            let dy = y0_2 - y0_1;
            let dv1 = vy1/vx1;
            let dv2 = vy2/vx2;
            let x = (dy + dv1*x0_1 - dv2*x0_2)/(dv1 - dv2);
            let y = y0_1 + dv1*(x - x0_1);
            let t1 = (x - x0_1)/vx1;
            let t2 = (x - x0_2)/vx2;
            let condtion = t1 >= 0.0 && t2 >= 0.0 && in_desired_range(x) && in_desired_range(y);
            if condtion {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let input = include_str!("./input.txt");
    let res = move_hails(input, 200000000000000, 400000000000000);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(move_hails(test_input, 7, 27), 2);
    }
}

