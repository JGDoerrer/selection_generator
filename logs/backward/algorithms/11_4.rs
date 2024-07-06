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
/// n = 5, i = 1
fn select_15([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_17([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_14([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_15([a, b, f, d, e], is_dual)
    } else {
        select_15([a, c, e, d, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([c, d, e], is_dual)
    } else {
        select_16([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_19([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_20([a, f, c, d, e, g], is_dual)
    } else {
        select_20([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_13([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_14([a, b, g, e, f, d], is_dual)
    } else {
        select_19([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_17([a, d], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_25([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_17([a, b], is_dual)
    } else {
        select_18([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([b, c, e, d], is_dual)
    } else {
        select_25([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_23([e, a, c, f, d, g], is_dual)
    } else {
        select_23([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_19([b, c, a, d, e, f, g], is_dual)
    } else {
        select_22([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_12([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_13([a, d, e, b, c, f, g], is_dual)
    } else {
        select_21([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_25([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, b, e, f], is_dual)
    } else {
        select_29([b, c, d, a, e, f], is_dual)
    }
}
/// n = 4, i = 0
fn select_31([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_16([a, b, c], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_30([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_31([a, b, c, d], is_dual)
    } else {
        select_18([e], is_dual)
    }
}
/// n = 7, i = 2
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_30([b, c, d, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_33([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_15([a, c, d, b, e], is_dual)
    } else {
        select_15([b, c, d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_32([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_28([a, d, f, g, e, c], !is_dual)
    } else {
        select_33([a, f, d, g, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_27([a, b, d, e, f, g, i], is_dual)
    } else {
        select_32([a, h, i, b, f, c, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_12([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_26([a, b, c, d, g, f, i, j, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([a, f, b, e, h, i], is_dual)
    } else {
        select_16([c, d, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_39([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, b, d], is_dual)
    } else {
        select_24([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_38([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([b, c, d, e, f, g], is_dual)
    } else {
        select_39([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_36([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_38([a, f, c, d, e, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_35([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_36([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_36([c, b, d, e, f, a, g, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_34([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_12([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_35([a, b, c, d, g, f, i, j, k, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_10([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([b, d, c, a, e, f, g, h, i, j], is_dual)
    } else {
        select_34([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_9([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_10([a, b, c, e, d, f, g, i, j, h, k], is_dual)
    } else {
        select_10([b, a, c, d, e, f, h, i, k, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_8([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_9([a, c, d, b, e, h, g, i, j, f, k], is_dual)
    } else {
        select_9([b, c, d, a, e, h, f, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_7([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_8([b, c, d, a, e, g, h, f, j, k, i], is_dual)
    } else {
        select_8([b, c, e, a, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_6([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_7([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    } else {
        select_7([a, b, c, d, f, g, h, i, j, e, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_6([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_6([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_5([g, h, a, e, f, b, c, d, i, j, k], is_dual)
    } else {
        select_5([g, h, d, e, f, b, c, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_4([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    } else {
        select_4([a, b, c, d, f, g, h, i, e, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_3([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_3([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_52([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_25([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_51([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_52([b, c, a, d, e], is_dual)
    } else {
        select_24([a, c, b, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_53([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([a, b, c, e], is_dual)
    } else {
        select_39([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_51([a, b, d, f, e], is_dual)
    } else {
        select_53([a, b, c, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_50([a, b, c, d, e, f], is_dual)
    } else {
        select_50([a, b, f, g, e, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, b, e, f], is_dual)
    } else {
        select_24([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([a, b, e, f, d, g], is_dual)
    } else {
        select_15([b, c, d, f, e], is_dual)
    }
}
/// n = 4, i = 1
fn select_59([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([a, b, c], is_dual)
    } else {
        select_18([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_29([a, c, d, e, f, h], is_dual)
    } else {
        select_59([b, c, d, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_55([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_56([a, b, c, d, f, e, g], is_dual)
    } else {
        select_58([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_55([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_55([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_49([a, b, c, f, g, h, e], is_dual)
    } else {
        select_54([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_48([a, b, f, i, g, h, c, d, e], !is_dual)
    } else {
        select_48([a, b, g, i, f, h, c, e, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_64([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, c, d, g], is_dual)
    } else {
        select_24([b, c, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_63([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_56([b, c, d, a, e, f, g], is_dual)
    } else {
        select_64([a, d, c, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_62([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_32([a, e, d, b, f, g, h], is_dual)
    } else {
        select_63([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 3, i = 1
fn select_68([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_25([a, b, c], is_dual)
    } else {
        select_25([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_67([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_68([a, d, e], is_dual)
    } else {
        select_24([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_66([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_67([b, e, d, a, f], is_dual)
    } else {
        select_29([b, a, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_66([a, b, g, e, f, c], !is_dual)
    } else {
        select_32([a, e, g, b, f, c, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_62([a, b, e, d, g, f, h, i], is_dual)
    } else {
        select_65([a, b, c, e, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_71([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_14([a, c, d, b, e, f], is_dual)
    } else {
        select_22([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_70([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_71([a, c, e, b, h, f, g], is_dual)
    } else {
        select_71([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_74([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_29([a, b, c, d, f, g], is_dual)
    } else {
        select_39([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_73([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_74([a, c, d, b, e, f, g], is_dual)
    } else {
        select_74([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_72([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_73([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_71([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_70([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_72([a, f, c, e, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_60([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_61([a, b, e, c, f, d, h, g, i], is_dual)
    } else {
        select_69([a, b, e, d, f, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_47([a, b, d, e, g, h, c, i, j], is_dual)
    } else {
        select_60([a, b, d, e, c, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_81([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([b, c, e], is_dual)
    } else {
        select_24([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_80([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_81([a, b, c, d, e, f, g], is_dual)
    } else {
        select_29([d, b, c, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_79([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_80([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_38([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_79([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_73([a, e, f, i, c, g, d], !is_dual)
    }
}
/// n = 5, i = 2
fn select_85([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_25([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_84([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_23([a, c, b, d, e, f], is_dual)
    } else {
        select_85([a, b, e, f, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([c, d, e, f], is_dual)
    } else {
        select_59([a, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_84([b, g, c, a, f, h], is_dual)
    } else {
        select_86([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_82([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_83([a, b, g, d, e, f, h, j], is_dual)
    } else {
        select_83([a, c, f, d, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_77([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_78([a, f, j, k, b, g, i, c, h], !is_dual)
    } else {
        select_82([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([a, c, d, f, e, h], is_dual)
    } else {
        select_23([b, d, c, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_90([a, b, h, c, f, g, i, j], is_dual)
    } else {
        select_86([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_39([b, c, e, g], is_dual)
    } else {
        select_59([a, d, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_93([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_39([a, g, c, i], is_dual)
    } else {
        select_64([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_92([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_93([b, a, d, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_88([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_89([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_91([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_87([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_78([a, f, j, k, b, h, i, c, g], !is_dual)
    } else {
        select_88([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_76([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_77([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_87([a, b, d, c, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_75([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_76([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_76([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_45([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_46([a, b, c, d, f, g, h, k, i, j], is_dual)
    } else {
        select_75([a, b, c, e, d, f, g, i, k, j, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_100([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_23([a, d, b, c, e, f], is_dual)
    } else {
        select_23([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_99([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_33([b, c, d, e, f], is_dual)
    } else {
        select_100([a, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_101([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_19([a, b, c, d, e, f, g], is_dual)
    } else {
        select_59([a, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_98([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_99([a, b, c, f, e, h, g], is_dual)
    } else {
        select_101([a, d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_104([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([b, c, d, e], is_dual)
    } else {
        select_25([a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_105([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_20([a, b, c, d, e, f], is_dual)
    } else {
        select_15([e, c, d, a, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_103([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_104([e, h, i, j, a, f, b], !is_dual)
    } else {
        select_105([e, b, c, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_99([a, b, c, f, e, g, h], is_dual)
    } else {
        select_103([a, d, b, c, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_97([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_98([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_102([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_96([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_97([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_97([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_27([a, b, d, e, f, g, h], is_dual)
    } else {
        select_28([a, b, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_73([a, b, g, h, f, e, c], !is_dual)
    } else {
        select_73([a, b, g, h, e, f, d], !is_dual)
    }
}
/// n = 10, i = 3
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_107([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_108([a, b, c, h, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_95([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_96([a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_106([a, b, e, c, d, f, g, i, j, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_56([a, e, c, g, f, d, i], is_dual)
    } else {
        select_56([b, d, c, g, f, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_23([g, h, f, d, a, b], !is_dual)
    } else {
        select_39([a, f, c, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_116([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_23([d, f, g, e, a, b], !is_dual)
    } else {
        select_23([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_115([e, b, c, d, f, g, i, h], is_dual)
    } else {
        select_116([g, e, j, d, h, a, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_112([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_113([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_114([c, b, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_119([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_52([a, d, g, f, c], is_dual)
    } else {
        select_52([b, c, g, e, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_118([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_67([g, f, b, c, d], !is_dual)
    } else {
        select_119([g, h, e, f, a, d, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_121([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([f, c, g, e, d, a], !is_dual)
    } else {
        select_52([d, g, c, e, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_84([a, d, c, e, f, h], is_dual)
    } else {
        select_121([a, b, f, d, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_117([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_118([f, h, g, i, a, e, d, c], !is_dual)
    } else {
        select_120([a, c, b, e, g, f, d, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_111([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_112([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_117([b, c, f, e, h, g, a, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_125([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_23([a, f, b, e, d, g], is_dual)
    } else {
        select_15([b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_74([d, e, h, a, b, g, f], !is_dual)
    } else {
        select_125([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_28([a, e, h, i, g, f], !is_dual)
    } else {
        select_14([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_124([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_126([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_129([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_23([a, c, b, d, e, f], is_dual)
    } else {
        select_29([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([a, e, c, d, f, h], is_dual)
    } else {
        select_129([e, g, h, a, b, f, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_127([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_99([b, c, d, a, f, e, g], is_dual)
    } else {
        select_128([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_123([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_127([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_110([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_111([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_122([a, b, i, j, e, f, h, g, d], !is_dual)
    }
}
/// n = 7, i = 3
fn select_134([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([d, f, g, a, e, b], !is_dual)
    } else {
        select_52([e, f, d, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_134([d, g, h, a, e, f, b], !is_dual)
    } else {
        select_15([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_132([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([a, f, b, d, h, e, i, g, j], is_dual)
    } else {
        select_133([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_132([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_132([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_131([a, b, d, c, g, e, h, f, i, j], is_dual)
    } else {
        select_131([a, b, d, c, g, f, h, e, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_110([a, b, e, d, g, h, f, j, i, k], is_dual)
    } else {
        select_130([a, b, f, c, i, j, e, g, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_94([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_95([a, b, c, d, e, g, i, j, f, h, k], is_dual)
    } else {
        select_109([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_44([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_45([a, b, c, e, f, d, h, j, g, i, k], is_dual)
    } else {
        select_94([a, b, c, f, e, h, g, j, i, d, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_142([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([c, d, e, f], is_dual)
    } else {
        select_39([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_143([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_31([c, d, e, g], is_dual)
    } else {
        select_15([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_141([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_142([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_143([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_140([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_141([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_141([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_145([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_19([a, c, d, e, b, f, g], is_dual)
    } else {
        select_19([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_144([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_145([a, b, c, e, f, d, g], is_dual)
    } else {
        select_145([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_140([a, b, f, c, d, e, g, i, j], is_dual)
    } else {
        select_144([a, c, d, e, b, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_149([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_23([a, h, b, f, i, j], is_dual)
    } else {
        select_31([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_142([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_149([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_150([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_86([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_86([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_147([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_148([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_150([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_146([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_147([a, b, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_144([a, d, e, f, b, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_138([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_139([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_146([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_153([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_66([a, b, c, d, e, f], is_dual)
    } else {
        select_32([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_156([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_23([a, d, c, e, f, g], is_dual)
    } else {
        select_39([a, b, d, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_155([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_58([b, a, c, d, e, f, h, g], is_dual)
    } else {
        select_156([a, g, h, i, e, f, b], !is_dual)
    }
}
/// n = 6, i = 2
fn select_158([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_23([a, b, c, e, d, f], is_dual)
    } else {
        select_24([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 3
fn select_157([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_134([d, f, g, a, e, b, c], !is_dual)
    } else {
        select_158([a, f, g, d, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_154([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_155([a, b, c, d, e, g, h, i, f], is_dual)
    } else {
        select_157([a, i, g, e, f, c, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_153([a, i, e, f, b, g, c], !is_dual)
    } else {
        select_154([a, i, h, g, f, c, e, b, d], !is_dual)
    }
}
/// n = 6, i = 2
fn select_163([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, d, c, f], is_dual)
    } else {
        select_24([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_162([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_163([a, b, c, e, d, f], is_dual)
    } else {
        select_163([a, b, d, e, c, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_81([b, a, c, d, e, f, g], is_dual)
    } else {
        select_39([a, e, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_162([d, c, a, f, e, g], is_dual)
    } else {
        select_164([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_161([i, j, k, f, g, a, b, c], !is_dual)
    } else {
        select_36([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_166([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_51([a, h, b, i, e], is_dual)
    } else {
        select_38([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_168([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_81([a, c, d, e, g, f, i], is_dual)
    } else {
        select_57([a, b, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_169([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_81([a, c, d, e, g, f, i], is_dual)
    } else {
        select_39([a, h, b, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_168([b, e, c, d, f, g, h, i, j], is_dual)
    } else {
        select_169([a, f, c, d, e, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_166([a, f, c, d, g, h, e, i, k], is_dual)
    } else {
        select_167([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_159([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_160([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_165([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_151([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_152([a, c, b, h, g, i, j, k, f], is_dual)
    } else {
        select_159([a, c, b, d, e, g, f, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_138([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_151([a, c, d, e, f, g, h, i, b, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_174([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, f, e, g], is_dual)
    } else {
        select_28([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_173([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_174([a, e, b, d, g, f, h], is_dual)
    } else {
        select_71([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_177([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_20([a, b, c, d, e, f], is_dual)
    } else {
        select_39([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_178([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([a, b, d, g, f, i], is_dual)
    } else {
        select_24([c, d, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_176([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_177([a, d, c, h, g, e, j], is_dual)
    } else {
        select_178([a, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_180([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_23([a, c, b, e, f, g], is_dual)
    } else {
        select_59([a, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_180([a, b, c, h, e, g, j], is_dual)
    } else {
        select_56([b, g, d, e, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_176([c, e, b, d, a, f, g, h, i, j], is_dual)
    } else {
        select_179([c, a, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_172([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_173([b, a, f, e, c, g, i, h], is_dual)
    } else {
        select_175([a, b, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_171([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_172([a, c, d, b, e, g, h, i, f, j], is_dual)
    } else {
        select_172([b, c, d, a, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_170([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_171([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_171([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_136([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_137([a, b, d, c, e, f, g, h, j, k, i], is_dual)
    } else {
        select_170([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_99([a, b, c, d, f, g, h], is_dual)
    } else {
        select_33([b, c, d, e, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_188([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_68([a, e, f], is_dual)
    } else {
        select_15([a, b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_188([a, b, g, e, f, h], is_dual)
    } else {
        select_104([a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_186([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_187([c, f, d, e, g, a, h, i], is_dual)
    } else {
        select_89([c, b, a, d, e, g, f, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_184([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_185([c, d, e, a, f, g, i, h], is_dual)
    } else {
        select_186([a, c, b, d, e, g, f, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_183([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_172([c, a, b, e, d, g, f, i, j, h], is_dual)
    } else {
        select_184([c, e, d, a, b, g, f, h, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_193([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_81([c, a, b, d, f, e, g], is_dual)
    } else {
        select_39([a, b, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_39([c, d, e, g], is_dual)
    } else {
        select_39([a, b, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_192([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_193([c, d, b, e, f, a, h, g], is_dual)
    } else {
        select_194([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_191([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_99([a, c, d, e, h, g, i], is_dual)
    } else {
        select_192([a, b, c, d, f, i, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_197([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_81([a, b, c, d, e, f, g], is_dual)
    } else {
        select_68([a, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_28([a, h, d, e, g, k], is_dual)
    } else {
        select_197([j, h, k, i, a, f, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_200([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_24([b, c, d, e], is_dual)
    } else {
        select_25([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_199([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_200([a, c, d, e, f, g, h], is_dual)
    } else {
        select_39([b, f, c, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_28([a, h, c, d, g, j], is_dual)
    } else {
        select_199([k, j, h, i, f, a, e, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_195([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_196([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_198([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_191([a, c, d, e, b, g, f, i, h, j], is_dual)
    } else {
        select_195([a, b, c, d, e, g, h, i, f, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_189([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_190([a, b, e, c, d, f, g, i, j, h, k], is_dual)
    } else {
        select_172([a, c, d, e, b, g, f, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_182([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_183([d, e, c, b, a, g, f, j, i, h], is_dual)
    } else {
        select_189([a, b, d, e, c, g, h, i, j, f, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_181([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_182([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_182([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_135([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_136([a, b, e, d, f, g, c, h, i, j, k], is_dual)
    } else {
        select_181([a, b, e, c, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_43([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_44([a, b, c, e, f, g, h, i, j, d, k], is_dual)
    } else {
        select_135([a, b, c, e, f, d, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_42([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_43([a, b, c, d, e, g, f, j, i, k, h], is_dual)
    } else {
        select_43([a, b, c, d, f, g, e, j, h, k, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_41([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_42([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_42([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_40([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_41([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_41([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_2([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_40([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
