use thiserror::Error;

#[derive(Error, Debug)]
pub enum TicketError {
    #[error("The title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("The description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    ParseStatusError(#[from] crate::status::ParseStatusError),
}
