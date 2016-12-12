#[derive(Deserialize, Debug)]
pub struct Depth {
    asks: Vec<[f64; 2]>,
    bids: Vec<[f64; 2]>,
}
