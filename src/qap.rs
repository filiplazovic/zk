// (0, 0), (1, 0), (2, 0), (3, 5)
use rustnomial::Polynomial as Rustnomial;

type Polynomial = [f32; 4];
type PolynomialGroup = [Polynomial; 6];
type TransposedPolynomialGroup = [Row; 4];
type Witness = [f32; 6];
type Row = [f32; 6];
type Group = [Row; 4];

fn transpose(a: &PolynomialGroup) -> TransposedPolynomialGroup {
	let mut res: TransposedPolynomialGroup = [[0.; 6]; 4];
	for i in 0..6 {
		for j in 0..4 {
			res[j][i] = a[i][j];
		}
	}
	res
}

fn poly_pairwise_mul(w: &Witness, p: &Row) -> f32 {
	let mut sum = 0.;
	for (w_i, p_i) in w.iter().zip(p.iter()) {
		sum += w_i * p_i;
	}
	sum
}

fn dot_product(w: &Witness, pg: &TransposedPolynomialGroup) -> Polynomial {
	let mut res: Polynomial = [0.; 4];
	for (i, p) in pg.iter().enumerate() {
		let sum = poly_pairwise_mul(w, p);
		res[i] = sum;
	}
	res
}

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
	// println!("a: {:?}", res_a);
	// println!("b: {:?}", res_b);
	// println!("c: {:?}", res_c);

	let secret: Witness = [1., 3., 35., 9., 27., 30.];
	let a_t = transpose(&a);
	let mut a_d = dot_product(&secret, &a_t);
	println!("A.s {:?}", a_d);

	let b_t = transpose(&b);
	let mut b_d = dot_product(&secret, &b_t);
	println!("B.s {:?}", b_d);

	let c_t = transpose(&c);
	let mut c_d = dot_product(&secret, &c_t);
	println!("C.s {:?}", c_d);

	// Reverse to match vitalik article
	a_d.reverse();
	b_d.reverse();
	c_d.reverse();
	let rust_ad = Rustnomial::new(a_d.to_vec());
	let rust_bd = Rustnomial::new(b_d.to_vec());
	let rust_cd = Rustnomial::new(c_d.to_vec());

	let t = rust_ad * rust_bd - rust_cd;
	// Reverse to match vitalik article
	let mut poly_display = t.terms.clone();
	poly_display.reverse();
	println!("A.s * B.s - C.s {:?}", poly_display);

	let mut minimal_poly = [24., -50., 35., -10., 1.].to_vec();
	minimal_poly.reverse();
	let z = Rustnomial::<f32>::new(minimal_poly);

	let h = t.div_mod(&z);

	println!("{:?}", h);
}
