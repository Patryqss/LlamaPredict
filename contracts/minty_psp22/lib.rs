#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod minty_psp22 {
    use ink::storage_item;
    use ink::prelude::{vec::Vec, string::String};
    use psp22::{PSP22Data, PSP22Error, PSP22Event, PSP22Metadata, PSP22Mintable, PSP22};

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
    pub struct MintyData {
        pub max_mint: u128,
    }

    #[ink(storage)]
    pub struct MintyPSP22 {
        pub psp22: PSP22Data,
        pub data: MintyData,
        pub name: Option<String>,
        pub symbol: Option<String>,
        pub decimals: u8,
    }

    impl MintyPSP22 {
        #[ink(constructor)]
        pub fn new(
            max_mint: u128,
            name: String,
            symbol: String,
            decimals: u8
        ) -> Self {
            let caller = Self::env().caller();
            let psp22 = PSP22Data::new(0, caller);
            let data = MintyData {
                max_mint
            };
            Self {
                psp22,
                data,
                name: Some(name),
                symbol: Some(symbol),
                decimals
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
    }

    impl PSP22 for MintyPSP22 {
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
            let events = self.psp22.transfer_from(caller, from, to, value)?;

            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Mintable for MintyPSP22 {
        #[ink(message)]
        fn mint(&mut self, value: u128) -> Result<(), PSP22Error> {
            if value > self.data.max_mint {
                return Err(PSP22Error::Custom(String::from("MintMax1000")));
            }
            let events = self.psp22.mint(self.env().caller(), value)?;

            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Metadata for MintyPSP22 {
        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }
        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }
        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }
}
