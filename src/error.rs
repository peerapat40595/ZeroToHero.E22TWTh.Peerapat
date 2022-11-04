use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Poll does not exist")]
    PollNotFound {},

    #[error("Vote Option does not exist")]
    VoteOptionNotFound {},

    #[error("Insufficient funds")]
    InsufficientFunds {},
}
