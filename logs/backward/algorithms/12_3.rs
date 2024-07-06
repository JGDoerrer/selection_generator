/// n = 1, i = 0
fn select_18([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_17([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_18([a], is_dual)
    } else {
        select_18([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_16([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_17([a, b], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_15([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_16([a, b, c], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_14([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_18([e], is_dual)
    }
}
/// n = 5, i = 1
fn select_20([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_17([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_19([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_20([a, c, d, b, e], is_dual)
    } else {
        select_20([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_14([a, c, d, e, f], is_dual)
    } else {
        select_19([a, b, c, e, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_17([a, d], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, b, e, f], is_dual)
    } else {
        select_24([a, c, d, f], is_dual)
    }
}
/// n = 3, i = 1
fn select_26([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_17([a, b], is_dual)
    } else {
        select_18([c], is_dual)
    }
}
/// n = 5, i = 2
fn select_25([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_26([d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_23([d, f, g, a, e, b], !is_dual)
    } else {
        select_25([e, f, d, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_21([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_22([a, b, c, d, f, g, e], is_dual)
    } else {
        select_22([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_12([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_13([b, c, d, e, f, g], is_dual)
    } else {
        select_21([f, h, i, a, b, c, g], !is_dual)
    }
}
/// n = 6, i = 1
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([c, d, e], is_dual)
    } else {
        select_16([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([a, f, c, d, e, g], is_dual)
    } else {
        select_30([b, e, c, d, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([b, c, e, d], is_dual)
    } else {
        select_26([a, b, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_29([b, c, d, e, g, f, h], is_dual)
    } else {
        select_31([a, b, c, f, g, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_34([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, b, d], is_dual)
    } else {
        select_24([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_34([a, f, b, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_36([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([a, b, c], is_dual)
    } else {
        select_18([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_15([c, d, e, f], is_dual)
    } else {
        select_36([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_33([a, c, b, h, g, f, i], is_dual)
    } else {
        select_35([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_27([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_28([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_32([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_12([a, b, c, d, f, g, i, h, j], is_dual)
    } else {
        select_27([c, a, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_41([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_26([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_41([d, c, a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_40([a, b, c, d, e, f], is_dual)
    } else {
        select_26([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([b, c, e], is_dual)
    } else {
        select_24([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_44([a, b, c, e, f, g, h], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_45([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_15([c, d, e, f], is_dual)
    } else {
        select_34([a, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_43([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_45([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_39([a, b, g, f, c, h], is_dual)
    } else {
        select_42([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_12([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_38([c, a, d, b, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_11([a, b, c, e, f, g, h, j, i, k], is_dual)
    } else {
        select_37([a, c, b, d, i, h, k, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_10([c, a, d, b, e, f, h, i, j, g, k], is_dual)
    } else {
        select_10([c, b, d, a, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_9([a, b, c, d, f, i, g, h, k, j, l], is_dual)
    } else {
        select_9([a, b, d, c, e, j, g, h, l, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_8([c, d, b, e, a, f, h, i, g, k, l, j], is_dual)
    } else {
        select_8([c, d, b, f, a, e, h, i, g, j, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_7([a, b, c, d, e, f, g, i, j, k, l, h], is_dual)
    } else {
        select_7([b, a, c, d, e, f, h, i, j, k, l, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_6([a, b, c, d, e, f, h, i, j, k, l, g], is_dual)
    } else {
        select_6([a, b, c, d, e, g, h, i, j, k, l, f], is_dual)
    }
}
/// n = 12, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_5([a, b, c, d, e, f, g, i, j, k, l, h], is_dual)
    } else {
        select_5([a, b, c, d, h, f, g, i, j, k, l, e], is_dual)
    }
}
/// n = 12, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_4([a, b, c, d, e, f, g, h, j, k, l, i], is_dual)
    } else {
        select_4([a, b, c, i, e, f, g, h, j, k, l, d], is_dual)
    }
}
/// n = 12, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_3([a, i, j, b, c, d, e, f, g, h, k, l], is_dual)
    } else {
        select_3([h, i, j, b, c, d, e, f, g, a, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_2([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
