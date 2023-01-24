use std::fs;
use process_killer::procs_cfg;
use sysinfo::{System, SystemExt, ProcessExt};

// Register procs killer by procs name
fn reg_procs_killer(sys: &mut System, procs_name: &str) {
    for procs in sys.processes_by_exact_name(procs_name) {
        println!("name: {} | pid: {}", procs.name(), procs.pid())
    }
}

fn main() {
    let mut sys = System::new();
    let cfg_str = fs::read_to_string("./test_files/config.json").expect("config must exist");
    let procs_cfg = procs_cfg::parse_config(&cfg_str).expect("must have a valid config");

    loop {
        sys.refresh_processes();
        for cfg in procs_cfg.iter() {
            reg_procs_killer(&mut sys, &cfg.name);
        }
        std::thread::sleep(std::time::Duration::from_millis(5000))
    }
}
