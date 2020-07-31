// {
//     "preopens": {
//         "fixture": "fixture"
//     }
// }

fn main() {
  assert!(std::fs::create_dir("fixture/new_directory").is_ok());
  assert!(std::fs::metadata("fixture/new_directory").unwrap().is_dir());
}
