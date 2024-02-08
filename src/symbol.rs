use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]

pub struct Symbol {
    pub id: i64,
    pub ask: f64,
    pub bid: f64,
}
