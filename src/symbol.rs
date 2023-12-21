// mod ctrader_open_api;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug, Default)]

pub struct Symbol {
    pub id: i64,

    #[derivative(Default(value = "0"))]
    pub bid: u64,

    #[derivative(Default(value = "0"))]
    pub ask: u64,

    #[derivative(Default(value = "0"))]
    pub spread: u64,
}

impl Symbol {
    fn get_spread(ask: u64, bid: u64) -> u64 {
        ask - bid
    }
}
