// Import status & json
use rocket::{http::Status, serde::json::Json};

# [get("/calculateDisselUsageForDistance?<distance>&<year_of_production>&<fuel_usage_per100_km>")]
pub fn calculate_diesel_usage_for_distance(distance: f64, year_of_production: u16, fuel_usage_per100_km: f64) -> Result<Json<f64>, Status> {
    Ok(Json(distance * fuel_usage_per100_km / 100.0))
}