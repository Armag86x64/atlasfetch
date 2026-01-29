use std::env;
use std::fs;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;

use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

use chrono;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct KernelInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct UptimeInfo {
    pub hours: u64,
    pub minutes: u64,
    pub date_installation: String,
}

#[derive(Debug)]
pub struct DistroInfo {
    pub name: String,
    pub arch: String,
    pub shell: String,
    pub build_id: String,
    pub uptime_info: UptimeInfo,
    pub kernel_info: KernelInfo,
}

impl UptimeInfo {
    pub fn new() -> Result<Self> {
        let content = fs::read_to_string("/proc/uptime")
            .context("distro: UptimeInfo::new() - Error reading file /proc/uptime")?;

        let uptime = content
            .split_whitespace()
            .next()
            .context("distro: UptimeInfo::new() - Invalid format /proc/uptime")?
            .parse::<f64>()
            .context("distro: UptimeInfo::new() - Error parsing to f64")?;

        let total_minutes = (uptime / 60.0) as u64;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        let date_installation = Self::get_date_installation().unwrap_or("None".to_string());

        Ok(Self {
            hours: hours as u64,
            minutes: minutes as u64,
            date_installation,
        })
    }

    fn format_timestamp(timestamp: u64) -> String {
        use std::time::Duration;

        let system_time = UNIX_EPOCH + Duration::from_secs(timestamp);
        let datetime: chrono::DateTime<chrono::Local> = system_time.into();

        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn get_date_installation() -> Option<String> {
        let paths_to_check = ["/", "/etc", "/var/log", "/root"];

        let mut oldest_time = std::u64::MAX;
        //let mut oldest_path = "";

        for path_str in &paths_to_check {
            let path = Path::new(path_str);

            if let Ok(metadata) = fs::metadata(path) {
                let ctime = metadata.ctime() as u64;
                if ctime > 0 && ctime < oldest_time {
                    oldest_time = ctime;
                    //oldest_path = path_str;
                }
            }
        }

        if oldest_time == std::u64::MAX {
            return None;
        }
        Some(Self::format_timestamp(oldest_time))
    }
}

impl KernelInfo {
    pub fn new() -> Result<Self> {
        let (name, version) = KernelInfo::get_kernel_info()?;

        Ok(Self { name, version })
    }

    fn get_kernel_info() -> Result<(String, String)> {
        let file = fs::read_to_string("/proc/version").unwrap();

        let mut file_split = file.split_whitespace();

        let name = file_split.nth(0).unwrap().to_string();
        let version = file_split.nth(1).unwrap().to_string();

        Ok((name, version))
    }
}

impl DistroInfo {
    pub fn new() -> Result<Self> {
        let (name, build_id) = Self::parse_os_release()?;
        let arch: String = Self::get_arch()?;
        let shell: String = Self::get_shell()?;

        let uptime_info: UptimeInfo = UptimeInfo::new()?;

        let kernel_info: KernelInfo = KernelInfo::new()?;

        Ok(Self {
            name,
            arch,
            shell,
            build_id,
            uptime_info,
            kernel_info,
        })
    }

    fn parse_os_release() -> Result<(String, String)> {
        let file = File::open("/etc/os-release")
            .context("distro: DistroInfo::parse_os_release() - Error opening file")?;
        let reader = BufReader::new(file);

        let mut distro_name = String::new();
        let mut build_id = String::new();

        for line in reader.lines().map_while(Result::ok) {
            if line.starts_with("NAME=") {
                distro_name = line["NAME=".len()..].trim_matches('"').to_string();
            } else if line.starts_with("BUILD_ID=") {
                build_id = line["BUILD_ID=".len()..].to_string();
            }

            if !distro_name.is_empty() && !build_id.is_empty() {
                break;
            }
        }

        Ok((distro_name, build_id))
    }
    /*
        fn parse_line(line: &str) -> String {
            line.split_whitespace().nth(0).unwrap().to_string()
        }
    */
    fn get_arch() -> Result<String> {
        let output = Command::new("uname")
            .arg("-m")
            .output()
            .context("distro: DistroInfo::get_arch() - error runtime \"uname -m\"")?;

        let parsed_output = String::from_utf8(output.stdout)
            .context("distro: DistroInfo::get_arch() - Error converting to UTF-8")?;

        Ok(parsed_output.trim().to_string())
    }

    fn get_shell() -> Result<String> {
        Ok(env::var("SHELL")
            .context("distro: DistroInfo::get_shell() - Error getting variable $SHELL")?)
    }
}
