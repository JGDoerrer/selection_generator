/// n = 1, i = 0
fn select_16([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_15([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_16([a], is_dual)
    } else {
        select_16([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_14([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_15([a, b], is_dual)
    } else {
        select_15([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_13([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_14([a, b, c], is_dual)
    } else {
        select_14([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_12([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_13([d, e, f, g], is_dual)
    } else {
        select_13([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_11([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_12([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_12([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 12, i = 1
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([c, d, i, e, f, g, h, k, l, j], is_dual)
    } else {
        select_11([a, b, j, e, f, g, h, k, l, i], is_dual)
    }
}
/// n = 13, i = 1
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_10([a, j, c, d, e, f, g, h, i, k, l, m], is_dual)
    } else {
        select_10([b, i, c, d, e, f, g, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_9([a, h, b, c, d, e, f, g, j, i, k, l, m], is_dual)
    } else {
        select_9([a, i, b, c, d, e, f, g, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_8([g, a, b, c, d, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_8([j, a, b, c, d, e, f, h, i, g, k, l, m], is_dual)
    }
}
/// n = 11, i = 1
fn select_20([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_11([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_11([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_19([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_20([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_20([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_25([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_14([c, d, e], is_dual)
    } else {
        select_14([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([a, f, c, d, e, g], is_dual)
    } else {
        select_25([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, d, e, b, f, g], is_dual)
    } else {
        select_24([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_27([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_14([b, c, d], is_dual)
    } else {
        select_15([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_25([a, b, c, d, e, f], is_dual)
    } else {
        select_27([e, c, d, a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_23([a, g, b, d, e, f, h], is_dual)
    } else {
        select_26([a, c, d, e, g, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_25([a, b, c, d, f, g], is_dual)
    } else {
        select_25([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_30([c, d, e, f, g, h, i], is_dual)
    } else {
        select_30([c, d, a, b, i, h, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_31([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_30([b, c, d, e, f, g, h], is_dual)
    } else {
        select_25([a, h, b, c, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_29([a, f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_31([g, b, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_21([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_22([a, c, h, e, f, g, b, i], is_dual)
    } else {
        select_28([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 1
fn select_18([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_19([f, g, a, b, c, d, e, h, i, j, k], is_dual)
    } else {
        select_21([f, g, a, k, b, c, i, h, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_34([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([b, c, h, d, e, f, g, j, k, i], is_dual)
    } else {
        select_29([a, i, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_10([a, h, b, c, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_34([i, b, c, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_32([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_33([g, a, b, c, d, e, f, h, i, j, k, l], is_dual)
    } else {
        select_33([i, a, b, c, d, e, f, h, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_17([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_18([j, b, c, d, e, f, g, h, i, k, l], is_dual)
    } else {
        select_32([a, i, b, c, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_7([a, f, b, c, d, e, g, h, i, j, k, l, m], is_dual)
    } else {
        select_17([k, b, c, d, e, g, h, i, j, f, l, m], is_dual)
    }
}
/// n = 12, i = 1
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_18([c, a, b, d, k, f, g, h, i, l, j], is_dual)
    } else {
        select_18([c, a, b, e, j, f, g, h, i, l, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_36([h, i, a, f, g, b, c, d, e, j, k, l], is_dual)
    } else {
        select_36([h, i, e, f, g, b, c, d, a, j, k, l], is_dual)
    }
}
/// n = 13, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_6([g, a, h, i, j, b, c, d, e, f, k, l, m], is_dual)
    } else {
        select_35([b, c, d, e, f, g, l, i, j, k, a, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_5([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_5([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_4([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 7, i = 1
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_23([a, b, c, e, f, d, g], is_dual)
    } else {
        select_23([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_49([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_15([a, d], is_dual)
    } else {
        select_15([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_48([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_49([a, c, b, d], is_dual)
    } else {
        select_49([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_47([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_48([a, b, c, d], is_dual)
    } else {
        select_48([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_46([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_47([a, b, c, e], is_dual)
    } else {
        select_48([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_26([a, b, d, e, c, f], is_dual)
    } else {
        select_26([a, c, d, e, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_45([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_46([a, b, c, f, d], is_dual)
    } else {
        select_50([a, b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_43([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_44([a, b, c, d, e, f, g], is_dual)
    } else {
        select_45([a, b, c, d, g, e], is_dual)
    }
}
/// n = 8, i = 1
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_43([a, b, c, d, e, f, h], is_dual)
    } else {
        select_44([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_12([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_30([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_53([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_54([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_54([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_27([a, b, c, e, f], is_dual)
    } else {
        select_27([a, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_26([a, b, c, d, f, g], is_dual)
    } else {
        select_57([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_23([a, b, c, d, e, g, h], is_dual)
    } else {
        select_56([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_52([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_53([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_55([a, b, h, e, f, g, c, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_51([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_52([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_52([a, b, c, e, f, g, h, d, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_42([a, b, c, d, e, f, j, i], is_dual)
    } else {
        select_51([a, b, c, d, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_40([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_41([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_41([a, b, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_62([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_11([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_54([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_61([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_62([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_62([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_61([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_52([a, b, c, i, f, g, h, d, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_19([a, b, c, e, f, g, h, i, d, j, k], is_dual)
    } else {
        select_19([a, b, d, e, f, g, h, i, c, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_59([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_60([a, b, c, d, j, g, h, i, e, k], is_dual)
    } else {
        select_63([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    } else {
        select_59([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_39([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_40([a, b, c, d, e, f, g, h, l, k], is_dual)
    } else {
        select_58([a, b, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_38([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_39([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    } else {
        select_39([a, b, c, d, e, f, g, i, j, k, h, l], is_dual)
    }
}
/// n = 11, i = 1
fn select_65([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_40([a, b, c, d, e, f, g, h, k, j], is_dual)
    } else {
        select_40([a, b, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_64([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_65([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_65([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_37([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_38([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_64([a, b, c, d, e, f, g, h, i, l, j], is_dual)
    }
}
/// n = 13, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_37([a, b, c, d, e, f, g, h, i, j, m, k], is_dual)
    }
}
/// n = 14, i = 1
fn select_70([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_7([a, l, c, d, e, f, g, h, i, j, k, m, n], is_dual)
    } else {
        select_7([b, k, c, d, e, f, g, h, i, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_69([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_70([a, f, b, c, d, e, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_70([a, k, b, c, d, e, g, h, i, j, l, f, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_68([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_69([e, a, b, c, d, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_69([l, a, b, c, d, f, g, h, i, j, k, e, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_67([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_4([a, b, c, d, e, f, g, h, m, k, l, i, n], is_dual)
    } else {
        select_68([i, j, k, l, a, b, c, d, e, f, g, h, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_66([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_67([a, b, c, d, e, f, g, h, i, j, l, m, k, n], is_dual)
    } else {
        select_67([a, b, c, d, e, f, g, h, i, k, l, m, j, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, n, l], is_dual)
    } else {
        select_66([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
