use ark_std::marker::PhantomData;
use std::collections::HashMap;
use std::convert::TryInto;

use crate::keccak::FieldHasher;
use ark_ff::PrimeField;

pub struct SparseMerkleTree<F: PrimeField, FH: FieldHasher<F>> {
	tree: HashMap<u128, F>,
	_fh: PhantomData<FH>,
}

impl<F: PrimeField, FH: FieldHasher<F>> SparseMerkleTree<F, FH> {
	pub fn new(leaves: &Vec<F>) -> Self {
		let last_level_size = leaves.len().next_power_of_two() as u128;
		let tree_size = 2 * last_level_size - 1;
		let tree_height = log2(tree_size);

		let mut tree = HashMap::new();

		for (i, leaf) in leaves.iter().enumerate() {
			tree.insert(i as u128, leaf.clone());
		}

		for i in (0..tree_size).step_by(2) {
			let parent_index = parent(i);
			let sibling_index = sibling(i);

			let left = tree[&i];
			let right = tree[&sibling_index];
			tree.insert(parent_index, FH::hash_two(&left, &right));
		}

		Self {
			tree: tree,
			_fh: PhantomData,
		}
	}
}

pub fn log2(x: u128) -> u8 {
	let val = if x == 0 {
		0
	} else if x.is_power_of_two() {
		1u128.leading_zeros() - x.leading_zeros()
	} else {
		1u128.leading_zeros() - x.leading_zeros()
	};

	val.try_into().unwrap()
}

#[inline]
fn sibling(index: u128) -> u128 {
	if is_left_child(index) {
		index + 1
	} else {
		index - 1
	}
}

#[inline]
fn is_left_child(index: u128) -> bool {
	index % 2 == 0
}

#[inline]
fn parent(index: u128) -> u128 {
	index >> 1
}
