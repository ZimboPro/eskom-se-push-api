use eskom_se_push_api::EskomAPI;

fn main() {
  let api = EskomAPI::new_with_env(None);
  let resp = api.areas_search("brooklyn");
  match resp {
    Ok(area) => {
      println!("{:?}", area);
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
