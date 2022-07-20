use std::time::Instant;
use sysinfo::{CpuExt, SystemExt};

const LOGO: &'static [&str] = &[
    "          #                ",
    "            #  #           ",
    "             #             ",
    "            ###            ",
    "            ###            ",
    "            ###            ",
    "            ###            ",
    "         #########         ",
    "         #########         ",
    "                           ",
    "                           ",
    "                           ",
];

pub fn print_info(prefix: &str, content: &str) {
    println!("\x1b[34;1m{}\x1b[0m: {}", prefix, content)
}

// NOTE: this can be improved
pub fn format_uptime(secs: u64) -> String {
    let mut fmt = String::with_capacity(1024);

    let days = secs / 60 / 60 / 24;
    let hours = secs / 60 / 60 % 24;
    let minutes = secs / 60 % 60;

    if days != 0 {
        fmt += &format!("{} days ", days);
    }
    if hours != 0 {
        fmt += &format!("{} hours ", hours);
    }
    if minutes != 0 {
        fmt += &format!("{} mins ", minutes);
    }

    fmt
}

fn main() {
    let start = Instant::now();
    let mut sys = sysinfo::System::default();
    sys.refresh_all();

    let user = "daniel";
    let hostname = sys.host_name().unwrap_or("localhost".into());
    let seperator_len = user.len() + hostname.len() + 1;

    let os_arch = std::env::consts::ARCH;
    let os_name = sys.name().unwrap_or(std::env::consts::OS.into());
    let kernel_ver = sys.kernel_version().unwrap_or("unknown".into());

    let uptime = sys.uptime();

    let cpu = sys.global_cpu_info();
    let cpu_brand = cpu.brand();
    let cpu_usage = cpu.cpu_usage();

    let used_mem = sys.used_memory() / 1024;
    let total_mem = sys.total_memory() / 1024;

    for i in 0..LOGO.len() {
        print!("{}", LOGO[i]);
        match i {
            0 => {
                println!("\x1b[35;1m{}\x1b[0m@\x1b[35;1m{}\x1b[0m", user, hostname)
            }
            1 => println!("{}", format!("{:-^1$}", '-', seperator_len)),
            2 => print_info("OS", &format!("{} ({})", os_name, os_arch)),
            3 => print_info("Kernel", &kernel_ver),
            4 => print_info("Uptime", &format_uptime(uptime)),
            5 => print_info("CPU", &format!("{} ({:.2}%)", cpu_brand, cpu_usage)),
            6 => print_info("Memory", &format!("{} MiB / {} MiB", used_mem, total_mem)),
            7 => {
                let end = Instant::now();
                print_info("Fetch speed", format!("{:.3?}", end - start).as_str());
            }
            _ => println!(),
        }
    }
}
