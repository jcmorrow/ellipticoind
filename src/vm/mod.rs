extern crate hex;
extern crate metered_wasmi;
pub extern crate r2d2_redis;
pub extern crate rocksdb;
extern crate serde;
extern crate serde_cbor;
extern crate sha3;
extern crate time;

mod backend;
mod call;
mod error;
mod externals;
mod gas;
mod gas_costs;
mod helpers;
mod import_resolver;
mod memory;
mod transaction;

pub mod env;
pub mod state;
pub use backend::Backend;
pub use env::Env;
pub use helpers::zero_pad_vec;
pub use metered_wasmi::RuntimeValue;
use metered_wasmi::{ImportsBuilder, Module, ModuleInstance, ModuleRef, NopExternals};
pub use r2d2_redis::redis;
pub use r2d2_redis::redis::{pipe, Client, Commands, ControlFlow, PubSubCommands};
pub use r2d2_redis::{r2d2, RedisConnectionManager};
pub use rocksdb::DB;
pub use state::{Changeset, State};
pub use transaction::{CompletedTransaction, Transaction};

pub struct VM<'a> {
    pub instance: &'a ModuleRef,
    pub transaction: &'a Transaction,
    pub state: &'a mut State,
    pub gas: Option<u32>,
    pub env: &'a Env,
}

pub fn new_module_instance(code: Vec<u8>) -> Result<ModuleRef, metered_wasmi::Error> {
    let module = Module::from_buffer(code)?;

    let mut imports = ImportsBuilder::new();
    imports.push_resolver("env", &import_resolver::ImportResolver);
    Ok(ModuleInstance::new(&module, &imports)
        .expect("Failed to instantiate module")
        .run_start(&mut NopExternals)
        .expect("Failed to run start function in module"))
}
