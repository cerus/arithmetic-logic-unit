use crate::model::binary_representation::BinaryRepresentation;
use crate::model::adder::half_adder::HalfAdder;
use crate::model::adder::full_adder::FullAdder;
use std::cmp::max;

// Declare RippleCarryAdder
pub struct RippleCarryAdder {
    full_adder: FullAdder,
}

// Implement the previously declared RippleCarryAdder struct
impl RippleCarryAdder {
    // Returns a new RippleCarryAdder object
    pub fn new() -> RippleCarryAdder {
        return RippleCarryAdder {
            full_adder: FullAdder::new(),
        };
    }

    // Calculates the sum of a and b
    pub fn calc(&self, a: &BinaryRepresentation, b: &BinaryRepresentation) -> BinaryRepresentation {
        // Set length to longest array length
        let m_len = max(a.arr.len(), b.arr.len());

        // Create a clone of a
        let mut a_clone = BinaryRepresentation::from_vec((*a.arr).to_owned());
        // Transform (pad with zeros) and reverse the clone
        a_clone.transform(m_len);
        a_clone.reverse();

        // Create a clone of b
        let mut b_clone = BinaryRepresentation::from_vec((*b.arr).to_owned());
        // Transform (pad with zeros) and reverse the clone
        b_clone.transform(m_len);
        b_clone.reverse();

        // Declare the vec that holds the result and the carry bit that gets rippled along
        let mut res: Vec<bool> = Vec::new();
        let mut carry = false;

        // Loop
        for i in 0..m_len {
            // Try to fetch bit at index i
            let bit_a = a_clone.arr.get(i)
                .or(Some(&false))
                .expect("Failed to fetch bool");
            let bit_b = b_clone.arr.get(i)
                .or(Some(&false))
                .expect("Failed to fetch bool");

            // Calculate result of the bits at index i, set the carry bit to the calculated
            // carry bit and push the calculated sum to the result vec
            let result = self.full_adder.calc(*bit_a, *bit_b, carry);
            carry = result.carry;
            res.push(result.sum);
        }

        // Push the last carry bit and reverse the result vec
        res.push(carry);
        res.reverse();

        // Wrap the result vec in a BinaryRepresentation
        return BinaryRepresentation::from_vec(res);
    }
}