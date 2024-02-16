#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;

use errors::PredictorError;
use ink::primitives::AccountId;
use psp22::PSP22Error;

pub use errors::ManagerError;

pub type ManagerResult<T> = Result<T, ManagerError>;

#[ink::trait_definition]
pub trait ManagerTrait {
    #[ink(message)]
    fn disable(&mut self, token: AccountId) -> ManagerResult<()>;
    #[ink(message)]
    fn enable(&mut self, token: AccountId, address: AccountId) -> ManagerResult<()>;
}

#[ink::trait_definition]
pub trait PSP22Extras {
    #[ink(message)]
    fn mint_to(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error>;
    #[ink(message)]
    fn burn_from(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error>;
}

#[ink::trait_definition]
pub trait Predictor {
    #[ink(message)]
    fn add_tokens(&self, underlying_token: AccountId, from: AccountId, amount: u128) -> Result<(), PredictorError>;
}