/// n = 1, i = 0
fn select_10([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_9([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_10([a], is_dual)
    } else {
        select_10([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_8([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_9([a, d], is_dual)
    } else {
        select_9([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_11([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_9([a, b], is_dual)
    } else {
        select_10([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_7([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_8([b, c, e, d], is_dual)
    } else {
        select_11([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_12([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_8([a, b, c, d], is_dual)
    } else {
        select_11([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_6([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_7([a, b, c, e, d, f], is_dual)
    } else {
        select_12([d, c, a, e, f], is_dual)
    }
}
/// n = 3, i = 0
fn select_15([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_9([a, b], is_dual)
    } else {
        select_9([b, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_14([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_15([b, c, d], is_dual)
    } else {
        select_9([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_13([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_14([b, a, c, d, e], is_dual)
    } else {
        select_8([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_5([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_6([a, c, d, b, e, f], is_dual)
    } else {
        select_13([a, b, d, c, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_18([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([c, d, e], is_dual)
    } else {
        select_15([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_18([a, b, c, d, e, f], is_dual)
    } else {
        select_14([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_6([a, f, c, b, e, g], is_dual)
    } else {
        select_17([a, c, b, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_4([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_5([a, b, c, f, g, e], is_dual)
    } else {
        select_16([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_3([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_4([a, b, c, e, d, f, g], is_dual)
    } else {
        select_4([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_2([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_3([a, b, c, d, e, f, g], is_dual)
    } else {
        select_3([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_1([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_2([a, b, c, d, f, e, g], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_0([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1([a, b, c, d, e, f, g], is_dual)
    } else {
        select_1([a, b, c, d, e, g, f], is_dual)
    }
}
