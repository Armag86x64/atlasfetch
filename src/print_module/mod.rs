use crate::hardware::{
    battery::BatteryInfo, cpu::CpuInfo, disk::DiskInfo, gpu::GpuInfo, memory::MemoryInfo,
};
use crate::system::distro::DistroInfo;

use anyhow::Result;

static VERTICAL_1: &str =
    "\t╭──────────────────────╮    ╭─────────────────────────────────────────────────────────────╮";
static VERTICAL_2: &str =
    "\t╰──────────────────────╯    ╰─────────────────────────────────────────────────────────────╯";

fn format_line(label: &str, value: &str) -> String {
    format!("\t    {:<29}{}", label, value)
}

pub fn print_battery() -> Result<()> {
    let battery_info: Vec<BatteryInfo> = match BatteryInfo::get_bats() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    for battery in &battery_info {
        println!("\n{}", VERTICAL_1);
        println!();

        println!(
            "{}",
            format_line(
                &format!("Battery [{}]", battery.bat_name),
                &format!("Model: {}", battery.model)
            )
        );

        println!(
            "{}",
            format_line(
                "",
                &format!("Status: {:?} [{}%]", battery.status, battery.power_level)
            )
        );

        println!("\n{}", VERTICAL_2);
    }

    Ok(())
}

pub fn print_cpu() -> Result<()> {
    let cpu_info: CpuInfo = match CpuInfo::new() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let cache_vec = cpu_info.cache_list;

    println!("\n{}", VERTICAL_1);
    println!();

    println!(
        "{}",
        format_line(
            "CPU Model",
            &format!(
                "{} [{}] {} @ GHz",
                cpu_info.model_name, cpu_info.siblings_count, cpu_info.max_frequency
            )
        )
    );

    for cache_info in &cache_vec {
        println!(
            "{}",
            format_line(
                "CPU Cache",
                &format!(
                    "L{}  {} KiB  ({:?})",
                    cache_info.level, cache_info.size, cache_info.cache_type
                )
            )
        );
    }

    println!("\n{}", VERTICAL_2);

    Ok(())
}

pub fn parse_disk() -> Result<()> {
    let disk_info: DiskInfo = match DiskInfo::new() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let partition_vec = disk_info.partitions;

    println!("\n{}", VERTICAL_1);
    println!();

    println!(
        "{}",
        format_line(
            "Disk",
            &format!(
                "(/dev/{}) {} GiB/{} GiB",
                disk_info.mount_point, disk_info.free_size, disk_info.total_size
            )
        )
    );

    for part_info in partition_vec {
        println!(
            "{}",
            format_line(
                "",
                &format!(
                    "({} -> {}) {} GiB ({})",
                    part_info.name, part_info.mount_point, part_info.size, part_info.file_system
                )
            )
        );
    }

    println!("\n{}\n", VERTICAL_2);

    Ok(())
}

pub fn parse_gpu() -> Result<()> {
    let gpu_info: GpuInfo = match GpuInfo::new() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    println!("\n{}", VERTICAL_1);
    println!();

    println!(
        "{}",
        format_line(
            "GPU",
            &format!("{} {}", gpu_info.vendor_name, gpu_info.device_name)
        )
    );

    println!("\n{}", VERTICAL_2);

    Ok(())
}

pub fn parse_memory() -> Result<()> {
    let memory_info: MemoryInfo = match MemoryInfo::new() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    println!("\n{}", VERTICAL_1);
    println!();

    println!(
        "{}",
        format_line(
            "RAM",
            &format!(
                "{} GiB/{} GiB ({} GiB)",
                memory_info.used_size, memory_info.total_size, memory_info.free_size
            )
        )
    );

    println!("\n{}", VERTICAL_2);

    Ok(())
}

pub fn parse_distro() -> Result<()> {
    let distro_info: DistroInfo = match DistroInfo::new() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    println!("\n{}", VERTICAL_1);
    println!();

    println!(
        "{}",
        format_line(
            "OS",
            &format!(
                "{} {} [{}]",
                distro_info.name, distro_info.build_id, distro_info.arch
            )
        )
    );

    println!(
        "{}",
        format_line(
            "Kernel",
            &format!(
                "{} {}",
                distro_info.kernel_info.name, distro_info.kernel_info.version
            )
        )
    );

    println!(
        "{}",
        format_line(
            "Uptime",
            &format!(
                "{} hours {} mins",
                distro_info.uptime_info.hours, distro_info.uptime_info.minutes
            )
        )
    );

    println!(
        "{}",
        format_line(
            "OS Installed",
            &distro_info.uptime_info.date_installation.to_string()
        )
    );

    println!("{}", format_line("Shell", &distro_info.shell));

    println!("\n{}", VERTICAL_2);

    Ok(())
}
