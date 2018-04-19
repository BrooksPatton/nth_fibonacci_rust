fn main() {
    let n = 93;
    let fib = calculate_fib(n);

    println!("The {} number in the fib sequence is {}", n, fib);
}

fn calculate_fib(mut n: usize) -> usize {
    // 0, 1, 1, 2, 3, 5, ...

    n -= 1;
    let mut first = 0;
    let mut second = 1;
    let mut current: usize;

    while n > 0 {
        current = first + second;
        first = second;
        second = current;
        n -= 1;
    }

    second
}

// fn calculate_fib(n: u32) -> u32 {
//     // if n == 1 || n == 0 {
//     //     return n;
//     // } else {
//     //     return calculate_fib(n - 1) + calculate_fib(n - 2);
//     // }

//     match n {
//         0 => 0,
//         1 => 1,
//         num => calculate_fib(num - 1) + calculate_fib(num - 2)
//     }
// }