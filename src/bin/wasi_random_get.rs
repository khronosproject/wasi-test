// {
// }

fn main() {
    unsafe {
        let mut buf = Vec::new();
        buf.resize(1024, 0);

        assert!(wasi::random_get(buf.as_mut_ptr(), buf.len()).is_ok());
        assert!(buf.iter().any(|x| *x != 0));
    }
}
