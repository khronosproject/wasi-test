// {
//     "stdout": "stdout"
// }

fn main() {
    unsafe {
        let data = b"stdout";
        let ciovec = [wasi::Ciovec {
            buf: data.as_ptr(),
            buf_len: data.len(),
        }];

        let nwritten = wasi::fd_write(wasi::FD_STDOUT, &ciovec).unwrap();
        assert!(nwritten > 0);
    }
}
