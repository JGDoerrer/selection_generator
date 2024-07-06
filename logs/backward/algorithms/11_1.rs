/// n = 1, i = 0
fn select_13([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_12([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_13([a], is_dual)
    } else {
        select_13([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_11([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_12([a, b], is_dual)
    } else {
        select_12([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_10([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_11([a, b, c], is_dual)
    } else {
        select_11([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_9([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_10([d, e, f, g], is_dual)
    } else {
        select_10([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_9([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_9([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_16([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_11([c, d, e], is_dual)
    } else {
        select_11([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_15([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_16([a, b, c, d, f, g], is_dual)
    } else {
        select_16([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_14([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_9([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_15([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_8([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_14([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_7([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_7([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_15([a, g, c, d, e, f, h], is_dual)
    } else {
        select_15([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_19([b, h, d, e, f, g, a, i], is_dual)
    } else {
        select_14([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_18([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_18([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_6([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_17([a, b, c, i, f, g, h, d, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_16([a, f, c, d, e, g], is_dual)
    } else {
        select_16([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, e, b, f, g], is_dual)
    } else {
        select_23([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_21([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_22([a, b, c, e, f, d, g], is_dual)
    } else {
        select_22([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_21([a, b, c, d, e, h, g], is_dual)
    } else {
        select_21([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_5([a, b, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_20([a, b, c, d, f, j, i, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_30([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_11([b, c, d], is_dual)
    } else {
        select_12([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([a, b, f, d, e], is_dual)
    } else {
        select_30([a, c, e, d, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, b, e, f], is_dual)
    } else {
        select_29([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_22([a, b, d, c, e, f, g], is_dual)
    } else {
        select_28([a, b, d, g, f, c], is_dual)
    }
}
/// n = 8, i = 1
fn select_26([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_21([a, b, c, g, e, f, h], is_dual)
    } else {
        select_27([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_25([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_26([d, e, f, a, b, i, g, h], is_dual)
    } else {
        select_26([d, e, f, a, c, h, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_24([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_25([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_25([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_4([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_24([a, b, c, d, e, g, j, i, f], is_dual)
    }
}
/// n = 10, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_3([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_3([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_36([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_8([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_8([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_35([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_36([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_36([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_34([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_35([a, b, c, e, f, g, h, i, d, j, k], is_dual)
    } else {
        select_35([a, b, d, e, f, g, h, i, c, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_33([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_5([a, b, c, d, j, g, h, i, e, k], is_dual)
    } else {
        select_34([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_32([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_33([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_24([a, b, c, d, e, g, k, j, f], is_dual)
    }
}
/// n = 11, i = 1
fn select_31([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_32([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_32([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_2([a, b, c, d, e, f, g, h, k, i], is_dual)
    } else {
        select_31([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
