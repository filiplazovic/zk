pub mod ark_qap;
pub mod diffie_hellman;
pub mod elliptic;
pub mod fiat_shamir;
mod keccak;
pub mod merkle;
pub mod qap;
pub mod r1cs;

fn main() {
	ark_qap::run_example();
}
