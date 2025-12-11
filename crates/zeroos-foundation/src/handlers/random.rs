use crate::kfn;

pub fn sys_getrandom(buf: usize, buflen: usize, _flags: usize) -> isize {
    kfn::krandom(buf as *mut u8, buflen)
}
