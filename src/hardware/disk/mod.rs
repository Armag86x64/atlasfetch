use std::fs;

use sys_info;
use sysinfo::Disks;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct PartitionInfo {
    pub name: String,
    pub size: f64,
    pub mount_point: String,
    pub file_system: String,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub total_size: f64,
    pub free_size: f64,
    pub mount_point: String,
    pub partitions: Vec<PartitionInfo>,
}

impl DiskInfo {
    pub fn new() -> Result<Self> {
        let total_size: f64;
        let free_size: f64;
        let mount_point = Self::get_mount_point()?;
        let partitions = PartitionInfo::build()?;

        match sys_info::disk_info().context("DiskInfo::new()") {
            Ok(diskinfo) => {
                total_size = parse_to_gb(diskinfo.total, false);
                free_size = parse_to_gb(diskinfo.free, false);
            }
            Err(error) => return Err(error),
        };

        Ok(Self {
            total_size,
            free_size,
            mount_point,
            partitions,
        })
    }

    fn get_mount_point() -> Result<String> {
        let file = fs::read_to_string("/proc/partitions")
            .context("disk::DiskInfo::get_mount_point() - Error reading file")?;

        let parsed_file = file
            .lines()
            .nth(2)
            .context("disk::DiskInfo::get_mount_point() - Error parsing line")?;

        let parsed_line = parsed_file
            .split_whitespace()
            .nth(3)
            .context("disk::DiskInfo::get_mount_point() - Error parsing line")?;

        Ok(parsed_line.to_string())
    }
}

impl PartitionInfo {
    fn new(name: String, size: f64, mount_point: String, file_system: String) -> Self {
        Self {
            name,
            size,
            mount_point,
            file_system,
        }
    }

    pub fn build() -> Result<Vec<PartitionInfo>> {
        let mut returned_vec: Vec<PartitionInfo> = Vec::new();

        let disk_list = Disks::new_with_refreshed_list();

        for disk in disk_list.list() {
            let name = disk
                .name()
                .to_str()
                .context("disk::PartitionInfo::build() - name")?
                .to_string();
            let size = parse_to_gb(disk.available_space(), true);
            let mount_point = disk
                .mount_point()
                .to_str()
                .context("disk::PartitionInfo::build() - mount_point")?
                .to_string();
            let file_system = disk
                .file_system()
                .to_str()
                .context("disk::PartitionInfo::build() - file_system")?
                .to_string();

            returned_vec.push(Self::new(name, size, mount_point, file_system))
        }

        Ok(returned_vec)
    }
}

fn parse_to_gb(data: u64, bytes: bool) -> f64 {
    let mb_total = if bytes {
        data as f64 / 1024.0 / 1024.0
    } else {
        data as f64 / 1024.0
    };

    let gb = (mb_total / 1024.0).trunc() as u64;

    let mb_fraction = (mb_total - (gb as f64 * 1024.0)) / 1024.0;

    (gb as f64) + (mb_fraction * 100.0).trunc() / 100.0
}

/*
fn parse_partition() -> Vec<(String, f64)> {
    let parsed_file = fs::read_to_string("/proc/partitions").unwrap();

    let mut parsed_part: Vec<(String, f64)> = Vec::new();

    for line in parsed_file.lines().skip(3) {
        parsed_part.push(parse_line_partitions(line));
    }

    parsed_part
}

fn parse_fstab() -> Vec<(String, String)> {
    let parsed_file = fs::read_to_string("/etc/fstab").unwrap();

    let mut parsed_part: Vec<(String, String)> = Vec::new();

    for line in parsed_file.lines().skip(5) {
        if line.starts_with("UUID") {
            parsed_part.push(parse_line_fstab(line));
        }
    }

    parsed_part.reverse();
    parsed_part
}

fn parse_line_fstab(line: &str) -> (String, String) {
    let mut line = line.split_whitespace().skip(1);

    let mount_point = line.nth(0).unwrap().to_string();
    let file_system = line.nth(0).unwrap().to_string();

    (mount_point, file_system)
}

fn parse_line_partitions(line: &str) -> (String, f64) {
    let mut line = line.split_whitespace().skip(2);

    let blocks: f64 = parse_to_gb(line.next().unwrap().parse::<u64>().unwrap());
    let name: String = format!("/dev/{}", line.next().unwrap());

    (name, blocks)
}


*/
