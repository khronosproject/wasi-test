// {
// }

unsafe fn test_fd_fdstat_get_stdin() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDIN).unwrap();
    assert_eq!(fdstat.fs_filetype, wasi::FILETYPE_CHARACTER_DEVICE);
    assert!((fdstat.fs_flags & wasi::FDFLAGS_APPEND) != 0);
}

unsafe fn test_fd_fdstat_get_stdout() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDOUT).unwrap();
    assert_eq!(fdstat.fs_filetype, wasi::FILETYPE_CHARACTER_DEVICE);
    assert!((fdstat.fs_flags & wasi::FDFLAGS_APPEND) != 0);
}

unsafe fn test_fd_fdstat_get_stderr() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDERR).unwrap();
    assert_eq!(fdstat.fs_filetype, wasi::FILETYPE_CHARACTER_DEVICE);
    assert!((fdstat.fs_flags & wasi::FDFLAGS_APPEND) != 0);
}

fn main() {
    unsafe {
        test_fd_fdstat_get_stdin();
        test_fd_fdstat_get_stdout();
        test_fd_fdstat_get_stderr();
    }
}
