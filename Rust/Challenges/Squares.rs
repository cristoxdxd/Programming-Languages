pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum.pow(2)

    // (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sum

    // (1..=n).map(|x| x.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn main() {
    println!("{}", difference(10));
}