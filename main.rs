fn main() {
    // constanty
    const age: u32 = 1_000_000;
    println!("Info: {}", age);

    // Кортеджи
    let user_alex: (i32, bool, f64, char)  = (42, true, 1.77, 'R');
    println!("Info: {}", user_alex.0);

    // Массивы
    let mut nums: [i8; 6] = [1,4,6,7,0,9];
    nums[0] = 10;
    println!("Info {}", nums[0]);
}
