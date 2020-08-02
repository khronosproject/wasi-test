// {
//     "exitCode": 1
// }

fn main() {
    unsafe {
        wasi::proc_exit(1);
        unreachable!();
    }
}
