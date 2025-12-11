//! LLVM-specific properties

/// LLVM configuration (all properties provided by user)
#[derive(Debug, Clone)]
pub struct LLVMConfig {
    /// LLVM target triple (e.g., "riscv64-unknown-linux-musl")
    /// Note: LLVM target typically uses "unknown" as vendor
    pub llvm_target: String,
    /// ISA features for LLVM (e.g., "+m,+a,+c")
    pub features: String,
    /// ABI/calling convention (e.g., "lp64", "ilp32")
    pub abi: String,
    /// LLVM data layout string
    pub data_layout: String,
}
