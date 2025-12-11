#[inline]
pub fn krandom(buf: *mut u8, len: usize) -> isize {
    unsafe { (crate::KERNEL.random.fill_bytes)(buf, len) }
}

pub trait KRandom: Sized {
    fn random() -> Self;
}

impl KRandom for u8 {
    #[inline]
    fn random() -> Self {
        let mut val = 0u8;
        krandom(&mut val as *mut u8, 1);
        val
    }
}

impl KRandom for u16 {
    #[inline]
    fn random() -> Self {
        let mut val = 0u16;
        krandom(&mut val as *mut u16 as *mut u8, 2);
        val
    }
}

impl KRandom for u32 {
    #[inline]
    fn random() -> Self {
        let mut val = 0u32;
        krandom(&mut val as *mut u32 as *mut u8, 4);
        val
    }
}

impl KRandom for u64 {
    #[inline]
    fn random() -> Self {
        let mut val = 0u64;
        krandom(&mut val as *mut u64 as *mut u8, 8);
        val
    }
}

impl KRandom for u128 {
    #[inline]
    fn random() -> Self {
        let mut val = 0u128;
        krandom(&mut val as *mut u128 as *mut u8, 16);
        val
    }
}

impl KRandom for usize {
    #[inline]
    fn random() -> Self {
        let mut val = 0usize;
        krandom(
            &mut val as *mut usize as *mut u8,
            core::mem::size_of::<usize>(),
        );
        val
    }
}

impl KRandom for i8 {
    #[inline]
    fn random() -> Self {
        let mut val = 0i8;
        krandom(&mut val as *mut i8 as *mut u8, 1);
        val
    }
}

impl KRandom for i16 {
    #[inline]
    fn random() -> Self {
        let mut val = 0i16;
        krandom(&mut val as *mut i16 as *mut u8, 2);
        val
    }
}

impl KRandom for i32 {
    #[inline]
    fn random() -> Self {
        let mut val = 0i32;
        krandom(&mut val as *mut i32 as *mut u8, 4);
        val
    }
}

impl KRandom for i64 {
    #[inline]
    fn random() -> Self {
        let mut val = 0i64;
        krandom(&mut val as *mut i64 as *mut u8, 8);
        val
    }
}

impl KRandom for i128 {
    #[inline]
    fn random() -> Self {
        let mut val = 0i128;
        krandom(&mut val as *mut i128 as *mut u8, 16);
        val
    }
}

impl KRandom for isize {
    #[inline]
    fn random() -> Self {
        let mut val = 0isize;
        krandom(
            &mut val as *mut isize as *mut u8,
            core::mem::size_of::<isize>(),
        );
        val
    }
}

#[inline]
pub fn krandom_seed(seed: u64) {
    unsafe { (crate::KERNEL.random.init_seed)(seed) }
}
