fn main() {
    println!("enter number of terms");

    let mut input: String = Default::default();

    std::io::stdin()
        .read_line(&mut input)
        .expect("invalid input");

    let n: u32 = input.trim().parse().expect("invalid number");

    let total = fib(n);

    println!("{n}th term is {total}");
}

fn fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c: u32;

    for _ in 1..n {
        c = b;
        b = a + b;
        a = c;
    }

    b
}
