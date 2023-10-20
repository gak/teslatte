# Teslatte 🚗🔋☀️☕

<a href="https://crates.io/crates/teslatte">![Crates.io](https://img.shields.io/crates/v/teslatte?style=for-the-badge&color=c02020)</a>
<a href="https://docs.rs/teslatte">![docs.rs](https://img.shields.io/docsrs/teslatte?style=for-the-badge)</a>
<a href="https://github.com/gak/teslatte#license">![MIT or Apache-2.0](https://img.shields.io/crates/l/teslatte?style=for-the-badge)</a>

> #### ⚠️ Alpha Warning! ⚠️
> This Rust crate is still in alpha stage. It is something I quickly put together if anyone needed it. I'm aiming to work on it as I need more features.
> 
> `0.1.x` will have breaking API changes. Don't rely on this project for anything important yet.

Teslatte is both a CLI and a Rust crate for interacting with the Tesla API.

A Tesla API using the `owner-api.teslamotors.com` endpoint as well as parts of the OAuth flow.

Currently, it only supports some the API.

It is fairly trivial to add in new endpoints if you feel like creating a PR. Please let me know if your PR is a massive change before spending a lot of time on it.

## References

Thanks to https://tesla-api.timdorr.com/ for their excellent reference.

Tesla API recently released [API documentation for their "Fleet API"](https://developer.tesla.com/docs/fleet-api) which appears to be similar to the Tesla Owner API.

## CLI

There is a CLI that can be used to interact with the API. Example:

```bash
$ teslatte --help
Usage: teslatte api [OPTIONS] <COMMAND>

Commands:
  vehicles      List of vehicles
  vehicle       Specific Vehicle
  energy-sites  List of energy sites
  energy-site   Specific energy site
  powerwall     Powerwall queries
  help          Print this message or the help of the given subcommand(s)

Options:
  -a, --access-token <ACCESS_TOKEN>  Access token. If not provided, will try to load from the cli.json file [env: TESLA_ACCESS_TOKEN=]
  -h, --help                         Print help
  
# Prints a URL to start the OAuth flow, then asks for the token URL, then saves the token to `cli.json`.
$ teslatte auth --save 

# Lists your vehicles:
$ teslatte api vehicles
{
  "response": [{
    "vehicle_id": 1234567890,
  }]
}

$ teslatte api vehicle 1234567890
Specific Vehicle

Usage: teslatte api vehicle <ID> <COMMAND>

Commands:
  vehicle-data             Get vehicle data
  charge-port-door-open    Open the charge port door or unlocks the cable
  charge-port-door-close   For vehicles with a motorized charge port, this closes it
  set-charge-limit         Set charge limit
  set-charging-amps        Set charge amps
  charge-standard          Set the charge limit to the standard %
  charge-max-range         Set the charge limit to the maximum %
  charge-start             Start charging
  charge-stop              Stop charging
  set-scheduled-charging   Set scheduled charging
  set-scheduled-departure  Set scheduled departure
  honk-horn                Honk!
  flash-lights             Flash the lights
  help                     Print this message or the help of the given subcommand(s)

Arguments:
  <ID>

Options:
  -h, --help  Print help
  
$ teslatte api vehicle 1234567890 vehicle_data
{ ... }

```

## Crate example

A basic example: [examples/basic.rs](examples/basic.rs)

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
