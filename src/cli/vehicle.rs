use crate::cli::print_json;
use crate::vehicles::{
    SetChargeLimit, SetChargingAmps, SetScheduledCharging, SetScheduledDeparture, SetTemperatures,
};
use crate::{OwnerApi, VehicleId};
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum VehicleCommand {
    /// Get vehicle data.
    VehicleData,

    /// Open the charge port door or unlocks the cable.
    ChargePortDoorOpen,

    /// For vehicles with a motorized charge port, this closes it.
    ChargePortDoorClose,

    /// Set charge limit.
    SetChargeLimit(SetChargeLimit),

    /// Set charge amps.
    SetChargingAmps(SetChargingAmps),

    /// Set the charge limit to the standard %.
    ChargeStandard,

    /// Set the charge limit to the maximum %.
    ChargeMaxRange,

    /// Start charging.
    ChargeStart,

    /// Stop charging.
    ChargeStop,

    /// Set scheduled charging.
    SetScheduledCharging(SetScheduledCharging),

    /// Set scheduled departure.
    SetScheduledDeparture(SetScheduledDeparture),

    /// Honk!
    HonkHorn,

    /// Flash the lights.
    FlashLights,

    /// Enable the HVAC
    EnableHvac,

    /// Disable the HVAC
    DisableHvac,

    /// Set the temperature for the HVAC
    HvacTemperature(SetTemperatures),

    /// Door unlock
    DoorUnlock,

    /// Door lock
    DoorLock,

    /// For keyless driving
    RemoteStartDrive,
}

#[derive(Debug, Args)]
pub struct VehicleArgs {
    pub id: VehicleId,

    #[clap(subcommand)]
    pub command: VehicleCommand,
}

impl VehicleArgs {
    pub async fn run(self, api: &OwnerApi) -> miette::Result<()> {
        match self.command {
            VehicleCommand::VehicleData => {
                print_json(api.vehicle_data(&self.id).await);
            }
            VehicleCommand::SetChargeLimit(limit) => {
                print_json(api.set_charge_limit(&self.id, &limit).await);
            }
            VehicleCommand::SetChargingAmps(charging_amps) => {
                print_json(api.set_charging_amps(&self.id, &charging_amps).await);
            }
            VehicleCommand::ChargeStart => {
                print_json(api.charge_start(&self.id).await);
            }
            VehicleCommand::ChargeStop => {
                print_json(api.charge_stop(&self.id).await);
            }
            VehicleCommand::ChargePortDoorOpen => {
                print_json(api.charge_port_door_open(&self.id).await);
            }
            VehicleCommand::ChargePortDoorClose => {
                print_json(api.charge_port_door_close(&self.id).await);
            }
            VehicleCommand::ChargeStandard => {
                print_json(api.charge_standard(&self.id).await);
            }
            VehicleCommand::ChargeMaxRange => {
                print_json(api.charge_max_range(&self.id).await);
            }
            VehicleCommand::SetScheduledCharging(charging) => {
                print_json(api.set_scheduled_charging(&self.id, &charging).await);
            }
            VehicleCommand::SetScheduledDeparture(departure) => {
                print_json(api.set_scheduled_departure(&self.id, &departure).await);
            }
            VehicleCommand::HonkHorn => {
                print_json(api.honk_horn(&self.id).await);
            }
            VehicleCommand::FlashLights => {
                print_json(api.flash_lights(&self.id).await);
            }
            VehicleCommand::EnableHvac => {
                print_json(api.auto_conditioning_start(&self.id).await);
            }
            VehicleCommand::DisableHvac => {
                print_json(api.auto_conditioning_stop(&self.id).await);
            }
            VehicleCommand::HvacTemperature(temps) => {
                print_json(api.set_temps(&self.id, &temps).await);
            }
            VehicleCommand::DoorUnlock => {
                print_json(api.door_unlock(&self.id).await);
            }
            VehicleCommand::DoorLock => {
                print_json(api.door_lock(&self.id).await);
            }
            VehicleCommand::RemoteStartDrive => {
                print_json(api.remote_start_drive(&self.id).await);
            }
        }
        Ok(())
    }
}
