use crate::model::adder::AdderResult;
use crate::model::gates::and_gate::AndGate;
use crate::model::gates::xor_gate::XorGate;
use crate::model::gates::Gate;

// Declare HalfAdder struct
pub struct HalfAdder {
    and_gate: AndGate,
    xor_gate: XorGate,
}

// Implement the previously declared HalfAdder struct
impl HalfAdder {
    // Returns a new HalfAdder object
    pub fn new() -> HalfAdder {
        return HalfAdder {
            and_gate: AndGate {},
            xor_gate: XorGate {},
        };
    }

    // Calculates the result of parameter a and b
    pub fn calc(&self, a: bool, b: bool) -> AdderResult {
        // Return the result
        return AdderResult {
            // Set sum to result of xor(a, b)
            sum: self.xor_gate.calc(a, b),
            // Set carry to and(a, b)
            carry: self.and_gate.calc(a, b),
        };
    }
}