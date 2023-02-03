use eskom_se_push_api::EskomAPI;

fn main() {
  let api = EskomAPI::new_with_env(None);
  let resp = api.status();
  match resp {
    Ok(status) => {
      println!("{:?}", status);
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
