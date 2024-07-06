/// n = 1, i = 0
fn select_24([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_23([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_24([a], is_dual)
    } else {
        select_24([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_22([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_23([a, b], is_dual)
    } else {
        select_23([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_21([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_22([a, b, c], is_dual)
    } else {
        select_22([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_21([d, e, f, g], is_dual)
    } else {
        select_21([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_19([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_20([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_20([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 11, i = 1
fn select_18([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_19([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_19([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_17([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_18([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_18([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 15, i = 2
fn select_16([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_17([g, i, a, c, d, h, k, m, j, l, o], is_dual)
    } else {
        select_17([h, i, b, e, f, g, j, l, k, m, n], is_dual)
    }
}
/// n = 7, i = 1
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_21([c, d, e, f], is_dual)
    } else {
        select_22([a, b, g], is_dual)
    }
}
/// n = 9, i = 1
fn select_28([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_29([a, h, d, e, f, g, i], is_dual)
    } else {
        select_20([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_27([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_28([h, c, d, e, f, g, a, i, j], is_dual)
    } else {
        select_19([e, f, g, a, b, c, d, h, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_26([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_27([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_27([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_25([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_17([a, d, g, h, i, b, c, k, j, l, m], is_dual)
    } else {
        select_26([a, b, c, e, f, d, j, l, k, n], is_dual)
    }
}
/// n = 15, i = 2
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < l) || (is_dual && n > l) {
        select_16([f, g, b, c, h, i, d, e, a, j, k, l, m, n, o], is_dual)
    } else {
        select_25([a, b, c, e, d, j, g, h, i, k, n, m, l, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_15([a, d, e, b, f, c, g, h, i, k, l, m, n, j, o], is_dual)
    } else {
        select_15([a, d, e, c, f, b, g, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 2
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_14([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    } else {
        select_14([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < p) || (is_dual && o > p) {
        select_13([g, i, j, a, c, d, h, b, e, f, k, l, m, n, p], is_dual)
    } else {
        select_13([h, i, j, b, e, f, g, a, c, d, l, k, n, m, o], is_dual)
    }
}
/// n = 4, i = 1
fn select_38([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_22([a, b, c], is_dual)
    } else {
        select_24([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_37([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_21([c, d, e, f], is_dual)
    } else {
        select_38([a, b, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_22([c, d, e], is_dual)
    } else {
        select_22([a, b, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_42([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([a, d], is_dual)
    } else {
        select_23([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_41([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_42([a, c, b, d], is_dual)
    } else {
        select_42([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_39([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_40([a, b, c, d, e, f], is_dual)
    } else {
        select_41([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_37([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_39([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_40([b, c, d, e, g, f], is_dual)
    } else {
        select_38([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_43([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_29([b, c, d, e, i, g, h], is_dual)
    } else {
        select_44([f, a, h, b, c, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_35([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_36([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_43([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_35([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_35([a, b, e, d, f, g, c, h, i, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_48([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([b, c, d, e], is_dual)
    } else {
        select_23([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_20([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_48([g, d, e, f, a, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([a, b, c, d, e, f, g], is_dual)
    } else {
        select_48([f, c, d, e, a, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_46([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_47([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_49([a, g, d, e, f, b, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_53([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_42([a, c, d, g], is_dual)
    } else {
        select_42([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_53([a, d, b, g, e, f, h], is_dual)
    } else {
        select_53([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_56([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_23([a, b], is_dual)
    } else {
        select_24([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_42([b, c, e, d], is_dual)
    } else {
        select_56([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_54([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_55([e, a, c, f, d, g], is_dual)
    } else {
        select_55([d, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_51([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_52([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_54([c, e, d, g, a, f, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_59([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_42([a, b, c, d], is_dual)
    } else {
        select_56([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_58([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_55([a, b, c, e, d, f], is_dual)
    } else {
        select_59([d, c, a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_61([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_42([a, b, e, f], is_dual)
    } else {
        select_42([a, c, d, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_62([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_42([a, b, c, d], is_dual)
    } else {
        select_56([d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_61([d, f, g, a, e, b], !is_dual)
    } else {
        select_62([e, f, d, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_57([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_58([a, c, d, g, f, i], is_dual)
    } else {
        select_60([h, i, g, a, e, c, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_51([b, a, d, e, c, f, g, h, i], is_dual)
    } else {
        select_57([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_45([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_46([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_50([a, b, c, i, d, h, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_34([c, a, d, e, g, b, h, i, j, k], is_dual)
    } else {
        select_45([a, b, c, f, d, e, i, h, k, j, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_67([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([b, c, d], is_dual)
    } else {
        select_23([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_66([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_40([a, b, c, d, e, f], is_dual)
    } else {
        select_67([e, c, d, a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_47([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_66([e, h, a, b, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_40([a, f, c, d, e, g], is_dual)
    } else {
        select_40([b, e, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_70([b, c, d, e, g, f, h], is_dual)
    } else {
        select_55([a, b, c, f, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_68([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_69([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_36([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_64([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_65([d, g, e, f, a, c, h, i], is_dual)
    } else {
        select_68([a, b, c, h, e, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_64([a, b, e, g, c, d, i, h, j, k], is_dual)
    } else {
        select_34([a, b, c, d, f, e, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_32([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_33([b, a, c, e, f, d, g, i, j, h, k, l], is_dual)
    } else {
        select_63([c, b, e, f, d, g, a, h, j, k, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_31([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_32([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    } else {
        select_32([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_55([a, b, d, g, f, i], is_dual)
    } else {
        select_42([c, d, e, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_76([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_70([c, d, e, f, j, g, i], is_dual)
    } else {
        select_77([a, b, d, c, g, h, k, j, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_79([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_55([a, b, c, f, e, g], is_dual)
    } else {
        select_55([a, b, d, f, e, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_80([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_55([a, b, c, e, d, f], is_dual)
    } else {
        select_42([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_78([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_79([a, c, b, d, e, f, g], is_dual)
    } else {
        select_80([a, b, d, e, g, f], is_dual)
    }
}
/// n = 13, i = 3
fn select_75([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_76([b, c, g, d, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_78([a, b, l, c, g, i, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_82([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_52([b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_55([a, i, b, g, e, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_85([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([b, c, d], is_dual)
    } else {
        select_56([a, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_21([c, d, e, g], is_dual)
    } else {
        select_85([a, b, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_83([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_29([b, h, c, d, e, g, i], is_dual)
    } else {
        select_84([a, g, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_81([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_82([a, b, i, c, f, h, g, j, k, l], is_dual)
    } else {
        select_83([a, c, d, e, g, f, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_74([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_75([b, c, a, d, e, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_81([b, h, d, e, f, g, a, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_55([f, c, g, e, d, a], !is_dual)
    } else {
        select_62([d, g, c, e, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_89([g, i, a, h, e, b, c], !is_dual)
    } else {
        select_80([a, b, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_87([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_88([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_88([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_55([a, e, c, f, d, h], is_dual)
    } else {
        select_61([b, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_90([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_84([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_91([b, g, c, f, a, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_86([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_87([a, b, c, d, j, g, h, l, k], is_dual)
    } else {
        select_90([a, c, d, e, f, h, i, k, j, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_73([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_74([a, b, c, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_86([a, b, c, d, f, g, h, i, k, m, l, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_96([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_62([d, f, e, a, b], !is_dual)
    } else {
        select_62([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_95([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_66([b, c, a, d, e, f], is_dual)
    } else {
        select_96([b, c, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_95([a, c, e, b, h, f, g], is_dual)
    } else {
        select_95([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_100([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_22([b, c, e], is_dual)
    } else {
        select_42([a, f, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_99([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_100([e, c, d, f, h, g, j], is_dual)
    } else {
        select_100([e, a, b, f, i, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_29([c, d, e, f, i, j, h], is_dual)
    } else {
        select_99([b, h, c, d, a, g, k, i, j, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_102([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_67([a, c, d, b, e], is_dual)
    } else {
        select_67([b, c, d, a, e], is_dual)
    }
}
/// n = 12, i = 3
fn select_101([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_29([d, e, f, g, i, h, k], is_dual)
    } else {
        select_102([a, b, c, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_98([a, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_101([a, h, b, f, g, d, e, j, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_94([h, a, b, l, d, i, j, n], is_dual)
    } else {
        select_97([b, d, c, e, i, f, g, h, j, k, m, l, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_106([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_67([a, b, c, d, e], is_dual)
    } else {
        select_56([e, a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_29([b, c, a, d, e, f, g], is_dual)
    } else {
        select_106([g, b, c, a, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_104([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_47([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_105([f, a, i, b, c, g, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_109([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_100([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([e, b, c, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_110([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_100([a, c, d, b, e, f, g], is_dual)
    } else {
        select_100([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_109([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_110([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_100([a, b, c, e, f, g, h], is_dual)
    } else {
        select_22([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_113([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_21([c, d, e, f], is_dual)
    } else {
        select_41([a, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_111([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_112([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_113([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_107([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_108([a, b, c, h, f, g, i, j], is_dual)
    } else {
        select_111([a, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_104([c, d, h, e, f, i, g, k, j, l], is_dual)
    } else {
        select_107([g, a, b, d, j, h, i, l, m, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_92([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_93([b, c, d, a, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_103([b, j, d, e, f, g, h, i, a, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_72([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_73([b, c, d, f, a, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_92([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_20([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_37([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_29([b, c, d, e, f, g, h], is_dual)
    } else {
        select_39([a, h, b, c, f, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_117([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_118([f, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_119([f, a, j, b, c, h, g, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_117([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_117([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_122([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_29([a, b, c, d, e, f, g], is_dual)
    } else {
        select_67([g, a, b, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_121([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_122([c, f, d, e, a, g, h], is_dual)
    } else {
        select_105([a, b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_120([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_27([f, a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_121([f, j, a, b, c, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_116([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_120([h, a, b, c, d, f, g, e, i, j, k], is_dual)
    }
}
/// n = 5, i = 0
fn select_127([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_21([a, b, c, d], is_dual)
    } else {
        select_21([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_126([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_66([a, b, c, j, h, k], is_dual)
    } else {
        select_127([d, e, f, g, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_129([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_67([a, b, f, d, e], is_dual)
    } else {
        select_67([a, c, e, d, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_52([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_129([c, d, e, g, f, a], is_dual)
    }
}
/// n = 11, i = 2
fn select_125([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_126([b, c, d, a, e, f, g, h, j, i, k], is_dual)
    } else {
        select_128([b, i, d, c, a, h, k, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_132([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_67([b, a, c, d, e], is_dual)
    } else {
        select_42([b, e, d, a], is_dual)
    }
}
/// n = 10, i = 2
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_127([c, d, e, f, h], is_dual)
    } else {
        select_132([a, b, i, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_55([g, h, f, d, a, b], !is_dual)
    } else {
        select_41([a, f, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_134([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_134([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_130([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_131([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_133([b, i, c, a, j, g, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_124([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_125([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_130([a, b, c, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_137([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_127([b, c, d, e, g], is_dual)
    } else {
        select_62([a, h, f, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_139([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_41([a, b, c, d], is_dual)
    } else {
        select_56([d, b, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_127([c, d, e, f, g], is_dual)
    } else {
        select_139([a, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_136([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_137([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_138([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 3, i = 1
fn select_143([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_56([a, b, c], is_dual)
    } else {
        select_56([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_142([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_143([a, d, e], is_dual)
    } else {
        select_42([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_141([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_142([e, f, a, c, d], !is_dual)
    } else {
        select_142([f, e, d, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_85([a, c, d, e, f, h], is_dual)
    } else {
        select_38([b, c, d, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_146([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_42([a, d, c, f], is_dual)
    } else {
        select_42([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_144([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_145([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_146([e, h, f, g, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_140([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_141([a, h, e, g, f, b], !is_dual)
    } else {
        select_144([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_135([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_136([a, b, d, e, f, h, g, i, k, j], is_dual)
    } else {
        select_140([a, b, c, i, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_123([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_124([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_135([a, c, i, e, f, g, h, b, j, l, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_115([f, j, g, h, e, a, b, i, l, k, m], is_dual)
    } else {
        select_123([a, b, c, d, e, f, k, i, j, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_71([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_72([a, d, b, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_114([a, d, c, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_30([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_31([a, b, d, c, e, f, g, n, k, p, l, o], is_dual)
    } else {
        select_71([a, b, c, d, e, h, l, i, j, k, m, o, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < k) || (is_dual && o > k) {
        select_12([c, h, f, g, i, j, d, e, a, b, k, l, m, n, o, p], is_dual)
    } else {
        select_30([a, b, e, c, h, i, j, d, f, g, l, n, m, o, p, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_155([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_21([a, b, c, d], is_dual)
    } else {
        select_24([e], is_dual)
    }
}
/// n = 6, i = 1
fn select_154([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_155([a, c, d, e, f], is_dual)
    } else {
        select_102([a, b, c, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_139([a, b, c, i, j], is_dual)
    } else {
        select_37([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_153([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_154([a, b, c, d, j, h], is_dual)
    } else {
        select_156([a, b, h, e, f, g, k, i, j, l], is_dual)
    }
}
/// n = 7, i = 1
fn select_158([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_70([a, c, d, e, b, f, g], is_dual)
    } else {
        select_70([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_158([a, b, i, c, d, j, h], is_dual)
    } else {
        select_21([e, f, g, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_152([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_153([a, b, c, d, e, f, g, i, j, k, h, l], is_dual)
    } else {
        select_157([c, d, a, b, e, f, g, h, j, k, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_160([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_28([d, e, i, a, b, c, g, h, j], is_dual)
    } else {
        select_28([d, f, h, a, b, c, g, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_163([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_55([a, c, b, e, f, g], is_dual)
    } else {
        select_38([a, c, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_70([d, e, b, c, f, g, h], is_dual)
    } else {
        select_163([a, e, b, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_162([e, a, h, b, c, f, g, i, j], is_dual)
    } else {
        select_70([b, c, d, i, f, g, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_159([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_160([a, b, j, c, d, e, g, h, i, k], is_dual)
    } else {
        select_161([k, c, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_151([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_152([a, b, c, d, i, j, m, k, l, o, p, n], is_dual)
    } else {
        select_159([g, h, e, f, l, a, i, j, n, k, m, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_21([c, d, e, g], is_dual)
    } else {
        select_67([a, b, h, f, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_166([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_20([b, c, h, d, e, f, i, j], is_dual)
    } else {
        select_167([a, i, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_169([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_55([a, d, c, e, f, g], is_dual)
    } else {
        select_41([a, b, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_168([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_102([a, b, d, g, f], is_dual)
    } else {
        select_169([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_166([c, a, b, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_168([a, b, c, j, g, k, i, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_29([a, g, c, d, e, f, h], is_dual)
    } else {
        select_29([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_170([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_171([a, b, c, d, j, g, h, i], is_dual)
    } else {
        select_171([a, b, e, f, i, g, h, j], is_dual)
    }
}
/// n = 16, i = 3
fn select_164([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_165([a, b, c, i, j, m, h, k, l, o, p], is_dual)
    } else {
        select_170([d, e, f, g, k, l, i, j, m, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_150([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_151([a, b, c, d, e, f, g, h, j, k, i, l, m, n, o, p], is_dual)
    } else {
        select_164([c, d, a, e, f, g, h, i, j, k, l, b, m, n, o, p], is_dual)
    }
}
/// n = 12, i = 3
fn select_176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_70([d, e, f, i, g, h, j], is_dual)
    } else {
        select_102([a, b, c, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_175([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_171([d, e, f, g, l, h, i, j], is_dual)
    } else {
        select_176([a, b, c, d, e, j, h, i, k, l, m, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_179([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_55([a, b, c, d, e, f], is_dual)
    } else {
        select_56([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_178([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_169([a, b, c, d, e, g, f], is_dual)
    } else {
        select_179([a, b, c, f, g, e], is_dual)
    }
}
/// n = 12, i = 3
fn select_177([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_178([b, c, j, d, a, l, i], is_dual)
    } else {
        select_176([b, c, d, e, f, a, g, h, i, k, j, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_174([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_175([b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_177([a, b, c, d, e, f, i, j, l, n, m, o], is_dual)
    }
}
/// n = 10, i = 2
fn select_183([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_40([c, d, e, f, h, i], is_dual)
    } else {
        select_40([a, b, e, f, g, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_100([b, c, d, e, g, f, h], is_dual)
    } else {
        select_41([a, h, b, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_183([a, b, e, f, c, d, g, h, i, j], is_dual)
    } else {
        select_184([a, b, e, f, i, h, g, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_118([g, b, c, d, e, j, k, h, i, l], is_dual)
    } else {
        select_182([g, f, a, i, b, c, k, h, j, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_180([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_181([c, d, i, f, g, b, a, h, l, k, j, m], is_dual)
    } else {
        select_181([c, e, h, f, g, b, a, i, l, k, j, m], is_dual)
    }
}
/// n = 16, i = 3
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_174([a, b, c, i, e, f, g, h, j, k, m, l, n, o, p], is_dual)
    } else {
        select_180([a, l, d, e, f, g, h, j, k, n, i, m, o], is_dual)
    }
}
/// n = 6, i = 2
fn select_189([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_55([a, d, b, c, e, f], is_dual)
    } else {
        select_55([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_188([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_189([f, a, b, c, e, g], is_dual)
    } else {
        select_189([e, a, b, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_158([b, c, d, a, e, f, g], is_dual)
    } else {
        select_188([b, c, d, g, f, a, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_191([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_70([a, g, d, e, f, i, h], is_dual)
    } else {
        select_66([f, h, b, c, j, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_193([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_67([b, c, d, g, f], is_dual)
    } else {
        select_62([a, b, e, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_48([c, d, e, i, f, h], is_dual)
    } else {
        select_193([c, h, a, b, f, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_191([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_192([c, d, b, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_186([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_187([b, c, d, g, a, k, j, i], is_dual)
    } else {
        select_190([b, a, e, f, c, d, h, i, k, l, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_196([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_142([a, c, f, e, h], is_dual)
    } else {
        select_89([b, c, f, e, d, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_198([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([a, h, b, f, i, j], is_dual)
    } else {
        select_21([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_112([b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_198([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_195([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_196([b, c, h, f, a, g, i, j], is_dual)
    } else {
        select_197([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_194([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_195([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_146([b, c, i, f, a, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_186([b, a, d, e, f, g, c, i, h, k, l, j], is_dual)
    } else {
        select_194([a, b, h, f, g, c, j, i, k, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < i) || (is_dual && o > i) {
        select_173([b, c, d, e, a, f, g, h, j, i, k, l, m, n, o, p], is_dual)
    } else {
        select_185([b, a, j, c, d, k, m, l, o, n, i, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_149([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_150([a, b, d, e, c, g, h, i, j, f, k, l, m, n, o, p], is_dual)
    } else {
        select_172([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o, p], is_dual)
    }
}
/// n = 6, i = 2
fn select_205([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_62([e, f, a, b, c], !is_dual)
    } else {
        select_38([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_204([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_155([b, d, e, f, g], is_dual)
    } else {
        select_205([a, b, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_204([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_66([b, f, d, e, c, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_207([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_85([a, c, d, b, e, f], is_dual)
    } else {
        select_85([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_206([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_158([b, c, a, d, e, f, g], is_dual)
    } else {
        select_207([a, g, b, c, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_203([a, b, c, e, g, f, h, i], is_dual)
    } else {
        select_206([a, b, c, d, h, f, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_40([b, c, d, e, g, h], is_dual)
    } else {
        select_67([a, d, e, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_85([b, c, d, e, f, g], is_dual)
    } else {
        select_41([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_210([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_211([a, d, e, b, c, f, h, g, i], is_dual)
    } else {
        select_212([a, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_214([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([b, c, d, e], is_dual)
    } else {
        select_38([a, g, h, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_215([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_85([a, b, c, d, f, g], is_dual)
    } else {
        select_41([a, e, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_213([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_214([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_215([a, b, g, e, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_209([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_210([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_213([a, g, d, e, f, b, h, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_208([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_209([f, e, h, c, d, g, j, i, l, k], is_dual)
    } else {
        select_209([f, e, i, a, b, g, k, h, l, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_202([a, c, d, e, b, h, m, l, j, k], is_dual)
    } else {
        select_208([c, d, f, g, b, a, h, j, i, k, m, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_219([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_155([a, b, c, d, f], is_dual)
    } else {
        select_155([a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_218([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_219([b, c, d, e, f, g], is_dual)
    } else {
        select_96([h, f, i, a, b, g], !is_dual)
    }
}
/// n = 7, i = 3
fn select_222([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_85([a, b, c, d, e, f], is_dual)
    } else {
        select_85([a, b, c, d, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_221([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_222([a, b, c, d, f, e, g], is_dual)
    } else {
        select_80([a, b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_220([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_221([g, j, f, b, a, i, h], !is_dual)
    } else {
        select_69([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_217([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_218([a, b, c, d, j, g, h, l, k], is_dual)
    } else {
        select_220([a, b, h, e, f, g, k, i, j, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_226([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_100([e, c, d, f, i, h, k], is_dual)
    } else {
        select_100([e, a, b, f, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_225([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_226([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    } else {
        select_110([a, i, d, e, f, h, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_228([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_100([a, b, c, d, e, f, g], is_dual)
    } else {
        select_100([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_229([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_100([a, c, d, e, g, f, i], is_dual)
    } else {
        select_41([a, h, b, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_227([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_228([e, c, d, a, f, g, h], is_dual)
    } else {
        select_229([b, a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_224([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_225([b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_227([a, b, c, d, g, k, h, l, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_231([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_229([f, a, c, d, e, g, h, j, i], is_dual)
    } else {
        select_229([e, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_232([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_89([a, b, i, f, e, j, h], is_dual)
    } else {
        select_228([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_230([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_231([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_232([c, a, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_223([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_224([b, a, e, f, c, d, g, i, h, k, j, l], is_dual)
    } else {
        select_230([a, b, h, e, f, j, i, g, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_216([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_217([a, b, e, f, c, d, g, j, h, i, k, l], is_dual)
    } else {
        select_223([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_201([a, b, c, d, i, f, g, h, k, j, l, m, n], is_dual)
    } else {
        select_216([a, b, e, j, f, g, h, l, i, k, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_237([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_52([c, d, e, a, b, f, g, h], is_dual)
    } else {
        select_52([c, d, e, b, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_236([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_158([a, b, d, e, f, h, g], is_dual)
    } else {
        select_237([a, b, c, d, g, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_240([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_61([a, b, c, f, i, j], is_dual)
    } else {
        select_67([c, d, e, h, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_239([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_48([d, e, f, j, g, i], is_dual)
    } else {
        select_240([a, d, i, b, c, g, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_238([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_191([g, c, d, e, f, a, h, i, j, k], is_dual)
    } else {
        select_239([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_236([b, c, f, g, a, h, k, l, j], is_dual)
    } else {
        select_238([a, f, d, e, b, c, h, i, j, l, m, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_242([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_158([a, b, f, d, e, g, h], is_dual)
    } else {
        select_48([c, a, b, h, f, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_244([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_169([b, c, d, a, e, f, g], is_dual)
    } else {
        select_169([b, c, e, a, d, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_243([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_154([c, d, e, f, g, h], is_dual)
    } else {
        select_244([h, a, b, c, d, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_241([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_242([c, d, b, e, f, g, a, h, i], is_dual)
    } else {
        select_243([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_234([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_235([a, b, c, d, e, f, i, h, j, m, k, l, n], is_dual)
    } else {
        select_241([h, a, b, c, g, l, k, i, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_248([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_29([b, f, c, d, e, g, h], is_dual)
    } else {
        select_37([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_249([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_167([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_113([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_247([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_248([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_249([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_250([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_51([a, d, c, b, f, h, i, g, j], is_dual)
    } else {
        select_128([b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_246([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_247([g, b, a, e, f, h, i, j, k], is_dual)
    } else {
        select_250([a, b, j, c, d, g, k, i, h, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_254([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_55([a, c, b, d, e, f], is_dual)
    } else {
        select_59([a, b, e, f, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_253([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_254([b, g, c, a, f, h], is_dual)
    } else {
        select_37([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_252([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_204([b, a, g, d, e, f, h, j], is_dual)
    } else {
        select_253([a, c, f, d, e, g, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_257([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_41([a, b, c, e], is_dual)
    } else {
        select_41([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_256([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_257([a, b, c, e, f], is_dual)
    } else {
        select_139([a, b, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_258([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_54([b, c, d, a, e, f, h], is_dual)
    } else {
        select_54([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_255([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_256([b, a, c, e, h, g], is_dual)
    } else {
        select_258([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_251([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_252([b, a, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_255([a, b, c, i, d, j, h, g, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_245([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_246([b, a, c, i, f, g, h, m, k, j, l, n], is_dual)
    } else {
        select_251([b, a, h, j, d, e, l, n, i, m, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_233([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_234([b, c, d, f, g, e, a, i, h, k, l, m, j, n], is_dual)
    } else {
        select_245([b, a, e, f, g, c, d, i, k, j, l, h, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_199([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_200([a, c, e, f, b, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_233([a, c, e, f, d, g, h, i, b, j, k, m, n, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_148([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < k) || (is_dual && o > k) {
        select_149([a, c, d, e, f, g, b, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_199([a, c, b, d, e, f, g, m, j, l, o, n, k, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_147([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_148([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o, p], is_dual)
    } else {
        select_148([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < l) || (is_dual && p > l) {
        select_11([i, j, a, g, h, c, d, b, e, f, k, l, m, n, o, p], is_dual)
    } else {
        select_147([i, j, a, g, b, c, d, h, e, f, k, m, n, o, p, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_10([a, h, b, c, d, e, f, g, i, j, l, k, m, n, o, p], is_dual)
    } else {
        select_10([a, k, b, c, d, e, f, g, i, j, l, h, m, n, o, p], is_dual)
    }
}
/// n = 9, i = 2
fn select_269([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_127([c, d, e, f, g], is_dual)
    } else {
        select_38([a, b, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_270([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_29([a, b, c, d, e, g, h], is_dual)
    } else {
        select_29([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_268([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_269([a, h, c, d, e, f, g, i, j], is_dual)
    } else {
        select_270([b, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_272([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_127([c, d, e, f, h], is_dual)
    } else {
        select_106([a, b, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_272([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_138([h, a, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_267([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_268([b, a, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_271([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_275([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_127([d, e, f, g, h], is_dual)
    } else {
        select_102([a, b, c, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_276([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, b, c, d, f, g], is_dual)
    } else {
        select_21([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_274([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_275([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_276([h, d, e, f, g, c, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_278([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_167([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_167([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_277([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_278([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_276([h, c, d, e, f, a, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_273([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_274([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_277([c, b, d, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_266([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_267([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_273([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_283([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_55([a, d, b, e, h, i], is_dual)
    } else {
        select_53([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_282([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_283([b, a, c, d, g, f, i, j, h], is_dual)
    } else {
        select_189([b, a, h, e, j, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_284([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_66([e, c, a, d, f, g], is_dual)
    } else {
        select_66([a, b, d, f, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_281([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_282([b, c, d, e, a, g, f, h, i, j], is_dual)
    } else {
        select_284([c, f, d, e, a, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_287([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_100([b, a, c, d, e, f, g], is_dual)
    } else {
        select_41([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_287([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_287([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_289([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_55([a, b, c, e, d, g], is_dual)
    } else {
        select_55([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_290([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_146([a, b, c, e, d, f], is_dual)
    } else {
        select_146([a, b, d, e, c, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_288([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_289([g, a, c, d, e, h, i], is_dual)
    } else {
        select_290([c, b, e, h, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_286([d, b, c, a, e, f, g, h, i], is_dual)
    } else {
        select_288([c, d, a, e, b, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_280([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_281([b, a, c, e, d, f, g, i, h, j], is_dual)
    } else {
        select_285([a, b, c, e, h, g, i, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_279([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_280([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_280([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_265([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_266([a, b, c, d, f, g, h, j, k, l, m], is_dual)
    } else {
        select_279([a, b, c, d, k, e, i, l, j, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_295([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_207([a, b, c, d, f, g], is_dual)
    } else {
        select_155([b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_158([b, c, d, a, e, f, g], is_dual)
    } else {
        select_295([a, f, b, c, e, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_293([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_46([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_294([a, b, c, d, i, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_292([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_293([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_293([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 1
fn select_297([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_65([c, h, e, f, a, b, g, i], is_dual)
    } else {
        select_65([d, g, e, f, a, b, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_300([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_155([e, a, c, d, g], is_dual)
    } else {
        select_66([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_299([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_47([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_300([b, h, c, d, a, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_298([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_65([d, g, e, f, a, b, h, i], is_dual)
    } else {
        select_299([a, b, c, h, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_296([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_297([a, c, b, e, f, g, d, h, i], is_dual)
    } else {
        select_298([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_291([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_292([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_296([a, b, d, e, c, g, h, f, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_264([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_265([a, b, c, d, h, e, f, g, i, j, k, m, l], is_dual)
    } else {
        select_291([a, b, j, e, f, g, c, d, l, k, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_304([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_209([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_209([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_307([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_142([a, c, b, d, e], is_dual)
    } else {
        select_142([b, c, a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_306([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_307([a, b, e, c, f], is_dual)
    } else {
        select_207([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_308([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_111([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_111([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_305([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_306([a, b, c, g, d, h], is_dual)
    } else {
        select_308([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_303([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_304([a, b, c, e, d, f, g, h, j, i], is_dual)
    } else {
        select_305([a, b, c, d, f, g, i, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_312([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_66([a, c, b, d, e, f], is_dual)
    } else {
        select_132([a, c, f, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_311([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_312([a, c, b, e, d, f], is_dual)
    } else {
        select_95([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_315([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([b, c, d, e], is_dual)
    } else {
        select_56([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_314([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_37([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_315([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_316([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_96([f, d, g, e, a, b], !is_dual)
    } else {
        select_96([f, e, g, d, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_313([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_314([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_316([b, c, g, f, a, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_310([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_311([a, f, c, d, b, g, h], is_dual)
    } else {
        select_313([a, b, e, c, d, f, h, i, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_320([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_41([a, b, c, d], is_dual)
    } else {
        select_41([a, b, d, c], is_dual)
    }
}
/// n = 8, i = 3
fn select_319([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_320([b, c, e, f], is_dual)
    } else {
        select_79([a, d, b, c, e, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_321([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_143([a, f, g], is_dual)
    } else {
        select_146([a, b, c, d, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_318([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_319([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_321([f, h, a, g, e, b, c], !is_dual)
    }
}
/// n = 11, i = 3
fn select_317([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_274([c, d, b, a, e, f, g, h, i, j], is_dual)
    } else {
        select_318([b, c, d, i, a, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_309([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_310([a, c, d, e, i, b, j, h, k], is_dual)
    } else {
        select_317([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_302([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_303([a, b, c, d, e, i, h, k, l, j], is_dual)
    } else {
        select_309([a, b, h, c, d, f, g, j, i, l, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_327([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_55([a, f, b, e, d, g], is_dual)
    } else {
        select_67([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_326([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_327([e, c, d, h, f, g, j], is_dual)
    } else {
        select_54([a, f, b, e, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_325([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_286([d, b, f, e, a, g, i, h, j], is_dual)
    } else {
        select_326([b, a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_329([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_58([a, e, b, h, g, i], is_dual)
    } else {
        select_327([a, c, d, f, e, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_331([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_62([b, f, d, a, h], is_dual)
    } else {
        select_53([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_330([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_331([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_331([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_328([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_329([a, e, d, c, g, h, f, j, i], is_dual)
    } else {
        select_330([a, d, b, f, g, e, j, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_324([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_325([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_328([a, d, c, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_335([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_55([b, d, c, e, g, f], is_dual)
    } else {
        select_62([d, h, a, b, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_334([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_335([b, a, c, f, e, h, i, g], is_dual)
    } else {
        select_77([b, d, c, a, e, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_337([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_85([e, c, d, f, h, i], is_dual)
    } else {
        select_85([e, a, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_336([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_337([c, d, e, f, a, g, i, h, j], is_dual)
    } else {
        select_85([a, b, h, g, f, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_333([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_334([b, a, d, h, g, f, j, k, i], is_dual)
    } else {
        select_336([b, d, c, f, e, g, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_340([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_41([a, g, c, i], is_dual)
    } else {
        select_53([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_339([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_52([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_340([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_339([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_80([a, f, i, e, b, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_332([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_333([b, c, d, e, a, f, h, g, i, j, k], is_dual)
    } else {
        select_338([b, c, g, e, f, a, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_323([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_324([a, g, b, c, e, i, h, f, k, j], is_dual)
    } else {
        select_332([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_322([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_323([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_323([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_301([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_302([a, b, c, d, h, f, g, i, j, l, m, k], is_dual)
    } else {
        select_322([a, b, c, d, j, e, k, h, m, l, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < c) || (is_dual && k > c) {
        select_264([a, b, d, g, c, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_301([a, b, d, g, h, e, f, i, j, k, c, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_21([c, d, e, f], is_dual)
    } else {
        select_62([h, i, a, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_347([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_37([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_348([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_350([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([a, b, c, d, f, g], is_dual)
    } else {
        select_41([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_349([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_350([a, f, b, d, g, e, h], is_dual)
    } else {
        select_350([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_346([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_347([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_349([b, c, h, f, g, a, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_352([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_44([a, d, e, b, c, f, g, h], is_dual)
    } else {
        select_163([a, f, d, e, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_351([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_49([b, d, e, f, h, g, i], is_dual)
    } else {
        select_352([a, b, d, c, i, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_345([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_346([a, b, d, c, h, i, g, k, l, j], is_dual)
    } else {
        select_351([b, c, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_356([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_55([a, c, d, e, f, h], is_dual)
    } else {
        select_55([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_355([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_142([g, e, c, b, f], !is_dual)
    } else {
        select_356([a, c, d, b, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_357([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_48([f, c, d, e, b, g], is_dual)
    } else {
        select_37([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_354([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_355([a, b, c, h, g, f, i, j], is_dual)
    } else {
        select_357([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_354([b, a, c, d, h, i, g, k, l, j], is_dual)
    } else {
        select_351([b, d, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_344([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_345([a, c, d, b, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_353([a, c, b, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_360([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_122([c, f, d, e, b, g, h], is_dual)
    } else {
        select_204([a, b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_359([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_360([a, b, d, c, h, i, g, k, j], is_dual)
    } else {
        select_351([a, c, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_364([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_55([a, b, c, d, e, g], is_dual)
    } else {
        select_62([f, g, a, d, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_363([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_37([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_364([b, c, g, a, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_362([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_122([c, f, d, e, a, g, h], is_dual)
    } else {
        select_363([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_361([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_362([a, b, d, c, h, i, g, k, j, l], is_dual)
    } else {
        select_351([b, c, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_359([b, c, d, a, e, f, g, i, j, h, k], is_dual)
    } else {
        select_361([a, b, d, c, e, f, h, i, j, g, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_343([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_344([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_358([b, d, a, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_342([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_343([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    } else {
        select_343([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_371([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_85([a, c, d, e, f, g], is_dual)
    } else {
        select_41([a, b, h, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_372([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_85([a, b, c, f, i, j], is_dual)
    } else {
        select_40([b, c, d, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_370([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_371([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_372([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_369([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_370([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_370([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_374([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_207([a, e, c, d, f, g], is_dual)
    } else {
        select_58([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_373([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_122([a, c, d, e, b, f, g], is_dual)
    } else {
        select_374([b, f, d, e, a, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_368([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_369([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_373([a, b, g, e, f, c, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_376([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_49([b, c, a, d, e, f, g], is_dual)
    } else {
        select_96([b, c, g, f, a, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_375([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_65([a, d, e, f, b, c, g, h], is_dual)
    } else {
        select_376([b, c, g, e, f, a, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_367([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_368([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_375([a, b, d, c, f, g, e, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_380([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_207([a, d, f, g, e, c], !is_dual)
    } else {
        select_102([a, f, d, g, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_381([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_58([a, c, d, f, e, g], is_dual)
    } else {
        select_139([a, b, c, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_379([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_380([a, e, d, b, f, g, h], is_dual)
    } else {
        select_381([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_382([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_215([a, c, d, b, e, f, g], is_dual)
    } else {
        select_215([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_378([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_379([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_382([a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 9, i = 1
fn select_384([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_47([a, b, c, d, e, f, h, i], is_dual)
    } else {
        select_47([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_386([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_214([a, b, c, d, f, g, h, i], is_dual)
    } else {
        select_214([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_388([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_38([a, b, c, d], is_dual)
    } else {
        select_38([a, b, c, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_387([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_127([c, d, e, f, g], is_dual)
    } else {
        select_388([a, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_385([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_386([g, c, d, e, f, a, h, i, j], is_dual)
    } else {
        select_387([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_383([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_384([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_385([a, h, d, e, f, g, b, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_377([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_378([a, b, i, d, c, h, j, k], is_dual)
    } else {
        select_383([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_366([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_367([a, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_377([a, b, c, f, d, e, g, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_393([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_127([b, c, d, e, g], is_dual)
    } else {
        select_42([a, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_392([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_275([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_393([b, d, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_391([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_136([a, h, d, e, f, g, b, i, j, k], is_dual)
    } else {
        select_392([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_395([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_382([a, b, c, f, h, e, g], is_dual)
    } else {
        select_382([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 5, i = 2
fn select_398([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_41([a, b, c, d], is_dual)
    } else {
        select_56([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_397([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_207([a, b, c, d, e, f], is_dual)
    } else {
        select_398([a, b, e, c, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_396([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_382([a, b, e, g, c, f, d], !is_dual)
    } else {
        select_397([a, b, d, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_394([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_395([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_396([a, b, h, e, f, g, c], !is_dual)
    }
}
/// n = 11, i = 3
fn select_390([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_391([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_394([a, b, i, d, c, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_389([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_367([a, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_390([a, b, c, f, d, e, g, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_365([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_366([a, b, c, f, g, d, h, e, i, j, k], is_dual)
    } else {
        select_389([a, b, c, f, g, e, h, d, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_341([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_342([a, b, c, d, g, h, k, i, j, m, l, n], is_dual)
    } else {
        select_365([a, b, c, d, g, e, f, l, n, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < l) || (is_dual && d > l) {
        select_263([a, b, c, e, f, g, h, j, l, d, k, n, m], is_dual)
    } else {
        select_341([a, b, c, e, f, g, h, d, i, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_405([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_214([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_214([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_404([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_369([a, b, e, h, f, g, j, i, k, l], is_dual)
    } else {
        select_405([a, b, c, d, k, h, j, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_403([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_404([g, h, a, b, k, c, d, i, j, m, n, l], is_dual)
    } else {
        select_404([g, h, e, f, i, c, d, k, j, m, l, n], is_dual)
    }
}
/// n = 12, i = 3
fn select_408([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_118([a, d, e, b, c, h, g, i, j, l], is_dual)
    } else {
        select_158([b, c, f, g, i, h, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_409([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_88([a, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_168([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_407([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_408([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_409([a, b, c, i, f, j, h, g, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_406([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_407([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_407([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_402([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_403([c, i, d, e, g, h, a, b, m, j, k, l, o, n], is_dual)
    } else {
        select_406([a, b, c, l, g, h, f, i, k, n, m, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_414([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_29([b, c, d, e, i, g, h], is_dual)
    } else {
        select_193([a, h, b, c, f, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_413([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_47([h, b, c, d, e, f, i, j], is_dual)
    } else {
        select_414([a, b, c, d, e, g, i, j, h, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_417([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_21([d, e, f, i], is_dual)
    } else {
        select_40([a, b, c, h, g, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_416([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_20([c, d, j, e, f, g, i, k], is_dual)
    } else {
        select_417([a, b, i, e, f, g, h, k, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_419([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_100([a, c, d, e, g, f, h], is_dual)
    } else {
        select_23([a, b], is_dual)
    }
}
/// n = 10, i = 3
fn select_418([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_207([a, h, d, e, g, j], is_dual)
    } else {
        select_419([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_415([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_416([b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_418([a, d, e, b, c, j, i, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_412([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_413([i, d, e, f, g, h, k, a, j, l, m, n], is_dual)
    } else {
        select_415([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 7, i = 1
fn select_422([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_70([a, b, c, d, e, f, g], is_dual)
    } else {
        select_129([a, b, g, e, f, d], is_dual)
    }
}
/// n = 7, i = 1
fn select_421([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_422([a, d, e, b, c, f, g], is_dual)
    } else {
        select_422([c, d, e, b, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_420([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_187([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_421([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 14, i = 3
fn select_411([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_412([a, e, f, g, h, b, c, d, i, j, k, l, n, m], is_dual)
    } else {
        select_420([a, k, b, c, d, j, m, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_425([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_312([a, b, e, f, g, c], is_dual)
    } else {
        select_237([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_424([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_236([d, e, f, j, g, h, i, l, k], is_dual)
    } else {
        select_425([a, b, c, f, k, h, m, j], is_dual)
    }
}
/// n = 9, i = 1
fn select_428([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_20([a, b, c, d, e, f, h, i], is_dual)
    } else {
        select_21([d, e, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_429([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_320([a, b, c, e], is_dual)
    } else {
        select_41([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_427([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_428([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_429([a, b, c, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_430([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_158([d, e, f, g, i, h, j], is_dual)
    } else {
        select_429([a, b, c, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_426([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_427([a, b, c, g, h, i, m, k, l, o], is_dual)
    } else {
        select_430([a, b, c, d, e, f, l, j, k, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_423([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_424([b, c, d, e, f, k, g, a, j, l, m, n, o], is_dual)
    } else {
        select_426([b, c, d, e, f, g, a, h, i, j, l, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_410([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < m) || (is_dual && b > m) {
        select_411([a, c, d, e, f, j, h, i, m, k, b, l, o, n], is_dual)
    } else {
        select_423([a, c, d, e, b, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_401([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_402([a, b, f, d, e, g, h, i, j, k, l, c, m, n, o], is_dual)
    } else {
        select_410([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    }
}
/// n = 9, i = 1
fn select_434([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_47([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_122([a, i, b, c, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_433([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_121([a, h, c, e, f, b, g, i, j], is_dual)
    } else {
        select_434([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_436([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_47([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_374([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_435([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_369([a, b, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_436([d, g, e, f, a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_432([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_433([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_435([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_439([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_118([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_118([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_438([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_369([f, g, a, i, d, e, h, j, k, l], is_dual)
    } else {
        select_439([f, g, d, e, b, c, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_437([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_438([e, a, b, c, d, f, g, h, i, j, k, l], is_dual)
    } else {
        select_120([h, a, b, c, d, f, g, e, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_432([a, b, c, d, f, e, j, i, l, k, m], is_dual)
    } else {
        select_437([d, i, g, h, e, a, b, c, k, j, l, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_400([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < l) || (is_dual && n > l) {
        select_401([a, b, c, d, e, g, i, f, h, j, k, l, m, n, o], is_dual)
    } else {
        select_431([a, b, c, g, f, i, h, k, j, n, m, l, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_445([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_210([a, b, h, d, e, f, g, i, k, j], is_dual)
    } else {
        select_210([a, c, g, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_447([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_215([a, b, f, d, h, e, g], is_dual)
    } else {
        select_215([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_446([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_447([a, c, g, e, f, b, i, h], is_dual)
    } else {
        select_213([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_444([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_445([a, c, h, d, e, f, g, i, j, l, k], is_dual)
    } else {
        select_446([a, g, j, b, f, k, h, l, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_443([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_444([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_444([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_451([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_215([a, b, c, d, f, e, g], is_dual)
    } else {
        select_85([d, f, g, a, b, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_450([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_451([a, b, g, e, h, i, f], is_dual)
    } else {
        select_213([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_454([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_67([b, c, d, e, f], is_dual)
    } else {
        select_38([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_453([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_29([b, c, d, e, f, g, h], is_dual)
    } else {
        select_454([a, h, b, c, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_452([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_204([a, b, c, d, h, g, j, i], is_dual)
    } else {
        select_453([a, b, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_449([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_450([b, h, e, f, g, a, i, j, k], is_dual)
    } else {
        select_452([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_457([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_142([a, f, d, g, h], is_dual)
    } else {
        select_228([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_456([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_457([e, c, d, a, f, g, h, i], is_dual)
    } else {
        select_229([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_459([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_289([a, b, c, f, g, e, h], is_dual)
    } else {
        select_289([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_458([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_314([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_459([a, h, b, c, f, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_455([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_456([a, g, d, e, b, f, h, i, j], is_dual)
    } else {
        select_458([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_448([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_449([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_455([a, g, d, e, f, b, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_442([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_443([a, b, c, j, f, g, l, h, i, k, n, m], is_dual)
    } else {
        select_448([a, b, h, k, d, e, i, m, j, n, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_464([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_58([b, c, d, f, e, g], is_dual)
    } else {
        select_96([g, f, h, a, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_463([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_221([a, g, h, e, b, f, d], !is_dual)
    } else {
        select_464([a, b, c, d, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_465([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_219([a, c, d, e, f, g], is_dual)
    } else {
        select_295([a, b, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_462([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_463([a, b, i, c, j, g, h, k], is_dual)
    } else {
        select_465([a, b, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_461([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_462([a, b, f, c, d, e, g, h, j, k, i], is_dual)
    } else {
        select_420([a, h, c, d, e, b, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_469([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_155([a, b, c, d, f], is_dual)
    } else {
        select_102([a, b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_219([a, c, d, e, f, g], is_dual)
    } else {
        select_469([a, h, f, i, b, g], !is_dual)
    }
}
/// n = 9, i = 2
fn select_470([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_49([f, a, c, d, e, g, h], is_dual)
    } else {
        select_132([a, b, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_467([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_468([f, g, c, d, e, a, h, i, j], is_dual)
    } else {
        select_470([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_466([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_467([a, e, b, c, d, f, g, i, j, h], is_dual)
    } else {
        select_420([a, f, b, c, d, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_460([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_461([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_466([a, c, d, e, g, h, i, b, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_441([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_442([a, b, e, c, d, f, g, h, i, j, k, l, n, m], is_dual)
    } else {
        select_460([a, b, c, d, m, e, h, i, n, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_472([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_292([a, b, c, d, i, g, h, k, j, l, m], is_dual)
    } else {
        select_405([a, b, e, f, l, i, k, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_476([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_295([a, b, c, d, e, f, g], is_dual)
    } else {
        select_397([a, b, c, h, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_475([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_476([a, b, c, d, g, f, h, i], is_dual)
    } else {
        select_476([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_474([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_475([a, b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_475([a, b, c, e, f, d, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_479([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_210([f, e, h, c, d, g, j, i, l, k], is_dual)
    } else {
        select_210([f, e, i, a, b, g, k, h, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_480([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_428([b, c, h, d, e, f, g, i, j], is_dual)
    } else {
        select_387([a, i, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_478([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_479([b, c, d, e, m, a, i, j, k, l, n, o], is_dual)
    } else {
        select_480([a, d, e, f, g, h, l, i, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_477([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_478([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    } else {
        select_478([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_473([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < o) || (is_dual && j > o) {
        select_474([a, b, l, c, d, m, o, j, n], is_dual)
    } else {
        select_477([a, b, e, f, c, d, g, h, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_471([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_472([a, b, e, f, g, h, i, j, k, n, l, m, o], is_dual)
    } else {
        select_473([a, b, c, d, g, h, e, f, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_440([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_441([a, b, d, e, f, h, i, k, c, j, l, n, m, o], is_dual)
    } else {
        select_471([a, b, d, e, c, f, h, i, g, j, l, m, k, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_399([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_400([a, b, c, d, e, g, f, i, h, j, k, m, l, n, o], is_dual)
    } else {
        select_440([a, b, c, d, e, g, i, f, h, k, m, l, j, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < n) || (is_dual && k > n) {
        select_262([a, b, c, j, d, g, h, e, f, l, m, n, o, k], is_dual)
    } else {
        select_399([a, b, c, g, h, f, d, i, e, j, m, l, k, o, n], is_dual)
    }
}
/// n = 11, i = 2
fn select_488([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_127([c, d, e, f, i], is_dual)
    } else {
        select_129([a, b, j, g, h, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_489([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_127([c, d, e, f, h], is_dual)
    } else {
        select_67([i, a, b, j, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_487([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_488([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_489([a, h, c, d, e, f, i, g, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_490([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_158([a, b, c, e, f, d, g], is_dual)
    } else {
        select_158([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_486([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_487([d, e, a, b, c, f, g, h, i, j, k], is_dual)
    } else {
        select_490([a, b, c, g, f, i, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_493([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_207([a, f, c, d, g, h], is_dual)
    } else {
        select_398([a, h, b, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_492([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_493([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_58([a, g, b, h, e, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_491([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_492([e, f, b, c, j, k, g, h, i], is_dual)
    } else {
        select_95([h, i, a, d, e, f, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_485([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_486([a, b, c, d, f, e, g, h, j, i, k, l], is_dual)
    } else {
        select_491([d, b, c, f, g, h, i, a, k, j, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_498([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([a, b, d, f, e, h], is_dual)
    } else {
        select_100([c, a, b, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_497([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_155([a, c, d, h, g], is_dual)
    } else {
        select_498([a, g, b, e, i, f, j, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_500([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_100([a, c, d, e, h, g, j], is_dual)
    } else {
        select_100([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_499([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_500([a, f, c, d, e, h, i, g, j, k], is_dual)
    } else {
        select_184([g, b, c, d, f, i, h, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_496([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_497([j, c, a, b, f, g, h, m, k, l], is_dual)
    } else {
        select_499([h, c, d, e, k, f, l, g, i, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_495([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_496([a, b, c, d, e, f, g, h, i, l, j, k, m], is_dual)
    } else {
        select_496([a, b, c, d, e, f, g, h, i, l, k, j, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_502([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_312([a, b, c, e, d, f], is_dual)
    } else {
        select_312([a, b, d, e, c, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_504([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([b, f, c, d, e, g, h], is_dual)
    } else {
        select_132([e, a, h, f, i], is_dual)
    }
}
/// n = 12, i = 2
fn select_503([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_504([a, k, b, c, l, g, h, i, j], is_dual)
    } else {
        select_393([a, d, e, f, j, g, h, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_501([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_502([a, b, c, j, h, m], is_dual)
    } else {
        select_503([d, e, f, a, b, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_494([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_495([b, c, d, e, f, g, i, h, j, a, k, l, m], is_dual)
    } else {
        select_501([a, b, c, d, e, f, g, i, j, l, h, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < n) || (is_dual && j > n) {
        select_485([a, b, c, i, d, e, k, h, l, n, m, j], is_dual)
    } else {
        select_494([a, b, c, e, f, g, h, l, j, i, m, k, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_509([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_207([a, b, c, f, e, g], is_dual)
    } else {
        select_207([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_510([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_70([b, c, a, d, e, f, g], is_dual)
    } else {
        select_54([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_509([a, e, b, g, f, i, h], is_dual)
    } else {
        select_510([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_513([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_55([a, g, c, f, d, i], is_dual)
    } else {
        select_53([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_512([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_331([a, e, c, g, f, h, d, i], is_dual)
    } else {
        select_513([a, c, b, d, f, e, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_511([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_512([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_512([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_507([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_508([a, d, b, c, e, g, f, i, h], is_dual)
    } else {
        select_511([a, e, c, d, h, g, f, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_517([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_61([a, b, e, f, d, g], is_dual)
    } else {
        select_67([b, c, d, f, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_516([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_106([a, b, g, d, e, i], is_dual)
    } else {
        select_517([a, e, c, d, f, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_518([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([b, e, c, d, f, g, h], is_dual)
    } else {
        select_67([a, f, h, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_516([g, b, c, a, e, f, h, i, j], is_dual)
    } else {
        select_518([c, b, a, d, f, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_520([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_29([b, h, c, d, e, g, i], is_dual)
    } else {
        select_167([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_519([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_516([i, c, b, a, f, g, h, j, k], is_dual)
    } else {
        select_520([b, c, a, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_514([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_515([a, j, c, b, k, g, f, i, h, l], is_dual)
    } else {
        select_519([a, c, f, d, e, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_506([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_507([a, g, j, b, h, l, f, m, i, k], is_dual)
    } else {
        select_514([a, c, f, d, e, g, i, h, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_505([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_506([a, b, c, d, e, f, g, j, h, k, i, l, m], is_dual)
    } else {
        select_506([a, b, c, d, e, f, g, j, i, k, h, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_483([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_484([a, b, c, d, e, f, g, h, j, l, m, i, k, n], is_dual)
    } else {
        select_505([a, c, e, f, g, i, h, b, k, l, j, m, n], is_dual)
    }
}
/// n = 11, i = 2
fn select_525([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_166([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_166([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_524([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_525([d, e, f, a, b, c, g, h, i, j, l], is_dual)
    } else {
        select_490([a, b, c, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_528([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_29([g, h, d, e, f, j, i], is_dual)
    } else {
        select_29([g, i, a, b, c, k, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_529([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_320([a, b, c, h], is_dual)
    } else {
        select_110([a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_527([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_528([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_529([a, b, c, d, i, k, h, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_530([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_320([a, b, c, i], is_dual)
    } else {
        select_113([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_526([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_527([a, b, c, i, f, g, h, j, k, m, l, n], is_dual)
    } else {
        select_530([a, b, c, d, e, l, j, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_523([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_524([b, c, d, i, g, h, a, l, j, k, n, m], is_dual)
    } else {
        select_526([b, c, d, g, h, a, e, f, j, k, i, l, m, n], is_dual)
    }
}
/// n = 11, i = 2
fn select_534([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_29([g, h, b, c, d, k, i], is_dual)
    } else {
        select_70([a, i, e, f, g, j, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_58([a, c, b, j, g, k], is_dual)
    } else {
        select_167([b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_533([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_534([a, e, f, g, c, d, h, i, k, l, j], is_dual)
    } else {
        select_535([a, i, b, e, f, g, j, k, h, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_537([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_58([a, h, b, i, f, j], is_dual)
    } else {
        select_37([a, f, c, d, e, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_536([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_537([a, h, d, e, f, i, g, j, k, l], is_dual)
    } else {
        select_534([a, d, e, f, b, c, g, h, j, k, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_532([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_533([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_536([h, c, d, e, f, g, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_532([a, e, f, g, b, c, d, h, i, k, j, l, m], is_dual)
    } else {
        select_420([a, h, b, c, d, k, m, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_523([a, c, d, e, f, g, b, h, j, i, k, l, m, n], is_dual)
    } else {
        select_531([a, c, d, e, i, f, g, b, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_543([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_100([c, a, b, d, f, e, g], is_dual)
    } else {
        select_41([a, b, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_542([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_543([a, b, d, e, h, f, i, g], is_dual)
    } else {
        select_320([a, b, c, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_541([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_542([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_542([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_544([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_47([d, e, i, f, g, h, k, j], is_dual)
    } else {
        select_429([a, b, c, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_540([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < l) || (is_dual && a > l) {
        select_541([b, c, d, j, l, a, m, i, k], is_dual)
    } else {
        select_544([b, c, d, a, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_546([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_312([b, c, a, e, d, f], is_dual)
    } else {
        select_188([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_545([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_187([a, e, f, i, g, j, h, k], is_dual)
    } else {
        select_546([a, b, c, d, k, l, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_539([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_540([a, b, c, d, h, e, f, g, j, k, l, i, m], is_dual)
    } else {
        select_545([a, b, c, d, e, f, g, i, j, l, k, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_550([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_89([a, b, f, d, e, g, h], is_dual)
    } else {
        select_60([g, f, h, d, e, c, a], !is_dual)
    }
}
/// n = 11, i = 3
fn select_549([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_276([b, c, d, e, h, g, i], is_dual)
    } else {
        select_550([b, a, i, f, g, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_551([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_276([c, d, e, f, i, g, j], is_dual)
    } else {
        select_51([a, c, j, b, g, h, k, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_548([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_549([h, c, d, e, f, i, g, a, j, k, l], is_dual)
    } else {
        select_551([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_547([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_548([a, e, f, b, c, d, g, h, i, j, l, k], is_dual)
    } else {
        select_420([a, i, b, c, d, g, k, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_539([a, d, e, f, b, c, g, h, j, k, i, l, m], is_dual)
    } else {
        select_547([a, d, e, f, c, h, j, i, b, k, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_521([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_522([a, b, d, e, f, c, g, h, i, j, m, k, n, l], is_dual)
    } else {
        select_538([a, b, c, d, e, f, g, i, j, l, m, n, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_483([a, b, d, g, h, e, f, i, j, k, l, m, c, n], is_dual)
    } else {
        select_521([a, b, d, c, e, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_558([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_37([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_37([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_557([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_558([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_28([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_556([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_27([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_557([a, i, c, e, f, g, b, h, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_560([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_422([b, c, d, a, e, f, g], is_dual)
    } else {
        select_207([a, b, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_562([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_70([b, d, a, c, e, f, g], is_dual)
    } else {
        select_207([a, e, c, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_561([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_562([b, g, c, d, a, f, h, i, j], is_dual)
    } else {
        select_69([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_559([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_560([a, f, d, e, b, g, h, i, j], is_dual)
    } else {
        select_561([a, b, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_555([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_556([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_559([a, b, j, d, e, c, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_554([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_432([a, b, c, d, e, i, j, h, l, k, m], is_dual)
    } else {
        select_555([a, b, c, f, g, d, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_553([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_554([a, b, c, d, g, e, h, i, f, j, k, l, m], is_dual)
    } else {
        select_554([a, b, c, d, g, f, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_567([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_275([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_178([b, c, i, d, a, j, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_569([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_207([a, d, f, g, e, c], !is_dual)
    } else {
        select_155([a, d, f, g, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_568([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_219([a, c, d, e, g, h], is_dual)
    } else {
        select_569([a, h, b, i, f, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_566([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_567([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_568([a, c, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_571([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_382([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_95([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_570([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_94([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_571([a, f, c, e, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_565([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_566([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_570([a, b, d, i, e, c, j, h, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_572([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_382([a, b, c, f, e, g, h], is_dual)
    } else {
        select_207([a, b, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_564([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < c) || (is_dual && k > c) {
        select_565([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_572([a, b, i, d, j, h, k, c], is_dual)
    }
}
/// n = 7, i = 3
fn select_577([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_215([a, b, c, d, e, f, g], is_dual)
    } else {
        select_215([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_576([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_577([a, b, g, e, f, h, i], is_dual)
    } else {
        select_214([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_575([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_576([b, j, c, d, h, a, i, k, l], is_dual)
    } else {
        select_480([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_574([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_575([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    } else {
        select_575([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_573([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_432([a, b, c, d, g, h, j, i, k, l, m], is_dual)
    } else {
        select_574([a, b, c, e, f, d, h, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_563([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_564([a, b, c, d, l, g, h, i, k, n, m], is_dual)
    } else {
        select_573([a, b, c, e, g, h, f, i, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_552([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_553([a, b, c, g, e, f, h, i, j, k, m, l, n], is_dual)
    } else {
        select_563([a, b, c, d, g, h, e, f, i, j, l, m, k, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_481([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_482([a, b, c, d, e, f, g, j, m, k, l, n, i, o], is_dual)
    } else {
        select_552([a, b, c, d, e, f, i, h, k, j, n, l, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_260([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_261([a, b, c, d, e, f, g, h, i, k, j, l, m, n, o], is_dual)
    } else {
        select_481([a, b, c, f, g, h, i, d, k, j, l, m, e, n, o], is_dual)
    }
}
/// n = 12, i = 2
fn select_586([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_118([g, c, d, e, f, i, h, j, k, l], is_dual)
    } else {
        select_118([g, c, d, a, b, k, h, j, i, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_585([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_586([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_586([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_584([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_116([i, b, c, d, e, f, g, h, j, k, l], is_dual)
    } else {
        select_585([a, h, b, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_583([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_115([e, f, j, l, d, a, b, c, k, n, m], is_dual)
    } else {
        select_584([g, d, k, h, i, a, b, c, j, m, l, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_591([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_207([a, e, c, d, f, g], is_dual)
    } else {
        select_41([a, g, e, b], !is_dual)
    }
}
/// n = 8, i = 2
fn select_590([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_591([a, f, d, e, b, g, h], is_dual)
    } else {
        select_253([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_589([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_436([a, d, e, f, b, c, g, h, i], is_dual)
    } else {
        select_590([b, c, g, e, f, a, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_588([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_589([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_589([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_28([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_596([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_102([a, b, d, e, h], is_dual)
    } else {
        select_163([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_595([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_244([d, a, h, b, c, g, i], is_dual)
    } else {
        select_596([b, c, a, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_593([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_594([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_595([a, b, c, j, d, e, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_592([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_593([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_593([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_587([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_588([a, b, j, c, d, e, f, k, i], is_dual)
    } else {
        select_592([a, b, e, f, g, h, c, d, j, i, k], is_dual)
    }
}
/// n = 15, i = 3
fn select_582([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < m) || (is_dual && o > m) {
        select_583([a, c, d, b, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_587([a, b, e, f, c, d, j, l, k, n, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_602([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_127([d, e, f, g, h], is_dual)
    } else {
        select_189([a, b, c, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_601([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_602([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_393([c, d, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_603([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_219([a, c, d, e, f, h], is_dual)
    } else {
        select_111([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_600([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_601([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_603([a, h, d, e, f, g, b, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_605([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_248([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_248([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_607([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_113([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_198([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_606([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_607([c, a, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_607([c, b, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_604([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_605([c, d, e, a, b, f, g, h, i], is_dual)
    } else {
        select_606([a, b, c, d, e, f, g, i, j, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_599([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_600([a, b, c, f, g, h, j, i, k, m, l], is_dual)
    } else {
        select_604([a, b, c, d, e, k, i, l, j, m], is_dual)
    }
}
/// n = 12, i = 3
fn select_610([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_269([a, i, d, e, f, g, h, j, l], is_dual)
    } else {
        select_275([b, c, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_612([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_155([c, d, e, f, g], is_dual)
    } else {
        select_189([c, a, b, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_614([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_41([b, c, e, g], is_dual)
    } else {
        select_38([a, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_613([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_113([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_614([a, b, c, h, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_611([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_612([b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_613([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_609([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_610([a, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_611([a, c, d, b, k, h, j, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_608([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_609([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_609([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_598([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_599([b, c, d, e, f, g, h, i, a, j, k, l, m], is_dual)
    } else {
        select_608([b, c, a, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 12, i = 2
fn select_615([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_115([e, j, g, h, d, a, b, c, i, k, l], is_dual)
    } else {
        select_115([f, i, g, h, d, a, b, c, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_597([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_598([a, c, d, b, e, f, g, j, k, l, n, o, m], is_dual)
    } else {
        select_615([a, c, d, g, h, i, b, l, j, k, m, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_581([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_582([a, b, c, d, e, f, h, g, k, j, m, n, o, l, p], is_dual)
    } else {
        select_597([a, b, c, d, e, f, g, i, j, k, l, m, o, n, p], is_dual)
    }
}
/// n = 11, i = 3
fn select_622([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_58([a, i, b, j, g, k], is_dual)
    } else {
        select_127([c, d, e, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_621([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_622([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_126([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_623([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_278([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_138([a, h, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_620([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_621([a, c, b, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_623([a, i, d, e, f, g, h, b, j, l, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_625([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_118([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_249([b, h, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_624([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_625([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_625([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_619([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_620([a, b, c, d, e, f, g, h, i, k, l, j], is_dual)
    } else {
        select_624([b, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_618([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_619([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    } else {
        select_619([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_630([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_66([a, b, d, e, c, f], is_dual)
    } else {
        select_66([a, c, d, e, b, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_629([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_314([a, d, b, c, e, f, h, i], is_dual)
    } else {
        select_630([a, b, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_628([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_629([a, c, d, h, e, b, j, i, k], is_dual)
    } else {
        select_567([a, c, d, e, b, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_632([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_158([c, d, e, a, b, f, g], is_dual)
    } else {
        select_630([a, b, f, c, d, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_631([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_243([b, c, d, e, a, f, h, g, i], is_dual)
    } else {
        select_632([b, c, d, e, g, a, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_627([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_628([a, b, e, f, g, c, d, i, j, h, k], is_dual)
    } else {
        select_631([a, e, f, c, d, b, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_636([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_614([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_340([b, a, d, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_637([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_44([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_79([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_635([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_636([b, c, g, d, a, f, h, i, j], is_dual)
    } else {
        select_637([b, c, d, a, e, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_49([a, d, b, c, e, f, g], is_dual)
    } else {
        select_630([a, b, c, e, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_634([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_635([a, b, c, f, e, g, h, i, j, k], is_dual)
    } else {
        select_638([a, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_640([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_249([a, d, b, c, e, f, g, h, j], is_dual)
    } else {
        select_630([b, c, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_643([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_67([c, d, f, e, h], is_dual)
    } else {
        select_41([a, b, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_644([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_67([a, c, h, e, j], is_dual)
    } else {
        select_67([b, d, g, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_642([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_643([a, f, c, d, g, e, h, i, j], is_dual)
    } else {
        select_644([a, c, b, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_641([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_642([a, d, b, e, f, g, c, h, i, j], is_dual)
    } else {
        select_642([a, d, c, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_639([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_640([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_641([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_633([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_634([a, b, c, d, e, g, i, h, k, j, l], is_dual)
    } else {
        select_639([a, b, c, d, h, f, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_626([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_627([a, b, c, h, d, e, g, j, i, k, l], is_dual)
    } else {
        select_633([a, b, d, e, c, g, f, i, j, h, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_617([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_618([a, j, b, e, c, d, i, m, k, o, l, n], is_dual)
    } else {
        select_626([a, b, e, f, g, h, l, j, k, n, m, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_650([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_167([b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_340([a, b, h, c, g, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_649([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_650([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    } else {
        select_613([a, b, f, d, e, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_653([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_55([a, e, c, f, d, h], is_dual)
    } else {
        select_55([b, d, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_652([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_653([a, b, d, e, g, f, h, i], is_dual)
    } else {
        select_55([b, c, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_651([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_652([a, b, i, c, f, h, g, j, k], is_dual)
    } else {
        select_248([a, c, d, e, g, f, h, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_648([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_649([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_651([b, c, a, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_655([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_158([b, c, d, a, e, f, g], is_dual)
    } else {
        select_154([a, f, b, c, e, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_654([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_46([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_655([a, b, c, d, i, h, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_647([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_648([g, a, c, b, d, e, h, i, k, j, l], is_dual)
    } else {
        select_654([c, d, e, f, b, h, i, g, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_660([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_55([a, c, b, f, g, h], is_dual)
    } else {
        select_41([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_659([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_37([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_660([a, b, h, c, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_658([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_119([a, c, g, d, e, f, h, i, k], is_dual)
    } else {
        select_659([a, b, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_661([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_119([b, c, f, d, e, g, h, i, j], is_dual)
    } else {
        select_637([b, a, h, d, e, g, f, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_657([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_658([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_661([c, a, d, e, f, g, b, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_656([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_654([b, c, d, f, a, h, i, g, j, k], is_dual)
    } else {
        select_657([g, a, b, e, c, d, i, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_646([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_647([a, b, c, d, e, f, i, g, j, h, k, l], is_dual)
    } else {
        select_656([a, c, d, e, b, f, i, h, j, g, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_666([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_29([a, b, c, d, f, g, h], is_dual)
    } else {
        select_79([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_665([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_118([f, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_666([a, j, b, c, f, h, g, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_664([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_117([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_665([g, a, b, c, d, f, e, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_654([a, b, c, e, d, j, h, i, l, k], is_dual)
    } else {
        select_664([b, c, f, g, d, a, i, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_669([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_118([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_637([b, a, j, c, d, h, g, i, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_670([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_118([f, a, b, c, d, g, h, i, j, l], is_dual)
    } else {
        select_659([f, e, j, a, b, h, g, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_668([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_669([a, g, b, c, d, e, h, f, i, j, l, k], is_dual)
    } else {
        select_670([b, c, d, e, a, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_667([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_668([b, c, d, f, g, a, h, i, k, j, l, m], is_dual)
    } else {
        select_654([b, c, d, e, a, j, i, h, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_662([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_663([b, d, e, c, g, a, f, i, h, j, l, k], is_dual)
    } else {
        select_667([a, b, d, e, g, c, f, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_645([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_646([b, j, c, d, e, g, k, a, h, i, l, m], is_dual)
    } else {
        select_662([b, c, a, d, e, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_616([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < c) || (is_dual && l > c) {
        select_617([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    } else {
        select_645([a, b, f, g, h, l, i, j, k, c, n, m, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_580([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_581([a, e, c, d, g, h, b, i, f, k, j, l, m, n, p, o], is_dual)
    } else {
        select_616([a, e, b, g, h, f, c, d, k, l, j, m, o, p, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_579([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_580([a, b, d, e, f, c, g, h, i, k, l, m, n, j, o, p], is_dual)
    } else {
        select_580([a, c, d, e, f, b, g, h, i, j, l, m, n, k, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_578([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_579([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o, p], is_dual)
    } else {
        select_579([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_259([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && d < o) || (is_dual && d > o) {
        select_260([a, b, c, e, g, f, h, i, l, k, m, o, n, d, p], is_dual)
    } else {
        select_578([a, b, c, e, d, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < a) || (is_dual && m > a) {
        select_9([f, a, i, j, k, g, h, b, c, d, e, l, m, n, o, p], is_dual)
    } else {
        select_259([b, c, d, e, f, g, h, j, k, i, l, m, a, n, o, p], is_dual)
    }
}
/// n = 9, i = 2
fn select_681([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_612([a, b, c, d, g, h, f, i], is_dual)
    } else {
        select_612([a, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_47([c, e, f, a, b, d, g, h], is_dual)
    } else {
        select_154([a, b, c, d, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_680([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_681([b, c, d, a, f, h, g, i, j], is_dual)
    } else {
        select_682([b, c, a, e, d, g, j, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_685([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_102([b, c, d, e, f], is_dual)
    } else {
        select_205([a, b, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_684([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_685([a, c, d, f, g, i, h], is_dual)
    } else {
        select_469([a, b, e, h, j, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_688([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_41([c, d, e, g], is_dual)
    } else {
        select_41([a, b, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_687([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_643([b, c, a, d, e, g, f, i, h], is_dual)
    } else {
        select_688([a, f, b, c, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_686([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_687([b, c, d, g, f, a, h, i, j], is_dual)
    } else {
        select_613([b, c, d, a, e, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_683([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_684([a, b, c, d, g, e, i, j, h, k], is_dual)
    } else {
        select_686([b, a, c, d, f, h, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_679([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_680([b, d, e, c, f, a, g, i, j, h], is_dual)
    } else {
        select_683([b, a, d, e, f, c, h, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_678([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_679([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_679([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_692([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_47([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_314([a, g, d, e, f, b, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_691([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_27([c, e, f, g, a, b, h, i, j, k], is_dual)
    } else {
        select_692([a, b, d, c, i, j, h, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_694([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_227([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_227([c, b, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_696([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_89([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_653([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_695([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_696([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_696([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_694([a, b, c, e, f, i, h, k, j], is_dual)
    } else {
        select_695([c, a, b, d, j, g, k, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_691([b, c, d, e, a, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_693([b, c, a, e, d, j, i, l, k, h, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_699([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([a, b, d, e, f, c, g], is_dual)
    } else {
        select_314([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_698([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_27([c, e, f, g, a, b, d, h, i, j], is_dual)
    } else {
        select_699([a, b, c, d, h, i, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_703([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_55([a, c, b, d, e, f], is_dual)
    } else {
        select_85([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_702([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_222([a, c, d, f, e, g, h], is_dual)
    } else {
        select_703([a, e, b, g, f, d, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_701([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_330([a, d, b, c, f, e, h, i, g], is_dual)
    } else {
        select_702([a, e, d, b, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_706([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_61([a, c, e, h, d, g], is_dual)
    } else {
        select_61([b, c, d, h, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_102([d, e, c, g, f], is_dual)
    } else {
        select_706([a, b, f, d, e, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_707([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_89([a, b, h, e, d, i, g], is_dual)
    } else {
        select_290([b, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_704([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_705([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_707([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_700([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_701([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_704([a, c, b, e, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_697([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_698([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    } else {
        select_700([b, c, a, d, i, k, j, h, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_689([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_690([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_697([a, b, c, d, i, f, g, h, j, k, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_677([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_678([a, d, b, c, e, l, f, i, n, j, m], is_dual)
    } else {
        select_689([a, b, c, d, e, g, h, i, j, k, m, n, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_713([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_96([a, c, d, f, g, i], is_dual)
    } else {
        select_706([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_715([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_62([g, h, a, f, b], !is_dual)
    } else {
        select_38([e, c, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_716([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_55([a, c, b, f, g, e], is_dual)
    } else {
        select_55([a, d, b, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_714([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_715([a, d, b, f, h, e, i, g], is_dual)
    } else {
        select_716([a, e, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_712([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_713([c, d, b, e, f, a, h, g, i], is_dual)
    } else {
        select_714([c, d, a, e, f, b, h, g, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_717([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, b, c, d, e, g, h], is_dual)
    } else {
        select_49([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_711([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_712([a, b, c, d, j, h, i, k, l], is_dual)
    } else {
        select_717([b, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_719([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_47([a, c, g, d, e, f, h, i], is_dual)
    } else {
        select_66([a, b, c, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_718([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_719([a, b, h, d, e, f, g, i, j, k], is_dual)
    } else {
        select_47([a, c, i, d, e, f, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_710([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_711([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_718([b, a, d, e, f, g, h, i, c, j, k], is_dual)
    }
}
/// n = 9, i = 1
fn select_721([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_46([a, b, h, e, f, g, c, i], is_dual)
    } else {
        select_594([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_724([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_66([a, c, d, e, f, g], is_dual)
    } else {
        select_23([b, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_723([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_724([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_312([a, b, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_722([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_236([c, d, b, e, f, a, g, h, i], is_dual)
    } else {
        select_723([c, d, a, f, e, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_720([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_721([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_722([a, b, c, d, j, e, i, h, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_709([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_710([a, b, c, f, d, e, j, h, i, l, m, k], is_dual)
    } else {
        select_720([i, a, d, e, g, b, c, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_728([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_131([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_622([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_727([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_728([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_728([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_730([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_118([a, b, i, d, e, f, g, h, j, k], is_dual)
    } else {
        select_118([a, c, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_729([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_730([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_730([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_726([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_727([a, b, c, d, e, f, i, h, k, l, j], is_dual)
    } else {
        select_729([a, d, e, g, b, c, f, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_733([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_47([b, a, g, d, e, f, h, i], is_dual)
    } else {
        select_312([a, i, b, c, j, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_734([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_47([b, a, g, d, e, f, h, i], is_dual)
    } else {
        select_509([a, i, b, c, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_732([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_733([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_734([a, c, b, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_731([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_732([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_732([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 15, i = 3
fn select_725([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_726([a, b, c, e, f, l, g, j, k, n, m, o], is_dual)
    } else {
        select_731([a, b, c, d, h, i, m, j, o, l, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_708([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < l) || (is_dual && d > l) {
        select_709([a, b, c, f, g, l, i, j, d, k, n, m, o], is_dual)
    } else {
        select_725([a, b, c, e, f, g, i, d, h, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_676([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < m) || (is_dual && a > m) {
        select_677([b, c, d, e, f, h, g, k, j, l, m, a, n, o], is_dual)
    } else {
        select_708([b, c, d, e, f, a, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 1
fn select_740([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_171([a, b, c, d, e, g, h, i], is_dual)
    } else {
        select_276([a, c, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_742([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_155([a, b, c, i, j], is_dual)
    } else {
        select_127([d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_741([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_270([c, i, d, e, f, g, h, j], is_dual)
    } else {
        select_742([a, b, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_739([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_740([c, h, d, e, f, g, i, b, j], is_dual)
    } else {
        select_741([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_744([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_118([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_248([a, h, d, e, f, g, b, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_744([a, c, e, b, f, h, g, i, j, k], is_dual)
    } else {
        select_744([a, b, d, c, f, i, g, h, k, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_738([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_739([a, c, e, f, g, h, l, i, k, m, n], is_dual)
    } else {
        select_743([a, b, c, d, e, m, i, j, k, l, n], is_dual)
    }
}
/// n = 13, i = 3
fn select_747([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_269([a, j, d, e, f, g, h, k, m], is_dual)
    } else {
        select_126([b, c, h, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_746([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_747([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_747([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_750([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_207([a, b, d, e, f, h], is_dual)
    } else {
        select_205([b, c, d, e, g, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_749([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_295([a, b, e, f, g, h, j], is_dual)
    } else {
        select_750([a, b, c, d, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_748([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_360([c, d, f, a, b, h, g, i, j], is_dual)
    } else {
        select_749([a, b, c, e, d, g, h, j, i, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_745([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_746([a, b, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_748([a, b, c, d, l, e, j, i, m, k, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_737([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < l) || (is_dual && n > l) {
        select_738([c, d, e, g, h, a, b, f, i, j, k, m, l, n], is_dual)
    } else {
        select_745([a, b, c, d, g, e, f, k, i, j, m, n, l, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_753([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_744([c, d, f, a, b, e, g, h, i, j], is_dual)
    } else {
        select_606([a, b, c, d, e, h, g, j, k, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_752([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_600([a, b, c, f, g, h, k, i, l, n, m], is_dual)
    } else {
        select_753([a, b, c, d, l, e, i, j, m, k, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_751([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_752([a, b, c, d, g, e, f, k, i, j, m, n, l, o], is_dual)
    } else {
        select_738([c, d, e, g, h, a, b, f, i, j, k, m, l, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_736([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_737([b, c, a, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_751([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 3
fn select_758([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_347([c, b, d, e, f, g, a, h, j, i], is_dual)
    } else {
        select_249([c, a, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_757([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_247([c, a, b, e, f, g, d, h, i], is_dual)
    } else {
        select_758([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_759([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_247([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_247([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_756([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_757([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_759([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_762([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_171([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_558([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_761([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_762([a, c, h, e, f, g, b, i, j], is_dual)
    } else {
        select_594([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_760([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_757([a, d, b, c, e, f, h, j, k, i], is_dual)
    } else {
        select_761([a, d, e, g, b, c, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_755([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_756([a, d, b, c, e, g, k, m, j, l], is_dual)
    } else {
        select_760([a, b, c, d, f, j, h, i, l, k, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_766([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_118([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_347([b, h, d, e, f, g, a, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_765([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_766([a, b, d, c, e, i, g, h, k, j, l], is_dual)
    } else {
        select_744([b, c, f, a, e, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_764([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_743([a, b, d, e, g, f, c, h, i, j, k], is_dual)
    } else {
        select_765([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_768([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_605([a, b, c, d, f, h, g, j, i], is_dual)
    } else {
        select_744([a, d, e, b, c, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_769([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_766([a, b, e, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_605([b, c, d, a, f, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_767([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_768([a, d, e, b, f, g, c, h, i, j], is_dual)
    } else {
        select_769([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_763([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_764([a, b, c, f, h, k, g, j, i, m, l, n], is_dual)
    } else {
        select_767([a, c, b, d, e, h, l, j, k, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_754([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_755([a, b, c, e, d, f, i, h, j, l, m, k, n], is_dual)
    } else {
        select_763([a, d, e, b, c, f, h, g, j, i, k, m, l, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_735([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_736([a, c, d, e, g, f, h, i, b, k, j, m, n, l, o], is_dual)
    } else {
        select_754([a, c, d, b, e, g, f, i, l, k, m, j, n, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_675([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_676([a, b, c, d, e, f, h, l, i, j, m, k, n, o, p], is_dual)
    } else {
        select_735([a, b, c, d, e, g, h, k, i, j, m, l, o, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_674([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_675([a, b, e, f, c, i, g, d, h, j, m, k, l, o, p, n], is_dual)
    } else {
        select_675([a, b, e, f, d, i, h, c, g, j, m, l, k, n, p, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_673([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_674([i, f, e, g, b, c, d, h, a, l, k, n, j, o, m, p], is_dual)
    } else {
        select_674([i, f, e, h, b, c, d, g, a, l, k, m, j, o, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_672([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_673([j, g, h, i, f, a, d, e, b, n, m, c, k, l, o, p], is_dual)
    } else {
        select_673([j, g, h, i, f, c, d, e, b, n, m, a, k, l, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_671([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_672([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o, p], is_dual)
    } else {
        select_672([a, b, c, e, f, g, h, i, j, k, d, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < o) || (is_dual && m > o) {
        select_8([a, b, c, d, e, f, j, g, k, h, i, l, n, o, p, m], is_dual)
    } else {
        select_671([a, b, c, d, e, f, j, h, i, k, g, l, n, m, p, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_7([g, h, i, j, k, c, d, a, b, e, f, l, m, o, p, n], is_dual)
    } else {
        select_7([g, h, i, j, k, c, d, a, b, f, e, l, m, n, p, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_6([a, b, c, d, e, f, g, h, i, j, k, m, n, o, l, p], is_dual)
    } else {
        select_6([a, b, c, d, e, l, g, h, i, j, k, m, n, o, f, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_5([l, m, a, j, k, b, c, d, e, f, g, h, i, n, o, p], is_dual)
    } else {
        select_5([l, m, i, j, k, b, c, d, e, f, g, h, a, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_4([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o, p], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, i, k, l, m, n, j, o, p], is_dual)
    }
}
/// n = 13, i = 3
fn select_783([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_17([d, e, f, g, h, a, b, c, i, j, k], is_dual)
    } else {
        select_759([a, b, c, d, e, i, j, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_782([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_783([a, b, c, d, f, h, e, i, j, l, k, m, n], is_dual)
    } else {
        select_783([a, b, c, d, e, g, f, j, i, m, k, l, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_787([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_66([c, e, f, h, g, j], is_dual)
    } else {
        select_102([a, b, d, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_786([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_787([b, c, d, a, f, e, g, h, j, i, k], is_dual)
    } else {
        select_128([d, h, e, f, a, g, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_785([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_786([a, b, c, d, j, e, h, i, k, l, m], is_dual)
    } else {
        select_46([a, d, e, f, g, k, h, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_784([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_785([a, b, c, e, g, d, h, i, j, l, k, m, n], is_dual)
    } else {
        select_785([a, b, c, d, f, e, i, h, j, m, k, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_781([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_782([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_784([d, b, c, e, f, g, h, i, j, k, a, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_780([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_781([a, c, d, e, f, g, b, h, i, k, l, j, m, n], is_dual)
    } else {
        select_754([a, c, d, b, e, g, f, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_779([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_780([a, b, c, d, e, g, f, h, i, j, k, l, n, m], is_dual)
    } else {
        select_780([a, b, c, d, f, h, e, g, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_779([h, e, c, d, b, f, a, g, j, i, l, m, n, k], is_dual)
    } else {
        select_779([h, e, c, d, b, g, a, f, j, i, k, m, n, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_795([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_49([a, c, b, e, g, f, i], is_dual)
    } else {
        select_158([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_46([a, b, d, e, f, i, h, j], is_dual)
    } else {
        select_795([a, b, c, d, j, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_799([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_21([c, d, e, i], is_dual)
    } else {
        select_53([a, b, h, f, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_798([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_113([a, h, c, d, e, f, i, k], is_dual)
    } else {
        select_799([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_797([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_798([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_798([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_802([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_100([a, b, c, d, g, e, f], is_dual)
    } else {
        select_100([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_801([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_287([a, c, e, f, g, d, h, i], is_dual)
    } else {
        select_802([c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_800([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_801([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_801([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_796([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_797([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_800([a, b, i, d, c, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_793([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_794([c, d, e, f, a, b, g, h, j, i, k], is_dual)
    } else {
        select_796([a, b, c, e, d, h, g, j, k, i, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_803([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_701([b, a, f, c, h, e, j, i, g], is_dual)
    } else {
        select_591([b, a, d, g, i, f, h], is_dual)
    }
}
/// n = 14, i = 3
fn select_792([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_793([a, k, b, c, e, f, h, i, m, j, l, n], is_dual)
    } else {
        select_803([b, a, e, d, h, g, j, k, n, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_808([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_211([f, d, e, b, c, g, i, h, j], is_dual)
    } else {
        select_184([h, a, d, e, f, i, g, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_810([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_100([a, c, d, e, g, f, h], is_dual)
    } else {
        select_42([a, b, e, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_809([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_44([a, c, h, d, e, i, g, k], is_dual)
    } else {
        select_810([b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_808([b, c, f, d, e, h, i, g, j, k], is_dual)
    } else {
        select_809([a, b, g, d, e, h, f, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_811([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_642([a, c, b, j, f, g, h, i, k, l], is_dual)
    } else {
        select_249([a, c, d, e, i, f, g, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_807([c, d, a, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_811([c, d, b, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_805([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_806([b, a, c, e, d, i, g, h, j, k, m, l], is_dual)
    } else {
        select_64([b, c, d, f, a, j, g, i, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_815([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_500([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_146([a, b, i, f, e, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_815([c, g, a, b, f, j, h, l, k, i], is_dual)
    } else {
        select_128([i, c, e, d, f, g, k, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_818([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([d, b, c, e, f, h], is_dual)
    } else {
        select_53([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_817([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_155([e, c, d, h, g], is_dual)
    } else {
        select_818([a, g, b, e, f, i, j, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_816([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_724([i, d, a, e, f, g, l, j], is_dual)
    } else {
        select_817([d, g, b, c, f, j, h, k, l, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_813([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_814([c, d, a, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_816([e, c, d, b, f, a, g, h, i, j, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_812([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_813([b, a, e, f, d, g, h, j, k, i, m, l], is_dual)
    } else {
        select_251([a, b, c, i, e, f, l, g, j, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_804([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_805([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_812([a, b, c, e, d, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_791([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_792([c, b, a, f, e, d, i, h, g, j, k, l, n, m], is_dual)
    } else {
        select_804([c, a, d, b, f, h, j, g, i, k, m, n, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_824([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_52([e, b, c, d, f, g, h, i], is_dual)
    } else {
        select_100([a, c, f, e, g, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_826([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_61([a, c, g, f, d, i], is_dual)
    } else {
        select_53([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_825([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_826([b, a, c, e, f, i, h, j, g], is_dual)
    } else {
        select_826([b, a, d, e, f, i, g, j, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_823([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_824([c, d, e, a, f, b, g, h, i, j], is_dual)
    } else {
        select_825([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_828([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_222([a, b, f, d, e, g, h], is_dual)
    } else {
        select_222([a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_827([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_828([a, b, c, h, i, f, g, j], is_dual)
    } else {
        select_352([a, b, f, d, e, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_822([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_823([a, b, e, d, i, g, j, l, h, k], is_dual)
    } else {
        select_827([a, c, e, f, h, g, j, k, i, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_830([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_47([c, e, f, a, d, g, h, i], is_dual)
    } else {
        select_111([a, b, c, d, h, g, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_831([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_158([c, d, e, a, f, g, h], is_dual)
    } else {
        select_111([a, b, c, d, g, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_829([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_830([a, b, e, f, c, d, g, h, j, i, k], is_dual)
    } else {
        select_831([a, b, c, d, e, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_821([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_822([a, c, d, e, b, f, h, i, j, g, k, l], is_dual)
    } else {
        select_829([a, c, b, g, d, f, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_835([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([d, e, b, c, f, g, h], is_dual)
    } else {
        select_132([h, a, f, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_836([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_79([a, c, d, g, h, f, i], is_dual)
    } else {
        select_340([a, b, c, f, e, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_834([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_835([b, c, e, d, g, h, i, f, j], is_dual)
    } else {
        select_836([a, b, f, d, g, h, i, e, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_838([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_58([a, d, b, h, e, g], is_dual)
    } else {
        select_58([a, d, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_837([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_838([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_132([a, d, g, f, b], is_dual)
    }
}
/// n = 11, i = 3
fn select_833([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_834([a, d, c, b, e, g, h, f, i, j, k], is_dual)
    } else {
        select_837([g, b, d, e, h, j, i, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_842([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_55([a, e, b, d, g, f], is_dual)
    } else {
        select_38([a, c, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_841([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_500([b, d, a, c, e, f, g, i, h, j], is_dual)
    } else {
        select_842([a, d, c, f, h, i, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_844([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_100([a, b, f, d, g, e, h], is_dual)
    } else {
        select_100([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_843([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_66([a, g, d, e, i, j], is_dual)
    } else {
        select_844([b, a, c, f, h, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_841([a, d, i, c, h, g, f, l, j, k], is_dual)
    } else {
        select_843([b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_839([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_611([a, b, i, c, e, f, g, j, k, l], is_dual)
    } else {
        select_840([a, c, d, b, e, f, h, g, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_832([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_833([b, d, g, e, f, h, a, i, j, k, l], is_dual)
    } else {
        select_839([d, b, c, e, a, h, f, i, j, g, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_820([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_821([a, c, b, d, h, f, j, g, l, k, i, m], is_dual)
    } else {
        select_832([b, c, d, a, e, g, k, i, h, j, m, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_44([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_44([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_848([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_849([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_36([c, g, d, e, f, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_847([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_360([c, d, f, a, i, g, h, k, j], is_dual)
    } else {
        select_848([a, c, b, e, d, h, g, i, j, l, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_852([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_66([i, c, d, e, f, h], is_dual)
    } else {
        select_287([a, c, b, f, j, g, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_853([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_66([b, e, d, g, f, h], is_dual)
    } else {
        select_37([a, c, b, d, f, g, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_851([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_852([a, c, b, e, f, h, g, i, k, j, l], is_dual)
    } else {
        select_853([a, b, c, d, i, j, g, k, h, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_856([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_55([a, b, c, i, f, j], is_dual)
    } else {
        select_67([c, d, e, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_855([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_856([a, c, f, d, e, g, h, j, i, k], is_dual)
    } else {
        select_39([a, c, b, i, g, f, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_858([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_85([a, b, f, e, h, i], is_dual)
    } else {
        select_22([c, d, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_857([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_142([b, h, e, a, i], is_dual)
    } else {
        select_858([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_854([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_855([a, b, e, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_857([f, a, c, d, g, h, j, k, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_850([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_851([b, c, d, a, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_854([b, g, e, f, d, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_846([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_847([a, b, c, d, f, e, h, g, i, j, k, l], is_dual)
    } else {
        select_850([a, b, c, f, d, e, h, i, j, g, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_861([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_221([e, h, i, a, b, f, c], !is_dual)
    } else {
        select_509([b, e, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_862([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_253([a, b, c, d, g, h, f, i], is_dual)
    } else {
        select_253([a, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_860([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_861([a, b, c, g, h, f, j, i, k], is_dual)
    } else {
        select_862([b, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_865([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_66([a, c, d, e, f, g], is_dual)
    } else {
        select_228([b, a, f, d, e, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_867([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_42([b, c, d, e], is_dual)
    } else {
        select_56([a, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_866([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_867([a, b, h, d, i, f, g], is_dual)
    } else {
        select_41([a, g, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_864([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_865([c, b, d, a, e, f, g, h, i], is_dual)
    } else {
        select_866([c, a, d, b, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_869([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_77([a, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_39([a, d, c, h, g, e, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_870([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_163([a, b, c, h, e, g, j], is_dual)
    } else {
        select_517([b, g, d, e, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_868([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_869([c, e, b, d, a, f, g, h, i, j], is_dual)
    } else {
        select_870([c, a, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_863([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_864([a, c, b, d, h, f, j, i, g], is_dual)
    } else {
        select_868([a, b, c, e, d, f, h, g, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_860([a, b, d, f, e, g, h, k, i, j, l], is_dual)
    } else {
        select_863([a, b, c, d, j, g, i, f, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_845([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_846([b, d, c, a, h, f, i, g, j, k, m, l], is_dual)
    } else {
        select_859([b, a, d, e, c, j, h, g, i, l, m, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_819([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_820([a, b, d, e, f, g, h, j, l, k, i, m, n], is_dual)
    } else {
        select_845([b, a, d, c, f, i, h, j, k, g, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_790([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_791([b, c, a, e, f, d, i, j, h, l, k, m, n, g], is_dual)
    } else {
        select_819([b, c, e, a, d, f, i, g, h, j, l, m, n, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_789([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_790([d, e, b, c, f, a, i, h, k, g, l, m, j, n], is_dual)
    } else {
        select_790([d, f, b, c, e, a, i, h, j, g, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_878([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_207([a, e, b, d, g, h], is_dual)
    } else {
        select_66([b, c, a, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_877([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_295([a, f, d, e, g, h, j], is_dual)
    } else {
        select_878([a, b, c, h, f, i, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_880([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_653([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_653([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_879([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_880([b, e, a, d, g, f, i, h], is_dual)
    } else {
        select_258([a, b, f, c, h, e, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_876([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_877([b, a, f, d, e, g, i, h, k, j], is_dual)
    } else {
        select_879([b, a, h, c, j, f, k, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_882([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_422([a, d, e, b, c, f, g], is_dual)
    } else {
        select_510([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_884([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_48([b, c, d, g, e, h], is_dual)
    } else {
        select_818([b, h, a, e, f, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_883([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_504([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_884([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_881([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_882([b, h, c, a, e, i, g, j], is_dual)
    } else {
        select_883([b, a, d, c, g, f, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_875([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_876([a, b, d, c, e, g, h, i, j, f, k], is_dual)
    } else {
        select_881([a, b, c, d, e, g, f, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_887([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_204([b, c, d, a, e, g, f, h], is_dual)
    } else {
        select_878([b, c, f, d, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_886([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_187([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_887([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_890([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_289([i, a, b, e, f, h, j], is_dual)
    } else {
        select_228([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_891([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_229([f, a, c, d, e, g, i, h, j], is_dual)
    } else {
        select_229([e, b, c, d, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_889([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_890([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_891([b, c, d, e, a, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_893([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_500([a, e, b, c, g, f, h, j, i, k], is_dual)
    } else {
        select_169([a, i, e, d, f, g, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_894([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_58([i, a, b, e, f, h], is_dual)
    } else {
        select_228([a, c, d, e, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_892([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_893([b, d, e, a, c, g, f, h, i, j, k], is_dual)
    } else {
        select_894([f, c, d, e, a, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_888([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_889([a, b, g, d, e, h, i, f, j, k], is_dual)
    } else {
        select_892([b, a, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_885([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_886([i, a, c, d, f, b, j, h, k, l], is_dual)
    } else {
        select_888([a, b, e, c, d, h, g, i, k, l, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_874([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_875([b, a, c, f, j, m, h, i, g, l, k], is_dual)
    } else {
        select_885([a, b, d, e, f, g, h, k, i, l, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_873([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_874([a, c, b, d, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_874([b, c, a, d, e, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_900([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_40([c, f, d, e, h, g], is_dual)
    } else {
        select_41([a, b, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_899([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_900([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_66([c, d, e, f, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_898([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_899([a, b, c, d, e, h, i, g, j, k, l], is_dual)
    } else {
        select_899([a, b, c, d, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_902([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_207([e, h, c, d, g, i], is_dual)
    } else {
        select_207([e, i, a, b, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_901([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_902([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_429([b, c, i, a, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_897([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_898([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    } else {
        select_901([a, b, c, d, h, i, k, j, l, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_904([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_171([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_562([a, b, i, c, g, h, f, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_906([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_70([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([a, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_907([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_70([a, d, b, c, e, f, g], is_dual)
    } else {
        select_398([a, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_905([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_906([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_907([a, b, h, c, e, g, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_903([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_904([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_905([a, c, d, i, f, g, h, b, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_896([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_897([a, b, c, d, e, f, h, i, k, l, m, n], is_dual)
    } else {
        select_903([a, d, e, f, g, h, l, i, j, m, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_911([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_142([a, f, e, b, c], !is_dual)
    } else {
        select_189([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_910([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_221([a, b, d, e, f, g, h], is_dual)
    } else {
        select_911([e, b, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_909([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_203([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_910([b, c, d, g, f, a, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_908([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_909([k, a, c, d, b, g, l, h, j], is_dual)
    } else {
        select_208([c, d, e, f, b, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_895([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_896([a, c, d, e, f, b, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_908([a, b, c, d, e, j, h, k, m, l, n, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_872([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_873([e, f, g, b, c, d, h, i, l, m, k, j, n], is_dual)
    } else {
        select_895([e, f, b, c, d, a, g, h, i, j, k, n, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_918([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_155([e, c, d, f, g], is_dual)
    } else {
        select_106([a, b, g, e, h, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_917([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_122([b, e, c, d, f, g, h], is_dual)
    } else {
        select_918([a, g, c, d, f, e, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_916([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_525([f, a, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_917([f, j, a, b, g, h, e, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_920([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_51([a, c, d, b, e, f, g, i, h], is_dual)
    } else {
        select_397([a, g, b, e, h, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_919([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_920([a, b, c, i, g, h, f, j, k], is_dual)
    } else {
        select_247([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_915([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_916([a, h, e, f, c, d, g, i, k, j, l], is_dual)
    } else {
        select_919([a, b, d, c, j, g, i, h, l, m, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_924([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_54([b, c, d, a, e, f, h], is_dual)
    } else {
        select_54([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_923([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_397([b, e, c, a, g, f], is_dual)
    } else {
        select_924([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_922([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_247([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_923([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_925([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_594([e, f, a, b, c, d, g, h, i], is_dual)
    } else {
        select_376([f, e, i, a, b, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_921([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_922([a, c, b, d, e, i, h, k, l, j], is_dual)
    } else {
        select_925([b, h, f, g, e, a, c, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_914([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_915([a, c, b, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_921([g, a, b, c, i, e, f, h, j, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_913([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_914([a, c, b, d, e, f, i, h, j, k, g, l, m], is_dual)
    } else {
        select_914([b, c, a, d, e, f, i, g, j, k, h, l, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_930([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_142([b, e, d, a, f], is_dual)
    } else {
        select_85([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_929([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_247([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_930([a, c, h, g, i, b], is_dual)
    }
}
/// n = 12, i = 3
fn select_928([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_297([a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_929([a, b, c, d, h, i, g, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_927([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_928([a, b, c, d, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_928([a, c, b, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_935([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_66([a, c, d, e, f, g], is_dual)
    } else {
        select_48([b, a, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_934([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_47([a, c, d, e, f, g, i, h], is_dual)
    } else {
        select_935([a, b, h, c, d, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_933([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_65([d, i, e, f, a, g, h, j], is_dual)
    } else {
        select_934([a, b, c, h, e, f, g, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_932([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_933([c, d, e, f, a, h, g, i, j, k, l], is_dual)
    } else {
        select_919([a, b, d, c, i, g, j, h, l, m, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_931([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_932([a, b, c, d, e, f, i, h, j, k, g, l, m], is_dual)
    } else {
        select_932([b, a, c, d, e, f, i, g, j, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_926([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_927([i, a, b, c, f, j, g, h, k, d, l, m], is_dual)
    } else {
        select_931([a, b, c, e, d, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_912([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_913([d, e, f, a, b, c, i, j, h, g, l, k, m], is_dual)
    } else {
        select_926([d, e, b, c, a, f, i, j, h, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_871([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_872([c, a, b, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    } else {
        select_912([h, a, b, e, f, g, c, m, i, j, k, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_788([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_789([a, b, c, d, e, f, h, i, j, k, l, m, g, n], is_dual)
    } else {
        select_871([a, g, b, c, d, e, f, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_778([a, b, c, g, d, e, f, h, i, k, l, m, j, n], is_dual)
    } else {
        select_788([a, b, c, d, e, f, h, i, j, k, l, m, g, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_777([a, b, c, d, e, f, g, h, j, k, l, m, i, n], is_dual)
    } else {
        select_777([a, b, c, d, e, i, g, h, j, k, l, m, f, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_775([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_776([j, i, a, g, h, b, c, d, e, m, f, k, l, n], is_dual)
    } else {
        select_776([j, i, f, g, h, b, c, d, e, m, a, k, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_774([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_775([a, b, c, d, e, f, g, i, j, k, h, l, m, n], is_dual)
    } else {
        select_775([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_948([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_155([i, d, e, f, h], is_dual)
    } else {
        select_96([a, b, c, j, k, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_949([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_21([c, d, e, g], is_dual)
    } else {
        select_67([h, a, b, i, f], is_dual)
    }
}
/// n = 12, i = 3
fn select_947([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_948([a, b, g, d, e, f, h, i, j, k, l], is_dual)
    } else {
        select_949([c, h, d, e, f, g, j, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_952([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_67([b, d, e, h, i], is_dual)
    } else {
        select_53([a, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_951([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_952([b, d, c, a, e, f, g, h, i, j], is_dual)
    } else {
        select_42([d, i, h, a], is_dual)
    }
}
/// n = 13, i = 3
fn select_950([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_269([a, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_951([a, b, k, c, d, h, j, i, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_946([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_947([a, j, c, e, f, g, i, h, k, l, m, n], is_dual)
    } else {
        select_950([b, c, h, d, e, f, g, i, l, j, k, n, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_955([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_70([b, c, d, e, h, f, g], is_dual)
    } else {
        select_52([a, b, g, c, f, h, e, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_956([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_228([b, a, c, d, e, f, g], is_dual)
    } else {
        select_290([b, f, d, e, a, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_954([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_955([b, c, a, d, f, e, g, h, i], is_dual)
    } else {
        select_956([a, b, g, f, h, i, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_957([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_118([a, c, h, d, e, f, g, i, j, k], is_dual)
    } else {
        select_906([a, b, c, j, g, h, k, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_953([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_954([a, b, j, c, h, g, i, l, m], is_dual)
    } else {
        select_957([b, a, i, d, e, f, g, h, k, j, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_945([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_946([b, a, c, d, e, f, g, h, j, i, k, l, m, n], is_dual)
    } else {
        select_953([b, i, c, e, f, g, a, h, j, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_960([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_158([a, b, e, d, f, g, h], is_dual)
    } else {
        select_314([a, c, b, d, g, f, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_962([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_127([b, c, d, e, f], is_dual)
    } else {
        select_56([a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_961([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_269([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_962([g, c, d, e, f, a, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_959([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_960([a, b, d, k, c, h, i, l, j, m], is_dual)
    } else {
        select_961([a, d, e, f, g, j, h, k, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_964([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_956([a, c, b, f, h, g, e], is_dual)
    } else {
        select_128([b, c, d, a, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_966([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_52([c, a, d, b, e, f, g, h], is_dual)
    } else {
        select_54([b, f, d, e, a, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_967([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_335([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_335([a, b, c, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_965([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_966([a, b, d, c, e, f, h, g, i], is_dual)
    } else {
        select_967([a, b, d, g, f, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_963([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_964([a, b, e, d, c, f, g, h], is_dual)
    } else {
        select_965([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_958([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_959([a, b, c, d, e, f, g, i, h, j, k, m, l], is_dual)
    } else {
        select_963([a, i, b, k, c, h, m, l, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_944([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_945([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_958([a, b, j, d, e, f, g, k, i, h, l, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_943([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_944([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_944([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_972([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_509([a, e, b, d, g, f, h], is_dual)
    } else {
        select_95([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_971([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_972([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_972([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_973([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_311([b, g, c, d, a, i, h], is_dual)
    } else {
        select_883([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_970([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_971([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_973([c, b, a, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_978([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_67([a, c, d, e, h], is_dual)
    } else {
        select_53([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_977([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_978([b, f, c, d, e, g, i, h], is_dual)
    } else {
        select_134([h, b, a, e, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_979([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_37([h, b, c, d, e, f, i, k], is_dual)
    } else {
        select_198([f, a, c, d, e, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_976([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_977([a, b, c, j, f, h, i, g, k, l], is_dual)
    } else {
        select_979([a, b, d, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_975([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_311([g, i, a, b, f, k, j], is_dual)
    } else {
        select_976([c, e, d, a, b, g, f, h, i, j, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_982([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_517([a, f, b, e, g, i, j], is_dual)
    } else {
        select_844([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_981([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_326([b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_982([b, a, d, g, f, h, e, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_980([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_981([h, d, c, a, j, f, e, g, l, i, k], is_dual)
    } else {
        select_981([g, c, d, b, i, e, f, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_974([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_975([a, b, c, e, d, h, g, i, j, f, l, k], is_dual)
    } else {
        select_980([b, c, d, e, g, h, f, i, a, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_969([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_970([a, b, h, g, d, k, i, j, f, l], is_dual)
    } else {
        select_974([a, b, d, c, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_968([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_969([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_969([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_942([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < d) || (is_dual && l > d) {
        select_943([a, b, c, e, f, d, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_968([a, b, c, f, e, l, i, d, j, k, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_989([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_167([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_818([g, c, b, a, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_988([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_122([c, h, d, e, a, g, i], is_dual)
    } else {
        select_989([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_991([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([b, c, d, e, g, h, i], is_dual)
    } else {
        select_978([a, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_990([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_991([a, c, d, b, j, g, h, i, k, l], is_dual)
    } else {
        select_171([c, d, e, f, k, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_987([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_988([a, c, b, i, j, h, g, l, m, k, n], is_dual)
    } else {
        select_990([c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_994([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_155([a, d, e, h, g], is_dual)
    } else {
        select_163([a, b, c, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_993([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_122([c, h, d, e, a, g, i], is_dual)
    } else {
        select_994([a, b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_992([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_993([a, c, b, i, j, h, g, l, m, k], is_dual)
    } else {
        select_990([c, f, d, e, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_986([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_987([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_992([b, a, c, e, f, d, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_985([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_986([a, c, d, e, f, g, b, h, i, j, k, l, m, n], is_dual)
    } else {
        select_986([b, c, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_998([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_906([a, h, d, e, f, b, i, k], is_dual)
    } else {
        select_650([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_997([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_998([a, b, c, i, j, g, h, l, m, k, n], is_dual)
    } else {
        select_990([c, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1002([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_143([a, g, h], is_dual)
    } else {
        select_53([a, c, b, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1001([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1002([e, d, c, a, f, h, g, i], is_dual)
    } else {
        select_340([a, c, b, d, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1000([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1001([a, c, b, i, f, g, h, j, k], is_dual)
    } else {
        select_314([a, c, d, e, h, f, i, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_999([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1000([a, c, b, i, j, g, h, l, m, k, n], is_dual)
    } else {
        select_990([c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_996([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_997([a, b, c, d, e, f, g, i, j, k, h, l, m, n], is_dual)
    } else {
        select_999([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_995([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_996([a, c, d, e, f, g, b, h, i, j, k, l, m, n], is_dual)
    } else {
        select_996([b, c, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_984([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_985([a, b, c, e, f, g, h, d, i, j, k, l, m, n], is_dual)
    } else {
        select_995([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_1007([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_207([a, e, d, g, f, b], !is_dual)
    } else {
        select_58([a, c, b, e, d, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1006([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1007([f, e, i, a, b, h, g], !is_dual)
    } else {
        select_510([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1005([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1006([b, c, i, d, g, a, h, j, k], is_dual)
    } else {
        select_625([c, b, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1004([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1005([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_1005([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1003([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1004([e, f, g, d, a, b, j, h, i, k, l], is_dual)
    } else {
        select_1004([e, f, g, d, a, c, i, h, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_983([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_984([a, b, c, d, f, e, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1003([g, h, l, d, a, b, c, j, k, e, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_941([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_942([a, b, c, d, g, f, e, k, i, j, m, n, l, o], is_dual)
    } else {
        select_983([a, b, c, d, e, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_1014([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_204([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_158([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_1013([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_594([e, f, a, b, c, d, h, i, j], is_dual)
    } else {
        select_1014([g, e, f, j, a, b, h, i, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1016([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_47([b, a, g, d, e, f, h, i], is_dual)
    } else {
        select_66([a, c, b, i, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1018([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_58([a, c, d, f, e, h], is_dual)
    } else {
        select_55([b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1017([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_219([b, d, e, f, h, i], is_dual)
    } else {
        select_1018([a, b, i, c, g, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1015([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1016([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1017([a, c, b, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1012([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1013([c, i, f, g, d, e, a, h, k, j, l], is_dual)
    } else {
        select_1015([a, b, c, d, e, j, h, i, l, m, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1020([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_586([a, f, b, c, d, e, g, i, h, j, k, l], is_dual)
    } else {
        select_665([h, b, c, d, e, g, f, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1019([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1020([e, c, i, f, g, d, a, j, h, l, k, m], is_dual)
    } else {
        select_1015([a, b, c, d, j, k, h, i, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1011([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1012([a, b, c, d, e, f, j, h, i, m, l, k, n], is_dual)
    } else {
        select_1019([a, b, c, f, g, d, e, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_1025([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_810([c, d, a, b, e, f, h, g], is_dual)
    } else {
        select_100([c, a, b, e, h, g, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_1024([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_278([a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1025([b, j, a, c, h, g, k, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1027([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_96([b, d, e, f, a, h], is_dual)
    } else {
        select_716([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1026([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_276([c, d, e, f, h, g, i], is_dual)
    } else {
        select_1027([a, b, c, i, g, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1023([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1024([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1026([a, i, b, d, e, f, h, g, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1022([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1013([f, g, i, j, b, c, a, h, k, l, m], is_dual)
    } else {
        select_1023([a, d, e, b, c, k, h, i, j, m, n, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1028([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1020([e, f, g, i, j, b, a, k, h, l, m, n], is_dual)
    } else {
        select_1023([a, c, d, b, k, l, h, i, j, n, o, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_1021([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1022([a, b, c, e, f, d, k, h, i, j, m, n, l, o], is_dual)
    } else {
        select_1028([a, d, e, f, g, b, c, h, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1010([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_1011([a, j, b, c, d, e, g, h, l, k, m, i, n, o], is_dual)
    } else {
        select_1021([a, c, d, e, b, f, g, h, i, j, k, m, n, l, o], is_dual)
    }
}
/// n = 9, i = 1
fn select_1033([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_47([a, b, h, d, e, f, g, i], is_dual)
    } else {
        select_47([a, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 12, i = 2
fn select_1032([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_594([b, c, f, g, a, h, i, j, k], is_dual)
    } else {
        select_1033([a, d, e, b, c, j, h, i, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_1034([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_18([d, e, f, g, a, h, i, j, k, l, m], is_dual)
    } else {
        select_1033([a, b, c, j, k, l, h, i, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_1031([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1032([c, a, b, f, g, h, i, j, k, m, n, l], is_dual)
    } else {
        select_1034([c, f, g, d, e, a, b, j, k, h, i, l, n, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1036([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_27([b, e, f, g, a, h, i, j, k, l], is_dual)
    } else {
        select_1033([a, c, d, b, j, k, h, i, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1035([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1032([d, a, b, f, g, c, h, i, j, l, m, k], is_dual)
    } else {
        select_1036([d, c, f, g, e, a, b, i, j, h, k, m, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_1030([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < h) || (is_dual && m > h) {
        select_1031([b, c, d, a, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1035([b, c, a, d, i, f, g, m, j, k, l, h, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1041([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_660([b, a, c, f, g, e, i, h], is_dual)
    } else {
        select_39([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_962([b, d, e, f, i, h, j, k], is_dual)
    } else {
        select_1041([b, a, j, c, h, g, k, l, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_1039([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1040([g, b, a, e, f, j, i, h, l, k, m, n], is_dual)
    } else {
        select_1040([g, b, a, c, d, k, i, h, l, j, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_1044([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_70([c, d, a, e, f, g, h], is_dual)
    } else {
        select_66([a, b, f, g, e, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1045([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_70([c, d, e, f, g, h, i], is_dual)
    } else {
        select_70([a, b, g, h, e, f, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1043([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_1044([a, f, c, d, i, g, h, e, j], is_dual)
    } else {
        select_1045([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1042([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1043([a, d, b, e, f, g, c, h, i, j], is_dual)
    } else {
        select_1043([a, d, c, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1038([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_1039([e, b, c, d, f, g, a, i, h, j, k, l, m, n], is_dual)
    } else {
        select_1042([b, c, d, e, k, i, h, m, l, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1048([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_167([b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_77([a, h, b, c, g, f, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1047([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_118([f, a, b, c, d, k, h, i, j, m], is_dual)
    } else {
        select_1048([f, e, j, a, b, h, g, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1046([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_203([a, b, c, i, k, g, l, j], is_dual)
    } else {
        select_1047([b, c, e, f, d, a, h, g, j, k, i, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1037([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1038([a, b, c, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    } else {
        select_1046([b, c, d, h, f, g, i, j, a, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1029([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_1030([b, c, d, e, f, a, g, j, i, h, k, m, l, n], is_dual)
    } else {
        select_1037([a, b, c, d, f, e, k, i, j, l, m, n, h, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1009([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1010([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    } else {
        select_1029([b, a, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1008([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1009([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    } else {
        select_1009([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_940([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_941([a, b, c, e, g, h, f, i, j, k, l, m, d, n, o], is_dual)
    } else {
        select_1008([a, b, c, e, d, f, g, h, i, j, l, m, k, n, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_1057([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_70([a, d, b, c, e, f, g], is_dual)
    } else {
        select_38([g, a, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1056([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_171([a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_1057([a, b, i, c, f, h, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1055([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1056([c, d, e, a, h, f, g, i, j, k], is_dual)
    } else {
        select_882([f, g, a, b, i, h, k, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1059([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_171([b, d, a, e, g, f, h, i], is_dual)
    } else {
        select_158([a, f, c, e, h, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1058([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1059([a, c, b, d, i, f, g, h, j, k], is_dual)
    } else {
        select_70([c, d, e, j, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1054([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1055([b, d, c, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_1058([b, d, e, a, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1063([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_58([d, f, b, h, e, i], is_dual)
    } else {
        select_54([a, e, c, d, g, f, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1062([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_520([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1063([i, b, c, a, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1061([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_882([a, i, b, e, g, h, k, j], is_dual)
    } else {
        select_1062([a, c, d, b, h, f, g, i, j, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1066([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_169([d, e, a, g, f, h, i], is_dual)
    } else {
        select_169([b, c, a, h, f, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1065([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1066([e, a, b, c, d, g, i, j, k, l], is_dual)
    } else {
        select_168([c, d, e, f, g, h, k, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1069([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_40([c, d, e, f, g, h], is_dual)
    } else {
        select_22([a, b, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1068([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1069([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_320([a, b, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1067([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1068([a, b, f, c, d, g, h, j, i, k], is_dual)
    } else {
        select_110([a, b, e, i, g, f, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1064([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1065([b, c, a, d, f, e, h, g, i, j, k, l], is_dual)
    } else {
        select_1067([b, c, d, g, f, h, i, j, a, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1060([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1061([a, d, c, e, f, g, h, i, b, j, k, l], is_dual)
    } else {
        select_1064([a, b, c, d, f, e, i, h, g, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1053([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1054([a, d, c, f, e, h, g, i, j, l, k], is_dual)
    } else {
        select_1060([a, g, b, d, c, f, h, k, i, l, j, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1074([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_155([b, d, e, h, g], is_dual)
    } else {
        select_145([a, b, c, g, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1073([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_122([c, h, d, e, b, g, i], is_dual)
    } else {
        select_1074([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1076([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([b, c, d, e, i, g, h], is_dual)
    } else {
        select_858([a, b, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1076([f, i, a, d, e, h, g, j, k, l, m], is_dual)
    } else {
        select_28([a, d, e, b, c, k, g, j, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_1072([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1073([a, b, c, i, k, g, h, j, m, l, n], is_dual)
    } else {
        select_1075([d, b, h, e, f, a, i, g, l, k, j, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_1079([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_66([g, b, c, d, f, h], is_dual)
    } else {
        select_321([a, b, e, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1078([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_122([b, c, d, e, g, h, i], is_dual)
    } else {
        select_1079([a, d, b, c, f, i, g, h, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1080([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_422([c, d, e, b, f, g, h], is_dual)
    } else {
        select_56([a, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1077([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1078([a, b, g, c, d, f, h, k, i, j, l], is_dual)
    } else {
        select_1080([a, b, e, i, f, g, j, h, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1071([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < g) || (is_dual && m > g) {
        select_1072([a, b, d, c, e, f, h, i, g, j, k, l, m, n], is_dual)
    } else {
        select_1077([a, b, c, k, d, h, i, j, m, g, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1070([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1071([b, c, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1072([b, c, e, a, f, g, h, i, d, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1052([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_1053([a, c, d, e, i, g, b, h, j, k, l, m, n], is_dual)
    } else {
        select_1070([a, c, e, d, g, b, f, h, j, k, i, m, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1086([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_222([a, c, d, e, f, g, h], is_dual)
    } else {
        select_320([a, b, g, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1085([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1086([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_371([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1087([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_214([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_320([a, b, h, i], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1084([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1085([a, b, c, h, g, i, f, j], is_dual)
    } else {
        select_1087([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1089([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_225([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_225([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1091([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_183([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_211([g, c, d, e, f, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_1090([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_49([a, d, f, g, k, h, j], is_dual)
    } else {
        select_1091([a, d, b, c, e, j, h, i, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1088([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_1089([a, b, d, e, c, h, j, l, k, i, m], is_dual)
    } else {
        select_1090([c, d, e, f, g, a, b, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1083([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_1084([a, b, k, f, g, c, i, j, m, l], is_dual)
    } else {
        select_1088([a, b, d, f, g, e, c, h, i, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1093([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_439([a, b, d, e, c, h, i, j, l, m], is_dual)
    } else {
        select_1033([c, f, g, a, b, j, h, i, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1092([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1036([d, c, f, g, e, a, b, i, j, h, k, m, l], is_dual)
    } else {
        select_1093([a, b, d, c, h, f, g, i, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1082([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1083([b, c, a, e, g, d, i, j, k, l, m, h, n], is_dual)
    } else {
        select_1092([b, c, e, d, g, a, f, j, h, i, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1081([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1082([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    } else {
        select_1082([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1051([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_1052([a, b, d, e, f, g, h, i, j, k, c, l, m, n], is_dual)
    } else {
        select_1081([a, b, c, d, e, f, g, h, j, k, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1050([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1051([a, b, c, d, e, g, f, h, i, k, l, j, m, n], is_dual)
    } else {
        select_1051([a, b, c, d, f, g, e, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 12, i = 2
fn select_1099([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_586([a, g, b, c, d, e, f, i, h, j, k, l], is_dual)
    } else {
        select_117([h, b, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1098([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_648([a, b, d, c, j, k, h, i, m, l, n], is_dual)
    } else {
        select_1099([e, c, h, f, g, d, a, j, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1100([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1099([e, b, h, f, g, c, a, j, i, l, k, m], is_dual)
    } else {
        select_657([a, b, c, d, j, k, i, h, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1097([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1098([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_1100([a, b, d, c, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_1105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_113([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_113([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1104([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1105([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_1105([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_70([b, c, d, e, f, g, h], is_dual)
    } else {
        select_23([a, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1106([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_171([b, c, d, e, i, g, h, j], is_dual)
    } else {
        select_1107([a, b, c, j, f, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_1103([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1104([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_1106([b, c, d, e, f, g, h, i, a, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1102([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1103([a, c, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_657([a, b, c, d, i, j, h, g, l, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1108([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_648([a, b, d, c, i, j, g, h, l, k, m], is_dual)
    } else {
        select_1103([a, d, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1102([a, b, d, c, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_1108([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1096([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_1097([a, b, c, d, f, e, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1101([a, b, c, d, f, l, h, i, j, k, e, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1095([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1096([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_1096([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_1115([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_102([b, c, d, e, f], is_dual)
    } else {
        select_189([a, b, c, f, e, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_348([e, f, c, d, g, i, h, j, k], is_dual)
    } else {
        select_348([e, f, a, b, h, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1115([b, c, d, i, g, j, h], is_dual)
    } else {
        select_1116([c, d, e, f, a, b, h, i, g, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1119([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_55([a, e, b, d, g, h], is_dual)
    } else {
        select_23([c, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1118([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1119([b, a, c, g, e, i, h, f], is_dual)
    } else {
        select_189([b, a, f, d, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1120([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_37([a, c, d, e, g, h, i, j], is_dual)
    } else {
        select_340([a, b, c, i, f, h, g, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1117([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1118([a, b, c, i, f, h, j, k, g], is_dual)
    } else {
        select_1120([a, c, b, d, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1114([b, c, a, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1117([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1123([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_254([a, b, e, f, h, d], is_dual)
    } else {
        select_254([a, c, d, f, g, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_52([b, a, d, c, e, f, g, h], is_dual)
    } else {
        select_179([a, e, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1123([b, a, c, g, h, f, e, i], is_dual)
    } else {
        select_1124([a, c, b, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_52([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_327([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_1127([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_320([a, b, d, e], is_dual)
    } else {
        select_67([c, a, b, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1125([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1126([a, c, d, b, e, f, h, g, i], is_dual)
    } else {
        select_1127([b, g, c, f, h, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1121([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1122([c, b, d, e, a, f, g, h, i], is_dual)
    } else {
        select_1125([c, a, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1113([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1121([a, b, c, d, i, h, j, k, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1112([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_1112([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_564([a, b, c, h, e, f, g, i, j, k, l], is_dual)
    } else {
        select_1111([a, b, c, d, i, f, g, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1133([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_79([e, c, a, d, g, h, f], is_dual)
    } else {
        select_134([a, b, d, e, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1134([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_89([h, f, e, g, d, a, b], !is_dual)
    } else {
        select_89([h, g, d, f, e, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1133([b, c, a, d, e, f, g, i, h], is_dual)
    } else {
        select_1134([b, c, f, e, a, i, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1132([a, b, c, g, f, h, i, e, j], is_dual)
    } else {
        select_328([a, c, b, d, f, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_203([b, c, d, a, g, f, i, h], is_dual)
    } else {
        select_346([a, b, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1131([b, c, e, d, f, g, h, a, i, j], is_dual)
    } else {
        select_1135([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1139([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_226([b, c, d, e, a, g, h, i, j, k, l], is_dual)
    } else {
        select_184([f, a, b, c, g, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1138([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1139([d, b, c, e, f, a, g, h, i, j, l, k], is_dual)
    } else {
        select_529([b, c, j, e, f, i, h, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1140([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_808([b, a, g, d, e, f, h, j, i, l], is_dual)
    } else {
        select_128([a, f, i, c, h, g, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1137([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1138([c, a, b, d, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_1140([c, b, d, e, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_1143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_20([a, c, g, d, e, f, i, h], is_dual)
    } else {
        select_66([a, b, c, h, g, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1142([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1143([c, d, g, e, f, i, h, j, k, l], is_dual)
    } else {
        select_787([a, b, c, j, d, g, h, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1144([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_158([a, i, c, f, j, h, l], is_dual)
    } else {
        select_166([b, d, e, a, f, h, g, i, j, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1141([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1142([a, b, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_1144([c, b, d, e, f, g, h, i, a, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1136([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1137([a, c, b, d, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_1141([a, c, d, b, f, g, e, i, h, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1129([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1130([a, b, c, d, k, i, l, j, h, m], is_dual)
    } else {
        select_1136([a, c, b, d, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1128([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1129([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_1129([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1109([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_1110([a, b, c, e, j, g, h, i, d, k, m, l], is_dual)
    } else {
        select_1128([a, b, c, e, f, d, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1094([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_1095([a, b, c, e, f, g, d, h, j, k, i, l, m, n], is_dual)
    } else {
        select_1109([a, b, c, d, e, f, g, l, j, k, m, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1049([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_1050([a, b, c, e, d, g, h, i, j, f, k, l, m, n], is_dual)
    } else {
        select_1094([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_939([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_940([a, b, c, d, f, h, e, i, g, k, l, j, m, n, o], is_dual)
    } else {
        select_1049([a, b, c, d, k, e, g, f, h, m, j, l, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_938([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_939([a, b, c, d, e, g, f, h, i, k, l, m, n, j, o], is_dual)
    } else {
        select_939([a, b, c, d, f, g, e, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_937([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_938([a, b, c, d, e, g, h, i, j, f, k, l, m, n, o], is_dual)
    } else {
        select_938([a, b, c, d, f, g, h, i, j, e, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_936([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_937([a, b, c, d, e, f, g, i, j, k, h, l, m, n, o], is_dual)
    } else {
        select_937([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_773([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_774([a, b, c, d, e, f, g, h, m, i, j, o, n, l], is_dual)
    } else {
        select_936([a, b, c, d, e, f, g, h, j, i, k, m, l, o, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1149([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_778([c, d, a, b, e, f, g, h, i, k, l, m, n, j], is_dual)
    } else {
        select_778([d, c, a, b, e, f, g, h, j, k, l, m, n, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_1148([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1149([a, b, c, d, e, f, g, h, j, k, l, m, i, n], is_dual)
    } else {
        select_1149([a, b, c, d, e, f, i, h, j, k, l, m, g, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_1159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_907([a, c, d, g, f, b, h, i, j], is_dual)
    } else {
        select_370([a, c, d, b, e, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1160([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_222([a, b, f, d, g, e, h], is_dual)
    } else {
        select_327([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1158([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1159([a, b, e, i, d, f, h, j, k, l], is_dual)
    } else {
        select_1160([a, k, c, f, l, g, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1157([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1158([a, b, c, d, i, f, g, h, j, k, l, m], is_dual)
    } else {
        select_360([a, b, d, e, k, f, h, i, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1163([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_37([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_703([b, g, c, a, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1162([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_122([c, f, d, e, a, g, h], is_dual)
    } else {
        select_1163([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_906([a, g, c, d, e, i, h, j], is_dual)
    } else {
        select_512([a, h, b, e, j, f, k, g, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_1161([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1162([k, a, c, d, e, f, h, j, l, m], is_dual)
    } else {
        select_1164([a, b, d, e, f, g, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1156([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_1157([a, b, c, e, h, f, g, i, k, j, l, m, n], is_dual)
    } else {
        select_1161([a, c, d, b, i, f, g, h, j, m, l, k, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1169([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_142([a, b, d, e, i], is_dual)
    } else {
        select_331([d, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_867([a, c, d, e, f, g, h], is_dual)
    } else {
        select_41([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1172([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([a, b, c, e, h, i], is_dual)
    } else {
        select_67([d, b, c, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1170([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1171([a, h, b, e, f, i, g, j], is_dual)
    } else {
        select_1172([a, c, d, f, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1168([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1169([b, f, d, e, a, g, h, i, j], is_dual)
    } else {
        select_1170([b, d, a, c, e, g, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1168([b, a, d, e, g, i, f, j, h, k], is_dual)
    } else {
        select_1168([a, b, c, e, h, i, f, j, g, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_79([a, c, d, h, g, f, j], is_dual)
    } else {
        select_77([a, c, b, f, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_869([b, d, c, e, a, f, g, h, i, j], is_dual)
    } else {
        select_1175([b, d, e, a, f, c, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1177([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_44([a, c, j, e, f, i, h, l], is_dual)
    } else {
        select_211([b, e, f, d, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1177([b, c, d, a, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_36([b, i, d, e, f, a, h, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1173([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_1174([a, b, c, d, j, g, k, i, h, l], is_dual)
    } else {
        select_1176([b, a, d, c, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1166([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1167([a, b, c, j, d, g, h, l, m, i, k], is_dual)
    } else {
        select_1173([a, b, c, g, e, f, i, h, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1181([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_327([a, c, d, f, g, e, h], is_dual)
    } else {
        select_212([a, b, c, e, f, h, i, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1180([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1181([a, b, j, c, g, h, k, i, l], is_dual)
    } else {
        select_138([a, b, d, e, f, i, h, j, l, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1179([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1104([a, b, e, f, d, k, h, i, j, l, m], is_dual)
    } else {
        select_1180([a, b, c, d, h, i, g, j, k, m, n, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1183([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_48([c, d, e, g, f, h], is_dual)
    } else {
        select_169([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1183([d, e, a, b, h, f, i, g, j], is_dual)
    } else {
        select_1183([d, e, a, c, g, f, i, h, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1178([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < c) || (is_dual && l > c) {
        select_1179([a, b, d, c, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1182([e, f, l, a, b, h, i, c, j, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1165([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < m) || (is_dual && g > m) {
        select_1166([b, a, c, e, d, i, h, j, k, m, l, g, n], is_dual)
    } else {
        select_1178([b, c, d, e, a, f, h, g, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1155([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_1156([a, d, e, b, f, g, i, h, j, k, c, l, m, n], is_dual)
    } else {
        select_1165([a, b, c, d, e, f, g, i, j, h, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_1189([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_70([c, d, a, e, g, f, h], is_dual)
    } else {
        select_66([a, b, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1188([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1189([a, b, h, d, c, g, f, i, j], is_dual)
    } else {
        select_69([a, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1187([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1188([b, c, d, a, e, g, f, h, i, j], is_dual)
    } else {
        select_704([b, a, c, h, g, j, i, f, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1192([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_70([a, b, c, d, e, f, g], is_dual)
    } else {
        select_41([a, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1191([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_906([b, a, d, e, f, c, g, h], is_dual)
    } else {
        select_1192([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1190([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_209([a, b, d, c, e, f, g, h, j, i], is_dual)
    } else {
        select_1191([b, a, c, e, g, f, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1186([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1187([a, b, d, g, e, f, h, i, k, j, l], is_dual)
    } else {
        select_1190([a, b, c, d, j, f, h, g, l, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_1197([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_143([a, e, f], is_dual)
    } else {
        select_67([a, b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1196([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_145([a, b, d, f, e, g, i, h], is_dual)
    } else {
        select_1197([a, c, g, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1199([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_55([a, c, d, f, e, h], is_dual)
    } else {
        select_55([b, d, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1198([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1199([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_39([b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1195([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1196([c, d, b, e, f, a, g, i, h], is_dual)
    } else {
        select_1198([c, d, a, e, f, b, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1201([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_89([h, g, a, e, f, b, d], !is_dual)
    } else {
        select_139([a, c, d, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_1202([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_142([f, d, b, a, e], !is_dual)
    } else {
        select_55([b, c, a, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1200([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1201([a, d, c, b, f, e, h, g], is_dual)
    } else {
        select_1202([d, a, e, f, b, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1194([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1195([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_1200([a, c, b, d, g, h, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1205([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_207([a, d, c, e, f, h], is_dual)
    } else {
        select_703([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1204([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_955([b, c, a, d, f, e, g, h, i], is_dual)
    } else {
        select_1205([a, b, g, h, f, i, e, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1208([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_55([d, b, c, e, f, g], is_dual)
    } else {
        select_59([a, b, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1207([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_142([b, f, d, a, h], is_dual)
    } else {
        select_1208([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1206([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_642([a, b, d, c, e, f, g, i, h, j], is_dual)
    } else {
        select_1207([a, f, d, h, g, i, e, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1203([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1204([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_1206([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1193([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1194([a, c, b, d, i, f, k, g, j], is_dual)
    } else {
        select_1203([a, b, c, g, e, f, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1185([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1186([a, c, b, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_1193([a, b, h, c, e, f, j, i, k, g, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1213([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_58([g, b, c, d, f, i], is_dual)
    } else {
        select_146([a, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1212([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1213([e, b, c, d, g, f, h, j, i], is_dual)
    } else {
        select_80([d, i, a, j, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1216([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_100([a, b, c, d, e, f, g], is_dual)
    } else {
        select_85([d, b, c, e, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1215([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_58([h, a, b, e, f, j], is_dual)
    } else {
        select_1216([a, c, d, e, i, g, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_1217([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_142([e, f, a, b, c], !is_dual)
    } else {
        select_320([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1214([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1215([c, d, a, b, e, f, g, h, j, i], is_dual)
    } else {
        select_1217([e, a, b, i, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1211([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1212([b, c, d, f, e, g, a, h, i, j], is_dual)
    } else {
        select_1214([a, b, c, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1210([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1211([a, b, c, d, f, g, h, e, i, j], is_dual)
    } else {
        select_1211([b, a, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1220([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_49([f, b, c, d, e, g, h], is_dual)
    } else {
        select_214([a, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1222([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_70([c, d, a, b, e, f, g], is_dual)
    } else {
        select_155([a, b, e, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1221([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_171([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1222([a, i, b, c, g, h, f, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1219([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1220([a, b, c, h, i, f, g, k, j, l], is_dual)
    } else {
        select_1221([a, d, e, c, g, f, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1226([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_61([b, c, d, e, f, g], is_dual)
    } else {
        select_146([a, c, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1225([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_1226([b, a, f, e, d, g, i, h], is_dual)
    } else {
        select_100([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1224([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1225([e, d, c, a, g, f, i, h, j], is_dual)
    } else {
        select_841([b, c, a, d, f, g, e, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1223([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_701([a, b, f, d, h, g, e, i, j], is_dual)
    } else {
        select_1224([b, a, d, c, e, g, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1219([a, c, d, b, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_1223([a, b, c, i, f, h, k, j, g, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1209([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_1210([a, b, j, d, f, g, c, h, l, k], is_dual)
    } else {
        select_1218([a, b, d, c, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_1185([a, b, c, h, e, f, g, j, i, l, k, m], is_dual)
    } else {
        select_1209([a, b, c, d, i, f, g, h, k, m, l, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1154([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_1155([b, c, d, a, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_1184([b, c, d, j, e, g, h, l, i, a, k, n, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1232([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_422([c, d, e, a, f, g, h], is_dual)
    } else {
        select_66([a, b, f, g, e, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1231([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_834([a, c, b, h, e, f, g, j, i, k, l], is_dual)
    } else {
        select_1232([f, c, d, e, g, h, k, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1235([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_113([a, b, d, e, h, g, i, k], is_dual)
    } else {
        select_643([a, b, c, i, f, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1234([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_612([a, b, c, e, g, h, j, i], is_dual)
    } else {
        select_1235([a, b, d, c, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1237([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_500([a, c, b, g, e, f, i, j, h, k], is_dual)
    } else {
        select_844([c, b, d, f, h, g, j, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_1236([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1237([b, i, d, c, f, h, g, l, j, k, m], is_dual)
    } else {
        select_843([a, d, b, e, j, h, f, i, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1233([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1234([a, i, b, d, e, h, k, f, j, m, l], is_dual)
    } else {
        select_1236([b, a, d, c, e, f, h, g, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1230([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1231([b, g, d, e, f, a, h, i, j, k, l, m], is_dual)
    } else {
        select_1233([b, c, d, e, a, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_1241([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_155([a, c, d, f, g], is_dual)
    } else {
        select_55([a, g, b, e, h, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_1240([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_499([b, c, a, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_1241([a, b, d, h, e, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1239([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1234([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_1240([c, b, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1243([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_49([a, c, d, e, h, f, i], is_dual)
    } else {
        select_284([a, c, b, i, f, g, j, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1245([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_167([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_517([g, c, b, a, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1244([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1245([a, c, f, d, e, g, i, h, j], is_dual)
    } else {
        select_253([a, b, h, d, e, f, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1243([b, e, c, d, g, f, h, i, j, k], is_dual)
    } else {
        select_1244([f, a, e, b, d, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1238([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1239([f, a, b, e, j, g, i, k, h, m, l], is_dual)
    } else {
        select_1242([a, c, d, h, e, f, g, i, k, l, j, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1229([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1230([a, b, c, d, i, f, g, h, k, j, m, n, l], is_dual)
    } else {
        select_1238([b, c, a, h, e, j, f, g, i, l, k, n, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1250([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1069([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_67([h, d, e, a, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1249([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1250([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_78([a, b, g, c, i, f, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1252([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_48([g, c, d, e, a, h], is_dual)
    } else {
        select_167([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1253([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([a, e, c, d, g, f, h], is_dual)
    } else {
        select_58([a, b, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1251([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1252([a, f, d, e, h, g, k, i, j], is_dual)
    } else {
        select_1253([a, i, b, c, f, g, j, h, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1248([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1249([a, b, k, e, f, g, i, j, l, m], is_dual)
    } else {
        select_1251([a, c, g, e, f, d, h, j, i, l, k, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_1255([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_422([a, c, d, b, e, f, g], is_dual)
    } else {
        select_102([a, f, b, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1254([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1255([b, c, d, a, g, k, j, h], is_dual)
    } else {
        select_892([a, b, c, e, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1247([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1248([a, b, h, d, e, f, g, i, k, l, j, m, n], is_dual)
    } else {
        select_1254([a, b, c, j, e, f, g, h, k, m, n, l], is_dual)
    }
}
/// n = 8, i = 1
fn select_1257([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_65([a, d, e, f, b, c, g, h], is_dual)
    } else {
        select_158([b, c, g, e, f, a, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1258([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1016([a, c, b, d, e, f, h, g, i, j], is_dual)
    } else {
        select_1016([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1256([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1257([a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1258([b, c, a, d, e, i, g, h, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1246([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1247([a, b, c, f, d, e, g, h, i, k, j, m, n, l], is_dual)
    } else {
        select_1256([a, b, c, d, e, k, g, h, l, m, j, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1228([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1229([b, d, e, c, f, h, i, g, j, a, k, l, m, n], is_dual)
    } else {
        select_1246([b, d, e, a, c, f, h, i, j, g, k, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1227([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1228([a, b, c, d, e, f, h, i, j, k, l, g, m, n], is_dual)
    } else {
        select_1228([a, c, b, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1153([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1154([f, c, d, e, a, b, i, j, g, h, l, m, n, k], is_dual)
    } else {
        select_1227([f, c, d, b, e, a, i, j, h, k, g, l, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1152([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1153([a, b, c, d, e, f, g, h, j, k, l, m, i, n], is_dual)
    } else {
        select_1153([a, b, c, d, e, f, g, i, j, k, l, m, h, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1151([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1152([a, b, c, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_1153([a, g, c, d, e, f, h, b, j, k, l, m, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1150([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < f) || (is_dual && n > f) {
        select_777([a, f, b, c, d, e, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1151([a, b, c, d, e, g, h, i, j, k, l, m, n, f], is_dual)
    }
}
/// n = 14, i = 3
fn select_1147([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1148([a, f, b, c, d, e, g, h, i, k, l, m, n, j], is_dual)
    } else {
        select_1150([a, b, c, d, e, g, h, i, j, k, l, m, n, f], is_dual)
    }
}
/// n = 14, i = 3
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_1147([a, b, c, d, e, f, g, h, i, k, l, m, n, j], is_dual)
    } else {
        select_1147([a, b, c, d, j, f, g, h, i, k, l, m, n, e], is_dual)
    }
}
/// n = 14, i = 3
fn select_1145([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_1146([a, b, c, d, e, f, g, h, i, j, l, m, n, k], is_dual)
    } else {
        select_1146([a, b, c, k, e, f, g, h, i, j, l, m, n, d], is_dual)
    }
}
/// n = 15, i = 3
fn select_772([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_773([a, b, c, d, e, f, g, h, j, i, k, l, n, m, o], is_dual)
    } else {
        select_1145([j, m, n, a, b, c, d, e, f, g, h, l, i, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_1272([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_158([a, b, e, d, h, g, i], is_dual)
    } else {
        select_1252([a, c, b, d, g, f, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_786([b, a, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1272([b, d, c, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1270([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1271([a, b, c, d, l, e, i, j, m, k, n], is_dual)
    } else {
        select_273([a, c, b, f, g, h, k, i, l, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_1275([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_119([a, b, h, d, e, f, g, i, j], is_dual)
    } else {
        select_666([c, g, d, e, a, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_117([c, b, d, e, f, a, h, g, i, j, k], is_dual)
    } else {
        select_1275([a, c, i, e, f, g, h, b, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1273([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1020([d, a, e, f, g, c, b, i, h, j, k, l], is_dual)
    } else {
        select_1274([b, c, j, d, f, g, h, a, i, k, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_1269([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_1270([b, a, d, c, f, e, k, l, i, j, m, o, p, n], is_dual)
    } else {
        select_1273([b, d, e, g, h, a, m, i, k, l, n, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_1280([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_169([a, e, c, d, f, g, h], is_dual)
    } else {
        select_818([c, b, a, f, g, h, e, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1279([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_962([a, d, e, f, i, h, j, k], is_dual)
    } else {
        select_1280([a, j, b, c, k, g, h, i, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1282([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_58([g, b, a, e, h, i], is_dual)
    } else {
        select_66([a, e, c, d, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1283([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_142([f, a, h, d, g], is_dual)
    } else {
        select_1197([d, b, c, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1281([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1282([a, b, d, f, e, g, i, h, j], is_dual)
    } else {
        select_1283([b, c, h, e, g, f, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1279([b, c, d, a, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1281([b, i, c, d, a, g, k, h, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1277([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_246([b, d, a, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_1278([b, d, c, e, f, g, a, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1286([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_253([a, g, i, c, d, k, h, l], is_dual)
    } else {
        select_36([a, b, h, e, f, g, j, i, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1288([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_70([d, g, e, f, h, j, i], is_dual)
    } else {
        select_70([c, i, a, b, h, k, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1287([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1288([c, d, a, b, e, f, h, g, i, j, k], is_dual)
    } else {
        select_192([e, f, g, c, d, a, i, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1285([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1286([f, e, a, b, c, d, g, h, i, j, k, l], is_dual)
    } else {
        select_1287([f, g, a, b, c, d, e, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1291([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_169([a, h, e, b, f, g, i], is_dual)
    } else {
        select_978([a, e, c, d, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1290([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_69([a, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_1291([a, c, b, h, d, g, i, f, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1289([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1290([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_161([d, a, e, f, b, c, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1284([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1285([d, i, e, f, a, c, b, l, j, n, m, k], is_dual)
    } else {
        select_1289([c, b, d, h, a, g, k, i, m, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1276([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_1277([b, a, c, d, l, g, h, n, i, k, m, o], is_dual)
    } else {
        select_1284([a, c, b, e, g, h, f, i, j, k, m, l, o, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < i) || (is_dual && n > i) {
        select_1269([b, c, d, e, f, g, a, h, j, k, i, l, m, o, n, p], is_dual)
    } else {
        select_1276([a, b, c, e, d, g, f, l, j, k, n, m, i, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1267([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1268([a, d, b, g, e, c, h, f, i, j, l, k, o, n, p, m], is_dual)
    } else {
        select_1268([a, d, c, g, f, b, h, e, i, k, l, j, o, m, p, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_1266([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1267([f, e, g, c, d, h, b, a, k, j, m, i, o, l, n, p], is_dual)
    } else {
        select_1267([f, e, h, c, d, g, b, a, k, j, l, i, o, m, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1265([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1266([i, h, f, g, e, a, c, d, m, l, b, j, k, o, n, p], is_dual)
    } else {
        select_1266([i, h, f, g, e, b, c, d, m, l, a, j, k, o, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1264([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1265([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o, p], is_dual)
    } else {
        select_1265([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1264([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o, p], is_dual)
    } else {
        select_1264([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1262([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1263([a, b, c, d, e, f, g, i, h, j, k, m, n, l, o, p], is_dual)
    } else {
        select_1263([a, b, c, d, e, f, h, i, g, j, k, l, n, m, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1261([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1262([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o, p], is_dual)
    } else {
        select_1262([a, b, c, d, e, f, h, i, j, k, l, g, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1260([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < p) || (is_dual && o > p) {
        select_1261([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o], is_dual)
    } else {
        select_1261([a, b, c, d, e, f, g, h, i, k, j, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1259([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < o) || (is_dual && i > o) {
        select_773([a, b, c, d, e, f, g, h, j, k, n, m, o, p, i], is_dual)
    } else {
        select_1260([a, b, c, d, e, f, g, h, j, i, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_771([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < p) || (is_dual && j > p) {
        select_772([a, b, c, d, e, f, g, h, i, k, n, m, o, p, j], is_dual)
    } else {
        select_1259([a, b, c, d, e, f, g, h, i, k, j, l, m, n, o, p], is_dual)
    }
}
/// n = 12, i = 3
fn select_1306([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_226([a, b, c, d, e, g, h, i, k, l, j], is_dual)
    } else {
        select_810([e, f, c, d, g, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1307([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_226([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_184([a, c, d, e, i, h, g, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1305([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1306([e, f, b, g, d, c, h, j, i, l, m, k], is_dual)
    } else {
        select_1307([a, d, i, e, f, h, g, j, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1304([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1305([a, b, c, f, d, e, g, h, i, j, l, k, m], is_dual)
    } else {
        select_328([a, j, c, f, k, i, h, g, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1310([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_978([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_350([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1309([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1310([a, d, b, g, e, h, f, j, i], is_dual)
    } else {
        select_1310([a, d, c, f, e, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1308([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1309([a, b, i, c, f, g, k, h, l, j], is_dual)
    } else {
        select_209([a, g, h, d, e, f, j, i, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1303([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1304([a, c, b, e, f, d, g, i, h, j, l, k, m], is_dual)
    } else {
        select_1308([a, h, d, e, f, g, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1302([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1303([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_1303([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1314([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_95([a, b, c, f, h, i, g], is_dual)
    } else {
        select_510([a, d, b, e, h, g, f, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1313([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_719([a, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1314([a, b, i, d, c, h, g, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1317([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_193([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_212([f, a, c, d, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1316([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_118([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1317([a, i, b, c, g, f, h, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1318([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_122([c, f, d, e, a, g, h], is_dual)
    } else {
        select_253([a, b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1315([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1316([b, d, e, f, h, a, g, i, j, k, l], is_dual)
    } else {
        select_1318([a, b, c, f, i, g, h, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1312([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1313([a, b, d, g, e, f, c, h, i, j, k], is_dual)
    } else {
        select_1315([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1320([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_439([e, f, a, b, c, d, g, h, i, j], is_dual)
    } else {
        select_313([f, e, i, a, b, g, h, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_1322([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_188([a, b, c, d, e, f, g], is_dual)
    } else {
        select_398([e, f, b, g, a], is_dual)
    }
}
/// n = 9, i = 2
fn select_1321([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_762([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_1322([a, c, d, h, g, b, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1319([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1320([c, e, f, g, h, a, b, j, i, k, l], is_dual)
    } else {
        select_1321([a, b, c, d, e, i, h, k, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1311([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1312([a, b, c, d, e, f, k, i, j, m, n, l], is_dual)
    } else {
        select_1319([a, b, c, d, j, g, h, i, k, l, n, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_1301([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_1302([a, b, c, k, d, g, h, m, i, l, j, o, n], is_dual)
    } else {
        select_1311([a, b, c, i, g, h, e, f, j, l, k, n, m, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_1328([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_52([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_290([b, a, f, g, h, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1329([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_52([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_54([b, c, d, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1327([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1328([a, d, c, e, b, f, g, h], is_dual)
    } else {
        select_1329([d, c, b, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1327([a, b, i, c, d, g, h, j, k], is_dual)
    } else {
        select_762([b, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1330([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_203([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_203([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1325([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1326([b, a, e, f, c, d, g, i, h, k, j], is_dual)
    } else {
        select_1330([b, g, c, d, a, i, j, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_1333([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_416([c, d, f, g, a, b, e, h, i, j, k, l], is_dual)
    } else {
        select_630([a, b, j, e, i, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1335([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_48([h, d, e, f, a, i], is_dual)
    } else {
        select_417([b, c, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_1334([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_416([b, c, f, g, a, d, e, h, i, k, j, l], is_dual)
    } else {
        select_1335([a, b, c, d, e, i, h, k, l, j], is_dual)
    }
}
/// n = 15, i = 3
fn select_1332([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1333([a, b, d, e, j, g, h, k, l, m, i, n, o], is_dual)
    } else {
        select_1334([c, d, e, f, i, g, h, k, l, j, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_1339([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_85([e, c, d, h, g, j], is_dual)
    } else {
        select_85([e, a, b, i, f, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1338([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_44([j, c, d, e, f, i, g, l], is_dual)
    } else {
        select_1339([a, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1337([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_416([b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1338([b, c, a, k, d, e, h, i, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1336([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_311([h, l, a, b, k, m, i], is_dual)
    } else {
        select_1337([e, c, d, a, b, f, g, h, j, i, k, l, n, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_1331([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1332([b, c, d, e, f, a, g, h, j, i, k, l, m, n, o], is_dual)
    } else {
        select_1336([b, c, e, f, i, g, h, a, j, k, l, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1324([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < n) || (is_dual && i > n) {
        select_1325([a, b, c, d, k, l, m, j, n, i, o], is_dual)
    } else {
        select_1331([b, c, d, a, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1323([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1324([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    } else {
        select_1324([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1300([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_1301([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    } else {
        select_1323([a, b, d, c, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
/// n = 12, i = 3
fn select_1346([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_127([d, e, f, g, i], is_dual)
    } else {
        select_79([a, j, b, c, k, h, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1345([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1346([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_962([i, d, e, f, g, a, j, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1347([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1307([a, c, b, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_330([a, c, b, h, f, j, i, k, g], is_dual)
    }
}
/// n = 14, i = 3
fn select_1344([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1345([a, c, d, e, f, g, i, j, l, k, m, n], is_dual)
    } else {
        select_1347([a, k, b, c, d, h, m, j, i, n, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1343([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < b) || (is_dual && l > b) {
        select_1344([a, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    } else {
        select_888([a, b, c, d, e, m, i, j, k, o, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1351([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_417([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_193([h, d, b, c, a, g, j, i, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1350([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_219([a, f, g, h, l, k], is_dual)
    } else {
        select_1351([a, b, c, k, d, e, i, j, m, n, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1349([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_1350([b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_190([b, a, c, d, e, f, j, k, n, o, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_1353([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_122([b, c, d, e, a, f, g], is_dual)
    } else {
        select_429([a, b, c, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1352([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_311([a, i, b, c, d, g, j], is_dual)
    } else {
        select_1353([a, b, c, e, f, j, h, i], is_dual)
    }
}
/// n = 15, i = 3
fn select_1348([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_1349([b, a, h, i, c, d, e, f, g, k, j, l, n, o, m], is_dual)
    } else {
        select_1352([b, c, d, a, h, i, m, k, o, j], is_dual)
    }
}
/// n = 15, i = 3
fn select_1342([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1343([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o], is_dual)
    } else {
        select_1348([b, c, a, d, g, h, i, e, f, j, k, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 1
fn select_1358([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_127([e, f, g, h, i], is_dual)
    } else {
        select_127([a, b, c, d, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_1357([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_269([a, j, e, f, g, h, i, k, l], is_dual)
    } else {
        select_1358([b, c, d, i, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1356([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1357([a, c, d, e, f, g, h, i, b, j, k, l], is_dual)
    } else {
        select_1357([b, c, d, e, f, g, h, i, a, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1360([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_131([a, b, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_126([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_1359([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1360([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_1360([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_1355([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1356([a, b, d, e, j, f, g, h, i, k, l, m], is_dual)
    } else {
        select_1359([a, b, c, k, f, g, h, i, j, l, m], is_dual)
    }
}
/// n = 10, i = 1
fn select_1361([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_721([a, b, c, i, f, g, h, d, j], is_dual)
    } else {
        select_26([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 15, i = 3
fn select_1354([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1355([a, b, c, g, h, d, e, f, k, j, l, m, o], is_dual)
    } else {
        select_1361([a, d, e, f, i, b, j, l, k, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1341([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_1342([a, b, c, d, h, i, e, f, g, k, l, m, j, n, o], is_dual)
    } else {
        select_1354([a, c, d, e, f, g, h, i, b, k, j, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1340([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1341([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    } else {
        select_1341([a, c, d, e, f, g, h, i, j, b, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1299([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < l) || (is_dual && d > l) {
        select_1300([a, b, c, e, f, g, h, i, j, k, l, m, d, n, o], is_dual)
    } else {
        select_1340([a, b, c, e, f, d, g, h, i, j, k, m, l, n, o], is_dual)
    }
}
/// n = 11, i = 2
fn select_1368([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_724([a, c, b, j, h, g, k, i], is_dual)
    } else {
        select_276([c, d, e, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1369([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_724([a, c, b, h, g, f, j, i], is_dual)
    } else {
        select_48([c, d, e, i, g, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_1367([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1368([a, b, c, e, f, i, g, h, k, j, l], is_dual)
    } else {
        select_1369([a, b, c, d, j, g, h, i, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1371([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_171([c, d, e, f, i, g, h, j], is_dual)
    } else {
        select_162([a, b, j, c, d, g, h, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1370([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_360([a, b, c, h, i, j, g, l, k], is_dual)
    } else {
        select_1371([a, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_1366([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1367([e, d, a, b, c, k, j, g, h, i, l, m], is_dual)
    } else {
        select_1370([f, e, d, i, a, b, j, g, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1365([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1366([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1366([a, b, c, e, d, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1364([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1365([a, b, c, d, e, f, h, i, j, k, l, g, m], is_dual)
    } else {
        select_1365([a, b, c, d, e, g, h, i, j, k, l, f, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1376([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_1105([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_1183([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_1375([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1376([a, b, c, d, h, i, g, j, l, k], is_dual)
    } else {
        select_1104([a, b, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_1374([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1375([c, d, a, b, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_1375([c, d, b, a, e, f, g, i, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_1373([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1374([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_1374([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1372([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_985([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    } else {
        select_1373([a, b, c, d, h, f, g, i, k, l, j, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1363([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1364([a, g, h, d, e, b, c, f, k, l, i, j, m], is_dual)
    } else {
        select_1372([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1362([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_1095([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_1363([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    }
}
/// n = 16, i = 3
fn select_1298([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_1299([a, b, c, d, e, h, f, g, i, l, k, m, o, p, n], is_dual)
    } else {
        select_1362([a, b, c, d, e, h, i, j, m, k, l, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1297([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_1298([a, b, c, d, e, i, j, g, f, h, k, l, m, p, o, n], is_dual)
    } else {
        select_1298([a, b, c, d, f, i, j, h, e, g, l, k, m, p, n, o], is_dual)
    }
}
/// n = 16, i = 3
fn select_1296([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1297([a, b, c, d, e, g, f, h, i, j, l, m, n, k, o, p], is_dual)
    } else {
        select_1297([a, b, c, d, f, g, e, h, i, j, k, m, n, l, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1295([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1296([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o, p], is_dual)
    } else {
        select_1296([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1294([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1295([f, g, h, i, j, k, c, d, e, a, b, l, n, o, m, p], is_dual)
    } else {
        select_1295([f, g, h, i, j, k, c, e, d, a, b, l, m, o, n, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1293([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1294([k, l, a, i, j, b, c, d, e, f, g, h, m, n, o, p], is_dual)
    } else {
        select_1294([k, l, h, i, j, b, c, d, e, f, g, a, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_1292([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1293([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o, p], is_dual)
    } else {
        select_1293([a, b, c, d, e, f, g, h, j, k, l, m, i, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < o) || (is_dual && k > o) {
        select_771([a, b, c, d, e, f, g, h, i, j, l, m, n, o, k, p], is_dual)
    } else {
        select_1292([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < l) || (is_dual && p > l) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o, p], is_dual)
    } else {
        select_770([a, b, c, d, e, f, g, h, i, j, k, m, n, o, p, l], is_dual)
    }
}
/// n = 16, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m, p], is_dual)
    }
}
/// n = 16, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < o) || (is_dual && p > o) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o], is_dual)
    }
}
