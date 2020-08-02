// {
// }

fn main() {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_REALTIME).unwrap();
        assert!(resolution > 0);
    }
}
