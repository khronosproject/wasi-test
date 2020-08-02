// {
//     "exitCode": 0
// }

fn main() {
    unsafe {
        wasi::proc_exit(0);
        unreachable!();
    }
}
