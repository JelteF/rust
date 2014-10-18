pub fn add_three_times_four(x: int) -> int {
    times_four(add_three(x))
}

fn add_three(x: int) -> int {
    x + 3
}

fn times_four(x: int) -> int {
    x * 4
}
