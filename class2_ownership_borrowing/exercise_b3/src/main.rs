//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
fn main() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;
    
    //for n in &mut values {
    // for n in 0..v.len() {
    //     max = std::cmp::max(max, v[n]);
    // }
    for n in v.iter_mut() {
        max = std::cmp::max(max, *n);
    }

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    //for n in &mut values {
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}
