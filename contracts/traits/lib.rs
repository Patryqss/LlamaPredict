#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;

use ink::primitives::AccountId;
pub use errors::ManagerError;

pub type ManagerResult<T> = Result<T, ManagerError>;

#[ink::trait_definition]
pub trait ManagerTrait {
    #[ink(message)]
    fn disable(&mut self, token: AccountId) -> ManagerResult<()>;
    #[ink(message)]
    fn enable(&mut self, token: AccountId, address: AccountId) -> ManagerResult<()>;
}