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
/// n = 4, i = 1
fn select_18([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_15([a, b, c], is_dual)
    } else {
        select_17([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_13([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_14([c, d, e, f], is_dual)
    } else {
        select_18([a, b, g, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_20([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_16([a, b], is_dual)
    } else {
        select_17([c], is_dual)
    }
}
/// n = 7, i = 2
fn select_19([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_14([b, c, d, e], is_dual)
    } else {
        select_20([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_12([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_13([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_19([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_22([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_14([b, c, d, e], is_dual)
    } else {
        select_16([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_23([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_14([d, e, f, g], is_dual)
    } else {
        select_14([a, b, c, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_22([g, d, e, f, a, h], is_dual)
    } else {
        select_23([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_11([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_12([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_21([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_27([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_16([a, d], is_dual)
    } else {
        select_16([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_20([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_28([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_20([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_25([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_26([a, b, c, e, d, f], is_dual)
    } else {
        select_28([d, c, a, e, f], is_dual)
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
/// n = 5, i = 1
fn select_31([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_15([b, c, d], is_dual)
    } else {
        select_16([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_31([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_25([a, f, c, b, e, g], is_dual)
    } else {
        select_29([a, c, b, d, e, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_10([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_11([a, b, g, d, e, f, h, i, j], is_dual)
    } else {
        select_24([a, b, c, i, g, j, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_13([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_33([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_34([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_34([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_37([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_31([b, a, c, d, e], is_dual)
    } else {
        select_27([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 1
fn select_36([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_29([a, c, b, d, e, f], is_dual)
    } else {
        select_37([a, c, f, e, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_40([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_20([d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_40([d, f, e, a, b], !is_dual)
    } else {
        select_40([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_38([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([b, c, a, d, e, f], is_dual)
    } else {
        select_39([b, c, f, e, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_35([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_36([a, c, b, e, d, f], is_dual)
    } else {
        select_38([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_32([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_33([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_35([a, b, c, d, i, j, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_9([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_10([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_32([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 0
fn select_45([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_14([a, b, c, d], is_dual)
    } else {
        select_14([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_44([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_29([a, b, c, j, h, k], is_dual)
    } else {
        select_45([d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_45([c, d, e, f, h], is_dual)
    } else {
        select_37([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_43([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_44([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_46([a, b, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_42([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_43([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_43([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_38([a, c, e, b, h, f, g], is_dual)
    } else {
        select_38([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_25([a, c, d, b, e, f], is_dual)
    } else {
        select_37([a, b, d, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_38([a, c, d, b, f, e, g], is_dual)
    } else {
        select_50([a, f, b, c, g, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_48([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_49([a, f, c, e, b, h, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_41([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_42([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_47([a, b, d, j, e, c, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_8([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_9([a, b, c, i, f, g, h, d, j, k], is_dual)
    } else {
        select_41([a, b, c, d, e, f, g, h, i, j, k], is_dual)
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
/// n = 5, i = 2
fn select_56([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_57([a, b, c, d], is_dual)
    } else {
        select_20([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_25([a, b, c, d, e, f], is_dual)
    } else {
        select_56([a, d, c, f, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_54([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_55([a, b, c, d, e, f], is_dual)
    } else {
        select_56([a, e, f, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_53([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_54([a, b, d, c, e, f], is_dual)
    } else {
        select_54([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([b, c, d, e, f], is_dual)
    } else {
        select_20([a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_59([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_60([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_60([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_14([c, d, e, f], is_dual)
    } else {
        select_57([a, b, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([b, c, e], is_dual)
    } else {
        select_27([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_65([a, b, c, e, f, g, h], is_dual)
    } else {
        select_15([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_62([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_63([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_64([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_61([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_62([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_62([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_59([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_61([a, b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_52([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_53([a, b, c, h, d, i], is_dual)
    } else {
        select_58([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_68([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_46([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_46([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_67([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_68([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_68([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_66([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_67([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_67([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_51([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_52([a, b, c, d, e, f, k, j, l], is_dual)
    } else {
        select_66([a, b, c, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_8([a, b, c, d, j, g, h, i, e, k, l], is_dual)
    } else {
        select_51([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_14([c, d, e, g], is_dual)
    } else {
        select_31([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_74([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_63([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_75([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_77([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_14([c, d, e, f], is_dual)
    } else {
        select_15([a, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_76([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_77([b, f, c, d, e, g, h], is_dual)
    } else {
        select_13([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_74([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_76([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_73([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_49([a, c, d, h, g, b, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_79([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_74([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_34([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_82([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([a, f, c, d, e, g], is_dual)
    } else {
        select_30([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_83([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_26([e, a, c, f, d, g], is_dual)
    } else {
        select_26([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_81([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_82([b, c, a, d, e, f, g], is_dual)
    } else {
        select_83([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_80([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_81([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_50([a, f, b, g, h, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_79([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_80([a, c, i, d, g, b, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_71([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_72([a, b, c, d, e, h, g, j, i], is_dual)
    } else {
        select_78([a, b, c, f, d, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_70([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_71([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_71([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_89([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_14([a, b, c, d], is_dual)
    } else {
        select_17([e], is_dual)
    }
}
/// n = 5, i = 2
fn select_90([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_40([b, c, a, d, e], is_dual)
    } else {
        select_27([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_88([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_89([a, c, d, e, f], is_dual)
    } else {
        select_90([a, b, f, g, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_87([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_88([a, b, c, h, f, g, i], is_dual)
    } else {
        select_62([a, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_86([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_87([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_80([a, c, h, d, f, b, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_89([b, c, d, e, f], is_dual)
    } else {
        select_40([e, g, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_92([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_93([a, c, d, e, b, f, g], is_dual)
    } else {
        select_93([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_95([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_82([a, c, d, e, b, f, g], is_dual)
    } else {
        select_82([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_95([a, c, d, b, e, f, g], is_dual)
    } else {
        select_38([a, c, g, d, b, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_92([a, b, c, d, g, h, f], is_dual)
    } else {
        select_94([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_85([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_86([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_91([a, b, c, d, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_84([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_85([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_85([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_69([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_70([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_84([a, b, c, d, h, g, e, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_7([a, b, c, d, e, g, f, h, i, j, k, l], is_dual)
    } else {
        select_69([a, b, c, d, e, k, g, f, j, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_99([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_78([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_78([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_98([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_52([a, b, c, d, f, g, h, j, k], is_dual)
    } else {
        select_99([a, b, c, d, e, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_97([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_8([a, b, c, d, f, g, h, i, e, j, k], is_dual)
    } else {
        select_98([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_102([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_72([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_72([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_106([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_29([a, b, d, e, c, f], is_dual)
    } else {
        select_29([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_95([a, d, e, b, c, f, g], is_dual)
    } else {
        select_106([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_105([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_105([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_88([a, b, c, g, e, f, h], is_dual)
    } else {
        select_88([a, b, d, f, e, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_110([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_25([a, b, c, d, e, f], is_dual)
    } else {
        select_20([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_109([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_88([a, b, c, d, e, f, g], is_dual)
    } else {
        select_110([a, b, f, e, c, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_107([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_108([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_109([a, b, c, g, e, h, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_104([d, a, b, c, e, f, g, h], is_dual)
    } else {
        select_107([d, a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_102([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_103([d, e, h, a, b, g, i, c], is_dual)
    }
}
/// n = 9, i = 2
fn select_100([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_101([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_101([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_96([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_97([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_100([a, b, c, d, e, g, j, f, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_6([a, b, c, d, e, f, i, g, h, j, k, l], is_dual)
    } else {
        select_96([a, b, c, d, e, f, g, h, j, l, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_117([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_25([a, b, c, e, f, g], is_dual)
    } else {
        select_31([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_116([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_55([a, c, f, e, b, g], is_dual)
    } else {
        select_117([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_11([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_116([a, b, h, d, c, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_74([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_74([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_33([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_119([a, b, h, e, f, g, c, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_115([a, b, c, d, e, j, h, i, k], is_dual)
    } else {
        select_118([a, b, c, d, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_30([b, c, d, e, g, f], is_dual)
    } else {
        select_18([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_124([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_30([c, g, d, e, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_126([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_57([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_125([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_126([a, c, b, h, g, f, i], is_dual)
    } else {
        select_13([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_123([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_125([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_121([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_122([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_95([a, c, g, e, f, d, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_121([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_121([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_114([a, b, c, f, g, d, e, h, i, j, k], is_dual)
    } else {
        select_120([a, b, c, d, e, h, i, k, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_82([c, d, e, f, g, h, i], is_dual)
    } else {
        select_82([a, b, g, h, e, f, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_29([a, b, f, g, e, i], is_dual)
    } else {
        select_82([c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_131([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_132([a, f, c, d, i, g, h, e, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_130([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_130([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_134([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_21([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_117([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_94([a, b, c, d, h, g, j, i], is_dual)
    } else {
        select_134([c, g, e, f, a, b, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_128([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_129([a, b, c, e, f, i, g, h, j, k], is_dual)
    } else {
        select_133([a, b, g, f, c, d, h, i, k, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_129([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_129([a, b, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_127([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_128([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_135([a, b, g, e, f, c, j, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_113([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_127([a, b, c, e, f, g, h, i, j, d, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_112([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_112([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_5([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_111([a, b, c, d, e, f, i, j, k, h, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_63([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_63([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_146([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_65([a, c, d, b, e, f, g], is_dual)
    } else {
        select_65([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_145([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_63([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_146([a, b, c, h, g, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_144([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_145([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_142([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_143([a, b, c, e, d, h, g, i, k, j], is_dual)
    } else {
        select_143([a, b, d, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_141([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_142([c, d, a, e, b, f, h, i, j, g, k], is_dual)
    } else {
        select_142([c, d, b, e, a, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_140([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_141([c, d, a, b, e, f, g, h, j, k, i], is_dual)
    } else {
        select_141([c, d, a, b, f, e, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_139([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_140([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    } else {
        select_140([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_152([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_29([b, d, a, e, f, g], is_dual)
    } else {
        select_29([a, c, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_151([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_152([a, b, f, d, h, g, e, i], is_dual)
    } else {
        select_132([b, d, a, c, g, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_150([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_130([a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_151([a, b, e, c, f, i, g, k, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_149([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_150([c, d, e, a, b, h, j, f, g, i, k], is_dual)
    } else {
        select_150([c, e, d, a, b, h, i, f, g, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_157([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_26([a, b, c, e, d, f], is_dual)
    } else {
        select_27([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_158([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_26([a, f, b, e, d, g], is_dual)
    } else {
        select_31([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_156([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_157([a, b, f, d, g, e], is_dual)
    } else {
        select_158([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_160([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_65([b, c, d, e, g, f, h], is_dual)
    } else {
        select_57([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_159([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_160([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_160([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_155([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_156([a, b, g, e, f, c, h], is_dual)
    } else {
        select_159([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_154([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_151([a, b, e, d, f, h, g, j, i], is_dual)
    } else {
        select_155([a, b, c, e, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_153([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_154([c, b, d, a, e, g, f, i, j, h], is_dual)
    } else {
        select_154([c, b, e, a, d, g, f, h, j, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_148([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_149([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_153([f, b, c, d, e, k, h, i, j, g], is_dual)
    }
}
/// n = 11, i = 2
fn select_147([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_148([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    } else {
        select_148([a, b, c, d, f, g, h, i, j, e, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_138([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_139([a, e, b, c, d, f, g, i, j, k, h], is_dual)
    } else {
        select_147([a, b, c, d, f, g, h, i, j, k, e], is_dual)
    }
}
/// n = 11, i = 2
fn select_166([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_74([f, a, d, e, i, h, g, j, k], is_dual)
    } else {
        select_74([f, a, b, c, j, h, g, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_166([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_166([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([f, g, a, d, e, i, h, j, k], is_dual)
    } else {
        select_11([f, g, a, b, c, j, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_165([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_167([h, a, b, c, d, f, g, e, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_171([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_15([b, c, d], is_dual)
    } else {
        select_20([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_170([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_171([a, c, d, b, e, f], is_dual)
    } else {
        select_171([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_169([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_94([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_170([a, b, f, g, h, c], is_dual)
    }
}
/// n = 10, i = 2
fn select_168([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_169([a, b, c, e, i, h, d, j], is_dual)
    } else {
        select_118([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_164([d, i, g, h, e, a, b, c, k, j, l], is_dual)
    } else {
        select_168([a, b, c, d, f, e, j, i, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_162([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_163([a, b, c, f, d, h, e, g, i, k, l, j], is_dual)
    } else {
        select_163([a, b, c, f, e, h, d, g, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_161([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_162([a, b, c, f, g, d, h, e, j, k, l, i], is_dual)
    } else {
        select_162([a, b, c, f, g, e, h, d, i, k, l, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_138([e, f, g, k, a, b, c, i, j, l, d], is_dual)
    } else {
        select_161([a, b, c, e, f, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_137([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_137([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_4([a, b, c, d, e, f, h, i, g, k, j, l], is_dual)
    } else {
        select_136([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_179([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_95([a, c, e, b, f, g, h], is_dual)
    } else {
        select_95([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_183([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, b, e, f], is_dual)
    } else {
        select_27([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_182([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_183([a, b, e, f, d, g], is_dual)
    } else {
        select_31([b, c, d, f, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_181([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_182([g, c, b, a, f, i, h], is_dual)
    } else {
        select_75([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([a, d, b, h, e, g], is_dual)
    } else {
        select_25([a, d, c, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_180([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_181([a, c, e, b, d, f, g, h, i], is_dual)
    } else {
        select_184([a, c, g, e, f, b, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_178([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_179([a, b, f, e, c, h, g, j, i], is_dual)
    } else {
        select_180([a, b, g, e, d, i, h, f, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_177([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_129([a, f, d, b, e, g, i, h, j, k], is_dual)
    } else {
        select_178([a, b, d, c, h, f, g, k, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_158([a, b, f, d, e, h, g], is_dual)
    } else {
        select_158([a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_188([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_188([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_186([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_187([d, j, a, b, e, f, h, i], is_dual)
    } else {
        select_187([d, h, b, c, f, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_190([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_81([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_24([a, e, b, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_192([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_45([b, c, d, e, g], is_dual)
    } else {
        select_27([a, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_191([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_46([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_192([g, c, d, e, f, h, a, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_189([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_190([a, b, k, c, l, g, h, i, j], is_dual)
    } else {
        select_191([a, b, d, e, f, j, g, i, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < k) || (is_dual && b > k) {
        select_186([c, d, k, a, g, h, b, i, j, l], is_dual)
    } else {
        select_189([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_177([a, c, i, d, e, b, g, h, l, j, k], is_dual)
    } else {
        select_185([a, c, d, e, b, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_144([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_144([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_195([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_143([a, b, c, d, h, i, g, j, l, k], is_dual)
    } else {
        select_196([a, b, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_194([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_142([b, c, a, d, h, f, k, i, j, g, l], is_dual)
    } else {
        select_195([b, c, d, f, a, e, i, g, h, j, l, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_193([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_194([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_194([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_175([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_176([a, b, e, d, f, g, h, i, j, c, k, l], is_dual)
    } else {
        select_193([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_174([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_175([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_175([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_200([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_86([a, b, c, d, j, g, h, k, i], is_dual)
    } else {
        select_78([a, b, c, h, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_199([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_200([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_200([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_204([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_95([a, b, c, d, e, f, g], is_dual)
    } else {
        select_18([b, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_203([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_94([a, b, c, e, f, g, h, i], is_dual)
    } else {
        select_204([a, b, d, c, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_129([a, b, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_203([a, b, c, d, h, g, j, i, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_201([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_202([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_202([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_199([a, b, c, d, f, e, g, h, j, i, k], is_dual)
    } else {
        select_201([a, b, c, d, f, i, h, e, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_198([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_201([a, b, c, d, f, i, h, k, j, e], is_dual)
    }
}
/// n = 12, i = 2
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_174([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_197([a, b, c, d, e, g, j, i, l, k, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_208([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_141([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_141([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_207([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_208([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_208([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_211([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_195([c, d, a, b, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_195([c, d, b, a, e, f, g, i, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_210([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_211([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_211([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_209([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_210([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_210([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_207([a, b, c, d, f, g, e, j, l, k, i], is_dual)
    } else {
        select_209([a, b, c, d, f, g, e, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_206([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_206([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_173([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_205([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_3([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_172([a, b, c, d, e, f, g, i, j, k, l, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_2([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
