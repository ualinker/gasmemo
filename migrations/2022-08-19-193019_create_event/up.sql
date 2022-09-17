create table event
(
    id         integer primary key autoincrement          not null,
    vehicle_id integer                                    not null,
    type       text CHECK (type IN ("refuel", "service")) not null,
    odometer   integer                                    not null,
    datetime   text                                       not null,
    price      float                                      not null,
    foreign key (vehicle_id) references vehicle (id)
)