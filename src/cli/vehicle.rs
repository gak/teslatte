use crate::vehicles::{
    Endpoints, GetVehicleData, SetChargeLimit, SetChargingAmps, SetScheduledCharging,
    SetScheduledDeparture, SetTemperatures,
};
use crate::{OwnerApi, VehicleApi, VehicleId};
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum VehicleCommand {
    /// Get vehicle data.
    VehicleData(Endpoints),

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

    /// Wake up
    WakeUp,

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
            VehicleCommand::VehicleData(endpoints) => {
                let get_vehicle_data = GetVehicleData::new_with_endpoints(self.id, endpoints);
                api.vehicle_data(&get_vehicle_data).await?;
            }
            VehicleCommand::SetChargeLimit(limit) => {
                api.set_charge_limit(&self.id, &limit).await?;
            }
            VehicleCommand::SetChargingAmps(charging_amps) => {
                api.set_charging_amps(&self.id, &charging_amps).await?;
            }
            VehicleCommand::ChargeStart => {
                api.charge_start(&self.id).await?;
            }
            VehicleCommand::ChargeStop => {
                api.charge_stop(&self.id).await?;
            }
            VehicleCommand::ChargePortDoorOpen => {
                api.charge_port_door_open(&self.id).await?;
            }
            VehicleCommand::ChargePortDoorClose => {
                api.charge_port_door_close(&self.id).await?;
            }
            VehicleCommand::ChargeStandard => {
                api.charge_standard(&self.id).await?;
            }
            VehicleCommand::ChargeMaxRange => {
                api.charge_max_range(&self.id).await?;
            }
            VehicleCommand::SetScheduledCharging(charging) => {
                api.set_scheduled_charging(&self.id, &charging).await?;
            }
            VehicleCommand::SetScheduledDeparture(departure) => {
                api.set_scheduled_departure(&self.id, &departure).await?;
            }
            VehicleCommand::WakeUp => {
                api.wake_up(&self.id).await?;
            }
            VehicleCommand::HonkHorn => {
                api.honk_horn(&self.id).await?;
            }
            VehicleCommand::FlashLights => {
                api.flash_lights(&self.id).await?;
            }
            VehicleCommand::EnableHvac => {
                api.auto_conditioning_start(&self.id).await?;
            }
            VehicleCommand::DisableHvac => {
                api.auto_conditioning_stop(&self.id).await?;
            }
            VehicleCommand::HvacTemperature(temps) => {
                api.set_temps(&self.id, &temps).await?;
            }
            VehicleCommand::DoorUnlock => {
                api.door_unlock(&self.id).await?;
            }
            VehicleCommand::DoorLock => {
                api.door_lock(&self.id).await?;
            }
            VehicleCommand::RemoteStartDrive => {
                api.remote_start_drive(&self.id).await?;
            }
        }
        Ok(())
    }
}
