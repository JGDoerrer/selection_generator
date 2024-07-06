/// n = 1, i = 0
fn select_4([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_3([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_4([a], is_dual)
    } else {
        select_4([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_2([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_3([a, d], is_dual)
    } else {
        select_3([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_1([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_2([a, c, b, d], is_dual)
    } else {
        select_2([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_0([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1([a, b, c, d], is_dual)
    } else {
        select_1([a, b, d, c], is_dual)
    }
}
