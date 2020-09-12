// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  let file = std::fs::File::create("fixtures/file").unwrap();

  assert!(file.set_len(5).is_ok());
  assert!(file.sync_all().is_ok());
  let metadata = std::fs::metadata("fixtures/file").unwrap();
  assert_eq!(metadata.len(), 5);

  assert!(file.set_len(25).is_ok());
  assert!(file.sync_all().is_ok());
  let metadata = std::fs::metadata("fixtures/file").unwrap();
  assert_eq!(metadata.len(), 25);
}
