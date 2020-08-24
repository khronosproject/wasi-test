// {
// }

fn main() {
    unsafe {
        let fdstat = wasi::fd_fdstat_get(wasi::FD_STDIN).unwrap();
        assert_eq!(fdstat.fs_filetype, wasi::FILETYPE_CHARACTER_DEVICE);
        assert!((fdstat.fs_flags & wasi::FDFLAGS_APPEND) != 0);
    }
}
