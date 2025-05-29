use sysinfo::System;
use std::thread;

fn main() {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu_usage(); // Refresh CPU data

        let cpu_usage = sys.global_cpu_usage(); // CPU usage percentage

        let total_memory = sys.total_memory() as f64 / 1_048_576.0;
        let used_memory = sys.used_memory() as f64 / 1_048_576.0;

        let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let seconds = uptime_secs % 60;

        println!("CPU Usage: {:.2}%", cpu_usage);

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
