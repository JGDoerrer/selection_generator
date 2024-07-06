/// n = 1, i = 0
fn select_15([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_14([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_15([a], is_dual)
    } else {
        select_15([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_13([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_14([a, b], is_dual)
    } else {
        select_14([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_12([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([c, d, e], is_dual)
    } else {
        select_13([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_11([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_12([a, f, c, d, e, g], is_dual)
    } else {
        select_12([b, e, c, d, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([c, d, e, f, g, h, i], is_dual)
    } else {
        select_11([a, b, g, h, e, f, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_18([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_13([b, c, d], is_dual)
    } else {
        select_14([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_12([a, b, c, d, e, f], is_dual)
    } else {
        select_18([e, c, d, a, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_16([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_17([a, b, f, g, e, i], is_dual)
    } else {
        select_11([c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_10([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_16([a, f, c, d, i, g, h, e, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_17([b, d, a, e, f, g], is_dual)
    } else {
        select_17([a, c, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_19([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_20([a, b, f, d, h, g, e, i], is_dual)
    } else {
        select_16([b, d, a, c, g, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_9([a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_19([a, b, d, c, f, h, g, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_8([c, d, e, a, b, h, j, f, g, i, k], is_dual)
    } else {
        select_8([c, e, d, a, b, h, i, f, g, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_11([a, c, d, e, b, f, g], is_dual)
    } else {
        select_11([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_23([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_24([a, c, e, b, f, g, h], is_dual)
    } else {
        select_24([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_22([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_23([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_23([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_30([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_14([a, d], is_dual)
    } else {
        select_14([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([b, c, e], is_dual)
    } else {
        select_30([a, f, d, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_31([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_30([a, c, b, d], is_dual)
    } else {
        select_30([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_28([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([b, c, d, e, g, f, h], is_dual)
    } else {
        select_31([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_28([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_27([a, b, c, h, e, f, g, i], is_dual)
    } else {
        select_27([a, b, d, g, e, f, h, i], is_dual)
    }
}
/// n = 3, i = 1
fn select_36([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_14([a, b], is_dual)
    } else {
        select_15([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_30([b, c, e, d], is_dual)
    } else {
        select_36([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_34([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_35([a, b, c, e, d, f], is_dual)
    } else {
        select_30([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_37([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_35([a, f, b, e, d, g], is_dual)
    } else {
        select_18([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_34([a, b, f, d, g, e], is_dual)
    } else {
        select_37([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_33([a, b, g, e, f, c, h], is_dual)
    } else {
        select_27([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_25([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_26([b, c, a, d, f, g, e, h, i], is_dual)
    } else {
        select_32([b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_21([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_22([i, c, d, b, a, g, h, f, j], is_dual)
    } else {
        select_25([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_7([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_21([f, b, c, d, e, k, h, i, j, g], is_dual)
    }
}
/// n = 4, i = 0
fn select_44([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_13([a, b, c], is_dual)
    } else {
        select_13([b, c, d], is_dual)
    }
}
/// n = 6, i = 1
fn select_43([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_44([b, c, d, e], is_dual)
    } else {
        select_14([a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_45([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_35([a, d, c, e, f, g], is_dual)
    } else {
        select_31([a, b, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_42([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_43([c, d, e, g, f, h], is_dual)
    } else {
        select_45([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_44([c, d, e, f], is_dual)
    } else {
        select_31([a, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_47([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_47([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_42([a, b, g, e, f, c, h, i, j], is_dual)
    } else {
        select_46([c, d, e, f, a, b, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_40([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_41([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_41([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_39([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_40([c, d, a, e, b, f, h, i, j, g, k], is_dual)
    } else {
        select_40([c, d, b, e, a, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_38([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_39([c, d, a, b, e, f, g, h, j, k, i], is_dual)
    } else {
        select_39([c, d, a, b, f, e, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_6([a, b, c, d, e, g, h, i, j, k, f], is_dual)
    } else {
        select_38([a, f, b, c, d, e, h, i, j, k, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_5([a, b, c, d, e, f, h, i, j, k, g], is_dual)
    } else {
        select_5([a, b, c, d, g, f, h, i, j, k, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_4([a, b, c, d, e, f, g, i, j, k, h], is_dual)
    } else {
        select_4([a, b, c, h, e, f, g, i, j, k, d], is_dual)
    }
}
/// n = 11, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_3([a, h, i, b, c, d, e, f, g, j, k], is_dual)
    } else {
        select_3([g, h, i, b, c, d, e, f, a, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_2([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
