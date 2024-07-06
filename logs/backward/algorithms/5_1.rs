/// n = 1, i = 0
fn select_6([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_5([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_6([a], is_dual)
    } else {
        select_6([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_4([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_5([a, d], is_dual)
    } else {
        select_5([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_3([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_4([a, c, b, d], is_dual)
    } else {
        select_4([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_2([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_3([a, b, c, d], is_dual)
    } else {
        select_3([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_1([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_2([a, b, c, e], is_dual)
    } else {
        select_3([b, c, d, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_0([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1([a, b, c, d, e], is_dual)
    } else {
        select_1([a, b, c, e, d], is_dual)
    }
}
