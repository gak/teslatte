# Teslatte 🚗🔋☕

> :warning: **Alpha Warning!** This Rust crate is still in alpha stage. It is something I put together if anyone needs it, and I'm aiming to work on it as I need more features.

A Tesla API using the `owner-api.teslamotors.com` endpoint as well as "interactive" OAuth.

Currently, it only supports some the `/api/1/vehicles` endpoint, but it will be expanded in the future.

It is fairly trivial to add in new endpoints if you feel like creating a PR. Please let me know if your PR is a massive change before spending a lot of time on it.

## Example

A basic example: [examples/basic.rs](examples/basic.rs)

## Endpoints

Here's a lazy dump of the endpoints I've implemented so far:

```rust
    get!(vehicles, Vec<Vehicle>, "");
    get_v!(vehicle_data, VehicleData, "/vehicle_data");
    get_v!(charge_state, ChargeState, "/data_request/charge_state");
    post_vd!(set_charge_limit, SetChargeLimit, "/command/set_charge_limit");
    post_vd!(set_charging_amps, SetChargingAmps, "/command/set_charging_amps");
    post_v!(charge_start, "/command/charge_start");
    post_v!(charge_stop, "/command/charge_stop");
```

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