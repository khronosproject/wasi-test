// {
//     "preopens": {
//         "fixture": "fixture"
//     },
//     "files": {
//         "fixture/new_file": "new_file"
//     }
// }

fn main() {
  assert!(std::fs::write("fixture/new_file", b"new_file").is_ok())
}
