use std::{fs, time::{SystemTime, UNIX_EPOCH}};
use clap::Parser;
use process_killer::procs_cfg::{parse_config, ProcsConfig};
use sysinfo::{System, SystemExt, ProcessExt, Signal};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[arg(short, long)]
    config_path: std::path::PathBuf
}

// Register procs killer by procs name
fn reg_procs_killer(sys: &mut System, procs_cfg: &ProcsConfig) {
    for procs in sys.processes_by_exact_name(&procs_cfg.name) {
        let now = SystemTime::now();
        let d = now.duration_since(UNIX_EPOCH).expect("time went backwards");

        let ds = d.as_secs();
        let d_start_time = ds - procs.start_time();

        println!("name: {} | pid: {} | delta: {}s", procs.name(), procs.pid(), d_start_time);

        if let Some(s) = procs_cfg.expired_seconds {
            if d_start_time >= s {
                procs.kill_with(Signal::Quit).unwrap_or(false); 
                println!("above procs is gonna be killed!");
            }
        }
    }
}

fn main() {
    let args = AppArgs::parse();
    
    let cfg_str = fs::read_to_string(args.config_path).expect("config must exist");
    let procs_cfg = parse_config(&cfg_str).expect("must have a valid config");

    let mut sys = System::new();

    loop {
        sys.refresh_processes();
        for cfg in procs_cfg.iter() {
            reg_procs_killer(&mut sys, &cfg);
        }
        std::thread::sleep(std::time::Duration::from_millis(5000))
    }
}
