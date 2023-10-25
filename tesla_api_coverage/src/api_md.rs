use crate::fleet::FleetEndpoint;
use crate::teslatte::TeslatteEndpoint;
use crate::timdorr::TimdorrEndpoint;
use crate::vehicle_command::VehicleCommandEndpoint;
use std::collections::HashMap;

pub fn generate_api_md(
    teslatte: Vec<TeslatteEndpoint>,
    fleet: HashMap<String, FleetEndpoint>,
    vehicle_command: HashMap<String, VehicleCommandEndpoint>,
    timdorr: HashMap<String, TimdorrEndpoint>,
) -> anyhow::Result<()> {
    todo!()
}
