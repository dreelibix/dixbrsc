/*
The main function server as the entry point of this rust program.
It prints a greeting message to the console.
*/
fn main() {
    // Declare an immutable variable to store the studio name.
    let studio_name = "DreeLibIX's Den";
    // Use println! macro to print a message to console.
    // ! show that println is a macro.
    println!("{}へようこそ！", studio_name);
    // Get argument and assign it to immutable variable.
    let args: Vec<String> = std::env::args().skip(1).collect();
    print!("あなたの入力：");
    // Use std::env::args() to get command line arguments.
    // 下にargsを使う必要があるから、ここは&を付けって、引用を借りる必要がある。
    for arg in &args {
        print!("{} ", arg);
    }
    println!();
    let nums = parse_args_to_i32(&args[1..]);
    if !nums.is_empty() {
        // 入力に問題がなければ計算を始まる。
        println!("結果: {}", calc(&args[0], nums));
    } else {
        println!("No item to calc.");
    }
}
/*
Parse the numbers in command line arguments to i32.
*/
fn parse_args_to_i32(args: &[String]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for arg in args {
        match arg.parse::<i32>() {
            Ok(num) => res.push(num),
            // Return empty vector when parses error.
            Err(_) => return vec![],
        }
    }
    res
}

/*
悪い書き方です。
*/
// fn calc(operator: &String, nums: Vec<i32>) -> i32 {
//     let mut temp = nums[0];
//     for num in nums.iter().skip(1) {
//         match operator.as_str() {
//             "add" => {
//                 temp = temp + num;
//             },
//             _ => return 0,
//         }
//     }
//     temp
// }
fn calc(operator: &str, nums: Vec<i32>) -> i32{
    nums.iter().skip(1).fold(nums[0], |acc, &num| {
        match operator {
            "add" => acc + num,
            "sub" => acc - num,
            "mul" => acc * num,
            "div" => acc / num,
            _ => return 0,
        }
    })
}