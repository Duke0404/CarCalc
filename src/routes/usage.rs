// Import dependencies
use rocket::{http::Status, serde::json::Json};

// Function to get fuel usage according to distance
fn fuel_usage(distance: f32, fuel: f32) -> f32 {
    distance * fuel
}

// Route to get fuel usage
# [get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
pub fn calculate_diesel_usage_for_distance(distance: f32, yearOfProduction: u16, fuelUsagePer100KM: f32) -> Result<Json<f32>, Status> {
    Ok(
        Json(
            fuel_usage(distance, fuelUsagePer100KM / 100.0)
        )
    )
}
