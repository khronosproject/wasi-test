// {
//     "preopens": {
//         "fixture": "fixture"
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
            wasi::RIGHTS_FD_WRITE,
            0,
            0,
        ).unwrap();

        let data = b"new_file";
        let ciovec = [wasi::Ciovec {
            buf: data.as_ptr(),
            buf_len: data.len(),
        }];

        let nwritten = wasi::fd_write(file_fd, &ciovec).unwrap();
        assert!(nwritten > 0);
    }
}
