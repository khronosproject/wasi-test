// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  assert!(std::fs::File::create("fixtures/new_file").is_ok());
  assert!(std::fs::metadata("fixtures/new_file").unwrap().is_file());
}
