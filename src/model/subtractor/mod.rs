pub mod half_subtractor;
pub mod full_subtractor;
pub mod ripple_carry_subtractor;

pub struct SubtractorResult {
    pub diff: bool,
    pub borrow: bool,
}