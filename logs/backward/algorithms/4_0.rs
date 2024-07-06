/// n = 1, i = 0
fn select_3([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_2([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_3([a], is_dual)
    } else {
        select_3([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_1([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_2([a, b], is_dual)
    } else {
        select_2([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_0([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1([a, b, c], is_dual)
    } else {
        select_1([b, c, d], is_dual)
    }
}
