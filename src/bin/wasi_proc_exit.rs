// {
//     "exitCode": 1
// }

unsafe fn test_proc_exit_one() {
    wasi::proc_exit(1);
    unreachable!();
}

fn main() {
    unsafe {
        test_proc_exit_one();
    }
}
