use eskom_se_push_api::EskomAPI;

fn main() {
  let api = EskomAPI::new_with_env(None);
  let resp = api.check_allowance();
  match resp {
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
