use anyhow::{Context, Result};
use std::fs;

#[derive(Debug)]
pub struct SwapInfo {
    pub total_size: f64,
    pub free_size: f64,
    pub used_size: f64,
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total_size: f64,
    pub free_size: f64,
    pub used_size: f64,
    pub swap_info: SwapInfo,
}

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        let ram_info = Self::from_keys("MemTotal", "MemAvailable", None)?;
        let swap_info = Self::from_keys("SwapTotal", "SwapFree", Some("SwapCached"))?;

        let swap_obj = SwapInfo {
            total_size: swap_info.0,
            free_size: swap_info.1,
            used_size: swap_info.2,
        };

        Ok(Self {
            total_size: ram_info.0,
            free_size: ram_info.1,
            used_size: ram_info.2,
            swap_info: swap_obj,
        })
    }

    fn from_keys(
        total_key: &str,
        free_key: &str,
        used_key: Option<&str>,
    ) -> Result<(f64, f64, f64)> {
        let meminfo = fs::read_to_string("/proc/meminfo")
            .context("memory: from_keys - error reading file")?;
        let mut info = (0.0, 0.0, 0.0);

        for line in meminfo.lines() {
            if line.starts_with(total_key) {
                info.0 = parse_to_gb(line);
            } else if line.starts_with(free_key) {
                info.1 = parse_to_gb(line);
            } else if let Some(key) = used_key {
                if line.starts_with(key) {
                    info.2 = parse_to_gb(line);
                }
            }
        }

        if used_key.is_none() {
            info.2 = ((info.0 - info.1) * 100.0).trunc() / 100.0;
        }

        Ok(info)
    }
}

fn parse_to_gb(line: &str) -> f64 {
    let kb: f64 = line
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0.0);

    let gb = (kb / 1024.0 / 1024.0).trunc();
    let mb: f64 = ((kb / 1024.0 - (gb * 1024.0)) / 10.0).trunc() / 100.0;
    gb + mb
}
