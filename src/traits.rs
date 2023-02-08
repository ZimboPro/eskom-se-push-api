use std::borrow::Cow;
use std::error::Error;
use async_trait::async_trait;
use bytes::Bytes;
use http::{self, header, Method, Request, Uri};
use serde::de::DeserializeOwned;
use crate::APIError;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

trait Endpoint {
    fn method(&self) -> Method;
    fn endpoint(&self) -> Cow<'static, str>;
    fn parameters(&self) -> QueryParams {
        QueryParams::default() // Many endpoints don't have parameters
    }
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None) // Many endpoints also do not have request bodies
    }
}

trait Client {
    type Error: Error + Send + Sync + 'static;
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, APIError<Self::Error>>;
    fn rest(&self, request: RequestBuilder, body: Vec<u8>) -> Result<Response<Bytes>, APIError<Self::Error>>;
}

trait Query<T, C> {
    fn query(&self, client: &C) -> Result<T, APIError<C::Error>>;
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, APIError<C::Error>> {
        // compute the URL
        // add query parameters and the body to the request
        // send off the request
        // check the response status and extract errors if needed

        serde_json::from_value::<T>(v).map_err(APIError::data_type::<T>)
    }
}

