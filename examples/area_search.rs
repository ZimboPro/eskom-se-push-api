use eskom_se_push_api::{area_search::{AreaSearchURLBuilder, AreaSearch}, get_token_from_env, constants::TOKEN_KEY, Endpoint, ureq_client::handle_ureq_response};

fn main() {
  match get_token_from_env(None) {
    Ok(val) => {
      let api = AreaSearchURLBuilder::default().search_term("brooklyn").build().unwrap();
      // Need to import the Endpoint trait
      let response = ureq::request(api.method(), api.url().unwrap().as_str())
            .set(TOKEN_KEY, &val)
            .call();
      match handle_ureq_response::<AreaSearch>(response) {
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
