// {
//     "preopens": {
//         "fixtures": "fixtures"
//     },
//     "files": {
//         "fixtures/new_file": "new_file"
//     }
// }

fn main() {
  assert!(std::fs::write("fixtures/new_file", b"new_file").is_ok())
}
