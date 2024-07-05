// Compound types
// arrays, tuples, slices, and strings (slice strings)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers array: {:?}", numbers);

    // String array
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits 1st element: {}", fruits[0]);
    println!("Fruits 2nd element: {}", fruits[1]);
    println!("Fruits 3rd element: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    // Mix tuple
    let mix_tuple = ("Alice".to_string(), 30, false, [1,2,3,4]);
    println!("Mix tuple: {:?}", mix_tuple);
}
