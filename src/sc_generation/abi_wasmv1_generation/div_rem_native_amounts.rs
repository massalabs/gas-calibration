use rand::{rngs::ThreadRng, Rng};

pub fn generate_abi_div_rem_native_amounts(rng: &mut ThreadRng, calls: &mut Vec<String>) {
    calls.push(format!(
        "env.div_rem_native_amounts(env.make_native_amount({}, 0), env.make_native_amount({}, 0));",
        rng.gen_range(1..100),
        rng.gen_range(1..100)
    ));
}
