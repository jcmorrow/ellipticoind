use crate::helpers::random;
use crate::vm::{Env, State, Transaction};
use serde_cbor::Value;

pub fn is_system_contract(transaction: &Transaction) -> bool {
    transaction.contract_address == [[0; 32].to_vec(), "System".as_bytes().to_vec()].concat()
}

pub fn run(transaction: &Transaction, state: &mut State, env: &Env) -> Value {
    match transaction.function.as_str() {
        "create_contract" => create_contract(transaction, state, env),
        _ => Value::Null,
    }
}

pub fn create_contract(transaction: &Transaction, state: &mut State, env: &Env) -> Value {
    if let [Value::Text(contract_name), serde_cbor::Value::Bytes(code), serde_cbor::Value::Array(arguments)] =
        &transaction.arguments[..]
    {
        let contract_address = [&transaction.sender, contract_name.as_bytes()].concat();
        state.set_code(&contract_address, code);
        let result = run_constuctor(transaction, state, env, contract_name, arguments);
        result
    } else {
        Value::Null
    }
}
fn run_constuctor(
    transaction: &Transaction,
    state: &mut State,
    env: &Env,
    contract_name: &str,
    arguments: &Vec<Value>,
) -> Value {
    let (result, _gas_left) = Transaction {
        function: "constructor".to_string(),
        arguments: arguments.to_vec(),
        sender: transaction.sender.clone(),
        nonce: transaction.nonce,
        gas_limit: transaction.gas_limit,
        contract_address: [
            transaction.sender.clone(),
            contract_name.as_bytes().to_vec(),
        ]
        .concat(),
    }
    .run(state, env);
    result
}

pub fn transfer(
    transaction: &Transaction,
    amount: u32,
    from: Vec<u8>,
    to: Vec<u8>,
    vm_state: &mut crate::vm::State,
    env: &Env,
) -> serde_cbor::Value {
    let arguments = vec![
        to.into_iter()
            .map(|n| n.into())
            .collect::<Vec<Value>>()
            .into(),
        Value::Integer(amount as i128),
    ];
    let transfer = Transaction {
        function: "transfer".to_string(),
        nonce: random(),
        gas_limit: transaction.gas_limit,
        contract_address: [[0 as u8; 32].to_vec(), "Ellipticoin".as_bytes().to_vec()].concat(),
        sender: from.clone(),
        arguments: arguments.clone(),
    };
    let (result, _) = transfer.run(vm_state, &env);
    result
}
