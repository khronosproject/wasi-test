// {
//     "preopens": {
//         "/fixture": "fixture"
//     }
// }

fn main() {
  assert!(
    std::fs::hard_link("/fixture/file", "/fixture/hardlink_to_file").is_ok()
  );
  assert_eq!(
    std::fs::read("/fixture/file").unwrap(),
    std::fs::read("/fixture/hardlink_to_file").unwrap()
  );
}
