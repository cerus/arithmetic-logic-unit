pub mod nand_gate;
pub mod xor_gate;
pub mod or_gate;
pub mod and_gate;
pub mod not_gate;

pub trait Gate {
    fn calc(&self, a: bool, b: bool) -> bool;
}