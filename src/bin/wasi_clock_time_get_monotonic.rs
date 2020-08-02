// {
// }

fn main() {
    unsafe {
        let time1 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 0).unwrap();
        assert!(time1 > 0);

        let time2 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 1).unwrap();
        assert!(time2 > 0);

        assert!(time1 < time2);
    }
}
