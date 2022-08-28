// Import dependencies
use rocket::{http::Status, serde::json::Json};
use rand::Rng;

// Function to genereate a random number between 0 and 1 non-inclusive
fn gen_random() -> f32 {
	let mut rng = rand::thread_rng();
	f32::trunc(rng.gen_range(0.01..1.00) * 100.0) / 100.0
}

// Route to get probability of failure
# [get("/probabilityOfUnitInjectorFail?<VIN>")]
pub fn probability_of_unit_injector_fail(VIN: String) -> Result<Json<f32>, Status> {
    Ok(
		Json(
			gen_random()
		)
	)
}
