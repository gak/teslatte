use clap::{Args, Parser, Subcommand};
use teslatte::vehicles::{SetChargeLimit, SetChargingAmps};
use teslatte::{Api, VehicleId};

#[derive(Debug, Args)]
pub struct VehicleArgs {
    pub id: VehicleId,

    #[clap(subcommand)]
    pub command: VehicleCommand,
}

impl VehicleArgs {
    pub async fn run(self, api: &Api) -> miette::Result<()> {
        match self.command {
            VehicleCommand::Data => {
                dbg!(api.vehicle_data(&self.id).await?);
            }
            VehicleCommand::ChargeState => {
                dbg!(api.charge_state(&self.id).await?);
            }
            VehicleCommand::SetChargeLimit { percent } => {
                dbg!(
                    api.set_charge_limit(&self.id, &SetChargeLimit { percent })
                        .await?
                );
            }
            VehicleCommand::SetChargingAmps { charging_amps } => {
                dbg!(
                    api.set_charging_amps(&self.id, &SetChargingAmps { charging_amps })
                        .await?
                );
            }
            VehicleCommand::ChargeStart => {
                dbg!(api.charge_start(&self.id).await?);
            }
            VehicleCommand::ChargeStop => {
                dbg!(api.charge_stop(&self.id).await?);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum VehicleCommand {
    /// Get vehicle data.
    Data,

    /// Get charge state.
    ChargeState,

    /// Set charge limit.
    SetChargeLimit { percent: u8 },

    /// Set charge amps.
    SetChargingAmps { charging_amps: i64 },

    /// Start charging.
    ChargeStart,

    /// Stop charging.
    ChargeStop,
}
