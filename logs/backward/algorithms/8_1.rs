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
/// n = 6, i = 1
fn select_6([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_7([c, d, e], is_dual)
    } else {
        select_7([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_5([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_6([a, f, c, d, e, g], is_dual)
    } else {
        select_6([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_4([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_5([a, c, d, e, b, f, g], is_dual)
    } else {
        select_5([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_3([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_4([a, b, c, e, f, d, g], is_dual)
    } else {
        select_4([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_2([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_3([a, b, c, d, e, h, g], is_dual)
    } else {
        select_3([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_1([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_2([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_2([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_0([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1([a, b, c, d, e, f, h, g], is_dual)
    }
}
