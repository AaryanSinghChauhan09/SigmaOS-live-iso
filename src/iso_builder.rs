#[derive(Debug, Clone, PartialEq)]
pub enum IsoState { Uninitialized, Preparing, Building, Complete, Failed(String) }
#[derive(Debug, Clone)]
pub struct IsoSpec {
    pub label: String,
    pub version: String,
    pub arch: String,
    pub include_live_session: bool,
}
impl IsoSpec {
    pub fn new(label: &str, version: &str) -> Self {
        Self { label: label.to_string(), version: version.to_string(), arch: "x86_64".to_string(), include_live_session: true }
    }
    pub fn iso_filename(&self) -> String {
        format!("{}-{}-{}.iso", self.label.to_lowercase(), self.version, self.arch)
    }
}
pub struct IsoBuilder { spec: IsoSpec, state: IsoState }
impl IsoBuilder {
    pub fn new(spec: IsoSpec) -> Self { Self { spec, state: IsoState::Uninitialized } }
    pub fn prepare(&mut self) { self.state = IsoState::Preparing; }
    pub fn build(&mut self) -> Result<String, String> {
        if self.state != IsoState::Preparing { return Err("not in Preparing state".to_string()); }
        self.state = IsoState::Complete;
        Ok(self.spec.iso_filename())
    }
    pub fn state(&self) -> &IsoState { &self.state }
    pub fn spec(&self) -> &IsoSpec { &self.spec }
}
