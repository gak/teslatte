//! Parse the whole teslatte project and find any get*! post*! macros.

use crate::nom_help::{ignore_whitespace, quoted_string, short_trace};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_until1, take_while1};
use nom::character::complete::alpha1;
use nom::character::is_alphabetic;
use nom::combinator::opt;
use nom::{IResult, Needed};
use reqwest::Method;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct TeslatteEndpoint {
    pub method: Method,
    pub endpoint: String,
    pub uri: String,
    // pub args: Vec<String>,
    // pub post_struct: Option<String>,
}

pub fn parse(path: &Path) -> anyhow::Result<Vec<TeslatteEndpoint>> {
    let mut path = PathBuf::from(path);
    path.push("src");
    path.push("**/*.rs");

    debug!("Globbing {path:?}");

    let mut endpoints = vec![];
    let pattern = path.to_str().unwrap();
    for file in glob::glob(pattern).unwrap() {
        let path = file?;

        let append_endpoints = parse_file(&path)?;
        endpoints.extend(append_endpoints);
    }

    Ok(endpoints)
}

/// Examples
///
/// impl VehicleApi for OwnerApi {
///     get!(vehicles, Vec<Vehicle>, "/vehicles");
///     get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", VehicleId);
///     post_arg_empty!(wake_up, "/vehicles/{}/command/wake_up", VehicleId);
///
/// Another one:
///
/// impl OwnerApi {
///     pub_get_arg!(powerwall_status, PowerwallStatus, "/powerwalls/{}/status", PowerwallId);
///     pub_get_args!(powerwall_energy_history, PowerwallEnergyHistory, "/powerwalls/{}/energyhistory", PowerwallEnergyHistoryValues);
/// }
///
fn parse_file(path: &PathBuf) -> anyhow::Result<Vec<TeslatteEndpoint>> {
    debug!("Parsing file: {path:?}");
    let content = read_to_string(path)?;

    let mut endpoints = vec![];
    let mut inside_owner_api = false;

    for line in content.lines() {
        let line = line.trim();
        trace!(?line);

        let owner_api_start = is_owner_api_start(line);
        if owner_api_start {
            if inside_owner_api {
                panic!("Nested OwnerApi")
            }

            trace!("Found OwnerApi");
            inside_owner_api = true;
            continue;
        }

        if line == "}" && inside_owner_api {
            trace!("End OwnerApi");
            inside_owner_api = false;
            continue;
        }

        if !inside_owner_api {
            continue;
        }

        trace!("Looking at line: {line:?}");
        let (_, maybe_endpoint) =
            opt(alt((get, get_arg, get_args, post_arg, post_arg_empty)))(line).unwrap();
        if let Some(endpoint) = maybe_endpoint {
            endpoints.push(endpoint);
        }
    }

    Ok(endpoints)
}

fn is_owner_api_start(line: &str) -> bool {
    line.ends_with("OwnerApi {")
}

fn macro_fn_name_then_comma(expected_tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    return move |s: &str| -> IResult<&str, &str> {
        short_trace("common macro", s);
        let (s, _) = ignore_whitespace(s)?;
        let (s, _) = tag(expected_tag)(s)?;
        let (s, _) = tag("(")(s)?;
        let (s, fn_name) = function_name(s)?;
        let (s, ()) = comma(s)?;

        Ok((s, fn_name))
    };
}

// get!(vehicles, Vec<Vehicle>, "/vehicles");
// pub_get!(vehicles, Vec<Vehicle>, "/vehicles");
fn get(s: &str) -> IResult<&str, TeslatteEndpoint> {
    let (s, fn_name) = alt((
        macro_fn_name_then_comma("get!"),
        macro_fn_name_then_comma("pub_get!"),
    ))(s)?;

    let (s, response_type) = struct_name(s)?;
    let (s, ()) = comma(s)?;
    let (s, uri) = quoted_string(s)?;
    let (s, _) = end_args(s)?;

    let endpoint = TeslatteEndpoint {
        method: Method::GET,
        endpoint: fn_name.to_string(),
        uri: uri.to_string(),
    };

    Ok((s, endpoint))
}

// get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", VehicleId);
fn get_arg(s: &str) -> IResult<&str, TeslatteEndpoint> {
    let (s, fn_name) = alt((
        macro_fn_name_then_comma("get_arg!"),
        macro_fn_name_then_comma("pub_get_arg!"),
    ))(s)?;
    let (s, response_type) = struct_name(s)?;
    let (s, ()) = comma(s)?;
    let (s, uri) = quoted_string(s)?;
    let (s, ()) = comma(s)?;
    let (s, arg_type) = struct_name(s)?;
    let (s, _) = end_args(s)?;

    let endpoint = TeslatteEndpoint {
        method: Method::GET,
        endpoint: fn_name.to_string(),
        uri: uri.to_string(),
    };

    Ok((s, endpoint))
}

