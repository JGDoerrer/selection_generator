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
/// n = 4, i = 0
fn select_12([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_13([a, b, c], is_dual)
    } else {
        select_13([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_11([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_12([d, e, f, g], is_dual)
    } else {
        select_12([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_10([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_11([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 11, i = 1
fn select_9([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_10([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_10([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_9([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_9([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_8([g, h, d, e, k, a, b, c, i, j, l], is_dual)
    } else {
        select_8([g, h, d, f, j, a, b, c, i, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_7([g, h, i, a, e, f, b, c, d, j, k, l], is_dual)
    } else {
        select_7([g, h, i, d, e, f, b, c, a, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_6([a, b, c, d, e, g, h, i, j, f, k, l], is_dual)
    } else {
        select_6([a, b, c, d, f, g, h, i, j, e, k, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([c, d, e], is_dual)
    } else {
        select_13([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, f, c, d, e, g], is_dual)
    } else {
        select_23([b, e, c, d, f, g], is_dual)
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
        select_13([b, c, d], is_dual)
    } else {
        select_14([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_23([a, b, c, d, e, f], is_dual)
    } else {
        select_25([e, c, d, a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_21([a, g, b, d, e, f, h], is_dual)
    } else {
        select_24([a, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_19([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_20([e, a, b, c, i, f, g, h], is_dual)
    } else {
        select_20([e, a, b, d, h, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_19([a, b, c, d, e, g, h, i, f], is_dual)
    } else {
        select_19([a, b, c, f, e, g, h, i, d], is_dual)
    }
}
/// n = 9, i = 1
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_18([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_18([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_31([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_23([a, b, c, d, f, g], is_dual)
    } else {
        select_23([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_31([a, g, c, d, e, f, h], is_dual)
    } else {
        select_31([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_11([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_31([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_30([b, h, d, e, f, g, a, i], is_dual)
    } else {
        select_32([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_29([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_29([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_27([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_28([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_28([a, b, c, e, f, g, h, d, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_35([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_21([a, b, c, e, f, d, g], is_dual)
    } else {
        select_21([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_40([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_14([a, d], is_dual)
    } else {
        select_14([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_39([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_40([a, c, b, d], is_dual)
    } else {
        select_40([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_38([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_39([a, b, c, d], is_dual)
    } else {
        select_39([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_37([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_38([a, b, c, e], is_dual)
    } else {
        select_39([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_41([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_24([a, b, d, e, c, f], is_dual)
    } else {
        select_24([a, c, d, e, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_36([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_37([a, b, c, f, d], is_dual)
    } else {
        select_41([a, b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_34([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_35([a, b, c, d, e, f, g], is_dual)
    } else {
        select_36([a, b, c, d, g, e], is_dual)
    }
}
/// n = 8, i = 1
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_34([a, b, c, d, e, f, h], is_dual)
    } else {
        select_35([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_26([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_27([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_33([a, b, c, d, e, f, j, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_16([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_17([a, b, c, d, e, g, j, i, f], is_dual)
    } else {
        select_26([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_5([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_16([a, b, c, d, e, f, h, l, k, g], is_dual)
    }
}
/// n = 10, i = 1
fn select_47([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_10([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_32([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_29([a, c, i, e, f, g, h, b, j], is_dual)
    } else {
        select_47([a, c, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_45([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_8([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_46([a, b, d, j, f, g, h, i, c, k], is_dual)
    }
}
/// n = 13, i = 1
fn select_44([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_45([g, h, i, a, b, c, d, m, j, k, l], is_dual)
    } else {
        select_45([g, h, i, a, b, e, f, l, j, k, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_43([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_44([a, f, b, c, d, e, g, h, i, k, j, l, m], is_dual)
    } else {
        select_44([a, j, b, c, d, e, g, h, i, k, f, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_42([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_43([e, a, b, c, d, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_43([k, a, b, c, d, f, g, h, i, j, e, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_4([a, b, c, d, e, f, g, l, j, k, h, m], is_dual)
    } else {
        select_42([h, i, j, k, a, b, c, d, e, f, g, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_3([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 10, i = 1
fn select_50([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_16([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_16([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 12, i = 1
fn select_51([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_5([a, b, c, d, e, f, g, i, j, k, h, l], is_dual)
    } else {
        select_5([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_49([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_50([a, b, c, d, e, f, g, h, l, i], is_dual)
    } else {
        select_51([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 1
fn select_59([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_31([c, d, e, f, g, h, i], is_dual)
    } else {
        select_31([c, d, a, b, i, h, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_31([b, c, d, e, f, g, h], is_dual)
    } else {
        select_23([a, h, b, c, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([a, f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_60([g, b, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_57([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_20([a, c, h, e, f, g, b, i], is_dual)
    } else {
        select_58([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 1
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_8([f, g, a, b, c, d, e, h, i, j, k], is_dual)
    } else {
        select_57([f, g, a, k, b, c, i, h, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_55([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_56([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_56([i, a, b, c, d, f, g, h, e, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_54([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_55([a, g, h, i, b, c, d, e, f, j, k], is_dual)
    } else {
        select_26([b, c, d, e, f, j, h, i, a, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_53([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_54([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_54([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_52([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_50([a, b, c, d, e, f, g, h, k, i], is_dual)
    } else {
        select_53([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_48([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_49([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_52([a, b, c, d, e, f, g, h, i, l, j], is_dual)
    }
}
/// n = 13, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_48([a, b, c, d, e, f, g, h, i, j, m, k], is_dual)
    }
}
/// n = 13, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    }
}
