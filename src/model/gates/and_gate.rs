use crate::model::gates::Gate;

// Declare AndGate struct
pub struct AndGate {}

// Implement the Gate trait for the previously declared AndGate struct
impl Gate for AndGate {
    fn calc(&self, a: bool, b: bool) -> bool {
        return a && b;
    }
}