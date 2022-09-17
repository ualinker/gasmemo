pub mod event;
pub mod vehicle;

pub mod diesel_types {
    pub use super::event::EventType;
    pub use super::vehicle::VehicleType;
}
