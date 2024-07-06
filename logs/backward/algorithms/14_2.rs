/// n = 1, i = 0
fn select_19([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_18([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_19([a], is_dual)
    } else {
        select_19([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_17([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_18([a, b], is_dual)
    } else {
        select_18([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_16([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_17([a, b, c], is_dual)
    } else {
        select_17([b, c, d], is_dual)
    }
}
/// n = 7, i = 1
fn select_15([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([c, d, e, f], is_dual)
    } else {
        select_17([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_16([d, e, f, g], is_dual)
    } else {
        select_16([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_14([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_15([a, h, d, e, f, g, i], is_dual)
    } else {
        select_20([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_15([a, g, c, d, e, f, h], is_dual)
    } else {
        select_15([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_13([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_14([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_21([b, h, d, e, f, g, a, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_12([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_13([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_13([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_24([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_20([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_20([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 11, i = 1
fn select_23([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_24([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_24([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_22([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_23([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_12([a, b, c, e, d, j, k, i, m], is_dual)
    } else {
        select_22([a, d, f, g, h, b, c, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_11([a, d, e, c, h, g, b, i, k, j, l, n, m], is_dual)
    } else {
        select_11([a, d, e, b, h, f, c, j, k, i, m, n, l], is_dual)
    }
}
/// n = 14, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_10([a, b, d, e, f, c, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_10([a, c, d, e, f, b, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_9([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_9([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_8([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_8([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_7([a, b, c, d, e, f, h, i, g, j, l, m, k, n], is_dual)
    } else {
        select_7([a, b, c, d, e, g, h, i, f, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_6([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_6([a, b, c, d, e, g, h, i, j, k, f, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_5([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_5([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 6, i = 1
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([c, d, e], is_dual)
    } else {
        select_17([a, b, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_34([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_15([b, c, d, e, f, g, h], is_dual)
    } else {
        select_35([a, h, b, c, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_34([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_34([f, a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_39([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_17([a, b, c], is_dual)
    } else {
        select_19([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_35([b, c, d, e, g, f], is_dual)
    } else {
        select_39([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_38([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_35([c, g, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([c, d, e, f], is_dual)
    } else {
        select_39([a, b, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_44([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_18([a, d], is_dual)
    } else {
        select_18([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_43([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_44([a, c, b, d], is_dual)
    } else {
        select_44([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_35([a, b, c, d, e, f], is_dual)
    } else {
        select_43([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_40([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_41([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_42([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_37([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_40([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_32([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_33([d, g, e, f, a, c, h, i], is_dual)
    } else {
        select_36([a, b, c, h, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_14([h, c, d, e, f, g, a, i, j], is_dual)
    } else {
        select_24([e, f, g, a, b, c, d, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_48([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_38([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_15([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_47([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_48([c, a, d, e, f, b, g, i, h, j], is_dual)
    } else {
        select_40([b, g, c, e, f, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_45([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_46([g, a, b, c, d, e, i, h, j, k], is_dual)
    } else {
        select_47([g, f, k, a, b, c, h, i, j, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_31([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_32([a, b, c, d, j, k, h, i, m, l], is_dual)
    } else {
        select_45([e, c, i, f, g, b, a, j, h, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_30([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_31([g, d, e, f, a, b, c, i, k, h, l, j, m], is_dual)
    } else {
        select_31([g, d, f, e, a, b, c, i, j, h, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_29([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_30([f, g, h, a, d, e, b, k, c, i, j, l, m], is_dual)
    } else {
        select_30([f, g, h, c, d, e, b, k, a, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_28([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_29([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_29([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_27([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_28([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_28([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 3, i = 1
fn select_59([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_18([a, b], is_dual)
    } else {
        select_19([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_58([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_44([b, c, e, d], is_dual)
    } else {
        select_59([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_60([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_44([a, b, c, d], is_dual)
    } else {
        select_59([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_58([a, c, b, d, e, f], is_dual)
    } else {
        select_60([a, b, e, f, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_62([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_17([b, c, d], is_dual)
    } else {
        select_18([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_61([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_62([a, c, d, b, e], is_dual)
    } else {
        select_62([b, c, d, a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_57([a, b, f, d, g, e], is_dual)
    } else {
        select_61([a, d, c, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_64([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_35([a, b, c, d, e, f], is_dual)
    } else {
        select_62([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_66([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_44([a, b, e, f], is_dual)
    } else {
        select_44([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_66([a, b, e, f, d, g], is_dual)
    } else {
        select_62([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_64([d, b, c, e, f, g], is_dual)
    } else {
        select_65([a, g, b, d, f, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_56([a, b, g, e, f, h, d], is_dual)
    } else {
        select_63([b, a, c, e, f, d, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_55([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_55([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 5, i = 0
fn select_70([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_16([a, b, c, d], is_dual)
    } else {
        select_16([a, b, c, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_71([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_62([a, b, f, d, e], is_dual)
    } else {
        select_62([a, c, e, d, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_69([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_70([c, d, e, f, i], is_dual)
    } else {
        select_71([a, b, j, g, h, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_73([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_62([b, a, c, d, e], is_dual)
    } else {
        select_44([b, e, d, a], is_dual)
    }
}
/// n = 10, i = 2
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_70([c, d, e, f, h], is_dual)
    } else {
        select_73([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_68([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_69([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_72([a, h, c, d, e, f, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_67([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_68([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_68([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_53([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_54([a, b, j, d, c, k, h, i], is_dual)
    } else {
        select_67([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_78([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_58([a, b, c, e, d, f], is_dual)
    } else {
        select_60([d, c, a, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_78([a, d, b, h, e, g], is_dual)
    } else {
        select_78([a, d, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_76([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_77([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_73([a, d, g, f, b], is_dual)
    }
}
/// n = 7, i = 1
fn select_81([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, f, c, d, e, g], is_dual)
    } else {
        select_35([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_80([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_81([a, c, d, e, b, f, g], is_dual)
    } else {
        select_81([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_79([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_80([a, c, e, b, f, g, h], is_dual)
    } else {
        select_80([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_76([a, c, f, e, h, g, b, i], is_dual)
    } else {
        select_79([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_85([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([b, c, e], is_dual)
    } else {
        select_44([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_84([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([a, b, f, d, g, e, h], is_dual)
    } else {
        select_85([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_86([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_58([a, e, b, d, g, f], is_dual)
    } else {
        select_39([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_84([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_86([a, b, f, e, h, g, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_82([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_83([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_83([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_74([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_75([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_82([a, b, d, g, f, i, h, c], is_dual)
    }
}
/// n = 11, i = 2
fn select_52([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_53([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_74([a, b, c, e, j, h, d, i, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_64([a, b, e, g, f, i], is_dual)
    } else {
        select_17([c, d, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_58([a, c, b, f, g, e], is_dual)
    } else {
        select_58([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_64([a, c, d, e, g, f], is_dual)
    } else {
        select_93([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_91([b, d, a, c, e, g, f, i, h], is_dual)
    } else {
        select_92([b, f, e, d, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_90([a, b, g, e, d, i, h, f, j], is_dual)
    } else {
        select_79([a, b, f, e, c, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_88([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_75([a, g, b, e, c, h, i, j, f], is_dual)
    } else {
        select_89([a, c, e, d, b, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_88([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_88([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_51([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_52([a, b, c, e, f, d, g, h, j, i, k], is_dual)
    } else {
        select_87([a, b, c, e, i, f, d, h, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_100([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([c, d, e, f], is_dual)
    } else {
        select_43([a, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_99([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_100([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_100([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_102([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_85([a, c, d, b, e, f, g], is_dual)
    } else {
        select_85([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_100([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_102([a, b, c, h, g, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_98([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_99([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_101([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_97([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_98([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_98([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_96([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_97([c, d, a, e, b, f, h, i, j, g, k], is_dual)
    } else {
        select_97([c, d, b, e, a, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_95([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_96([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_96([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_94([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_95([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_95([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_50([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_51([a, b, c, d, g, f, h, i, j, e, k], is_dual)
    } else {
        select_94([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_49([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_50([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_50([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_26([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_27([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_49([a, b, c, d, e, f, g, h, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_25([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_26([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    } else {
        select_26([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < j) || (is_dual && n > j) {
        select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_25([a, b, c, d, e, f, g, h, i, k, m, n, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
