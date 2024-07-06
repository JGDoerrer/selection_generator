/// n = 1, i = 0
fn select_17([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_16([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_17([a], is_dual)
    } else {
        select_17([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_15([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_16([a, b], is_dual)
    } else {
        select_16([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_14([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_15([a, b, c], is_dual)
    } else {
        select_15([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_13([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_14([d, e, f, g], is_dual)
    } else {
        select_14([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_12([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_13([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_13([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 12, i = 1
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_12([c, d, i, e, f, g, h, k, l, j], is_dual)
    } else {
        select_12([a, b, j, e, f, g, h, k, l, i], is_dual)
    }
}
/// n = 13, i = 1
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_11([a, j, c, d, e, f, g, h, i, k, l, m], is_dual)
    } else {
        select_11([b, i, c, d, e, f, g, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_10([a, h, b, c, d, e, f, g, j, i, k, l, m], is_dual)
    } else {
        select_10([a, i, b, c, d, e, f, g, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_9([g, a, b, c, d, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_9([j, a, b, c, d, e, f, h, i, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_8([a, l, c, d, e, f, g, h, i, j, k, m, n], is_dual)
    } else {
        select_8([b, k, c, d, e, f, g, h, i, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_7([a, f, b, c, d, e, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_7([a, k, b, c, d, e, g, h, i, j, l, f, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_6([e, a, b, c, d, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_6([l, a, b, c, d, f, g, h, i, j, k, e, m, n], is_dual)
    }
}
/// n = 15, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_5([i, n, k, l, a, b, c, d, e, f, g, h, m, o], is_dual)
    } else {
        select_5([j, m, k, l, a, b, c, d, e, f, g, h, n, o], is_dual)
    }
}
/// n = 15, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_4([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, j, k, l, m, i, n, o], is_dual)
    }
}
/// n = 15, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 6, i = 1
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([c, d, e], is_dual)
    } else {
        select_15([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_30([a, b, c, d, f, g], is_dual)
    } else {
        select_30([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_13([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_29([a, h, d, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_27([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_28([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_32([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([a, f, c, d, e, g], is_dual)
    } else {
        select_30([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_31([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_32([a, c, d, e, b, f, g], is_dual)
    } else {
        select_32([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_27([e, f, a, b, c, d, g, h, i], is_dual)
    } else {
        select_31([e, f, i, a, b, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_12([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_28([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_33([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_34([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_34([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_25([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_26([a, j, d, e, f, g, h, i, k], is_dual)
    } else {
        select_33([f, g, h, d, e, b, c, i, k, j], is_dual)
    }
}
/// n = 9, i = 1
fn select_38([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_29([c, d, e, f, g, h, i], is_dual)
    } else {
        select_29([c, d, a, b, i, h, g], is_dual)
    }
}
/// n = 11, i = 1
fn select_37([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_12([b, c, h, d, e, f, g, j, k, i], is_dual)
    } else {
        select_38([a, i, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([a, h, b, c, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_37([i, b, c, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_36([g, a, b, c, d, e, f, h, i, j, k, l], is_dual)
    } else {
        select_36([i, a, b, c, d, e, f, h, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_24([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_25([j, b, c, d, e, f, g, h, i, k, l], is_dual)
    } else {
        select_35([a, i, b, c, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 1
fn select_23([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_8([a, f, b, c, d, e, g, h, i, j, k, l, m], is_dual)
    } else {
        select_24([k, b, c, d, e, g, h, i, j, f, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_22([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_23([e, a, b, c, d, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_23([k, a, b, c, d, f, g, h, i, j, e, l, m], is_dual)
    }
}
/// n = 14, i = 1
fn select_21([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_22([h, m, j, k, a, b, c, d, e, f, g, l, n], is_dual)
    } else {
        select_22([i, l, j, k, a, b, c, d, e, f, g, m, n], is_dual)
    }
}
/// n = 11, i = 1
fn select_42([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_25([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_25([i, a, b, c, d, f, g, h, e, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_46([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_31([a, b, c, e, f, d, g], is_dual)
    } else {
        select_31([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_51([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_16([a, d], is_dual)
    } else {
        select_16([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_50([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_51([a, c, b, d], is_dual)
    } else {
        select_51([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_49([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_50([a, b, c, d], is_dual)
    } else {
        select_50([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_48([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_49([a, b, c, e], is_dual)
    } else {
        select_50([b, c, d, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_54([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_15([b, c, d], is_dual)
    } else {
        select_16([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_53([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_54([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_53([a, b, d, e, c, f], is_dual)
    } else {
        select_53([a, c, d, e, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_47([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_48([a, b, c, f, d], is_dual)
    } else {
        select_52([a, b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_45([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_46([a, b, c, d, e, f, g], is_dual)
    } else {
        select_47([a, b, c, d, g, e], is_dual)
    }
}
/// n = 8, i = 1
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_45([a, b, c, d, e, f, h], is_dual)
    } else {
        select_46([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_59([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_54([a, b, c, e, f], is_dual)
    } else {
        select_54([a, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_58([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_53([a, b, c, d, f, g], is_dual)
    } else {
        select_59([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_57([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_31([a, b, c, d, e, g, h], is_dual)
    } else {
        select_58([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_27([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_57([a, b, h, e, f, g, c, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_56([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_56([a, b, c, e, f, g, h, d, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_43([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_44([a, b, c, d, e, f, j, i], is_dual)
    } else {
        select_55([a, b, c, d, e, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_41([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_42([a, g, h, i, b, c, d, e, f, j, k], is_dual)
    } else {
        select_43([b, c, d, e, f, j, h, i, a, k], is_dual)
    }
}
/// n = 12, i = 1
fn select_40([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_41([d, e, f, g, h, i, a, b, l, j, k], is_dual)
    } else {
        select_41([d, e, f, g, h, i, a, c, k, j, l], is_dual)
    }
}
/// n = 12, i = 1
fn select_61([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_42([f, k, h, i, a, b, c, d, e, j, l], is_dual)
    } else {
        select_42([g, j, h, i, a, b, c, d, e, k, l], is_dual)
    }
}
/// n = 13, i = 1
fn select_60([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_23([g, a, h, i, j, b, c, d, e, f, k, l, m], is_dual)
    } else {
        select_61([b, c, d, e, f, g, l, i, j, k, a, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_39([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_40([a, b, m, d, e, f, g, h, i, k, l, j], is_dual)
    } else {
        select_60([d, e, f, g, h, i, a, b, c, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 1
fn select_20([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_21([d, e, f, g, h, i, j, a, b, c, k, l, m, n], is_dual)
    } else {
        select_39([a, b, n, d, e, f, g, h, i, j, l, m, k], is_dual)
    }
}
/// n = 14, i = 1
fn select_19([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_20([a, k, l, b, c, d, e, f, g, h, i, j, m, n], is_dual)
    } else {
        select_20([j, k, l, b, c, d, e, f, g, h, i, a, m, n], is_dual)
    }
}
/// n = 14, i = 1
fn select_18([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_19([a, b, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_19([a, b, c, d, e, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 15, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < m) || (is_dual && o > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_18([a, b, c, d, e, f, g, h, i, j, k, l, o, m], is_dual)
    }
}
/// n = 15, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
