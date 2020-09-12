// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  let file = std::fs::File::open("fixtures/file").unwrap();
  let metadata = file.metadata().unwrap();
  assert!(metadata.is_file());
  assert!(metadata.len() == 5);

  let file = std::fs::File::open("fixtures/symlink_to_file").unwrap();
  let metadata = file.metadata().unwrap();
  assert!(metadata.is_file());
  assert!(metadata.len() == 5);

  let file = std::fs::File::open("fixtures/directory/file").unwrap();
  let metadata = file.metadata().unwrap();
  assert!(metadata.is_file());
  assert!(metadata.len() == 15);

  let file = std::fs::File::open("fixtures/directory/symlink_to_file").unwrap();
  let metadata = file.metadata().unwrap();
  assert!(metadata.is_file());
  assert!(metadata.len() == 15);
}
