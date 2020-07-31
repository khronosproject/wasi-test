// {
//     "preopens": {
//         "fixture": "fixture"
//     }
// }

fn main() {
  let file = std::fs::File::create("fixture/file").unwrap();

  assert!(file.set_len(5).is_ok());
  assert!(file.sync_all().is_ok());
  let metadata = std::fs::metadata("fixture/file").unwrap();
  assert_eq!(metadata.len(), 5);

  assert!(file.set_len(25).is_ok());
  assert!(file.sync_all().is_ok());
  let metadata = std::fs::metadata("fixture/file").unwrap();
  assert_eq!(metadata.len(), 25);
}
