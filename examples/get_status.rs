use eskom_se_push_api::{
  constants::TOKEN_KEY,
  get_token_from_env,
  status::{EskomStatus, EskomStatusUrlBuilder},
  ureq_client::handle_ureq_response,
  Endpoint,
};

fn main() {
  match get_token_from_env(None) {
    Ok(val) => {
      let api = EskomStatusUrlBuilder::default().build().unwrap();
      // Need to import the Endpoint trait
      let response = ureq::request(api.method(), api.url().unwrap().as_str())
        .set(TOKEN_KEY, &val)
        .call();
      match handle_ureq_response::<EskomStatus>(response) {
        Ok(status) => {
          println!("{:?}", status);
        }
        Err(e) => {
          eprintln!("Error: {}", e);
        }
      }
    }
    Err(e) => panic!("Environment variable error: {}", e),
  }
}
