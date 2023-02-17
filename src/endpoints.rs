use http::Request;
use reqwest::Url;

use crate::{status::EskomStatus, area_info::AreaInfo, area_search::AreaSearch, topics_nearby::TopicsNearby, area_nearby::AreaNearby, Endpoints, traits::Endpoint};

pub fn get_load_shedding_status(token: &str) -> Request<EskomStatus> {
    Request::builder()
    .method("GET")
    .uri("https://developer.sepush.co.za/business/2.0/status")
    .header("token", token).body(())
    .unwrap()
}

pub fn get_areas_search(&token: &str, search_term: &str) -> Request<AreaSearch> {
    Request::builder()
    .method("GET")
    .uri(format!("https://developer.sepush.co.za/business/2.0/areas_search?text={search_term}"))
    .header("token", token)
    .body(())
    .unwrap()
}

pub fn get_areas_nearby(&token: &str, lat: f32, long: f32) -> Request<AreaNearby> {
    Request::builder()
    .method("GET")
    .uri(format!("https://developer.sepush.co.za/business/2.0/areas_nearby?lat={lat}&long={long}"))
    .header("token", token)
    .body(())
    .unwrap()
}

pub fn get_topics_search(token: &str, lat: f32, long: f32) -> Request<TopicsNearby> {
    Request::builder()
    .method("GET")
    .uri(format!("https://developer.sepush.co.za/business/2.0/topics_nearby?lat={lat}&long={long}"))
    .header("token", token)
    .body(())
    .unwrap()
}

pub fn check_allowance(token: &str) -> Request<EskomStatus> {
    Request::builder()
    .method("GET")
    .uri("https://developer.sepush.co.za/business/2.0/api_allowance")
    .header("token", token).body(())
    .unwrap()
}