use super::static_evm_quadruplet;

pub fn generate_abi_evm_get_address_from_pubkey(calls: &mut Vec<String>) {
    let (_hash, _message, _signature, public_key) = static_evm_quadruplet();
    calls.push(format!(
        "{{let public_key = new Uint8Array({});
        public_key.set({:?});
        env.evm_get_address_from_pubkey(public_key);}}",
        public_key.len(),
        public_key,
    ));
}
