use diesel::*;

use crate::persistence::schema::vehicle;

#[derive(Debug, Clone, DbEnum)]
pub enum VehicleType {
    Car,
    Motorcycle,
}

#[derive(Queryable)]
pub struct Vehicle {
    pub id: i32,
    pub vehicle_type: VehicleType,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub odometer: i32,
}

#[derive(Insertable)]
#[table_name = "vehicle"]
pub struct NewVehicle {
    pub vehicle_type: VehicleType,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub odometer: i32,
}

impl Vehicle {
    pub fn make(&self) -> String {
        self.make.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}
