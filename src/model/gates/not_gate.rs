// Declare NotGate struct
pub struct NotGate {}

// Implement the previously declared NotGate struct
impl NotGate {
    pub fn calc(&self, a: bool) -> bool {
        return !a;
    }
}
