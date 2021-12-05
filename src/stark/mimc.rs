use super::prime_field::PrimeField;
use num_bigint::BigUint;
use num_integer::Integer;

fn mimc(input: BigUint, steps: usize, round_constants: Vec<BigUint>) -> BigUint {
	let mut inp = input;
	for i in 0..(steps - 1) {
		inp = inp.pow_mod(3);
		inp = inp.add_mod(&round_constants[i % round_constants.len()]);
	}
	inp
}
