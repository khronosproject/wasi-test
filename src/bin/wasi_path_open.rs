// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

unsafe fn test_wasi_path_open_parent_directory() {
    let dir_fd = 3;
    let err = wasi::path_open(
        dir_fd,
        0,
        "..",
        wasi::OFLAGS_DIRECTORY,
        0,
        0,
        0,
    ).unwrap_err();

    assert_eq!(err.raw_error(), wasi::ERRNO_NOTCAPABLE);
}

unsafe fn test_wasi_path_open_symlink_to_parent_directory() {
    let dir_fd = 3;
    let err = wasi::path_open(
        dir_fd,
        0,
        "symlink_to_parent_directory",
        wasi::OFLAGS_DIRECTORY,
        0,
        0,
        0,
    ).unwrap_err();

    assert_eq!(err.raw_error(), wasi::ERRNO_NOTCAPABLE);
}

fn main() {
    unsafe {
        test_wasi_path_open_parent_directory();
        test_wasi_path_open_symlink_to_parent_directory();
    }
}
