const PRECISION: u128 = 1_000_000_000_000_000_000;

struct Fees {
    fee_in_percent: u128,
    fee_out_percent: u128,
}

pub fn new(fee_in_percent: u128, fee_out_percent: u128) -> Fees {
    Fees {
        fee_in_percent,
        fee_out_percent,
    }
}

pub fn fee_in_percent(fees: &Fees) -> u128 {
    fees.fee_in_percent
}

pub fn fee_out_percent(fees: &Fees) -> u128 {
    fees.fee_out_percent
}

pub fn get_fee_in_amount(fees: &Fees, amount: u64) -> u64 {
    get_fee_amount(amount, fees.fee_in_percent)
}

pub fn get_fee_out_amount(fees: &Fees, amount: u64) -> u64 {
    get_fee_amount(amount, fees.fee_out_percent)
}

pub fn get_fee_in_initial_amount(fees: &Fees, amount: u64) -> u64 {
    get_initial_amount(amount, fees.fee_in_percent)
}

pub fn get_fee_out_initial_amount(fees: &Fees, amount: u64) -> u64 {
    get_initial_amount(amount, fees.fee_out_percent)
}

fn get_fee_amount(x: u64, percent: u256) -> u64 {
    mul_div_up(x as u256, percent, PRECISION) as u64
}

fn get_initial_amount(x: u64, percent: u256) -> u64 {
    mul_div_up(x as u256, PRECISION, PRECISION - percent) as u64
}
