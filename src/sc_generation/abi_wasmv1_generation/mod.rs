use massa_hash::Hash;
use massa_models::{address::Address, amount::Amount};
use massa_signature::KeyPair;
use rand::Rng;

mod abort;
mod add_native_amount;
mod address_from_public_key;
mod append_ds_value;
mod base58_check_to_bytes;
mod bytes_to_base58_check;
mod call;
mod caller_has_write_access;
mod check_address;
mod check_native_amount;
mod check_pubkey;
mod check_signature;
mod checked_add_native_time;
mod checked_div_native_time;
mod checked_mul_native_time;
mod checked_scalar_div_native_time;
mod checked_sub_native_time;
mod compare_address;
mod compare_native_amount;
mod compare_native_time;
mod compare_pub_key;
mod create_sc;
mod delete_ds_entry;
mod div_rem_native_amount;
mod ds_entry_exists;
mod generate_event;
mod get_address_category;
mod get_address_version;
mod get_balance;
mod get_bytecode;
mod get_call_coins;
mod get_call_stack;
mod get_current_slot;
mod get_ds_keys;
mod get_ds_value;
mod get_native_time;
mod get_op_data;
mod get_op_keys;
mod get_origin_operation_id;
mod get_owned_addresses;
mod get_pubkey_version;
mod get_remaining_gas;
mod get_signature_version;
mod hash_blake3;
mod hash_keccak256;
mod hash_sha256;
mod local_execution;
mod native_amount_from_string;
mod native_amount_to_string;
mod op_entry_exists;
mod scalar_div_rem_native_amount;
mod scalar_mul_native_amount;
mod send_async_message;
mod set_bytecode;
mod set_ds_value;
mod sub_native_amount;
mod transfer_coins;
mod unsafe_random;
mod verify_evm_signature;
mod verify_signature;
pub use abort::generate_abi_abort;
pub use add_native_amount::generate_abi_add_native_amount;
pub use address_from_public_key::generate_abi_address_from_public_key;
pub use append_ds_value::generate_abi_append_ds_value;
pub use base58_check_to_bytes::generate_abi_base58_check_to_bytes;
pub use bytes_to_base58_check::generate_abi_bytes_to_base58_check;
pub use call::generate_abi_call;
pub use caller_has_write_access::generate_abi_caller_has_write_access;
pub use check_address::generate_abi_check_address;
pub use check_native_amount::generate_abi_check_native_amount;
pub use check_pubkey::generate_abi_check_pubkey;
pub use check_signature::generate_abi_check_signature;
pub use checked_add_native_time::generate_abi_checked_add_native_time;
pub use checked_div_native_time::generate_abi_checked_div_native_time;
pub use checked_mul_native_time::generate_abi_checked_mul_native_time;
pub use checked_scalar_div_native_time::generate_abi_checked_scalar_div_native_time;
pub use checked_sub_native_time::generate_abi_checked_sub_native_time;
pub use compare_address::generate_abi_compare_address;
pub use compare_native_amount::generate_abi_compare_native_amount;
pub use compare_native_time::generate_abi_compare_native_time;
pub use compare_pub_key::generate_abi_compare_pub_key;
pub use create_sc::generate_abi_create_sc;
pub use delete_ds_entry::generate_abi_delete_ds_entry;
pub use div_rem_native_amount::generate_abi_div_rem_native_amount;
pub use ds_entry_exists::generate_abi_ds_entry_exists;
pub use generate_event::generate_abi_generate_event;
pub use get_address_category::generate_abi_get_address_category;
pub use get_address_version::generate_abi_get_address_version;
pub use get_balance::generate_abi_get_balance;
pub use get_bytecode::generate_abi_get_bytecode;
pub use get_call_coins::generate_abi_get_call_coins;
pub use get_call_stack::generate_abi_get_call_stack;
pub use get_current_slot::generate_abi_get_current_slot;
pub use get_ds_keys::generate_abi_get_ds_keys;
pub use get_ds_value::generate_abi_get_ds_value;
pub use get_native_time::generate_abi_get_native_time;
pub use get_op_data::generate_abi_get_op_data;
pub use get_op_keys::generate_abi_get_op_keys;
pub use get_origin_operation_id::generate_abi_get_origin_operation_id;
pub use get_owned_addresses::generate_abi_get_owned_addresses;
pub use get_pubkey_version::generate_abi_get_pubkey_version;
pub use get_remaining_gas::generate_abi_get_remaining_gas;
pub use get_signature_version::generate_abi_get_signature_version;
pub use hash_blake3::generate_abi_hash_blake3;
pub use hash_keccak256::generate_abi_hash_keccak256;
pub use hash_sha256::generate_abi_hash_sha256;
pub use local_execution::generate_abi_local_execution;
pub use native_amount_from_string::generate_abi_native_amount_from_string;
pub use native_amount_to_string::generate_abi_native_amount_to_string;
pub use op_entry_exists::generate_abi_op_entry_exists;
pub use scalar_div_rem_native_amount::generate_abi_scalar_div_rem_native_amount;
pub use scalar_mul_native_amount::generate_abi_scalar_mul_native_amount;
pub use send_async_message::generate_abi_send_async_message;
pub use set_bytecode::generate_abi_set_bytecode;
pub use set_ds_value::generate_abi_set_ds_value;
pub use sub_native_amount::generate_abi_sub_native_amount;
pub use transfer_coins::generate_abi_transfer_coins;
pub use unsafe_random::generate_abi_unsafe_random;
pub use verify_evm_signature::generate_abi_verify_evm_signature;
pub use verify_signature::generate_abi_verify_signature;

