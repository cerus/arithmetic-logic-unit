use crate::model::gates::and_gate::AndGate;
use crate::model::gates::xor_gate::XorGate;
use crate::model::gates::not_gate::NotGate;
use crate::model::subtractor::SubtractorResult;
use crate::model::gates::Gate;

// Declare HalfSubtractor struct
pub struct HalfSubtractor {
    and_gate: AndGate,
    xor_gate: XorGate,
    not_gate: NotGate,
}

// Implement the previously declared HalfSubtractor struct
impl HalfSubtractor {
    // Returns a new HalfSubtractor object
    pub fn new() -> HalfSubtractor {
        return HalfSubtractor {
            and_gate: AndGate {},
            xor_gate: XorGate {},
            not_gate: NotGate {},
        };
    }

    // Calculates the result of parameter a and b
    pub fn calc(&self, a: bool, b: bool) -> SubtractorResult {
        // Return the result
        return SubtractorResult {
            // Set diff to result of xor(a, b)
            diff: self.xor_gate.calc(a, b),
            // Set borrow to and(not(a), b)
            borrow: self.and_gate.calc(self.not_gate.calc(a), b),
        };
    }
}