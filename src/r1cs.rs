type R1csNumber = [i32; 6];

fn dot_product(a: R1csNumber, b: R1csNumber) -> i32 {
	let mut sum = 0;
	for (i, num) in b.iter().enumerate() {
		sum += num * a[i];
	}
	sum
}

fn r1cs_eval(a: R1csNumber, b: R1csNumber, c: R1csNumber) -> i32 {
	let secret: R1csNumber = [1, 3, 35, 9, 27, 30];

	let a_sum = dot_product(a, secret);
	let b_sum = dot_product(b, secret);
	let c_sum = dot_product(c, secret);

	a_sum * b_sum - c_sum
}

fn r1cs_add(a: R1csNumber, b: R1csNumber) -> R1csNumber {
	let mut res = [0; 6];
	for (i, num) in a.iter().enumerate() {
		res[i] = num + b[i];
	}
	res
}

// Circuit:
// sym_1 = x * x
// y = sym_1 * x
// sym_2 = y + x
// ~out = sym_2 + 5
fn run_example() {
	let a1: R1csNumber = [0, 1, 0, 0, 0, 0];
	let output1: R1csNumber = [0, 0, 0, 1, 0, 0];
	let res = r1cs_eval(a1, a1, output1);
	println!("sym1: {}", res);

	let output2 = [0, 0, 0, 0, 1, 0];
	let res2 = r1cs_eval(output1, a1, output2);
	println!("y: {}", res2);

	let a3 = r1cs_add(output2, a1);
	let b3: R1csNumber = [1, 0, 0, 0, 0, 0];
	let output3 = [0, 0, 0, 0, 0, 1];
	let res3 = r1cs_eval(a3, b3, output3);

	println!("sym2: {}", res3);

	let const_5: R1csNumber = [5, 0, 0, 0, 0, 0];
	let a4 = r1cs_add(const_5, output3);
	let b4: R1csNumber = [1, 0, 0, 0, 0, 0];
	let output4: R1csNumber = [0, 0, 1, 0, 0, 0];
	let res4 = r1cs_eval(a4, b4, output4);

	println!("out: {}", res4);
}
