use super::{ArchSpec, LLVMConfig, TargetConfig};

pub const PROFILE_RISCV64IMAC_ZERO_LINUX_MUSL: &str = "riscv64imac-zero-linux-musl";

pub struct TargetProfile {
    pub config: TargetConfig,
    pub arch_spec: ArchSpec,
    pub llvm_config: LLVMConfig,
}

pub fn load_target_profile(profile: &str) -> Option<TargetProfile> {
    match profile {
        PROFILE_RISCV64IMAC_ZERO_LINUX_MUSL => Some(TargetProfile {
            config: TargetConfig::new(
                "riscv64imac".to_string(),
                "zero".to_string(),
                "linux".to_string(),
                "musl".to_string(),
            ),
            arch_spec: ArchSpec {
                arch: "riscv64",
                cpu: "generic-rv64",
                pointer_width: "64",
                max_atomic_width: 64,
                endian: "little",
            },
            llvm_config: LLVMConfig {
                llvm_target: "riscv64-unknown-linux-musl".to_string(),
                features: "+m,+a,+c".to_string(),
                abi: "lp64".to_string(),
                data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".to_string(),
            },
        }),
        _ => None,
    }
}

pub fn list_profiles() -> Vec<&'static str> {
    vec![PROFILE_RISCV64IMAC_ZERO_LINUX_MUSL]
}
