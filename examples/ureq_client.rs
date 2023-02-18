use eskom_se_push_api::area_info::{AreaInfo, AreaInfoURLBuilder};
use eskom_se_push_api::get_token_from_env;
use eskom_se_push_api::ureq_client::UreqClient;

fn main() {
  match get_token_from_env(None) {
    Ok(val) => {
      let api = UreqClient::new(val);
      match api.get_load_shedding_status() {
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
