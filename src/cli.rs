use crate::Data;

pub mod calendar_history;
pub mod energy;
pub mod powerwall;
pub mod vehicle;

pub fn print_json<T>(data: Data<T>) {
    #[cfg(feature = "cli-pretty-json")]
    {
        use colored_json::prelude::*;
        println!("{}", data.body().to_colored_json_auto().unwrap());
    }

    #[cfg(not(feature = "cli-pretty-json"))]
    {
        println!("{:#?}", data.body());
        panic!();
    }
}
