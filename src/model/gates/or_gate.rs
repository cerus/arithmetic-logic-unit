use crate::model::gates::Gate;

// Declare OrGate struct
pub struct OrGate {}

// Implement the Gate trait for the previously declared OrGate struct
impl Gate for OrGate {
    fn calc(&self, a: bool, b: bool) -> bool {
        return a || b;
    }
}