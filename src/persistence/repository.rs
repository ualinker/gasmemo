use crate::persistence::model::vehicle::{NewVehicle, Vehicle};
use crate::AppConfig;
use diesel::prelude::*;
use diesel::{insert_into, QueryResult, RunQueryDsl};

pub struct Repository<'a> {
    config: &'a AppConfig,
}

impl<'a, 'b> Repository<'a> {
    pub fn new(config: &'a AppConfig) -> Self {
        Repository { config }
    }

    pub fn insert_vehicle(&self, vehicle: NewVehicle) -> QueryResult<usize> {
        insert_into(crate::persistence::schema::vehicle::table)
            .values(vehicle)
            .execute(&self.config.db)
    }

    pub fn load_vehicle(&self) -> Result<Option<Vehicle>, diesel::result::Error> {
        use crate::persistence::schema::vehicle::dsl::*;

        vehicle.first(&self.config.db).optional()
    }
}
