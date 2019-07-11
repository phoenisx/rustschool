pub fn raindrops(n: u32) -> String {
  let mut rain_str = String::with_capacity(15); // with max capacity
  if n % 3 == 0 {
    rain_str.push_str("Pling");
  }
  if n % 5 == 0 {
    rain_str.push_str("Plang");
  }
  if n % 7 == 0 {
    rain_str.push_str("Plong");
  }

  if rain_str.is_empty() {
    return n.to_string();
  }

  rain_str
}
