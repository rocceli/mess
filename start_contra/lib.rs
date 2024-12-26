#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod start_contra {
    use scale::{Decode, Encode};

    #[derive(Decode, Encode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        AlreadySet,
    }

    #[ink(storage)]
    pub struct StartContra {
        value: bool,
    }

    impl StartContra {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: false }
        }

        #[ink(message)]
        pub fn flip(&mut self) -> Result<(), ContractError> {
            if self.value {
                return Err(ContractError::AlreadySet);
            }
            self.value = !self.value;
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let start_contra = StartContra::default();
            assert_eq!(start_contra.get(), false);
        }

        #[ink::test]
        fn flip_works() {
            let mut start_contra = StartContra::new(false);
            assert!(start_contra.flip().is_ok());
            assert_eq!(start_contra.get(), true);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = StartContraRef::default();
            let contract = client
                .instantiate("start_contra", &ink_e2e::alice(), &constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let get = contract.call_builder::<StartContra>().get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            
            assert!(matches!(get_result.return_value(), false));
            Ok(())
        }
    }
}