use crate::model::gates::Gate;

// Declare NandGate struct
pub struct NandGate {}

// Implement the Gate trait for the previously declared NandGate struct
impl Gate for NandGate {
    fn calc(&self, a: bool, b: bool) -> bool {
        return !(a && b);
    }
}