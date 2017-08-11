use serde::de::{self, Deserialize};

 
  pub fn string_to_f64<'de, D>(d: D) -> Result<f64, D::Error>
      where D: de::Deserializer<'de>
  {

      Ok((String::deserialize(d)?).parse().expect("Parse error"))
  }

  pub fn string_to_u64<'de, D>(d: D) -> Result<u64, D::Error>
      where D: de::Deserializer<'de>
  {

      Ok((String::deserialize(d)?).parse().expect("Parse error"))
  }
  