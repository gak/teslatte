# Teslatte 🚗🔋☕

> #### ⚠️ Alpha Warning! ⚠️
> This Rust crate is still in alpha stage. It is something I quickly put together if anyone needed it. I'm aiming to work on it as I need more features.

A Tesla API using the `owner-api.teslamotors.com` endpoint as well as "interactive" OAuth.

Currently, it only supports some the `/api/1/vehicles` endpoint, but might be expanded in the future.

It is fairly trivial to add in new endpoints if you feel like creating a PR. Please let me know if your PR is a massive change before spending a lot of time on it.

Thanks to https://tesla-api.timdorr.com/ for their excellent reference.

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
  data               Get vehicle data
  charge-state       Get charge state
  set-charge-limit   Set charge limit
  set-charging-amps  Set charge amps
  charge-start       Start charging
  charge-stop        Stop charging
  help               Print this message or the help of the given subcommand(s)

Arguments:
  <ID>

Options:
  -h, --help  Print help
  
$ teslatte api vehicle 1234567890 data
{ ... }

```

## Example

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