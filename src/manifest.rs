use crate::error::LiveIsoError;

/// Describes the contents and configuration of a live ISO image.
#[derive(Debug, Clone)]
pub struct IsoManifest {
    /// Human-readable label for the ISO (e.g. "SigmaOS").
    pub label: String,
    /// Version string (e.g. "1.0.0").
    pub version: String,
    /// Target architecture (e.g. "x86_64").
    pub arch: String,
    /// List of package names to include in the image.
    pub packages: Vec<String>,
    /// Extra kernel boot parameters.
    pub boot_params: Vec<String>,
}

impl IsoManifest {
    /// Creates a new [`IsoManifest`] with the given core fields and empty lists.
    pub fn new(
        label: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            version: version.into(),
            arch: arch.into(),
            packages: Vec::new(),
            boot_params: Vec::new(),
        }
    }

    /// Adds a package to the manifest.
    pub fn add_package(&mut self, pkg: impl Into<String>) {
        self.packages.push(pkg.into());
    }

    /// Adds a kernel boot parameter.
    pub fn add_boot_param(&mut self, param: impl Into<String>) {
        self.boot_params.push(param.into());
    }

    /// Validates that required fields are non-empty and architecture is supported.
    ///
    /// Returns [`LiveIsoError::InvalidManifest`] with a descriptive message on failure.
    pub fn validate(&self) -> Result<(), LiveIsoError> {
        if self.label.trim().is_empty() {
            return Err(LiveIsoError::InvalidManifest("label must not be empty".into()));
        }
        if self.version.trim().is_empty() {
            return Err(LiveIsoError::InvalidManifest("version must not be empty".into()));
        }
        let supported = ["x86_64", "aarch64", "riscv64"];
        if !supported.contains(&self.arch.as_str()) {
            return Err(LiveIsoError::InvalidManifest(format!(
                "unsupported arch '{}'; supported: {:?}",
                self.arch, supported
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AAA: Arrange valid manifest, Act validate, Assert Ok.
    #[test]
    fn test_valid_manifest_passes_validation() {
        // Arrange
        let mut m = IsoManifest::new("SigmaOS", "1.0.0", "x86_64");
        m.add_package("base");
        // Act
        let result = m.validate();
        // Assert
        assert!(result.is_ok());
    }

    // AAA: Arrange manifest with empty label, Act validate, Assert Err.
    #[test]
    fn test_empty_label_fails_validation() {
        // Arrange
        let m = IsoManifest::new("", "1.0.0", "x86_64");
        // Act
        let result = m.validate();
        // Assert
        assert!(matches!(result, Err(LiveIsoError::InvalidManifest(_))));
    }

    // AAA: Arrange manifest with bad arch, Act validate, Assert Err mentioning arch.
    #[test]
    fn test_unsupported_arch_fails_validation() {
        // Arrange
        let m = IsoManifest::new("SigmaOS", "2.0", "mips");
        // Act
        let result = m.validate();
        // Assert
        match result {
            Err(LiveIsoError::InvalidManifest(msg)) => assert!(msg.contains("mips")),
            _ => panic!("expected InvalidManifest error"),
        }
    }

    // AAA: Arrange manifest, Act add_package/add_boot_param, Assert lengths.
    #[test]
    fn test_adding_packages_and_params() {
        // Arrange
        let mut m = IsoManifest::new("SigmaOS", "1.0", "aarch64");
        // Act
        m.add_package("kernel");
        m.add_package("udev");
        m.add_boot_param("quiet");
        // Assert
        assert_eq!(m.packages.len(), 2);
        assert_eq!(m.boot_params.len(), 1);
    }
}
