// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  let metadata = std::fs::metadata("fixtures/directory").unwrap();
  assert!(metadata.is_dir());

  let metadata = std::fs::metadata("fixtures/symlink_to_directory").unwrap();
  assert!(metadata.is_dir());

  let metadata = std::fs::metadata("fixtures/file").unwrap();
  assert!(metadata.is_file());
  assert_eq!(metadata.len(), 5);

  let metadata = std::fs::metadata("fixtures/symlink_to_file").unwrap();
  assert!(metadata.is_file());
  assert_eq!(metadata.len(), 5);

  let metadata = std::fs::metadata("fixtures/directory/file").unwrap();
  assert!(metadata.is_file());
  assert_eq!(metadata.len(), 15);

  let metadata =
    std::fs::metadata("fixtures/directory/symlink_to_file").unwrap();
  assert!(metadata.is_file());
  assert_eq!(metadata.len(), 15);
}
