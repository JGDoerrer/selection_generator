/// n = 1, i = 0
fn select_23([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_22([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([a], is_dual)
    } else {
        select_23([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_21([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_22([a, b], is_dual)
    } else {
        select_22([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([c, d, e], is_dual)
    } else {
        select_21([a, b, f], is_dual)
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
/// n = 4, i = 0
fn select_25([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_21([a, b, c], is_dual)
    } else {
        select_21([b, c, d], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_25([b, c, d, e], is_dual)
    } else {
        select_22([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_18([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([a, b, c, d, f, g, h], is_dual)
    } else {
        select_24([b, c, d, e, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_29([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_22([a, d], is_dual)
    } else {
        select_22([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_28([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_29([a, c, d, g], is_dual)
    } else {
        select_29([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_27([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_28([a, d, b, g, e, f, h], is_dual)
    } else {
        select_28([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_32([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_22([a, b], is_dual)
    } else {
        select_23([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_29([b, c, e, d], is_dual)
    } else {
        select_32([a, b, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_31([d, b, c, e, f, h], is_dual)
    } else {
        select_28([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_26([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_27([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_30([e, b, d, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_17([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_18([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_26([a, i, b, c, g, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_16([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_17([a, c, d, e, f, g, b, h, i, k, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_36([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_21([a, b, c], is_dual)
    } else {
        select_23([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([c, d, e, f], is_dual)
    } else {
        select_36([a, b, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_38([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, b, d], is_dual)
    } else {
        select_29([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_37([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_20([a, b, c, d, e, f], is_dual)
    } else {
        select_38([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_34([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_35([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_37([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_19([b, c, d, e, g, f, h], is_dual)
    } else {
        select_31([a, b, c, f, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_34([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_39([b, d, a, e, f, c, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_16([a, b, c, d, f, g, h, i, j, k, l], is_dual)
    } else {
        select_33([c, a, d, e, b, j, h, k, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_45([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_21([b, c, d], is_dual)
    } else {
        select_22([a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_46([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_29([a, b, c, d], is_dual)
    } else {
        select_32([d, a, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_44([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, g, f], is_dual)
    } else {
        select_46([a, b, e, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_43([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_24([c, d, e, i, f, h], is_dual)
    } else {
        select_44([c, h, a, b, f, g, j, i, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_21([b, c, d], is_dual)
    } else {
        select_32([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_48([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_49([a, c, d, b, e, f], is_dual)
    } else {
        select_49([b, c, d, a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_29([c, d, e, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([a, f, c, d, g, i], is_dual)
    } else {
        select_50([h, i, f, a, e, b], !is_dual)
    }
}
/// n = 12, i = 3
fn select_42([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_43([c, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_47([a, b, c, d, g, k, h, j, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_52([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_19([d, g, e, f, h, j, i], is_dual)
    } else {
        select_19([c, i, a, b, h, k, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_51([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_43([e, f, g, c, d, a, i, h, k, j, l], is_dual)
    } else {
        select_52([c, d, a, b, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_41([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_42([f, e, a, b, c, d, g, h, i, j, k, l], is_dual)
    } else {
        select_51([f, g, a, b, c, d, e, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_31([a, d, c, e, f, g], is_dual)
    } else {
        select_38([a, b, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_55([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_56([b, c, d, a, e, f, g], is_dual)
    } else {
        select_56([b, c, e, a, d, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_31([a, b, c, d, e, f], is_dual)
    } else {
        select_32([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_54([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_55([e, a, b, c, d, f, g], is_dual)
    } else {
        select_57([c, d, b, f, g, a], is_dual)
    }
}
/// n = 7, i = 1
fn select_59([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_19([a, c, d, e, b, f, g], is_dual)
    } else {
        select_19([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_61([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([a, d, b, c, e, f], is_dual)
    } else {
        select_31([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_61([f, a, b, c, e, g], is_dual)
    } else {
        select_61([e, a, b, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_59([b, c, d, a, e, f, g], is_dual)
    } else {
        select_60([b, c, d, g, f, a, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_53([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_54([a, b, c, d, e, h, i], is_dual)
    } else {
        select_58([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_40([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_41([d, e, f, g, a, b, c, j, i, l, m, k], is_dual)
    } else {
        select_53([b, c, d, e, h, a, k, m, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_15([a, b, c, f, j, g, h, l, i, k, m, n], is_dual)
    } else {
        select_40([b, a, c, d, e, g, h, i, k, j, m, l, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_65([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([e, a, b, c, h, f, i, g, j], is_dual)
    } else {
        select_39([e, a, b, d, g, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_66([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_34([a, b, c, d, h, f, i, g, j], is_dual)
    } else {
        select_34([a, b, c, e, g, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_64([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_65([d, a, e, f, b, c, g, h, i, j], is_dual)
    } else {
        select_66([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([c, d, e, f], is_dual)
    } else {
        select_21([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_71([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_25([d, e, f, g], is_dual)
    } else {
        select_25([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_70([a, h, d, e, f, g, i], is_dual)
    } else {
        select_71([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_68([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_69([d, e, i, a, b, c, g, h, j], is_dual)
    } else {
        select_69([d, f, h, a, b, c, g, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_67([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_68([e, f, g, a, c, d, b, h, i, j], is_dual)
    } else {
        select_68([e, f, g, b, c, d, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_63([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_64([a, b, c, k, d, e, i, j, h, l], is_dual)
    } else {
        select_67([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 5, i = 0
fn select_76([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_25([a, b, c, d], is_dual)
    } else {
        select_25([a, b, c, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_76([c, d, e, f, g], is_dual)
    } else {
        select_36([a, b, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_74([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_75([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_37([a, c, b, i, h, g, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_79([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_45([a, c, d, b, e], is_dual)
    } else {
        select_45([b, c, d, a, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_79([a, b, c, i, j], is_dual)
    } else {
        select_76([d, e, f, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_81([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_25([a, b, c, d], is_dual)
    } else {
        select_23([e], is_dual)
    }
}
/// n = 10, i = 3
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_81([a, e, f, h, j], is_dual)
    } else {
        select_79([b, c, d, g, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_77([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_78([b, c, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_80([a, b, c, e, d, k, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_73([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_74([d, i, e, f, g, h, a, j, k, m], is_dual)
    } else {
        select_77([d, b, c, e, a, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_20([c, d, e, f, h, i], is_dual)
    } else {
        select_20([a, b, e, f, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_84([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_85([a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_21([c, d, e], is_dual)
    }
}
/// n = 4, i = 1
fn select_87([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_38([a, b, c, d], is_dual)
    } else {
        select_38([a, b, d, c], is_dual)
    }
}
/// n = 8, i = 2
fn select_88([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([c, d, e, f], is_dual)
    } else {
        select_38([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_86([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_87([a, b, c, i], is_dual)
    } else {
        select_88([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_83([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_84([a, b, e, f, g, c, d, h, j, i, k], is_dual)
    } else {
        select_86([a, b, i, e, f, g, h, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([b, f, c, d, e, g, h], is_dual)
    } else {
        select_35([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_24([c, d, e, g, f, h], is_dual)
    } else {
        select_56([a, b, c, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_90([a, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_91([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_82([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_83([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_89([a, b, h, e, f, g, c, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_72([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_73([a, d, e, c, b, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_82([d, e, a, i, f, g, h, b, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_62([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_63([b, e, f, g, h, a, l, i, j, k, m, n], is_dual)
    } else {
        select_72([b, a, e, c, d, f, j, k, i, l, n, o, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_14([b, a, c, d, e, f, g, k, j, l, m, n, i, o], is_dual)
    } else {
        select_62([b, c, d, e, f, g, a, h, j, i, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_13([a, d, b, g, h, e, c, f, i, j, k, l, n, o, m], is_dual)
    } else {
        select_13([a, d, c, g, h, f, b, e, i, k, j, l, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_12([f, e, g, c, d, h, a, b, j, i, l, m, n, k, o], is_dual)
    } else {
        select_12([f, e, h, c, d, g, a, b, j, i, k, m, n, l, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_11([f, g, h, a, b, c, d, e, i, j, k, l, m, o, n], is_dual)
    } else {
        select_11([f, h, g, a, b, c, d, e, i, j, k, l, n, o, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_10([i, g, a, c, d, h, e, f, l, b, j, k, m, n, o], is_dual)
    } else {
        select_10([i, g, b, c, d, h, e, f, l, a, j, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_9([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    } else {
        select_9([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_8([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    } else {
        select_8([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_25([b, c, d, e], is_dual)
    } else {
        select_36([a, g, h, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_105([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_49([a, b, c, d, f, g], is_dual)
    } else {
        select_38([a, e, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_103([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_104([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_105([a, b, g, e, h, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_106([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_105([a, b, f, d, h, e, g], is_dual)
    } else {
        select_105([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_102([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_103([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_106([a, c, g, e, f, b, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_108([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_79([b, c, d, e, f], is_dual)
    } else {
        select_61([a, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_107([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_108([b, a, c, d, e, f, g], is_dual)
    } else {
        select_49([b, c, f, e, g, a], is_dual)
    }
}
/// n = 9, i = 3
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_102([a, b, d, c, e, g, f, h, i], is_dual)
    } else {
        select_107([g, a, c, b, e, h, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_113([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_29([a, b, c, d], is_dual)
    } else {
        select_32([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_112([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_113([d, c, a, e, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_112([a, i, b, j, g, k], is_dual)
    } else {
        select_76([c, d, e, f, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_115([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_20([a, b, c, d, e, f], is_dual)
    } else {
        select_45([e, c, d, a, f], is_dual)
    }
}
/// n = 11, i = 2
fn select_114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_115([a, b, c, j, h, k], is_dual)
    } else {
        select_76([d, e, f, g, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_111([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_114([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_118([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_25([c, d, e, g], is_dual)
    } else {
        select_45([a, b, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_117([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_118([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_118([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_120([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_46([b, c, a, d, e], is_dual)
    } else {
        select_29([a, c, b, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_119([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_76([c, d, e, f, g], is_dual)
    } else {
        select_120([a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_117([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_119([a, h, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_110([a, c, b, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_116([a, i, d, e, f, g, h, b, j, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_100([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_101([a, b, d, j, h, i, c, k, l], is_dual)
    } else {
        select_109([a, b, d, c, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_63([c, a, e, f, g, b, i, h, j, k, l, m], is_dual)
    } else {
        select_100([a, c, b, d, e, j, k, h, i, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_99([e, f, d, g, c, a, b, j, l, h, i, k, m, n], is_dual)
    } else {
        select_99([e, g, d, f, c, a, b, j, k, h, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_98([g, h, f, e, a, c, d, k, l, b, i, j, m, n], is_dual)
    } else {
        select_98([g, h, f, e, b, c, d, k, l, a, i, j, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_97([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_97([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_24([f, c, d, e, b, g], is_dual)
    } else {
        select_35([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_127([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_59([c, d, e, b, f, g, h], is_dual)
    } else {
        select_128([a, b, c, d, g, f, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_131([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_31([b, d, c, e, g, f], is_dual)
    } else {
        select_46([d, h, a, b, f], !is_dual)
    }
}
/// n = 6, i = 2
fn select_133([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_29([a, b, e, f], is_dual)
    } else {
        select_29([a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_132([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([a, e, c, f, d, h], is_dual)
    } else {
        select_133([b, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_130([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_131([b, a, c, e, d, f, h, g], is_dual)
    } else {
        select_132([b, c, a, e, d, g, h, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_135([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_38([a, b, c, e], is_dual)
    } else {
        select_38([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_136([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_25([b, c, d, e], is_dual)
    } else {
        select_32([a, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_134([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_135([i, a, b, f, j], is_dual)
    } else {
        select_136([b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_130([c, b, g, a, f, i, j, h], is_dual)
    } else {
        select_134([b, c, a, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_127([b, c, d, e, a, g, f, i, h, j], is_dual)
    } else {
        select_129([a, b, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_139([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_35([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_136([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_59([b, c, e, a, f, g, h], is_dual)
    } else {
        select_139([a, d, b, c, g, f, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_142([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_31([f, c, g, e, d, a], !is_dual)
    } else {
        select_46([d, g, c, e, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_144([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([b, c, e], is_dual)
    } else {
        select_29([a, f, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_143([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_144([a, b, c, d, e, f, g], is_dual)
    } else {
        select_144([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_141([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_142([a, b, i, f, e, j, h], is_dual)
    } else {
        select_143([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_146([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_144([a, c, d, e, g, f, i], is_dual)
    } else {
        select_38([a, h, b, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_145([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_146([f, a, c, d, e, g, h, j, i], is_dual)
    } else {
        select_146([e, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_140([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_141([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_145([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_137([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_138([b, c, d, e, a, g, f, i, h, j], is_dual)
    } else {
        select_140([b, a, e, c, d, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_126([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_137([a, b, d, e, c, f, h, i, g, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_149([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_90([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_90([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_151([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_71([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_35([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_150([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_90([a, h, d, e, f, g, b, i, j], is_dual)
    } else {
        select_151([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_149([a, b, c, d, f, h, g, j, i], is_dual)
    } else {
        select_150([a, d, e, b, c, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_155([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_46([e, f, a, b, c], !is_dual)
    } else {
        select_36([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_154([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_81([b, d, e, f, g], is_dual)
    } else {
        select_155([a, b, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_153([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_154([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_115([b, f, d, e, c, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_157([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_81([a, c, d, e, g], is_dual)
    } else {
        select_115([a, b, c, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_159([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_46([d, f, e, a, b], !is_dual)
    } else {
        select_46([e, f, d, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_112([b, c, d, f, e, g], is_dual)
    } else {
        select_159([g, f, h, a, b, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_157([b, c, d, e, f, g, h], is_dual)
    } else {
        select_158([a, b, h, c, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_152([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_153([a, c, d, b, g, f, i, h], is_dual)
    } else {
        select_156([a, b, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_147([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_148([a, d, e, c, b, f, g, h, j, i], is_dual)
    } else {
        select_152([a, b, d, e, c, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_124([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_125([b, c, i, d, e, g, h, a, j, k], is_dual)
    } else {
        select_147([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_124([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_124([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 1
fn select_165([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_71([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_71([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_164([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_165([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_69([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_168([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_144([b, c, d, e, g, f, h], is_dual)
    } else {
        select_38([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_167([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_168([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_168([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_166([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_167([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_167([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_164([c, d, a, i, e, f, g, h, j, k], is_dual)
    } else {
        select_166([a, h, b, c, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_162([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_163([a, b, c, d, e, f, g, j, i, h, k, l, m], is_dual)
    } else {
        select_163([b, a, c, d, e, f, g, j, h, i, k, l, m], is_dual)
    }
}
/// n = 9, i = 1
fn select_172([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([d, e, f, g, h], is_dual)
    } else {
        select_25([a, b, c, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_173([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, b, c, d, f, g], is_dual)
    } else {
        select_25([b, c, d, e], is_dual)
    }
}
/// n = 9, i = 1
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_172([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_173([h, d, e, f, g, a, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_170([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_171([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_154([a, b, j, c, d, h, i, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_176([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_113([a, e, d, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_178([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_31([a, c, b, e, f, g], is_dual)
    } else {
        select_36([a, c, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_177([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_115([b, c, d, e, f, g], is_dual)
    } else {
        select_178([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_176([g, i, f, b, a, h, c], !is_dual)
    } else {
        select_177([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_179([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_75([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_173([g, c, d, e, f, b, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_174([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_175([a, b, c, d, j, h, i, k, l], is_dual)
    } else {
        select_179([a, c, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_170([a, c, d, i, e, f, g, h, j, k, l], is_dual)
    } else {
        select_174([a, b, c, j, e, f, g, h, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_161([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_162([b, c, d, e, f, g, h, i, j, a, k, l, m], is_dual)
    } else {
        select_169([b, c, d, a, f, g, h, i, j, e, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_161([a, b, d, e, f, g, h, i, c, j, k, l, m], is_dual)
    } else {
        select_161([a, c, d, e, f, g, h, i, b, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_122([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < k) || (is_dual && e > k) {
        select_123([a, b, c, d, f, g, k, j, e, l, m], is_dual)
    } else {
        select_160([a, b, c, d, f, g, e, h, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_186([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([e, a, c, f, d, g], is_dual)
    } else {
        select_31([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_186([b, c, d, a, e, f, h], is_dual)
    } else {
        select_186([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([a, e, c, f, d, h], is_dual)
    } else {
        select_31([b, d, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_188([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_188([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_184([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_185([a, b, f, c, h, e, g, i], is_dual)
    } else {
        select_187([b, e, a, d, g, f, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_191([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_31([a, c, d, f, e, h], is_dual)
    } else {
        select_31([b, d, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_142([e, h, a, g, f, b, c], !is_dual)
    } else {
        select_191([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_189([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_190([a, b, h, c, i, f, g, j], is_dual)
    } else {
        select_128([b, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_183([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_184([b, a, h, c, j, f, k, g, i], is_dual)
    } else {
        select_189([b, a, f, d, e, g, i, h, k, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_193([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_108([a, c, d, e, b, f, g], is_dual)
    } else {
        select_108([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_195([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_115([f, h, b, c, j, g], is_dual)
    } else {
        select_19([a, g, d, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_194([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_195([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_43([c, d, b, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_193([a, i, c, d, b, k, g], is_dual)
    } else {
        select_194([a, b, e, f, c, d, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_182([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_183([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_192([b, c, a, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_181([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_182([a, b, d, c, e, f, h, i, g, j, k], is_dual)
    } else {
        select_182([a, c, d, b, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_181([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_181([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_121([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < d) || (is_dual && l > d) {
        select_122([a, b, c, e, f, d, g, h, i, j, k, l, m], is_dual)
    } else {
        select_180([a, b, c, e, f, g, k, j, l, d, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_95([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_96([a, b, c, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_121([a, b, c, d, e, f, g, h, l, j, m, k, n], is_dual)
    }
}
/// n = 7, i = 1
fn select_204([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_115([d, a, b, g, e, f], is_dual)
    } else {
        select_115([d, a, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_204([a, c, d, b, e, f, g], is_dual)
    } else {
        select_79([a, f, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_202([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_203([b, c, i, d, g, a, h, j], is_dual)
    } else {
        select_150([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_208([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_20([b, c, d, e, g, f], is_dual)
    } else {
        select_36([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_207([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_70([b, c, d, e, i, g, h], is_dual)
    } else {
        select_208([f, a, h, b, c, g, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_165([a, b, h, c, d, e, f, j, k, i], is_dual)
    } else {
        select_207([i, c, d, e, f, g, j, k, h, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_20([b, c, d, e, g, h], is_dual)
    } else {
        select_45([a, d, e, f, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_70([b, c, d, e, j, h, i], is_dual)
    } else {
        select_211([f, b, c, a, i, g, h, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_212([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_88([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_88([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_209([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_210([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    } else {
        select_212([b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_205([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_206([a, g, b, c, d, e, f, i, h, j, k, l], is_dual)
    } else {
        select_209([h, b, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_202([a, b, c, d, j, k, h, i, m, l], is_dual)
    } else {
        select_205([e, c, i, f, g, b, a, j, h, l, k, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_201([g, f, e, d, a, b, j, l, k, h, i, m, n], is_dual)
    } else {
        select_201([g, f, e, d, a, c, i, l, k, h, j, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_217([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_49([a, c, d, e, f, h], is_dual)
    } else {
        select_36([b, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_216([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_217([a, b, c, h, f, g, i, j], is_dual)
    } else {
        select_136([a, d, e, g, f, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_215([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_216([a, b, i, e, f, g, h, j, k, l], is_dual)
    } else {
        select_151([a, e, f, c, d, h, g, j, i, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_220([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_31([a, c, b, f, g, h], is_dual)
    } else {
        select_38([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_219([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_220([a, b, c, h, g, f, i, j], is_dual)
    } else {
        select_35([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_218([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_219([f, e, i, c, d, g, h, j, k, l], is_dual)
    } else {
        select_151([e, c, d, a, b, h, g, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_214([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_215([a, g, b, c, d, e, h, f, i, j, l, k], is_dual)
    } else {
        select_218([b, c, d, e, a, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_213([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_214([c, d, i, e, f, b, a, h, k, j, l, m], is_dual)
    } else {
        select_202([a, c, d, g, b, j, h, i, l, k], is_dual)
    }
}
/// n = 15, i = 3
fn select_199([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_200([b, f, g, e, d, c, a, h, k, l, j, i, m, n], is_dual)
    } else {
        select_213([a, b, c, d, k, l, e, i, j, n, m, h, o], is_dual)
    }
}
/// n = 7, i = 1
fn select_226([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_81([e, c, d, f, g], is_dual)
    } else {
        select_81([e, a, b, g, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_225([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_226([b, g, d, e, a, f, h], is_dual)
    } else {
        select_226([c, f, d, e, a, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_229([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_133([a, b, c, e, f, g], is_dual)
    } else {
        select_133([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_228([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([b, c, d, e, f, h, g], is_dual)
    } else {
        select_229([a, g, b, c, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_227([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_226([c, h, d, e, f, g, i], is_dual)
    } else {
        select_228([a, b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_224([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_225([f, c, g, d, e, h, a, i], is_dual)
    } else {
        select_227([b, a, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_233([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_133([d, f, g, a, e, b], !is_dual)
    } else {
        select_46([e, f, d, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_232([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_233([g, h, e, a, f, c, b], !is_dual)
    } else {
        select_233([g, h, f, a, e, d, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_231([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_139([a, c, d, e, f, g, h, j], is_dual)
    } else {
        select_232([a, b, c, h, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_234([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_151([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_151([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_230([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_231([f, a, i, d, e, g, h, j, l, k], is_dual)
    } else {
        select_234([f, g, d, e, b, c, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_223([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_224([b, c, d, a, j, g, h, i, l, k], is_dual)
    } else {
        select_230([c, b, i, e, f, a, g, h, k, j, m, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_237([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_81([a, d, e, h, g], is_dual)
    } else {
        select_178([a, b, c, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_236([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_226([c, h, d, e, a, g, i], is_dual)
    } else {
        select_237([a, b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_239([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_24([g, d, e, f, a, h], is_dual)
    } else {
        select_71([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_240([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_118([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_30([g, c, b, a, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_238([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_239([f, b, c, d, e, j, h, i], is_dual)
    } else {
        select_240([f, a, i, b, c, g, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_236([b, c, f, a, j, g, h, i, l, k], is_dual)
    } else {
        select_238([c, b, h, d, e, a, g, k, j, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_222([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_223([b, d, c, e, f, g, a, h, i, j, k, l, m], is_dual)
    } else {
        select_235([b, d, a, f, g, e, c, i, h, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_221([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_200([b, f, g, e, d, c, a, h, k, l, j, i, m, n], is_dual)
    } else {
        select_222([a, b, c, d, e, k, l, i, j, n, m, h, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_198([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_199([a, c, b, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_221([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_59([b, f, e, d, g, h, i], is_dual)
    } else {
        select_90([a, c, b, d, h, f, g, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_248([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_19([a, d, b, c, e, f, g], is_dual)
    } else {
        select_38([a, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_250([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_31([a, c, b, f, g, e], is_dual)
    } else {
        select_31([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_249([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_115([a, c, d, e, g, f], is_dual)
    } else {
        select_250([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_247([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_248([a, b, g, c, e, f, i, j], is_dual)
    } else {
        select_249([b, a, f, d, e, g, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_245([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_246([a, c, e, d, b, f, h, g, j, i, k], is_dual)
    } else {
        select_247([a, b, e, c, f, h, i, j, g, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_253([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_115([e, b, c, d, f, g], is_dual)
    } else {
        select_49([a, c, d, e, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_255([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_45([b, a, c, d, e], is_dual)
    } else {
        select_29([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 1
fn select_254([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_115([a, c, b, d, e, f], is_dual)
    } else {
        select_255([a, c, f, e, b], is_dual)
    }
}
/// n = 10, i = 3
fn select_252([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_253([a, c, b, g, e, f, i, j], is_dual)
    } else {
        select_254([b, e, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_251([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_246([a, c, e, d, b, f, h, g, j, i, k], is_dual)
    } else {
        select_252([a, b, e, c, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_244([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_245([c, b, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_251([c, a, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_258([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_59([b, g, c, f, i, h, k], is_dual)
    } else {
        select_151([a, d, e, b, f, h, g, i, j, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_260([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_70([b, c, d, e, f, g, h], is_dual)
    } else {
        select_37([a, h, b, c, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_259([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_260([a, b, g, d, e, f, i, h, k], is_dual)
    } else {
        select_249([b, a, h, c, f, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_257([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_258([a, c, b, e, f, d, g, h, j, i, k, l], is_dual)
    } else {
        select_259([a, b, c, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_264([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_31([a, b, c, f, e, g], is_dual)
    } else {
        select_31([a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_263([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_81([a, d, e, h, g], is_dual)
    } else {
        select_264([a, g, b, c, i, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_265([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_220([a, b, h, c, i, g, f, j], is_dual)
    } else {
        select_136([b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_262([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_263([b, d, e, c, f, h, g, j, i], is_dual)
    } else {
        select_265([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_258([a, c, b, e, f, d, g, h, j, i, k, l], is_dual)
    } else {
        select_262([a, b, c, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_257([c, b, d, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_261([c, a, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_244([a, b, d, c, j, e, h, i, l, m, k], is_dual)
    } else {
        select_256([a, c, d, b, i, f, g, h, k, j, m, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_268([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([b, e, c, d, f, g], is_dual)
    } else {
        select_255([g, e, h, a, b], !is_dual)
    }
}
/// n = 11, i = 3
fn select_267([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_64([c, b, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_268([b, c, g, h, i, j, a, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_266([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_213([a, c, d, b, f, g, e, h, i, j, k, l, m], is_dual)
    } else {
        select_267([a, c, b, j, d, e, h, i, l, m, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_242([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_243([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_266([a, c, d, b, f, g, h, e, i, j, k, l, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_275([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_45([a, b, c, d, e], is_dual)
    } else {
        select_32([e, a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_70([b, c, a, d, e, f, g], is_dual)
    } else {
        select_275([g, b, c, a, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_273([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_274([f, a, h, d, e, g, i, j], is_dual)
    } else {
        select_239([f, d, e, b, c, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_277([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_186([b, c, d, a, e, f, h], is_dual)
    } else {
        select_186([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_276([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_59([b, c, d, a, e, f, g], is_dual)
    } else {
        select_277([b, c, g, d, a, f, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_272([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_273([d, a, h, e, f, b, g, j, i, k], is_dual)
    } else {
        select_276([g, a, b, c, i, h, k, j, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_272([a, b, c, d, e, j, g, h, i, k, l, m], is_dual)
    } else {
        select_272([a, b, c, d, f, i, g, h, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_281([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_115([a, b, e, f, g, i], is_dual)
    } else {
        select_19([c, d, a, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_280([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_39([a, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_281([a, b, h, d, c, g, f, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_282([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_18([b, c, a, f, i, g, h, k], is_dual)
    } else {
        select_239([a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_279([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_280([b, g, f, d, e, a, h, i, j, k], is_dual)
    } else {
        select_282([b, d, e, a, c, f, h, i, g, k, j], is_dual)
    }
}
/// n = 9, i = 1
fn select_283([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_18([b, h, d, e, f, g, a, i], is_dual)
    } else {
        select_69([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 2
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_279([a, b, c, d, e, k, h, i, j, l, m], is_dual)
    } else {
        select_283([a, d, e, f, g, l, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_271([f, e, a, k, b, c, g, h, i, j, l, m, n], is_dual)
    } else {
        select_278([e, f, a, b, c, d, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_285([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_16([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_16([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_289([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_35([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_290([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([f, a, c, d, e, g, h], is_dual)
    } else {
        select_37([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_288([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_289([c, d, a, b, h, f, g, i, j], is_dual)
    } else {
        select_290([c, d, e, i, f, g, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_293([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_20([a, b, c, d, e, f], is_dual)
    } else {
        select_36([e, c, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_292([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_70([b, c, d, e, f, g, h], is_dual)
    } else {
        select_293([a, h, b, c, f, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_291([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_292([a, b, i, e, f, g, h, j, k], is_dual)
    } else {
        select_69([a, e, f, c, d, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_287([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_288([e, f, b, c, i, g, h, a, j, k], is_dual)
    } else {
        select_291([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_296([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_19([a, b, d, e, f, g, h], is_dual)
    } else {
        select_20([b, c, d, e, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_295([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_165([a, b, g, c, d, e, f, i, j, h], is_dual)
    } else {
        select_296([h, c, d, e, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_297([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_292([h, a, b, e, f, g, j, i, k], is_dual)
    } else {
        select_165([c, d, g, a, b, e, f, j, i, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_294([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_295([b, i, c, d, e, f, g, h, j, k], is_dual)
    } else {
        select_297([a, h, c, d, e, f, g, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 2
fn select_286([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_287([a, c, j, d, f, g, h, b, i, k, l], is_dual)
    } else {
        select_294([c, d, b, e, f, g, a, i, h, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_284([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_285([a, h, b, c, d, j, k, i, m, l, n], is_dual)
    } else {
        select_286([a, c, e, f, g, b, i, h, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_269([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_270([f, a, g, h, d, c, b, i, e, j, k, l, m, n], is_dual)
    } else {
        select_284([b, c, d, f, e, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_241([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_242([a, b, d, c, e, g, f, k, j, m, l, i, n], is_dual)
    } else {
        select_269([a, b, d, e, f, g, c, h, j, i, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_198([a, b, c, e, f, h, d, g, i, l, j, k, n, m, o], is_dual)
    } else {
        select_241([a, b, c, d, e, f, k, h, i, m, l, j, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_196([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_197([a, b, c, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_197([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_94([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_95([a, b, c, d, e, f, h, g, l, j, k, m, o, n], is_dual)
    } else {
        select_196([a, b, c, d, e, g, f, i, h, j, l, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_94([a, b, c, d, e, g, h, i, j, f, k, l, m, n, o], is_dual)
    } else {
        select_94([a, b, c, d, f, g, h, i, j, e, k, l, m, n, o], is_dual)
    }
}
/// n = 14, i = 3
fn select_300([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_96([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    } else {
        select_96([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_309([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_118([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_88([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_308([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_90([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_309([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_312([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_113([a, c, b, d, e], is_dual)
    } else {
        select_113([b, c, a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_311([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_48([a, b, c, d, e, f], is_dual)
    } else {
        select_312([a, b, e, c, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_310([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_311([b, e, c, a, g, f], is_dual)
    } else {
        select_277([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_307([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_308([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_310([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_315([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_208([h, e, f, a, b, g, i, j], is_dual)
    } else {
        select_208([i, c, d, a, b, g, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_165([d, e, h, a, b, f, g, k, j, i], is_dual)
    } else {
        select_315([f, g, a, b, c, i, j, k, h, l], is_dual)
    }
}
/// n = 9, i = 1
fn select_317([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([c, d, e, f, g, h, i], is_dual)
    } else {
        select_70([c, d, a, b, i, h, g], is_dual)
    }
}
/// n = 11, i = 1
fn select_316([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_165([b, c, h, d, e, f, g, j, k, i], is_dual)
    } else {
        select_317([a, i, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_314([a, b, g, c, d, e, f, h, i, j, k, l], is_dual)
    } else {
        select_316([h, a, b, c, d, e, f, g, k, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_306([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_307([a, j, b, c, d, k, i, m, n, l], is_dual)
    } else {
        select_313([e, f, b, i, g, h, d, a, l, k, j, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_305([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_306([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_306([a, c, b, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_304([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_305([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_305([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_303([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_304([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_304([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_324([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_48([a, b, c, f, e, g], is_dual)
    } else {
        select_48([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_325([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_115([b, c, a, d, e, f], is_dual)
    } else {
        select_159([b, c, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_323([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_324([a, e, b, d, g, f, h], is_dual)
    } else {
        select_325([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_322([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_323([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_323([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_328([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_19([b, f, c, d, e, g, h], is_dual)
    } else {
        select_255([e, a, h, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_330([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_144([a, b, c, d, e, f, g], is_dual)
    } else {
        select_36([e, b, c, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_331([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_144([a, b, c, e, f, g, h], is_dual)
    } else {
        select_21([b, c, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_329([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_330([a, b, h, f, g, e, i, j], is_dual)
    } else {
        select_331([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_327([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_328([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_329([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_326([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_193([b, g, c, d, a, i, h], is_dual)
    } else {
        select_327([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_321([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_322([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_326([c, b, a, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_320([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_321([a, b, d, c, e, g, h, i, f, j], is_dual)
    } else {
        select_321([a, c, d, b, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_319([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_320([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_320([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_318([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_319([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_319([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_302([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < f) || (is_dual && m > f) {
        select_303([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_318([a, b, c, d, e, k, l, m, f, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_338([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_268([a, b, c, i, g, j, k, h], is_dual)
    } else {
        select_151([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_340([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_48([a, e, d, g, f, b], !is_dual)
    } else {
        select_112([a, c, b, e, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_339([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_325([b, c, d, a, f, e, g], is_dual)
    } else {
        select_340([f, e, h, a, b, g, c], !is_dual)
    }
}
/// n = 12, i = 3
fn select_337([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_338([a, b, h, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_339([a, b, c, k, g, l, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_336([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_337([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_337([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_335([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_336([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_336([a, b, d, c, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_334([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_335([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_335([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_333([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_334([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_334([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_347([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([a, f, c, d, g, h], is_dual)
    } else {
        select_312([a, h, b, e, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_346([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_59([b, g, a, d, e, i, h], is_dual)
    } else {
        select_347([a, c, b, h, f, g, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_350([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_31([a, c, b, d, e, f], is_dual)
    } else {
        select_49([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_349([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, d, c, e, f, h], is_dual)
    } else {
        select_350([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_351([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_105([f, g, h, a, d, e, c], !is_dual)
    } else {
        select_31([a, b, e, g, f, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_349([a, d, c, f, g, e, i, h], is_dual)
    } else {
        select_351([a, e, b, h, d, i, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_345([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_346([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_348([b, h, c, j, f, a, k, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_354([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_115([e, a, c, d, f, g], is_dual)
    } else {
        select_112([a, b, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_356([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_25([d, e, f, i], is_dual)
    } else {
        select_20([a, b, c, h, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_355([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_356([b, c, a, d, e, f, g, i, h, j], is_dual)
    } else {
        select_24([h, d, e, f, a, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_353([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_354([b, g, e, f, a, j, h, i, k], is_dual)
    } else {
        select_355([b, e, f, a, c, d, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_352([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_193([a, b, c, d, i, k, g], is_dual)
    } else {
        select_353([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_344([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_345([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_352([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_343([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_344([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_344([a, b, d, c, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_342([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_343([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_343([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_341([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_342([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_342([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_332([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_333([a, b, c, d, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_341([a, b, c, d, e, f, g, k, j, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_301([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_302([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_332([a, b, c, d, e, f, k, i, j, g, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_299([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_300([a, b, c, d, e, f, k, h, g, j, m, l, n, o], is_dual)
    } else {
        select_301([a, b, c, d, e, f, g, i, h, l, k, n, m, o], is_dual)
    }
}
/// n = 13, i = 3
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_121([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    } else {
        select_121([a, b, c, d, f, g, h, i, j, e, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_357([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_300([a, b, c, d, e, f, g, i, h, j, k, l, m, n], is_dual)
    } else {
        select_358([a, b, c, d, e, f, g, h, i, l, m, k, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_298([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_299([a, b, c, d, e, f, i, g, j, h, l, k, m, n, o], is_dual)
    } else {
        select_357([a, b, c, d, e, f, g, h, l, i, k, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_92([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_93([a, b, c, d, e, f, h, g, i, j, l, k, n, o, m], is_dual)
    } else {
        select_298([a, b, c, d, e, f, h, i, g, j, l, k, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_7([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_92([a, b, c, d, e, f, h, i, k, j, l, m, n, g, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_6([a, b, c, d, e, f, g, j, h, k, i, n, m, o, l], is_dual)
    } else {
        select_6([a, b, c, d, e, f, g, j, i, k, h, n, l, o, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_5([a, b, c, d, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    } else {
        select_5([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_4([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_373([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_59([a, c, d, b, e, f, g], is_dual)
    } else {
        select_325([a, c, g, d, b, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_374([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_292([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_296([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_372([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_373([a, h, c, b, f, i, g, j], is_dual)
    } else {
        select_374([a, b, d, e, c, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_375([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_276([h, a, b, c, d, j, g, i, k], is_dual)
    } else {
        select_374([b, c, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_371([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_372([b, d, c, e, f, a, g, i, j, h], is_dual)
    } else {
        select_375([a, b, d, c, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_380([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_45([b, c, d, e, f], is_dual)
    } else {
        select_36([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_379([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_19([b, c, d, e, g, f, h], is_dual)
    } else {
        select_380([a, b, d, e, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_378([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_69([a, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_379([b, j, a, c, d, g, h, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_17([a, i, c, e, f, g, b, h, j, k, l], is_dual)
    } else {
        select_378([c, a, b, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_376([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_377([a, b, d, e, c, j, g, h, i, k, l, m], is_dual)
    } else {
        select_202([b, a, c, f, h, i, g, j, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_370([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_371([a, b, d, c, i, j, g, h, l, m, k], is_dual)
    } else {
        select_376([a, c, b, e, f, d, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_369([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_370([d, e, c, f, a, b, i, k, g, h, j, l, m], is_dual)
    } else {
        select_370([d, f, c, e, a, b, i, j, g, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_368([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_369([f, g, e, a, c, d, j, k, b, h, i, l, m], is_dual)
    } else {
        select_369([f, g, e, b, c, d, j, k, a, h, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_367([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_368([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_368([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_366([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_367([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_367([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_389([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_81([b, c, d, e, f], is_dual)
    } else {
        select_46([e, g, a, b, f], !is_dual)
    }
}
/// n = 8, i = 2
fn select_388([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_154([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_389([f, b, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_390([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_69([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_69([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_387([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_388([f, e, h, c, d, g, i, j], is_dual)
    } else {
        select_390([e, f, c, d, a, b, g, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_386([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_307([a, c, b, d, e, i, h, k, l, j], is_dual)
    } else {
        select_387([b, h, f, g, e, a, c, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_386([a, b, d, c, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_386([a, c, d, b, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_384([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_385([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    } else {
        select_385([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_383([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_384([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    } else {
        select_384([a, b, c, e, f, g, h, i, d, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_382([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_318([a, b, c, d, e, g, j, k, f, l], is_dual)
    } else {
        select_383([a, b, c, d, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_381([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_382([a, b, c, d, e, f, g, h, k, j, l, m], is_dual)
    } else {
        select_382([a, b, c, d, e, f, g, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_365([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_366([a, b, c, d, e, f, g, h, k, j, m, l, n], is_dual)
    } else {
        select_381([a, b, c, d, e, f, g, i, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_401([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([a, b, c, d, e, f], is_dual)
    } else {
        select_49([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_400([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_104([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_401([a, b, g, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_399([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_103([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_400([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_403([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_19([d, e, b, c, f, g, h], is_dual)
    } else {
        select_178([a, e, b, c, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_18([c, d, e, f, i, g, h, j], is_dual)
    } else {
        select_403([a, b, j, c, d, g, h, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_398([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_399([b, a, c, g, h, j, k, l, i], is_dual)
    } else {
        select_402([b, d, e, f, a, c, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_405([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_172([c, d, i, e, f, g, h, j, k], is_dual)
    } else {
        select_78([a, b, j, e, f, g, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_404([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_405([a, b, c, j, e, f, g, h, k, i, l, m], is_dual)
    } else {
        select_405([a, b, d, i, e, f, g, h, k, j, l, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_397([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_398([b, a, c, m, d, e, j, k, l, i, n, o], is_dual)
    } else {
        select_404([b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_396([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_397([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    } else {
        select_397([b, c, d, e, f, g, h, i, a, j, k, l, m, n, o], is_dual)
    }
}
/// n = 6, i = 2
fn select_412([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_29([a, d, c, f], is_dual)
    } else {
        select_29([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_411([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_412([a, b, c, e, d, f], is_dual)
    } else {
        select_412([a, b, d, e, c, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_413([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_133([b, c, d, e, f, g], is_dual)
    } else {
        select_412([a, c, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_410([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_411([d, c, a, f, e, h], is_dual)
    } else {
        select_413([c, a, b, e, d, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_409([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_410([b, c, d, a, f, e, g, h], is_dual)
    } else {
        select_349([b, a, d, e, c, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_414([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_34([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_207([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_408([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_409([b, a, c, i, h, j, g, k], is_dual)
    } else {
        select_414([c, b, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_416([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_154([f, e, h, c, d, g, i, j], is_dual)
    } else {
        select_239([e, c, d, a, b, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_415([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_399([a, b, c, d, i, g, j, k, h], is_dual)
    } else {
        select_416([b, c, e, f, d, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_407([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_408([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_415([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_406([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_407([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_407([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 15, i = 3
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < j) || (is_dual && n > j) {
        select_396([a, b, d, e, c, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_406([a, b, d, c, e, k, m, l, n, j, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_394([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_395([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    } else {
        select_395([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 3
fn select_422([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_39([c, d, e, a, b, f, h, g, i], is_dual)
    } else {
        select_187([a, b, c, f, h, i, g, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_424([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_188([a, b, c, e, f, g, h, i], is_dual)
    } else {
        select_264([b, d, c, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_426([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([a, b, d, g, f, i], is_dual)
    } else {
        select_29([c, d, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_425([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([e, b, c, d, g, h, f, i], is_dual)
    } else {
        select_426([a, f, d, b, g, e, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_423([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_424([a, b, c, d, f, h, j, i, k], is_dual)
    } else {
        select_425([a, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_421([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_422([a, g, b, d, e, h, f, i, j, k], is_dual)
    } else {
        select_423([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_428([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_114([b, a, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_114([a, b, c, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_430([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_81([a, b, c, i, j], is_dual)
    } else {
        select_76([d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_429([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_172([c, d, i, e, f, g, h, j, k], is_dual)
    } else {
        select_430([a, b, j, e, f, g, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_427([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_428([a, b, j, c, e, f, g, h, i, k, l, m], is_dual)
    } else {
        select_429([b, c, d, i, e, f, g, h, k, j, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_420([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < l) || (is_dual && a > l) {
        select_421([b, c, l, d, e, i, a, j, k, m, n], is_dual)
    } else {
        select_427([b, c, d, e, a, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_433([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_59([a, c, b, d, e, f, g], is_dual)
    } else {
        select_79([a, b, c, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_432([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_164([a, d, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_433([a, b, c, i, k, h, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_436([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_19([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([a, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_435([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_436([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_436([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_434([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_164([a, d, b, h, e, f, g, i, j, k], is_dual)
    } else {
        select_435([a, b, c, i, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_432([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_434([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_419([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_420([a, c, b, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    } else {
        select_431([a, c, j, e, f, g, h, i, b, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_418([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_419([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_419([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_440([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_112([a, d, b, e, c, f], is_dual)
    } else {
        select_112([a, d, c, e, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_441([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_79([a, b, c, d, e], is_dual)
    } else {
        select_38([a, b, e, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_439([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_440([a, c, d, e, b, f], is_dual)
    } else {
        select_441([a, c, d, b, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_442([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_48([a, b, c, d, e, f], is_dual)
    } else {
        select_36([b, f, g, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_438([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_439([a, b, c, f, g, e], !is_dual)
    } else {
        select_442([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_445([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_48([a, e, c, d, f, g], is_dual)
    } else {
        select_255([a, e, h, b, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_446([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_350([b, g, c, a, f, h, i], is_dual)
    } else {
        select_35([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_444([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_445([a, f, d, e, b, g, h, i], is_dual)
    } else {
        select_446([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_448([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_48([a, e, c, d, f, g], is_dual)
    } else {
        select_112([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_447([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_226([a, c, d, e, b, f, g], is_dual)
    } else {
        select_448([b, f, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_443([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_444([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_447([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_437([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_438([a, b, c, g, h, d, i], is_dual)
    } else {
        select_443([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 15, i = 3
fn select_417([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < j) || (is_dual && n > j) {
        select_418([a, b, c, d, j, f, g, h, i, l, k, m, n, o], is_dual)
    } else {
        select_437([a, b, c, d, e, n, j, l, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_393([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_394([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    } else {
        select_417([a, b, c, d, e, g, h, i, j, k, l, f, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_392([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_393([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    } else {
        select_393([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 1
fn select_455([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_164([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_164([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_456([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_414([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_414([a, b, e, d, f, g, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_454([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_455([e, f, h, a, b, c, d, i, j, k], is_dual)
    } else {
        select_456([h, g, e, f, k, a, b, i, j, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_453([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_454([b, j, g, h, e, f, a, c, i, l, k, m], is_dual)
    } else {
        select_100([a, c, b, d, e, f, k, i, j, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_452([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_453([f, g, e, h, c, d, a, b, i, k, l, j, m, n], is_dual)
    } else {
        select_453([f, h, e, g, c, d, a, b, i, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_451([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_452([h, i, f, g, e, a, c, d, b, j, k, l, m, n], is_dual)
    } else {
        select_452([h, i, f, g, e, b, c, d, a, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_450([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_451([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    } else {
        select_451([a, b, d, e, f, g, h, i, j, c, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_449([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_450([a, b, c, d, e, g, h, i, j, k, f, l, m, n], is_dual)
    } else {
        select_450([a, b, c, d, f, g, h, i, j, k, e, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_391([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_392([a, b, c, d, e, f, g, j, h, i, k, l, m, n, o], is_dual)
    } else {
        select_449([a, b, c, d, e, f, g, h, i, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_364([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < m) || (is_dual && g > m) {
        select_365([a, b, c, d, e, f, h, i, k, m, l, g, n, o], is_dual)
    } else {
        select_391([a, b, c, d, e, f, h, g, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_465([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_331([a, b, c, d, f, g, h, i], is_dual)
    } else {
        select_331([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_464([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_465([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_119([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_463([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_464([a, b, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_339([a, b, c, i, g, j, h, k], is_dual)
    }
}
/// n = 9, i = 1
fn select_467([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_226([a, h, d, e, f, g, i], is_dual)
    } else {
        select_239([f, d, e, b, c, g, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_468([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_157([b, d, e, f, h, g, i], is_dual)
    } else {
        select_177([a, b, d, c, i, g, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_466([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_467([c, d, g, e, f, b, h, i, j], is_dual)
    } else {
        select_468([a, b, i, c, e, f, h, g, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_462([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_463([a, b, c, d, i, j, g, h, l, m, k], is_dual)
    } else {
        select_466([a, d, e, f, b, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_461([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_462([d, e, f, c, a, b, i, k, g, h, j, l, m], is_dual)
    } else {
        select_462([d, f, e, c, a, b, i, j, g, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_460([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_461([f, g, e, a, c, d, j, k, b, h, i, l, m], is_dual)
    } else {
        select_461([f, g, e, b, c, d, j, k, a, h, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_459([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_460([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_460([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_474([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_311([a, e, g, d, b, f], !is_dual)
    } else {
        select_340([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_473([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_474([a, b, d, c, e, f, g], is_dual)
    } else {
        select_474([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_472([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_473([a, b, c, h, g, f, d], !is_dual)
    } else {
        select_473([a, b, c, h, f, g, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_478([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_105([a, b, c, d, f, e, g], is_dual)
    } else {
        select_49([d, f, g, a, b, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_477([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_478([a, c, d, b, e, f, g], is_dual)
    } else {
        select_478([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_480([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_48([a, d, f, g, e, c], !is_dual)
    } else {
        select_79([a, f, d, g, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_481([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_115([a, d, b, c, e, f], is_dual)
    } else {
        select_79([a, b, c, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_479([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_480([a, e, d, b, f, g, h], is_dual)
    } else {
        select_481([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_476([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_477([a, b, c, f, g, h, e], is_dual)
    } else {
        select_479([a, b, c, d, e, f, h, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_484([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_105([a, c, d, b, e, f, g], is_dual)
    } else {
        select_105([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_483([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_325([a, c, d, b, f, e, g], is_dual)
    } else {
        select_484([a, f, e, h, b, g, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_482([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_483([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_107([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_475([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_476([a, b, d, e, f, g, c, h], is_dual)
    } else {
        select_482([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_471([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_472([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_475([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_488([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_311([a, b, c, d, f, g], is_dual)
    } else {
        select_48([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_487([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_373([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_488([a, b, c, d, g, h, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_491([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_135([a, b, d, c, e], is_dual)
    } else {
        select_135([a, c, d, b, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_490([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_491([a, b, c, e, f], is_dual)
    } else {
        select_312([a, b, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_493([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_48([a, b, c, d, f, g], is_dual)
    } else {
        select_81([b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_492([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_440([a, b, g, h, c, f], !is_dual)
    } else {
        select_493([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_490([a, b, h, i, f, c], !is_dual)
    } else {
        select_492([a, b, f, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_486([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_487([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_489([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_485([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_486([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_475([a, b, f, h, d, c, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_470([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_471([a, b, c, g, d, h, i, f], is_dual)
    } else {
        select_485([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_469([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_470([a, b, c, e, i, h, j, k, d], is_dual)
    } else {
        select_180([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_458([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_459([a, b, c, d, f, e, g, h, i, j, k, l, m], is_dual)
    } else {
        select_469([a, b, c, d, f, j, k, i, l, e, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_457([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_458([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_458([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_363([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < h) || (is_dual && o > h) {
        select_364([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_457([a, b, c, d, e, f, g, i, l, m, n, o, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_506([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, d, e, f, g], is_dual)
    } else {
        select_38([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_505([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_211([a, d, e, b, c, f, h, g, i], is_dual)
    } else {
        select_506([a, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_504([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_103([a, g, d, e, f, b, h, j, i], is_dual)
    } else {
        select_505([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_503([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_504([a, b, c, e, h, i, g, k, l, j], is_dual)
    } else {
        select_414([b, a, d, f, e, g, h, i, j, k], is_dual)
    }
}
/// n = 3, i = 1
fn select_511([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_32([a, b, c], is_dual)
    } else {
        select_32([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_510([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_511([a, d, e], is_dual)
    } else {
        select_29([a, b, c, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_509([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_142([h, g, e, d, f, a, c], !is_dual)
    } else {
        select_510([b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_424([b, c, a, d, e, g, f, h, i], is_dual)
    } else {
        select_509([c, b, f, g, e, a, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_513([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_426([a, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_37([a, d, c, h, g, e, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_515([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_133([a, b, e, f, d, g], is_dual)
    } else {
        select_45([b, c, d, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_514([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_178([a, b, c, h, e, g, j], is_dual)
    } else {
        select_515([b, g, d, e, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_512([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_513([c, e, b, d, a, f, g, h, i, j], is_dual)
    } else {
        select_514([c, a, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_507([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_508([b, c, a, d, h, f, i, j, g], is_dual)
    } else {
        select_512([b, a, c, e, d, f, h, g, j, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_502([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_503([a, b, d, g, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_507([b, a, c, d, l, h, k, g, m, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_501([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_502([a, c, b, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_502([b, c, a, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_500([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_501([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_501([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_499([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_500([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_500([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_523([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_31([a, h, b, f, i, j], is_dual)
    } else {
        select_25([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_522([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_88([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_523([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_524([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_142([g, f, a, d, e, b, c], !is_dual)
    } else {
        select_411([g, d, a, e, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_521([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_522([b, a, c, d, e, g, f, h, j, i], is_dual)
    } else {
        select_524([a, b, h, g, i, j, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_526([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_115([b, a, d, e, f, g], is_dual)
    } else {
        select_112([a, c, b, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_525([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_430([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_526([a, b, j, c, d, h, k, i, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_520([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_521([b, i, e, f, g, h, a, j, k, l], is_dual)
    } else {
        select_525([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_528([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_75([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_190([a, b, i, c, g, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_527([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_528([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_521([b, h, d, e, f, g, a, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_519([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_520([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_527([a, b, i, e, f, g, h, c, j, l, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_532([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_19([b, c, a, d, e, f, g], is_dual)
    } else {
        select_186([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_531([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_340([f, e, i, a, b, h, g], !is_dual)
    } else {
        select_532([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_530([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_531([a, b, e, c, f, g, d, h, i], is_dual)
    } else {
        select_531([a, b, e, d, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_534([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_309([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_151([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_533([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_534([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_534([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_529([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_530([b, c, d, e, i, h, a, j, k], is_dual)
    } else {
        select_533([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_518([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_519([a, b, c, f, d, e, g, h, i, k, j, l], is_dual)
    } else {
        select_529([a, b, i, d, e, c, g, h, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_539([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_211([f, d, e, b, c, g, i, h, j], is_dual)
    } else {
        select_168([h, a, d, e, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_538([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_329([b, f, d, e, a, g, h, i, j, k], is_dual)
    } else {
        select_539([b, a, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_537([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_538([a, d, e, b, c, f, g, h, i, k, j], is_dual)
    } else {
        select_193([a, h, b, c, f, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_543([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_144([a, c, d, e, f, g, i], is_dual)
    } else {
        select_412([a, b, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_544([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_144([a, b, c, d, g, e, f], is_dual)
    } else {
        select_144([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_542([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_543([b, e, c, d, f, h, g, j, i], is_dual)
    } else {
        select_544([b, a, g, f, e, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_546([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_133([a, c, e, h, d, g], is_dual)
    } else {
        select_133([b, c, d, h, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_79([d, e, c, g, f], is_dual)
    } else {
        select_546([a, b, f, d, e, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_541([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_542([c, d, a, b, f, g, e, j, h, i], is_dual)
    } else {
        select_545([c, e, d, f, a, g, h, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_548([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_350([a, e, b, g, h, d, f], is_dual)
    } else {
        select_350([a, d, c, f, h, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_547([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_548([a, b, c, e, f, i, j, h], is_dual)
    } else {
        select_545([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_540([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_541([a, d, g, b, e, i, f, k, j, h], is_dual)
    } else {
        select_547([a, d, f, c, e, h, g, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_536([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_537([b, a, c, d, e, g, h, i, f, k, j], is_dual)
    } else {
        select_540([b, c, d, e, g, f, h, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_321([a, b, g, c, e, i, h, j, f, k], is_dual)
    } else {
        select_536([a, c, b, e, d, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_517([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_518([a, c, d, b, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_535([a, c, d, f, e, h, i, j, k, b, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_516([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_517([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_517([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_498([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_499([a, b, c, d, e, g, i, h, j, k, m, l, n], is_dual)
    } else {
        select_516([a, b, c, d, e, f, g, l, j, i, n, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_555([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_103([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_103([a, c, d, e, f, b, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_554([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_414([c, a, d, f, b, e, g, h, i, j], is_dual)
    } else {
        select_555([a, b, c, e, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_558([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_115([a, c, d, e, f, g], is_dual)
    } else {
        select_22([b, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_557([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_410([a, c, b, f, g, h, e, i], is_dual)
    } else {
        select_558([b, c, a, d, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_559([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_349([a, c, b, g, f, h, e, i], is_dual)
    } else {
        select_249([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_556([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_557([a, d, c, e, f, b, h, g, i], is_dual)
    } else {
        select_559([a, d, b, e, f, c, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_553([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_554([a, b, c, e, g, f, h, i, j, k, l], is_dual)
    } else {
        select_556([a, c, b, d, k, i, l, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_552([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_553([c, a, d, b, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_553([c, b, d, a, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_551([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_552([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_552([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_550([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_551([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_551([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_564([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_474([f, g, e, b, a, h, c], !is_dual)
    } else {
        select_310([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_563([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_564([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_564([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_567([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_78([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_173([h, d, e, f, g, c, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_569([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_81([a, b, c, d, f], is_dual)
    } else {
        select_81([a, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_570([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_136([g, c, d, e, a, h, j], is_dual)
    } else {
        select_523([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_568([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_569([a, c, d, e, f, i], is_dual)
    } else {
        select_570([a, b, c, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_566([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_567([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_568([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_573([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_76([b, c, d, e, f], is_dual)
    } else {
        select_32([a, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_572([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_573([a, c, d, e, f, g, i, k], is_dual)
    } else {
        select_522([a, b, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_576([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_31([a, c, b, d, e, f], is_dual)
    } else {
        select_113([a, b, e, f, d], is_dual)
    }
}
/// n = 11, i = 3
fn select_575([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([c, d, e, f, h], is_dual)
    } else {
        select_576([a, b, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_574([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_575([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_575([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_571([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_572([c, a, d, e, f, g, h, b, i, k, j], is_dual)
    } else {
        select_574([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_565([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_566([a, c, b, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_571([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_562([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_563([a, b, d, e, i, c, j, k], is_dual)
    } else {
        select_565([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_578([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_530([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_530([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_579([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_533([a, b, d, e, f, g, h, c, i, j], is_dual)
    } else {
        select_533([a, c, d, e, f, g, h, b, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_577([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_578([a, b, d, e, f, i, c, j, k], is_dual)
    } else {
        select_579([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_561([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_562([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_577([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_560([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_319([a, b, c, e, f, g, i, j, d, k], is_dual)
    } else {
        select_561([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_549([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_550([a, b, c, d, e, f, i, h, j, l, k, m], is_dual)
    } else {
        select_560([a, b, c, d, f, e, g, k, m, i, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_497([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_498([a, b, c, d, f, e, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_549([a, b, c, d, e, f, j, h, l, k, m, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_496([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_497([a, b, c, d, e, g, h, i, f, j, k, l, m, n], is_dual)
    } else {
        select_497([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_589([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([c, d, e, f, h], is_dual)
    } else {
        select_255([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_588([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_111([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_589([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_587([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_588([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_588([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_591([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_151([a, b, i, d, e, f, g, h, j, k], is_dual)
    } else {
        select_151([a, c, h, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_590([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_591([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_591([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_586([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_587([a, b, c, d, e, f, i, h, k, l, j], is_dual)
    } else {
        select_590([a, d, e, g, b, c, f, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_596([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_46([a, d, g, f, c], is_dual)
    } else {
        select_46([b, c, g, e, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_595([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_112([f, b, c, g, e, h], is_dual)
    } else {
        select_596([a, b, d, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_595([c, d, a, e, b, f, g, h, i], is_dual)
    } else {
        select_595([c, d, b, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_593([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_153([f, a, b, d, e, h, g, i], is_dual)
    } else {
        select_594([a, b, c, g, f, h, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_598([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_204([c, d, e, b, f, g, h], is_dual)
    } else {
        select_32([a, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_600([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_81([e, c, d, f, g], is_dual)
    } else {
        select_159([a, b, g, e, h, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_599([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_204([d, e, h, b, g, f, i], is_dual)
    } else {
        select_600([a, c, b, f, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_597([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_598([a, b, e, f, h, g, c, i, j, k], is_dual)
    } else {
        select_599([a, b, d, c, e, g, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_592([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_593([c, d, g, e, i, a, h, f, j, k], is_dual)
    } else {
        select_597([b, c, d, a, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_585([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_586([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_592([b, c, d, e, g, h, j, i, a, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_584([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_585([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    } else {
        select_585([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_602([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_437([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_437([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_604([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_473([a, b, c, d, f, e, g], is_dual)
    } else {
        select_473([a, b, c, e, g, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_603([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_604([a, b, c, d, f, e, g], is_dual)
    } else {
        select_604([a, b, c, e, f, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_601([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_602([a, b, c, g, e, f, h, i, j], is_dual)
    } else {
        select_603([a, b, c, i, j, d, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_583([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_584([a, b, c, e, d, f, g, h, i, j, k, l], is_dual)
    } else {
        select_601([a, b, c, e, f, i, j, k, d, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_605([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_470([a, b, c, e, h, g, i, j, d], is_dual)
    } else {
        select_319([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_582([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_583([a, b, c, d, f, e, g, h, i, j, k, l], is_dual)
    } else {
        select_605([a, b, c, d, f, i, j, k, e, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_612([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_59([b, c, a, d, e, f, g], is_dual)
    } else {
        select_48([a, g, b, c, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_611([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_612([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_612([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_610([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_438([a, b, g, d, h, c, i], is_dual)
    } else {
        select_611([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_609([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_610([a, b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_473([a, b, c, h, i, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_617([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_112([d, b, c, e, f, g], is_dual)
    } else {
        select_510([a, b, d, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_616([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_253([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_617([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_615([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_531([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_616([c, b, a, e, f, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_620([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_142([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_188([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_619([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_620([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_620([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_618([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_619([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_176([b, c, f, e, g, h, a], is_dual)
    }
}
/// n = 9, i = 3
fn select_614([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_615([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_618([a, c, d, g, f, h, b, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_624([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_87([a, b, d, e], !is_dual)
    } else {
        select_36([a, d, e, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_623([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_624([a, b, f, c, g], is_dual)
    } else {
        select_493([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_626([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_331([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_88([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_625([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_626([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_626([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_622([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_623([a, b, c, d, h, g, i], is_dual)
    } else {
        select_625([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_628([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_139([a, c, d, e, f, b, g, i], is_dual)
    } else {
        select_139([a, b, d, e, f, c, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_630([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_233([a, b, c, d, f, g, e], is_dual)
    } else {
        select_233([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_631([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_135([f, a, b, d, e], is_dual)
    } else {
        select_135([e, a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_629([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_630([b, c, d, a, e, f, g], is_dual)
    } else {
        select_631([a, c, d, b, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_627([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_628([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_629([a, b, h, i, c, d, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_621([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_622([a, b, d, f, c, e, g, h, i], is_dual)
    } else {
        select_627([a, b, c, d, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_613([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_614([a, b, g, d, f, c, h, i, j], is_dual)
    } else {
        select_621([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_608([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_609([a, b, c, d, e, j, k, g, i], is_dual)
    } else {
        select_613([a, b, c, d, g, f, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_635([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_324([a, b, c, f, e, h, g], is_dual)
    } else {
        select_324([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_634([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_635([a, b, c, f, e, h, g, j], is_dual)
    } else {
        select_635([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_633([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_634([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_634([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_636([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_634([a, b, c, e, d, f, g, h, i, j], is_dual)
    } else {
        select_634([a, b, d, e, c, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_632([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_633([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_636([a, b, c, d, e, g, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_607([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_608([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_632([a, b, c, e, h, g, j, i, k, d], is_dual)
    }
}
/// n = 7, i = 1
fn select_642([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_59([a, b, c, e, f, d, g], is_dual)
    } else {
        select_59([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_643([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_90([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_90([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_641([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_642([a, c, d, e, b, g, h], is_dual)
    } else {
        select_643([a, b, f, c, d, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_644([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_642([a, b, c, d, g, h, j], is_dual)
    } else {
        select_283([a, e, f, b, c, d, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_640([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_641([a, b, c, d, e, h, i, g, j, k], is_dual)
    } else {
        select_644([a, c, d, e, b, f, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_639([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_640([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_640([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_648([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_105([a, b, f, d, g, e, h], is_dual)
    } else {
        select_401([a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_647([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_103([a, f, c, d, e, g, h, j, i], is_dual)
    } else {
        select_648([a, h, b, e, i, f, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_646([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_647([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_647([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_651([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_24([b, a, c, d, e, f], is_dual)
    } else {
        select_155([a, e, g, h, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_650([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_651([e, g, c, d, f, h, i, j], is_dual)
    } else {
        select_239([e, c, d, a, b, f, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_649([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_373([a, b, c, f, h, g, i, j], is_dual)
    } else {
        select_650([c, g, d, e, a, b, j, h, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_645([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_646([a, b, g, e, f, h, c, i, j, k], is_dual)
    } else {
        select_649([a, b, c, e, f, d, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_638([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_639([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_645([a, b, c, g, e, f, h, j, i, d, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_655([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_323([b, a, f, e, c, g, i, h], is_dual)
    } else {
        select_512([a, b, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_654([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_655([a, c, d, b, e, g, h, i, f, j], is_dual)
    } else {
        select_655([b, c, d, a, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_653([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_654([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_654([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_652([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_653([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_653([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_637([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_638([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_652([a, b, c, d, e, f, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_606([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_607([a, b, c, d, f, g, h, i, e, j, k], is_dual)
    } else {
        select_637([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_581([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_582([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_606([a, b, c, d, e, i, h, f, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_662([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_373([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_492([a, b, c, d, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_661([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_662([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_662([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_667([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_31([a, f, b, e, d, g], is_dual)
    } else {
        select_45([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_666([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_667([a, b, c, d, e, f, g], is_dual)
    } else {
        select_50([a, b, f, d, g, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_665([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_666([a, c, d, b, e, f, g], is_dual)
    } else {
        select_666([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_664([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_490([a, b, c, f, g, h], is_dual)
    } else {
        select_665([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_663([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_664([a, b, c, e, d, f, g, h], is_dual)
    } else {
        select_664([a, b, d, e, c, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_660([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_661([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_663([a, b, c, d, g, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_668([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_661([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_661([a, b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_659([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_660([a, b, c, d, e, f, h, i, g], is_dual)
    } else {
        select_668([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_658([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_659([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_659([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_657([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_658([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_658([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_676([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, b, c, e, h, i], is_dual)
    } else {
        select_45([d, b, c, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_675([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_115([b, e, c, d, g, h], is_dual)
    } else {
        select_676([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_678([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_144([b, a, c, d, e, f, g], is_dual)
    } else {
        select_38([a, e, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_677([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_411([d, c, a, f, e, g], is_dual)
    } else {
        select_678([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_674([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_675([b, c, d, e, a, g, f, i, h, j], is_dual)
    } else {
        select_677([a, b, c, g, h, i, j, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_679([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_666([a, b, c, d, f, e, g], is_dual)
    } else {
        select_620([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_673([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_674([a, c, d, b, e, f, g, h, j, i], is_dual)
    } else {
        select_679([a, c, h, f, i, g, b, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_672([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_673([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_673([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_683([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_576([b, g, c, a, f, h], is_dual)
    } else {
        select_35([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_683([a, b, e, c, d, f, g, i], is_dual)
    } else {
        select_441([a, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_685([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_48([a, g, f, h, d, b], !is_dual)
    } else {
        select_48([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_684([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_440([a, b, f, d, i, g], is_dual)
    } else {
        select_685([a, b, c, g, e, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_681([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_682([a, d, b, c, e, g, f, i, h], is_dual)
    } else {
        select_684([a, d, c, e, f, b, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_687([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_620([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_620([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_686([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_687([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_665([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_680([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_681([a, b, e, d, c, g, f, i, h], is_dual)
    } else {
        select_686([a, b, e, f, h, g, c, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_671([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_672([a, b, d, c, e, f, g, i, j, h], is_dual)
    } else {
        select_680([a, b, c, e, d, i, h, j, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_689([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_662([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_662([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_688([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_614([a, b, c, d, e, f, h, i, g], is_dual)
    } else {
        select_689([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_670([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_671([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_688([a, b, d, h, e, g, c, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_669([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_670([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_670([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_656([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_657([a, b, c, d, e, h, i, f, j], is_dual)
    } else {
        select_669([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_580([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_581([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_656([a, b, c, d, e, f, k, l, g, i], is_dual)
    }
}
/// n = 14, i = 3
fn select_495([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < g) || (is_dual && n > g) {
        select_496([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_580([a, b, c, d, e, f, j, k, l, m, n, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_699([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_18([d, e, a, c, f, g, h, i], is_dual)
    } else {
        select_139([a, b, c, g, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_698([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_699([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_112([a, c, j, f, b, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_703([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_46([b, f, d, a, h], is_dual)
    } else {
        select_28([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_702([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_703([e, b, a, d, f, h, i, g], is_dual)
    } else {
        select_703([d, c, a, e, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_704([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_142([a, b, h, e, d, i, g], is_dual)
    } else {
        select_411([b, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_701([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_702([d, b, c, a, e, f, h, i, g], is_dual)
    } else {
        select_704([c, a, d, e, b, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_707([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_511([a, g, h], is_dual)
    } else {
        select_28([a, c, b, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_708([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_38([a, g, c, i], is_dual)
    } else {
        select_28([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_706([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_707([e, d, c, a, f, h, g, i], is_dual)
    } else {
        select_708([a, c, b, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_706([a, d, c, b, e, f, g, h, i], is_dual)
    } else {
        select_112([a, d, g, e, b, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_700([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_701([b, a, e, d, f, g, i, h, j], is_dual)
    } else {
        select_705([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_697([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_698([b, c, d, a, e, g, f, h, j, i, k], is_dual)
    } else {
        select_700([a, b, c, h, d, g, i, f, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_711([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_19([a, d, b, c, e, f, g], is_dual)
    } else {
        select_312([a, g, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_712([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_115([b, a, c, d, e, f], is_dual)
    } else {
        select_312([a, f, b, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_710([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_711([a, e, h, d, f, g, j, i, k], is_dual)
    } else {
        select_712([a, b, c, i, f, k, h, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_709([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_528([a, b, d, e, f, j, g, i, k, m, l], is_dual)
    } else {
        select_710([a, b, d, c, k, g, h, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_696([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_697([a, b, c, k, d, g, l, h, j, i, m], is_dual)
    } else {
        select_709([a, b, d, c, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_717([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_81([a, c, d, e, f], is_dual)
    } else {
        select_79([a, b, c, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_716([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([h, a, b, c, d, g, i], is_dual)
    } else {
        select_717([c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_718([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_59([c, d, e, a, b, f, g], is_dual)
    } else {
        select_717([a, b, c, d, f, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_715([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_716([a, b, c, d, f, g, j, i, k], is_dual)
    } else {
        select_718([c, d, a, b, e, h, k, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_720([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_18([d, e, a, b, f, g, h, i], is_dual)
    } else {
        select_59([a, b, c, g, h, f, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_721([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_212([c, h, e, f, a, b, g, j, i, k], is_dual)
    } else {
        select_212([d, g, e, f, a, b, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_719([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_720([a, b, c, d, e, h, i, j, l, m], is_dual)
    } else {
        select_721([a, b, d, e, f, g, i, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_714([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_715([c, d, a, b, e, i, j, h, m, l, k], is_dual)
    } else {
        select_719([c, d, e, f, g, a, b, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 2
fn select_723([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_721([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_721([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_726([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_118([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_523([a, b, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_725([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_151([a, c, g, d, e, f, i, h, j, l], is_dual)
    } else {
        select_726([a, b, h, d, e, f, i, g, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_729([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_133([b, c, d, f, g, h], is_dual)
    } else {
        select_29([a, d, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_728([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_729([c, a, b, d, f, e, h, i, g], is_dual)
    } else {
        select_28([c, e, d, f, a, i, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_727([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_728([a, b, c, j, g, h, i, k, l], is_dual)
    } else {
        select_173([c, d, e, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_724([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_725([c, a, d, e, f, g, b, h, i, j, l, k], is_dual)
    } else {
        select_727([c, b, d, e, f, g, a, h, i, j, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_722([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_723([a, c, d, e, g, b, h, i, j, k, l], is_dual)
    } else {
        select_724([a, b, c, f, d, e, i, h, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_713([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_714([b, c, d, e, g, a, f, j, h, i, k, m, l], is_dual)
    } else {
        select_722([b, a, c, d, e, i, g, l, j, k, h, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_695([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < m) || (is_dual && a > m) {
        select_696([b, c, d, f, e, j, h, i, k, l, m, a, n], is_dual)
    } else {
        select_713([b, c, d, a, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_733([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_308([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_107([a, b, c, h, g, d, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_732([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_16([a, b, c, f, d, e, g, h, i, j, k], is_dual)
    } else {
        select_733([c, a, d, e, b, h, g, j, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_737([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_19([d, e, b, c, f, g, h], is_dual)
    } else {
        select_255([h, a, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_739([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_412([a, b, d, e, h, i], is_dual)
    } else {
        select_412([a, c, d, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_738([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_56([a, g, d, c, f, h, i], is_dual)
    } else {
        select_739([a, b, d, h, e, f, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_736([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_737([b, c, e, d, g, h, i, f, j], is_dual)
    } else {
        select_738([a, b, f, d, g, h, i, e, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_741([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_678([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_678([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_742([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_330([b, a, e, f, g, d, h, i], is_dual)
    } else {
        select_544([b, c, d, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_740([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_741([d, b, c, a, e, f, g, h, i], is_dual)
    } else {
        select_742([c, d, a, e, b, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_735([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_736([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_740([a, f, h, b, j, g, i, e, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_746([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_144([a, c, d, e, h, g, j], is_dual)
    } else {
        select_144([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_747([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([a, e, b, d, g, f], is_dual)
    } else {
        select_36([a, c, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_745([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_746([b, d, a, c, e, f, g, i, h, j], is_dual)
    } else {
        select_747([a, d, c, f, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_748([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_543([a, b, c, f, e, h, g, i, j], is_dual)
    } else {
        select_144([a, d, g, e, h, f, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_744([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_745([a, d, f, c, h, g, e, j, k, i], is_dual)
    } else {
        select_748([c, e, b, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_751([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_144([a, b, f, d, g, e, h], is_dual)
    } else {
        select_144([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_750([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_515([a, f, b, e, g, i, j], is_dual)
    } else {
        select_751([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_749([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_702([d, b, f, e, a, g, j, i, h], is_dual)
    } else {
        select_750([b, a, d, c, e, g, f, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_744([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_749([a, g, c, d, i, f, h, e, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_734([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_735([a, d, b, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_743([a, d, c, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_731([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_732([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    } else {
        select_734([b, c, d, e, f, g, h, i, a, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_755([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_154([a, b, g, d, e, f, h, i], is_dual)
    } else {
        select_226([c, f, d, e, b, g, h], is_dual)
    }
}
/// n = 12, i = 2
fn select_754([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_755([a, b, c, h, i, j, g, l, k], is_dual)
    } else {
        select_402([a, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_758([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_115([b, c, d, e, f, g], is_dual)
    } else {
        select_380([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_757([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_226([c, f, d, e, b, g, h], is_dual)
    } else {
        select_758([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_756([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_757([a, b, c, h, i, j, g, l, k, m], is_dual)
    } else {
        select_402([a, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_753([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_754([a, c, b, d, e, f, g, i, j, k, h, l], is_dual)
    } else {
        select_756([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 12, i = 2
fn select_759([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_754([c, a, b, d, e, f, h, i, j, k, g, l], is_dual)
    } else {
        select_754([c, b, a, d, e, f, g, i, j, k, h, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_752([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_753([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    } else {
        select_759([a, d, b, e, f, g, c, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_730([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_731([a, c, b, d, e, j, k, i, l, h, m], is_dual)
    } else {
        select_752([a, c, d, e, f, b, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_694([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_695([a, b, c, f, d, g, e, h, j, i, k, l, m, n], is_dual)
    } else {
        select_730([a, b, c, d, e, f, j, h, i, l, k, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_693([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_694([g, d, c, e, f, a, b, i, k, h, m, l, j, n], is_dual)
    } else {
        select_694([g, d, c, f, e, a, b, i, j, h, m, l, k, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_692([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_693([g, h, f, a, d, e, b, k, c, i, j, m, l, n], is_dual)
    } else {
        select_693([g, h, f, c, d, e, b, k, a, i, j, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_691([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_692([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    } else {
        select_692([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_691([a, b, c, d, e, f, h, i, j, g, k, l, m, n], is_dual)
    } else {
        select_691([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_494([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_495([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    } else {
        select_690([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_362([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_363([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_494([a, b, c, d, e, f, g, h, l, k, i, m, n, o], is_dual)
    }
}
/// n = 12, i = 3
fn select_765([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_333([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_333([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_766([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_459([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_459([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_764([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_765([a, b, c, d, e, f, g, h, k, l, j, m], is_dual)
    } else {
        select_766([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 1
fn select_774([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_165([e, f, g, a, i, c, d, h, j, k], is_dual)
    } else {
        select_165([e, f, g, b, h, c, d, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_773([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_774([c, d, a, i, e, f, g, h, j, k, l], is_dual)
    } else {
        select_166([a, h, b, j, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_772([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_773([a, b, c, d, e, f, g, j, i, k, h, l, m, n], is_dual)
    } else {
        select_773([b, a, c, d, e, f, g, j, h, k, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_771([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_772([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    } else {
        select_772([b, c, d, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_771([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    } else {
        select_771([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_780([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_48([a, b, c, i, j, k], is_dual)
    } else {
        select_76([d, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_779([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_172([c, d, i, e, f, g, h, j, k], is_dual)
    } else {
        select_780([a, b, j, e, f, g, h, i, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_779([a, b, c, j, e, f, g, h, k, i, l, m, n], is_dual)
    } else {
        select_779([a, b, d, i, e, f, g, h, k, j, l, m, n], is_dual)
    }
}
/// n = 8, i = 1
fn select_783([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_70([a, b, c, d, e, g, h], is_dual)
    } else {
        select_70([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_782([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_172([b, c, h, d, e, f, g, i, j], is_dual)
    } else {
        select_783([a, i, d, e, f, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_785([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_76([d, e, f, g, i], is_dual)
    } else {
        select_136([a, b, c, j, h, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_784([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_785([a, b, i, d, e, f, g, h, j, k, l, m], is_dual)
    } else {
        select_783([c, j, d, e, f, g, i, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_781([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_782([b, c, k, d, e, f, g, i, j, l], is_dual)
    } else {
        select_784([a, j, b, d, e, f, g, h, i, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_778([a, b, c, d, e, f, g, h, j, k, i, l, m, n], is_dual)
    } else {
        select_781([b, c, d, e, f, g, h, i, j, k, a, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_777([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_777([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 2
fn select_789([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_292([g, b, c, e, f, a, h, i, j], is_dual)
    } else {
        select_317([a, d, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_788([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_789([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_225([a, d, g, e, f, h, b, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_792([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_48([a, e, c, f, h, i], is_dual)
    } else {
        select_19([b, d, a, c, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_791([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_792([h, b, c, a, f, e, g, i, j], is_dual)
    } else {
        select_792([g, b, d, a, f, e, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_794([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_81([e, b, c, f, h], is_dual)
    } else {
        select_19([a, d, b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_793([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_794([a, b, h, c, e, g, f, i], is_dual)
    } else {
        select_19([a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_790([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_791([e, b, c, d, h, a, f, g, i, j], is_dual)
    } else {
        select_793([c, d, a, e, f, g, b, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_787([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_788([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_790([a, c, d, i, e, g, b, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_796([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_104([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_104([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_795([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_439([a, b, c, g, h, f], !is_dual)
    } else {
        select_796([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_786([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_787([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_795([a, b, g, h, i, j, c, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_775([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < d) || (is_dual && l > d) {
        select_776([a, b, c, e, f, d, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_786([a, b, c, l, e, f, d, j, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_769([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_770([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_775([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_803([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_76([c, d, e, f, g], is_dual)
    } else {
        select_312([a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_804([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([c, d, e, f, h], is_dual)
    } else {
        select_49([a, b, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_802([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_803([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_804([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_805([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_75([a, h, c, d, e, f, g, i, j], is_dual)
    } else {
        select_783([b, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_801([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_802([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_805([b, a, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_807([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_430([a, b, h, d, e, f, g, i, j, k], is_dual)
    } else {
        select_783([c, i, d, e, f, g, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_808([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_780([a, h, c, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_804([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_807([b, d, a, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_808([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_800([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_801([a, i, c, e, f, g, h, b, j, k, l], is_dual)
    } else {
        select_806([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 1
fn select_812([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_76([e, f, g, h, i], is_dual)
    } else {
        select_76([a, b, c, d, j], is_dual)
    }
}
/// n = 11, i = 1
fn select_811([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_783([a, j, e, f, g, h, i, k], is_dual)
    } else {
        select_812([b, c, d, i, e, f, g, h, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_813([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_430([a, b, k, f, g, h, i, j, l, m], is_dual)
    } else {
        select_812([c, d, e, j, f, g, h, i, k, l], is_dual)
    }
}
/// n = 13, i = 2
fn select_810([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_811([j, c, d, e, f, g, h, i, b, k, l], is_dual)
    } else {
        select_813([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_809([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_810([a, c, d, e, f, g, h, i, j, b, k, l, m], is_dual)
    } else {
        select_810([b, c, d, e, f, g, h, i, j, a, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_799([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_800([a, b, d, k, g, h, i, j, c, l, m, n], is_dual)
    } else {
        select_809([a, b, d, c, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_816([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_464([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_464([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_819([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_142([b, a, e, c, f, d, g], is_dual)
    } else {
        select_142([b, a, e, d, f, c, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_820([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_142([a, b, c, e, d, g, f], is_dual)
    } else {
        select_50([g, c, d, e, f, a], !is_dual)
    }
}
/// n = 7, i = 3
fn select_818([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_819([c, a, e, f, d, b, g], is_dual)
    } else {
        select_820([c, b, e, f, a, g, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_817([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_818([a, c, d, b, e, f, g], is_dual)
    } else {
        select_818([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_815([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_816([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_817([a, b, d, h, c, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_822([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_295([a, f, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_273([h, b, c, d, e, g, f, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_826([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_49([a, c, d, e, f, g], is_dual)
    } else {
        select_38([a, b, h, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_827([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_49([a, b, c, f, i, j], is_dual)
    } else {
        select_20([b, c, d, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_825([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_826([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_827([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_824([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_825([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_825([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_823([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_824([f, g, a, i, d, e, h, j, k, l], is_dual)
    } else {
        select_234([f, g, d, e, b, c, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_821([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_822([f, a, b, c, d, g, h, e, i, j, k], is_dual)
    } else {
        select_823([e, a, b, c, d, g, h, f, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_815([a, b, c, d, e, f, j, k, l, i], is_dual)
    } else {
        select_821([d, f, g, h, e, a, b, c, i, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_798([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_799([a, b, c, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_814([a, b, c, d, e, m, f, g, k, l, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_797([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_798([a, b, c, d, f, g, h, i, j, k, e, l, m, n, o], is_dual)
    } else {
        select_798([a, b, c, e, f, g, h, i, j, k, d, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_768([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_769([a, b, c, d, e, f, l, i, j, k, g, m, n, o], is_dual)
    } else {
        select_797([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 7, i = 3
fn select_832([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_477([a, b, c, d, f, e, g], is_dual)
    } else {
        select_491([a, b, f, e, c], is_dual)
    }
}
/// n = 11, i = 3
fn select_833([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_338([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_338([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_831([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_832([a, b, c, j, k, d, h], is_dual)
    } else {
        select_833([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_834([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_687([a, b, c, i, g, j, k, h], is_dual)
    } else {
        select_643([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_830([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_831([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_834([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_829([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_830([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    } else {
        select_830([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_828([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_829([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_829([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 15, i = 3
fn select_767([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < f) || (is_dual && n > f) {
        select_768([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_828([a, b, c, d, e, g, l, m, n, f, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_763([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < m) || (is_dual && g > m) {
        select_764([a, b, c, d, e, f, h, m, i, g, l, n, o], is_dual)
    } else {
        select_767([a, b, c, d, e, f, h, i, g, j, k, l, m, n, o], is_dual)
    }
}
/// n = 7, i = 2
fn select_843([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_325([a, c, d, b, f, e, g], is_dual)
    } else {
        select_311([a, f, b, c, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_842([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_483([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_843([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_841([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_816([a, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_842([a, b, c, d, i, j, h, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_845([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_153([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_153([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_846([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_149([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_149([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_844([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_845([a, b, c, d, e, i, h, j], is_dual)
    } else {
        select_846([a, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_840([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_841([a, b, c, d, e, f, i, h, k, l, j], is_dual)
    } else {
        select_844([a, b, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_839([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_840([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_840([a, b, d, c, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_838([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_839([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    } else {
        select_839([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_837([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_838([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_838([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_836([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_765([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_837([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_854([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_569([a, c, d, e, f, h], is_dual)
    } else {
        select_626([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_853([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_642([a, c, d, e, f, g, h], is_dual)
    } else {
        select_854([a, b, c, d, e, f, g, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_855([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_642([a, d, e, f, b, c, g], is_dual)
    } else {
        select_625([a, b, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_852([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_853([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_855([a, b, h, c, d, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_851([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_852([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_852([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_850([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_602([a, b, c, h, e, f, i, d, j], is_dual)
    } else {
        select_851([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_858([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_801([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_801([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_863([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, b, c, d, f, g], is_dual)
    } else {
        select_38([b, d, c, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_862([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_510([f, e, a, c, d], !is_dual)
    } else {
        select_863([f, g, d, a, b, e, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_864([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_29([b, c, d, e], is_dual)
    } else {
        select_32([a, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_861([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_862([a, b, c, e, f, d, g], is_dual)
    } else {
        select_864([c, a, f, d, g, b, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_866([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_61([a, c, e, d, g, f], is_dual)
    } else {
        select_863([g, h, e, a, b, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_865([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_866([b, a, c, d, e, g, f, h], is_dual)
    } else {
        select_864([c, b, g, f, h, a, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_860([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_861([b, f, c, e, a, g, h], is_dual)
    } else {
        select_865([b, a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_859([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_860([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_860([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_857([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_858([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_859([a, b, c, e, i, d, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_856([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_857([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_661([a, b, c, d, e, i, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_850([a, b, c, d, f, g, i, e, j, k], is_dual)
    } else {
        select_856([a, b, c, d, f, g, e, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_848([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_849([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_828([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_868([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_160([a, b, c, d, f, g, h, i, j, e, k, l, m], is_dual)
    } else {
        select_160([a, b, c, e, f, g, h, i, j, d, k, l, m], is_dual)
    }
}
/// n = 6, i = 0
fn select_875([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_76([a, b, c, d, e], is_dual)
    } else {
        select_76([a, b, d, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_876([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_112([a, b, c, d, e, f], is_dual)
    } else {
        select_32([a, f, b], is_dual)
    }
}
/// n = 12, i = 3
fn select_874([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_875([c, d, e, f, g, i], is_dual)
    } else {
        select_876([a, b, j, h, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_878([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_48([a, e, c, d, f, g], is_dual)
    } else {
        select_38([a, g, e, b], !is_dual)
    }
}
/// n = 13, i = 3
fn select_877([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_875([d, e, f, g, h, j], is_dual)
    } else {
        select_878([a, b, c, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_873([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_874([a, b, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_877([b, a, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_872([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_873([a, c, d, e, f, g, h, i, b, j, k, l, m], is_dual)
    } else {
        select_873([b, c, d, e, f, g, h, i, a, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_881([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_573([a, b, c, d, e, g, h, i], is_dual)
    } else {
        select_573([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_882([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_75([a, b, c, d, e, f, h, i, j], is_dual)
    } else {
        select_75([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_880([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_881([h, c, d, e, f, g, a, i, j], is_dual)
    } else {
        select_882([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 1
fn select_883([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_171([a, b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_171([a, b, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_879([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_880([a, i, d, e, f, g, h, b, j, k], is_dual)
    } else {
        select_883([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_871([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_872([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_879([a, b, j, e, f, g, h, i, d, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_870([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_871([a, b, c, e, f, g, h, i, j, d, k, l, m], is_dual)
    } else {
        select_871([a, b, d, e, f, g, h, i, j, c, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_869([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_870([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_661([a, b, c, d, e, k, f, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_867([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < k) || (is_dual && b > k) {
        select_868([a, b, c, d, e, f, g, h, i, j, l, m, n], is_dual)
    } else {
        select_869([a, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_847([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < l) || (is_dual && f > l) {
        select_848([a, b, c, d, e, g, h, l, f, m, n], is_dual)
    } else {
        select_867([a, b, c, d, e, g, h, f, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_835([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_836([a, b, c, d, e, f, h, i, l, g, m, n], is_dual)
    } else {
        select_847([a, b, c, d, e, f, h, i, g, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_762([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_763([a, b, c, d, e, f, g, j, k, h, i, l, m, n, o], is_dual)
    } else {
        select_835([a, b, c, d, e, f, g, h, i, j, l, n, m, o], is_dual)
    }
}
/// n = 14, i = 3
fn select_893([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_812([b, c, d, j, e, f, g, h, k, l], is_dual)
    } else {
        select_804([a, k, e, f, g, h, i, j, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_895([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([c, d, e, f, h], is_dual)
    } else {
        select_50([a, b, i, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_894([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_895([a, h, c, d, e, f, g, i, j, k, l], is_dual)
    } else {
        select_117([g, b, c, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_892([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_893([b, a, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_894([b, k, e, f, g, h, i, a, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_897([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_172([c, d, k, e, f, g, h, j, l], is_dual)
    } else {
        select_785([a, b, j, e, f, g, h, i, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_896([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_782([j, c, d, e, f, g, h, b, k, l], is_dual)
    } else {
        select_897([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_891([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_892([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_896([b, a, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 12, i = 3
fn select_901([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_76([c, d, e, f, i], is_dual)
    } else {
        select_515([a, j, b, g, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_900([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_901([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_589([a, g, c, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_899([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_900([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_864([b, k, h, a, l, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_904([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_87([b, c, d, e], is_dual)
    } else {
        select_49([a, b, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_905([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_112([d, b, c, f, e, g], is_dual)
    } else {
        select_863([a, e, b, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_903([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_904([a, b, f, d, g, e, h], is_dual)
    } else {
        select_905([a, c, b, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_907([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_76([d, e, f, g, i], is_dual)
    } else {
        select_178([a, b, c, j, h, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_906([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_589([i, b, d, e, f, g, h, j, k, l], is_dual)
    } else {
        select_907([a, h, c, d, e, f, g, j, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_902([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_903([b, a, k, j, h, l, m, i], is_dual)
    } else {
        select_906([b, a, c, d, e, f, g, h, j, i, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_898([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_899([b, h, d, e, f, g, i, a, j, k, l, m], is_dual)
    } else {
        select_902([b, a, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_890([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_891([b, c, a, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    } else {
        select_898([b, c, j, e, f, g, h, a, i, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_913([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_144([a, b, c, d, e, f, g], is_dual)
    } else {
        select_511([a, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_912([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_913([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_913([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_911([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_912([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_826([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_915([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_411([c, a, d, e, b, f], is_dual)
    } else {
        select_411([c, b, d, e, a, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_914([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_491([a, d, e, c, f], is_dual)
    } else {
        select_915([d, e, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_910([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_911([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_914([h, f, g, a, b, c, e], !is_dual)
    }
}
/// n = 10, i = 3
fn select_918([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_708([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_917([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_918([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_918([c, b, d, e, f, a, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_916([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_917([c, d, a, e, f, b, g, h, i, j], is_dual)
    } else {
        select_917([c, d, b, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_909([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_910([b, c, d, g, i, h, j, a], is_dual)
    } else {
        select_916([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_908([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_879([a, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_909([a, b, c, d, l, e, j, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_889([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_890([a, d, c, e, f, g, h, i, j, k, b, l, m, n], is_dual)
    } else {
        select_908([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_924([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_27([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_186([c, e, d, g, a, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_923([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_924([a, c, d, e, h, f, g, i, j], is_dual)
    } else {
        select_190([a, b, d, c, i, g, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_922([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_923([b, a, f, e, d, g, i, h, k, j], is_dual)
    } else {
        select_184([b, a, g, c, j, f, k, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_921([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_922([a, b, c, e, d, g, h, i, f, j, k], is_dual)
    } else {
        select_326([b, a, c, f, e, h, j, g, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_920([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_921([a, c, b, d, e, h, g, i, f, j, k], is_dual)
    } else {
        select_921([b, c, a, d, e, h, f, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_919([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_920([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_920([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_888([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < c) || (is_dual && l > c) {
        select_889([a, b, d, e, f, c, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_919([a, b, d, e, l, f, j, c, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_887([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_888([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_888([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_932([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_81([a, b, c, d, f], is_dual)
    } else {
        select_79([a, b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_931([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_569([a, c, d, e, f, g], is_dual)
    } else {
        select_932([a, h, f, i, b, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_930([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_931([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_626([a, b, d, e, f, c, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_929([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_930([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_817([a, b, d, g, c, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_936([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_48([e, f, d, h, g, a], !is_dual)
    } else {
        select_186([b, c, a, e, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_937([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_19([a, b, c, d, e, f, g], is_dual)
    } else {
        select_36([e, b, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_935([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_936([b, c, g, e, f, a, h, i], is_dual)
    } else {
        select_937([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_938([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_226([c, f, d, e, a, g, h], is_dual)
    } else {
        select_274([a, b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_934([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_935([a, c, h, d, f, b, g, i, j], is_dual)
    } else {
        select_938([a, c, d, b, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_940([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_239([e, a, b, c, d, f, g, h], is_dual)
    } else {
        select_448([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_939([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_940([c, h, e, f, a, b, g, i, j], is_dual)
    } else {
        select_940([d, g, e, f, a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_933([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_934([d, e, i, a, b, f, g, h, j, k], is_dual)
    } else {
        select_939([d, e, a, b, c, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_928([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_929([a, b, c, d, h, i, j, k, g], is_dual)
    } else {
        select_933([d, e, f, a, b, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_927([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_928([a, b, c, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_928([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_946([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_913([e, c, d, a, g, f, h, i], is_dual)
    } else {
        select_146([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_947([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_104([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_350([a, g, b, h, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_945([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_946([a, c, d, e, b, g, f, h, i], is_dual)
    } else {
        select_947([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_948([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_143([e, c, d, a, f, g, h], is_dual)
    } else {
        select_146([b, a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_944([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_945([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_948([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_950([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_39([b, c, d, a, e, f, h, g, i], is_dual)
    } else {
        select_792([b, g, c, d, a, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_949([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_950([a, b, j, c, d, h, i, g, k, l], is_dual)
    } else {
        select_283([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_943([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_944([a, b, c, i, j, g, k, l, h], is_dual)
    } else {
        select_949([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_942([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_943([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_943([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_941([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_942([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_942([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_926([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_927([a, b, c, d, e, g, j, i, f, k, l], is_dual)
    } else {
        select_941([a, b, c, d, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_925([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_769([a, b, c, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_926([a, b, c, d, e, f, h, l, k, g, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_886([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_887([a, b, c, d, e, f, i, g, h, l, k, n, o, m], is_dual)
    } else {
        select_925([a, b, c, d, e, g, h, j, f, k, l, m, n, o], is_dual)
    }
}
/// n = 12, i = 3
fn select_957([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_297([d, e, b, c, f, g, a, h, j, i, k], is_dual)
    } else {
        select_492([a, i, b, c, j, k, l, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_956([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_957([a, b, h, d, e, f, g, k, i, j, l, m], is_dual)
    } else {
        select_842([a, i, b, c, l, m, h, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_955([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_956([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_956([a, c, b, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_954([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_955([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_955([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_953([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_954([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_954([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_958([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_470([a, b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_470([a, b, c, e, f, d, g, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_952([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_953([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_958([a, b, c, d, e, l, m, f, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_965([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_157([a, c, d, e, f, g, h], is_dual)
    } else {
        select_59([a, b, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_967([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_115([a, b, c, h, g, j], is_dual)
    } else {
        select_25([d, e, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_966([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_967([b, c, f, a, d, e, g, h, i, j], is_dual)
    } else {
        select_249([b, h, f, c, a, g, j, i], is_dual)
    }
}
/// n = 12, i = 2
fn select_964([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_965([a, b, e, f, g, k, h, j], is_dual)
    } else {
        select_966([a, b, e, c, d, j, h, i, l, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_969([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_226([a, g, d, e, f, j, h], is_dual)
    } else {
        select_532([a, b, c, h, f, g, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_971([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_70([c, i, d, e, f, g, j], is_dual)
    } else {
        select_356([a, b, g, d, e, f, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_972([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_356([a, b, i, e, f, g, h, k, j, l], is_dual)
    } else {
        select_71([c, d, j, e, f, g, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_970([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_971([b, c, h, e, f, g, a, i, j, k, l], is_dual)
    } else {
        select_972([b, c, a, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_968([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_969([b, j, e, c, d, a, h, i, k, l, m], is_dual)
    } else {
        select_970([b, c, d, e, a, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_963([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_964([a, c, f, g, e, b, h, i, j, k, m, l], is_dual)
    } else {
        select_968([a, b, f, g, d, c, i, h, j, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_975([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_311([a, f, b, g, h, e], is_dual)
    } else {
        select_532([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_974([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_975([a, c, i, d, g, b, h, j], is_dual)
    } else {
        select_534([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_973([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_307([a, b, c, d, e, h, g, j, i, k], is_dual)
    } else {
        select_974([a, c, d, f, b, e, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 3
fn select_962([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_963([a, c, b, e, d, f, g, i, h, j, l, k, m, n], is_dual)
    } else {
        select_973([a, c, b, d, i, j, h, m, l, n, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_961([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_962([a, b, d, c, e, f, g, i, j, k, h, l, m, n], is_dual)
    } else {
        select_962([a, c, d, b, e, f, g, h, j, k, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_960([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_961([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_961([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_959([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_960([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    } else {
        select_960([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_951([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_952([a, b, c, d, e, f, j, h, i, l, k, n, m], is_dual)
    } else {
        select_959([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_885([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < f) || (is_dual && n > f) {
        select_886([a, b, c, d, e, g, f, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_951([a, b, c, d, e, g, i, h, l, k, n, m, f, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_984([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_217([b, c, a, d, e, f, h, g], is_dual)
    } else {
        select_510([b, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_983([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_866([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_984([b, a, d, e, c, f, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_987([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_144([a, c, d, b, e, f, g], is_dual)
    } else {
        select_144([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_986([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_330([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_987([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_985([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_986([a, b, d, e, f, g, i, h], is_dual)
    } else {
        select_932([a, c, d, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_982([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_983([a, b, h, c, d, i, g, j], is_dual)
    } else {
        select_985([a, b, c, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_981([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_982([a, b, d, g, e, f, i, h, j, k], is_dual)
    } else {
        select_842([a, h, b, c, j, k, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_980([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_981([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_981([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_979([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_980([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_980([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_978([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_979([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    } else {
        select_979([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_977([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_657([a, b, c, d, e, i, j, f, k], is_dual)
    } else {
        select_978([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_976([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_764([a, b, c, d, e, f, g, j, i, l, k, m, n], is_dual)
    } else {
        select_977([a, b, c, d, e, f, h, m, n, j, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_884([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < g) || (is_dual && m > g) {
        select_885([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_976([a, b, c, d, e, f, h, k, j, m, l, g, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_761([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_762([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_884([a, b, c, d, e, f, g, j, i, k, l, m, h, n, o], is_dual)
    }
}
/// n = 13, i = 3
fn select_995([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_213([a, c, e, b, f, g, d, i, h, j, k, l, m], is_dual)
    } else {
        select_973([a, c, b, d, j, e, h, i, l, m, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_994([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_995([a, b, d, c, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_995([a, c, d, b, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_993([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_994([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_994([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_992([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_993([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_993([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 12, i = 2
fn select_1001([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_206([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_206([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1000([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1001([a, f, b, c, d, e, g, h, i, j, k, l], is_dual)
    } else {
        select_823([i, b, c, d, e, g, h, f, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_1003([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_824([a, b, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_940([d, g, e, f, a, b, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1002([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_822([d, a, e, f, g, b, c, h, i, j, k], is_dual)
    } else {
        select_1003([b, c, i, d, f, g, a, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_999([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_1000([e, d, f, g, h, a, b, c, i, j, k, l, m], is_dual)
    } else {
        select_1002([a, b, c, e, j, g, h, i, d, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1008([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_76([b, c, d, e, g], is_dual)
    } else {
        select_46([a, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1007([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1008([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_119([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1006([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_931([a, b, c, d, j, h, i, k, l], is_dual)
    } else {
        select_1007([a, b, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1010([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_165([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_79([a, b, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_1011([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_118([a, i, d, e, f, g, h, j, k], is_dual)
    } else {
        select_71([b, c, h, d, e, f, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1009([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1010([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1011([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1005([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1006([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    } else {
        select_1009([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_1014([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_491([a, b, f, c, e], is_dual)
    } else {
        select_491([a, b, e, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1015([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_630([e, g, h, f, a, b, c], !is_dual)
    } else {
        select_630([f, g, h, e, a, b, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1013([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1014([e, f, a, h, g, b], !is_dual)
    } else {
        select_1015([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1018([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_275([a, b, i, g, j, k], is_dual)
    } else {
        select_76([c, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1017([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1018([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_119([h, a, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1019([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_573([a, c, d, e, f, g, i, j], is_dual)
    } else {
        select_309([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1016([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1017([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_1019([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1012([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_1013([a, c, d, i, h, b, j, k], is_dual)
    } else {
        select_1016([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1004([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_1005([a, b, d, c, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1012([a, b, d, i, f, g, h, c, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_998([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_999([a, b, c, e, g, h, d, i, j, k, l, m, n], is_dual)
    } else {
        select_1004([a, b, c, d, f, e, j, k, i, m, n, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1024([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_226([c, f, d, e, a, g, h], is_dual)
    } else {
        select_446([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1025([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_239([f, a, b, c, d, g, h, i], is_dual)
    } else {
        select_446([f, e, i, a, b, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1023([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_1024([a, b, h, e, f, g, c, i, j, k], is_dual)
    } else {
        select_1025([c, d, e, f, b, a, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1027([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_151([f, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_825([f, a, j, b, c, h, g, i, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1026([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1027([h, b, c, d, e, f, g, i, j, k, l, m], is_dual)
    } else {
        select_206([a, g, b, c, d, e, f, i, h, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1022([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1023([a, b, c, d, j, k, h, i, m, l, n], is_dual)
    } else {
        select_1026([e, c, i, f, g, b, a, j, h, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1021([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1022([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_1022([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1029([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1023([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_1023([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1028([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1029([a, b, c, d, e, i, j, h, l, k, m], is_dual)
    } else {
        select_1002([a, b, c, f, g, d, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1020([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_1021([a, b, c, d, f, g, e, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1028([a, b, c, d, f, g, k, i, j, e, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_997([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_998([a, b, c, d, f, e, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_1020([a, b, c, e, f, d, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_996([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_997([a, b, c, d, f, g, h, i, e, j, k, l, m, n], is_dual)
    } else {
        select_997([a, b, c, e, f, g, h, i, d, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_991([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_992([a, b, c, d, e, f, g, h, l, k, m, j, n], is_dual)
    } else {
        select_996([a, b, c, d, e, g, h, f, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_990([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_991([a, b, c, d, e, f, h, i, j, g, k, l, m, n], is_dual)
    } else {
        select_991([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1033([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_180([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    } else {
        select_180([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1032([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_658([a, b, c, d, e, k, f, i, l], is_dual)
    } else {
        select_1033([a, b, c, d, e, i, g, h, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_1042([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_49([a, c, d, e, f, h], is_dual)
    } else {
        select_49([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1041([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1042([a, f, c, d, e, g, h, i], is_dual)
    } else {
        select_264([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1043([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_19([a, c, d, e, f, g, h], is_dual)
    } else {
        select_255([a, b, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1040([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1041([b, f, d, e, g, a, h, j, i], is_dual)
    } else {
        select_1043([b, c, a, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1039([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_193([a, g, c, d, f, i, h], is_dual)
    } else {
        select_1040([a, b, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1046([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_115([b, d, a, e, f, g], is_dual)
    } else {
        select_115([a, c, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1047([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1042([a, e, c, d, g, f, i, h], is_dual)
    } else {
        select_46([e, i, h, a, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1045([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1046([a, b, d, c, f, e, h, g], is_dual)
    } else {
        select_1047([a, d, b, c, f, g, h, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1044([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1045([b, a, f, c, h, e, j, i, g], is_dual)
    } else {
        select_184([e, a, b, d, g, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1038([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1039([b, d, a, c, e, g, h, f, j, i], is_dual)
    } else {
        select_1044([b, d, e, c, g, f, h, a, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1037([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1038([a, b, c, d, e, g, h, i, f, j], is_dual)
    } else {
        select_1038([a, c, b, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1036([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1037([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_1037([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1035([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1036([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_1036([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1053([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_216([a, f, c, d, e, h, g, i, k, j], is_dual)
    } else {
        select_265([g, a, b, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1052([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_184([f, a, i, b, k, g, l, h, j], is_dual)
    } else {
        select_1053([a, g, c, d, e, h, f, j, i, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1054([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_193([a, k, b, c, g, l, h], is_dual)
    } else {
        select_51([a, d, b, c, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1051([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1052([b, c, d, e, f, h, g, a, i, j, k, l], is_dual)
    } else {
        select_1054([b, a, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1050([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1051([a, b, c, d, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_1051([a, c, b, d, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1049([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1050([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_1050([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1048([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1049([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_1049([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1034([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1035([a, b, c, d, e, f, j, k, l, i], is_dual)
    } else {
        select_1048([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1031([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1032([a, b, c, d, e, f, h, i, g, j, k, l], is_dual)
    } else {
        select_1034([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1030([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < g) || (is_dual && m > g) {
        select_300([a, b, c, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_1031([a, b, c, d, e, f, h, k, l, m, g, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_989([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_990([a, b, c, d, e, f, g, i, h, j, k, l, m, n], is_dual)
    } else {
        select_1030([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1065([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_104([a, d, e, g, f, h, i, j], is_dual)
    } else {
        select_104([a, b, c, h, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1064([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_108([a, b, c, h, f, i, g], is_dual)
    } else {
        select_1065([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1063([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_647([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_1064([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1067([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_143([d, b, c, e, f, g, h], is_dual)
    } else {
        select_411([g, a, h, i, d, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_1069([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_144([a, c, d, e, g, f, h], is_dual)
    } else {
        select_22([a, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_1068([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1069([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_87([a, b, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1066([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1067([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_1068([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1062([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1063([a, f, c, d, e, h, g, i, j, k], is_dual)
    } else {
        select_1066([a, b, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1073([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_188([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_667([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1075([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_144([a, b, c, d, e, f, g], is_dual)
    } else {
        select_32([a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1074([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1075([a, b, f, d, g, e, h, i], is_dual)
    } else {
        select_1075([a, c, e, d, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1072([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1073([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1074([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_1078([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_511([a, f, g], is_dual)
    } else {
        select_412([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1077([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_188([a, b, d, e, g, f, h, i], is_dual)
    } else {
        select_1078([h, i, e, g, a, b, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_1079([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_115([c, d, a, b, e, f], is_dual)
    } else {
        select_79([a, b, c, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1076([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1077([a, c, d, b, e, f, h, g, i], is_dual)
    } else {
        select_1079([c, d, b, e, g, h, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1071([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1072([b, c, d, e, f, g, h, a, i], is_dual)
    } else {
        select_1076([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1070([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_487([a, i, c, d, e, g, h, j], is_dual)
    } else {
        select_1071([a, b, c, d, f, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1061([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1062([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    } else {
        select_1070([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_1083([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_484([a, b, e, g, c, f, d], !is_dual)
    } else {
        select_311([a, b, d, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1082([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_477([a, b, c, e, g, f, h], is_dual)
    } else {
        select_1083([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1085([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_442([a, b, c, f, g, e, h], is_dual)
    } else {
        select_324([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1084([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_477([a, b, c, d, h, g, i], is_dual)
    } else {
        select_1085([a, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1081([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1082([a, b, d, c, h, j, f, i], is_dual)
    } else {
        select_1084([a, b, d, f, e, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1088([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_324([a, e, b, g, f, i, h], is_dual)
    } else {
        select_532([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1087([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1088([a, d, c, b, f, g, e, h, i], is_dual)
    } else {
        select_975([a, b, g, c, e, h, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1090([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_986([a, b, c, h, f, g, i, j], is_dual)
    } else {
        select_626([a, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1089([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1090([a, b, e, c, d, f, g, h, j, i], is_dual)
    } else {
        select_193([a, g, c, d, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1086([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1087([a, c, e, d, f, h, g, b, i, j], is_dual)
    } else {
        select_1089([a, c, b, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1080([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1081([a, b, e, d, c, h, g, i, j, f], is_dual)
    } else {
        select_1086([a, b, e, c, d, h, g, f, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1060([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1061([a, c, d, b, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1080([a, f, c, i, d, b, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1093([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1088([a, d, f, c, h, g, e, i, j], is_dual)
    } else {
        select_1088([a, b, c, g, e, f, i, h, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1097([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_31([a, b, d, g, e, h], is_dual)
    } else {
        select_38([e, f, c, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_1096([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_331([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1097([a, h, b, f, e, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1095([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_108([a, b, c, h, e, i, g], is_dual)
    } else {
        select_1096([a, d, b, c, e, f, h, g, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1100([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_144([a, c, d, e, g, f, h], is_dual)
    } else {
        select_29([a, b, e, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1099([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_208([a, c, h, d, e, i, g, k], is_dual)
    } else {
        select_1100([b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1098([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_108([c, a, b, g, i, j, h], is_dual)
    } else {
        select_1099([c, d, e, a, b, f, i, g, h, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1094([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1095([f, b, c, d, a, g, h, i, j, k], is_dual)
    } else {
        select_1098([b, c, a, d, e, g, i, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1092([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1093([a, c, d, e, g, f, h, b, i, j, k], is_dual)
    } else {
        select_1094([a, b, c, e, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1104([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_48([e, h, c, d, g, i], is_dual)
    } else {
        select_48([e, i, a, b, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1103([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1104([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_347([a, f, d, e, b, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1105([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_651([a, e, c, d, f, g, h, i], is_dual)
    } else {
        select_36([a, b, h, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1103([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_1105([a, f, d, e, g, b, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1108([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_118([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_515([g, c, b, a, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1107([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_254([a, b, d, h, g, j], is_dual)
    } else {
        select_1108([a, c, e, b, g, f, i, h, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1109([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_325([a, g, c, d, h, f, i], is_dual)
    } else {
        select_685([a, b, f, e, i, h, j, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1106([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1107([a, b, d, c, e, g, f, i, h, k, j], is_dual)
    } else {
        select_1109([a, f, e, d, h, g, b, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1102([a, b, c, e, j, f, i, h, l, k], is_dual)
    } else {
        select_1106([a, b, c, d, h, f, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1091([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1092([a, b, c, g, e, f, j, h, k, i, l], is_dual)
    } else {
        select_1101([a, b, c, e, d, f, h, g, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1059([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_1060([a, b, c, d, h, k, f, g, l, i, j], is_dual)
    } else {
        select_1091([a, b, c, e, d, f, h, g, i, k, j, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1116([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_112([a, e, b, h, g, i], is_dual)
    } else {
        select_667([a, c, d, f, e, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_1118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([c, d, g, f, a, e], !is_dual)
    } else {
        select_36([c, b, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1117([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_703([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1118([g, f, a, d, h, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1116([a, b, d, c, f, g, e, h, i], is_dual)
    } else {
        select_1117([a, e, d, b, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1120([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_105([e, f, h, a, b, g, d], !is_dual)
    } else {
        select_264([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1119([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_524([a, c, e, g, b, h, f], is_dual)
    } else {
        select_1120([a, c, b, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1114([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1115([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_1119([a, b, c, f, h, g, i, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_863([a, b, g, e, h, f, i], is_dual)
    } else {
        select_136([a, c, d, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1123([c, a, d, e, f, b, g, h, i], is_dual)
    } else {
        select_1123([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1122([a, b, c, d, g, f, i, j, h], is_dual)
    } else {
        select_755([c, d, e, a, b, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1113([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1114([b, c, d, g, f, i, h, a, j], is_dual)
    } else {
        select_1121([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_48([a, b, e, f, h, i], is_dual)
    } else {
        select_19([c, d, a, b, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_422([a, b, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_1126([a, b, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1129([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_217([a, b, c, f, e, g, h, i], is_dual)
    } else {
        select_49([a, d, g, e, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1128([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1129([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_142([e, i, a, g, h, b, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1127([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1115([a, b, e, d, h, f, g, i, j], is_dual)
    } else {
        select_1128([a, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1124([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1125([a, b, c, f, e, i, g, h, j, k], is_dual)
    } else {
        select_1127([a, b, c, g, d, j, h, i, f, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1113([f, b, c, g, e, a, h, i, j, k], is_dual)
    } else {
        select_1124([b, c, a, d, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1134([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_35([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_522([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_1134([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_1136([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, b, c, d, e, f], is_dual)
    } else {
        select_48([a, b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1135([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_480([a, g, h, b, e, c, f], !is_dual)
    } else {
        select_1136([a, b, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1132([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_1133([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_1135([a, b, d, h, g, c, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1132([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_733([a, d, e, f, b, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1140([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_515([d, b, c, g, f, e, h], is_dual)
    } else {
        select_678([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1140([a, f, b, d, i, e, g, j, h], is_dual)
    } else {
        select_1140([a, e, c, d, h, f, g, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1142([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_56([b, c, d, e, f, g, h], is_dual)
    } else {
        select_576([a, d, g, f, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1144([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_144([c, a, b, d, f, e, g], is_dual)
    } else {
        select_38([a, b, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1145([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_31([a, g, c, f, d, i], is_dual)
    } else {
        select_28([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1144([a, b, c, e, g, h, i, j], is_dual)
    } else {
        select_1145([a, c, d, g, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1141([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1142([a, b, c, f, e, h, i, g, j], is_dual)
    } else {
        select_1143([b, c, a, d, e, f, i, g, j, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1139([b, c, d, e, f, g, h, a, i, j], is_dual)
    } else {
        select_1141([b, a, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1137([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1138([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_1138([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1131([a, c, d, e, b, f, g, i, h, j], is_dual)
    } else {
        select_1137([a, c, d, e, f, g, h, i, b, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1112([a, b, c, e, d, f, h, g, i, j, k], is_dual)
    } else {
        select_1130([a, f, b, c, d, h, g, k, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1151([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_70([b, h, c, d, e, g, i], is_dual)
    } else {
        select_118([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1150([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1011([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_1151([b, h, d, e, f, g, a, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1153([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_112([g, b, c, d, f, h], is_dual)
    } else {
        select_411([b, a, d, h, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1152([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_328([b, c, a, d, e, f, h, g, i], is_dual)
    } else {
        select_1153([b, c, g, e, f, a, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1149([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1150([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1152([b, c, j, d, a, g, h, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1156([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_515([e, c, d, a, f, g, h], is_dual)
    } else {
        select_708([b, d, a, c, f, e, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1155([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1156([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_1156([c, b, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1157([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_157([a, b, d, e, f, c, g], is_dual)
    } else {
        select_139([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1154([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1155([a, b, c, i, d, g, j, h, k], is_dual)
    } else {
        select_1157([a, b, c, e, f, h, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1148([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1149([c, d, e, f, a, b, g, h, j, i, k], is_dual)
    } else {
        select_1154([a, b, c, e, d, h, g, j, k, i, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1159([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_749([a, b, g, c, e, i, f, j, h, k], is_dual)
    } else {
        select_749([a, b, f, d, e, h, g, j, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1163([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_45([a, c, d, e, h], is_dual)
    } else {
        select_28([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1163([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_863([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1116([a, b, d, e, g, h, f, i, j], is_dual)
    } else {
        select_1162([a, d, c, f, h, g, e, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1161([a, b, g, c, j, e, f, h, i, k], is_dual)
    } else {
        select_1161([a, b, f, d, i, e, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1158([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1159([a, c, d, e, b, f, g, i, j, h, k], is_dual)
    } else {
        select_1160([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1147([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_1148([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1158([b, c, d, e, h, g, k, i, j, a, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1166([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1115([a, b, f, c, i, e, g, j, h], is_dual)
    } else {
        select_1115([a, b, e, d, h, f, g, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1168([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_59([a, b, d, e, f, i, h], is_dual)
    } else {
        select_56([a, b, c, d, g, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1170([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_707([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_707([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1169([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_493([a, b, d, e, g, h, j], is_dual)
    } else {
        select_1170([a, b, h, c, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1168([c, d, e, f, a, b, g, h, i, j], is_dual)
    } else {
        select_1169([a, b, e, c, d, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1165([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1166([b, c, d, e, i, g, h, a, j, k], is_dual)
    } else {
        select_1167([b, c, a, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1165([a, b, c, e, d, f, h, i, j, g, k], is_dual)
    } else {
        select_1131([a, b, c, e, d, g, j, i, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1147([a, b, c, d, f, e, h, g, i, k, j, l], is_dual)
    } else {
        select_1164([a, b, c, e, d, h, k, g, i, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < l) || (is_dual && b > l) {
        select_1111([a, c, d, e, h, k, g, l, i, j, b], is_dual)
    } else {
        select_1146([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1058([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_1059([a, b, d, e, f, g, h, i, c, j, k, l], is_dual)
    } else {
        select_1110([a, b, c, d, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1057([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1058([a, b, c, d, f, g, e, h, i, j, k, l], is_dual)
    } else {
        select_1058([a, b, c, e, f, g, d, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_1179([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_36([a, d, f, h], is_dual)
    } else {
        select_38([b, c, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1178([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_708([b, a, d, c, f, g, e, i, h], is_dual)
    } else {
        select_1179([b, a, e, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1177([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_216([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_1178([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1176([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1177([a, d, c, b, e, g, h, f, j, i], is_dual)
    } else {
        select_975([a, d, f, e, g, b, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1175([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1176([a, b, c, d, j, g, h, k, i, l], is_dual)
    } else {
        select_974([a, b, d, g, e, f, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_204([c, d, f, b, e, g, h], is_dual)
    } else {
        select_878([e, a, b, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1185([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_25([c, d, e, g], is_dual)
    } else {
        select_49([a, b, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_71([b, c, h, d, e, f, i, j], is_dual)
    } else {
        select_1185([a, i, d, e, f, g, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_70([b, h, c, d, e, g, i], is_dual)
    } else {
        select_1185([a, g, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1183([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1184([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_1186([a, h, d, e, f, g, b, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1181([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1182([b, c, j, d, a, g, h, i, k, l], is_dual)
    } else {
        select_1183([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1188([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1186([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_1186([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1191([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_412([f, h, d, g, a, b], !is_dual)
    } else {
        select_38([b, f, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_913([a, b, c, e, f, g, d, h], is_dual)
    } else {
        select_1191([a, b, c, e, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1189([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_440([g, b, c, d, e, h], is_dual)
    } else {
        select_1190([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1187([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1188([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1189([b, c, d, i, a, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1180([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1181([a, b, e, f, c, d, g, h, i, j, k, l], is_dual)
    } else {
        select_1187([a, b, c, d, e, h, g, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1174([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1175([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    } else {
        select_1180([a, c, e, f, b, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1173([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1174([a, b, d, c, e, f, i, h, j, k, g, l], is_dual)
    } else {
        select_1174([a, c, d, b, e, f, i, g, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1172([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1173([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_1173([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1171([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1172([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_1172([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1056([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1057([a, b, c, d, e, g, h, i, j, k, f, l], is_dual)
    } else {
        select_1171([a, b, c, d, e, g, f, h, i, k, j, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_373([a, f, c, b, e, h, g, i], is_dual)
    } else {
        select_388([a, b, d, c, g, f, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1202([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_115([a, c, d, i, h, k], is_dual)
    } else {
        select_24([b, e, f, j, g, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1201([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1202([b, c, e, f, a, d, g, h, i, j, k, l], is_dual)
    } else {
        select_249([b, i, f, e, a, h, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_115([e, c, a, d, f, g], is_dual)
    } else {
        select_115([a, b, d, f, e, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1200([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1201([b, c, d, f, e, a, g, h, i, j, k, l], is_dual)
    } else {
        select_1203([b, j, d, f, a, g, l, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1198([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_1199([b, a, d, i, f, c, h, k, j], is_dual)
    } else {
        select_1200([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1206([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_292([b, c, f, d, e, g, h, i, j], is_dual)
    } else {
        select_39([a, h, b, d, e, g, f, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1205([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_373([g, h, b, e, f, i, k, j], is_dual)
    } else {
        select_1206([a, c, d, b, i, f, g, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1209([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_115([a, c, d, e, g, h], is_dual)
    } else {
        select_1163([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_1208([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1209([b, e, d, f, j, g, i, m, k], is_dual)
    } else {
        select_249([a, e, k, c, g, h, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1210([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_558([a, d, c, j, f, g, l, i], is_dual)
    } else {
        select_249([b, d, i, e, f, h, k, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1207([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1208([c, b, e, a, d, f, h, i, g, k, j, l, m], is_dual)
    } else {
        select_1210([a, c, b, d, e, h, j, i, g, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1204([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1205([b, d, e, h, f, g, i, a, j, k, l, m], is_dual)
    } else {
        select_1207([b, c, d, e, f, a, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1197([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1198([a, b, i, d, c, f, h, j, k, l, g, m], is_dual)
    } else {
        select_1204([b, a, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1215([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_81([b, d, e, h, g], is_dual)
    } else {
        select_217([a, b, c, g, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1214([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_226([c, h, d, e, b, g, i], is_dual)
    } else {
        select_1215([a, b, g, d, e, f, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1217([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_159([b, c, d, g, f, h], is_dual)
    } else {
        select_132([a, d, c, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_157([b, c, d, e, h, g, i], is_dual)
    } else {
        select_1217([a, b, i, c, f, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1213([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1214([b, c, a, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_1216([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_917([a, b, c, j, d, g, h, i, k, l], is_dual)
    } else {
        select_965([a, b, d, e, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1212([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1213([b, c, d, e, a, h, g, i, k, j, l], is_dual)
    } else {
        select_1218([a, b, c, f, d, e, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1221([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_204([c, d, e, b, f, g, h], is_dual)
    } else {
        select_389([a, f, b, g, e, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1220([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1183([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1221([b, c, j, d, g, a, h, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1219([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1213([a, b, c, d, e, h, g, i, k, j, l], is_dual)
    } else {
        select_1220([a, b, e, f, c, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1211([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1212([a, b, c, e, f, d, h, i, j, k, g, l], is_dual)
    } else {
        select_1219([b, c, e, f, d, a, h, g, j, k, i, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1196([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1197([a, c, e, b, f, d, g, i, j, h, m, k, l], is_dual)
    } else {
        select_1211([a, b, e, d, c, j, g, i, h, m, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1195([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1196([c, d, f, e, a, b, i, k, g, h, j, m, l], is_dual)
    } else {
        select_1196([c, e, f, d, a, b, i, j, g, h, k, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1194([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1195([f, g, a, c, d, e, j, k, b, h, i, m, l], is_dual)
    } else {
        select_1195([f, g, b, c, d, e, j, k, a, h, i, m, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1193([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1194([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_1194([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1192([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1193([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_1193([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1055([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_1056([a, b, c, d, e, f, h, j, k, l, m, g], is_dual)
    } else {
        select_1192([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_988([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < h) || (is_dual && m > h) {
        select_989([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1055([a, b, c, d, e, f, g, i, l, m, k, h, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_760([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < i) || (is_dual && n > i) {
        select_761([a, b, c, d, e, f, g, h, j, i, k, l, m, n, o], is_dual)
    } else {
        select_988([a, b, c, d, e, f, g, h, j, m, l, n, i, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_361([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_362([a, b, c, d, e, f, g, h, i, k, j, l, m, o, n], is_dual)
    } else {
        select_760([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    }
}
/// n = 10, i = 3
fn select_1234([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_219([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_219([c, b, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1233([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1234([e, f, g, i, c, d, h, j, k, l], is_dual)
    } else {
        select_234([e, f, c, d, a, b, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1232([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1233([d, i, f, g, a, b, c, h, k, j, m, l], is_dual)
    } else {
        select_150([c, d, e, a, b, j, h, i, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1236([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_215([a, b, c, i, e, f, g, h, k, j, l, m], is_dual)
    } else {
        select_755([b, c, d, g, j, h, i, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1235([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1236([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_1236([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1231([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1232([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    } else {
        select_1235([b, c, a, e, f, g, h, d, i, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1240([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_154([e, f, g, c, d, i, h, j], is_dual)
    } else {
        select_81([e, a, b, j, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1241([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_59([b, f, a, d, e, g, h], is_dual)
    } else {
        select_115([a, c, b, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1239([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1240([a, d, e, f, b, c, g, h, i, j], is_dual)
    } else {
        select_1241([b, c, g, e, f, a, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_372([a, c, b, e, i, d, h, g, k, j], is_dual)
    } else {
        select_1239([a, b, c, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 1
fn select_1243([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_774([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_774([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_1243([a, c, e, b, i, f, g, h, j, k, l], is_dual)
    } else {
        select_965([a, b, d, c, j, l, i, m], is_dual)
    }
}
/// n = 13, i = 2
fn select_1237([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < k) || (is_dual && b > k) {
        select_1238([a, c, d, e, k, f, b, i, j, l, m], is_dual)
    } else {
        select_1242([a, c, d, e, f, b, g, h, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1230([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1231([a, b, c, d, f, h, e, i, j, l, m, k, n], is_dual)
    } else {
        select_1237([a, d, f, e, h, g, b, c, j, i, k, m, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1247([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_308([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_308([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1251([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([c, d, e, f], is_dual)
    } else {
        select_46([h, i, a, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1250([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_35([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_1251([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1249([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_309([c, a, d, e, f, g, b, h, i], is_dual)
    } else {
        select_1250([c, b, d, e, f, g, a, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1248([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_308([c, a, b, e, f, g, d, h, i], is_dual)
    } else {
        select_1249([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1246([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1247([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_1248([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1254([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_18([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_289([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1253([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1254([a, c, h, e, f, g, b, i, j], is_dual)
    } else {
        select_390([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1252([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1253([a, d, e, g, b, c, f, h, i, j], is_dual)
    } else {
        select_1248([a, d, b, c, e, f, h, j, k, i], is_dual)
    }
}
/// n = 13, i = 3
fn select_1245([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1246([a, d, b, c, e, g, k, m, j, l], is_dual)
    } else {
        select_1252([a, b, c, d, f, j, h, i, l, k, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_1258([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_151([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1250([b, h, d, e, f, g, a, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1257([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1258([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_150([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1256([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1253([a, c, d, g, b, f, h, i, j, k], is_dual)
    } else {
        select_1257([a, b, c, e, d, f, i, h, k, j, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1259([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_846([a, c, d, e, b, g, h, j, i], is_dual)
    } else {
        select_1257([a, b, c, f, d, e, g, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1255([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1256([a, b, c, f, h, k, g, j, i, m, l, n], is_dual)
    } else {
        select_1259([a, c, b, d, e, h, l, j, k, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1244([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1245([a, b, c, e, d, f, i, h, j, l, m, k, n], is_dual)
    } else {
        select_1255([a, d, e, b, c, f, h, g, j, i, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1229([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1230([a, c, d, e, f, g, b, h, i, k, l, j, m, n], is_dual)
    } else {
        select_1244([a, c, d, b, e, g, f, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1228([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1229([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_1229([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1227([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1228([a, b, c, d, e, f, h, g, i, j, k, l, n, m], is_dual)
    } else {
        select_1228([a, b, c, d, e, g, i, f, h, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1226([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1227([a, b, c, f, g, d, h, e, i, k, l, m, j, n], is_dual)
    } else {
        select_1227([a, b, c, f, g, e, h, d, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1225([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1226([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    } else {
        select_1226([a, b, c, e, f, g, h, i, j, d, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1224([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1225([f, g, h, i, j, c, a, b, d, e, k, m, n, l], is_dual)
    } else {
        select_1225([f, g, h, i, j, c, a, b, e, d, k, l, n, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1223([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1224([j, k, a, h, i, b, c, d, e, f, g, l, m, n], is_dual)
    } else {
        select_1224([j, k, g, h, i, b, c, d, e, f, a, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1222([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1223([a, b, c, d, e, f, g, h, j, k, l, i, m, n], is_dual)
    } else {
        select_1223([a, b, c, d, e, f, g, i, j, k, l, h, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_361([a, b, c, d, e, f, g, h, i, j, k, n, l, o, m], is_dual)
    } else {
        select_1222([a, b, c, d, e, f, g, h, i, n, j, m, o, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_359([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_360([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    } else {
        select_360([a, b, c, d, e, f, g, h, i, k, l, j, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < l) || (is_dual && o > l) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_359([a, b, c, d, e, f, g, h, i, j, k, m, n, o, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_90([f, a, d, e, i, h, g, j, k], is_dual)
    } else {
        select_90([f, a, b, c, j, h, g, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1273([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_206([a, f, b, c, d, e, g, i, h, j, k, l], is_dual)
    } else {
        select_1274([h, b, c, d, e, g, f, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1272([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1273([e, b, i, f, g, d, a, j, h, l, k, m], is_dual)
    } else {
        select_463([a, b, c, d, j, k, h, i, m, n, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1271([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1272([a, b, c, e, g, d, j, h, i, k, m, l, n, o], is_dual)
    } else {
        select_1272([a, b, c, d, f, e, k, h, i, j, n, l, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1270([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1271([a, b, c, d, f, e, g, h, i, k, l, m, j, n, o], is_dual)
    } else {
        select_1271([a, b, c, e, f, d, g, h, i, j, l, m, k, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1269([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1270([e, f, g, c, d, b, a, k, m, i, j, h, l, n, o], is_dual)
    } else {
        select_1270([e, g, f, c, d, b, a, k, l, i, j, h, m, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_1280([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_249([b, c, e, a, f, g, i, h], is_dual)
    } else {
        select_249([a, c, d, b, f, h, i, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_1282([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_275([b, c, d, e, f, g], is_dual)
    } else {
        select_576([a, b, c, e, f, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_1283([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([b, c, d, e, g, h, i], is_dual)
    } else {
        select_1163([a, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1281([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1282([a, b, c, k, g, j, m], is_dual)
    } else {
        select_1283([b, d, e, f, j, g, h, i, l, k], is_dual)
    }
}
/// n = 14, i = 3
fn select_1279([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_1280([a, b, c, l, f, h, g, k, n], is_dual)
    } else {
        select_1281([b, c, f, d, e, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1286([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_19([d, e, c, h, f, g, i], is_dual)
    } else {
        select_36([a, b, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1287([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_115([b, a, d, e, f, h], is_dual)
    } else {
        select_255([a, b, c, g, h], is_dual)
    }
}
/// n = 13, i = 3
fn select_1285([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1286([a, c, j, e, f, g, h, i, l, k, m], is_dual)
    } else {
        select_1287([a, b, c, d, k, i, j, m], is_dual)
    }
}
/// n = 13, i = 3
fn select_1288([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_204([e, f, i, j, g, h, l], is_dual)
    } else {
        select_207([b, c, d, g, h, a, j, k, i, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1284([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1285([a, b, h, d, e, f, i, j, g, k, l, m, n], is_dual)
    } else {
        select_1288([b, d, c, g, e, f, i, j, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1278([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1279([c, d, b, f, g, e, h, a, i, j, k, l, m, n], is_dual)
    } else {
        select_1284([c, d, a, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_1292([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_81([a, b, c, e, g], is_dual)
    } else {
        select_115([b, d, a, c, e, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_1291([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_154([a, b, d, e, f, i, h, j], is_dual)
    } else {
        select_1292([b, a, d, c, g, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1294([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_115([b, d, a, e, f, g], is_dual)
    } else {
        select_115([a, c, b, e, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1293([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_154([a, b, e, d, f, h, g, i], is_dual)
    } else {
        select_1294([a, b, c, g, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1290([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1291([b, c, d, f, a, e, g, h, i, j], is_dual)
    } else {
        select_1293([c, b, h, d, f, a, g, j, i], is_dual)
    }
}
/// n = 15, i = 3
fn select_1289([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_279([b, a, g, e, f, l, k, i, j, n, m], is_dual)
    } else {
        select_1290([a, b, c, g, d, m, k, h, o, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1277([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1278([a, c, d, b, i, f, g, h, j, k, m, n, l, o], is_dual)
    } else {
        select_1289([b, c, a, d, f, g, e, h, j, k, i, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1276([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1277([a, c, d, b, e, f, g, i, j, k, l, h, m, n, o], is_dual)
    } else {
        select_1277([b, c, d, a, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    }
}
/// n = 11, i = 2
fn select_1298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_59([a, g, i, c, d, k, h], is_dual)
    } else {
        select_296([b, a, h, e, f, g, j, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1300([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_85([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_20([c, j, d, e, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1299([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_1300([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_45([k, b, c, a, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_1297([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1298([e, f, a, b, c, d, g, h, i, j, k], is_dual)
    } else {
        select_1299([e, a, b, g, c, d, f, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1296([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_15([a, b, c, d, i, f, g, h, k, j, l, m], is_dual)
    } else {
        select_1297([e, h, f, g, d, b, c, i, j, k, l], is_dual)
    }
}
/// n = 15, i = 3
fn select_1295([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < h) || (is_dual && n > h) {
        select_200([a, f, g, e, d, b, c, h, k, l, j, i, m, n], is_dual)
    } else {
        select_1296([b, a, c, d, e, k, l, i, j, n, m, h, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1275([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_1276([e, f, g, d, c, a, b, k, l, j, h, i, m, n, o], is_dual)
    } else {
        select_1295([e, f, d, c, g, a, b, k, l, m, h, i, j, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1269([c, d, a, b, e, f, g, j, h, i, k, l, m, n, o], is_dual)
    } else {
        select_1275([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1267([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1268([g, h, f, e, a, c, d, l, m, k, b, i, j, n, o], is_dual)
    } else {
        select_1268([g, h, f, e, b, c, d, l, m, k, a, i, j, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1266([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1267([a, b, c, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_1267([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1265([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1266([f, g, h, i, c, d, e, a, b, l, n, j, k, m, o], is_dual)
    } else {
        select_1266([f, g, h, i, c, e, d, a, b, l, m, j, k, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1264([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1265([i, j, a, g, h, b, c, d, e, m, n, f, k, l, o], is_dual)
    } else {
        select_1265([i, j, f, g, h, b, c, d, e, m, n, a, k, l, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1264([a, b, c, d, e, f, g, i, j, k, h, l, m, n, o], is_dual)
    } else {
        select_1264([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1262([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1263([a, b, c, d, e, f, g, h, i, k, l, j, m, n, o], is_dual)
    } else {
        select_1263([a, b, c, d, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_1314([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_115([a, b, f, g, e, i], is_dual)
    } else {
        select_19([c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1313([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1046([a, b, f, d, h, g, e, i], is_dual)
    } else {
        select_1314([b, d, a, c, g, e, f, i, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1315([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_167([a, b, c, h, e, f, g, i], is_dual)
    } else {
        select_167([a, b, d, g, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1312([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1313([a, b, d, c, f, h, g, k, j], is_dual)
    } else {
        select_1315([a, b, d, e, f, j, h, i, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_1311([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_754([a, b, c, d, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_1312([g, b, c, e, f, a, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 1
fn select_1318([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_18([a, b, c, d, e, g, h, i], is_dual)
    } else {
        select_173([a, c, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 1
fn select_1317([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_782([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1318([b, i, d, e, f, g, h, a, j], is_dual)
    }
}
/// n = 14, i = 2
fn select_1316([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_279([a, b, c, d, e, m, i, j, k, l, n], is_dual)
    } else {
        select_1317([a, d, e, f, g, h, l, j, k, m], is_dual)
    }
}
/// n = 14, i = 2
fn select_1310([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < l) || (is_dual && b > l) {
        select_1311([a, c, d, l, e, f, b, i, j, k, m, n], is_dual)
    } else {
        select_1316([a, c, d, e, f, b, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1309([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < n) || (is_dual && l > n) {
        select_1231([a, b, c, d, e, f, j, k, i, n, m, l, o], is_dual)
    } else {
        select_1310([a, d, e, f, g, h, b, c, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1308([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_1244([a, c, d, b, e, f, j, h, m, k, l, i, n, o], is_dual)
    } else {
        select_1309([a, c, d, e, f, h, b, g, k, i, j, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1307([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_1229([a, b, c, d, e, g, f, k, i, j, m, l, o, n], is_dual)
    } else {
        select_1308([a, b, c, d, f, e, h, g, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1306([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1307([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    } else {
        select_1307([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1305([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1306([a, b, c, f, g, d, h, e, i, k, l, m, n, j, o], is_dual)
    } else {
        select_1306([a, b, c, f, g, e, h, d, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1304([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1305([a, b, c, d, f, g, h, i, j, e, k, l, m, n, o], is_dual)
    } else {
        select_1305([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1303([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1304([a, b, c, d, e, f, h, i, j, k, g, l, m, n, o], is_dual)
    } else {
        select_1304([a, b, c, d, e, g, h, i, j, k, f, l, m, n, o], is_dual)
    }
}
/// n = 6, i = 2
fn select_1331([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_511([a, e, f], is_dual)
    } else {
        select_45([a, b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1330([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_217([a, b, d, f, e, g, i, h], is_dual)
    } else {
        select_1331([a, c, g, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1332([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_191([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_37([b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1329([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1330([c, d, b, e, f, a, g, i, h], is_dual)
    } else {
        select_1332([c, d, a, e, f, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1328([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1329([b, c, a, d, f, g, i, h, j], is_dual)
    } else {
        select_1329([a, c, b, d, e, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1327([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1328([a, b, c, e, f, g, h, j, i, k], is_dual)
    } else {
        select_556([a, c, b, d, i, h, k, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1327([c, a, d, b, e, f, h, i, j, g, k], is_dual)
    } else {
        select_1327([c, b, d, a, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1325([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1326([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_1326([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1324([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_551([a, b, c, d, e, i, g, h, j, l, k, m], is_dual)
    } else {
        select_1325([a, b, c, e, d, f, k, h, m, i, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1323([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1324([f, g, c, d, b, e, a, i, k, h, l, j, m], is_dual)
    } else {
        select_1324([f, g, c, e, b, d, a, i, j, h, l, k, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_1340([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_576([f, a, b, d, e, g], is_dual)
    } else {
        select_576([e, a, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1339([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_289([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_1340([b, c, h, f, g, a, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_273([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_1339([b, c, h, e, f, g, a, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1337([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_822([d, a, e, f, g, b, c, h, i, j, k], is_dual)
    } else {
        select_1338([b, c, i, d, f, g, a, h, j, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1336([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_285([a, b, c, d, e, i, j, h, l, k, m], is_dual)
    } else {
        select_1337([a, b, d, f, g, c, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_1343([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_203([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_203([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1342([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_1253([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_1343([a, b, d, i, e, c, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_1341([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1337([a, b, c, f, g, d, h, i, j, k, l], is_dual)
    } else {
        select_1342([a, b, c, d, e, i, j, h, l, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1335([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1336([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_1341([a, b, d, e, c, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_1348([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_212([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_91([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1350([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([b, c, d, e, i, g, h], is_dual)
    } else {
        select_1100([a, b, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1349([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_210([c, a, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_1350([b, c, h, e, f, g, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_1347([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1348([a, h, b, d, e, f, i, g, j, k], is_dual)
    } else {
        select_1349([b, a, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1346([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_16([a, b, h, d, c, j, g, i, l, k, m], is_dual)
    } else {
        select_1347([a, c, e, f, b, i, g, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 2
fn select_1352([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_467([c, d, g, e, f, a, h, i, j], is_dual)
    } else {
        select_414([a, b, i, c, e, f, h, g, j, k], is_dual)
    }
}
/// n = 13, i = 2
fn select_1351([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1352([h, a, e, f, b, i, g, j, k, l, m], is_dual)
    } else {
        select_1347([a, b, c, d, j, k, g, h, i, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1345([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1346([a, b, c, j, e, f, g, i, l, k, h, m, n], is_dual)
    } else {
        select_1351([a, c, e, f, b, d, g, i, k, h, j, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1344([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1345([a, c, d, e, f, g, b, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1345([b, c, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1334([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1335([a, b, c, d, e, f, k, h, i, j, l, n, m], is_dual)
    } else {
        select_1344([a, b, c, d, g, f, e, h, j, k, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1333([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_1334([a, b, c, e, d, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_1324([a, b, c, d, j, e, g, h, l, k, m, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1322([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_1323([g, k, d, e, j, a, b, l, h, i, c, m, n], is_dual)
    } else {
        select_1333([a, b, d, e, c, f, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1321([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1322([a, b, c, d, f, g, h, e, i, j, k, l, m, n], is_dual)
    } else {
        select_1322([a, b, c, e, f, g, h, d, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1320([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1321([a, b, c, d, e, f, h, i, g, j, k, l, m, n], is_dual)
    } else {
        select_1321([a, b, c, d, e, g, h, i, f, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_1360([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_1352([a, b, c, d, k, l, h, i, j, n, m], is_dual)
    } else {
        select_205([e, f, g, i, j, b, a, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 2
fn select_1359([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1360([a, b, d, e, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_1360([a, c, d, e, f, g, h, b, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1358([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_1359([a, b, d, g, h, c, e, f, j, k, i, l, n, m], is_dual)
    } else {
        select_1336([a, b, c, d, l, g, h, m, j, k, i, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1357([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1358([a, b, c, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_1358([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_1363([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_373([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_442([a, b, f, g, h, c, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1362([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1363([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_795([a, b, c, d, g, h, f, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1364([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_1253([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_442([a, b, h, i, j, c, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1361([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_1362([a, b, c, i, e, d, h, j, k], is_dual)
    } else {
        select_1364([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 15, i = 3
fn select_1356([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < e) || (is_dual && o > e) {
        select_1357([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1361([a, b, c, d, l, j, k, m, n, o, e], is_dual)
    }
}
/// n = 15, i = 3
fn select_1355([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1356([a, b, c, d, e, h, i, f, g, j, k, m, n, l, o], is_dual)
    } else {
        select_959([a, b, c, d, e, f, k, h, i, l, m, j, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1354([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1355([a, b, c, d, e, f, h, i, j, g, k, l, m, n, o], is_dual)
    } else {
        select_1355([a, b, c, d, e, g, h, i, j, f, k, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_1375([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([f, b, a, e, h, i], is_dual)
    } else {
        select_36([g, c, d, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1374([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_81([g, d, e, i, h], is_dual)
    } else {
        select_1375([a, h, b, c, f, g, j, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1377([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_144([e, c, d, f, i, h, k], is_dual)
    } else {
        select_144([e, a, b, f, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1376([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1377([a, b, d, e, c, f, g, h, j, k, i], is_dual)
    } else {
        select_747([a, c, b, f, i, k, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1373([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1374([c, b, f, d, e, h, g, i, j, l, k], is_dual)
    } else {
        select_1376([a, g, c, d, e, h, f, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1372([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1373([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_140([a, g, h, d, e, j, i, f, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1371([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1372([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_1372([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1381([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_44([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_506([f, a, c, d, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1380([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1381([a, h, d, e, g, f, i, j, k], is_dual)
    } else {
        select_151([a, d, e, b, c, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1382([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_436([e, h, c, d, g, f, i, j], is_dual)
    } else {
        select_151([e, c, d, a, b, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1379([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1380([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_1382([a, b, c, d, g, f, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1378([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_845([a, b, d, e, c, i, j, h], is_dual)
    } else {
        select_1379([d, e, f, g, c, a, b, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1370([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1371([a, b, d, c, e, f, i, h, j, g, l, k], is_dual)
    } else {
        select_1378([a, b, i, c, g, e, f, h, j, l, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1386([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_48([g, h, e, f, c, a], !is_dual)
    } else {
        select_48([e, f, g, h, d, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1387([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1144([a, b, c, d, g, e, h, f], is_dual)
    } else {
        select_1144([a, b, c, d, g, f, h, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1385([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1386([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_1387([a, b, c, e, g, h, d, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_1388([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_59([a, c, e, d, f, g, h], is_dual)
    } else {
        select_626([a, b, c, d, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1384([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_1385([a, c, g, f, i, h, b, j], is_dual)
    } else {
        select_1388([a, c, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_1392([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([f, g, e, c, d, a], !is_dual)
    } else {
        select_32([e, b, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_1391([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_142([b, c, d, e, a, f, g], is_dual)
    } else {
        select_1392([c, a, e, b, f, d, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_1390([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_876([d, a, g, e, f, b], !is_dual)
    } else {
        select_1391([d, e, g, a, f, b, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1389([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1390([a, c, h, b, i, g, j], is_dual)
    } else {
        select_854([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1383([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1384([a, b, c, j, d, g, h, k, i, l], is_dual)
    } else {
        select_1389([a, b, c, e, f, i, g, j, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1369([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_1370([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_1383([a, b, d, c, f, g, h, e, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1393([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_999([a, b, c, e, g, h, d, f, i, j, k, l, m], is_dual)
    } else {
        select_858([a, b, c, d, e, f, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 3
fn select_1368([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1369([a, b, c, d, e, f, j, i, l, m, k, n], is_dual)
    } else {
        select_1393([a, b, c, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_1399([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_309([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_309([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1398([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_1399([a, b, h, e, f, g, c, i, j], is_dual)
    } else {
        select_234([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1397([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_67([a, d, f, g, b, c, h, i, j, k], is_dual)
    } else {
        select_1398([a, b, c, e, d, i, j, h, l, m], is_dual)
    }
}
/// n = 11, i = 2
fn select_1402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_114([b, c, d, a, e, f, g, h, j, i, k], is_dual)
    } else {
        select_249([b, i, d, c, a, h, k, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1403([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_157([a, c, b, e, g, f, i], is_dual)
    } else {
        select_59([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_1401([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_1402([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_1403([a, b, i, d, e, c, h, k, j], is_dual)
    }
}
/// n = 13, i = 2
fn select_1400([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_64([a, b, c, h, f, g, i, j, l, m], is_dual)
    } else {
        select_1401([a, c, b, g, h, d, e, j, k, m, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1396([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1397([a, b, c, e, d, g, h, l, i, j, k, m, n], is_dual)
    } else {
        select_1400([a, d, e, b, c, g, h, f, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_1407([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_480([a, b, c, d, f, e, g], is_dual)
    } else {
        select_876([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1406([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_1407([a, h, i, b, c, d, g], !is_dual)
    } else {
        select_628([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1405([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1406([a, b, c, d, e, g, i, h, j], is_dual)
    } else {
        select_622([a, c, d, f, b, e, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1408([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_214([c, d, e, f, g, b, a, h, i, j, k, l], is_dual)
    } else {
        select_733([a, c, d, e, b, j, h, k, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1404([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_1405([a, d, b, c, j, e, h, k, l, i], is_dual)
    } else {
        select_1408([a, d, e, b, c, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1395([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_1396([a, c, d, e, f, g, b, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1404([a, c, d, b, e, f, j, l, k, m, i, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1394([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1395([a, b, d, e, f, g, h, i, c, j, k, l, m, n], is_dual)
    } else {
        select_1395([a, c, d, e, f, g, h, i, b, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1367([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_1368([a, b, c, e, f, g, h, i, j, k, d, l, m, n], is_dual)
    } else {
        select_1394([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_1366([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1367([a, b, c, d, e, g, h, i, j, f, k, l, m, n], is_dual)
    } else {
        select_1367([a, b, c, d, f, g, h, i, j, e, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1416([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_340([a, e, i, b, f, h, g], !is_dual)
    } else {
        select_532([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1415([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1416([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_1416([a, b, d, c, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1414([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1415([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_1415([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1417([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_527([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_527([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1413([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_1414([a, b, c, e, i, h, d, k, j], is_dual)
    } else {
        select_1417([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1419([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_622([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_622([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1418([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_578([a, b, d, e, f, h, c, i, j], is_dual)
    } else {
        select_1419([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1412([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1413([a, b, c, d, g, e, f, h, j, k, i], is_dual)
    } else {
        select_1418([a, b, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1422([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_824([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_311([a, b, j, g, c, i], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1421([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_832([a, b, c, i, j, d, g], is_dual)
    } else {
        select_1422([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1424([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_253([a, f, c, d, e, g, h, i], is_dual)
    } else {
        select_620([a, b, h, e, f, i, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1423([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1424([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_1424([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1420([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_1421([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1423([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1411([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_1412([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1420([a, b, c, h, e, f, i, j, k, d], is_dual)
    }
}
/// n = 11, i = 3
fn select_1431([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_70([b, c, d, e, g, h, i], is_dual)
    } else {
        select_1375([a, i, b, c, f, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1430([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_328([b, f, d, e, a, g, h, i, j], is_dual)
    } else {
        select_1431([b, a, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1429([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_193([f, h, b, c, a, j, i], is_dual)
    } else {
        select_1430([a, d, e, b, c, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1428([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1429([b, a, c, d, e, g, h, i, f, k, j], is_dual)
    } else {
        select_540([b, d, c, e, g, h, f, i, a, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1427([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1428([a, c, b, e, d, f, g, h, j, i, k], is_dual)
    } else {
        select_321([a, b, c, g, e, i, h, j, f, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1426([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1427([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_1427([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1425([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_470([a, b, c, g, h, i, j, k, d], is_dual)
    } else {
        select_1426([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1410([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1411([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1425([a, b, c, d, g, f, h, i, j, e, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1439([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_70([a, b, c, d, f, g, h], is_dual)
    } else {
        select_264([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1438([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_219([a, b, h, d, e, g, f, i, j, k], is_dual)
    } else {
        select_1439([c, f, d, e, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1440([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1129([a, b, i, c, f, h, g, j, k], is_dual)
    } else {
        select_90([a, c, d, e, g, f, h, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1437([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1438([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_1440([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1436([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1437([a, b, c, e, d, i, g, h, k, j, l], is_dual)
    } else {
        select_202([a, c, d, f, b, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1435([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1436([a, c, e, b, f, d, h, i, g, k, j, l], is_dual)
    } else {
        select_973([a, c, b, d, i, e, g, h, k, l, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1434([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1435([a, b, d, c, e, f, h, i, j, g, k, l], is_dual)
    } else {
        select_1435([a, c, d, b, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1433([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1434([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_1434([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1432([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1433([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_1433([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1409([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_1410([a, b, c, d, e, g, j, k, i, f, l], is_dual)
    } else {
        select_1432([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_1365([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < g) || (is_dual && m > g) {
        select_1366([a, b, c, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_1409([a, b, c, d, e, f, h, k, l, m, g, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1353([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1354([a, b, c, d, e, f, g, j, h, i, k, m, l, n, o], is_dual)
    } else {
        select_1365([a, b, c, d, e, f, g, h, i, k, m, l, o, n], is_dual)
    }
}
/// n = 15, i = 3
fn select_1319([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_1320([a, b, c, d, e, f, g, k, j, m, h, l, n, o], is_dual)
    } else {
        select_1353([a, b, c, d, e, f, g, i, h, j, l, k, n, m, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1302([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_1303([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_1319([a, b, c, d, e, f, g, j, i, k, l, m, h, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1301([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1302([a, b, c, d, e, f, g, h, i, k, l, j, m, n, o], is_dual)
    } else {
        select_1302([a, b, c, d, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1261([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < m) || (is_dual && o > m) {
        select_1262([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1301([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m], is_dual)
    }
}
/// n = 15, i = 3
fn select_1260([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1261([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_1261([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    } else {
        select_1260([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
