use massa_models::config::THREAD_COUNT;
use rand::{rngs::ThreadRng, Rng};

use super::generate_string;

pub fn generate_abi_deferred_call_cancel(
    address_sc: &str,
    rng: &mut ThreadRng,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut u64,
) {
    preparation_calls.push(format!(
            "let call_id_{} = 
            env.deferredCallRegister(\"{}\", \"{}\", {}, {}, {} ,toBytes(\"{}\"), {});
            env.set(toBytes(\"CALL_ID_{}\"), toBytes(call_id_{}));
            ",
            call_already_prep,
            address_sc,
            generate_string(rng.gen_range(5..25)),
            rng.gen_range(100..1_000),
            rng.gen_range(0..THREAD_COUNT),
            rng.gen_range(100_000_000..200_000_000),
            rng.gen_range(0..2000),
            rng.gen_range(0..1000),
            call_already_prep,
            call_already_prep,
        ));

    calls.push(format!(
        "let call_id_{} = fromBytes(env.get(toBytes(\"CALL_ID_{}\")));
        env.deferredCallCancel(call_id_{});",
        call_already_prep, call_already_prep, call_already_prep,
    ));

    *call_already_prep = *call_already_prep + 1;
}
