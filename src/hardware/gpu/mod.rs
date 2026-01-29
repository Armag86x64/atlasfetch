use std::process::Command;
use std::str;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct GpuInfo {
    pub vendor_name: String,
    pub device_name: String,
}

impl GpuInfo {
    pub fn new() -> Result<Self> {
        let (vendor_name, device_name) = get_gpu_info()?;

        Ok(Self {
            vendor_name,
            device_name,
        })
    }
}

fn get_gpu_info() -> Result<(String, String)> {
    let output = Command::new("lspci")
        .arg("-vmm")
        .arg("-d")
        .arg("::0300")
        .output()
        .context("gpu: get_gpu_info - lspci runtime error")?;

    let output_str = str::from_utf8(&output.stdout)
        .context("gpu: get_gpu_info - output_str: UTF-8 conversion error")?;

    let mut vendor = None;
    let mut device = None;

    for line in output_str.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "Vendor" => vendor = Some(value.to_string()),
            "Device" => device = Some(value.to_string()),
            _ => {}
        }

        if vendor.is_some() && device.is_some() {
            break;
        }
    }

    Ok((
        vendor.context("gpu: error getting vendor information")?,
        device.context("gpu: error getting device information")?,
    ))
}
