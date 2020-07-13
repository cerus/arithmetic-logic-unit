use crate::model::gates::or_gate::OrGate;
use crate::model::subtractor::SubtractorResult;
use crate::model::subtractor::half_subtractor::HalfSubtractor;
use crate::model::gates::Gate;

// Declare FullSubtractor struct
pub struct FullSubtractor {
    half_sub: HalfSubtractor,
    or_gate: OrGate,
}

// Implement the previously declared FullSubtractor struct
impl FullSubtractor {
    pub fn new() -> FullSubtractor {
        return FullSubtractor {
            half_sub: HalfSubtractor::new(),
            or_gate: OrGate {},
        };
    }

    // Calculates the result of parameter a, b and c
    pub fn calc(&self, a: bool, b: bool, c: bool) -> SubtractorResult {
        // Calculate first result
        let res_1 = self.half_sub.calc(a, b);
        // Calculate second result
        let res_2 = self.half_sub.calc(res_1.diff, c);

        // Return the result
        return SubtractorResult {
            diff: res_2.diff,
            // Set borrow to or(borrow2, borrow1)
            borrow: self.or_gate.calc(res_2.borrow, res_1.borrow),
        };
    }
}