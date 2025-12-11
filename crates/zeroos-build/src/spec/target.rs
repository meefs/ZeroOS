//! Target configuration (user inputs)

/// Target configuration representing user-provided parameters
/// This follows the standard target triple format: {arch}-{vendor}-{sys}[-{abi}]
#[derive(Debug, Clone)]
pub struct TargetConfig {
    /// Architecture (e.g., "riscv64", "riscv32")
    pub arch: String,
    /// Vendor (e.g., "zero", "unknown")
    pub vendor: String,
    /// System/OS (e.g., "linux", "none")
    pub os: String,
    /// ABI (e.g., "musl", "gnu", "" for none)
    pub abi: String,
}

impl TargetConfig {
    /// Create a new target configuration
    ///
    /// Parameters follow target triple order: arch, vendor, os, abi
    pub fn new(arch: String, vendor: String, os: String, abi: String) -> Self {
        Self {
            arch,
            vendor,
            os,
            abi,
        }
    }

    /// Get the target triple for this configuration
    /// Format: {arch}-{vendor}-{os}[-{abi}]
    pub fn target_triple(&self) -> String {
        if self.abi.is_empty() {
            format!("{}-{}-{}", self.arch, self.vendor, self.os)
        } else {
            format!("{}-{}-{}-{}", self.arch, self.vendor, self.os, self.abi)
        }
    }
}
