use super::generate_pub_key;

pub fn generate_abi_check_pubkey(calls: &mut Vec<String>) {
    calls.push(format!("  env.check_pubkey(\"{}\");", generate_pub_key()));
}
