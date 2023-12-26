use itertools::Itertools;
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

mod input;

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl Hailstone {
    pub fn intersects(&self, other: &Hailstone, min: f64, max: f64) -> bool {
        let t2 = ((other.position.y - self.position.y) * self.velocity.x - (other.position.x - self.position.x) * self.velocity.y) / (other.velocity.x * self.velocity.y - other.velocity.y * self.velocity.x);
        let t1 = (other.position.x - self.position.x + t2 * other.velocity.x) / self.velocity.x;

        let x = self.position.x + t1 * self.velocity.x;
        let y = self.position.y + t1 * self.velocity.y;

        t1.is_sign_positive() && t2.is_sign_positive() && x >= min && y >= min && x <= max && y <= max
    }
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input.lines().map(|line| {
        let mut posvel = line.split(" @ ");
        let pos = posvel.next().unwrap().split(", ").collect::<Vec<&str>>();
        let vel = posvel.next().unwrap().split(", ").collect::<Vec<&str>>();
        let position = Vec3::new(pos[0].trim().parse::<f64>().unwrap(),
                                 pos[1].trim().parse::<f64>().unwrap(),
                                 pos[2].trim().parse::<f64>().unwrap());
        let velocity = Vec3::new(vel[0].trim().parse::<f64>().unwrap(),
                                    vel[1].trim().parse::<f64>().unwrap(),
                                    vel[2].trim().parse::<f64>().unwrap());
        Hailstone {
            position,
            velocity,
        }
    }).collect()
}

fn check_intersections_xy(hailstones: &mut Vec<Hailstone>, min: f64, max: f64) -> i32 {
    let mut count = 0;
    for (a, b) in hailstones.iter().tuple_combinations() {
        if a.intersects(b, min, max) {
            count += 1;
        }
    }
    count
}

fn solve_rock_position(hailstones: &Vec<Hailstone>) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.position.x as i64);
        let pyn = Int::from_i64(&ctx, hailstone.position.y as i64);
        let pzn = Int::from_i64(&ctx, hailstone.position.z as i64);
        let vxn = Int::from_i64(&ctx, hailstone.velocity.x as i64);
        let vyn = Int::from_i64(&ctx, hailstone.velocity.y as i64);
        let vzn = Int::from_i64(&ctx, hailstone.velocity.z as i64);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    println!("x: {}, y: {}, z: {}, x + y + z: {}", x, y, z, x + y + z);
    x + y + z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = input::EXAMPLE_INPUT;
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed[0].position.x, 19f64);
        assert_eq!(parsed[0].position.y, 13f64);
        assert_eq!(parsed[0].position.z, 30f64);
        assert_eq!(parsed[0].velocity.x, -2f64);
        assert_eq!(parsed[0].velocity.y, 1f64);
        assert_eq!(parsed[0].velocity.z, -2f64);
    }

    #[test]
    fn can_simulate_example_part_one() {
        let input = input::EXAMPLE_INPUT;
        let mut parsed = parse_input(input);
        let count = check_intersections_xy(&mut parsed, 7f64, 27f64);
        assert_eq!(count, 2);
    }

    #[test]
    fn can_simulate_part_one() {
        let input = input::INPUT;
        let mut parsed = parse_input(input);
        let count = check_intersections_xy(&mut parsed, 200000000000000f64, 400000000000000f64);
        assert_eq!(count, 20361);
    }

    #[test]
    fn can_simulate_example_part_two() {
        let input = input::EXAMPLE_INPUT;
        let parsed = parse_input(input);
        let result = solve_rock_position(&parsed);
        assert_eq!(result, 47);
    }

    #[test]
    fn can_simulate_part_two() {
        let input = input::INPUT;
        let parsed = parse_input(input);
        let result = solve_rock_position(&parsed);
        assert_eq!(result, 558415252330828);
    }
}
