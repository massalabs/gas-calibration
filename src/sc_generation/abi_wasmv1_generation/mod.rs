use std::str::FromStr;

use massa_models::address::Address;
use massa_signature::KeyPair;
use rand::Rng;

mod abort;
mod add_native_amounts;
mod address_from_public_key;
mod append_data;
mod base58_check_to_bytes;
mod blake3_hash;
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
mod delete_data;
mod div_rem_native_amount;
mod div_rem_native_amounts;
mod generate_event;
mod get_address_category;
mod get_address_version;
mod get_balance;
mod get_bytecode;
mod get_call_coins;
mod get_call_stack;
mod get_current_slot;
mod get_data;
mod get_keys;
mod get_native_time;
mod get_op_data;
mod get_op_keys;
mod get_origin_operation_id;
mod get_owned_addresses;
mod get_pubkey_version;
mod get_remaining_gas;
mod get_signature_version;
mod has_data;
mod has_op_key;
mod hash_keccak256;
mod hash_sha256;
mod local_execution;
mod mul_native_amount;
mod native_amount_from_string;
mod native_amount_to_string;
mod send_async_message;
mod set_bytecode;
mod set_data;
mod sub_native_amounts;
mod transfer_coins;
mod unsafe_random;
mod verify_evm_signature;
mod verify_signature;
pub use abort::generate_abi_abort;
pub use add_native_amounts::generate_abi_add_native_amounts;
pub use address_from_public_key::generate_abi_address_from_public_key;
pub use append_data::generate_abi_append_data;
pub use base58_check_to_bytes::generate_abi_base58_check_to_bytes;
pub use blake3_hash::generate_abi_blake3_hash;
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
pub use delete_data::generate_abi_delete_data;
pub use div_rem_native_amount::generate_abi_div_rem_native_amount;
pub use div_rem_native_amounts::generate_abi_div_rem_native_amounts;
pub use generate_event::generate_abi_generate_event;
pub use get_address_category::generate_abi_get_address_category;
pub use get_address_version::generate_abi_get_address_version;
pub use get_balance::generate_abi_get_balance;
pub use get_bytecode::generate_abi_get_bytecode;
pub use get_call_coins::generate_abi_get_call_coins;
pub use get_call_stack::generate_abi_get_call_stack;
pub use get_current_slot::generate_abi_get_current_slot;
pub use get_data::generate_abi_get_data;
pub use get_keys::generate_abi_get_keys;
pub use get_native_time::generate_abi_get_native_time;
pub use get_op_data::generate_abi_get_op_data;
pub use get_op_keys::generate_abi_get_op_keys;
pub use get_origin_operation_id::generate_abi_get_origin_operation_id;
pub use get_owned_addresses::generate_abi_get_owned_addresses;
pub use get_pubkey_version::generate_abi_get_pubkey_version;
pub use get_remaining_gas::generate_abi_get_remaining_gas;
pub use get_signature_version::generate_abi_get_signature_version;
pub use has_data::generate_abi_has_data;
pub use has_op_key::generate_abi_has_op_key;
pub use hash_keccak256::generate_abi_hash_keccak256;
pub use hash_sha256::generate_abi_hash_sha256;
pub use local_execution::generate_abi_local_execution;
pub use mul_native_amount::generate_abi_mul_native_amount;
pub use native_amount_from_string::generate_abi_native_amount_from_string;
pub use native_amount_to_string::generate_abi_native_amount_to_string;
pub use send_async_message::generate_abi_send_async_message;
pub use set_bytecode::generate_abi_set_bytecode;
pub use set_data::generate_abi_set_data;
pub use sub_native_amounts::generate_abi_sub_native_amounts;
pub use transfer_coins::generate_abi_transfer_coins;
pub use unsafe_random::generate_abi_unsafe_random;
pub use verify_evm_signature::generate_abi_verify_evm_signature;
pub use verify_signature::generate_abi_verify_signature;

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn generate_address() -> String {
    let keypair = KeyPair::generate(0).unwrap();

    Address::from_public_key(&keypair.get_public_key()).to_string()
}

fn static_public_key() -> String {
    let keypair =
        KeyPair::from_str("S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY").unwrap();
    keypair.get_public_key().to_string()
    // Secret key: S12mhS7vUJen4g3VssogCDmbFp9mBqLU4PmavdaXPbpw7jyt9GXY
    // Public key: P12WKRCnYPKhVuwtk1mSEiMFSAPRfThR74bfhBEHAnT53JnBNj9T
    // Address: A12cMW9zRKFDS43Z2W88VCmdQFxmHjAo54XvuVV34UzJeXRLXW9M
}