// pub_get_args!(powerwall_energy_history, PowerwallEnergyHistory, "/powerwalls/{}/energyhistory", PowerwallEnergyHistoryValues);
fn get_args(s: &str) -> IResult<&str, TeslatteEndpoint> {
    let (s, fn_name) = alt((
        macro_fn_name_then_comma("get_args!"),
        macro_fn_name_then_comma("pub_get_args!"),
    ))(s)?;
    let (s, response_type) = struct_name(s)?;
    let (s, ()) = comma(s)?;
    let (s, uri) = quoted_string(s)?;
    let (s, ()) = comma(s)?;
    let (s, arg_type) = struct_name(s)?;
    let (s, _) = end_args(s)?;

    let endpoint = TeslatteEndpoint {
        method: Method::GET,
        endpoint: fn_name.to_string(),
        uri: uri.to_string(),
    };

    Ok((s, endpoint))
}

// post_arg!(set_charge_limit, SetChargeLimit, "/vehicles/{}/command/set_charge_limit", VehicleId);
fn post_arg(s: &str) -> IResult<&str, TeslatteEndpoint> {
    let (s, fn_name) = alt((
        macro_fn_name_then_comma("post_arg!"),
        macro_fn_name_then_comma("pub_post_arg!"),
    ))(s)?;
    let (s, response_type) = struct_name(s)?;
    let (s, ()) = comma(s)?;
    let (s, uri) = quoted_string(s)?;
    let (s, ()) = comma(s)?;
    let (s, arg_type) = struct_name(s)?;
    let (s, _) = end_args(s)?;

    let endpoint = TeslatteEndpoint {
        method: Method::POST,
        endpoint: fn_name.to_string(),
        uri: uri.to_string(),
    };

    Ok((s, endpoint))
}

// post_arg_empty!(charge_port_door_open, "/vehicles/{}/command/charge_port_door_open", VehicleId);
// post_arg_empty!(charge_port_door_close, "/vehicles/{}/command/charge_port_door_close", VehicleId);
fn post_arg_empty(s: &str) -> IResult<&str, TeslatteEndpoint> {
    let (s, fn_name) = alt((
        macro_fn_name_then_comma("post_arg_empty!"),
        macro_fn_name_then_comma("pub_post_arg_empty!"),
    ))(s)?;
    let (s, uri) = quoted_string(s)?;
    let (s, ()) = comma(s)?;
    let (s, arg_type) = struct_name(s)?;
    let (s, _) = end_args(s)?;

    let endpoint = TeslatteEndpoint {
        method: Method::POST,
        endpoint: fn_name.to_string(),
        uri: uri.to_string(),
    };

    Ok((s, endpoint))
}

fn function_name(s: &str) -> IResult<&str, &str> {
    take_while1(is_function_chars)(s)
}

fn struct_name(s: &str) -> IResult<&str, &str> {
    let (s, name) = take_while1(is_type)(s)?;

    Ok((s, name))
}

fn is_function_chars(c: char) -> bool {
    is_lower_alpha(c) || c == '_'
}

fn is_lower_alpha(c: char) -> bool {
    c.is_ascii_lowercase()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_type(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '<' || c == '>'
}

fn comma(s: &str) -> IResult<&str, ()> {
    let (s, _) = tag(",")(s)?;
    let (s, _) = ignore_whitespace(s)?;

    Ok((s, ()))
}

fn end_args(s: &str) -> IResult<&str, ()> {
    let (s, _) = tag(");")(s)?;
    Ok((s, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let s = r#"get!(vehicles, Vec<Vehicle>, "/vehicles");"#;
        let (_, endpoint) = get(s).unwrap();
    }

    #[test]
    fn test_pub_get() {
        let s = r#"pub_get!(vehicles, Vec<Vehicle>, "/vehicles");"#;
        let (_, endpoint) = get(s).unwrap();
    }

    #[test]
    fn test_get_arg() {
        let s = r#"get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", VehicleId);"#;
        let (_, endpoint) = get_arg(s).unwrap();
    }

    #[test]
    fn test_pub_get_arg() {
        let s =
            r#"pub_get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", VehicleId);"#;
        let (_, endpoint) = get_arg(s).unwrap();
    }

    #[test]
    fn test_get_args() {
        let s = r#"get_args!(powerwall_energy_history, PowerwallEnergyHistory, "/powerwalls/{}/energyhistory", PowerwallEnergyHistoryValues);"#;
        let (_, endpoint) = get_args(s).unwrap();
    }

    #[test]
    fn test_pub_get_args() {
        let s = r#"pub_get_args!(powerwall_energy_history, PowerwallEnergyHistory, "/powerwalls/{}/energyhistory", PowerwallEnergyHistoryValues);"#;
        let (_, endpoint) = get_args(s).unwrap();
    }

    // post_arg!(set_charge_limit, SetChargeLimit, "/vehicles/{}/command/set_charge_limit", VehicleId);
    #[test]
    fn test_post_arg() {
        let s = r#"post_arg!(set_charge_limit, SetChargeLimit, "/vehicles/{}/command/set_charge_limit", VehicleId);"#;
        let (_, endpoint) = post_arg(s).unwrap();
    }

    #[test]
    fn test_post_arg_empty() {
        let s = r#"post_arg_empty!(wake_up, "/vehicles/{}/command/wake_up", VehicleId);"#;
        let (_, endpoint) = post_arg_empty(s).unwrap();
    }
}
