#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::conditional_psp22::ConditionalPSP22Ref;

#[ink::contract]
pub mod conditional_psp22 {
    use ink::storage_item;
    use ink::prelude::{vec::Vec, string::String};
    use psp22::{PSP22Data, PSP22Error, PSP22Event, PSP22};

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    #[storage_item]
    #[derive(Debug)]
    pub struct ConditionalTokenData {
        pub manager: AccountId,
        pub router: AccountId,
    }

    #[ink(storage)]
    pub struct ConditionalPSP22 {
        psp22: PSP22Data,
        data: ConditionalTokenData,
    }

    impl ConditionalPSP22 {
        #[ink(constructor)]
        pub fn new(router: AccountId) -> Self {
            let caller = Self::env().caller();
            let psp22 = PSP22Data::new(0, caller);
            let data = ConditionalTokenData {
                manager: caller,
                router,
            };
            Self {
                psp22,
                data,
            }
        }

        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }

        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error> {
            if self.env().caller() != self.data.manager {
                return Err(PSP22Error::Custom(String::from("Unauthorized")));
            }

            let events = self.psp22.mint(to, value)?;

            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        pub fn burn_from(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error> {
            if self.env().caller() != self.data.manager {
                return Err(PSP22Error::Custom(String::from("Unauthorized")));
            }

            let events = self.psp22.burn(from, value)?;

            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22 for ConditionalPSP22 {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.psp22.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.psp22.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.psp22.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: u128, _data: Vec<u8>) -> Result<(), PSP22Error> {
            let events = self.psp22.transfer(self.env().caller(), to, value)?;

            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.psp22.approve(self.env().caller(), spender, value)?;

            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.psp22.increase_allowance(self.env().caller(), spender, value)?;

            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.psp22.decrease_allowance(self.env().caller(), spender, value)?;

            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128, _data: Vec<u8>) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            if caller == self.data.router {
                let events = self.psp22.increase_allowance(from, caller, value)?;

                self.emit_events(events);
            }
            let events = self.psp22.transfer_from(caller, from, to, value)?;

            self.emit_events(events);
            Ok(())
        }
    }
}
