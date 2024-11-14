use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::generate_string;

pub fn generate_abi_deferred_call_cancel(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut bool,
) {
    let mut rng = rand::thread_rng();

    if !*call_already_prep {
        preparation_calls.push(format!(
            "let call_id = env.deferred_call_register(\"{}\", \"{}\", {}, {}, {} ,toBytes(\"{}\"), {});
            env.set(toBytes(\"CALL_ID\"), toBytes(call_id));
            ",
            address_sc,
            generate_string(rng.gen_range(5..25)),
            rng.gen_range(100..1_000),
            rng.gen_range(0..THREAD_COUNT),
            rng.gen_range(100_000_000..1_000_000_000),
            rng.gen_range(0..2000),
            rng.gen_range(0..1000)
        ));
        calls.push(
            "let call_id = fromBytes(env.get(toBytes(\"CALL_ID\")));"
                .to_string(),
        );
        calls.push("env.deferred_call_cancel(call_id);".to_string());
        *call_already_prep = true;
    } else {
        calls.push("env.deferred_call_cancel(call_id);".to_string());
    }
}
