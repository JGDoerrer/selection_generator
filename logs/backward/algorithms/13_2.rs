/// n = 1, i = 0
fn select_18([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_17([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_18([a], is_dual)
    } else {
        select_18([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_16([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_17([a, b], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_15([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_16([a, b, c], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_14([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_15([d, e, f, g], is_dual)
    } else {
        select_15([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_13([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_14([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_14([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 11, i = 1
fn select_12([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_13([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_13([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_11([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_12([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_15([c, d, e, f], is_dual)
    } else {
        select_16([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_22([a, g, c, d, e, f, h], is_dual)
    } else {
        select_22([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_21([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_21([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_20([a, b, c, e, f, g, d, h], is_dual)
    } else {
        select_20([a, b, d, e, f, g, c, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_11([a, e, f, g, h, b, c, d, i, j, k], is_dual)
    } else {
        select_19([a, b, c, d, e, i, j, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_10([a, b, c, d, f, h, e, i, j, l, k, m], is_dual)
    } else {
        select_10([a, b, c, d, e, g, f, j, i, m, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_9([a, d, e, f, b, g, c, h, j, k, l, i, m], is_dual)
    } else {
        select_9([a, d, e, f, c, g, b, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_8([a, b, d, e, f, g, h, i, c, j, k, l, m], is_dual)
    } else {
        select_8([a, c, d, e, f, g, h, i, b, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_7([a, b, c, d, f, g, h, i, j, e, k, l, m], is_dual)
    } else {
        select_7([a, b, c, e, f, g, h, i, j, d, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_6([a, b, c, d, e, h, i, j, f, g, l, m, k], is_dual)
    } else {
        select_6([a, b, c, d, e, h, i, j, g, f, k, m, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_5([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_5([a, b, c, d, e, g, h, i, j, k, f, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_4([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, i, j, k, l, h, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_34([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([c, d, e], is_dual)
    } else {
        select_16([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_34([a, f, c, d, e, g], is_dual)
    } else {
        select_34([b, e, c, d, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_32([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_22([a, b, e, f, g, h, j], is_dual)
    } else {
        select_33([c, d, e, h, f, g, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_37([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_17([a, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_15([c, d, e, g], is_dual)
    } else {
        select_37([a, b, h, f, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_40([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_17([a, d], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_39([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_40([a, c, d, g], is_dual)
    } else {
        select_40([b, c, e, f], is_dual)
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
/// n = 10, i = 2
fn select_35([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_36([b, e, c, d, f, g, i, h, j], is_dual)
    } else {
        select_38([h, a, e, b, g, f, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_31([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_32([d, e, a, c, f, b, g, h, j, i], is_dual)
    } else {
        select_35([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_30([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_20([a, b, e, f, g, k, h, j], is_dual)
    } else {
        select_31([a, b, e, c, d, j, h, i, l, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_29([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_30([a, c, f, g, e, b, h, i, j, k, m, l], is_dual)
    } else {
        select_30([a, b, f, g, d, c, i, h, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_28([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_29([a, b, d, c, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_29([a, c, d, b, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_27([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_28([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_26([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_27([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_27([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_25([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_26([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_26([a, b, c, d, e, g, f, h, i, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_24([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_25([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_23([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_24([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    } else {
        select_24([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_23([a, b, c, d, e, f, g, h, i, k, l, m, j], is_dual)
    }
}
/// n = 3, i = 1
fn select_55([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_17([a, b], is_dual)
    } else {
        select_18([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_54([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_40([b, c, e, d], is_dual)
    } else {
        select_55([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_56([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_40([a, b, c, d], is_dual)
    } else {
        select_55([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_53([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_54([a, b, c, e, d, f], is_dual)
    } else {
        select_56([d, c, a, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_57([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_37([b, a, c, d, e], is_dual)
    } else {
        select_40([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_53([a, c, d, b, e, f], is_dual)
    } else {
        select_57([a, b, d, c, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_59([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_55([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_58([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_59([a, c, d, b, e, f], is_dual)
    } else {
        select_59([b, c, d, a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_51([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_52([a, b, c, d, e, f], is_dual)
    } else {
        select_58([a, b, c, d, f, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_62([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_18([e], is_dual)
    }
}
/// n = 5, i = 2
fn select_63([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_40([a, b, c, d], is_dual)
    } else {
        select_55([d, a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_61([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_62([b, c, d, e, f], is_dual)
    } else {
        select_63([e, g, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_61([a, c, d, e, b, f, g], is_dual)
    } else {
        select_61([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_50([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_51([a, b, d, f, c, g], is_dual)
    } else {
        select_60([a, b, d, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_67([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_54([e, a, c, f, d, g], is_dual)
    } else {
        select_54([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_67([b, g, c, e, a, f, h], is_dual)
    } else {
        select_33([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_69([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_34([a, b, c, d, e, f], is_dual)
    } else {
        select_37([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_68([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_69([d, a, b, g, e, f], is_dual)
    } else {
        select_69([d, a, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_66([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_68([b, d, e, a, c, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_65([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_58([a, b, f, g, h, c], is_dual)
    }
}
/// n = 8, i = 2
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_50([a, b, c, d, g, h, f], is_dual)
    } else {
        select_64([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_73([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_33([a, c, d, e, b, f, g], is_dual)
    } else {
        select_33([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_73([a, c, e, b, f, g, h], is_dual)
    } else {
        select_73([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_71([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_72([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_72([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_70([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_71([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_71([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_49([a, b, c, e, h, g, i, d], is_dual)
    } else {
        select_70([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_79([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_63([d, f, e, a, b], !is_dual)
    } else {
        select_63([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_78([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_69([b, c, a, d, e, f], is_dual)
    } else {
        select_79([b, c, f, e, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_77([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_52([a, f, b, c, g, e], is_dual)
    } else {
        select_78([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_76([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_77([a, b, d, e, c, f, g], is_dual)
    } else {
        select_77([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_84([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([a, b, c], is_dual)
    } else {
        select_18([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_15([c, d, e, f], is_dual)
    } else {
        select_84([a, b, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_85([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([b, c, d, e], is_dual)
    } else {
        select_55([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_82([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_83([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_85([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_87([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([b, c, d, e], is_dual)
    } else {
        select_17([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_87([g, d, e, f, a, h], is_dual)
    } else {
        select_14([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_81([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_82([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_86([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_91([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_40([a, c, b, d], is_dual)
    } else {
        select_40([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_15([c, d, e, f], is_dual)
    } else {
        select_91([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_89([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_90([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_36([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_89([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_89([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_80([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_81([a, b, c, e, f, g, d, h, i], is_dual)
    } else {
        select_88([a, b, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_76([a, b, c, e, h, d, i], is_dual)
    } else {
        select_80([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_95([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_14([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_83([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_94([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_95([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_95([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_88([a, b, h, e, f, g, c, i, j], is_dual)
    } else {
        select_94([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_97([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_66([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_52([a, f, b, g, h, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_96([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_97([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_97([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_92([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_93([a, b, c, e, d, f, g, h, i, j], is_dual)
    } else {
        select_96([a, b, c, i, e, d, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_74([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_75([a, b, c, d, e, f, h, j, i], is_dual)
    } else {
        select_92([a, b, c, d, g, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_47([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_48([a, b, c, d, h, g, e, i, j], is_dual)
    } else {
        select_74([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_101([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_81([f, g, a, d, e, i, h, j, k], is_dual)
    } else {
        select_81([f, g, a, b, c, j, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_100([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_101([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_60([f, g, e, i, j, k, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_99([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_100([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_100([a, b, c, d, h, f, g, e, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_107([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_63([a, b, d, e, f], is_dual)
    } else {
        select_63([a, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_106([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_22([b, c, a, d, e, f, g], is_dual)
    } else {
        select_107([g, b, c, a, f, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_110([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_55([a, b, c], is_dual)
    } else {
        select_55([a, c, b], is_dual)
    }
}
/// n = 6, i = 2
fn select_109([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_110([a, e, f], is_dual)
    } else {
        select_37([a, b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_111([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_34([a, b, c, d, e, f], is_dual)
    } else {
        select_91([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_108([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_109([e, c, d, a, f, g], is_dual)
    } else {
        select_111([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_106([a, c, d, b, e, g, f, h], is_dual)
    } else {
        select_108([a, f, c, d, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_104([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_81([a, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_105([a, b, c, i, d, h, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_103([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_50([a, b, h, c, i, j, g], is_dual)
    } else {
        select_104([a, b, d, c, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_103([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_103([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_98([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_99([a, f, g, h, b, c, d, e, i, j, k], is_dual)
    } else {
        select_102([b, c, d, e, i, g, h, a, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_47([a, b, c, d, e, j, g, f, i, k], is_dual)
    } else {
        select_98([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_45([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_46([a, b, c, d, e, f, g, j, i, l, k], is_dual)
    } else {
        select_46([a, b, c, d, e, f, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_120([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_54([a, d, c, e, f, g], is_dual)
    } else {
        select_91([a, b, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_87([c, d, e, g, f, h], is_dual)
    } else {
        select_120([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_90([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_90([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_119([a, b, g, e, f, c, h, i, j], is_dual)
    } else {
        select_121([c, d, e, f, a, b, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_122([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_121([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_121([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_117([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_118([a, b, c, d, h, i, g, j, l, k], is_dual)
    } else {
        select_122([a, b, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_118([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_118([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_116([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_117([b, c, d, f, a, e, i, g, h, j, l, k], is_dual)
    } else {
        select_123([b, c, a, d, h, f, k, i, j, g, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_115([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_116([d, b, c, e, a, f, h, g, j, k, l, i], is_dual)
    } else {
        select_116([d, b, c, f, a, e, h, g, i, k, l, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_115([g, e, f, a, c, d, j, b, h, i, k, l], is_dual)
    } else {
        select_115([g, e, f, b, c, d, j, a, h, i, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_113([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_114([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_114([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_113([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_113([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_44([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_45([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_112([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_34([b, c, d, e, g, f], is_dual)
    } else {
        select_84([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_133([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_34([c, g, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_134([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_83([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_111([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_131([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_132([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_134([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_136([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_22([b, c, d, e, f, g, h], is_dual)
    } else {
        select_34([a, h, b, c, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_135([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_136([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_136([f, a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_131([a, b, c, h, e, f, g, i, j], is_dual)
    } else {
        select_135([d, g, e, f, a, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_22([b, c, d, e, i, g, h], is_dual)
    } else {
        select_133([f, a, h, b, c, g, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_13([a, b, h, c, d, e, f, j, k, i], is_dual)
    } else {
        select_139([i, c, d, e, f, g, j, k, h, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_140([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_89([f, a, d, e, i, h, g, j, k], is_dual)
    } else {
        select_89([f, a, b, c, j, h, g, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_138([a, g, b, c, d, e, f, i, h, j, k, l], is_dual)
    } else {
        select_140([h, b, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_129([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_130([a, b, c, d, j, k, h, i, m, l], is_dual)
    } else {
        select_137([e, c, i, f, g, b, a, j, h, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_129([g, d, e, f, a, b, c, i, k, h, l, j, m], is_dual)
    } else {
        select_129([g, d, f, e, a, b, c, i, j, h, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_128([f, g, h, a, d, e, b, k, c, i, j, l, m], is_dual)
    } else {
        select_128([f, g, h, c, d, e, b, k, a, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_126([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_127([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_127([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_125([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_126([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_126([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 11, i = 2
fn select_146([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_140([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_140([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_145([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_101([f, a, b, c, d, g, h, e, i, j, k], is_dual)
    } else {
        select_146([e, a, b, c, d, g, h, f, i, j, k], is_dual)
    }
}
/// n = 9, i = 1
fn select_150([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_22([a, h, d, e, f, g, i], is_dual)
    } else {
        select_14([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_149([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_150([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_21([b, h, d, e, f, g, a, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_149([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_66([a, i, c, d, b, g, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_147([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_104([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_148([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_144([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_145([d, i, g, h, e, a, b, c, k, j, l], is_dual)
    } else {
        select_147([a, b, c, d, f, e, j, i, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_143([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_144([a, b, c, d, f, e, g, h, j, k, i, l], is_dual)
    } else {
        select_144([a, b, c, e, f, d, g, h, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_142([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_143([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    } else {
        select_143([a, b, c, e, f, g, h, i, d, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_142([a, b, c, d, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_142([a, b, c, d, e, g, h, i, j, f, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_124([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_125([a, b, c, d, e, f, g, j, h, i, k, l, m], is_dual)
    } else {
        select_141([a, b, c, d, e, f, g, h, i, k, m, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_43([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_44([a, b, c, d, e, f, g, k, j, h, l, m], is_dual)
    } else {
        select_124([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_155([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_70([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_70([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_154([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_155([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_155([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_153([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_154([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_154([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_156([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_48([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_152([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_153([a, b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_156([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_163([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_53([a, f, c, b, e, g], is_dual)
    } else {
        select_69([a, c, b, d, e, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_162([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_81([a, b, g, d, e, f, h, i, j], is_dual)
    } else {
        select_163([a, b, c, i, g, j, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_167([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([b, c, e], is_dual)
    } else {
        select_40([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_166([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_167([a, b, f, d, g, e, h], is_dual)
    } else {
        select_167([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_168([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_54([a, e, b, d, g, f], is_dual)
    } else {
        select_84([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_165([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_166([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_168([a, b, f, e, h, g, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_165([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_165([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_162([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_164([a, b, d, i, g, c, h, j], is_dual)
    }
}
/// n = 5, i = 0
fn select_173([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_15([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_172([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_69([a, j, b, c, k, h], is_dual)
    } else {
        select_173([d, e, f, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_174([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_62([a, c, d, f, g], is_dual)
    } else {
        select_54([a, g, b, e, h, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_171([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_172([a, c, d, b, e, f, g, h, j, i, k], is_dual)
    } else {
        select_174([a, i, c, d, b, k, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_177([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_54([a, c, b, d, e, f], is_dual)
    } else {
        select_56([a, b, e, f, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_176([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_177([a, d, c, f, e, g], is_dual)
    } else {
        select_177([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_178([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_87([f, c, d, e, b, g], is_dual)
    } else {
        select_83([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_176([a, b, c, h, g, i, f], is_dual)
    } else {
        select_178([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_170([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_171([e, f, a, b, c, d, g, h, i, j, k], is_dual)
    } else {
        select_175([f, e, i, a, b, g, k, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_78([a, c, e, b, h, f, g], is_dual)
    } else {
        select_78([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_182([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_22([b, f, c, d, e, g, h], is_dual)
    } else {
        select_83([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_181([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_89([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_182([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_180([a, b, c, d, i, h, j, g], is_dual)
    } else {
        select_181([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_169([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_170([a, e, f, g, c, d, b, h, i, j, k], is_dual)
    } else {
        select_179([b, c, d, h, f, g, a, j, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_161([a, b, d, h, f, g, i, c, j, k], is_dual)
    } else {
        select_169([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_186([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_177([b, g, c, a, f, h], is_dual)
    } else {
        select_83([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_185([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_180([a, b, c, g, d, f, h, i], is_dual)
    } else {
        select_186([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_184([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_185([a, b, c, d, h, f, j, i, g], is_dual)
    } else {
        select_71([a, b, c, g, e, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_183([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_184([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_184([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_159([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_160([a, b, c, e, f, d, g, h, j, i, k], is_dual)
    } else {
        select_183([a, b, c, e, i, f, d, h, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_189([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_123([c, d, a, e, b, f, h, i, j, g, k], is_dual)
    } else {
        select_123([c, d, b, e, a, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_188([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_189([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_189([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_187([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_188([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_188([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_158([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_159([a, b, c, d, g, f, h, i, j, e, k], is_dual)
    } else {
        select_187([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_158([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_158([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_151([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_152([a, b, c, d, e, f, g, j, k, h], is_dual)
    } else {
        select_157([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_42([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_43([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_151([a, b, c, d, e, f, g, h, m, i, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_41([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_42([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_42([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_41([a, b, c, d, e, f, g, h, i, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    }
}
