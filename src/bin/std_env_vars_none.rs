// {
//     "env": {
//     }
// }

fn main() {
  let vars = std::env::vars();
  assert_eq!(vars.count(), 0);
}
