//! # Eskom-se-Push API
//!
//! The library is an unofficial lib and is not maintained by the API developers.
//!
//! This library is an API binding to the [EskomSePush](https://sepush.co.za) API.
//!
//! It does have a few small helper functions to assist.
//!
//! To get up and running, you can either pass in the API key or use environment variables.
//!
//! ## API key as a variable
//!
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new("XXXXXXXXXXXXXXXXXXXXXXXXX");
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```
//!
//! ## API key as an env variable
//!
//! The default env variable is `ESKOMSEPUSH_API_KEY`
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new_with_env(None);
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```
//!
//! //!
//! ## API key as an custom env variable
//!
//! Able to use custom env keys such as `MY_CUSTOM_KEY`
//! ```rust
//! use eskom_se_push_api::EskomAPI;
//!
//! fn main() {
//!   let api = EskomAPI::new_with_env(Some("MY_CUSTOM_KEY"));
//!   let resp = api.check_allowance();
//!   match resp {
//!     Ok(allowance) => {
//!       println!(
//!         "You have made {} API calls today",
//!         allowance.allowance.count
//!       );
//!     }
//!     Err(e) => {
//!       eprintln!("Error: {}", e);
//!     }
//!   }
//! }
//! ```
//!
//! ## Features
//!
//! There are currently 4 features but some are used in combinations to enable certain functionality
//!
//! * `reqwest` and `async`: Adds an async reqwest client and response handler
//! 
//! * `reqwest` and `sync`: Adds a blocking reqwest client and response handler
//! 
//! * `ureq`: Adds a ureq client and response handler
//!
//! None of the features are added by default

pub use traits::Endpoint;
#[cfg(any(feature = "async",doc))]
pub use traits::EndpointAsync;
extern crate thiserror;

pub mod allowance;
pub mod area_info;
pub mod area_nearby;
pub mod area_search;
pub mod constants;
pub mod errors;
#[cfg(any(all(feature = "async", feature = "reqwest"),doc))]
pub mod reqwest_async_client;
#[cfg(any(all(feature = "sync", feature = "reqwest"),doc))]
pub mod reqwest_blocking_client;
pub mod status;
pub mod topics_nearby;
mod traits;
#[cfg(any(feature = "ureq",doc))]
pub mod ureq_client;

pub fn get_token_from_env(var_name: Option<&str>) -> Result<String, std::env::VarError>  {
  dotenv::dotenv().ok();
  let key = var_name.unwrap_or("ESKOMSEPUSH_API_KEY");
  std::env::var(key)
}

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn it_works() {
    // let result = add(2, 2);
    // assert_eq!(result, 4);
  }
}
