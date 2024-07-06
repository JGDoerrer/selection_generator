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
/// n = 6, i = 1
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([c, d, e], is_dual)
    } else {
        select_18([a, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_21([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_19([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_16([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_17([a, b, c, d, e, f], is_dual)
    } else {
        select_21([e, c, d, a, f], is_dual)
    }
}
/// n = 4, i = 0
fn select_23([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_18([a, b, c], is_dual)
    } else {
        select_18([b, c, d], is_dual)
    }
}
/// n = 5, i = 0
fn select_22([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_23([a, b, c, d], is_dual)
    } else {
        select_23([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_15([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_16([a, b, c, j, h, k], is_dual)
    } else {
        select_22([d, e, f, g, i], is_dual)
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
/// n = 3, i = 1
fn select_28([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_19([a, b], is_dual)
    } else {
        select_20([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_28([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_25([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_26([a, c, b, f, g, e], is_dual)
    } else {
        select_26([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_24([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([a, c, d, e, g, f], is_dual)
    } else {
        select_25([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_14([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_15([b, c, d, a, e, f, g, h, j, i, k], is_dual)
    } else {
        select_24([b, i, d, c, a, h, k, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_31([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_23([a, b, c, d], is_dual)
    } else {
        select_20([e], is_dual)
    }
}
/// n = 5, i = 1
fn select_32([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_21([a, c, d, b, e], is_dual)
    } else {
        select_21([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_31([a, c, d, e, f], is_dual)
    } else {
        select_32([a, b, c, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_34([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_17([a, f, c, d, e, g], is_dual)
    } else {
        select_17([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_34([a, c, d, e, b, f, g], is_dual)
    } else {
        select_34([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_29([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_30([a, b, d, e, f, h], is_dual)
    } else {
        select_33([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 13, i = 2
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_14([a, b, f, j, c, d, e, i, k, m, l], is_dual)
    } else {
        select_29([a, b, f, g, h, l, i, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_13([a, c, f, g, h, e, b, i, j, k, m, n, l], is_dual)
    } else {
        select_13([a, b, f, g, h, d, c, j, i, l, m, n, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_12([a, b, d, c, e, f, g, h, j, k, i, l, m, n], is_dual)
    } else {
        select_12([a, c, d, b, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_11([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_11([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_10([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_10([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_9([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_9([a, b, c, d, e, g, f, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_8([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_8([a, b, c, d, e, g, h, i, j, k, f, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_7([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_7([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 5, i = 2
fn select_46([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_28([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_45([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_26([a, b, c, e, d, f], is_dual)
    } else {
        select_46([d, c, a, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_47([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_21([b, a, c, d, e], is_dual)
    } else {
        select_27([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_44([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_45([a, c, d, b, e, f], is_dual)
    } else {
        select_47([a, b, d, c, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_50([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_28([d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_50([d, f, e, a, b], !is_dual)
    } else {
        select_50([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_48([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_16([b, c, a, d, e, f], is_dual)
    } else {
        select_49([b, c, f, e, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_43([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_44([a, f, b, c, g, e], is_dual)
    } else {
        select_48([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_43([a, b, d, e, c, f, g], is_dual)
    } else {
        select_43([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_53([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_46([a, c, b, d, e], is_dual)
    } else {
        select_46([b, c, a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_44([a, b, c, d, e, f], is_dual)
    } else {
        select_53([a, b, e, f, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_57([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, b, d], is_dual)
    } else {
        select_27([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_56([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([c, d, e, f], is_dual)
    } else {
        select_57([a, b, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_59([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, e], is_dual)
    } else {
        select_27([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_59([a, b, c, e, f, g, h], is_dual)
    } else {
        select_18([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_56([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_58([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_55([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_55([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_52([a, b, c, g, d, h], is_dual)
    } else {
        select_54([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_42([a, b, c, e, g, d, h], is_dual)
    } else {
        select_51([a, b, c, e, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_59([b, c, d, e, g, f, h], is_dual)
    } else {
        select_57([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_64([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_64([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_63([a, b, c, h, e, f, g, i], is_dual)
    } else {
        select_63([a, b, d, g, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_62([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_62([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_60([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_61([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_61([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_40([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_41([a, b, c, d, e, h, i, g], is_dual)
    } else {
        select_60([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_40([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_40([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_38([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_39([a, b, c, d, e, k, f, h, j], is_dual)
    } else {
        select_39([a, b, c, d, e, j, g, i, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_71([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_45([a, b, c, e, f, g], is_dual)
    } else {
        select_21([b, c, d, e, f], is_dual)
    }
}
/// n = 3, i = 1
fn select_74([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([a, b, c], is_dual)
    } else {
        select_28([a, c, b], is_dual)
    }
}
/// n = 6, i = 2
fn select_73([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_74([a, e, f], is_dual)
    } else {
        select_21([a, b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_75([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_17([a, b, c, d, e, f], is_dual)
    } else {
        select_57([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_72([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_73([e, c, d, a, f, g], is_dual)
    } else {
        select_75([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_70([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_71([a, b, d, f, e, g, h], is_dual)
    } else {
        select_72([a, b, c, g, e, f, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_78([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([c, d, e, f], is_dual)
    } else {
        select_18([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_79([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([d, e, f, g], is_dual)
    } else {
        select_23([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_78([a, h, d, e, f, g, i], is_dual)
    } else {
        select_79([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_82([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_18([a, b, c], is_dual)
    } else {
        select_20([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_81([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([c, d, e, f], is_dual)
    } else {
        select_82([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_80([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_81([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_81([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_76([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_77([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_80([b, h, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_69([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_70([a, c, i, d, g, b, h, j], is_dual)
    } else {
        select_76([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_79([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_79([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_85([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_77([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_88([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([b, c, d, e], is_dual)
    } else {
        select_28([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_87([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_88([f, c, d, e, a, g, h], is_dual)
    } else {
        select_81([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_86([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_87([a, b, d, e, g, f, h, i], is_dual)
    } else {
        select_71([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_83([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_84([f, a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_86([f, j, a, b, c, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_68([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_69([a, b, i, d, f, g, c, h, j, k], is_dual)
    } else {
        select_83([d, c, e, f, g, a, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_78([a, g, c, d, e, f, h], is_dual)
    } else {
        select_78([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_77([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_92([b, h, d, e, f, g, a, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_94([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_26([e, a, c, f, d, g], is_dual)
    } else {
        select_26([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_93([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_34([b, c, a, d, e, f, g], is_dual)
    } else {
        select_94([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_90([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_91([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_93([a, i, c, d, b, g, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_69([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_90([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_67([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_68([a, b, c, f, g, d, h, i, j, k, l], is_dual)
    } else {
        select_89([a, b, c, d, e, i, j, h, l, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_98([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_33([a, c, e, b, f, g, h], is_dual)
    } else {
        select_33([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_97([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_98([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_98([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_97([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_97([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_95([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_96([a, b, c, d, e, g, j, i, k], is_dual)
    } else {
        select_96([a, b, c, e, d, f, k, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_66([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_67([a, b, c, e, g, d, f, j, h, i, l, k], is_dual)
    } else {
        select_95([a, b, c, d, e, i, g, k, j, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_65([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_66([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_66([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_37([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < l) || (is_dual && f > l) {
        select_38([a, b, c, d, e, g, j, i, l, k, f], is_dual)
    } else {
        select_65([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_56([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_56([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_108([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_59([a, c, d, b, e, f, g], is_dual)
    } else {
        select_59([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_56([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_108([a, b, c, h, g, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_106([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_107([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_104([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_105([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_105([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_110([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_106([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_106([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_105([a, b, c, d, h, i, g, j, l, k], is_dual)
    } else {
        select_110([a, b, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_104([b, c, a, d, h, f, k, i, j, g, l], is_dual)
    } else {
        select_109([b, c, d, f, a, e, i, g, h, j, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_102([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_103([d, b, c, e, a, f, h, g, j, k, l, i], is_dual)
    } else {
        select_103([d, b, c, f, a, e, h, g, i, k, l, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_101([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_102([g, e, f, a, c, d, j, b, h, i, k, l], is_dual)
    } else {
        select_102([g, e, f, b, c, d, j, a, h, i, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_100([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_101([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_101([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_100([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_100([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_36([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_37([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_99([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_35([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_36([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_36([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_35([a, b, c, d, e, f, g, h, i, m, j, n], is_dual)
    }
}
/// n = 11, i = 1
fn select_120([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_85([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_85([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_119([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_120([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_120([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 10, i = 1
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_84([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_91([a, c, i, e, f, g, h, b, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_119([a, c, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_121([a, b, d, e, c, k, l, i, j, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_117([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_118([a, d, c, g, h, f, b, i, k, l, j, m, o, n], is_dual)
    } else {
        select_118([a, d, b, g, h, e, c, j, k, l, i, n, o, m], is_dual)
    }
}
/// n = 15, i = 2
fn select_116([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_117([a, b, d, e, c, f, g, h, j, k, l, m, i, n, o], is_dual)
    } else {
        select_117([a, c, d, e, b, f, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_115([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_116([h, d, e, f, c, g, a, b, k, l, n, i, j, m, o], is_dual)
    } else {
        select_116([h, d, e, g, c, f, a, b, k, l, m, i, j, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_115([a, b, c, d, e, f, g, h, j, k, l, m, n, i, o], is_dual)
    } else {
        select_115([a, b, c, d, e, f, i, h, j, k, l, m, n, g, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_114([i, j, h, a, f, g, b, c, d, m, n, e, k, l, o], is_dual)
    } else {
        select_114([i, j, h, e, f, g, b, c, d, m, n, a, k, l, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_113([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o], is_dual)
    } else {
        select_113([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_112([a, b, c, d, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    } else {
        select_112([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < o) || (is_dual && m > o) {
        select_5([a, b, c, d, e, f, g, h, i, j, k, n, o, m], is_dual)
    } else {
        select_111([a, b, c, d, e, f, g, h, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_4([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 8, i = 1
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_22([c, d, e, f, g], is_dual)
    } else {
        select_18([a, b, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_22([c, d, e, f, h], is_dual)
    } else {
        select_21([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_133([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_134([b, i, c, d, e, f, h, j], is_dual)
    } else {
        select_135([a, h, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_132([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_133([c, a, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_133([c, b, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_139([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([c, d, e, g], is_dual)
    } else {
        select_21([a, b, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_78([b, h, c, d, e, g, i], is_dual)
    } else {
        select_139([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_137([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_138([c, a, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_138([c, b, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_136([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_137([c, d, a, b, e, f, g, h, j, i], is_dual)
    } else {
        select_137([e, f, a, b, c, d, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_131([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_132([c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_136([e, l, a, b, c, d, i, k, j, m], is_dual)
    }
}
/// n = 7, i = 1
fn select_143([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_31([a, c, d, e, g], is_dual)
    } else {
        select_16([a, b, c, e, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_144([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_16([a, b, d, e, c, f], is_dual)
    } else {
        select_16([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_142([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_143([a, d, b, c, e, f, g], is_dual)
    } else {
        select_144([a, b, c, e, f, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_145([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_30([a, b, d, e, f, g], is_dual)
    } else {
        select_144([a, b, c, d, e, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_141([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_142([a, b, c, d, i, g, j, h], is_dual)
    } else {
        select_145([a, b, c, e, f, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_148([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_94([c, d, a, e, f, b, g], is_dual)
    } else {
        select_94([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_149([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_16([e, c, a, d, f, g], is_dual)
    } else {
        select_16([a, b, d, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_147([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_148([a, b, c, e, f, h, i], is_dual)
    } else {
        select_149([a, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_146([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_29([a, b, d, e, f, i, g, j], is_dual)
    } else {
        select_147([a, b, d, c, j, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_140([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_141([g, a, b, f, c, d, h, j, k, i], is_dual)
    } else {
        select_146([c, d, f, e, a, b, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_130([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_131([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    } else {
        select_140([b, c, d, e, l, f, a, i, j, k, m], is_dual)
    }
}
/// n = 10, i = 1
fn select_152([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_84([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_84([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_151([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_152([g, h, i, c, d, e, f, j, k, l], is_dual)
    } else {
        select_152([e, f, i, a, b, g, h, k, j, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_150([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_151([a, b, e, f, c, d, g, h, i, j, k, m, l], is_dual)
    } else {
        select_151([a, b, g, h, c, d, e, f, i, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_129([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_130([a, d, e, f, g, c, h, i, j, k, b, l, m], is_dual)
    } else {
        select_150([h, i, f, g, b, c, d, e, a, l, j, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_129([a, b, c, e, f, g, h, i, j, d, k, l, m], is_dual)
    } else {
        select_129([a, b, d, e, f, g, h, i, j, c, k, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_157([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_84([g, a, c, d, h, j, l, i, k, n], is_dual)
    } else {
        select_84([h, b, e, f, g, i, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 1
fn select_159([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_77([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_77([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_158([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_84([c, f, g, h, a, b, j, i, k, l], is_dual)
    } else {
        select_159([a, b, d, e, c, i, k, j, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_156([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_157([e, f, a, b, g, h, c, d, i, j, k, l, m, n], is_dual)
    } else {
        select_158([a, b, d, c, i, f, g, h, j, m, l, k, n], is_dual)
    }
}
/// n = 11, i = 2
fn select_162([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_77([d, e, f, a, b, g, h, i, j], is_dual)
    } else {
        select_33([a, b, c, h, i, g, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_164([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_139([a, g, b, c, d, h, l, j, k], is_dual)
    } else {
        select_34([a, g, e, f, h, k, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_166([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_17([c, d, e, f, h, i], is_dual)
    } else {
        select_17([a, b, e, f, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_166([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_17([c, j, d, e, g, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_164([a, e, f, i, c, d, g, h, j, k, m, l], is_dual)
    } else {
        select_165([b, k, a, c, d, g, h, i, j, m, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_161([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_162([a, b, h, e, f, g, l, i, k, j, m], is_dual)
    } else {
        select_163([c, e, a, b, f, g, d, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < c) || (is_dual && k > c) {
        select_161([a, b, d, c, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_110([a, b, e, k, f, g, i, c, j, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_155([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_156([b, c, a, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_160([b, c, d, i, f, g, h, l, j, k, a, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_168([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_152([a, b, c, e, f, d, j, l, k, n], is_dual)
    } else {
        select_119([a, d, g, h, i, b, c, k, j, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_167([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_168([i, e, f, d, g, h, a, b, c, j, m, k, l, n], is_dual)
    } else {
        select_168([i, g, h, d, e, f, a, b, c, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_154([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_155([a, d, e, f, c, g, h, i, j, k, l, b, m, n], is_dual)
    } else {
        select_167([g, h, i, f, b, c, d, e, a, k, m, j, l, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_153([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_154([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    } else {
        select_154([a, b, d, e, f, g, h, i, j, c, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_128([g, h, i, j, c, d, e, f, b, k, l, n, m], is_dual)
    } else {
        select_153([g, h, i, j, c, d, b, a, e, f, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_176([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_75([a, c, b, h, g, f, i], is_dual)
    } else {
        select_81([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_178([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_17([b, c, d, e, g, f], is_dual)
    } else {
        select_82([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_177([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_178([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_78([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_176([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_177([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_175([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_175([a, b, e, d, f, g, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_152([e, f, h, a, b, c, d, i, j, k], is_dual)
    } else {
        select_174([h, g, e, f, k, a, b, i, j, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_173([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_173([a, b, c, d, e, f, i, h, g, j, k, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_183([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_31([a, b, c, d, f], is_dual)
    } else {
        select_31([a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_182([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_183([a, c, d, e, f, h], is_dual)
    } else {
        select_87([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_185([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_22([b, c, d, e, f], is_dual)
    } else {
        select_19([a, g], is_dual)
    }
}
/// n = 10, i = 1
fn select_186([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_22([e, f, g, h, i], is_dual)
    } else {
        select_22([a, b, c, d, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_184([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_185([i, e, f, g, h, a, j], is_dual)
    } else {
        select_186([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_181([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_182([a, i, e, f, g, h, b, j, k], is_dual)
    } else {
        select_184([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_189([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_50([b, c, a, d, e], is_dual)
    } else {
        select_27([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_188([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([a, c, d, e, f], is_dual)
    } else {
        select_189([a, b, f, g, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_191([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_26([a, c, b, d, e, f], is_dual)
    } else {
        select_46([a, b, e, f, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_191([b, g, c, a, f, h], is_dual)
    } else {
        select_81([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_188([a, f, d, e, b, g, h], is_dual)
    } else {
        select_190([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_181([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_187([a, b, j, d, e, c, i, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_194([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_56([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_139([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_193([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_194([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_194([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_196([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_79([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_81([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_195([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_196([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_196([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_193([a, b, h, e, f, g, c, i, j], is_dual)
    } else {
        select_195([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_179([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_180([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_192([a, b, c, i, f, g, h, d, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_171([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_172([d, j, h, i, e, f, a, b, c, l, k, m], is_dual)
    } else {
        select_179([a, b, c, d, g, e, f, k, j, m, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_204([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, c, d, g], is_dual)
    } else {
        select_27([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_21([a, c, d, e, h], is_dual)
    } else {
        select_204([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_202([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_16([a, c, d, e, g, h], is_dual)
    } else {
        select_203([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_201([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_202([a, b, d, c, i, g, h, j, k], is_dual)
    } else {
        select_143([a, d, e, f, j, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_206([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_16([a, c, d, e, f, g], is_dual)
    } else {
        select_19([b, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_205([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_143([a, c, d, e, h, g, i], is_dual)
    } else {
        select_206([a, b, c, i, f, g, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_201([b, d, f, e, a, g, h, i, j, k, l], is_dual)
    } else {
        select_205([a, d, c, b, i, h, g, k, l, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_199([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_104([a, h, b, c, d, f, i, j, g, k, l], is_dual)
    } else {
        select_200([b, c, d, a, f, e, i, g, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_198([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_199([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_199([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_78([b, f, c, d, e, g, h], is_dual)
    } else {
        select_81([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_210([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_196([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_211([a, h, d, e, f, g, b, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_209([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_210([a, c, e, b, f, h, g, i, j, k], is_dual)
    } else {
        select_210([a, b, d, c, f, i, g, h, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_208([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_209([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_209([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_214([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_183([a, c, d, e, f, g], is_dual)
    } else {
        select_30([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_215([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_87([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_143([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_214([a, c, d, e, f, g, h], is_dual)
    } else {
        select_215([a, b, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_216([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_76([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_91([a, b, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_212([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_213([a, b, c, e, f, g, h, j, k], is_dual)
    } else {
        select_216([a, b, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_207([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_208([a, b, c, d, e, f, m, j, k, l, n], is_dual)
    } else {
        select_212([a, b, d, f, g, h, i, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_197([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < l) || (is_dual && c > l) {
        select_198([a, b, d, e, f, l, g, c, j, k, m, n], is_dual)
    } else {
        select_207([a, b, d, e, f, g, c, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_170([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_171([a, b, c, f, d, e, h, g, k, j, n, m, l], is_dual)
    } else {
        select_197([a, b, c, f, g, h, i, d, e, j, k, l, m, n], is_dual)
    }
}
/// n = 13, i = 2
fn select_218([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_67([a, b, c, d, g, e, i, j, h, l, m, k], is_dual)
    } else {
        select_67([a, b, c, d, g, f, h, j, i, k, m, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_217([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_218([a, b, c, f, d, g, h, e, i, j, k, l, m], is_dual)
    } else {
        select_218([a, b, c, f, e, g, h, d, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < d) || (is_dual && m > d) {
        select_170([a, b, c, e, f, d, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_217([a, b, c, e, f, g, j, i, m, k, l, d, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_126([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_127([j, i, a, f, g, h, b, c, d, e, l, k, m, n], is_dual)
    } else {
        select_169([b, c, d, e, g, h, i, f, j, k, l, m, a, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_224([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_70([a, c, h, d, f, b, g, i], is_dual)
    } else {
        select_86([a, c, d, b, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_227([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_16([d, a, b, g, e, f], is_dual)
    } else {
        select_16([d, a, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_228([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_26([a, f, b, e, d, g], is_dual)
    } else {
        select_21([b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_226([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_227([a, b, c, h, e, f, g], is_dual)
    } else {
        select_228([d, a, b, h, e, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_225([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_226([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_226([a, b, c, e, f, g, h, d, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_223([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_224([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_225([c, e, f, a, b, d, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_230([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_55([f, e, c, d, h, g, i, j], is_dual)
    } else {
        select_55([f, e, a, b, i, g, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_229([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_230([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_230([a, b, c, d, g, f, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_222([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_223([a, b, c, d, e, i, h, k, j], is_dual)
    } else {
        select_229([d, h, f, g, a, b, c, j, i, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_221([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < d) || (is_dual && m > d) {
        select_171([a, b, c, e, d, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_222([a, b, c, e, g, f, k, j, m, l, d], is_dual)
    }
}
/// n = 13, i = 2
fn select_234([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_152([a, c, d, g, h, b, i, j, k, l], is_dual)
    } else {
        select_121([a, b, e, f, c, d, k, i, j, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_233([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_234([a, e, b, c, g, h, d, i, j, k, m, n, l], is_dual)
    } else {
        select_118([a, e, d, g, h, f, b, c, j, k, i, l, n, m], is_dual)
    }
}
/// n = 12, i = 2
fn select_237([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_91([f, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_91([e, a, b, f, i, j, g, h, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_16([a, b, e, g, f, i], is_dual)
    } else {
        select_18([c, d, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_242([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([b, c, d, e], is_dual)
    } else {
        select_19([a, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_241([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_203([a, c, b, h, f, g, i, j], is_dual)
    } else {
        select_242([c, d, e, i, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_239([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_240([a, b, e, f, d, g, i, j, k], is_dual)
    } else {
        select_241([b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_238([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_239([a, b, d, h, e, f, g, i, j, k, l], is_dual)
    } else {
        select_16([a, b, c, l, g, h], is_dual)
    }
}
/// n = 13, i = 2
fn select_236([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_237([d, g, e, f, a, b, h, k, i, j, l, m], is_dual)
    } else {
        select_238([b, e, f, c, a, h, i, j, g, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_244([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_84([b, e, f, g, a, h, i, j, k, l], is_dual)
    } else {
        select_91([a, c, d, b, j, k, h, i, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_244([c, b, f, g, e, a, h, j, k, i, l, n, m], is_dual)
    } else {
        select_244([c, a, f, g, d, b, i, j, k, h, m, n, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < l) || (is_dual && a > l) {
        select_236([b, c, h, e, f, g, l, i, j, k, a, m, n], is_dual)
    } else {
        select_243([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_232([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_233([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_235([a, d, e, c, f, g, h, i, j, k, l, b, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_231([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_232([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_232([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_220([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_221([a, b, c, d, g, e, i, f, k, l, m, n, j], is_dual)
    } else {
        select_231([a, b, c, d, g, f, i, e, h, l, j, k, n, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_252([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_16([b, a, c, d, e, f], is_dual)
    } else {
        select_57([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_251([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_252([a, c, d, e, b, f, g], is_dual)
    } else {
        select_252([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_254([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_28([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_253([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_254([a, c, d, b, e, f], is_dual)
    } else {
        select_254([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_250([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_251([a, b, c, d, f, h, g], is_dual)
    } else {
        select_253([a, b, e, g, f, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_249([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_213([a, b, c, e, f, g, i, j, k], is_dual)
    } else {
        select_250([a, b, c, j, d, h, k, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_258([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_22([d, e, f, g, h], is_dual)
    } else {
        select_32([a, b, c, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_257([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_258([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_185([h, d, e, f, g, c, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_261([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_26([a, d, b, c, e, f], is_dual)
    } else {
        select_26([a, d, c, b, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_260([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([c, d, e, f, g], is_dual)
    } else {
        select_261([c, a, b, g, h, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_259([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_260([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_260([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_256([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_257([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_259([b, c, a, d, j, i, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_255([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_256([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_256([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_248([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_249([a, b, e, d, f, g, h, i, c, j, k], is_dual)
    } else {
        select_255([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_194([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_211([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_264([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_265([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_43([a, c, d, h, g, b, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_263([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_264([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_264([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_268([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_144([a, b, c, d, f, h], is_dual)
    } else {
        select_33([a, d, e, b, c, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_267([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_268([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_268([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_272([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_26([a, b, c, e, d, f], is_dual)
    } else {
        select_27([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_271([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_272([a, b, f, d, g, e], is_dual)
    } else {
        select_228([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_270([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_271([a, b, c, d, e, f, g], is_dual)
    } else {
        select_271([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_269([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_270([b, c, g, e, f, a, h], is_dual)
    } else {
        select_63([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_266([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_267([d, a, b, c, e, f, g, h], is_dual)
    } else {
        select_269([d, a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_262([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_263([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_266([d, e, h, a, b, g, i, c], is_dual)
    }
}
/// n = 11, i = 2
fn select_247([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_248([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    } else {
        select_262([a, b, c, e, f, j, i, d, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_277([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_93([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_44([a, f, b, g, h, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_276([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_277([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_277([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_275([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_192([a, b, c, e, d, f, g, h, i, j], is_dual)
    } else {
        select_276([a, b, c, i, e, d, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_274([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_41([a, b, c, d, e, i, j, h], is_dual)
    } else {
        select_275([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_281([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_227([a, d, e, b, c, f, g], is_dual)
    } else {
        select_93([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_280([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_281([a, b, c, d, j, h, k, i], is_dual)
    } else {
        select_195([a, b, c, h, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_279([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_280([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_280([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_278([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_279([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_279([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_273([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_274([a, b, c, d, e, f, k, i, j, l], is_dual)
    } else {
        select_278([a, b, c, d, e, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_246([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_247([a, b, c, d, e, f, g, j, i, l, k], is_dual)
    } else {
        select_273([a, b, c, d, e, h, f, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_245([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_246([a, b, c, d, e, f, k, g, i, j, m, l], is_dual)
    } else {
        select_247([a, b, c, d, e, f, h, l, i, k, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_219([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < e) || (is_dual && m > e) {
        select_220([a, b, c, d, f, e, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_245([a, b, c, d, f, g, k, i, j, m, l, e, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_125([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_126([a, b, c, d, e, g, f, h, i, j, k, m, l, n], is_dual)
    } else {
        select_219([a, b, c, d, e, g, i, h, j, k, l, m, f, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_124([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_125([a, b, c, d, e, f, g, i, j, k, h, l, m, n], is_dual)
    } else {
        select_125([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_123([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_124([a, b, c, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    } else {
        select_124([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 12, i = 2
fn select_284([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_36([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_36([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_292([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_30([e, f, c, d, g, h], is_dual)
    } else {
        select_30([e, f, a, b, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_178([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_17([c, g, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_293([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_294([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_176([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_291([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_292([d, g, e, f, a, c, h, i], is_dual)
    } else {
        select_293([a, b, c, h, e, f, g, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_295([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_84([g, a, b, c, d, e, i, h, j, k], is_dual)
    } else {
        select_175([g, f, k, a, b, c, h, i, j, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_290([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_291([a, b, c, d, j, k, h, i, m, l], is_dual)
    } else {
        select_295([e, c, i, f, g, b, a, j, h, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_289([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_290([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_290([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_288([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_289([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    } else {
        select_289([a, b, c, e, d, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_287([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_288([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_288([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_301([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_194([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_196([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_300([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_301([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_277([a, c, i, d, g, b, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_299([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_264([a, b, c, d, e, h, g, j, i], is_dual)
    } else {
        select_300([a, b, c, f, d, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_298([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_299([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_299([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_303([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_265([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_265([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_302([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_303([a, b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_42([a, b, c, e, h, d, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_297([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_298([a, b, c, d, e, i, f, h, k, j], is_dual)
    } else {
        select_302([a, b, c, d, e, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_306([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_104([c, d, a, e, b, f, h, i, j, g, k], is_dual)
    } else {
        select_104([c, d, b, e, a, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_305([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_306([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_306([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_304([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_305([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_305([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_296([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_297([a, b, c, d, g, f, h, i, j, e, k], is_dual)
    } else {
        select_304([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_286([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_287([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_296([a, b, c, d, e, f, g, l, j, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_285([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_286([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_286([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_283([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_284([a, b, c, d, e, f, g, i, l, k, h, m], is_dual)
    } else {
        select_285([a, b, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_282([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_283([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_283([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_122([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_123([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_282([a, b, c, d, e, f, g, h, i, j, m, n, k], is_dual)
    }
}
/// n = 15, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < l) || (is_dual && o > l) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_122([a, b, c, d, e, f, g, h, i, j, k, n, o, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_318([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_159([a, b, c, d, i, j, l, k, n], is_dual)
    } else {
        select_120([e, f, g, h, a, b, k, i, j, l, m], is_dual)
    }
}
/// n = 15, i = 2
fn select_317([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_158([b, c, a, g, h, d, j, k, i, l, n, o, m], is_dual)
    } else {
        select_318([b, c, g, h, e, f, a, i, j, k, l, m, o, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_316([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_317([a, c, d, b, e, f, g, h, j, k, l, m, i, n, o], is_dual)
    } else {
        select_317([b, c, d, a, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_315([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_316([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    } else {
        select_316([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_315([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    } else {
        select_315([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 12, i = 2
fn select_324([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_15([a, b, i, d, e, f, g, h, j, k, l], is_dual)
    } else {
        select_134([c, j, d, e, f, g, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_323([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_324([a, d, b, e, f, g, h, i, c, j, k, l], is_dual)
    } else {
        select_324([a, d, c, e, f, g, h, i, b, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_326([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_260([a, b, c, d, g, h, f, i], is_dual)
    } else {
        select_260([a, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_328([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_56([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_81([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_329([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_178([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_28([a, h, b], is_dual)
    }
}
/// n = 10, i = 2
fn select_327([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_328([a, b, c, e, f, g, i, h, j], is_dual)
    } else {
        select_329([a, b, c, d, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_325([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_326([a, b, c, d, h, g, k, i, j], is_dual)
    } else {
        select_327([a, b, f, e, c, g, h, j, i, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_322([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_323([a, b, c, e, f, g, h, k, j, l, m, n], is_dual)
    } else {
        select_325([b, c, a, e, d, m, j, i, n, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_330([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_104([a, b, c, g, f, e, i, h, k, l, j], is_dual)
    } else {
        select_325([a, b, c, f, h, d, i, j, k, l, g], is_dual)
    }
}
/// n = 14, i = 2
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_322([b, c, d, e, f, a, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_330([c, d, b, l, e, f, a, i, j, k, n, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_331([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_151([d, j, h, i, b, c, e, f, a, m, l, k, n], is_dual)
    } else {
        select_168([a, e, f, d, h, i, g, b, c, j, l, k, n, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_321([a, d, e, f, c, g, h, i, j, k, b, l, m, n], is_dual)
    } else {
        select_331([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_319([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_320([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    } else {
        select_320([a, b, d, e, f, g, h, i, j, c, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_314([a, b, c, d, f, g, e, h, i, j, k, l, n, m, o], is_dual)
    } else {
        select_319([a, b, c, d, e, f, g, l, i, j, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_312([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_313([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    } else {
        select_313([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_311([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_312([f, g, h, i, j, k, b, c, a, d, e, l, m, o, n], is_dual)
    } else {
        select_312([f, g, h, i, j, k, d, e, a, b, c, l, m, n, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_341([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_191([a, b, c, e, g, h], is_dual)
    } else {
        select_94([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_340([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_294([b, c, d, a, e, f, h, g, i], is_dual)
    } else {
        select_341([b, g, c, d, a, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_339([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_91([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_340([a, b, j, c, d, h, i, g, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_338([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_339([a, b, c, d, k, l, h, i, j, n, m], is_dual)
    } else {
        select_295([e, f, g, i, j, b, a, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_337([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_338([a, b, e, f, c, g, h, i, j, k, d, l, m, n], is_dual)
    } else {
        select_338([a, b, e, f, d, g, h, i, j, k, c, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_345([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_45([a, f, c, b, e, g], is_dual)
    } else {
        select_16([a, c, b, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_344([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_345([a, e, b, d, g, f, h], is_dual)
    } else {
        select_48([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_346([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_183([a, c, d, e, g, h], is_dual)
    } else {
        select_45([a, h, b, i, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_343([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_344([a, b, i, c, j, g, f, h], is_dual)
    } else {
        select_346([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_342([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_61([a, k, b, e, f, g, h, i, l], is_dual)
    } else {
        select_343([a, b, e, c, d, g, h, l, j, k], is_dual)
    }
}
/// n = 14, i = 2
fn select_336([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < b) || (is_dual && n > b) {
        select_337([a, c, b, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_342([a, c, d, l, e, f, i, j, k, n, b, m], is_dual)
    }
}
/// n = 12, i = 2
fn select_350([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_84([c, e, f, g, a, b, h, i, j, k], is_dual)
    } else {
        select_29([a, b, d, c, i, j, h, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_351([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_162([a, b, c, d, f, g, h, i, k, l, m], is_dual)
    } else {
        select_110([a, b, d, e, f, g, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_350([b, c, a, e, d, j, k, i, h, n, m, l], is_dual)
    } else {
        select_351([b, c, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_348([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_349([a, c, d, b, e, f, g, i, j, k, l, h, m, n], is_dual)
    } else {
        select_349([b, c, d, a, e, f, g, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_347([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_348([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    } else {
        select_348([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_335([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_336([a, b, c, d, g, h, e, f, i, k, l, m, n, j], is_dual)
    } else {
        select_347([a, b, c, e, f, d, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_334([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_335([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_335([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_333([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_334([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    } else {
        select_334([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_332([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_333([a, b, c, d, e, f, g, h, i, l, k, o, m, n], is_dual)
    } else {
        select_333([a, b, c, d, e, f, g, h, j, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_310([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_311([k, a, h, i, j, b, c, d, e, f, g, m, o, l, n], is_dual)
    } else {
        select_332([b, c, d, e, f, g, i, j, h, k, l, m, n, a, o], is_dual)
    }
}
/// n = 14, i = 2
fn select_357([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_331([a, d, e, b, f, g, c, h, i, k, l, m, j, n], is_dual)
    } else {
        select_331([a, d, e, c, f, g, b, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_119([g, i, a, c, d, h, k, m, j, l, o], is_dual)
    } else {
        select_119([h, i, b, e, f, g, j, l, k, m, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_359([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_168([a, b, c, e, d, j, g, h, i, k, n, m, l, o], is_dual)
    } else {
        select_360([f, g, b, c, h, i, d, e, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_359([a, d, e, b, f, c, g, h, i, k, l, m, n, j, o], is_dual)
    } else {
        select_359([a, d, e, c, f, b, g, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_356([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_357([i, c, d, e, f, g, h, b, j, k, l, m, o, n], is_dual)
    } else {
        select_358([i, c, d, e, f, b, a, g, h, k, l, j, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_355([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_356([a, b, c, h, d, e, f, g, i, k, l, j, m, n, o], is_dual)
    } else {
        select_356([a, b, c, j, d, e, f, g, i, k, l, h, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_354([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_355([a, b, g, c, d, e, f, h, i, j, l, k, m, n, o], is_dual)
    } else {
        select_355([a, b, k, c, d, e, f, h, i, j, l, g, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_354([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m], is_dual)
    } else {
        select_354([b, a, c, d, e, f, g, h, i, j, k, m, n, o, l], is_dual)
    }
}
/// n = 15, i = 2
fn select_352([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_353([a, f, b, c, d, e, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_353([a, l, b, c, d, e, g, h, i, j, k, m, f, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < n) || (is_dual && h > n) {
        select_310([a, b, c, d, e, f, g, i, k, l, j, m, n, o, h], is_dual)
    } else {
        select_352([i, h, j, k, l, a, b, c, d, e, f, g, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_308([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_309([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    } else {
        select_309([a, b, c, d, e, f, g, h, j, k, l, m, i, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_307([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_308([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o], is_dual)
    } else {
        select_308([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < o) || (is_dual && m > o) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m], is_dual)
    } else {
        select_307([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
