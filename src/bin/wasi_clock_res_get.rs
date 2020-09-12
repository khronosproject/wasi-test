// {
// }

unsafe fn test_clock_res_get_monotonic() {
    let res = wasi::clock_res_get(wasi::CLOCKID_MONOTONIC).unwrap();
    assert!(res > 0);
}

unsafe fn test_clock_res_get_realtime() {
    let res = wasi::clock_res_get(wasi::CLOCKID_REALTIME).unwrap();
    assert!(res > 0);
}

unsafe fn test_clock_res_get_process_cputime_id() {
    let res = wasi::clock_res_get(wasi::CLOCKID_PROCESS_CPUTIME_ID).unwrap();
    assert!(res > 0);
}

unsafe fn test_clock_res_get_thread_cputime_id() {
    let res = wasi::clock_res_get(wasi::CLOCKID_THREAD_CPUTIME_ID).unwrap();
    assert!(res > 0);
}

fn main() {
    unsafe {
        test_clock_res_get_monotonic();
        test_clock_res_get_realtime();
        test_clock_res_get_process_cputime_id();
        test_clock_res_get_thread_cputime_id();
    }
}
