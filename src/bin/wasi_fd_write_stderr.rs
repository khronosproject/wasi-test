// {
//     "stderr": "stderr"
// }

fn main() {
    unsafe {
        let data = b"stderr";
        let ciovec = [wasi::Ciovec {
            buf: data.as_ptr(),
            buf_len: data.len(),
        }];

        let nwritten = wasi::fd_write(wasi::FD_STDERR, &ciovec).unwrap();
        assert!(nwritten > 0);
    }
}
