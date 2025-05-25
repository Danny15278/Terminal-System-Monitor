use sysinfo::System;
use std::thread;

fn main() {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu_usage(); // Refresh CPU data

        let cpu_usage = sys.global_cpu_usage(); // Get its usage percentage

        println!("CPU Usage: {:.2}%", cpu_usage);

        // Sleep to allow enough time for updated data
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
