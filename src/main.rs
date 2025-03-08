use enums::{Country, USAState};
use models::Location;

mod enums;
mod models;

fn main() {
    let location = Location::new(
        "house marshellyan",
        32.9321,
        -97.2834,
        Some("13128"),
        "ridgepointe rd",
        "Fort Worth",
        Some(USAState::TX),
        Country::UnitedStates,
        "76244",
        Some("CST"),
        None,
        None,
        None,
    );

    println!("{:#?}", location);
}
