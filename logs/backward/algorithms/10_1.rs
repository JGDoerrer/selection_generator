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
/// n = 5, i = 1
fn select_15([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_10([b, c, d], is_dual)
    } else {
        select_11([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_14([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_15([a, b, f, d, e], is_dual)
    } else {
        select_15([a, c, e, d, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_14([a, c, d, b, e, f], is_dual)
    } else {
        select_14([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_6([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_7([a, b, d, c, e, f, g], is_dual)
    } else {
        select_13([a, b, d, g, f, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_9([a, b, c, d, e, f], is_dual)
    } else {
        select_15([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_16([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, b, d, e, c, f], is_dual)
    } else {
        select_17([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_5([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_6([a, b, c, d, e, g, h], is_dual)
    } else {
        select_16([a, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_4([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_5([d, e, f, a, b, i, g, h], is_dual)
    } else {
        select_5([d, e, f, a, c, h, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_3([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_4([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_4([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 4, i = 0
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_10([a, b, c], is_dual)
    } else {
        select_10([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_23([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_24([d, e, f, g], is_dual)
    } else {
        select_24([a, b, c, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_25([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_9([a, b, c, d, f, g], is_dual)
    } else {
        select_9([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_22([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_25([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_21([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_22([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_22([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_20([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_21([a, b, c, i, e, f, g, h, j], is_dual)
    } else {
        select_21([a, b, d, h, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_19([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_20([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_20([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_7([d, e, a, b, h, f, g], is_dual)
    } else {
        select_7([d, e, a, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([a, e, f, b, c, d, g, h], is_dual)
    } else {
        select_27([d, e, f, b, c, a, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_18([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_19([a, b, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_26([a, b, c, d, f, j, i, e], is_dual)
    }
}
/// n = 10, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_3([a, b, c, d, e, g, j, i, f], is_dual)
    } else {
        select_18([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_2([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_2([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
