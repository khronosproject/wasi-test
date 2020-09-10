// {
// }

fn main() {
    unsafe {
        assert!(wasi::sched_yield().is_ok());
    }
}
