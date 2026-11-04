use sigmaos_live_iso::{Config, LiveIsoComponent, Component};
fn main() {
    env_logger::init();
    let mut c = LiveIsoComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS live-iso running");
    let _ = c.shutdown();
}
