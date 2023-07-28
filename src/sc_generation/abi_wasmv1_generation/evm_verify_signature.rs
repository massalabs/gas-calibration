use super::static_evm_quadruplet;

pub fn generate_abi_evm_verify_signature(calls: &mut Vec<String>) {
    let (_hash, message, signature, public_key) = static_evm_quadruplet();
    calls.push(format!(
        "{{let message = new Uint8Array({});
        message.set({:?});
        let signature = new Uint8Array({});
        signature.set({:?});
        let public_key = new Uint8Array({});
        public_key.set({:?});
        env.evm_verify_signature(message, signature, public_key);}}",
        message.len(),
        message,
        signature.len(),
        signature,
        public_key.len(),
        public_key,
    ));
}
