// (0, 0), (1, 0), (2, 0), (3, 5)

type Polynomial = [f32; 4];
type Row = [f32; 6];
type Group = [Row; 4];

fn eval_poly(x: f32, p: &Polynomial) -> f32 {
	p[0] + x * p[1] + x.powf(2.) * p[2] + x.powf(3.) * p[3]
}

fn eval_row(index: f32, polys: &[Polynomial; 6]) -> Row {
	let mut row: Row = [0.; 6];
	for (i, poly) in polys.iter().enumerate() {
		let res = eval_poly(index, poly);
		row[i] = res;
	}
	row
}

fn eval_group(polys: &[Polynomial; 6]) -> Group {
	let mut group: Group = [[0.; 6]; 4];
	for i in 1..5 {
		let row = eval_row(i as f32, &polys);
		group[i - 1] = row;
	}
	group
}

pub fn run_example() {
	let a = [
		[-5.0, 9.166, -5.0, 0.833],
		[8.0, -11.333, 5.0, -0.666],
		[0.0, 0.0, 0.0, 0.0],
		[-6.0, 9.5, -4.0, 0.5],
		[4.0, -7.0, 3.5, -0.5],
		[-1.0, 1.833, -1.0, 0.166],
	];

	let b = [
		[3.0, -5.166, 2.5, -0.333],
		[-2.0, 5.166, -2.5, 0.333],
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
	];

	let c = [
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
		[-1.0, 1.833, -1.0, 0.166],
		[4.0, -4.333, 1.5, -0.166],
		[-6.0, 9.5, -4.0, 0.5],
		[4.0, -7.0, 3.5, -0.5],
	];

	let res_a = eval_group(&a);
	let res_b = eval_group(&b);
	let res_c = eval_group(&c);
	println!("a: {:?}", res_a);
	println!("b: {:?}", res_b);
	println!("c: {:?}", res_c);
}
