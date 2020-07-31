// {
//     "preopens": {
//         "/fixture": "fixture"
//     }
// }

fn main() {
  assert!(std::fs::File::create("/fixture/new_file").is_ok());
  assert!(std::fs::metadata("/fixture/new_file").unwrap().is_file());
}
