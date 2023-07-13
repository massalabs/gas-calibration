use super::generate_pub_key;

pub fn generate_abi_compare_pub_key(calls: &mut Vec<String>) {
    calls.push(format!(
        "env.compare_pub_key(\"{}\", \"{}\");",
        generate_pub_key(),
        generate_pub_key()
    ));
}
