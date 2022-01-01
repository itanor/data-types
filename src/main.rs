fn main() {
    // integer types
    let i8_number: i8 = -128;
    println!("{}", i8_number);
    let u8_number: u8 = 127;
    println!("{}", u8_number);

    let i16_number: i16 = -4;
    println!("{}", i16_number);
    let u16_number: u16 = 4;
    println!("{}", u16_number);

    let i32_number: i32 = -4;
    println!("{}", i32_number);
    let u32_number: u32 = 4;
    println!("{}", u32_number);

    let i64_number: i64 = -4;
    println!("{}", i64_number);
    let u64_number: u64 = 4;
    println!("{}", u64_number);

    let i128_number: i128 = -4;
    println!("{}", i128_number);
    let u128_number: u128 = 4;
    println!("{}", u128_number);

    // float point types
    let x = 2.0;        // f64
    println!("{:.10}", x);
    let y: f32 = 3.0;   // f32
    println!("{:.10}", y);

    // numeric operations
    let sum = 5 + 10;
    println!("{}", sum);

    let difference = 95.5 - 4.3;
    println!("{:.10}", difference);

    let product = 4 * 30;
    println!("{}", product);

    let quotient = 56.7 /32.2;
    println!("{:.10}", quotient);

    let floored = 2 / 3;
    println!("{}", floored);

    let remainder = 42 % 5;
    println!("{}", remainder);

    // boolean type
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    // character type
    let c = 'z';
    let z = 'Z';
    println!("{} {}", c, z);

    // tuple type
    let tuple = (500, 6.4, 1);
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);
    let (x, y, z) = tuple;
    println!("{} {} {}", x, y, z);

    let new_tuple: (i32, f64, u8) = (400, 5.3, 0);
    println!("{} {} {}", new_tuple.0, new_tuple.1, new_tuple.2);

    let array = [1,2,3,4,5];
    for n in array.iter() {
        println!("{}", n);
    }

    let mapped: Vec<_> = array.iter()
        .map(|n| n * 2)
        .collect();
    for d in mapped.iter() {
        println!("{}", d);
    }

    const ARRAY_SIZE: usize = 5;
    // size of 5
    let a: [i32; ARRAY_SIZE] = [11,22,33,44,55];
    for i in a.iter() {
        println!("{}", i);
    }

    let initialized_array = [3; 5];
    for i in initialized_array.iter() {
        println!("{}", i);
    }
}
