// Import rocket
# [macro_use]
extern crate rocket;

// Import Routes
mod routes;

// Import route functions
use routes::usage::calculate_diesel_usage_for_distance;
use routes::fail_prob::probability_of_unit_injector_fail;

// Run the server
# [launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![calculate_diesel_usage_for_distance, probability_of_unit_injector_fail])
}
