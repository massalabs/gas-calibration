use super::static_public_key;

pub fn generate_abi_compare_pub_key(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.compare_pub_key(\"{}\", \"{}\");",
        static_public_key(),
        static_public_key()
    ));
}
