use super::static_public_key;

pub fn generate_abi_check_pubkey(calls: &mut Vec<String>) {
    calls.push(format!("  env.check_pubkey(\"{}\");", static_public_key()));
}
