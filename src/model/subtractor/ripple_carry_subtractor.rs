use crate::model::binary_representation::BinaryRepresentation;
use std::cmp::max;
use crate::model::subtractor::half_subtractor::HalfSubtractor;
use crate::model::subtractor::full_subtractor::FullSubtractor;

// Declare RippleCarrySubtractor
pub struct RippleCarrySubtractor {
    full_sub: FullSubtractor,
}

// Implement the previously declared RippleCarrySubtractor struct
impl RippleCarrySubtractor {
    // Returns a new RippleCarrySubtractor object
    pub fn new() -> RippleCarrySubtractor {
        return RippleCarrySubtractor {
            full_sub: FullSubtractor::new(),
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

        // Declare the vec that holds the result and the borrow bit that gets rippled along
        let mut res: Vec<bool> = Vec::new();
        let mut borrow = false;

        // Loop
        for i in 0..m_len {
            // Try to fetch bit at index i
            let bit_a = a_clone.arr.get(i)
                .or(Some(&false))
                .expect("Failed to fetch bool");
            let bit_b = b_clone.arr.get(i)
                .or(Some(&false))
                .expect("Failed to fetch bool");

            // Calculate result of the bits at index i, set the borrow bit to the calculated
            // borrow bit and push the calculated diff to the result vec
            let result = self.full_sub.calc(*bit_a, *bit_b, borrow);
            borrow = result.borrow;
            res.push(result.diff);
        }

        // Push the last borrow bit and reverse the result vec
        res.push(borrow);
        res.reverse();

        // Wrap the result vec in a BinaryRepresentation
        return BinaryRepresentation::from_vec(res);
    }
}
