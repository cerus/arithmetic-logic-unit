pub mod ripple_carry_adder;
pub mod full_adder;
pub mod half_adder;

pub struct AdderResult {
    pub sum: bool,
    pub carry: bool,
}