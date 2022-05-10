use cosmwasm_std::Storage;

pub trait WritableStorage<'a> {
    fn get_storage(&'a mut self) -> &'a mut dyn Storage;
}

pub trait CommandHandler<'a>: WritableStorage<'a> {
    type Props;

    type Response;

    type Error;

    fn execute(&mut self, props: Self::Props) -> Result<Self::Response, Self::Error>;
}

trait QueryHandler {}

trait CallbackHandler {}

trait UpgradeHandler {}

pub struct ContractModule<'a> {
    storage: &'a mut dyn Storage,
}

#[derive(Debug, PartialEq)]
pub enum CommandResponse {
    ExampleUpdateResponse { is_active: bool },
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    ExampleUpdateError {},
}

pub enum CommandMsg {
    ExampleUpdate { is_active: bool },
    ExampleFailingUpdate { is_active: bool },
}

impl<'a> WritableStorage<'a> for ContractModule<'a> {
    fn get_storage(&'a mut self) -> &'a mut dyn Storage {
        self.storage
    }
}

impl<'a> CommandHandler<'a> for ContractModule<'a> {
    type Props = CommandMsg;

    type Response = CommandResponse;

    type Error = CommandError;

    fn execute(&mut self, props: Self::Props) -> Result<Self::Response, Self::Error> {
        match props {
            CommandMsg::ExampleUpdate { is_active } => {
                Ok(CommandResponse::ExampleUpdateResponse { is_active })
            }
            CommandMsg::ExampleFailingUpdate { is_active: _ } => {
                Err(CommandError::ExampleUpdateError {})
            }
        }
    }
}

#[cfg(test)]
mod contratct_interface {
    use cosmwasm_std::testing::mock_dependencies;

    use crate::{CommandError, CommandHandler, CommandMsg, CommandResponse, ContractModule};

    #[test]
    fn contract_module_can_execute_a_command() {
        let mut deps = mock_dependencies(&[]);
        let mut contract_module = ContractModule {
            storage: deps.as_mut().storage,
        };

        let response = contract_module
            .execute(CommandMsg::ExampleUpdate { is_active: true })
            .unwrap();

        assert_eq!(
            CommandResponse::ExampleUpdateResponse { is_active: true },
            response
        );

        let response = contract_module
            .execute(CommandMsg::ExampleFailingUpdate { is_active: true })
            .unwrap_err();

        assert_eq!(CommandError::ExampleUpdateError {}, response);
    }
}
