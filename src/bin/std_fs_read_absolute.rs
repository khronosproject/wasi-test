// {
//     "preopens": {
//         "/fixtures": "fixtures"
//     }
// }

fn main() {
  assert_eq!(std::fs::read("/fixtures/file").unwrap(), b"file\n");
  assert_eq!(
    std::fs::read("/fixtures/symlink_to_file").unwrap(),
    b"file\n"
  );
  assert_eq!(
    std::fs::read("/fixtures/directory/file").unwrap(),
    b"directory/file\n"
  );
  assert_eq!(
    std::fs::read("/fixtures/directory/symlink_to_file").unwrap(),
    b"directory/file\n"
  );
}
