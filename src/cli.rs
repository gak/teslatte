pub mod energy;
pub mod powerwall;
pub mod vehicle;

use crate::error::TeslatteError;
use crate::Data;
use std::process::exit;

pub fn print_json<T>(result: Result<Data<T>, TeslatteError>) {
    match result {
        Ok(data) => print_json_data(data),
        Err(TeslatteError::ServerError { ref body, .. }) if body.is_some() => {
            print_json_str(&body.clone().unwrap())
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
pub fn print_json_data<T>(data: Data<T>) {
    // TODO: pretty print cli option
    print_json_str(data.body());
}

pub fn print_json_str(body: &str) {
    #[cfg(feature = "cli-pretty-json")]
    {
        use colored_json::prelude::*;
        println!("{}", body.to_colored_json_auto().unwrap());
    }

    #[cfg(not(feature = "cli-pretty-json"))]
    {
        println!("{}", body);
    }
}
