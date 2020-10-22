// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  let entries = std::fs::read_dir("fixtures").unwrap();
  assert_eq!(entries.count(), 5);

  let entries = std::fs::read_dir("fixtures/directory").unwrap();
  assert_eq!(entries.count(), 2);
}
