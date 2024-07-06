/// n = 1, i = 0
fn select_9([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_8([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_9([a], is_dual)
    } else {
        select_9([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_7([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_8([a, b], is_dual)
    } else {
        select_8([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_6([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_7([a, b, c], is_dual)
    } else {
        select_7([b, c, d], is_dual)
    }
}
/// n = 5, i = 0
fn select_5([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_6([a, b, c, d], is_dual)
    } else {
        select_6([a, b, c, e], is_dual)
    }
}
/// n = 6, i = 0
fn select_4([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_5([a, b, c, d, e], is_dual)
    } else {
        select_5([a, b, d, e, f], is_dual)
    }
}
/// n = 7, i = 0
fn select_3([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_4([a, b, c, d, e, f], is_dual)
    } else {
        select_4([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 0
fn select_2([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_3([a, b, c, d, e, f, g], is_dual)
    } else {
        select_3([a, b, c, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 0
fn select_1([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_2([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_2([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 0
fn select_0([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_1([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
