/// n = 1, i = 0
fn select_21([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_20([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_21([a], is_dual)
    } else {
        select_21([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_19([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_20([a, b], is_dual)
    } else {
        select_20([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_18([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_19([a, b, c], is_dual)
    } else {
        select_19([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_17([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([d, e, f, g], is_dual)
    } else {
        select_18([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_16([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_17([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_17([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 11, i = 1
fn select_15([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_16([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_16([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_14([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_15([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_15([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_25([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_19([a, b, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_24([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_25([a, h, d, e, f, g, i], is_dual)
    } else {
        select_17([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_23([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_16([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_24([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_22([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_23([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_23([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_14([a, d, g, h, i, b, c, k, j, l, m], is_dual)
    } else {
        select_22([a, b, c, e, f, d, j, l, k, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_13([a, d, e, c, h, i, g, b, j, k, l, m, o, n], is_dual)
    } else {
        select_13([a, d, e, b, h, i, f, c, k, j, l, n, o, m], is_dual)
    }
}
/// n = 15, i = 2
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_12([a, b, d, e, f, c, g, h, i, k, l, m, j, n, o], is_dual)
    } else {
        select_12([a, c, d, e, f, b, g, h, i, j, l, m, k, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_28([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_14([g, i, a, c, d, h, k, m, j, l, o], is_dual)
    } else {
        select_14([h, i, b, e, f, g, j, l, k, m, n], is_dual)
    }
}
/// n = 16, i = 2
fn select_27([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_28([f, g, b, j, h, i, c, d, a, k, l, n, m, o, p], is_dual)
    } else {
        select_28([e, g, c, k, h, i, b, d, a, j, l, o, m, n, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_26([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_27([a, b, d, e, c, f, g, h, i, k, l, m, n, j, o, p], is_dual)
    } else {
        select_27([a, c, d, e, b, f, g, h, i, j, l, m, n, k, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < p) || (is_dual && n > p) {
        select_11([a, b, c, f, d, e, h, g, m, j, k, l, p, o, n], is_dual)
    } else {
        select_26([a, b, c, f, g, h, i, d, e, j, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_10([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o, p], is_dual)
    } else {
        select_10([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_9([h, i, j, e, c, d, b, f, a, g, l, k, n, o, p, m], is_dual)
    } else {
        select_9([h, i, j, e, c, d, b, g, a, f, l, k, m, o, p, n], is_dual)
    }
}
/// n = 16, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_8([k, j, h, i, a, f, g, b, c, d, n, e, l, m, o, p], is_dual)
    } else {
        select_8([k, j, h, i, e, f, g, b, c, d, n, a, l, m, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_7([a, b, c, d, e, f, h, i, j, k, l, g, m, n, o, p], is_dual)
    } else {
        select_7([a, b, c, d, e, g, h, i, j, k, l, f, m, n, o, p], is_dual)
    }
}
/// n = 14, i = 2
fn select_37([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_23([h, b, e, f, g, i, k, j, l, m], is_dual)
    } else {
        select_23([g, a, c, d, h, j, l, i, k, n], is_dual)
    }
}
/// n = 9, i = 1
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_24([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_38([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_23([c, f, g, h, a, b, j, i, k, l], is_dual)
    } else {
        select_39([a, b, d, e, c, i, k, j, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_37([e, f, a, b, g, h, c, d, i, j, k, l, m, n], is_dual)
    } else {
        select_38([a, b, d, c, i, f, g, h, j, m, l, k, n], is_dual)
    }
}
/// n = 5, i = 1
fn select_43([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_21([e], is_dual)
    }
}
/// n = 5, i = 1
fn select_45([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_19([b, c, d], is_dual)
    } else {
        select_20([a, e], is_dual)
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
fn select_42([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_43([a, c, d, e, f], is_dual)
    } else {
        select_44([a, b, c, e, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_41([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_23([c, d, a, b, e, f, g, h, i, j], is_dual)
    } else {
        select_42([a, b, c, h, j, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([c, d, e], is_dual)
    } else {
        select_19([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_50([a, b, c, d, e, f], is_dual)
    } else {
        select_45([e, c, d, a, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_25([c, d, a, e, f, g, h], is_dual)
    } else {
        select_49([a, b, e, g, f, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_54([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_20([a, d], is_dual)
    } else {
        select_20([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_53([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_54([a, c, d, g], is_dual)
    } else {
        select_54([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_45([a, c, d, e, h], is_dual)
    } else {
        select_53([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_20([a, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_51([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_52([a, c, b, h, f, g, i, j], is_dual)
    } else {
        select_55([c, d, e, i, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_47([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_48([a, b, e, f, d, g, i, j, k], is_dual)
    } else {
        select_51([b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_57([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_49([b, d, a, e, f, g], is_dual)
    } else {
        select_49([a, c, b, f, e, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_61([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_20([a, b], is_dual)
    } else {
        select_21([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_60([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_54([b, c, e, d], is_dual)
    } else {
        select_61([a, b, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_62([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_54([a, c, b, d], is_dual)
    } else {
        select_54([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_59([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_60([a, d, c, e, f, g], is_dual)
    } else {
        select_62([a, b, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([a, b, c, f, e, h, g], is_dual)
    } else {
        select_44([a, b, d, g, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_57([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_58([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_47([c, e, d, f, a, b, h, g, i, k, j], is_dual)
    } else {
        select_56([c, a, b, e, g, h, j, k, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_40([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_41([a, b, c, e, f, g, k, i, j, l, m], is_dual)
    } else {
        select_46([a, b, c, d, e, l, h, i, j, k, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_36([b, c, a, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_40([b, c, d, i, f, g, h, l, j, k, a, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_63([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_13([i, e, f, d, g, h, a, b, c, j, m, k, l, n], is_dual)
    } else {
        select_13([i, g, h, d, e, f, a, b, c, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_34([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_35([a, d, e, f, c, g, h, i, j, k, l, b, m, n], is_dual)
    } else {
        select_63([g, h, i, f, b, c, d, e, a, k, m, j, l, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_34([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    } else {
        select_34([a, b, d, e, f, g, h, i, j, c, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_71([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_62([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([c, d, e, g], is_dual)
    } else {
        select_45([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_70([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_71([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_72([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_75([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_19([a, b, c], is_dual)
    } else {
        select_21([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_74([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_75([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([b, f, c, d, e, g, h], is_dual)
    } else {
        select_74([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_70([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_73([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_77([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_17([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_74([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_76([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_70([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_77([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_68([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_69([a, b, c, e, f, h, g, j, i], is_dual)
    } else {
        select_76([a, b, d, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_67([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_68([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_68([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_82([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_50([a, f, c, d, e, g], is_dual)
    } else {
        select_50([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_81([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_82([a, c, d, e, b, f, g], is_dual)
    } else {
        select_82([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_80([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_81([a, c, e, b, f, g, h], is_dual)
    } else {
        select_81([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_79([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_80([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_80([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_79([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_79([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_66([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_67([a, b, c, d, h, f, g, i, k, j], is_dual)
    } else {
        select_78([a, b, c, d, j, e, k, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_89([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_54([a, b, c, d], is_dual)
    } else {
        select_61([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_88([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_89([b, c, a, d, e], is_dual)
    } else {
        select_54([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_87([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_43([a, c, d, e, f], is_dual)
    } else {
        select_88([a, b, f, g, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_86([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_87([a, b, d, e, c, f, g], is_dual)
    } else {
        select_87([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, e], is_dual)
    } else {
        select_54([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_93([a, b, c, e, f, g, h], is_dual)
    } else {
        select_19([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_71([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_92([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_91([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_91([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_85([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_86([a, b, c, d, h, g, i], is_dual)
    } else {
        select_90([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_86([a, b, c, d, g, f, h], is_dual)
    } else {
        select_86([a, b, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_85([a, b, c, g, e, f, h, i, j], is_dual)
    } else {
        select_94([a, b, c, d, i, g, j, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_69([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_69([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_101([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_54([a, b, c, d], is_dual)
    } else {
        select_61([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_100([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_60([a, b, c, e, d, f], is_dual)
    } else {
        select_101([d, c, a, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_102([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_45([b, a, c, d, e], is_dual)
    } else {
        select_54([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_99([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_100([a, c, d, b, e, f], is_dual)
    } else {
        select_102([a, b, d, c, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_104([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_89([d, f, e, a, b], !is_dual)
    } else {
        select_89([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_103([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, a, d, e, f], is_dual)
    } else {
        select_104([b, c, f, e, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_98([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_99([a, f, b, c, g, e], is_dual)
    } else {
        select_103([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_97([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_98([a, b, d, e, c, f, g], is_dual)
    } else {
        select_98([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_96([a, b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_97([a, b, c, e, h, d, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_83([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_84([a, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_95([a, b, c, i, d, f, g, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_65([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_66([a, b, c, d, e, f, j, i, k, h, l], is_dual)
    } else {
        select_83([a, b, c, d, e, g, h, i, k, j, l], is_dual)
    }
}
/// n = 8, i = 1
fn select_110([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_25([a, b, c, d, e, g, h], is_dual)
    } else {
        select_25([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_109([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_110([b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_24([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 0
fn select_113([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_18([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_113([d, e, f, g, i], is_dual)
    } else {
        select_55([a, b, c, j, h, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_110([c, j, d, e, f, g, i, k], is_dual)
    } else {
        select_112([a, b, i, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_108([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_109([b, c, k, d, e, f, g, i, j, l], is_dual)
    } else {
        select_111([a, j, b, d, e, f, g, h, i, k, l, m], is_dual)
    }
}
/// n = 11, i = 2
fn select_116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_49([a, b, c, j, h, k], is_dual)
    } else {
        select_113([d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_117([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_113([c, d, e, f, h], is_dual)
    } else {
        select_45([a, b, i, g, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_115([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_116([h, a, c, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_117([i, b, d, e, f, g, j, h, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_109([b, c, k, d, e, f, g, h, j, l], is_dual)
    } else {
        select_115([a, b, j, d, e, f, g, h, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_107([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_108([c, a, d, e, f, g, h, i, b, j, k, l, m], is_dual)
    } else {
        select_114([c, b, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_121([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_60([a, c, b, f, g, e], is_dual)
    } else {
        select_60([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([a, c, d, e, g, f], is_dual)
    } else {
        select_121([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([b, d, a, c, e, g, f, i, h], is_dual)
    } else {
        select_120([b, f, e, d, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_80([a, b, f, e, c, h, g, j, i], is_dual)
    } else {
        select_119([a, b, g, e, d, i, h, f, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_106([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < b) || (is_dual && l > b) {
        select_107([a, c, d, e, b, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_118([a, c, d, l, e, b, i, j, k, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_126([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_43([a, b, c, d, f], is_dual)
    } else {
        select_43([a, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_125([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_42([a, b, c, d, e, g], is_dual)
    } else {
        select_126([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_127([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_42([a, b, d, e, f, h], is_dual)
    } else {
        select_81([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_125([a, b, d, e, f, g, i], is_dual)
    } else {
        select_127([a, b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_130([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_25([a, g, c, d, e, f, h], is_dual)
    } else {
        select_25([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_24([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_130([b, h, d, e, f, g, a, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_74([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_74([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_24([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_132([b, h, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_128([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_129([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_131([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_124([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_128([a, b, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_77([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_73([a, h, d, e, f, g, b, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_134([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_135([a, c, e, b, f, h, g, i, j, k], is_dual)
    } else {
        select_135([a, b, d, c, f, i, g, h, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_133([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_134([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_134([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_122([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_123([a, b, d, f, g, h, i, l, k, m, n], is_dual)
    } else {
        select_133([a, b, c, d, e, f, m, j, k, l, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_105([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < l) || (is_dual && c > l) {
        select_106([a, b, d, j, f, g, h, i, l, k, c, m, n], is_dual)
    } else {
        select_122([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_64([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < m) || (is_dual && d > m) {
        select_65([a, b, c, e, f, m, g, d, j, k, l, n], is_dual)
    } else {
        select_105([a, b, c, e, f, g, d, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_32([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_33([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_64([a, b, c, d, g, f, h, i, j, k, l, e, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_31([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_32([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_32([a, b, c, d, e, g, h, i, j, k, f, l, m, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_30([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_31([a, b, c, d, e, f, g, i, k, h, l, m, n, o], is_dual)
    } else {
        select_31([a, b, c, d, e, f, g, h, j, i, m, l, o, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_50([b, c, d, e, g, f], is_dual)
    } else {
        select_75([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_145([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_146([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_50([c, g, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_25([b, c, d, e, f, g, h], is_dual)
    } else {
        select_50([a, h, b, c, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_145([a, h, b, d, e, g, f, i, j], is_dual)
    } else {
        select_147([b, c, f, d, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_149([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_43([a, c, d, e, g], is_dual)
    } else {
        select_49([a, b, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_150([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_43([a, b, c, e, g], is_dual)
    } else {
        select_49([b, d, a, c, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_148([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_149([b, c, d, e, f, g, h], is_dual)
    } else {
        select_150([a, b, h, c, g, f, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_143([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_144([a, d, e, b, g, f, h, i, j, k], is_dual)
    } else {
        select_148([a, b, c, h, i, f, g, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_152([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_77([a, b, i, d, e, f, g, h, j, k], is_dual)
    } else {
        select_77([a, c, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_151([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_152([a, d, e, b, f, h, g, i, j, k, l], is_dual)
    } else {
        select_135([a, b, c, f, i, j, g, h, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_142([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_143([a, b, c, e, f, g, k, i, j, h, l], is_dual)
    } else {
        select_151([a, c, b, e, f, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 1
fn select_156([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_113([d, e, f, g, h], is_dual)
    } else {
        select_18([a, b, c, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_155([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_116([a, b, j, e, f, g, h, i, k, l, m], is_dual)
    } else {
        select_156([c, d, k, e, f, g, h, j, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_154([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_155([a, b, c, k, e, f, g, h, i, j, l, m, n], is_dual)
    } else {
        select_155([a, b, d, j, e, f, g, h, i, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_153([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_154([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_154([b, a, c, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_142([f, g, h, n, d, e, k, l, i, j, m, o], is_dual)
    } else {
        select_153([f, g, d, e, a, b, c, m, k, l, i, j, n, o], is_dual)
    }
}
/// n = 10, i = 2
fn select_161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_71([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_71([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_163([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_93([a, c, d, b, e, f, g], is_dual)
    } else {
        select_93([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_71([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_163([a, b, c, h, g, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_160([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_161([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_162([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_161([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_161([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_159([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_160([a, b, c, d, h, i, g, j, l, k], is_dual)
    } else {
        select_164([a, b, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_167([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_52([a, b, d, e, f, g, i, j], is_dual)
    } else {
        select_82([b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 12, i = 2
fn select_166([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_130([c, d, e, f, k, h, i, j], is_dual)
    } else {
        select_167([a, c, d, b, j, g, h, i, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_169([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_52([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_54([c, g, f, a], is_dual)
    }
}
/// n = 10, i = 2
fn select_168([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_149([a, c, d, e, h, g, i], is_dual)
    } else {
        select_169([a, b, c, i, f, g, h, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_165([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_166([c, f, d, e, a, g, h, i, j, k, l, m], is_dual)
    } else {
        select_168([a, c, b, i, j, h, g, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_158([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_159([a, h, b, c, e, f, i, j, k, g, l, m], is_dual)
    } else {
        select_165([b, c, a, e, f, d, i, g, j, k, l, h, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_157([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_158([d, e, f, a, b, c, j, i, l, g, h, k, m], is_dual)
    } else {
        select_158([d, f, e, a, b, c, j, i, k, g, h, l, m], is_dual)
    }
}
/// n = 15, i = 2
fn select_140([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_141([a, b, h, c, d, e, f, g, i, j, k, l, m, n, o], is_dual)
    } else {
        select_157([n, c, d, e, f, g, i, j, h, k, l, m, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_139([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_140([h, i, f, g, a, d, e, b, l, m, c, j, k, n, o], is_dual)
    } else {
        select_140([h, i, f, g, c, d, e, b, l, m, a, j, k, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_139([a, b, c, d, f, g, h, i, j, e, k, l, m, n, o], is_dual)
    } else {
        select_139([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_138([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o], is_dual)
    } else {
        select_138([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    }
}
/// n = 16, i = 2
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < p) || (is_dual && o > p) {
        select_31([a, b, c, d, e, f, g, h, i, m, n, l, p, o], is_dual)
    } else {
        select_137([a, b, c, d, e, f, g, j, k, h, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_29([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < o) || (is_dual && l > o) {
        select_30([a, b, c, d, e, f, g, h, i, m, k, o, n, l, p], is_dual)
    } else {
        select_136([a, b, c, d, e, f, g, i, k, h, j, n, l, m, p, o], is_dual)
    }
}
/// n = 16, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_6([a, b, c, d, e, f, g, i, h, j, k, l, m, o, n, p], is_dual)
    } else {
        select_29([a, b, c, d, e, f, g, i, k, j, l, m, n, o, h, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < p) || (is_dual && o > p) {
        select_5([e, f, g, h, i, j, k, l, a, b, c, d, m, n, p, o], is_dual)
    } else {
        select_5([e, f, g, h, i, j, k, l, a, b, d, c, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < d) || (is_dual && m > d) {
        select_4([a, b, c, d, e, f, g, h, i, j, k, l, n, o, p, m], is_dual)
    } else {
        select_4([a, b, c, m, e, f, g, h, i, j, k, l, n, o, p, d], is_dual)
    }
}
/// n = 16, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_3([a, m, n, b, c, d, e, f, g, h, i, j, k, l, o, p], is_dual)
    } else {
        select_3([l, m, n, b, c, d, e, f, g, h, i, j, k, a, o, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m, p], is_dual)
    }
}
/// n = 16, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < o) || (is_dual && p > o) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o], is_dual)
    }
}
