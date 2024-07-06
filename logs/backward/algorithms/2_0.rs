/// n = 1, i = 0
fn select_1([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_0([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1([a], is_dual)
    } else {
        select_1([b], is_dual)
    }
}
