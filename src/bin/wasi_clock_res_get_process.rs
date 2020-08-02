// {
// }

fn main() {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_PROCESS_CPUTIME_ID).unwrap();
        assert!(resolution > 0);
    }
}
