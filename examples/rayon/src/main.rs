#![cfg_attr(target_os = "none", no_std)]
#![no_main]

cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use platform::println;
    } else {
        use std::println;
        #[allow(unused_imports)]
        use platform;
    }
}

fn parallel_sum_of_squares(pool: &rayon::ThreadPool, n: u32) -> u64 {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    pool.install(|| {
        (1..=n)
            .into_par_iter()
            .map(|x| (x as u64) * (x as u64))
            .sum()
    })
}

#[no_mangle]
fn main() -> ! {
    println!("Rayon Parallel Computing Example");
    println!("=================================");

    let gp: usize;
    unsafe {
        core::arch::asm!("mv {}, gp", out(reg) gp);
    }
    println!("Initial gp = 0x{gp:016x}");

    let pool = match rayon::ThreadPoolBuilder::new().build() {
        Ok(p) => p,
        Err(e) => {
            println!("Error: Failed to create thread pool: {:?}", e);
            platform::exit(1)
        }
    };

    let n = 101;

    println!("\nComputing sum of squares from 1 to {}...", n);

    let result = parallel_sum_of_squares(&pool, n);
    println!("Result: {}", result);

    println!("\nComputation completed successfully!");
    println!("Note: Rayon executed on current thread only (no worker threads).");
    platform::exit(0)
}
