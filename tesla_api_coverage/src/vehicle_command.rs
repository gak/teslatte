use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{char, line_ending, space0, space1, tab};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::IResult;
use tracing::{trace, warn};

pub fn parse(s: &str) -> () {
    // Seek all the way to: var commands = map[string]*Command{\n
    // Afterwards has the first map entry.
    let commands_start = "var commands = map[string]*Command{\n";
    let offset = s.find(commands_start).unwrap();
    let s = &s[offset + commands_start.len()..];

    let (go, entries) = many1(map_entry)(s).unwrap();

    dbg!(&entries);

    warn!("todo: parse")
}

#[derive(Debug)]
struct MapEntry {
    endpoint: String,
    help: String,
    // requires_auth: bool,
    // requires_fleet: bool,
}

fn map_entry(s: &str) -> IResult<&str, MapEntry> {
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

    short_trace("--- map entry ---", s);

    // endpoint
    short_trace("endpoint", s);
    let (s, _) = ignore_whitespace(s)?;
    let (s, endpoint) = quoted_string(s)?;
    let (s, _) = until_eol(s)?;

    // help
    short_trace("help", s);
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("help:")(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, help) = quoted_string(s)?;
    let (s, _) = tag(",")(s)?;

    // requiresAuth
    short_trace("requiresAuth", s);
    let (s, requires_auth) = bool_field_or_false(s, "requiresAuth:")?;

    // requiresFleetAPI
    short_trace("requiresFleetAPI", s);
    let (s, requires_fleet) = bool_field_or_false(s, "requiresFleetAPI:")?;

    // required args
    short_trace("required args", s);
    let (s, required_args) = args(s, "args: []Argument{")?;

    // optional args
    short_trace("optional args", s);
    let (s, optional_args) = args(s, "optional: []Argument{")?;

    // check and ignore the handler, as there's not really much data we can take out of it.
    let (s, _) = ignore_whitespace(s)?;

    let (s, _) = take_until("},")(s)?;
    let (s, _) = tag("},")(s)?;

    // And the end of the record...
    let (s, _) = take_until("},")(s)?;
    let (s, _) = tag("},")(s)?;

    dbg!(endpoint, help, requires_auth, requires_fleet);

    Ok((
        s,
        MapEntry {
            endpoint: endpoint.to_string(),
            help: help.to_string(),
        },
    ))
}

/// Ignore the quotes and return the inner string.
/// e.g. "unlock"
fn quoted_string(s: &str) -> IResult<&str, &str> {
    short_trace("quoted string", s);
    let (s, _) = char('"')(s)?;
    let (s, string) = take_until("\"")(s)?;
    let (s, _) = char('"')(s)?;
    Ok((s, string))
}

fn ignore_whitespace(s: &str) -> IResult<&str, ()> {
    short_trace("ignore whitespace", s);
    let (s, ws) = many0(alt((tag("\t"), space1, line_ending)))(s)?;
    short_trace("ignore whitespace afterwards", s);
    Ok((s, ()))
}

fn until_eol(s: &str) -> IResult<&str, &str> {
    short_trace("eol", s);
    let (s, line) = take_until("\n")(s)?;
    let (s, _) = line_ending(s)?;
    short_trace("eol afterwards", s);
    Ok((s, line))
}

fn str_to_bool(s: &str) -> IResult<&str, bool> {
    short_trace("bool", s);
    let (s, bool_str) = alt((tag("true"), tag("false")))(s)?;
    let bool = match bool_str {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    };
    short_trace("bool afterwards", s);
    Ok((s, bool))
}

/// If the field isn't there, assume false.
fn bool_field<'a>(field_tag: &str) -> impl Fn(&'a str) -> IResult<&'a str, bool> + '_ {
    return move |s: &str| -> IResult<&'a str, bool> {
        let (s, _) = ignore_whitespace(s)?;
        let (s, _) = tag(field_tag)(s)?;
        let (s, _) = ignore_whitespace(s)?;
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
    short_trace("args", s);

    let (s, _) = ignore_whitespace(s)?;
    let (s, maybe_field) = opt(tag(field_tag))(s)?;
    if maybe_field.is_none() {
        trace!("no arg record");
        return Ok((s, vec![]));
    }

    let (s, _) = ignore_whitespace(s)?;
    let (s, args) = many1(arg)(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("},")(s)?;
    short_trace("args afterwards", s);
    Ok((s, args))
}

fn arg(s: &str) -> IResult<&str, Arg> {
    short_trace("arg", s);
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("Argument{")(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("name:")(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, name) = quoted_string(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("help:")(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, help) = quoted_string(s)?;
    let (s, _) = opt(tag(","))(s)?;
    let (s, _) = ignore_whitespace(s)?;
    let (s, _) = tag("},")(s)?;
    short_trace("arg afterwards", s);
    Ok((
        s,
        Arg {
            name: name.to_string(),
            help: help.to_string(),
        },
    ))
}

fn short_trace(prefix: &str, s: &str) {
    let mut max_len_left = 20;
    if s.len() < max_len_left {
        max_len_left = s.len();
    }
    trace!("{}: {:?}...", prefix, &s[0..max_len_left])
}
