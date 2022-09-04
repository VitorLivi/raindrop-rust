fn raindrops(n: u32) -> String {
  let mut raindrop = String::new();

  let mut if_is_factor_push = |factor, sound| {
    if n % factor == 0 {
      raindrop.push_str(sound);
    }
  };

  if_is_factor_push(3, "Pling");
  if_is_factor_push(5, "Plang");
  if_is_factor_push(7, "Plong");

  if raindrop.is_empty() {
    raindrop = n.to_string();
  }

  raindrop
}

fn main() {
  let result = raindrops(10);
  println!("{}", result);
}
