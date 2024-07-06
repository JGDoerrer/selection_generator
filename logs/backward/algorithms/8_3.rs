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
/// n = 3, i = 1
fn select_13([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_11([a, b], is_dual)
    } else {
        select_12([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_9([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_10([b, c, d], is_dual)
    } else {
        select_13([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_8([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_9([a, c, d, b, e, f], is_dual)
    } else {
        select_9([b, c, d, a, e, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_16([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_11([a, d], is_dual)
    } else {
        select_11([b, c], is_dual)
    }
}
/// n = 5, i = 2
fn select_15([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_16([a, b, c, d], is_dual)
    } else {
        select_13([a, d, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_14([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_15([a, c, b, d, e], is_dual)
    } else {
        select_15([b, c, a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_7([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_8([a, b, c, d, e, f], is_dual)
    } else {
        select_14([a, b, e, c, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_19([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([b, c, e, d], is_dual)
    } else {
        select_13([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_18([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_19([a, b, c, e, d, f], is_dual)
    } else {
        select_15([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_17([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_8([a, e, d, g, f, b], !is_dual)
    } else {
        select_18([a, c, b, e, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_6([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_7([a, e, g, d, b, f], !is_dual)
    } else {
        select_17([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_5([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_6([d, a, b, e, f, c, g], is_dual)
    } else {
        select_6([d, a, c, e, g, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_19([a, b, c, e, d, f], is_dual)
    } else {
        select_16([c, d, e, b], is_dual)
    }
}
/// n = 5, i = 1
fn select_25([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_10([b, c, d], is_dual)
    } else {
        select_11([a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_19([a, f, b, e, d, g], is_dual)
    } else {
        select_25([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_23([a, b, f, d, g, e], is_dual)
    } else {
        select_24([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_16([a, c, b, d], is_dual)
    } else {
        select_16([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_19([g, h, f, d, a, b], !is_dual)
    } else {
        select_28([a, f, c, e], is_dual)
    }
}
/// n = 4, i = 1
fn select_30([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_10([a, b, c], is_dual)
    } else {
        select_12([d], is_dual)
    }
}
/// n = 7, i = 2
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([a, e, b, d, g, f], is_dual)
    } else {
        select_30([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_27([a, c, b, e, f, g, d, h], is_dual)
    } else {
        select_29([b, c, a, e, d, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_22([a, c, d, b, f, e, g], is_dual)
    } else {
        select_26([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_21([a, c, b, d, f, e, h, g], is_dual)
    } else {
        select_6([a, h, e, f, b, g, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_4([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_5([c, f, g, b, e, a, h], is_dual)
    } else {
        select_20([b, c, a, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_3([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_4([a, b, c, e, d, f, g, h], is_dual)
    } else {
        select_4([a, b, d, e, c, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_2([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_3([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_3([a, b, c, d, f, h, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_2([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_2([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_0([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1([a, b, c, d, e, f, h, g], is_dual)
    }
}
