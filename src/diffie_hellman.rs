use ark_ff::PrimeField;
use ark_std::test_rng;

pub fn test<F: PrimeField>() {
	let mut rng = test_rng();
	let g = F::rand(&mut rng);

	let alice_secret = F::rand(&mut rng);
	// A = g ^ a
	let alice_encryped_secret = g.pow(alice_secret.into_repr());

	let bob_secret = F::rand(&mut rng);

	// B = g ^ b
	let bob_encryped_secret = g.pow(bob_secret.into_repr());

	// t1 = B ^ a = (g ^ b) ^ a = g ^ ba
	let alice_s = bob_encryped_secret.pow(alice_secret.into_repr());
	// t2 = A ^ b = (g ^ a) ^ b = g ^ ab
	let bob_s = alice_encryped_secret.pow(bob_secret.into_repr());

	// t1 == t2
	println!("encryption key for the message: {:?}, {:?}", alice_s, bob_s);
}
