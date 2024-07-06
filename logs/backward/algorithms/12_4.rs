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
/// n = 5, i = 1
fn select_15([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_16([a, b, c, d], is_dual)
    } else {
        select_19([e], is_dual)
    }
}
/// n = 5, i = 1
fn select_21([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_17([b, c, d], is_dual)
    } else {
        select_18([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_20([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_21([a, c, d, b, e], is_dual)
    } else {
        select_21([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_14([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_15([a, c, d, e, f], is_dual)
    } else {
        select_20([a, b, c, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([c, d, e], is_dual)
    } else {
        select_17([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, f, c, d, e, g], is_dual)
    } else {
        select_24([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, e, b, f, g], is_dual)
    } else {
        select_23([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_13([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_14([a, b, d, e, f, h], is_dual)
    } else {
        select_22([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_17([a, b, c], is_dual)
    } else {
        select_19([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([c, d, e, f], is_dual)
    } else {
        select_28([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_27([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_27([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([c, d, e, f], is_dual)
    } else {
        select_17([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_29([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_30([a, g, c, d, e, f, h], is_dual)
    } else {
        select_30([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_25([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_26([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_29([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_12([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_13([a, c, b, e, f, g, d, h], is_dual)
    } else {
        select_25([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 3, i = 1
fn select_36([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_18([a, b], is_dual)
    } else {
        select_19([c], is_dual)
    }
}
/// n = 3, i = 1
fn select_35([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_36([a, b, c], is_dual)
    } else {
        select_36([a, c, b], is_dual)
    }
}
/// n = 4, i = 1
fn select_37([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_18([a, d], is_dual)
    } else {
        select_18([b, c], is_dual)
    }
}
/// n = 5, i = 2
fn select_34([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_35([a, d, e], is_dual)
    } else {
        select_37([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_37([b, c, e, d], is_dual)
    } else {
        select_36([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_38([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_39([a, d, b, c, e, f], is_dual)
    } else {
        select_39([a, d, c, b, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_33([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_34([a, f, e, b, c], !is_dual)
    } else {
        select_38([a, b, c, d, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_41([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_24([a, b, c, d, e, f], is_dual)
    } else {
        select_21([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_41([a, b, d, e, c, f], is_dual)
    } else {
        select_41([a, c, d, e, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_32([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_33([a, c, d, f, b, g], is_dual)
    } else {
        select_40([a, c, d, b, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_44([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_17([b, c, d], is_dual)
    } else {
        select_36([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_43([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_44([a, c, d, b, e, f], is_dual)
    } else {
        select_44([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_15([a, c, d, e, f], is_dual)
    } else {
        select_43([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_31([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_32([a, b, c, d, e, f, g], is_dual)
    } else {
        select_42([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_11([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_12([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_31([a, b, d, e, h, c, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([b, d, a, c, e, f, g], is_dual)
    } else {
        select_43([a, e, c, f, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_51([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_37([a, b, c, d], is_dual)
    } else {
        select_36([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_39([a, b, c, e, d, f], is_dual)
    } else {
        select_51([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_52([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([e, a, c, f, d, g], is_dual)
    } else {
        select_39([d, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_50([e, h, i, a, g, f], !is_dual)
    } else {
        select_52([a, b, c, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_47([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([a, b, c, d, e, g, f, h, j], is_dual)
    } else {
        select_49([f, b, d, a, e, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_16([d, e, f, g], is_dual)
    } else {
        select_16([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_30([a, h, d, e, f, g, i], is_dual)
    } else {
        select_55([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_53([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_26([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_54([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_46([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_47([a, i, c, d, b, g, h, j, k, l], is_dual)
    } else {
        select_53([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([a, c, d, g], is_dual)
    } else {
        select_37([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_59([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_60([a, d, b, g, e, f, h], is_dual)
    } else {
        select_60([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_61([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([d, b, c, e, f, h], is_dual)
    } else {
        select_60([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_59([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_61([e, b, d, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_29([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_58([a, i, b, c, g, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_57([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_57([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_45([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_46([a, b, d, e, f, g, c, h, i, k, j, l], is_dual)
    } else {
        select_56([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_11([a, b, d, e, f, c, h, j, i], is_dual)
    } else {
        select_45([a, b, c, d, g, e, f, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_67([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_21([a, c, d, e, h], is_dual)
    } else {
        select_60([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_69([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_37([a, c, b, d], is_dual)
    } else {
        select_37([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_68([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_44([a, b, c, d, f, g], is_dual)
    } else {
        select_69([b, d, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_66([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_67([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_68([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_65([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_66([a, d, b, g, e, h, f, j, i], is_dual)
    } else {
        select_66([a, d, c, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_64([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_65([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_65([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_71([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_66([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_66([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_73([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_20([a, f, d, g, b], !is_dual)
    } else {
        select_43([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_75([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_39([a, d, c, e, f, g], is_dual)
    } else {
        select_69([a, b, d, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([a, d, c, f], is_dual)
    } else {
        select_37([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_74([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_75([a, b, d, c, e, f, g], is_dual)
    } else {
        select_76([b, d, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_72([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_73([a, e, d, b, f, g, h], is_dual)
    } else {
        select_74([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_70([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_71([a, b, e, c, d, g, f, h, i], is_dual)
    } else {
        select_72([a, b, f, e, c, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_64([a, b, d, g, e, f, i, h, k, j], is_dual)
    } else {
        select_70([a, b, f, h, c, j, g, k, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_81([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_21([b, a, c, d, e], is_dual)
    } else {
        select_37([b, e, d, a], is_dual)
    }
}
/// n = 7, i = 2
fn select_83([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([b, c, e], is_dual)
    } else {
        select_37([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_82([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_83([a, b, f, d, g, e, h], is_dual)
    } else {
        select_83([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_81([a, c, f, h, i], is_dual)
    } else {
        select_82([b, a, d, e, g, f, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_86([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([a, b, e, f], is_dual)
    } else {
        select_37([a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_86([b, c, d, e, f, g], is_dual)
    } else {
        select_76([a, c, d, e, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_87([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_86([a, b, e, f, d, g], is_dual)
    } else {
        select_21([b, c, d, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([a, b, c, e, g, h, i, j], is_dual)
    } else {
        select_87([b, c, d, h, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_79([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_80([a, d, f, c, h, g, e, j, k, i], is_dual)
    } else {
        select_84([c, e, b, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_91([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_37([a, b, c, d], is_dual)
    } else {
        select_36([d, a, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_91([b, f, d, a, h], is_dual)
    } else {
        select_60([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_89([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_90([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_90([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_39([a, f, b, e, d, g], is_dual)
    } else {
        select_21([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_94([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_39([a, b, c, e, d, g], is_dual)
    } else {
        select_39([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_92([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_93([a, c, d, f, g, e, i], is_dual)
    } else {
        select_94([a, b, e, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_88([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_89([a, d, b, e, g, f, j, i, h], is_dual)
    } else {
        select_92([a, f, c, d, g, h, e, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_78([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_79([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_88([a, g, b, d, f, i, h, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_77([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_78([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_78([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_62([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_63([a, b, f, c, e, d, h, g, i, j, k], is_dual)
    } else {
        select_77([a, b, d, f, e, c, h, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_10([a, b, c, e, d, f, g, i, h, j, k, l], is_dual)
    } else {
        select_62([a, b, c, e, g, f, i, h, d, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_101([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_44([a, b, c, d, f, g], is_dual)
    } else {
        select_69([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_100([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_101([a, c, d, b, e, f, g], is_dual)
    } else {
        select_101([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_103([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_21([a, b, f, d, e], is_dual)
    } else {
        select_21([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_102([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_103([a, c, d, b, e, f], is_dual)
    } else {
        select_52([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_99([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_100([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_102([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_100([a, b, c, f, h, e, g], is_dual)
    } else {
        select_100([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_98([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_99([a, h, e, i, f, b, g, c], !is_dual)
    } else {
        select_104([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_108([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_83([b, a, c, d, e, f, g], is_dual)
    } else {
        select_69([a, e, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_109([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([a, e, c, f, d, h], is_dual)
    } else {
        select_39([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_107([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_108([h, i, j, g, a, e, b, c], !is_dual)
    } else {
        select_109([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_110([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_75([b, c, d, a, e, f, g], is_dual)
    } else {
        select_91([d, f, e, a, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_107([b, a, c, d, e, f, h, g, j, i], is_dual)
    } else {
        select_110([a, b, c, f, g, h, e, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_113([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_21([a, b, c, d, e], is_dual)
    } else {
        select_36([e, a, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_114([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_86([d, f, g, a, e, b], !is_dual)
    } else {
        select_91([e, f, d, b, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_112([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_113([b, c, d, e, f, h], is_dual)
    } else {
        select_114([d, g, i, a, e, f, b], !is_dual)
    }
}
/// n = 5, i = 2
fn select_116([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_91([b, c, a, d, e], is_dual)
    } else {
        select_37([a, c, b, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_116([a, e, f, g, i], is_dual)
    } else {
        select_109([a, b, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_111([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_112([a, f, b, d, h, e, i, j, g], is_dual)
    } else {
        select_115([a, e, c, d, g, f, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_106([b, a, c, d, e, g, h, f, j, i], is_dual)
    } else {
        select_111([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_97([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_98([a, b, g, c, h, f, i, j, e], is_dual)
    } else {
        select_105([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_96([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_97([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_97([a, b, d, e, f, c, g, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_16([b, c, d, f], is_dual)
    } else {
        select_91([a, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_122([a, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_122([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_124([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_91([d, f, e, a, b], !is_dual)
    } else {
        select_91([e, f, d, a, c], !is_dual)
    }
}
/// n = 5, i = 0
fn select_125([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_16([a, b, c, d], is_dual)
    } else {
        select_16([a, b, c, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_124([i, j, k, a, b, h], !is_dual)
    } else {
        select_125([c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_120([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_121([b, c, d, e, f, g, a, h, i, k], is_dual)
    } else {
        select_123([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_83([a, b, c, e, f, g, h], is_dual)
    } else {
        select_17([b, c, d], is_dual)
    }
}
/// n = 9, i = 2
fn select_127([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_128([a, b, c, d, f, g, h, i], is_dual)
    } else {
        select_128([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_116([a, b, h, i, j], is_dual)
    } else {
        select_125([c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_127([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_129([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_119([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_120([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_126([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_132([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_20([b, c, d, e, f], is_dual)
    } else {
        select_38([a, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_133([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_15([b, c, d, e, f], is_dual)
    } else {
        select_91([e, g, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_131([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_132([a, c, d, b, e, f, g], is_dual)
    } else {
        select_133([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_136([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_39([a, c, b, d, e, f], is_dual)
    } else {
        select_51([a, b, e, f, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_135([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_136([b, c, d, a, e, f], is_dual)
    } else {
        select_39([b, d, a, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_135([g, d, e, h, a, b], !is_dual)
    } else {
        select_73([d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_130([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_131([b, c, a, d, e, f, g], is_dual)
    } else {
        select_134([b, c, d, e, f, g, a, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_118([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_119([g, d, e, a, b, c, f, h, k, i, j], is_dual)
    } else {
        select_130([g, i, j, k, d, e, f, h], !is_dual)
    }
}
/// n = 11, i = 4
fn select_117([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_118([e, f, g, b, c, d, a, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_95([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_96([a, c, d, e, f, h, i, b, j, k], is_dual)
    } else {
        select_117([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_9([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_95([a, b, d, c, e, f, i, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_8([a, b, c, d, e, g, f, h, i, k, j, l], is_dual)
    } else {
        select_8([a, b, c, e, d, f, g, h, j, l, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_7([f, g, c, b, d, a, e, i, h, k, l, j], is_dual)
    } else {
        select_7([f, g, c, b, e, a, d, i, h, j, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_6([h, g, a, e, f, b, c, k, d, i, j, l], is_dual)
    } else {
        select_6([h, g, d, e, f, b, c, k, a, i, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_5([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_5([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_4([d, e, f, g, h, i, a, b, c, j, l, k], is_dual)
    } else {
        select_4([d, e, f, g, h, i, a, c, b, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_3([a, i, j, b, c, d, e, f, g, h, k, l], is_dual)
    } else {
        select_3([h, i, j, b, c, d, e, f, g, a, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_2([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
