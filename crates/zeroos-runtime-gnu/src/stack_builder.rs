pub unsafe fn build_gnu_stack(
    stack_top: usize,
    _ehdr_start: usize,
    _program_name: &'static [u8],
) -> usize {
    stack_top
}
