/// n = 1, i = 0
fn select_11([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_10([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_11([a], is_dual)
    } else {
        select_11([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_9([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_10([a, b], is_dual)
    } else {
        select_10([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_8([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_9([c, d, e], is_dual)
    } else {
        select_9([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_7([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_8([a, f, c, d, e, g], is_dual)
    } else {
        select_8([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_6([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_7([a, c, d, e, b, f, g], is_dual)
    } else {
        select_7([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_14([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_9([b, c, d], is_dual)
    } else {
        select_10([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_14([a, b, f, d, e], is_dual)
    } else {
        select_14([a, c, e, d, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_12([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_13([a, c, d, b, e, f], is_dual)
    } else {
        select_13([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_5([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_6([a, b, d, c, e, f, g], is_dual)
    } else {
        select_12([a, b, d, g, f, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_16([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_8([a, b, c, d, e, f], is_dual)
    } else {
        select_14([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_15([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_16([a, b, d, e, c, f], is_dual)
    } else {
        select_16([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_4([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_5([a, b, c, d, e, g, h], is_dual)
    } else {
        select_15([a, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_3([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_4([d, e, f, a, b, i, g, h], is_dual)
    } else {
        select_4([d, e, f, a, c, h, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_2([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_3([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_3([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_1([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_2([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_2([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_0([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
