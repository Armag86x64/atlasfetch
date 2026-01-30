
<img src="images/demo.png" align ="center">


<h2 align="center">Atlasfetch</h2>

<dev align="center">
<img alt="GitHub License" src="https://img.shields.io/github/license/Armag86x64/atlasfetch?style=for-the-badge">
<img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/Armag86x64/atlasfetch?style=for-the-badge">
<a href="README_RU.md"><img alt="Static Badge" src="https://img.shields.io/badge/RU-README-red?style=for-the-badge">
</a>
</dev>

**Atlas Fetch** is a neofetch-like CLI utility for retrieving system information and displaying it in a user-friendly way. It is written entirely in Rust, using a minimal number of external crates to gather data. The target operating system is Linux; the program does not work on other systems (Windows/macOS).



<h2 align="center">Installation</h2>

To install the utility, you must first install Rust along with [Cargo](https://rust-lang.org/tools/install/).

```sh
cd ~
git clone https://github.com/Armag86x64/atlasfetch
cd atlasfetch
cargo build --release
sudo cp target/release/atlasfetch /bin
```



<h2 align="center">Usage</h2>

```sh
atlasfetch [options]
```

- **`--debug`**  
    Enables debug mode. If errors occur, it outputs information about them: the module (disk, cpu, gpu, etc.), the function (e.g., get_cache_size()), and the stage (e.g., opening a file) where the error happened.



<h2 align="center">  What information is displayed</h2>

**Atlas Fetch** generates a comprehensive system report, organized into sections. Below is a description of all fields and their values.

#### **1. Operating System (OS)**

Basic information about the operating environment.

- **OS**: Distribution name and CPU architecture (`Arch Linux rolling [x86_64]`).
    
- **Kernel**: Linux kernel version (`Linux 6.12.60-1-lts`).
    
- **Uptime**: Continuous system runtime since the last boot (`11h 35m`).
    
- **OS Installed**: Date and time of the initial operating system installation (`2025-11-08 14:54:26`).
    
- **Shell**: Path to the default command-line shell (`/bin/bash`).
    

#### **2. Battery**

Information about the portable power source (displayed only on laptops).

- **Model**: Battery model identifier (`AP16L5J`).
    
- **Status**: Current operating mode and charge level (`Discharging [69%]`). Possible values: `Charging`, `Discharging`, `Full`, `Not charging`.
    

#### **3. Graphics Processing Unit (GPU)**

Data about the primary/integrated graphics adapter.

- **Vendor & Model**: Chip manufacturer and model (`Intel Corporation Raptor Lake-P [UHD Graphics]`).
    

#### **4. Random Access Memory (RAM)**

Summary of RAM usage.

- **Usage**: Displayed in the format `Used/Total (Free)`. Example: `3.12 GiB/15.35 GiB (12.22 GiB)`.
    

#### **5. Central Processing Unit (CPU)**

Detailed information about the central processing unit.

- **CPU Model**: Full model name, number of logical cores (in square brackets), and maximum clock speed (`13th Gen Intel(R) Core(TM) i5-13420H [12] 4.6 GHz`).
    
- **CPU Cache**: CPU cache hierarchy specifying the type and size for each level:
    
    - **L1**: Separate cache for data (`48 KiB`) and instructions (`32 KiB`).
        
    - **L2**: Unified second-level cache (`1280 KiB`).
        
    - **L3**: Shared third-level cache (`12288 KiB`).
        

#### **6. Storage (Disk)**

Information about physical partitions and their mount points.

- **Physical Disk**: Total size and storage model (`(/dev/nvme0n1) 441.56 GiB/488.38 GiB`).
    
- **Partitions**: List of main partitions specifying:
    
    - Device and mount point (e.g., `/dev/nvme0n1p2 -> /`).
        
    - Used space (`430.78 GiB`).
        
    - File system (`btrfs`, `vfat`).
