use std::fs;
use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};

#[derive(Debug)]
pub enum CacheType {
    Data,
    Instruction,
    Unified,
    Unknow,
}

#[derive(Debug)]
pub struct CacheInfo {
    pub size: u32,
    pub level: u8,
    pub cache_type: CacheType,
}

#[derive(Debug)]
pub struct CpuInfo {
    pub model_name: String,
    pub siblings_count: u8,
    pub max_frequency: f32,
    pub cache_list: Vec<CacheInfo>,
}

impl CacheInfo {
    fn new(path: String) -> Result<Self> {
        Ok(Self {
            size: Self::get_cache_size(&path)?,
            level: Self::get_cache_level(&path)?,
            cache_type: Self::get_cache_type(&path)?,
        })
    }

    fn get_cache_size(start_path: &str) -> Result<u32> {
        let path = format!("{}/size", start_path);

        let cache_size =
            fs::read_to_string(&path).context("cpu: get_cache_size - error reading file")?;

        Ok(cache_size
            .replace("K", "")
            .trim()
            .parse::<u32>()
            .context("cpu: get_cache_size - error parsing to u32")?)
    }

    fn get_cache_level(start_path: &str) -> Result<u8> {
        let path = format!("{}/level", start_path);

        let cache_level_string =
            fs::read_to_string(&path).context("cpu: get_cache_level - error reading file")?;

        Ok(cache_level_string
            .trim()
            .parse::<u8>()
            .context("cpu: get_cache_level - error parsing to u8")?)
    }

    fn get_cache_type(start_path: &str) -> Result<CacheType> {
        let path = format!("{}/type", start_path);

        let cache_type_string =
            fs::read_to_string(&path).context("cpu: get_cache_type - error reading file")?;

        match cache_type_string.trim() {
            "Data" => Ok(CacheType::Data),
            "Instruction" => Ok(CacheType::Instruction),
            "Unified" => Ok(CacheType::Unified),
            _ => Ok(CacheType::Unknow),
        }
    }

    pub fn get_cache_indexes() -> Result<Vec<CacheInfo>> {
        let mut cache_vector: Vec<CacheInfo> = Vec::new();

        let path: &str = "/sys/devices/system/cpu/cpu0/cache/";

        let power_supply =
            fs::read_dir(path).context("cpu: get_cache_indexes - error reading dir")?;

        for entry in power_supply {
            let current_directory = match entry.context("cpu: get_cache_indexes - DirEntry error") {
                Ok(dir) => dir,
                Err(error) => {
                    return Err(error);
                }
            };

            let file_name = current_directory
                .file_name()
                .to_str()
                .context("cpu: get_cache_indexes - error file_name convertation")?
                .to_string();

            let cache_obj: CacheInfo = if file_name.contains("index") {
                let path = format!("{}{}", path, file_name);

                CacheInfo::new(path)?
            } else {
                continue;
            };

            cache_vector.push(cache_obj);
        }

        cache_vector.sort_by_key(|k| k.level);

        Ok(cache_vector)
    }
}

impl CpuInfo {
    pub fn new() -> Result<Self> {
        let cpuinfo: String = Self::get_cpuinfo()?;

        let model_name = Self::get_model(&cpuinfo);
        let siblings_count = Self::get_siblings(&cpuinfo)?;
        let max_frequency = Self::get_frequency().context("cpu: max_frequency - error getting")?;

        let cache_list = CacheInfo::get_cache_indexes()?;

        Ok(Self {
            model_name,
            siblings_count,
            max_frequency,
            cache_list,
        })
    }

    fn get_cpuinfo() -> Result<String> {
        let content =
            fs::read_to_string("/proc/cpuinfo").context("cpu: get_cpuinfo - error reading file")?;

        Ok(content)
    }

    fn get_model(cpuinfo: &str) -> String {
        let information_split: Vec<&str> = cpuinfo.split("\n").collect();

        let mut model_name: String = String::new();

        for i in 0..(information_split.len()) {
            if information_split[i].contains("model name") {
                let model_split: Vec<&str> = information_split[i].split(":").collect();

                model_name = String::from(model_split[1].trim());
                break;
            }
        }

        model_name
    }

    fn get_siblings(cpuinfo: &str) -> Result<u8> {
        let information_split: Vec<&str> = cpuinfo.split("\n").collect();

        let mut siblings: String = String::new();

        for i in 0..(information_split.len()) {
            if information_split[i].contains("siblings") {
                let siblings_split: Vec<&str> = information_split[i].split(":").collect();

                siblings = String::from(siblings_split[1].trim());
                break;
            }
        }

        Ok(siblings
            .parse::<u8>()
            .context("cpu: get_siblings - error parsing to u8")?)
    }

    fn get_frequency() -> Option<f32> {
        let to_check = [
            "/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq",
            "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq",
        ];

        for path in to_check {
            let mut content: String = String::new();

            let file = File::open(path);

            match file {
                Ok(mut file) => {
                    file.read_to_string(&mut content)
                        .context("cpu: get_frequency - error reading file")
                        .ok()?;

                    let freq: f32 = content
                        .trim()
                        .parse::<f32>()
                        .context("cpu: get_frequency - error parsing to f32")
                        .ok()?;

                    return Some(freq / 1_000_000.0);
                }
                Err(_) => {
                    continue;
                }
            };
        }
        None
    }
}
/*
pub fn get_cpu_info() -> CpuInfo {
    return CpuInfo::new();
}
*/
