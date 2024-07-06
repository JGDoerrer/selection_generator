/// n = 1, i = 0
fn select_12([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_11([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_12([a], is_dual)
    } else {
        select_12([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_10([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_11([a, b], is_dual)
    } else {
        select_11([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_9([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_10([c, d, e], is_dual)
    } else {
        select_10([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_8([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_9([a, f, c, d, e, g], is_dual)
    } else {
        select_9([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_7([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_8([a, c, d, e, b, f, g], is_dual)
    } else {
        select_8([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_6([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_7([a, c, e, b, f, g, h], is_dual)
    } else {
        select_7([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_5([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_6([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_6([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_4([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_5([e, a, b, c, d, f, g, i, h], is_dual)
    } else {
        select_5([e, a, b, d, c, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_3([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_4([a, b, c, d, e, g, h, i, f], is_dual)
    } else {
        select_4([a, b, c, f, e, g, h, i, d], is_dual)
    }
}
/// n = 9, i = 2
fn select_2([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_3([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_3([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_2([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_2([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_0([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
