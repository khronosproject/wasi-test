// {
//     "args": [
//     ]
// }

fn main() {
  let mut args = std::env::args();
  assert_eq!(args.len(), 1);
  assert!(args.next().unwrap().contains("std_env_args_none.wasm"));
}
