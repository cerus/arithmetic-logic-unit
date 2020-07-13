use crate::model::adder::ripple_carry_adder;
use crate::model::binary_representation::BinaryRepresentation;
use crate::model::adder::ripple_carry_adder::RippleCarryAdder;
use crate::model::subtractor::ripple_carry_subtractor::RippleCarrySubtractor;

mod model;

fn main() {
    println!("--- Testing binary representation ---");
    test(15, "1111");
    test(33, "100001");
    test(54, "110110");
    test(12, "1100");
    test(122, "1111010");

    println!("--- Testing ripple carry adder ---");
    test_adder_calc(1, 0, 1);
    test_adder_calc(1, 1, 2);
    test_adder_calc(34, 12, 46);
    test_adder_calc(23, 74, 97);
    test_adder_calc(11, 75, 86);

    println!("--- Testing ripple carry subtractor ---");
    test_sub_calc(2, 1, 1);
    test_sub_calc(10, 5, 5);
    test_sub_calc(345, 34, 311);
    test_sub_calc(34, 2, 32);
}

fn test(num: i32, correct_str: &str) {
    let bin_rep = BinaryRepresentation::from_num(num);
    println!(
        "Testing {}: Result = {}; Correct = {} [{}]",
        num,
        bin_rep.to_string(),
        correct_str,
        if bin_rep.to_string() == String::from(correct_str) {
            "Correct"
        } else {
            "Incorrect"
        }
    );
}

fn test_adder_calc(num1: i32, num2: i32, correct_num: i32) {
    let a = BinaryRepresentation::from_num(num1);
    let b = BinaryRepresentation::from_num(num2);
    let adder = RippleCarryAdder::new();
    let res = adder.calc(&a, &b);
    println!(
        "Testing calculation of {} ({}) + {} ({}): {} ({}) [{}]",
        num1,
        &a.to_string(),
        num2,
        &b.to_string(),
        res.to_i32(),
        res.to_string(),
        if res.to_i32() == correct_num {
            String::from("Correct")
        } else {
            String::from("Incorrect - ") + correct_num.to_string().as_str()
        }
    );
}

fn test_sub_calc(num1: i32, num2: i32, correct_num: i32) {
    let a = BinaryRepresentation::from_num(num1);
    let b = BinaryRepresentation::from_num(num2);
    let sub = RippleCarrySubtractor::new();
    let res = sub.calc(&a, &b);
    println!(
        "Testing calculation of {} ({}) - {} ({}): {} ({}) [{}]",
        num1,
        &a.to_string(),
        num2,
        &b.to_string(),
        res.to_i32(),
        res.to_string(),
        if res.to_i32() == correct_num {
            String::from("Correct")
        } else {
            String::from("Incorrect - ") + correct_num.to_string().as_str()
        }
    );
}