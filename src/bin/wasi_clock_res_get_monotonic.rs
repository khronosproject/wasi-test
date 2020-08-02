// {
// }

fn main() {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_MONOTONIC).unwrap();
        assert!(resolution > 0);
    }
}
