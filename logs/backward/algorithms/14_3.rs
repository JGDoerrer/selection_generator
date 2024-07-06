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
/// n = 5, i = 1
fn select_17([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_21([e], is_dual)
    }
}
/// n = 4, i = 1
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_20([a, d], is_dual)
    } else {
        select_20([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_25([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_20([a, b], is_dual)
    } else {
        select_21([c], is_dual)
    }
}
/// n = 5, i = 2
fn select_23([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_25([d, a, e], is_dual)
    }
}
/// n = 4, i = 1
fn select_26([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_19([a, b, c], is_dual)
    } else {
        select_21([d], is_dual)
    }
}
/// n = 6, i = 2
fn select_22([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_23([e, f, a, b, c], !is_dual)
    } else {
        select_26([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_16([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([b, d, e, f, g], is_dual)
    } else {
        select_22([a, b, c, g, f, h], is_dual)
    }
}
/// n = 5, i = 2
fn select_28([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_26([a, b, c, d], is_dual)
    } else {
        select_26([a, b, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_28([a, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_15([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_16([c, a, b, d, e, f, g, h], is_dual)
    } else {
        select_27([a, f, d, e, b, g, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([e, c, d, f, g], is_dual)
    } else {
        select_17([e, a, b, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_32([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_19([b, c, d], is_dual)
    } else {
        select_25([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_32([a, c, d, b, e, f], is_dual)
    } else {
        select_32([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_30([a, b, d, e, c, f, g], is_dual)
    } else {
        select_31([a, b, c, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_15([a, b, c, d, e, j, h, k, l], is_dual)
    } else {
        select_29([a, b, c, f, g, k, i, j, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_37([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([c, d, e], is_dual)
    } else {
        select_19([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([a, f, c, d, e, g], is_dual)
    } else {
        select_37([b, e, c, d, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_38([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_20([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_36([a, b, c, d, f, g, h], is_dual)
    } else {
        select_38([b, c, d, e, g, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_40([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_19([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([d, e, f, g], is_dual)
    } else {
        select_18([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_40([a, h, d, e, f, g, i], is_dual)
    } else {
        select_41([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_34([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_35([b, h, d, e, f, g, a, i], is_dual)
    } else {
        select_39([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_26([a, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_43([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_41([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_44([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_42([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_43([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_43([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_33([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_34([c, f, g, a, b, j, h, i, k], is_dual)
    } else {
        select_42([a, b, d, e, c, h, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_14([a, b, c, f, g, d, i, j, k, h, l, m], is_dual)
    } else {
        select_33([a, b, d, f, g, c, e, h, i, j, l, k, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([b, c, e, d], is_dual)
    } else {
        select_25([a, b, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_36([b, c, d, e, g, f, h], is_dual)
    } else {
        select_49([a, b, c, f, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_47([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_39([a, b, c, d, e, h, g, i, j], is_dual)
    } else {
        select_48([f, j, a, b, c, g, h, i, k], is_dual)
    }
}
/// n = 10, i = 1
fn select_51([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_41([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_41([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_37([b, c, d, e, g, f], is_dual)
    } else {
        select_26([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_52([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_40([b, c, d, e, i, g, h], is_dual)
    } else {
        select_53([f, a, h, b, c, g, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_50([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_51([a, b, h, c, d, e, f, j, k, i], is_dual)
    } else {
        select_52([i, c, d, e, f, g, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_47([h, b, c, d, e, g, f, i, j, k, l], is_dual)
    } else {
        select_50([a, f, b, c, d, e, g, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_40([b, f, c, d, e, g, h], is_dual)
    } else {
        select_44([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_54([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_55([a, h, d, e, f, g, b, i, j], is_dual)
    } else {
        select_43([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_45([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_46([e, b, i, f, g, c, a, j, h, l, k, m], is_dual)
    } else {
        select_54([a, b, d, c, j, k, h, i, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_13([a, b, c, e, g, d, i, h, j, l, m, k, n], is_dual)
    } else {
        select_45([c, e, d, g, f, a, b, h, j, i, k, m, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_62([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_19([b, c, d], is_dual)
    } else {
        select_20([a, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([c, d, e, g], is_dual)
    } else {
        select_62([a, b, h, f, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_64([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, b, d], is_dual)
    } else {
        select_24([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_64([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_60([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_61([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_63([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_59([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_60([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_60([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_65([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_39([h, c, d, e, f, g, a, i, j], is_dual)
    } else {
        select_51([e, f, g, a, b, c, d, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_58([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_59([a, b, d, c, i, j, h, l, m], is_dual)
    } else {
        select_65([c, e, f, g, a, b, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_58([a, b, d, e, g, c, i, h, j, k, m, l, n], is_dual)
    } else {
        select_58([a, b, c, e, f, d, j, h, i, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 1
fn select_69([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_38([g, d, e, f, a, h], is_dual)
    } else {
        select_41([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_68([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_30([a, h, d, e, f, g, i], is_dual)
    } else {
        select_69([f, d, e, b, c, g, i, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_71([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_62([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_17([a, c, d, e, g], is_dual)
    } else {
        select_71([a, b, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_67([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_68([d, a, g, e, f, b, h, j, i], is_dual)
    } else {
        select_70([a, c, b, h, i, g, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_76([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, e], is_dual)
    } else {
        select_24([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_75([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_76([a, c, d, e, g, f, h], is_dual)
    } else {
        select_24([a, b, e, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_74([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_36([b, c, d, e, i, g, h], is_dual)
    } else {
        select_75([a, b, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_73([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_74([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_24([b, j, f, a], is_dual)
    }
}
/// n = 9, i = 2
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_61([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_61([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_80([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_62([a, c, d, b, e], is_dual)
    } else {
        select_62([b, c, d, a, e], is_dual)
    }
}
/// n = 11, i = 3
fn select_79([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_71([c, e, f, h, g, j], is_dual)
    } else {
        select_80([a, b, d, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_77([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_78([a, b, e, f, g, k, j, i, m], is_dual)
    } else {
        select_79([a, b, c, e, d, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_72([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_73([c, e, i, f, g, h, d, j, k, l], is_dual)
    } else {
        select_77([a, b, c, e, d, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_66([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_67([d, e, f, g, c, j, h, i, m, k, l], is_dual)
    } else {
        select_72([a, b, d, c, f, e, i, h, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_56([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_57([a, b, d, e, c, f, g, h, i, j, l, m, k, n], is_dual)
    } else {
        select_66([a, b, c, d, e, f, g, i, j, k, l, m, h, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_12([b, c, a, e, f, g, h, d, i, j, k, l, m, n], is_dual)
    } else {
        select_56([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_87([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_23([h, i, a, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_86([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_44([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_87([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_55([a, c, d, e, f, b, g, h, j], is_dual)
    } else {
        select_86([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_59([a, c, b, e, f, g, d, h, i], is_dual)
    } else {
        select_85([a, c, d, e, f, g, b, h, j, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_91([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_25([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_44([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_91([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_89([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_90([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_69([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_59([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_89([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_84([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_88([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_89([a, b, h, e, f, g, c, i, j], is_dual)
    } else {
        select_42([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_92([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_84([a, d, b, c, e, f, h, j, k, i], is_dual)
    } else {
        select_93([a, d, e, g, b, c, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_82([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_83([a, d, b, c, e, g, k, m, j, l], is_dual)
    } else {
        select_92([a, b, c, d, f, j, h, i, l, k, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_97([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_43([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_86([b, h, d, e, f, g, a, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_54([b, c, f, a, e, h, g, i, j, k], is_dual)
    } else {
        select_97([a, b, d, c, e, i, g, h, k, j, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_98([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_54([a, c, e, b, f, h, g, i, j, k], is_dual)
    } else {
        select_54([a, b, d, c, f, i, g, h, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_96([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    } else {
        select_98([a, b, d, e, g, f, c, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_55([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_55([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_54([a, d, e, b, c, f, g, h, i, j], is_dual)
    } else {
        select_101([a, b, c, d, f, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_102([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_97([a, b, e, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_101([b, c, d, a, f, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_99([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_100([a, d, e, b, f, g, c, h, i, j], is_dual)
    } else {
        select_102([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_94([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_95([a, b, c, f, h, k, g, j, i, m, l, n], is_dual)
    } else {
        select_99([a, c, b, d, e, h, l, j, k, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_81([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_82([a, b, c, e, d, f, i, h, j, l, m, k, n], is_dual)
    } else {
        select_94([a, d, e, b, c, f, h, g, j, i, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([a, c, d, e, f, g, b, h, i, k, l, j, m, n], is_dual)
    } else {
        select_81([a, c, d, b, e, g, f, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_10([a, b, c, d, e, g, f, h, i, j, k, l, n, m], is_dual)
    } else {
        select_10([a, b, c, d, f, h, e, g, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_9([h, e, c, d, b, f, a, g, j, i, l, m, n, k], is_dual)
    } else {
        select_9([h, e, c, d, b, g, a, f, j, i, k, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_8([c, d, a, b, e, f, g, h, i, k, l, m, n, j], is_dual)
    } else {
        select_8([d, c, a, b, e, f, g, h, j, k, l, m, n, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_7([a, b, c, d, e, f, g, h, j, k, l, m, i, n], is_dual)
    } else {
        select_7([a, b, c, d, e, f, i, h, j, k, l, m, g, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_6([a, b, c, d, e, f, g, h, i, k, l, m, j, n], is_dual)
    } else {
        select_6([a, b, c, d, e, j, g, h, i, k, l, m, f, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_5([j, k, a, h, i, b, c, d, e, f, g, l, m, n], is_dual)
    } else {
        select_5([j, k, g, h, i, b, c, d, e, f, a, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_4([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_117([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_49([e, a, c, f, d, g], is_dual)
    } else {
        select_49([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_116([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_36([b, c, a, d, e, f, g], is_dual)
    } else {
        select_117([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_119([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_23([d, f, e, a, b], !is_dual)
    } else {
        select_23([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_71([b, c, a, d, e, f], is_dual)
    } else {
        select_119([b, c, f, e, a, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_116([e, c, a, d, f, h, g, i], is_dual)
    } else {
        select_118([f, a, b, g, e, i, h], is_dual)
    }
}
/// n = 5, i = 2
fn select_123([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_25([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_122([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_49([a, b, c, e, d, f], is_dual)
    } else {
        select_123([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_121([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_71([a, c, b, d, e, f], is_dual)
    } else {
        select_122([a, f, c, b, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_121([a, e, b, d, g, f, h], is_dual)
    } else {
        select_118([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_115([a, f, c, d, h, e, g, i, j], is_dual)
    } else {
        select_120([a, e, g, b, j, f, k, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_127([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([a, d, b, c, e, f], is_dual)
    } else {
        select_49([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_126([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_80([b, c, d, e, f], is_dual)
    } else {
        select_127([a, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_125([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_126([a, c, d, e, b, f, g], is_dual)
    } else {
        select_126([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_129([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_80([a, b, c, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_132([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, c, d, g], is_dual)
    } else {
        select_24([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_131([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_132([a, d, b, g, e, f, h], is_dual)
    } else {
        select_132([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_49([d, b, c, e, f, h], is_dual)
    } else {
        select_132([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_130([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_131([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_133([e, b, d, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_128([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_129([a, f, d, e, i, h], is_dual)
    } else {
        select_130([a, h, b, c, f, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_124([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_125([a, i, b, c, f, k, h], is_dual)
    } else {
        select_128([a, d, e, b, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_114([b, c, d, e, g, f, h, a, i, j, k], is_dual)
    } else {
        select_124([b, a, c, d, e, g, h, f, j, i, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_138([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_62([b, a, c, d, e], is_dual)
    } else {
        select_24([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 1
fn select_137([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_71([a, c, b, d, e, f], is_dual)
    } else {
        select_138([a, c, f, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_140([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, c, b, f, g, e], is_dual)
    } else {
        select_49([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_139([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_71([a, c, d, e, g, f], is_dual)
    } else {
        select_140([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_136([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_137([a, e, d, f, g, b], is_dual)
    } else {
        select_139([a, c, b, d, e, g, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_144([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, b, e, f], is_dual)
    } else {
        select_24([a, c, d, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_143([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_144([b, c, d, f, g, h], is_dual)
    } else {
        select_24([a, d, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_131([e, b, c, d, g, f, h, j], is_dual)
    } else {
        select_143([d, a, f, c, g, e, j, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_129([f, g, d, e, j, i], is_dual)
    } else {
        select_142([a, b, i, c, f, g, h, k, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_135([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_136([a, b, e, f, g, k, j, i], is_dual)
    } else {
        select_141([e, c, d, a, b, g, f, h, i, k, l, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_148([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_144([a, b, e, f, d, g], is_dual)
    } else {
        select_62([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_71([d, b, c, e, f, g], is_dual)
    } else {
        select_148([a, g, b, d, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_146([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_147([b, a, d, e, h, g, f, i], is_dual)
    } else {
        select_139([a, b, c, f, e, i, h, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_145([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_146([f, b, h, a, e, g, k, l, i], is_dual)
    } else {
        select_115([e, g, c, d, i, f, h, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_134([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_135([a, b, d, e, c, h, g, i, f, k, l, j], is_dual)
    } else {
        select_145([b, c, d, e, g, h, f, i, a, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_112([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_113([a, b, g, d, e, j, h, i, f, k, l], is_dual)
    } else {
        select_134([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_112([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_112([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_153([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_36([b, d, a, c, e, f, g], is_dual)
    } else {
        select_31([a, e, c, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_152([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_153([b, g, c, d, a, f, h, i, j], is_dual)
    } else {
        select_48([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_154([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_30([b, g, d, e, a, f, h], is_dual)
    } else {
        select_30([c, f, d, e, a, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_151([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_152([e, d, h, a, b, f, g, i, j, k], is_dual)
    } else {
        select_154([e, a, b, c, i, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_150([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_151([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    } else {
        select_151([a, b, c, f, e, g, h, i, d, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_159([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_64([a, b, c, d], is_dual)
    } else {
        select_25([a, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_71([b, a, c, d, e, f], is_dual)
    } else {
        select_159([a, f, b, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_157([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_158([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_158([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_26([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_160([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_161([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_161([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_157([a, b, c, d, h, i, g, j], is_dual)
    } else {
        select_160([a, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_165([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_32([a, b, c, d, f, g], is_dual)
    } else {
        select_64([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_164([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_165([a, c, d, b, e, f, g], is_dual)
    } else {
        select_165([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_163([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_164([a, b, c, d, f, e, g], is_dual)
    } else {
        select_31([a, b, f, g, c, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_168([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_49([a, b, c, e, d, g], is_dual)
    } else {
        select_49([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_167([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_122([a, g, d, e, f, b], !is_dual)
    } else {
        select_168([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_166([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_90([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_167([a, g, c, b, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_163([a, b, c, g, h, i, f], is_dual)
    } else {
        select_166([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_155([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_156([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_162([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_149([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_150([e, f, g, a, b, c, h, i, j, k, l], is_dual)
    } else {
        select_155([a, b, c, d, h, i, g, k, l, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_111([a, b, c, d, e, i, g, h, l, j, m, k], is_dual)
    } else {
        select_149([a, b, c, d, f, h, g, i, k, j, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_63([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_63([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_177([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_49([a, d, c, e, f, g], is_dual)
    } else {
        select_64([a, b, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_176([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_38([c, d, e, g, f, h], is_dual)
    } else {
        select_177([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_175([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_176([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_179([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_38([a, b, c, d, f, g], is_dual)
    } else {
        select_18([b, c, d, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_181([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_64([a, b, c, d], is_dual)
    } else {
        select_25([d, b, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_182([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_144([d, f, g, a, e, b], !is_dual)
    } else {
        select_23([e, f, d, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_181([a, b, d, h, g], is_dual)
    } else {
        select_182([f, g, h, a, e, d, c], !is_dual)
    }
}
/// n = 11, i = 3
fn select_178([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_179([c, d, e, f, h, g, i], is_dual)
    } else {
        select_180([a, b, c, i, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_174([a, b, c, f, e, g, h, i, j, k], is_dual)
    } else {
        select_178([a, b, d, c, e, h, g, i, k, l, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_186([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_32([b, c, d, e, f, g], is_dual)
    } else {
        select_64([a, g, h, b], !is_dual)
    }
}
/// n = 6, i = 2
fn select_187([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_49([a, b, c, e, d, f], is_dual)
    } else {
        select_24([c, d, e, b], is_dual)
    }
}
/// n = 8, i = 3
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_186([a, b, c, d, e, g, h, f], is_dual)
    } else {
        select_187([a, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_184([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_185([a, b, f, c, g, h, e, i], is_dual)
    } else {
        select_185([a, b, e, d, g, h, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_49([a, e, c, f, d, h], is_dual)
    } else {
        select_49([b, d, c, f, e, g], is_dual)
    }
}
/// n = 3, i = 1
fn select_192([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_25([a, b, c], is_dual)
    } else {
        select_25([a, c, b], is_dual)
    }
}
/// n = 6, i = 2
fn select_193([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, d, c, f], is_dual)
    } else {
        select_24([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_191([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_192([a, f, g], is_dual)
    } else {
        select_193([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_189([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_190([a, b, d, e, g, f, h, i], is_dual)
    } else {
        select_191([h, i, e, g, a, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_195([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_49([f, c, g, e, d, a], !is_dual)
    } else {
        select_23([d, g, c, e, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_195([h, g, a, e, f, b, d], !is_dual)
    } else {
        select_181([a, c, d, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_188([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_189([c, a, b, d, e, f, g, i, h], is_dual)
    } else {
        select_194([a, c, b, f, e, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_183([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_184([a, b, c, f, e, i, g, j, h], is_dual)
    } else {
        select_188([a, b, e, d, h, f, g, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_173([b, c, a, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_183([b, c, d, h, g, k, i, j, a, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_199([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_122([a, c, d, b, e, f], is_dual)
    } else {
        select_138([a, b, d, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_198([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_199([a, f, b, c, g, e], is_dual)
    } else {
        select_118([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_146([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_198([a, b, e, f, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_202([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_36([a, c, d, e, b, f, g], is_dual)
    } else {
        select_36([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_131([c, d, e, a, b, f, g, h], is_dual)
    } else {
        select_131([c, d, e, b, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_201([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_202([a, b, d, e, f, h, g], is_dual)
    } else {
        select_203([a, b, c, d, g, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_204([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_129([a, f, d, e, g, h], is_dual)
    } else {
        select_180([a, b, c, h, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_200([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_201([c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_204([a, b, e, c, d, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_197([b, c, e, d, g, i, h, j, a, k], is_dual)
    } else {
        select_200([b, c, a, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_171([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_172([a, b, d, e, g, f, h, i, j, l, k, m], is_dual)
    } else {
        select_196([a, b, d, c, e, k, h, j, g, m, l], is_dual)
    }
}
/// n = 8, i = 1
fn select_209([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_36([a, b, d, e, f, g, h], is_dual)
    } else {
        select_37([b, c, d, e, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_208([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_74([a, h, b, d, e, f, g, j, i, k], is_dual)
    } else {
        select_209([b, c, i, d, e, g, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_36([b, c, d, f, e, g, h], is_dual)
    } else {
        select_71([e, a, d, g, f, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_35([b, c, d, e, i, f, h, j], is_dual)
    } else {
        select_211([a, b, c, j, f, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_207([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_208([c, a, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_210([c, b, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_214([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_40([b, h, c, d, e, g, i], is_dual)
    } else {
        select_61([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_213([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_214([c, a, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_214([c, b, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_217([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_193([a, b, c, e, d, f], is_dual)
    } else {
        select_193([a, b, d, e, c, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_216([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_122([g, b, c, d, f, h], is_dual)
    } else {
        select_217([b, a, d, h, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_219([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_49([a, b, d, f, e, h], is_dual)
    } else {
        select_76([c, a, b, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_220([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_49([a, b, d, g, e, h], is_dual)
    } else {
        select_64([e, f, c, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_218([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_219([d, b, c, f, e, g, h, i], is_dual)
    } else {
        select_220([e, a, c, f, d, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_215([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_216([d, c, a, e, f, b, g, h], is_dual)
    } else {
        select_218([c, b, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_212([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_213([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_215([b, c, i, d, a, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_207([a, d, e, f, b, c, g, h, j, i, k], is_dual)
    } else {
        select_212([a, b, c, e, d, h, g, j, k, i, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_223([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_55([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_60([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_225([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_117([b, c, d, a, e, f, h], is_dual)
    } else {
        select_117([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_224([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_199([b, e, c, a, g, f], is_dual)
    } else {
        select_225([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_222([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_223([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_224([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_227([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_199([a, f, b, g, h, e], is_dual)
    } else {
        select_116([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_228([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_43([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_60([b, h, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_226([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_227([a, c, i, d, g, b, h, j], is_dual)
    } else {
        select_228([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_221([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_222([a, b, c, d, e, h, g, j, i, k], is_dual)
    } else {
        select_226([a, c, d, f, b, e, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_206([a, b, e, c, f, d, h, g, i, j, k, l], is_dual)
    } else {
        select_221([a, b, c, d, e, h, g, j, i, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_170([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_171([a, c, b, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_205([a, c, b, h, d, f, k, i, j, l, g, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_170([a, b, d, e, f, g, c, h, i, j, k, l, m], is_dual)
    } else {
        select_170([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_110([a, b, c, e, f, g, h, i, j, d, k, l, m], is_dual)
    } else {
        select_169([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_234([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_195([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_190([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_233([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_234([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_234([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_235([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_121([a, b, c, d, e, f, g], is_dual)
    } else {
        select_199([a, b, c, f, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_232([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_233([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_235([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_238([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_122([a, d, b, e, c, f], is_dual)
    } else {
        select_122([a, d, c, e, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_239([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_17([a, b, c, e, g], is_dual)
    } else {
        select_71([a, d, b, c, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_237([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_238([a, b, c, h, j, g], is_dual)
    } else {
        select_239([a, e, g, d, f, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_241([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_122([a, e, b, g, d, h], is_dual)
    } else {
        select_122([a, d, c, f, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_240([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_241([a, b, f, e, i, h, j, g], is_dual)
    } else {
        select_118([a, g, c, d, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_236([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_237([a, b, c, e, d, g, f, i, h, j], is_dual)
    } else {
        select_240([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_231([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_232([a, b, d, h, j, f, c, i], is_dual)
    } else {
        select_236([a, b, d, c, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_230([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_231([a, b, c, d, e, g, h, i, f, j], is_dual)
    } else {
        select_231([a, b, d, c, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_229([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_230([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_230([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_108([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < e) || (is_dual && m > e) {
        select_109([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_229([a, b, c, d, i, l, k, m, e, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_251([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_49([a, c, b, e, f, g], is_dual)
    } else {
        select_26([a, c, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_250([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_17([a, d, e, h, g], is_dual)
    } else {
        select_251([a, b, c, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_249([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_30([c, h, d, e, a, g, i], is_dual)
    } else {
        select_250([a, b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_253([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_49([d, f, g, e, a, b], !is_dual)
    } else {
        select_49([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_252([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([a, c, d, f, g], is_dual)
    } else {
        select_253([h, f, i, a, e, g, b], !is_dual)
    }
}
/// n = 11, i = 3
fn select_248([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_249([b, c, a, d, e, g, f, h, i, j], is_dual)
    } else {
        select_252([a, c, b, i, g, h, j, f, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_257([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_76([a, c, d, e, h, g, j], is_dual)
    } else {
        select_76([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_258([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_49([a, b, c, d, e, f], is_dual)
    } else {
        select_25([a, f, b], is_dual)
    }
}
/// n = 10, i = 3
fn select_256([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_257([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_258([a, h, b, f, e, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_259([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_181([a, h, b, i, e], is_dual)
    } else {
        select_186([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_255([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_256([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_259([a, f, c, d, g, h, e, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_261([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_195([g, i, a, h, e, b, c], !is_dual)
    } else {
        select_187([a, b, d, g, f, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_264([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_24([b, c, d, e], is_dual)
    } else {
        select_25([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_263([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_264([a, c, d, e, f, g, h], is_dual)
    } else {
        select_64([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_49([a, g, c, f, d, i], is_dual)
    } else {
        select_132([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_262([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_263([a, b, d, e, g, i, h, j], is_dual)
    } else {
        select_265([a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_260([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_261([a, b, f, d, h, g, e, i, j], is_dual)
    } else {
        select_262([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_254([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_255([a, d, b, c, e, g, h, i, f, j], is_dual)
    } else {
        select_260([a, b, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_247([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_248([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    } else {
        select_254([b, c, h, d, f, a, g, i, j, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_269([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_71([a, b, d, e, c, f], is_dual)
    } else {
        select_71([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_268([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_202([a, d, e, b, c, f, g], is_dual)
    } else {
        select_269([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_271([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_36([c, d, a, b, e, f, g], is_dual)
    } else {
        select_80([a, b, e, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_270([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_189([a, b, c, f, e, i, h, g, j], is_dual)
    } else {
        select_271([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_267([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_268([h, b, c, d, e, g, i, j], is_dual)
    } else {
        select_270([a, b, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_275([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_64([b, c, e, g], is_dual)
    } else {
        select_26([a, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_274([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_63([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_275([a, b, c, h, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_273([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_274([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_274([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_272([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_174([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_273([a, b, c, d, e, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_266([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_267([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_272([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_247([a, c, e, d, f, g, h, i, b, j, k], is_dual)
    } else {
        select_266([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_279([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_202([a, c, d, b, e, f, g], is_dual)
    } else {
        select_118([a, c, g, d, b, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_282([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_62([b, c, d, g, f], is_dual)
    } else {
        select_23([a, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_281([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_282([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_186([f, a, c, d, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_280([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_43([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_281([a, i, b, c, g, f, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_279([a, g, c, d, i, h, k, j], is_dual)
    } else {
        select_280([b, c, h, e, f, a, g, j, i, l, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_286([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_49([a, c, b, d, e, f], is_dual)
    } else {
        select_123([a, b, e, f, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_285([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_131([b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_286([a, b, i, g, e, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_288([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_76([a, b, c, d, e, f, g], is_dual)
    } else {
        select_25([a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_287([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_288([a, b, f, d, g, e, h, i], is_dual)
    } else {
        select_288([a, c, e, d, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_284([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_285([a, c, b, d, f, g, e, h, i, j], is_dual)
    } else {
        select_287([a, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 5, i = 0
fn select_291([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_18([a, b, c, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_290([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_291([b, c, d, e, f], is_dual)
    } else {
        select_25([a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_292([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_71([b, d, c, e, f, g], is_dual)
    } else {
        select_91([a, b, c, f, e, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_289([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_290([a, d, e, f, i, g, j, l], is_dual)
    } else {
        select_292([a, b, j, c, g, h, k, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_283([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_284([b, c, i, d, a, g, h, k, j, l], is_dual)
    } else {
        select_289([b, c, d, a, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_277([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_278([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_283([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_36([b, f, c, d, e, g, h], is_dual)
    } else {
        select_138([e, a, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_297([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_36([b, e, c, d, f, g, h], is_dual)
    } else {
        select_62([a, f, h, e, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_295([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_296([c, b, d, e, a, f, g, h, i], is_dual)
    } else {
        select_297([c, a, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_120([a, c, g, d, b, f, i, h], is_dual)
    } else {
        select_295([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_42([a, b, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_279([a, b, c, d, j, h, k, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_293([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_294([a, b, c, i, e, d, h, k, j], is_dual)
    } else {
        select_298([a, b, c, e, d, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_276([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_277([a, b, c, e, g, d, f, h, j, k, i, l], is_dual)
    } else {
        select_293([a, b, e, d, g, c, f, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_245([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_246([a, b, h, d, e, g, c, j, i, k, l], is_dual)
    } else {
        select_276([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_305([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([c, d, e, g], is_dual)
    } else {
        select_32([a, b, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_304([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_41([b, c, h, d, e, f, i, j], is_dual)
    } else {
        select_305([a, i, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_306([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_40([b, h, c, d, e, g, i], is_dual)
    } else {
        select_305([a, g, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_303([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_304([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_306([a, h, d, e, f, g, b, i, j, k, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_309([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_62([a, b, f, d, e], is_dual)
    } else {
        select_62([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_308([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_36([a, b, c, d, e, f, g], is_dual)
    } else {
        select_309([a, b, g, e, f, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_310([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_181([a, b, f, g, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_307([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_308([c, d, f, b, e, g, h], is_dual)
    } else {
        select_310([e, a, b, g, f, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_302([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_303([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_307([b, c, j, d, a, g, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_301([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_302([a, b, d, f, c, h, g, i, j, k, l, m], is_dual)
    } else {
        select_302([a, b, c, e, d, i, g, h, j, l, k, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_313([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_296([b, c, a, g, e, f, h, i, k], is_dual)
    } else {
        select_137([a, e, d, h, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_315([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_71([a, b, c, h, g, j], is_dual)
    } else {
        select_18([d, e, f, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_315([a, c, i, d, e, f, h, k, j, l], is_dual)
    } else {
        select_61([b, j, d, e, f, g, i, k, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_312([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_313([b, c, j, d, a, g, h, i, k, l, m], is_dual)
    } else {
        select_314([b, c, d, a, e, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_317([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_238([a, b, c, i, j, g], is_dual)
    } else {
        select_118([a, d, g, e, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_319([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_61([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_148([g, c, b, a, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_137([a, b, d, h, g, j], is_dual)
    } else {
        select_319([a, c, e, b, g, f, i, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_316([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_317([a, b, g, c, e, h, f, j, i, k], is_dual)
    } else {
        select_318([a, c, b, e, d, f, h, g, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_311([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_312([a, c, b, f, d, e, g, h, i, j, l, k, m], is_dual)
    } else {
        select_316([a, b, c, j, f, g, k, h, l, i, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_300([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_301([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    } else {
        select_311([a, b, d, c, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_323([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_202([b, c, e, f, g, h, i], is_dual)
    } else {
        select_60([a, d, b, c, h, f, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_324([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_202([a, b, e, g, i, h, k], is_dual)
    } else {
        select_214([c, d, a, b, h, f, g, i, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_322([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_323([a, b, c, g, e, f, j, h, i, k, l], is_dual)
    } else {
        select_324([b, c, a, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_327([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_122([a, b, c, d, e, f], is_dual)
    } else {
        select_25([a, f, b], is_dual)
    }
}
/// n = 11, i = 3
fn select_326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_35([b, g, c, d, e, f, h, i], is_dual)
    } else {
        select_327([f, a, i, g, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_330([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([b, c, d, f], is_dual)
    } else {
        select_123([a, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_329([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_330([a, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_330([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_332([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_76([b, a, c, d, e, f, g], is_dual)
    } else {
        select_64([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_331([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_148([d, b, c, g, f, e, h], is_dual)
    } else {
        select_332([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_328([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_329([a, c, d, e, i, f, h, j, k, l], is_dual)
    } else {
        select_331([a, j, b, f, k, g, h, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_325([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_326([a, g, c, d, e, i, f, h, j, l, k], is_dual)
    } else {
        select_328([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_322([a, b, c, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_325([a, d, b, c, e, g, h, i, j, k, f, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_337([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_32([a, b, c, d, f, g], is_dual)
    } else {
        select_64([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_336([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_286([b, d, c, f, g, e], is_dual)
    } else {
        select_337([b, a, e, g, h, d, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_335([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_336([a, b, f, e, h, g, i, d], is_dual)
    } else {
        select_331([a, b, c, e, d, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_339([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_71([g, b, c, d, f, h], is_dual)
    } else {
        select_191([a, b, e, f, g, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_341([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([f, g, e, c, d, a], !is_dual)
    } else {
        select_25([e, b, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_340([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([b, c, d, f, g], is_dual)
    } else {
        select_341([f, i, e, h, a, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_339([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_340([a, b, d, f, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_334([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_335([b, f, d, a, e, g, h, i, j], is_dual)
    } else {
        select_338([b, a, d, c, e, g, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_333([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_334([a, b, g, c, e, j, f, h, k, i], is_dual)
    } else {
        select_334([a, b, f, d, e, i, g, h, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_321([b, a, c, e, d, f, g, h, i, j, k, l], is_dual)
    } else {
        select_333([f, b, c, e, g, k, h, i, a, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_299([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < b) || (is_dual && l > b) {
        select_300([a, c, d, b, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_320([a, c, d, e, i, g, h, l, j, b, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_244([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_245([a, b, k, d, e, f, g, h, j, l, m, n], is_dual)
    } else {
        select_299([a, b, c, d, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_36([d, e, b, c, f, g, h], is_dual)
    } else {
        select_251([a, e, b, c, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_347([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_35([c, d, e, f, i, g, h, j], is_dual)
    } else {
        select_348([a, b, j, c, d, g, h, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_346([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_15([a, b, c, g, h, j, k, i, l], is_dual)
    } else {
        select_347([c, d, e, f, a, b, g, h, j, i, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_351([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_31([a, b, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_350([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_39([d, e, f, a, b, c, g, h, i], is_dual)
    } else {
        select_351([a, b, c, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_350([a, b, h, c, e, f, g, j, i, k, l], is_dual)
    } else {
        select_209([c, d, i, e, f, g, h, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_345([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_346([e, f, d, i, a, b, g, h, k, j, l, m], is_dual)
    } else {
        select_349([e, f, a, b, c, j, g, h, k, i, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_356([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_76([a, c, d, b, e, f, g], is_dual)
    } else {
        select_76([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_355([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_257([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_356([a, g, c, d, e, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_354([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_355([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_355([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_359([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_62([a, c, d, e, h], is_dual)
    } else {
        select_132([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_358([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_36([b, c, d, e, g, h, i], is_dual)
    } else {
        select_359([a, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_357([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_35([c, d, e, f, k, h, i, j], is_dual)
    } else {
        select_358([a, c, d, b, j, g, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_354([a, b, c, h, i, g, k, l, j, m], is_dual)
    } else {
        select_357([c, d, e, f, a, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_361([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_175([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_175([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_273([a, b, c, d, g, h, j, k, i, l], is_dual)
    } else {
        select_361([c, d, e, f, a, b, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_352([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_353([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_360([a, b, c, g, e, f, i, j, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_344([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_345([a, f, g, d, b, c, e, i, j, h, k, l, m], is_dual)
    } else {
        select_352([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_366([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_71([b, a, c, d, e, f], is_dual)
    } else {
        select_64([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_365([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_366([a, c, d, e, b, f, g], is_dual)
    } else {
        select_366([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_368([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_122([a, b, c, d, e, g], is_dual)
    } else {
        select_181([a, b, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_369([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_17([a, d, f, g, b], !is_dual)
    } else {
        select_31([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_367([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_368([a, f, h, b, g, c, d], !is_dual)
    } else {
        select_369([a, f, h, b, g, d, e], !is_dual)
    }
}
/// n = 10, i = 3
fn select_364([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_365([a, b, d, f, g, i, h], is_dual)
    } else {
        select_367([a, b, c, e, h, j, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_371([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_118([a, c, e, b, h, f, g], is_dual)
    } else {
        select_118([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_370([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_371([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_198([a, f, c, e, b, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_363([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_364([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_370([a, b, e, g, f, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_375([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_80([a, b, c, i, j], is_dual)
    } else {
        select_291([d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_376([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_80([b, c, a, h, j], is_dual)
    } else {
        select_80([e, f, d, g, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_374([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_375([c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_376([e, c, d, k, a, b, j, i, m, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_378([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_61([a, h, d, e, f, g, j, i, l], is_dual)
    } else {
        select_19([b, c, k], is_dual)
    }
}
/// n = 4, i = 1
fn select_380([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_64([a, b, c, d], is_dual)
    } else {
        select_64([a, b, d, c], is_dual)
    }
}
/// n = 9, i = 2
fn select_379([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_380([a, b, c, i], is_dual)
    } else {
        select_63([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_378([c, a, b, d, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_379([a, b, h, d, e, f, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_373([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_374([c, d, a, b, e, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_377([c, d, b, f, g, h, j, a, i, k, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_383([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_53([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_53([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_385([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, c, b, f, g, h], is_dual)
    } else {
        select_64([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_384([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_44([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_385([a, b, h, c, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_382([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_383([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_384([g, b, d, e, f, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_381([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_382([a, b, e, f, c, d, g, h, i, k, j], is_dual)
    } else {
        select_125([a, g, c, d, h, j, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_372([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < b) || (is_dual && k > b) {
        select_373([a, c, d, e, f, b, g, h, j, i, k, l, m], is_dual)
    } else {
        select_381([a, c, d, e, k, f, b, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_362([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_363([a, b, c, e, d, k, j, l, m, i], is_dual)
    } else {
        select_372([a, d, e, b, c, f, g, h, j, i, k, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_343([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_344([a, b, c, e, f, d, h, l, k, i, j, m, n], is_dual)
    } else {
        select_362([a, b, c, d, e, g, f, i, j, l, m, k, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_391([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_195([a, b, c, e, d, g, f], is_dual)
    } else {
        select_187([g, c, d, e, f, a], !is_dual)
    }
}
/// n = 8, i = 3
fn select_392([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_117([b, c, d, a, e, f, h], is_dual)
    } else {
        select_117([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_390([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_391([h, e, b, f, g, c, a], !is_dual)
    } else {
        select_392([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_389([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_390([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_198([c, a, b, e, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_393([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_227([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_227([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_388([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_389([a, b, c, d, e, i, h, j], is_dual)
    } else {
        select_393([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_398([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_49([a, b, c, f, e, g], is_dual)
    } else {
        select_49([a, b, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_397([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_53([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_398([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_396([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_43([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_397([b, a, j, c, d, h, g, i, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_400([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_44([a, c, d, e, f, g, h, j], is_dual)
    } else {
        select_385([a, b, c, h, g, f, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_399([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_400([f, e, i, c, d, g, h, j, k, l], is_dual)
    } else {
        select_43([e, c, d, a, b, h, g, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_396([a, g, b, c, d, e, h, f, i, j, l, k], is_dual)
    } else {
        select_399([b, c, d, e, a, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_402([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_129([a, b, c, d, e, g], is_dual)
    } else {
        select_269([a, c, d, b, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_403([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_70([a, b, d, e, f, c, g], is_dual)
    } else {
        select_70([a, c, d, e, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_401([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_402([a, b, c, d, h, g, i], is_dual)
    } else {
        select_403([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_394([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_395([c, d, e, f, g, b, a, h, i, j, k, l], is_dual)
    } else {
        select_401([a, c, d, e, b, j, h, k, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_387([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_388([a, d, b, c, j, e, h, k, l, i], is_dual)
    } else {
        select_394([a, d, e, b, c, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_386([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_387([a, c, d, b, e, f, j, i, k, l, h, m], is_dual)
    } else {
        select_344([a, c, d, e, f, b, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_342([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_343([a, c, d, b, e, f, g, h, j, k, i, l, m, n], is_dual)
    } else {
        select_386([a, i, c, d, e, f, h, b, l, j, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_244([a, b, d, e, f, g, h, j, i, k, c, l, m, n], is_dual)
    } else {
        select_342([a, b, c, d, e, f, g, h, j, k, i, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_412([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_144([b, c, d, e, f, g], is_dual)
    } else {
        select_193([a, c, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_411([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_412([c, a, b, e, d, g, h, f], is_dual)
    } else {
        select_217([d, c, a, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_413([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_71([a, c, d, e, g, h], is_dual)
    } else {
        select_359([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_410([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_411([a, c, b, g, h, i, f, j], is_dual)
    } else {
        select_413([b, c, a, d, e, g, f, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_416([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_76([a, b, c, d, e, f, g], is_dual)
    } else {
        select_76([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_415([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_195([b, f, d, a, e, g, h], is_dual)
    } else {
        select_416([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_414([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_415([a, c, b, g, f, h, e, i], is_dual)
    } else {
        select_139([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_409([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_410([a, c, d, e, g, f, h, i, k, j], is_dual)
    } else {
        select_414([a, b, d, i, f, h, j, g, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_419([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([b, c, d, e, f], is_dual)
    } else {
        select_23([e, g, a, b, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_418([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_137([a, c, d, e, f, g], is_dual)
    } else {
        select_419([b, a, c, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_420([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_331([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_331([c, b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_417([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_418([b, c, d, a, f, e, h, g, i], is_dual)
    } else {
        select_420([a, b, c, d, f, g, h, e, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_408([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_409([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_417([a, b, c, h, f, i, j, k, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_424([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_217([d, c, a, f, e, g], is_dual)
    } else {
        select_332([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_426([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_32([a, b, c, e, h, i], is_dual)
    } else {
        select_62([d, b, c, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_425([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_71([b, e, c, d, g, h], is_dual)
    } else {
        select_426([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_423([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_424([a, b, c, g, h, i, j, f], is_dual)
    } else {
        select_425([b, c, d, e, a, g, f, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_429([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_49([a, f, b, e, d, g], is_dual)
    } else {
        select_62([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_428([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_429([a, b, c, d, e, f, g], is_dual)
    } else {
        select_187([a, b, f, d, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_427([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_428([a, b, c, d, f, e, g], is_dual)
    } else {
        select_234([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_422([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_423([a, c, d, b, e, f, g, h, j, i], is_dual)
    } else {
        select_427([a, c, h, f, i, g, b, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_421([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_422([a, b, h, c, e, g, j, k, l, i], is_dual)
    } else {
        select_409([a, b, c, d, f, g, i, h, k, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_407([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_408([b, h, c, d, f, g, a, i, k, j, l], is_dual)
    } else {
        select_421([b, a, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_406([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_407([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_407([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_436([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_49([a, c, b, d, e, f], is_dual)
    } else {
        select_32([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_435([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_436([a, e, b, g, h, d, f], is_dual)
    } else {
        select_436([a, d, c, f, h, e, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_438([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_64([a, b, c, e], is_dual)
    } else {
        select_64([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_437([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_438([f, a, b, d, e], is_dual)
    } else {
        select_438([e, a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_434([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_435([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_437([a, f, g, h, d, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_440([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_122([a, d, g, e, f, b], !is_dual)
    } else {
        select_122([a, e, g, d, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_439([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_164([a, d, c, f, h, e, g], is_dual)
    } else {
        select_440([a, b, e, d, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_433([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_434([a, c, d, b, f, e, h, g], is_dual)
    } else {
        select_439([a, b, d, e, f, c, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_444([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([a, e, b, d, g, f], is_dual)
    } else {
        select_26([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_445([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_49([g, h, f, d, a, b], !is_dual)
    } else {
        select_64([a, f, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_443([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_444([b, c, a, e, d, g, f], is_dual)
    } else {
        select_445([a, c, b, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_442([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_428([a, c, d, b, f, e, g], is_dual)
    } else {
        select_443([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_441([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_442([a, c, b, e, h, f, i, g], is_dual)
    } else {
        select_442([a, b, c, d, h, g, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_432([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_433([a, f, c, e, b, h, g, i], is_dual)
    } else {
        select_441([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_448([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_199([a, e, g, d, b, f], !is_dual)
    } else {
        select_440([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_447([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_448([d, a, b, e, f, c, g], is_dual)
    } else {
        select_448([d, a, c, e, g, b, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_446([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_447([c, f, g, a, e, b, h], is_dual)
    } else {
        select_433([a, c, b, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_431([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_432([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_446([a, b, g, d, c, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_430([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_431([a, b, c, d, g, j, i, f, h], is_dual)
    } else {
        select_431([a, b, c, e, f, j, h, g, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_405([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_406([a, b, c, e, f, d, g, h, i, k, j, l], is_dual)
    } else {
        select_430([a, b, c, f, j, i, d, k, h, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_453([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_174([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_174([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_456([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_61([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_133([g, c, b, a, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_455([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_30([c, h, d, e, a, g, i], is_dual)
    } else {
        select_456([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_454([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_249([b, c, e, a, g, f, h, i, j, k], is_dual)
    } else {
        select_455([a, c, d, b, h, f, g, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_452([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_453([a, c, b, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_454([b, d, c, e, f, g, h, i, a, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_451([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_221([a, c, b, d, i, e, g, h, k, l, j], is_dual)
    } else {
        select_452([a, c, e, b, f, d, h, i, g, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_450([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_451([a, b, d, c, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_451([a, c, d, b, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_449([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_450([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_450([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_404([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_405([a, b, c, d, e, j, f, h, k, l, i, m], is_dual)
    } else {
        select_449([a, b, c, d, i, e, g, h, l, j, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_242([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < d) || (is_dual && m > d) {
        select_243([a, b, c, e, f, d, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_404([a, b, c, e, f, g, k, i, j, m, d, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_107([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_108([a, b, c, d, e, k, h, i, f, l, j, m, n], is_dual)
    } else {
        select_242([a, b, c, d, e, g, f, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_106([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_107([a, b, c, d, e, g, h, i, f, j, k, l, m, n], is_dual)
    } else {
        select_107([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_466([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_182([a, b, c, d, f, g, e], is_dual)
    } else {
        select_182([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_465([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_129([b, c, d, e, f, g], is_dual)
    } else {
        select_466([f, h, i, a, b, c, g], !is_dual)
    }
}
/// n = 7, i = 2
fn select_469([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_64([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_44([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_469([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_467([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_468([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_52([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_464([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_465([a, b, c, e, g, h, j, i, k], is_dual)
    } else {
        select_467([c, a, d, f, b, e, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_471([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_202([a, c, b, d, e, f, g], is_dual)
    } else {
        select_80([a, b, c, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_470([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_465([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_471([c, a, d, b, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_463([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_464([a, b, c, e, g, f, h, i, j, k, l], is_dual)
    } else {
        select_470([a, c, b, d, k, i, l, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_462([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_463([c, a, d, b, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_463([c, b, d, a, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_461([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_462([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_462([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_460([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_461([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_461([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_478([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_181([a, b, h, i, j], is_dual)
    } else {
        select_291([c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_479([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_61([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_61([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_477([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_478([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_479([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_476([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_477([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_477([a, c, d, e, f, g, h, b, i, k, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_482([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_195([b, a, e, c, f, d, g], is_dual)
    } else {
        select_195([b, a, e, d, f, c, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_483([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_195([a, b, d, g, f, h, e], is_dual)
    } else {
        select_195([a, c, d, g, e, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_481([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_482([b, e, d, h, g, a, f], is_dual)
    } else {
        select_483([b, a, c, d, f, h, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_486([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([a, d, g, f, c], is_dual)
    } else {
        select_23([b, c, g, e, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_485([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_195([d, g, a, f, e, c, b], !is_dual)
    } else {
        select_486([e, g, d, f, a, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_484([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_466([d, g, h, e, a, f, b], !is_dual)
    } else {
        select_485([g, e, h, a, d, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_480([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_481([b, d, c, f, a, g, e, h], is_dual)
    } else {
        select_484([b, d, a, f, e, c, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_475([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_476([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_480([b, c, d, i, h, a, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_440([f, e, i, a, b, h, g], !is_dual)
    } else {
        select_116([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_488([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_489([a, b, e, c, f, g, d, h, i], is_dual)
    } else {
        select_489([a, b, e, d, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_490([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_228([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_228([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_487([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_488([b, c, d, e, i, h, a, j, k], is_dual)
    } else {
        select_490([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_474([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_475([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_487([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_473([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_474([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_474([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_494([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_120([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_120([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_497([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_38([b, c, d, g, e, h], is_dual)
    } else {
        select_133([b, h, a, e, f, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_496([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_296([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_497([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_495([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_125([b, g, c, d, a, i, h], is_dual)
    } else {
        select_496([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_493([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_494([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_495([c, b, a, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_492([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_493([a, b, d, c, e, g, h, i, f, j], is_dual)
    } else {
        select_493([a, c, d, b, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_491([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_492([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_492([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_472([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_473([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    } else {
        select_491([a, b, c, e, f, g, i, j, d, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_459([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_460([a, b, c, d, e, f, i, h, j, l, k, m], is_dual)
    } else {
        select_472([a, b, c, d, f, e, g, k, m, i, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_458([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_459([a, b, c, d, e, g, f, h, j, k, l, i, m], is_dual)
    } else {
        select_459([a, b, c, d, f, g, e, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_457([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_458([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_458([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_105([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_106([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_457([a, b, c, d, e, f, g, h, l, j, m, n, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_104([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_105([a, b, c, d, e, f, g, i, j, h, k, l, m, n], is_dual)
    } else {
        select_105([a, b, c, d, e, f, h, i, j, g, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_509([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_202([a, c, e, b, f, g, h], is_dual)
    } else {
        select_202([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_509([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_509([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_507([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_508([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_508([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_506([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_507([a, b, c, d, e, g, j, i, k], is_dual)
    } else {
        select_507([a, b, c, e, d, f, k, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_513([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_35([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_130([a, i, b, c, g, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_512([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_513([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_513([a, c, d, e, f, g, b, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_511([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_512([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_512([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_516([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_468([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_48([b, d, a, e, f, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_515([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_471([a, g, c, e, f, b, h, i], is_dual)
    } else {
        select_516([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_519([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_36([a, b, c, d, e, f, g], is_dual)
    } else {
        select_26([e, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_518([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_39([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_519([a, i, b, c, g, f, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_517([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_68([f, a, b, c, d, g, e, h, i], is_dual)
    } else {
        select_518([e, a, b, c, d, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_514([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_515([a, b, d, h, f, g, c, i, j], is_dual)
    } else {
        select_517([c, e, f, g, d, a, b, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_510([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_511([a, b, c, d, f, e, i, h, k, j, l], is_dual)
    } else {
        select_514([a, b, d, e, g, c, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_505([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_506([a, b, d, e, f, c, g, h, j, i, k], is_dual)
    } else {
        select_510([a, b, c, d, f, e, g, i, j, h, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_70([a, d, e, f, j, h, i], is_dual)
    } else {
        select_413([a, b, d, c, i, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_523([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_249([a, d, c, b, i, h, g, k, l, j], is_dual)
    } else {
        select_524([b, d, f, e, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_453([a, h, b, c, d, f, i, j, g, k, l], is_dual)
    } else {
        select_523([b, c, d, a, f, e, i, g, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_521([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_522([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_522([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_520([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_521([a, c, e, d, g, f, b, i, j, h, l, k], is_dual)
    } else {
        select_510([a, i, b, c, e, d, g, k, j, h, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_504([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_505([a, i, b, c, e, d, f, h, j, k, m, l], is_dual)
    } else {
        select_520([a, b, d, f, c, g, e, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_503([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_504([g, d, e, c, f, b, a, h, k, j, l, i, m], is_dual)
    } else {
        select_504([g, d, f, c, e, b, a, h, k, i, l, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_502([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_503([h, g, f, a, d, e, b, c, i, j, k, l, m], is_dual)
    } else {
        select_503([h, g, f, c, d, e, b, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_501([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_502([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_502([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_500([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_501([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_501([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_535([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_148([a, f, b, d, e, h, i], is_dual)
    } else {
        select_148([a, e, c, d, f, g, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_536([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_380([a, b, d, e], is_dual)
    } else {
        select_62([c, a, b, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_534([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_535([b, a, c, d, e, f, h, i, g], is_dual)
    } else {
        select_536([a, g, b, d, i, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_533([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_534([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_534([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_537([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_59([a, b, e, c, d, f, g, h, j], is_dual)
    } else {
        select_403([c, d, g, a, b, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_532([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_533([b, c, e, d, g, h, a, i, j], is_dual)
    } else {
        select_537([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_541([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_17([a, b, c, d, f], is_dual)
    } else {
        select_17([a, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_540([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_541([a, c, d, e, f, g], is_dual)
    } else {
        select_351([a, b, c, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_544([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_32([a, c, d, e, f, h], is_dual)
    } else {
        select_26([b, c, d, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_543([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_544([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_80([b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_542([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_543([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_543([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_539([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_540([a, b, f, g, h, i, j, l], is_dual)
    } else {
        select_542([a, b, c, d, e, j, k, i, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_547([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_18([c, d, e, i], is_dual)
    } else {
        select_132([a, b, h, f, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_546([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_63([a, h, c, d, e, f, i, k], is_dual)
    } else {
        select_547([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_545([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_546([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_546([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_539([b, c, a, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_545([b, c, d, f, g, h, i, a, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_532([a, c, d, e, j, f, b, i, k, l], is_dual)
    } else {
        select_538([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_551([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_202([a, b, d, e, f, g, h], is_dual)
    } else {
        select_70([a, c, b, e, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_550([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_268([a, b, g, c, e, h, f, i], is_dual)
    } else {
        select_551([a, c, b, e, d, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_549([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_550([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_550([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_548([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_549([a, b, c, e, i, f, d, h, j], is_dual)
    } else {
        select_514([a, b, c, e, f, d, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_530([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_531([a, b, c, d, e, g, f, i, j, l, m, k], is_dual)
    } else {
        select_548([a, b, c, d, f, h, j, i, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_554([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_226([a, b, c, d, f, g, h, e, i, j], is_dual)
    } else {
        select_226([a, b, c, e, f, g, h, d, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_557([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_202([c, f, e, a, b, g, h], is_dual)
    } else {
        select_202([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_556([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_557([c, d, a, e, b, h, g, i, f], is_dual)
    } else {
        select_557([c, d, b, e, a, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_555([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_556([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_556([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_553([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_554([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_555([a, b, d, e, f, i, h, c, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_562([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_291([c, d, e, f, h], is_dual)
    } else {
        select_138([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_563([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_122([a, i, b, j, g, k], is_dual)
    } else {
        select_291([c, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_561([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_562([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_563([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_560([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_561([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_561([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_559([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_232([a, b, d, i, j, h, c, k], is_dual)
    } else {
        select_560([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_564([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_422([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_422([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_558([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_559([a, b, c, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_564([a, b, c, e, d, j, i, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_552([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_553([a, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_558([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_529([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_530([a, b, d, e, g, c, h, f, i, j, m, k, l], is_dual)
    } else {
        select_552([a, b, c, g, f, d, e, j, i, m, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_528([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_529([a, b, c, e, f, d, g, h, j, k, i, l, m], is_dual)
    } else {
        select_529([a, b, d, e, f, c, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_527([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_528([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    } else {
        select_528([a, b, d, e, f, g, h, i, c, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_526([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_527([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    } else {
        select_527([a, b, c, d, f, g, h, i, j, e, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_574([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_122([a, e, b, h, g, i], is_dual)
    } else {
        select_429([a, c, d, f, e, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_576([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_23([b, f, d, a, h], is_dual)
    } else {
        select_132([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_575([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_576([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_576([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_573([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_574([a, e, d, c, g, h, f, j, i], is_dual)
    } else {
        select_575([a, d, b, f, g, e, j, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_578([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_257([b, d, a, c, e, f, g, i, h, j], is_dual)
    } else {
        select_444([a, d, c, f, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_579([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_412([a, b, c, e, g, h, i, j], is_dual)
    } else {
        select_148([b, c, d, h, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_577([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_578([a, d, f, c, h, g, e, j, k, i], is_dual)
    } else {
        select_579([c, e, b, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_572([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_573([a, g, c, d, i, f, h, e, j, k], is_dual)
    } else {
        select_577([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_571([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_572([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_572([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_583([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_359([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_337([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_582([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_583([a, d, b, g, e, h, f, j, i], is_dual)
    } else {
        select_583([a, d, c, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_581([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_582([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_582([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_586([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_71([a, d, b, c, e, f], is_dual)
    } else {
        select_80([a, b, c, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_585([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_368([a, e, d, b, f, g, h], is_dual)
    } else {
        select_586([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_587([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_583([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_583([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_584([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_585([a, b, f, e, c, g, i, h], is_dual)
    } else {
        select_587([a, b, e, c, d, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_580([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_581([a, b, d, g, e, f, i, h, k, j], is_dual)
    } else {
        select_584([a, b, f, h, c, j, g, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_570([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_571([a, b, e, d, f, c, g, h, i, j, k], is_dual)
    } else {
        select_580([a, b, e, c, f, d, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_569([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_570([a, b, d, e, i, f, j, h, c, k, l], is_dual)
    } else {
        select_510([a, b, d, e, c, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_593([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_438([e, a, b, d, f], is_dual)
    } else {
        select_181([d, b, c, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_195([i, g, a, e, h, b, c], !is_dual)
    } else {
        select_122([a, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_592([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_593([c, b, g, e, a, i], is_dual)
    } else {
        select_594([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_596([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_71([e, c, a, d, f, g], is_dual)
    } else {
        select_71([a, b, d, f, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_595([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_596([a, c, d, e, f, g, i, h], is_dual)
    } else {
        select_586([a, b, h, c, f, e, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_591([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_592([a, f, b, d, h, g, i, e, j], is_dual)
    } else {
        select_595([b, a, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_599([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_38([g, c, d, e, a, h], is_dual)
    } else {
        select_61([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_598([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_70([a, c, d, e, h, g, j], is_dual)
    } else {
        select_599([a, b, d, e, g, f, h, i, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_601([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_195([g, f, a, d, e, b, c], !is_dual)
    } else {
        select_217([g, d, a, e, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_602([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_429([a, c, d, f, g, e, h], is_dual)
    } else {
        select_186([a, b, c, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_600([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_601([i, f, g, a, b, c, h], !is_dual)
    } else {
        select_602([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_597([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_598([b, c, a, d, e, g, f, i, h, k, j], is_dual)
    } else {
        select_600([a, b, h, c, g, f, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_590([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_591([a, b, g, e, f, c, h, j, i, k], is_dual)
    } else {
        select_597([a, b, e, c, d, f, h, g, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_605([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_337([g, h, e, a, b, f, c], !is_dual)
    } else {
        select_127([a, c, e, d, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_604([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_605([b, a, c, d, e, g, f, h], is_dual)
    } else {
        select_264([c, b, g, f, h, a, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_603([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_489([a, b, f, d, e, h, g, i, j], is_dual)
    } else {
        select_604([a, b, c, i, e, j, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_589([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_590([b, a, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_603([b, c, g, e, f, i, h, a, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_588([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_589([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_589([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_568([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_569([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_588([a, b, c, e, h, g, j, i, d, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_612([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_168([g, a, c, d, e, h, i], is_dual)
    } else {
        select_217([c, b, e, h, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_613([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_332([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_332([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_611([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_612([c, d, a, e, b, f, g, h, i], is_dual)
    } else {
        select_613([d, b, c, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_616([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_64([a, g, c, i], is_dual)
    } else {
        select_132([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_615([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_398([a, c, d, g, h, f, i], is_dual)
    } else {
        select_616([a, b, c, f, e, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_614([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_216([c, e, d, a, f, g, h, i], is_dual)
    } else {
        select_615([b, c, a, d, f, g, h, e, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_610([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_611([a, b, g, c, i, f, h, e, j], is_dual)
    } else {
        select_614([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_609([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_610([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_610([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_619([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_31([e, f, d, h, g, a], !is_dual)
    } else {
        select_117([b, c, a, e, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_618([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_619([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_199([e, f, h, d, a, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_617([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_618([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_618([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_608([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_609([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_617([a, b, f, g, h, i, j, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_624([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_122([a, b, c, d, e, f], is_dual)
    } else {
        select_159([a, d, c, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_625([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_429([a, b, f, d, e, h, g], is_dual)
    } else {
        select_429([a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_623([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_624([a, e, f, g, h, i], is_dual)
    } else {
        select_625([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_622([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_623([b, c, g, e, f, i, h, a, j], is_dual)
    } else {
        select_260([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_621([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_622([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_622([a, b, e, d, f, g, c, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_629([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_181([e, a, f, g, h], is_dual)
    } else {
        select_117([a, b, c, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_628([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_55([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_629([b, h, c, f, a, g, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_631([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_127([f, a, b, c, e, g], is_dual)
    } else {
        select_127([e, a, b, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_630([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_202([b, c, d, a, e, f, g], is_dual)
    } else {
        select_631([b, c, d, g, f, a, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_627([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_628([a, b, e, c, d, h, g, j, k, i], is_dual)
    } else {
        select_630([a, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_633([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_332([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_275([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_632([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_238([a, c, d, e, g, h], is_dual)
    } else {
        select_633([a, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_626([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_627([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_632([a, i, c, d, f, b, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_620([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_621([a, g, c, d, e, b, h, i, j, k], is_dual)
    } else {
        select_626([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_607([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_608([a, b, g, d, f, j, h, c, i, k], is_dual)
    } else {
        select_620([a, b, c, d, f, e, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_164([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_118([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_637([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_638([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_198([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_639([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_199([a, b, c, d, e, f], is_dual)
    } else {
        select_31([a, b, c, g, f, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_636([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_637([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_639([a, b, f, d, g, h, c], is_dual)
    }
}
/// n = 11, i = 3
fn select_635([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_636([a, b, c, d, e, j, h, k], is_dual)
    } else {
        select_155([a, b, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_643([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_202([a, b, c, e, f, d, g], is_dual)
    } else {
        select_202([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_642([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_89([a, b, f, c, d, e, g, i, j], is_dual)
    } else {
        select_643([a, c, d, e, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_645([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_202([b, c, e, a, f, g, h], is_dual)
    } else {
        select_70([a, d, b, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_644([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_645([e, b, c, f, a, g, d, i, h], is_dual)
    } else {
        select_557([b, c, e, d, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_641([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_642([a, b, d, e, f, c, h, g, i, j], is_dual)
    } else {
        select_644([a, d, e, f, c, b, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_640([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_641([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_641([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_634([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_635([a, b, c, f, g, d, e, j, i, h, k], is_dual)
    } else {
        select_640([a, b, c, f, d, e, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_606([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_607([a, b, c, f, g, e, h, i, d, j, k], is_dual)
    } else {
        select_634([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_567([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_568([a, b, c, d, e, g, f, i, h, k, j, l], is_dual)
    } else {
        select_606([a, b, c, d, f, e, i, k, h, l, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_649([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_621([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_621([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_648([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_608([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_649([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_652([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_238([a, b, g, h, c, f], !is_dual)
    } else {
        select_351([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_651([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_279([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_652([a, b, c, d, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_650([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_651([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_651([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_647([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_648([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_650([a, b, c, d, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_659([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_341([g, h, a, f, e, b, d], !is_dual)
    } else {
        select_385([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_658([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_624([a, d, f, e, b, h], is_dual)
    } else {
        select_659([a, c, d, b, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_661([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_44([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_44([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_660([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_661([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_661([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_657([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_658([b, c, d, h, a, g, i, j], is_dual)
    } else {
        select_660([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_656([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_401([a, d, e, f, b, c, g, h, i], is_dual)
    } else {
        select_657([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_665([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_80([a, b, d, g, f], is_dual)
    } else {
        select_177([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_664([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_665([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_355([a, d, b, c, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_666([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_331([a, f, b, d, i, e, g, j, h], is_dual)
    } else {
        select_331([a, e, c, d, h, f, g, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_663([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_664([b, a, c, d, e, g, h, i, f, j], is_dual)
    } else {
        select_666([b, c, d, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_662([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_663([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_663([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_655([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_656([a, c, d, e, b, f, g, i, h, j], is_dual)
    } else {
        select_662([a, c, d, e, f, g, h, i, b, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_670([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_368([a, h, i, b, f, c, g], !is_dual)
    } else {
        select_351([a, b, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_669([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_670([a, b, c, d, g, f, i, j, h], is_dual)
    } else {
        select_279([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_668([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_669([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_669([a, d, c, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_673([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_30([a, c, d, e, b, f, g], is_dual)
    } else {
        select_27([b, f, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_672([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_268([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_673([a, b, e, c, d, f, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_389([a, d, b, c, f, h, i, g], is_dual)
    } else {
        select_672([a, d, b, c, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_667([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_668([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_671([a, c, d, e, g, i, h, b, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_654([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_655([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_667([a, b, d, e, c, f, h, i, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_677([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_279([a, b, c, e, f, g, h, i], is_dual)
    } else {
        select_166([a, b, d, c, g, f, i, j, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_679([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_368([a, d, c, f, e, h, g], is_dual)
    } else {
        select_181([a, g, b, h, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_680([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_202([a, b, e, d, f, g, h], is_dual)
    } else {
        select_90([a, c, b, d, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_678([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_679([a, c, g, f, i, h, b, j], is_dual)
    } else {
        select_680([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_676([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_677([a, b, g, c, e, f, h, j, i, k], is_dual)
    } else {
        select_678([a, b, c, d, h, f, i, j, g, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_684([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_286([b, g, c, a, f, h], is_dual)
    } else {
        select_44([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_683([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_684([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_310([a, b, d, e, f, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_685([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_684([a, b, c, e, f, g, h, j], is_dual)
    } else {
        select_71([a, d, e, g, f, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_683([a, b, e, d, i, h, f, k], is_dual)
    } else {
        select_685([a, b, e, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_687([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_601([g, h, d, e, i, a, f], !is_dual)
    } else {
        select_612([b, c, a, e, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_689([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_36([a, b, c, d, e, f, g], is_dual)
    } else {
        select_64([a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_688([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_137([b, e, d, f, g, h], is_dual)
    } else {
        select_689([a, c, b, g, e, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_686([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_687([b, c, g, e, f, h, a, i, j], is_dual)
    } else {
        select_688([b, a, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_681([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_682([a, b, e, c, d, f, h, g, i, j, k], is_dual)
    } else {
        select_686([a, g, c, e, b, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_675([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_676([a, b, e, c, f, g, d, h, i, j, k], is_dual)
    } else {
        select_681([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_674([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_655([a, b, d, h, c, e, j, g, k, i], is_dual)
    } else {
        select_675([a, b, d, c, f, e, h, g, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_653([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_654([a, b, g, d, i, e, c, h, k, j], is_dual)
    } else {
        select_674([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_646([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_647([a, b, c, g, e, i, h, k, j, d], is_dual)
    } else {
        select_653([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_566([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_567([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_646([a, b, c, d, f, i, k, h, l, j, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_697([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_616([b, a, d, c, f, g, e, i, h], is_dual)
    } else {
        select_275([b, a, e, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_696([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_397([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_697([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_695([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_696([a, d, c, b, e, g, h, f, j, i], is_dual)
    } else {
        select_227([a, d, f, e, g, b, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_694([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_226([a, b, d, g, e, f, h, i, j, k], is_dual)
    } else {
        select_695([a, b, c, d, j, g, h, k, i, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_699([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_632([a, b, c, d, i, g, h, j, k], is_dual)
    } else {
        select_403([a, c, d, e, f, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_698([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_699([a, b, c, d, e, h, g, i, k, j, l], is_dual)
    } else {
        select_302([a, b, e, f, c, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_693([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_694([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    } else {
        select_698([a, c, e, f, b, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_692([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_693([a, b, d, c, e, f, i, h, j, k, g, l], is_dual)
    } else {
        select_693([a, c, d, b, e, f, i, g, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_691([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_692([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_692([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_691([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_691([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_565([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_566([a, b, c, d, e, g, h, i, j, k, f, l], is_dual)
    } else {
        select_690([a, b, c, d, e, g, f, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_525([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_526([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_565([a, b, c, d, e, f, h, i, k, l, g, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_499([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_500([a, b, c, d, e, f, g, h, j, k, m, l, n], is_dual)
    } else {
        select_525([a, b, c, d, e, f, g, h, i, l, n, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_703([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_9([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_9([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_702([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_703([a, b, c, f, g, d, h, e, i, k, l, m, j, n], is_dual)
    } else {
        select_703([a, b, c, f, g, e, h, d, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_701([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_702([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_702([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_700([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_701([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_701([a, b, c, d, e, g, h, i, j, k, f, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_498([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_499([a, b, c, d, e, f, g, j, i, k, l, m, h, n], is_dual)
    } else {
        select_700([a, b, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < n) || (is_dual && i > n) {
        select_104([a, b, c, d, e, f, g, h, j, k, l, m, n, i], is_dual)
    } else {
        select_498([a, b, c, d, e, f, g, h, j, i, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < j) || (is_dual && n > j) {
        select_3([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    } else {
        select_103([a, b, c, d, e, f, g, h, i, k, l, m, n, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
