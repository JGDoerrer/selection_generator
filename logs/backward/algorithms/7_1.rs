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
/// n = 3, i = 0
fn select_6([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_7([a, b], is_dual)
    } else {
        select_7([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_5([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_6([c, d, e], is_dual)
    } else {
        select_6([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_4([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_5([a, f, c, d, e, g], is_dual)
    } else {
        select_5([b, e, c, d, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_10([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_6([b, c, d], is_dual)
    } else {
        select_7([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_9([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_10([a, b, f, d, e], is_dual)
    } else {
        select_10([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_3([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_4([a, b, c, d, e, f, g], is_dual)
    } else {
        select_9([a, b, g, e, f, d], is_dual)
    }
}
/// n = 7, i = 1
fn select_2([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_3([a, d, e, b, c, f, g], is_dual)
    } else {
        select_3([c, d, e, b, a, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_1([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_2([a, b, c, d, f, e, g], is_dual)
    } else {
        select_2([a, b, c, e, f, d, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_0([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1([a, b, c, d, e, f, g], is_dual)
    } else {
        select_1([a, b, c, d, e, g, f], is_dual)
    }
}
