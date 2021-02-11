// {
// }

unsafe fn test_clock_time_get_monotonic() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 0).unwrap();
    assert!(time1 > 0);

    for _ in 0..100000 {
        let time2 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 1).unwrap();
        assert!(time2 >= time1);

        if time2 > time1 {
            break;
        }
    }

    let time2 = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, 1).unwrap();
    assert!(time2 > time1);
}

unsafe fn test_clock_time_get_realtime() {
    let time1 = wasi::clock_time_get(wasi::CLOCKID_REALTIME, 0).unwrap();
    assert!(time1 > 0);

    for _ in 0..100000 {
        let time2 = wasi::clock_time_get(wasi::CLOCKID_REALTIME, 1).unwrap();
        assert!(time2 >= time1);

        if time2 > time1 {
            break;
        }
    }

    let time2 = wasi::clock_time_get(wasi::CLOCKID_REALTIME, 1).unwrap();
    assert!(time2 > time1);
}

fn main() {
    unsafe {
        test_clock_time_get_monotonic();
        test_clock_time_get_realtime();
    }
}
