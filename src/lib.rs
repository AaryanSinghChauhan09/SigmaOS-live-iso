pub mod config;
pub mod error;
pub mod iso_builder;
pub mod service;
pub use config::Config;
pub use error::{ComponentError, Result};
pub use iso_builder::{IsoBuilder, IsoSpec, IsoState};
pub use service::{Service, ServiceStatus};
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}
pub struct LiveIsoComponent { service: service::Service }
impl LiveIsoComponent {
    pub fn new(config: Config) -> Self { Self { service: service::Service::new(config) } }
}
impl Component for LiveIsoComponent {
    fn name(&self) -> &str { "sigmaos-live-iso" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("live-iso not running".to_string())) }
    }
}
