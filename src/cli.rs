pub mod energy;
pub mod powerwall;
pub mod vehicle;

use crate::Data;

pub fn print_json<T>(data: Data<T>) {
    #[cfg(feature = "cli-pretty-json")]
    {
        use colored_json::prelude::*;
        println!("{}", data.body().to_colored_json_auto().unwrap());
    }

    #[cfg(not(feature = "cli-pretty-json"))]
    {
        println!("{}", data.body());
    }
}
