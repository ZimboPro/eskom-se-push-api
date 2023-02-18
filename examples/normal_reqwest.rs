use eskom_se_push_api::{allowance::{AllowanceCheckURLBuilder, Allowance, AllowanceCheck}, Endpoint, constants::TOKEN_KEY, ureq_client::handle_ureq_response};
use http::header;

fn main() {
  let api = AllowanceCheckURLBuilder::default().build().unwrap();
  // Need to import the Endpoint trait
    let mut headers = header::HeaderMap::new();
    headers.insert(TOKEN_KEY, header::HeaderValue::from_str("XXXXXXXXXXXXXXXXXXXXXXX").unwrap());
    let client = reqwest::blocking::ClientBuilder::new()
      .default_headers(headers)
      .build()
      .unwrap();
   let response = client.get(api.url().unwrap().as_str()).send();
  match handle_reqwest_response_blocking::<AllowanceCheck>(response) {
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
