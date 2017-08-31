use types::trade::TradeType;

#[derive(Deserialize, Debug, Serialize)]
pub struct DepthC {
    pub asks: Vec<[f64; 2]>,
    pub bids: Vec<[f64; 2]>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Depth {
    pub depth: Vec<DepthValue>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DepthValue {
    pub price: f64,
    pub amount: f64,
    pub ask_type: TradeType,
}

impl From<DepthC> for Depth {
    fn from(d: DepthC) -> Self {
        let mut c = Depth { depth: vec![] };
        for i in d.asks {
            c.depth.push(DepthValue {
                price: i[0],
                amount: i[1],
                ask_type: TradeType::Ask,
            });
        }
        for i in d.bids {
            c.depth.push(DepthValue {
                price: i[0],
                amount: i[1],
                ask_type: TradeType::Bid,
            });
        }
        c
    }
}
