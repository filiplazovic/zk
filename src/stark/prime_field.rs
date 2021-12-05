use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::identities::Zero;

pub trait PrimeField {
	fn modulus() -> BigUint;
	fn add_mod(&self, other: &BigUint) -> BigUint;
	fn mul_mod(&self, other: &BigUint) -> BigUint;
	fn sub_mod(&self, other: &BigUint) -> BigUint;
	fn pow_mod(&self, exp: u32) -> BigUint;
}

impl PrimeField for BigUint {
	fn modulus() -> BigUint {
		BigUint::zero()
	}

	fn add_mod(&self, other: &BigUint) -> Self {
		let modulus = <Self as PrimeField>::modulus();
		(self + other).mod_floor(&modulus)
	}

	fn sub_mod(&self, other: &BigUint) -> Self {
		let modulus = <Self as PrimeField>::modulus();
		(self - other).mod_floor(&modulus)
	}

	fn mul_mod(&self, other: &BigUint) -> Self {
		let modulus = <Self as PrimeField>::modulus();
		(self * other).mod_floor(&modulus)
	}

	fn pow_mod(&self, exp: u32) -> Self {
		let modulus = <Self as PrimeField>::modulus();
		self.pow(exp).mod_floor(&modulus)
	}
}
