/// n = 1, i = 0
fn select_14([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_13([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_14([a], is_dual)
    } else {
        select_14([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_12([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_13([a, b], is_dual)
    } else {
        select_13([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_11([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_12([a, b, c], is_dual)
    } else {
        select_12([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_11([d, e, f, g], is_dual)
    } else {
        select_11([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_9([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_10([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_10([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_12([c, d, e], is_dual)
    } else {
        select_12([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_17([a, b, c, d, f, g], is_dual)
    } else {
        select_17([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_15([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_10([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_16([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_9([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_15([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_8([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_8([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 9, i = 1
fn select_19([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_15([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_15([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_17([a, f, c, d, e, g], is_dual)
    } else {
        select_17([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_21([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_22([a, c, d, e, b, f, g], is_dual)
    } else {
        select_22([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_25([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_12([b, c, d], is_dual)
    } else {
        select_13([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_17([a, b, c, d, e, f], is_dual)
    } else {
        select_25([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_25([a, b, c, e, f], is_dual)
    } else {
        select_25([a, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_24([a, b, c, d, f, g], is_dual)
    } else {
        select_26([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_21([a, b, c, d, e, g, h], is_dual)
    } else {
        select_23([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_19([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_20([a, b, h, e, f, g, c, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_7([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_18([a, b, c, i, f, g, h, d, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_29([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_9([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_9([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_28([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_29([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_27([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_28([a, b, c, e, f, g, h, i, d, j, k], is_dual)
    } else {
        select_28([a, b, d, e, f, g, h, i, c, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_6([a, b, c, d, j, g, h, i, e, k], is_dual)
    } else {
        select_27([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_21([a, g, b, d, e, f, h], is_dual)
    } else {
        select_24([a, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_33([e, a, b, c, i, f, g, h], is_dual)
    } else {
        select_33([e, a, b, d, h, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_31([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_32([a, b, c, d, e, g, h, i, f], is_dual)
    } else {
        select_32([a, b, c, f, e, g, h, i, d], is_dual)
    }
}
/// n = 9, i = 1
fn select_30([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_31([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_31([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 11, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_5([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_30([a, b, c, d, e, g, k, j, f], is_dual)
    }
}
/// n = 12, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_4([d, e, f, g, h, i, a, b, l, j, k], is_dual)
    } else {
        select_4([d, e, f, g, h, i, a, c, k, j, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_3([a, i, j, b, c, d, e, f, g, h, k, l], is_dual)
    } else {
        select_3([h, i, j, b, c, d, e, f, g, a, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_2([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
