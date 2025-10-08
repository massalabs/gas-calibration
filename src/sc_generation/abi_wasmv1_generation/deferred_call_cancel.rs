use massa_models::config::THREAD_COUNT;
use rand::Rng;

use super::generate_string;

pub fn generate_abi_deferred_call_cancel(
    address_sc: &str,
    calls: &mut Vec<String>,
    preparation_calls: &mut Vec<String>,
    call_already_prep: &mut u64,
) {
    let mut rng = rand::thread_rng();

    preparation_calls.push(format!(
            "let call_id_{} = 
            env.deferred_call_register(\"{}\", \"{}\", {}, {}, {} ,toBytes(\"{}\"), {});
            env.set_ds_value(toBytes(\"CALL_ID_{}\"), toBytes(call_id_{}), null);
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
        "
        let call_id_{} =  String.UTF8.decode(env.get_ds_value(toBytes(\"CALL_ID_{}\"), null).buffer);
        env.deferred_call_cancel(call_id_{});",
        call_already_prep,
        call_already_prep,
        call_already_prep,
    ));
    *call_already_prep = *call_already_prep + 1;
}
