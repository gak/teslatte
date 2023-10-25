use crate::Endpoint;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const START: &str = "<!-- tesla_api_coverage start table -->";
const END: &str = "<!-- tesla_api_coverage end table -->";

pub fn generate(merged: &HashMap<String, Endpoint>) -> anyhow::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("API.md");

    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);

    // Split the file into before and after the table.
    let mut parts = contents.split(START);
    let before = parts.next().unwrap();
    let mut parts = parts.next().unwrap().split(END);
    let table = parts.next().unwrap();
    let after = parts.next().unwrap();

    let mut table = vec![];
    table.push("| API | Owners API | Fleet API | Command Mode |".to_string());
    table.push("| --- | --- | --- | --- |".to_string());

    // Sort by URL for into a Vec for now. Keep the key.
    let mut endpoints = merged.iter().collect::<Vec<(&String, &Endpoint)>>();
    endpoints.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    for (name, endpoint) in endpoints {
        let mut row = vec![];
        row.push(format!("{}", name));

        if endpoint.teslatte_owners_api.is_some() {
            row.push("✅".to_string());
        } else if endpoint.timdorr.is_some() {
            row.push("🔴".to_string());
        } else {
            row.push("➖".to_string());
        }

        if endpoint.fleet.is_some() {
            row.push("🔴".to_string());
        } else {
            row.push("➖".to_string());
        }

        if endpoint.vehicle_command.is_some() {
            row.push("🔴".to_string());
        } else {
            row.push("➖".to_string());
        }

        table.push(format!("| {} |", row.join(" | ")));
    }

    let table = table.join("\n");

    // Now join them together and write the whole file back.
    let contents = format!("{before}{START}\n{table}\n{END}{after}");
    let mut file = File::create(&path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
