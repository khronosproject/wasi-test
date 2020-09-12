// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
  assert!(
    std::fs::hard_link("fixtures/file", "fixtures/hardlink_to_file").is_ok()
  );
  assert_eq!(
    std::fs::read("fixtures/file").unwrap(),
    std::fs::read("fixtures/hardlink_to_file").unwrap()
  );
}
