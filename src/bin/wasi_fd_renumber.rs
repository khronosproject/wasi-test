// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
    unsafe {
        let dir_fd = 3;
        let from_fd = wasi::path_open(
            dir_fd,
            0,
            "file1",
            wasi::OFLAGS_CREAT,
            wasi::RIGHTS_FD_READ | wasi::RIGHTS_FD_WRITE,
            0,
            0,
        ).unwrap();

        let to_fd = wasi::path_open(
            dir_fd,
            0,
            "to_file",
            wasi::OFLAGS_CREAT,
            wasi::RIGHTS_FD_READ | wasi::RIGHTS_FD_WRITE,
            0,
            0,
        ).unwrap();

        assert!(wasi::fd_renumber(from_fd, to_fd).is_ok());
    }
}
