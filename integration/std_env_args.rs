// { "args": ["one", "two", "three" ]}

fn main() {
  let mut args = std::env::args();
  assert_eq!(args.len(), 4);
  assert!(args.next().unwrap().contains("std_env_args.wasm"));
  assert_eq!(args.next().unwrap(), "one");
  assert_eq!(args.next().unwrap(), "two");
  assert_eq!(args.next().unwrap(), "three");
}
