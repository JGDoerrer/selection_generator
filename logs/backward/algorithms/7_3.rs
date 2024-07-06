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
/// n = 3, i = 0
fn select_8([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_9([a, b], is_dual)
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
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_8([b, c, d], is_dual)
    } else {
        select_11([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_6([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_7([a, c, d, b, e, f], is_dual)
    } else {
        select_7([b, c, d, a, e, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_14([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_9([a, d], is_dual)
    } else {
        select_9([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_13([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_14([a, c, b, d], is_dual)
    } else {
        select_14([b, c, a, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_12([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_13([a, b, c, d], is_dual)
    } else {
        select_11([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_5([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_6([a, b, c, d, e, f], is_dual)
    } else {
        select_12([a, b, e, c, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_7([a, b, c, d, f, g], is_dual)
    } else {
        select_13([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_15([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_16([a, c, d, b, e, f, g], is_dual)
    } else {
        select_16([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_4([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_5([a, b, d, c, e, f], is_dual)
    } else {
        select_15([a, b, e, g, c, f, d], !is_dual)
    }
}
/// n = 5, i = 2
fn select_19([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_13([a, b, c, e], is_dual)
    } else {
        select_13([a, c, b, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_18([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_19([a, b, d, c, e], is_dual)
    } else {
        select_19([a, c, d, b, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_18([a, b, f, c, e], is_dual)
    } else {
        select_18([a, b, e, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_3([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_4([a, b, e, g, c, f, d], !is_dual)
    } else {
        select_17([a, b, c, g, e, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_2([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_3([a, b, c, d, f, e, g], is_dual)
    } else {
        select_3([a, b, c, e, g, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_1([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_2([a, b, c, d, f, e, g], is_dual)
    } else {
        select_2([a, b, c, e, f, d, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_0([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1([a, b, c, d, e, f, g], is_dual)
    } else {
        select_1([a, b, c, d, e, g, f], is_dual)
    }
}
