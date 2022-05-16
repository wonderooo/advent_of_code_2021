fn main() {
    let input: Vec<u32> = std::fs::read_to_string("input")
        .unwrap_or_else(|e| panic!("input error: {}", e))
        .split("\n")
        .map(|i| i.parse::<u32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();
    println!("{:?}", desc_win(input.clone(), 2));
    println!("{:?}", desc_win(input.clone(), 4));
}

fn desc_win(depths: Vec<u32>, size: usize) -> usize {
    let depths: Vec<_> = depths
        .windows(size)
        .filter(|win| win[0] < win[size - 1])
        .collect();
    return depths.len();
} 
