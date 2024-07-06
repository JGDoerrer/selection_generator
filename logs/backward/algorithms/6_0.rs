/// n = 1, i = 0
fn select_5([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_4([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_5([a], is_dual)
    } else {
        select_5([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_3([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_4([a, b], is_dual)
    } else {
        select_4([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_2([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_3([a, b, c], is_dual)
    } else {
        select_3([b, c, d], is_dual)
    }
}
/// n = 5, i = 0
fn select_1([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_2([a, b, c, d], is_dual)
    } else {
        select_2([a, b, c, e], is_dual)
    }
}
/// n = 6, i = 0
fn select_0([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_1([a, b, c, d, e], is_dual)
    } else {
        select_1([a, b, d, e, f], is_dual)
    }
}
