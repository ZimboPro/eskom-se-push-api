use eskom_se_push_api::{
  allowance::{AllowanceCheck, AllowanceCheckURLBuilder},
  constants::TOKEN_KEY,
  ureq_client::handle_ureq_response,
  Endpoint,
};

fn main() {
  let api = AllowanceCheckURLBuilder::default().build().unwrap();
  // Need to import the Endpoint trait
  let response = ureq::request(api.method(), api.url().unwrap().as_str())
    .set(TOKEN_KEY, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
    .call();
  match handle_ureq_response::<AllowanceCheck>(response) {
    Ok(allowance) => {
      println!(
        "You have made {} API calls today",
        allowance.allowance.count
      );
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
