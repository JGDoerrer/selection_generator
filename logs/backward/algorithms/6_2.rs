/// n = 1, i = 0
fn select_8([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_7([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_8([a], is_dual)
    } else {
        select_8([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_6([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_7([a, d], is_dual)
    } else {
        select_7([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_5([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_6([a, c, b, d], is_dual)
    } else {
        select_6([b, c, a, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_4([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_5([a, b, c, e], is_dual)
    } else {
        select_5([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_3([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_4([f, a, b, d, e], is_dual)
    } else {
        select_4([d, b, c, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_2([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_3([a, c, d, b, e, f], is_dual)
    } else {
        select_3([b, c, d, a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_1([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_2([a, b, c, e, d, f], is_dual)
    } else {
        select_2([a, b, d, e, c, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_0([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1([a, b, c, d, e, f], is_dual)
    } else {
        select_1([a, b, c, d, f, e], is_dual)
    }
}
