table! {
    vehicle (id) {
        id -> Integer,
        vehicle_type -> crate::persistence::model::vehicle::VehicleTypeMapping,
        make -> Text,
        model -> Text,
        year -> Integer,
        odometer -> Integer,
    }
}

table! {
    event (id) {
        id -> Integer,
        vehicle_id -> Integer,
        #[sql_name="type"]
        type_ -> crate::persistence::model::event::EventTypeMapping,
        odometer -> Integer,
        datetime -> Text,
        price -> Float,
    }
}

joinable!(event -> vehicle(vehicle_id));
