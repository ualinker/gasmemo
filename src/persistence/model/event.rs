use diesel::*;

use crate::persistence::schema::event;

#[derive(Debug, Clone, DbEnum)]
pub enum EventType {
    Refuel,
    Service,
}

#[derive(Queryable)]
pub struct Event {
    id: usize,
    vehicle_id: usize,
    type_: EventType,
    odometer: u32,
    datetime: String,
    price: f32,
}

#[derive(Insertable)]
#[table_name = "event"]
pub struct NewEvent {
    pub vehicle_id: i32,
    pub type_: EventType,
    pub datetime: String,
    pub price: f32,
}
