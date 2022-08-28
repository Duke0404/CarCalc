// Import rocket & types
# [macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json};
use rand::Rng;

// Import calculate_diesel_usage_for_distance
use routes::calculate_diesel_usage_for_distance::calculate_diesel_usage_for_distance;

# [get("/probabilityOfUnitInjectorFail?<VIN>")]
fn probability_of_unit_injector_fail(VIN: String) -> Result<Json<f64>, Status> {
    let mut rng = rand::thread_rng();

    Ok(Json(f64::trunc(rng.gen_range(0.01..1.00) * 100.0) / 100.0))
}

// Run the server
# [launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![calculate_diesel_usage_for_distance, probability_of_unit_injector_fail])
}
