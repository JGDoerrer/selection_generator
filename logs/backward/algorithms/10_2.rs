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
/// n = 4, i = 1
fn select_16([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_13([a, d], is_dual)
    } else {
        select_13([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_15([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_16([a, c, b, d], is_dual)
    } else {
        select_16([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_11([c, d, e, f], is_dual)
    } else {
        select_15([a, b, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_18([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_12([b, c, d], is_dual)
    } else {
        select_13([a, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_11([c, d, e, g], is_dual)
    } else {
        select_18([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_9([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_10([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_17([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_20([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_11([c, d, e, f], is_dual)
    } else {
        select_12([a, b, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_22([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_12([a, b, c], is_dual)
    } else {
        select_14([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_11([c, d, e, f], is_dual)
    } else {
        select_22([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_19([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_20([b, f, c, d, e, g, h], is_dual)
    } else {
        select_21([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_8([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_9([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_19([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_25([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_18([a, b, f, d, e], is_dual)
    } else {
        select_18([a, c, e, d, f], is_dual)
    }
}
/// n = 3, i = 1
fn select_28([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_13([a, b], is_dual)
    } else {
        select_14([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_27([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([b, c, e, d], is_dual)
    } else {
        select_28([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([e, a, c, f, d, g], is_dual)
    } else {
        select_27([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_25([a, c, d, b, e, f], is_dual)
    } else {
        select_26([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_31([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_16([a, b, c, d], is_dual)
    } else {
        select_28([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([a, b, c, e, d, f], is_dual)
    } else {
        select_31([d, c, a, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_32([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_18([b, a, c, d, e], is_dual)
    } else {
        select_16([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_30([a, c, d, b, e, f], is_dual)
    } else {
        select_32([a, b, d, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_24([a, c, d, b, f, e, g], is_dual)
    } else {
        select_29([a, f, b, c, g, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_7([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_8([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_23([a, c, d, h, g, b, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_6([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_7([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_7([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_5([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_6([a, b, c, e, f, g, d, h, i], is_dual)
    } else {
        select_6([a, b, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_12([c, d, e], is_dual)
    } else {
        select_12([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_38([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_39([a, f, c, d, e, g], is_dual)
    } else {
        select_39([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_37([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_38([a, c, d, e, b, f, g], is_dual)
    } else {
        select_38([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_41([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_39([a, b, c, d, e, f], is_dual)
    } else {
        select_18([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_41([a, b, d, e, c, f], is_dual)
    } else {
        select_41([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_36([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_37([a, d, e, b, c, f, g], is_dual)
    } else {
        select_40([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_36([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_36([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_34([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_35([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_35([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_46([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_15([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_45([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_46([a, b, c, e], is_dual)
    } else {
        select_15([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_45([a, b, c, e, g], is_dual)
    } else {
        select_41([a, d, b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_36([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_44([a, c, d, f, h, g, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_50([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_12([b, c, e], is_dual)
    } else {
        select_16([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_50([a, b, f, d, g, e, h], is_dual)
    } else {
        select_50([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_51([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([a, e, b, d, g, f], is_dual)
    } else {
        select_22([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_49([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_51([a, b, f, e, h, g, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_48([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_48([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_43([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_47([a, b, d, e, f, g, c, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_34([a, b, c, e, d, f, g, h], is_dual)
    } else {
        select_42([a, b, c, e, f, g, h, d], is_dual)
    }
}
/// n = 9, i = 2
fn select_4([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_5([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_33([a, b, c, d, f, h, i, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_37([a, c, e, b, f, g, h], is_dual)
    } else {
        select_37([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_56([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_56([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_55([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_55([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_53([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_54([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_54([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_57([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_7([a, b, c, e, f, g, d, h, i], is_dual)
    } else {
        select_7([a, b, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_52([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_53([a, b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_57([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_4([a, b, c, d, e, g, i, f, j], is_dual)
    } else {
        select_52([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_3([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_3([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_2([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_2([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
