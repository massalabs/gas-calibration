use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_unsafe_random(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!("  env.unsafe_random({});", rng.gen_range(5..1000)));
}
