use ark_ff::to_bytes;
use ark_ff::PrimeField;
use ark_std::marker::PhantomData;
use tiny_keccak::{Hasher, Keccak};

pub fn keccak_256(input: &[u8]) -> Vec<u8> {
	let mut hasher = Keccak::v256();
	hasher.update(input);
	let mut res: [u8; 32] = [0; 32];
	hasher.finalize(&mut res);
	res.to_vec()
}

pub struct KeccakHasher<F: PrimeField> {
	_field: PhantomData<F>,
}

impl<F: PrimeField> FieldHasher<F> for KeccakHasher<F> {
	fn hash(inputs: &Vec<&F>) -> F {
		let bytes = to_bytes![inputs].unwrap();
		let res = keccak_256(&bytes);
		F::from_le_bytes_mod_order(&res)
	}

	fn hash_two(left: &F, right: &F) -> F {
		let inputs = vec![left, right];
		Self::hash(&inputs)
	}
}

pub trait FieldHasher<F: PrimeField> {
	fn hash(inputs: &Vec<&F>) -> F;
	fn hash_two(left: &F, right: &F) -> F;
}
