use anyhow::Result;

fn main(){
    let lines = read_bits("input").unwrap();
    let line_lenght: i32 = 12;
    let mut sums = vec![0, 0, 0, 0, 0, 0, 0, 0 ,0 ,0 ,0 , 0];

    lines
        .iter()
        .for_each(|line| {
            for i in 0..line_lenght as usize{
                sums[i] += line[i]
            }
        });
    let ans1: u32 = sums
        .into_iter()
        .map(|bit| if bit > 500 {1} else {0})
        .fold(0, |acc, next| (acc<<1) + next);
    println!("{:?}", ans1);
}

fn read_bits(filename: &str) -> Result<Vec<Vec<u32>>> {
    Ok(
    std::fs::read_to_string(filename)
        .unwrap_or_else(|err| panic!("ERROR READING FILE!: {:?}", err))
        .lines()
        .map(|line| line.chars().map(|sigle_c| char::to_digit(sigle_c, 2).unwrap()).collect())
        .collect()
    )
}
