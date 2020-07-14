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
        let res_1 = self.half_adder.calc(a, b);
        // Calculate second result
        let res_2 = self.half_adder.calc(res_1.sum, c);

        // Return the result
        return AdderResult {
            sum: res_2.sum,
            // Set carry to or(carry1, carry2)
            carry: self.or_gate.calc(res_2.carry, res_1.carry),
        };
    }
}
