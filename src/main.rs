pub mod ark_qap;
use ark_bn254::Fq as Bn254;
pub mod diffie_hellman;
pub mod elliptic;
pub mod fiat_shamir;
mod keccak;
pub mod merkle;
pub mod qap;
pub mod r1cs;

fn main() {
	fiat_shamir::test::<Bn254>();
}
