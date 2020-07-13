use crate::model::gates::Gate;

// Declare XorGate struct
pub struct XorGate {}

// Implement the Gate trait for the previously declared XorGate struct
impl Gate for XorGate {
    fn calc(&self, a: bool, b: bool) -> bool {
        return a ^ b;
    }
}