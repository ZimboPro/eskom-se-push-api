use serde::de::DeserializeOwned;

use crate::{errors::{HttpError, APIError}, status::EskomStatus, endpoints::{get_load_shedding_status, get_area_info}, area_info::AreaInfo};


struct UreqClient {
    token: String,
}

impl UreqClient {
    pub fn new(token: String) -> Self {
        UreqClient { token }
    }

    pub fn get_load_shedding_status(&self) -> Result<EskomStatus, HttpError> {
        let url = get_load_shedding_status(&self.token);
    }

    pub fn get_area_info(&self, area_id: &str) -> Result<AreaInfo, HttpError> {
        let url = get_area_info( area_id);
        let resp = ureq::get(url.as_str()).set("token", &self.token).call();
        handle_ureq_response(resp)
    }
}

pub fn handle_ureq_response<T: DeserializeOwned>(response: Result<ureq::Response, ureq::Error>) -> Result<T, HttpError> {
    match response {
        Ok(response) => {
            response.into_json::<T>().map_err(|e| HttpError::UnknownError(e.to_string()))
        },
        Err(ureq::Error::Status(code, response)) => {
            match code {
                400 => Err(HttpError::APIError(APIError::BadRequest)),
                403 => Err(HttpError::APIError(APIError::Forbidden)),
                404 => Err(HttpError::APIError(APIError::NotFound)),
                429 => Err(HttpError::APIError(APIError::TooManyRequests)),
                500..=509 => Err(HttpError::APIError(APIError::ServerError(response.into_string().unwrap()))),
                _ => Err(HttpError::Unknown)
            }
        },
        Err(_) => { Err(HttpError::NoInternet) }
    }
}