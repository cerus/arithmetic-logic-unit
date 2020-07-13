// Declare BinaryRepresentation struct
pub struct BinaryRepresentation {
    pub(crate) arr: Vec<bool>,
}

// Implement the previously declared BinaryRepresentation struct
impl BinaryRepresentation {
    // Returns a new BinaryRepresentation object
    pub fn new() -> BinaryRepresentation {
        return BinaryRepresentation { arr: Vec::new() };
    }

    // Creates and returns a new BinaryRepresentation object from the provided vec
    pub fn from_vec(vec: Vec<bool>) -> BinaryRepresentation {
        return BinaryRepresentation { arr: vec };
    }

    // Creates and returns a new BinaryRepresentation object from the provided number
    pub fn from_num(num: i32) -> BinaryRepresentation {
        let mut bin_rep = BinaryRepresentation::new();
        bin_rep.set_number(num);
        return bin_rep;
    }

    // Convert the provided number to binary and set the vec of
    // this representation to the converted binary
    pub fn set_number(&mut self, num: i32) {
        self.arr.clear();

        let mut temp_var: i32;
        let mut quotient = num / 2;
        let mut remainder = num % 2;

        // Push the first bit
        self.arr.push(if remainder == 0 { false } else { true });

        // Loop
        while quotient > 0 {
            temp_var = quotient;

            quotient = temp_var / 2;
            remainder = temp_var % 2;

            // Push the calculated bit
            self.arr.push(if remainder == 0 { false } else { true });
        }

        // Reverse the result
        self.reverse();
    }

    // Zero-pads the vec of this representation
    // If the length of the vec of this representation is greater than the
    // provided length nothing will happen
    pub fn transform(&mut self, len: usize) {
        if self.len() >= len {
            // Abort
            return;
        }

        let mut vec: Vec<bool> = Vec::new();

        let mut diff = len - self.len();
        while diff > 0 {
            // Push false (0) while the difference is greater than 0
            vec.push(false);
            diff -= 1;
        }

        // Append the vec of this representation to the zero filled vec and overwrite
        // the vec of this representation with the new vec
        vec.append(&mut self.arr);
        self.arr = vec;
    }

    // Reverse this representation
    pub fn reverse(&mut self) {
        self.arr.reverse()
    }

    // Convert this representation to a string
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for x in &self.arr {
            str.push(if *x { '1' } else { '0' });
        }
        return str;
    }

    // Convert this representation to a decimal
    pub fn to_i32(&self) -> i32 {
        let len = self.len();
        let mut decimal = 0;
        for i in 0..len {
            let bit = if self.arr[i] { 1 } else { 0 };
            // I don't really know what this calculation does but the formula says
            // this is right and it works so I'm not complaining
            decimal += ((bit as f32) * (2 as f32).powi((len - 1 - i) as i32)) as i32;
        }
        return decimal;
    }

    // Calculate the amount of bits of num in binary
    fn calc_bits(num: &i32) -> i32 {
        let mut temp_var: i32;
        let mut idx = 0;
        let mut quotient = num / 2;

        while quotient > 0 {
            temp_var = quotient;

            quotient = temp_var / 2;

            idx += 1;
        }

        return idx + 1;
    }

    // Return the amount of bits of this representation
    pub fn len(&self) -> usize {
        return self.arr.len();
    }
}