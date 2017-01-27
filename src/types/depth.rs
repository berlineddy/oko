#[derive(Deserialize, Debug)]
pub struct Depth {
    pub asks: Vec<[f64; 2]>,
    pub bids: Vec<[f64; 2]>,
}
