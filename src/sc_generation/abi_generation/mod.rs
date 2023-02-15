use std::str::FromStr;

use massa_models::address::Address;
use massa_signature::KeyPair;
use rand::Rng;

mod append;
mod append_of;
mod balance;
mod balance_of;
mod call;
mod call_coins;
mod call_stack;
mod caller_has_write_access;
mod create_sc;
mod current_period;
mod current_thread;
mod del;
mod del_of;
mod function_exists;
mod generate_event;
mod get;
mod get_bytecode;
mod get_bytecode_of;
mod get_keys;
mod get_keys_of;
mod get_of;
mod get_op_data;
mod get_op_keys;
mod has;
mod has_of;
mod has_op_key;
mod hash_sha256;
mod is_signature_valid;
mod local_call;
mod local_execution;
mod owned_addresses;
mod print;
mod public_key_to_address;
mod remaining_gas;
mod send_message;
mod set;
mod set_bytecode;
mod set_bytecode_of;
mod set_of;
mod time;
mod to_base58;
mod transfer_coins;
mod transfer_coins_of;
mod unsafe_random;

pub use append::generate_abi_append;
pub use append_of::generate_abi_append_of;
pub use balance::generate_abi_balance;
pub use balance_of::generate_abi_balance_of;
pub use call::generate_abi_call;
pub use call_coins::generate_abi_call_coins;
pub use call_stack::generate_abi_call_stack;
pub use caller_has_write_access::generate_abi_caller_has_write_access;
pub use create_sc::generate_abi_create_sc;
pub use current_period::generate_abi_current_period;
pub use current_thread::generate_abi_current_thread;
pub use del::generate_abi_del;
pub use del_of::generate_abi_del_of;
pub use function_exists::generate_abi_function_exists;
pub use generate_event::generate_abi_generate_event;
pub use get::generate_abi_get;
pub use get_bytecode::generate_abi_get_bytecode;
pub use get_bytecode_of::generate_abi_get_bytecode_of;
pub use get_keys::generate_abi_get_keys;
pub use get_keys_of::generate_abi_get_keys_of;
pub use get_of::generate_abi_get_of;
pub use get_op_data::generate_abi_get_op_data;
pub use get_op_keys::generate_abi_get_op_keys;
pub use has::generate_abi_has;
pub use has_of::generate_abi_has_of;
pub use has_op_key::generate_abi_has_op_key;
pub use hash_sha256::generate_abi_hash_sha256;
pub use is_signature_valid::generate_abi_is_signature_valid;
pub use local_call::generate_abi_local_call;
pub use local_execution::generate_abi_local_execution;
pub use owned_addresses::generate_abi_owned_addresses;
pub use print::generate_abi_print;
pub use public_key_to_address::generate_abi_public_key_to_address;
pub use remaining_gas::generate_abi_remaining_gas;
pub use send_message::generate_abi_send_message;
pub use set::generate_abi_set;
pub use set_bytecode::generate_abi_set_bytecode;
pub use set_bytecode_of::generate_abi_set_bytecode_of;
pub use set_of::generate_abi_set_of;
pub use time::generate_abi_time;
pub use to_base58::generate_abi_to_base58;
pub use transfer_coins::generate_abi_transfer_coins;
pub use transfer_coins_of::generate_abi_transfer_coins_of;
pub use unsafe_random::generate_abi_unsafe_random;

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    for _ in 0..length {
        string.push(rng.gen_range('a'..='z'));
    }
    string
}

fn generate_address() -> String {
    let keypair = KeyPair::generate();

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
