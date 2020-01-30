#[macro_use]
extern crate diesel;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod common;
pub(crate) mod dao;

pub mod api;
pub mod error;
pub mod provider;
pub mod requestor;
pub mod timeout;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use ya_model::activity::ACTIVITY_API;
