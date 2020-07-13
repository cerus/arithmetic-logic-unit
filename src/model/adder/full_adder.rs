use crate::model::adder::half_adder::HalfAdder;
use crate::model::gates::or_gate::OrGate;
use crate::model::adder::AdderResult;
use crate::model::gates::Gate;

// Declare FullAdder struct
pub struct FullAdder {
    half_adder: HalfAdder,
    or_gate: OrGate,
}

// Implement the previously declared FullAdder struct
impl FullAdder {
    // Returns a new FullAdder object
    pub fn new() -> FullAdder {
        return FullAdder {
            half_adder: HalfAdder::new(),
            or_gate: OrGate {},
        };
    }

    // Calculates the result of parameter a, b and c
    pub fn calc(&self, a: bool, b: bool, c: bool) -> AdderResult {
        // Calculate first result
        let result_1 = self.half_adder.calc(a, b);
        // Calculate second result
        let result_2 = self.half_adder.calc(result_1.sum, c);

        // Return the result
        return AdderResult {
            sum: result_2.sum,
            // Set carry to or(carry1, carry2)
            carry: self.or_gate.calc(result_2.carry, result_1.carry),
        };
    }
}