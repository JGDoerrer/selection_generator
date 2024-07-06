/// n = 1, i = 0
fn select_20([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_19([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_20([a], is_dual)
    } else {
        select_20([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_18([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_19([a, b], is_dual)
    } else {
        select_19([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_17([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_18([a, b, c], is_dual)
    } else {
        select_18([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_16([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_17([a, b, c, d], is_dual)
    } else {
        select_20([e], is_dual)
    }
}
/// n = 7, i = 1
fn select_15([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([e, c, d, f, g], is_dual)
    } else {
        select_16([e, a, b, g, f], is_dual)
    }
}
/// n = 3, i = 1
fn select_24([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_19([a, b], is_dual)
    } else {
        select_20([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_24([a, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_22([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_17([c, d, e, g], is_dual)
    } else {
        select_23([a, b, h, f, i, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_27([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_19([a, d], is_dual)
    } else {
        select_19([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_24([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, b, e, f], is_dual)
    } else {
        select_27([a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_26([a, e, c, f, d, h], is_dual)
    } else {
        select_28([b, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_21([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_22([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_25([b, g, c, f, a, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_14([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_15([c, h, d, e, a, g, i], is_dual)
    } else {
        select_21([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([b, c, d, e], is_dual)
    } else {
        select_19([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_17([d, e, f, g], is_dual)
    } else {
        select_17([a, b, c, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_31([g, d, e, f, a, h], is_dual)
    } else {
        select_32([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_29([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_30([e, a, b, c, d, j, h, i], is_dual)
    } else {
        select_21([e, f, i, a, b, g, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_14([a, b, h, e, f, g, c, i, j, k, l], is_dual)
    } else {
        select_29([c, d, e, f, a, b, g, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_13([a, b, d, f, c, h, g, i, j, k, l, m], is_dual)
    } else {
        select_13([a, b, c, e, d, i, g, h, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_12([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_12([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_39([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, c, d, g], is_dual)
    } else {
        select_27([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_39([a, d, b, g, e, f, h], is_dual)
    } else {
        select_39([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_41([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, b, d], is_dual)
    } else {
        select_27([b, c, a, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_40([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_41([a, g, c, i], is_dual)
    } else {
        select_39([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_37([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_38([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_40([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_36([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_37([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_37([c, b, d, e, f, a, g, h, j, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_45([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_19([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_44([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_45([a, c, d, b, e], is_dual)
    } else {
        select_45([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_43([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_16([a, c, d, e, f], is_dual)
    } else {
        select_44([a, b, c, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_48([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([c, d, e], is_dual)
    } else {
        select_18([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_47([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_48([a, f, c, d, e, g], is_dual)
    } else {
        select_48([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_46([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_47([a, c, d, e, b, f, g], is_dual)
    } else {
        select_47([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_43([a, b, d, e, f, h], is_dual)
    } else {
        select_46([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_36([a, b, c, j, d, g, h, i, k, l], is_dual)
    } else {
        select_42([a, b, d, e, f, i, h, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_34([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_35([a, b, c, e, d, i, g, h, j, l, k, m], is_dual)
    } else {
        select_13([b, c, d, f, a, h, g, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_34([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_34([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_11([b, c, d, e, a, f, h, i, g, k, j, l, m], is_dual)
    } else {
        select_33([a, b, c, e, d, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_10([a, b, d, e, c, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_10([a, c, d, f, b, e, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_9([d, b, e, c, a, f, j, h, i, g, l, m, k], is_dual)
    } else {
        select_9([d, b, f, c, a, e, j, h, i, g, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_8([b, c, a, d, e, f, i, g, h, k, l, m, j], is_dual)
    } else {
        select_8([c, b, a, d, e, f, j, g, h, k, l, m, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_7([a, b, c, d, e, f, h, i, j, k, l, m, g], is_dual)
    } else {
        select_7([a, b, c, d, e, g, h, i, j, k, l, m, f], is_dual)
    }
}
/// n = 13, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_6([a, b, c, d, e, f, g, i, j, k, l, m, h], is_dual)
    } else {
        select_6([a, b, c, d, h, f, g, i, j, k, l, m, e], is_dual)
    }
}
/// n = 13, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_5([i, a, g, h, b, c, d, e, l, m, f, j, k], is_dual)
    } else {
        select_5([i, f, g, h, b, c, d, e, l, m, a, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_4([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_4([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_3([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_12([b, c, d, e, a, f, h, g, j, k, i, l, m], is_dual)
    } else {
        select_34([a, b, c, e, d, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_56([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_57([a, b, d, e, c, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_57([a, c, d, f, b, e, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_55([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_56([d, b, e, c, a, f, i, h, g, k, l, m, j], is_dual)
    } else {
        select_56([d, b, f, c, a, e, i, h, g, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_54([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_55([b, c, a, d, e, f, h, g, j, k, l, m, i], is_dual)
    } else {
        select_55([c, b, a, d, e, f, i, g, j, k, l, m, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_53([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_54([a, b, c, d, e, f, h, i, j, k, l, g, m], is_dual)
    } else {
        select_54([a, b, c, d, e, g, h, i, j, k, l, f, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_52([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_53([a, b, c, d, e, f, g, i, j, k, l, h, m], is_dual)
    } else {
        select_53([a, b, c, d, h, f, g, i, j, k, l, e, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_51([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_52([i, a, g, h, b, c, d, e, l, f, j, k, m], is_dual)
    } else {
        select_52([i, f, g, h, b, c, d, e, l, a, j, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_50([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_51([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_51([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_49([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_50([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_50([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_2([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_49([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    }
}
