use super::static_evm_quadruplet;

pub fn generate_abi_evm_get_pubkey_from_signature(calls: &mut Vec<String>) {
    let (hash, _message, signature, _public_key) = static_evm_quadruplet();
    calls.push(format!(
        "{{let hash = new Uint8Array({});
        hash.set({:?});
        let signature = new Uint8Array({});
        signature.set({:?});
        env.evm_get_pubkey_from_signature(hash, signature);}}",
        hash.len(),
        hash,
        signature.len(),
        signature,
    ));
}
