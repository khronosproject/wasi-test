// {
//     "preopens": {
//         "fixtures": "fixtures"
//     }
// }

fn main() {
    assert!(std::fs::remove_dir_all("fixtures/directory").is_ok());
}
