// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  let metadata = std::fs::symlink_metadata("fixtures/directory").unwrap();
  assert!(metadata.file_type().is_dir());

  let metadata =
    std::fs::symlink_metadata("fixtures/symlink_to_directory").unwrap();
  assert!(metadata.file_type().is_symlink());

  let metadata = std::fs::symlink_metadata("fixtures/file").unwrap();
  assert!(metadata.file_type().is_file());

  let metadata = std::fs::symlink_metadata("fixtures/symlink_to_file").unwrap();
  assert!(metadata.file_type().is_symlink());

  let metadata = std::fs::symlink_metadata("fixtures/directory/file").unwrap();
  assert!(metadata.file_type().is_file());

  let metadata =
    std::fs::symlink_metadata("fixtures/directory/symlink_to_file").unwrap();
  assert!(metadata.file_type().is_symlink());
}
