// {
// }

unsafe fn test_fd_fdstat_get_stdin() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDIN).unwrap();

    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_BLOCK_DEVICE);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_DIRECTORY);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SOCKET_DGRAM);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SYMBOLIC_LINK);

    assert!((fdstat.fs_flags & wasi::FDFLAGS_NONBLOCK) == 0);

    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_FD_READ),
        wasi::RIGHTS_FD_READ
    );
    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_POLL_FD_READWRITE),
        wasi::RIGHTS_POLL_FD_READWRITE
    );
}

unsafe fn test_fd_fdstat_get_stdout() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDOUT).unwrap();

    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_BLOCK_DEVICE);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_DIRECTORY);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SOCKET_DGRAM);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SYMBOLIC_LINK);

    assert!((fdstat.fs_flags & wasi::FDFLAGS_NONBLOCK) == 0);

    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_FD_WRITE),
        wasi::RIGHTS_FD_WRITE
    );
    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_POLL_FD_READWRITE),
        wasi::RIGHTS_POLL_FD_READWRITE
    );
}

unsafe fn test_fd_fdstat_get_stderr() {
    let fdstat = wasi::fd_fdstat_get(wasi::FD_STDERR).unwrap();

    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_BLOCK_DEVICE);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_DIRECTORY);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SOCKET_DGRAM);
    assert_ne!(fdstat.fs_filetype, wasi::FILETYPE_SYMBOLIC_LINK);

    assert!((fdstat.fs_flags & wasi::FDFLAGS_NONBLOCK) == 0);

    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_FD_WRITE),
        wasi::RIGHTS_FD_WRITE
    );
    assert_eq!(
        (fdstat.fs_rights_base & wasi::RIGHTS_POLL_FD_READWRITE),
        wasi::RIGHTS_POLL_FD_READWRITE
    );
}

fn main() {
    unsafe {
        test_fd_fdstat_get_stdin();
        test_fd_fdstat_get_stdout();
        test_fd_fdstat_get_stderr();
    }
}
