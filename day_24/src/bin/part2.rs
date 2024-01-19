use nom::{
    IResult, 
    sequence::{tuple, preceded},
    character::complete::{i64 as nom_int, multispace1}, bytes::complete::tag
};
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

fn delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(",")(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, ()))
}

fn parse_matrix(line: &str) -> IResult<&str, [i64; 3]> {
    let (input, (x, y, z)) = tuple((
        nom_int, 
        preceded(delimiter, nom_int), 
        preceded(delimiter, nom_int)
    ))(line)?;

    Ok((input, [x, y, z]))
}

fn matrix_delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, ()))
}

fn parse_line(line: &str) -> IResult<&str, [[i64; 3]; 2]> {
    let (input, (pos, speed)) = tuple((parse_matrix, preceded(matrix_delimiter, parse_matrix)))(line)?;

    Ok((input, [pos, speed]))
}

fn move_hails(input: &str) -> usize {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let hails = input.lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect::<Vec<_>>();

    /*
    x = x0 + vx*t
    y = y0 + vy*t
    z = z0 + vz*t

    x3-x1/x2-x1=y3-y1/y2-y1=z3-z1/z2-z1
    */

    let [[x0_1, y0_1, z0_1], [vx_1, vy_1, vz_1]] = hails[4];
    let [[x0_2, y0_2, z0_2], [vx_2, vy_2, vz_2]] = hails[1];
    let [[x0_3, y0_3, z0_3], [vx_3, vy_3, vz_3]] = hails[2];
    let [[x0_4, y0_4, z0_4], [vx_4, vy_4, vz_4]] = hails[3];

    let t_1 = Int::new_const(&ctx, "t_1");
    let t_2 = Int::new_const(&ctx, "t_2");
    let t_3 = Int::new_const(&ctx, "t_3");
    let t_4 = Int::new_const(&ctx, "t_4");

    let x_1 = x0_1 + &t_1 * vx_1;
    let y_1 = y0_1 + &t_1 * vy_1;
    let z_1 = z0_1 + &t_1 * vz_1;
    let x_2 = x0_2 + &t_2 * vx_2;
    let y_2 = y0_2 + &t_2 * vy_2;
    let z_2 = z0_2 + &t_2 * vz_2;
    let x_3 = x0_3 + &t_3 * vx_3;
    let y_3 = y0_3 + &t_3 * vy_3;
    let z_3 = z0_3 + &t_3 * vz_3;
    let x_4 = x0_4 + &t_4 * vx_4;
    let y_4 = y0_4 + &t_4 * vy_4;
    let z_4 = z0_4 + &t_4 * vz_4;

    solver.assert(&((&x_3 - &x_1) * (&y_2 - &y_1))._eq(&((&y_3 - &y_1) * (&x_2 - &x_1))));
    solver.assert(&((&x_3 - &x_1) * (&z_2 - &z_1))._eq(&((&z_3 - &z_1) * (&x_2 - &x_1))));
    solver.assert(&((&x_4 - &x_1) * (&y_2 - &y_1))._eq(&((&y_4 - &y_1) * (&x_2 - &x_1))));
    solver.assert(&((&x_4 - &x_1) * (&z_2 - &z_1))._eq(&((&z_4 - &z_1) * (&x_2 - &x_1))));

    solver.assert(&((&x_4 - &x_1) * (&y_3 - &y_1))._eq(&((&y_4 - &y_1) * (&x_3 - &x_1))));
    solver.assert(&((&x_3 - &x_2) * (&y_2 - &y_1))._eq(&((&y_3 - &y_2) * (&x_2 - &x_1))));
    solver.assert(&((&x_3 - &x_2) * (&z_2 - &z_1))._eq(&((&z_3 - &z_2) * (&x_2 - &x_1))));
    solver.assert(&((&x_4 - &x_1) * (&z_3 - &z_1))._eq(&((&z_4 - &z_1) * (&x_3 - &x_1))));

    solver.assert(&t_1.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&t_2.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&t_3.gt(&Int::from_i64(&ctx, 0)));
    solver.assert(&t_4.gt(&Int::from_i64(&ctx, 0)));
    let res = match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let t_1_res = model.eval(&t_1, true).unwrap().as_i64().unwrap();
            let t_2_res = model.eval(&t_2, true).unwrap().as_i64().unwrap();

            let x1_res = x0_1 + vx_1 * t_1_res;
            let y1_res = y0_1 + vy_1 * t_1_res;
            let z1_res = z0_1 + vz_1 * t_1_res;
            let x2_res = x0_2 + vx_2 * t_2_res;
            let y2_res = y0_2 + vy_2 * t_2_res;
            let z2_res = z0_2 + vz_2 * t_2_res;

            let vx_1_res = (x2_res - x1_res) / (t_2_res - t_1_res);
            let vy_1_res = (y2_res - y1_res) / (t_2_res - t_1_res);
            let vz_1_res = (z2_res - z1_res) / (t_2_res - t_1_res);

            let x0_res = x1_res - vx_1_res * t_1_res;
            let y0_res = y1_res - vy_1_res * t_1_res;
            let z0_res = z1_res - vz_1_res * t_1_res;
            
            Some(x0_res + y0_res + z0_res)
        },
        _ => None
    };
    res.unwrap() as usize
}

fn main() {
    let input = include_str!("./input.txt");
    let res = move_hails(input);
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
        assert_eq!(move_hails(test_input), 47);
    }
}

