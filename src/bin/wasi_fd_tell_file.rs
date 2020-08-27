// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
    unsafe {
        let dir_fd = 3;
        let file_fd = wasi::path_open(
            dir_fd,
            0,
            "new_file",
            wasi::OFLAGS_CREAT,
            wasi::RIGHTS_FD_WRITE | wasi::RIGHTS_FD_TELL,
            0,
            0,
        ).unwrap();

        let offset = wasi::fd_tell(file_fd).unwrap();
        assert_eq!(offset, 0);

        let data = b"0";
        let ciovec = [wasi::Ciovec {
            buf: data.as_ptr(),
            buf_len: data.len(),
        }];

        let nwritten = wasi::fd_write(file_fd, &ciovec).unwrap();
        assert_eq!(nwritten, 1);

        let offset = wasi::fd_tell(file_fd).unwrap();
        assert_eq!(offset, 1);

        let nwritten = wasi::fd_write(file_fd, &ciovec).unwrap();
        assert_eq!(nwritten, 1);

        let offset = wasi::fd_tell(file_fd).unwrap();
        assert_eq!(offset, 2);

        let nwritten = wasi::fd_write(file_fd, &ciovec).unwrap();
        assert_eq!(nwritten, 1);

        let offset = wasi::fd_tell(file_fd).unwrap();
        assert_eq!(offset, 3);
    }
}
