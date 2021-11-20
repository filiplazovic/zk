const A: f64 = 1.0;
const B: f64 = 2.0;

#[derive(Clone, Debug)]
struct Point {
	x: f64,
	y: f64,
}

impl Point {
	fn new(x: f64) -> Point {
		let y = eval_ec(x);
		Self { x, y }
	}
}

fn eval_ec(x: f64) -> f64 {
	(x.powf(3.) + A * x + B).sqrt()
}

fn s(p: Point, q: Point) -> f64 {
	let y_diff = p.y - q.y;
	let x_diff = p.x - q.x;
	y_diff / x_diff
}

fn s_single(p: Point) -> f64 {
	let num = 3. * p.x.powf(2.) + A;
	let den = 2. * p.y;
	num / den
}

fn add(p: Point, q: Point) -> Point {
	let s = s(p.clone(), q.clone());
	let r_x = s.powf(2.) - (p.x + q.x);
	let r_y = s * (p.x - r_x) - p.y;
	Point { x: r_x, y: r_y }
}

fn double(p: Point) -> Point {
	let s = s_single(p.clone());
	let r_x = s.powf(2.) - p.x;
	let r_y = s * (p.x - r_x) - p.y;
	Point { x: r_x, y: r_y }
}

pub fn run_example() {
	let a = Point::new(1.0);
	let b = Point::new(4.0);
	let r = add(a.clone(), b);
	println!("a + b: {:?}", r);
	let d = double(a);
	println!("double: {:?}", d);
}
