use area_info::AreaInfo;
use reqwest::Response;
use serde::{Deserialize, de::DeserializeOwned};
use status::EskomStatus;
extern crate thiserror;

pub mod status;
pub mod allowance;
pub mod area_info;
pub mod area_nearby;
pub mod area_search;
pub mod topics_nearby;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct EskomAPI {
    client: reqwest::Client
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("Bad Request")]
    ResponseError(#[from] reqwest::Error), //400
    #[error("Timeout")]
    Timeout,
    #[error("No Internet")]
    NoInternet,
    #[error("UnknownError")]
    Unknown
}

enum Endpoints {
    Status,
    AreaInfo,
    AreasNearby,
    AreasSearch,
    TopicsNearby,
    CheckAllowace
}

impl ToString for Endpoints {
    fn to_string(&self) -> String {
        match self {
            Endpoints::Status => "https://developer.sepush.co.za/business/2.0/status".to_string(),
            Endpoints::AreaInfo => "https://developer.sepush.co.za/business/2.0/area".to_string(),
            Endpoints::AreasNearby => "https://developer.sepush.co.za/business/2.0/areas_nearby".to_string(),
            Endpoints::AreasSearch => "https://developer.sepush.co.za/business/2.0/areas_search".to_string(),
            Endpoints::TopicsNearby => "https://developer.sepush.co.za/business/2.0/topics_nearby".to_string(),
            Endpoints::CheckAllowace => "https://developer.sepush.co.za/business/2.0/api_allowance".to_string(),
        }
    }
}


impl EskomAPI {
    pub fn new(token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("token", reqwest::header::HeaderValue::from_str(token).unwrap());
        EskomAPI {
            client: reqwest::Client::builder().default_headers(headers).build().unwrap()
        }
    }
    
    pub async fn status_async(&self) -> Result<EskomStatus, HttpError> {
        let t = self.client.get(Endpoints::Status.to_string()).send().await;
        return self.handle_response_async::<EskomStatus>(t).await;
    }
    
    pub async fn area_info_async(&self) -> Result<AreaInfo, HttpError> {
        let t = self.client.get(Endpoints::AreaInfo.to_string()).send().await;
        return self.handle_response_async::<AreaInfo>(t).await;
    }

    async fn handle_response_async<T: DeserializeOwned>(&self, response: Result<Response, reqwest::Error>) -> Result<T, HttpError> {
        return match response {
            Ok(resp) => {
                    let r = resp.json::<T>().await;
                    match r {
                        Ok(r) => Ok(r),
                        Err(e) => {
                            if e.is_decode() {
                                Err(HttpError::ResponseError(e))
                            } else {
                                Err(HttpError::Unknown)
                            }
                        },
                    }
            },
            Err(err) => {
                return if err.is_timeout() {
                    Err(HttpError::Timeout)
                } else if err.is_status() {
                    Err(HttpError::ResponseError(err))
            } else {
                Err(HttpError::NoInternet)
            }
        }
    }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
