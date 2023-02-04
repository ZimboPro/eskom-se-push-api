use eskom_se_push_api::EskomAPI;

fn main() {
  let api = EskomAPI::new_with_env(None);
  let resp = api.area_info("tshwane-6-brooklyn");
  match resp {
    Ok(area_info) => {
      println!("{:?}", area_info);
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
