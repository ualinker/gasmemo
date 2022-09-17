use i18n_embed_fl::fl;
use std::io::{self, Write};

use crate::config::AppConfig;

pub struct IO<'a> {
    config: &'a AppConfig,
}

pub type ChoiceList<'a, T> = Vec<(String, Box<dyn Fn() -> T + 'a>)>;

impl<'a> IO<'a> {
    pub fn new(config: &'a AppConfig) -> IO<'a> {
        IO { config: &config }
    }

    pub fn readline(msg: &str) -> String {
        let mut buf = String::from("");
        print!("{} > ", msg);
        io::stdout().flush().expect("can not flush STDOUT");
        io::stdin()
            .read_line(&mut buf)
            .expect("can not read from STDIN");
        buf.trim().to_string()
    }

    // continue to read user input until it matches with parser_fn
    pub fn read_val<F, T, U>(label: &str, error_msg: &str, parser_fn: F) -> T
    where
        F: Fn(String) -> Result<T, U>,
    {
        loop {
            let str = IO::readline(label);
            let parsed = parser_fn(str);
            if let Ok(result) = parsed {
                return result;
            }
            println!("{}", error_msg);
        }
    }

    // Output a numbered choice list, input correct list item number, execute associated function
    pub fn choice<T>(&self, choices: ChoiceList<T>, input_label: &str) -> T {
        let list_size = choices.len();
        for (idx, (label, _)) in choices.iter().enumerate() {
            println!("{}. {}", idx + 1, label);
        }

        let err_msg = fl!(self.config.translator, "error-list", listSize = list_size);

        let choice = IO::read_val(input_label, &err_msg, |choice| {
            match choice.parse::<usize>() {
                Ok(choice) => {
                    return if choice >= 1 && choice <= list_size {
                        Ok(choice)
                    } else {
                        Err("")
                    }
                }
                _ => Err(""),
            }
        });

        let (_, f) = choices.get(choice - 1).unwrap();
        f()
    }
}
