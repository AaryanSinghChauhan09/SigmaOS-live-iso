use sigmaos_live_iso::{Config, IsoBuilder, IsoSpec, IsoState, LiveIsoComponent, Component};

#[test]
fn test_iso_spec_filename_aaa() {
    let spec = IsoSpec::new("SigmaOS", "0.1.0");
    assert_eq!(spec.iso_filename(), "sigmaos-0.1.0-x86_64.iso");
}

#[test]
fn test_iso_builder_full_flow_aaa() {
    let spec = IsoSpec::new("SigmaOS", "0.1.0");
    let mut builder = IsoBuilder::new(spec);
    builder.prepare();
    let result = builder.build();
    assert!(result.is_ok());
    assert_eq!(builder.state(), &IsoState::Complete);
}

#[test]
fn test_iso_builder_no_prepare_fails_aaa() {
    let spec = IsoSpec::new("SigmaOS", "0.1.0");
    let mut builder = IsoBuilder::new(spec);
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_iso_spec_live_session_aaa() {
    let spec = IsoSpec::new("SigmaOS", "1.0.0");
    assert!(spec.include_live_session);
    assert_eq!(spec.arch, "x86_64");
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = LiveIsoComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = LiveIsoComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-live-iso");
}
