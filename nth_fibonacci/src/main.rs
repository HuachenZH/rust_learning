use std::io;


fn fib_recur(nth_number: u64) -> u64 {
    if nth_number == 0 {return 0};
    if nth_number == 1 {return 1};
    fib_recur(nth_number-1) + fib_recur(nth_number-2)
}


fn fib_iter(nth_number: u64) -> u64 {
    if nth_number == 0 {return 0};
    if nth_number == 1 {return 1};

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = 1;

    for _ in 0..nth_number-1 {
        c = a + b;
        a = b;
        b = c;
    }
    c
}


fn main() {
    println!("nth fibonacci number");
    loop {
        let mut _input_n = String::new();
        println!("Please provide the n:");
        io::stdin()
            .read_line(&mut _input_n)
            .expect("Faild to read line.");

        if _input_n.trim() == "quit" {break};

        // string to u64
        let _input_n: u64 = match _input_n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number.");
                continue;
            },
        };

        let mut _output_fib = fib_recur(_input_n);
        println!("recur: {_input_n}th fibonacci number is {_output_fib}");
        let mut _output_fib = fib_iter(_input_n);
        println!("iter: {_input_n}th fibonacci number is {_output_fib}");
    }
}
