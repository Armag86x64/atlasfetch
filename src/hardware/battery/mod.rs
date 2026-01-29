use std::fs;

use anyhow::{Context, Result};

#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Unknow,
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub power_level: u8,
    pub status: BatteryStatus,
    pub bat_name: String,
    pub model: String,
}

impl BatteryInfo {
    fn new(path: &str, bat_name: String) -> Result<Self> {
        let power_level = Self::get_capacity(path)?;
        let status = Self::get_status(path)?;
        let model = Self::get_model(path)?;

        Ok(Self {
            power_level,
            status,
            bat_name,
            model,
        })
    }

    pub fn get_bats() -> Result<Vec<BatteryInfo>> {
        let mut battery_vector: Vec<BatteryInfo> = Vec::new();

        let path: &str = "/sys/class/power_supply/";

        let power_supply = fs::read_dir(path)
            .context("battery::BatteryInfo::get_bats() - Error of reading directory")?;

        for entry in power_supply {
            let bat_dir = match entry.context("battery: get_bats() - DirEntry error") {
                Ok(dir) => dir,
                Err(error) => {
                    return Err(error);
                }
            };

            let bat_name = bat_dir
                .file_name()
                .to_str()
                .context("battery: get_bats() - bat_name")?
                .to_string();

            let battery_obj: BatteryInfo = if bat_name.contains("BAT") {
                let path = format!("{}{}", path, bat_name);

                BatteryInfo::new(&path, bat_name)?
            } else {
                continue;
            };

            battery_vector.push(battery_obj);
        }

        Ok(battery_vector)
    }

    fn get_capacity(start_path: &str) -> Result<u8> {
        let path: String = format!("{}/capacity", start_path);

        let power_level = fs::read_to_string(&path)
            .context("battery::BatteryInfo::get_capacity(): Error reading capacity-file")?;

        Ok(power_level
            .trim()
            .parse::<u8>()
            .context("battery::BatteryInfo::get_capacity(): Error parsing power_level")?)
    }

    fn get_status(start_path: &str) -> Result<BatteryStatus> {
        let path: String = format!("{}/status", start_path);

        let status_str = fs::read_to_string(&path)
            .context("battery::BatteryInfo::get_status() - Error reading file")?;

        match status_str.trim() {
            "Discharging" => Ok(BatteryStatus::Discharging),
            "Charging" => Ok(BatteryStatus::Charging),
            _ => Ok(BatteryStatus::Unknow),
        }
    }

    fn get_model(start_path: &str) -> Result<String> {
        let path: String = format!("{}/model_name", start_path);

        let model_name = fs::read_to_string(&path)
            .context("battery::BatteryInfo::get_model() - Error reading file")?;

        Ok(model_name.trim().to_string())
    }
}
