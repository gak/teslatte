use crate::nom_help;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashMap;
use tracing::{debug, trace};

pub fn parse(s: &str) -> HashMap<String, VehicleCommandEndpoint> {
    // Seek all the way to: var commands = map[string]*Command{\n
    // Afterwards has the first map entry.
    let (s, _) = seek_to_map(s).unwrap();

    let (_, entries) = many1(map_entry)(s).unwrap();

    entries
        .into_iter()
        .map(|e| (e.endpoint.clone(), e))
        .collect()
}

pub fn seek_to_map(s: &str) -> IResult<&str, ()> {
    nom_help::short_trace("seek to map", s);
    let tag_str = "var commands = map[string]*Command{\n";
    // There's gotta be a nom function to these two lines.
    let (s, _) = take_until(tag_str)(s)?;
    let (s, _) = tag(tag_str)(s)?;
    nom_help::short_trace("seek to map done", s);
    Ok((s, ()))
}

#[derive(Debug, Clone)]
pub struct VehicleCommandEndpoint {
    pub endpoint: String,
    pub help: String,
    pub requires_auth: bool,
    pub requires_fleet: bool,
}

fn map_entry(s: &str) -> IResult<&str, VehicleCommandEndpoint> {
    // "unlock": &Command{
    // 	help:             "Unlock vehicle",
    // 	requiresAuth:     true,
    // 	requiresFleetAPI: false,
    //  args: []Argument{
    //      Argument{name: "TEMP", help: "Desired temperature (e.g., 70f or 21c; defaults to Celsius)"},
    //      Argument{name: "ROLE", help: "One of: owner, driver"},
    //  },
    //  handler: func(ctx context.Context, acct *account.Account, car *vehicle.Vehicle, args map[string]string) error {
    //      return car.Unlock(ctx)
    //  },
    // },

    nom_help::short_trace("--- map entry ---", s);

    // endpoint
    nom_help::short_trace("endpoint", s);
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, endpoint) = nom_help::quoted_string(s)?;
    let (s, _) = nom_help::until_eol(s)?;

    // help
    nom_help::short_trace("help", s);
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("help:")(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, help) = nom_help::quoted_string(s)?;
    let (s, _) = tag(",")(s)?;

    // requiresAuth
    nom_help::short_trace("requiresAuth", s);
    let (s, requires_auth) = bool_field_or_false(s, "requiresAuth:")?;

    // requiresFleetAPI
    nom_help::short_trace("requiresFleetAPI", s);
    let (s, requires_fleet) = bool_field_or_false(s, "requiresFleetAPI:")?;

    // Required args
    nom_help::short_trace("required args", s);
    let (s, required_args) = args(s, "args: []Argument{")?;

    // Optional args
    nom_help::short_trace("optional args", s);
    let (s, optional_args) = args(s, "optional: []Argument{")?;

    // Ignore the handler, as there's not really much data we can take out of it.
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = take_until("},")(s)?;
    let (s, _) = tag("},")(s)?;

    // And the end of the record...
    let (s, _) = take_until("},")(s)?;
    let (s, _) = tag("},")(s)?;

    let map_entry = VehicleCommandEndpoint {
        endpoint: endpoint.to_string(),
        help: help.to_string(),
        requires_auth,
        requires_fleet,
    };
    debug!(?map_entry);

    Ok((s, map_entry))
}

fn str_to_bool(s: &str) -> IResult<&str, bool> {
    nom_help::short_trace("bool", s);
    let (s, bool_str) = alt((tag("true"), tag("false")))(s)?;
    let bool = match bool_str {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    };
    nom_help::short_trace("bool afterwards", s);
    Ok((s, bool))
}

/// If the field isn't there, assume false.
fn bool_field<'a>(field_tag: &str) -> impl Fn(&'a str) -> IResult<&'a str, bool> + '_ {
    return move |s: &str| -> IResult<&'a str, bool> {
        let (s, _) = nom_help::ignore_whitespace(s)?;
        let (s, _) = tag(field_tag)(s)?;
        let (s, _) = nom_help::ignore_whitespace(s)?;
        let (s, value) = str_to_bool(s)?;
        let (s, _) = tag(",")(s)?;

        Ok((s, value))
    };
}

fn bool_field_or_false<'a>(s: &'a str, field_tag: &str) -> IResult<&'a str, bool> {
    let (s, value) = opt(bool_field(field_tag))(s)?;
    return Ok((s, value.unwrap_or(false)));
}

struct Arg {
    name: String,
    help: String,
}

fn args<'a>(s: &'a str, field_tag: &str) -> IResult<&'a str, Vec<Arg>> {
    nom_help::short_trace("args", s);

    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, maybe_field) = opt(tag(field_tag))(s)?;
    if maybe_field.is_none() {
        trace!("no arg record");
        return Ok((s, vec![]));
    }

    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, args) = many1(arg)(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("},")(s)?;
    nom_help::short_trace("args afterwards", s);
    Ok((s, args))
}

fn arg(s: &str) -> IResult<&str, Arg> {
    nom_help::short_trace("arg", s);
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("Argument{")(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("name:")(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, name) = nom_help::quoted_string(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("help:")(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, help) = nom_help::quoted_string(s)?;
    let (s, _) = opt(tag(","))(s)?;
    let (s, _) = nom_help::ignore_whitespace(s)?;
    let (s, _) = tag("},")(s)?;
    nom_help::short_trace("arg afterwards", s);
    Ok((
        s,
        Arg {
            name: name.to_string(),
            help: help.to_string(),
        },
    ))
}
