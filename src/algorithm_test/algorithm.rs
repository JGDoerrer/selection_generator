#[allow(unused)]
pub const N: usize = 2;
#[allow(unused)]
pub const I: usize = 0;

#[allow(unused)]
pub fn run(input: [usize; 2]) -> usize {
    select_0(input)
}

fn select_0([a, b]: [usize; 2]) -> usize {
    a.min(b)
}
