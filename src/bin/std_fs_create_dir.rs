// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  assert!(std::fs::create_dir("fixtures/new_directory").is_ok());
  assert!(std::fs::metadata("fixtures/new_directory").unwrap().is_dir());
}
