create table vehicle (
    id integer primary key autoincrement not null,
    vehicle_type text  CHECK(vehicle_type IN ("car", "motorcycle")) not null,
    make text not null,
    model text not null,
    year integer not null,
    odometer integer not null
)