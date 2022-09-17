mod io;

use crate::persistence::model::vehicle::{NewVehicle, Vehicle};
use crate::persistence::repository::Repository;
use crate::{config::AppConfig, persistence::model::vehicle::VehicleType};
use chrono::{Datelike, Local as Date};
use diesel::QueryResult;
use i18n_embed_fl::fl;
use io::IO;

use self::io::ChoiceList;

pub struct Cli<'a> {
    config: &'a AppConfig,
    io: IO<'a>,
    repository: Repository<'a>,
    current_vehicle: Option<Vehicle>,
}

impl<'a, 'b> Cli<'a> {
    pub fn new(config: &'a AppConfig) -> Cli<'a> {
        let repository = Repository::new(config);

        let current_vehicle = match repository.load_vehicle() {
            Ok(vehicle) => vehicle,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        };

        Cli {
            config,
            io: IO::new(config),
            repository,
            current_vehicle,
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            if let Some(v) = self.current_vehicle.as_ref() {
                println!("{} {} {}", v.make(), v.model(), v.year());
            }

            let choices: ChoiceList<()> = vec![
                (
                    fl!(self.config.translator, "action-add-vehicle"),
                    Box::new(|| self.add_vehicle()),
                ),
                (
                    fl!(self.config.translator, "action-refuel"),
                    Box::new(|| self.refuel()),
                ),
                (
                    fl!(self.config.translator, "action-service"),
                    Box::new(|| self.service()),
                ),
            ];

            self.io.choice(choices, "");
        }
    }

    fn add_vehicle(&self) {
        let choices: ChoiceList<VehicleType> = vec![
            (
                fl!(self.config.translator, "type-auto"),
                Box::new(|| VehicleType::Car),
            ),
            (
                fl!(self.config.translator, "type-moto"),
                Box::new(|| VehicleType::Motorcycle),
            ),
        ];

        let vehicle_type = self
            .io
            .choice(choices, &fl!(self.config.translator, "choose-vehicle-type"));

        let make = IO::read_val(
            &fl!(self.config.translator, "make"),
            &fl!(self.config.translator, "error-input-make"),
            |make| if make.is_empty() { Err("") } else { Ok(make) },
        );
        let model = IO::read_val(
            &fl!(self.config.translator, "model"),
            &fl!(self.config.translator, "error-input-model"),
            |model| if model.is_empty() { Err("") } else { Ok(model) },
        );
        let valid_years = 1900..Date::now().year() + 1;
        let year = IO::read_val(
            &fl!(self.config.translator, "production-year"),
            &fl!(
                self.config.translator,
                "error-input-year",
                yearMin = valid_years.start,
                yearMax = valid_years.end
            ),
            |year| match year.parse::<i32>() {
                Ok(year) => {
                    if valid_years.contains(&year) {
                        Ok(year)
                    } else {
                        Err("")
                    }
                }
                Err(_) => Err(""),
            },
        );
        let odometer = IO::read_val(
            &format!("{}: ", fl!(self.config.translator, "current-odo")),
            &fl!(self.config.translator, "error-input-odo"),
            |odo| match odo.parse::<i32>() {
                Ok(odo) => {
                    if odo >= 0 {
                        Ok(odo)
                    } else {
                        Err("")
                    }
                }
                Err(_) => Err(""),
            },
        );
        let vehicle = NewVehicle {
            vehicle_type,
            make,
            model,
            year,
            odometer,
        };
        if let Err(e) = self.repository.insert_vehicle(vehicle) {
            println!("ERROR: {:?}", e);
        } else {
            println!("{}", fl!(self.config.translator, "success-vehicle-added"));
        }
    }

    fn refuel(&self) {
        // @todo replace with Fluent keywords
        let odo = IO::read_val(
            "Одометр (км): ",
            "Не можу розпізнати значення одометра",
            |odo| odo.parse::<u32>(),
        );
        let amount = IO::read_val(
            "Заправлено (л): ",
            "Не можу розпізнати обʼєм заправленого палива",
            |amount| amount.parse::<u16>(),
        );
        println!("{} км, {} л", odo, amount)
    }

    fn service(&self) {}
}

trait DbErrorHandler {
    fn handle_error(&self);
}

impl<T> DbErrorHandler for QueryResult<T> {
    fn handle_error(&self) {
        if let Err(e) = &self {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }
}