pub fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn generate_b58_check_string(byte_size: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut data = vec![0_u8];
    for _ in 0..byte_size {
        data.push(rng.gen_range(u8::MIN..=u8::MAX));
    }

    bs58::encode(data).with_check().into_string()
}

fn generate_native_amount_string(mantissa: u64, scale: u32) -> String {
    let amount = Amount::from_mantissa_scale(mantissa, scale).unwrap();
    amount.to_string()
}

fn generate_address() -> String {
    let keypair = KeyPair::generate(0).unwrap();
    Address::from_public_key(&keypair.get_public_key()).to_string()
}

fn generate_pub_key() -> String {
    let keypair = KeyPair::generate(0).unwrap();
    keypair.get_public_key().to_string()
}

fn generate_signature() -> String {
    let keypair = KeyPair::generate(0).unwrap();

    let mut rng = rand::thread_rng();
    let mut message = String::new();
    for _ in 0..rng.gen_range(1..=100) {
        message.push(rng.gen_range('a'..='z'));
    }
    let msg_hash = Hash::compute_from(message.as_bytes());

    keypair.sign(&msg_hash).unwrap().to_string()
}

fn static_evm_triplet() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let message = b"test";
    let signature = [
        208, 208, 92, 53, 8, 6, 53, 181, 232, 101, 0, 108, 108, 79, 91, 93, 69, 126, 195, 66, 86,
        77, 143, 198, 124, 228, 14, 220, 38, 76, 205, 171, 63, 47, 54, 107, 91, 209, 227, 133, 130,
        83, 143, 237, 127, 166, 40, 33, 72, 232, 106, 249, 121, 112, 161, 12, 179, 48, 40, 150,
        245, 214, 142, 245, 27,
    ];
    let public_key = [
        4, 242, 192, 4, 239, 171, 3, 90, 235, 89, 176, 251, 75, 242, 14, 65, 253, 71, 177, 105, 0,
        111, 117, 57, 248, 100, 223, 210, 100, 6, 122, 58, 123, 252, 232, 199, 170, 56, 251, 37,
        74, 97, 93, 193, 95, 18, 147, 233, 195, 248, 196, 141, 114, 17, 114, 13, 138, 233, 242,
        105, 9, 142, 173, 144, 14,
    ];
    (message.to_vec(), signature.to_vec(), public_key.to_vec())
}
