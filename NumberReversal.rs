fn main() {
    let mut input = String::new();
    let b1 = std::io::stdin().read_line(&mut input).unwrap();
    let mut numb: i64 = 0;
    numb = input.trim().parse().unwrap();
    let mut ReverseN: i64 = 0;

    while numb > 0 {
        ReverseN = ReverseN * 10 + numb % 10;
        numb = numb / 10;
    }

    println!("{}", ReverseN);
}