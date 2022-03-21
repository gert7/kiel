use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct PriceCell {
    pub hour: u32,
    pub price: Decimal,
}

#[derive(Clone, Debug)]
pub struct DateColumn {
    pub date: String,
    pub cells: Vec<PriceCell>,
}

pub type PriceMatrix = Vec<DateColumn>;