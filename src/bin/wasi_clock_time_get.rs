// {
// }

unsafe fn test_clock_time_get_monotonic() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 0).unwrap();
    assert!(time1 > 0);

    let time2 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 1).unwrap();
    assert!(time2 > time1);
}

unsafe fn test_clock_time_get_realtime() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_REALTIME, 0).unwrap();
    assert!(time1 > 0);

    let time2 = wasi::clock_time_get(wasi::CLOCKID_REALTIME, 1).unwrap();
    assert!(time2 > time1);
}

unsafe fn test_clock_time_get_process_cputime_id() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_PROCESS_CPUTIME_ID, 0).unwrap();
    assert!(time1 > 0);

    let time2 = wasi::clock_time_get(wasi::CLOCKID_PROCESS_CPUTIME_ID, 1).unwrap();
    assert!(time2 > time1);
}

unsafe fn test_clock_time_get_thread_cputime_id() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_THREAD_CPUTIME_ID, 0).unwrap();
    assert!(time1 > 0);

    let time2 = wasi::clock_time_get(wasi::CLOCKID_THREAD_CPUTIME_ID, 1).unwrap();
    assert!(time2 > time1);
}

fn main() {
    unsafe {
        test_clock_time_get_monotonic();
        test_clock_time_get_realtime();
        test_clock_time_get_process_cputime_id();
        test_clock_time_get_thread_cputime_id();
    }
}
