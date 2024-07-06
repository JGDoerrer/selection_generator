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
/// n = 4, i = 1
fn select_22([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([a, d], is_dual)
    } else {
        select_23([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_25([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_23([a, b], is_dual)
    } else {
        select_24([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_21([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_22([b, c, e, d], is_dual)
    } else {
        select_25([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_26([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_22([a, b, c, d], is_dual)
    } else {
        select_25([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_21([a, c, b, d, e, f], is_dual)
    } else {
        select_26([a, b, e, f, d], is_dual)
    }
}
/// n = 3, i = 0
fn select_29([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_23([a, b], is_dual)
    } else {
        select_23([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_29([b, c, d], is_dual)
    } else {
        select_25([a, e, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_30([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_22([a, c, b, d], is_dual)
    } else {
        select_22([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_30([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_20([b, d, c, f, g, e], is_dual)
    } else {
        select_27([b, a, e, g, h, d, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_33([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([a, b, e, f], is_dual)
    } else {
        select_22([a, c, d, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_34([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_29([b, c, d], is_dual)
    } else {
        select_23([a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_32([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_33([a, b, e, f, d, g], is_dual)
    } else {
        select_34([b, c, d, f, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_29([b, c, e], is_dual)
    } else {
        select_22([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_36([b, a, c, d, e, f, g], is_dual)
    } else {
        select_30([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_31([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_32([d, b, c, g, f, e, h], is_dual)
    } else {
        select_35([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_19([a, b, f, e, h, g, i, d], is_dual)
    } else {
        select_31([a, b, c, e, d, f, g, i, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_39([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_34([a, c, d, b, e], is_dual)
    } else {
        select_34([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_21([a, d, b, c, e, f], is_dual)
    } else {
        select_21([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_38([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_39([b, c, d, e, f], is_dual)
    } else {
        select_40([a, b, c, f, e, g], is_dual)
    }
}
/// n = 3, i = 1
fn select_43([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_25([a, b, c], is_dual)
    } else {
        select_25([a, c, b], is_dual)
    }
}
/// n = 7, i = 3
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_21([a, c, b, d, e, g], is_dual)
    } else {
        select_43([b, e, f], is_dual)
    }
}
/// n = 4, i = 0
fn select_45([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_29([a, b, c], is_dual)
    } else {
        select_29([b, c, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, e], is_dual)
    } else {
        select_25([a, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_41([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_42([a, b, g, f, e, h, i], is_dual)
    } else {
        select_44([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_38([b, c, d, f, e, g, h], is_dual)
    } else {
        select_41([b, a, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_18([b, c, d, e, f, g, h, a, i], is_dual)
    } else {
        select_37([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_50([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_29([c, d, e], is_dual)
    } else {
        select_29([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_50([a, b, c, d, e, f], is_dual)
    } else {
        select_34([e, c, d, a, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_52([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_22([a, b, c, d], is_dual)
    } else {
        select_25([d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_51([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_52([d, f, e, a, b], !is_dual)
    } else {
        select_52([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_48([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, a, d, e, f], is_dual)
    } else {
        select_51([b, c, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_48([a, c, e, b, h, f, g], is_dual)
    } else {
        select_48([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_56([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_29([a, b, c], is_dual)
    } else {
        select_24([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_56([a, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_54([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_20([b, g, c, a, f, h], is_dual)
    } else {
        select_55([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_53([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_54([a, b, d, c, e, f, g, i], is_dual)
    } else {
        select_54([a, c, e, b, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_47([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_53([a, b, d, g, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_16([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_17([a, c, h, d, b, f, g, i, j], is_dual)
    } else {
        select_46([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_15([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_16([a, b, e, f, c, g, h, d, i, j], is_dual)
    } else {
        select_16([a, b, f, e, d, h, g, c, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_14([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_15([a, b, c, e, d, f, h, i, g, j], is_dual)
    } else {
        select_15([a, b, d, e, c, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_30([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_64([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_45([c, d, e, g], is_dual)
    } else {
        select_34([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_63([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_64([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_66([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_29([a, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_65([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_66([b, f, c, d, e, g, h], is_dual)
    } else {
        select_55([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_62([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_65([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_69([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_52([b, c, a, d, e], is_dual)
    } else {
        select_22([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_68([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_39([a, c, b, d, e], is_dual)
    } else {
        select_69([a, c, e, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_71([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_21([e, a, c, f, d, g], is_dual)
    } else {
        select_21([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_70([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_71([b, c, d, a, e, f, h], is_dual)
    } else {
        select_71([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_67([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_68([b, e, c, a, g, f], is_dual)
    } else {
        select_70([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_61([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_67([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_75([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_45([a, b, c, d], is_dual)
    } else {
        select_24([e], is_dual)
    }
}
/// n = 6, i = 1
fn select_74([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_75([a, c, d, e, f], is_dual)
    } else {
        select_39([a, b, c, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_75([a, b, c, d, f], is_dual)
    } else {
        select_39([a, b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_73([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_74([a, b, d, e, f, g], is_dual)
    } else {
        select_76([a, b, c, g, f, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_79([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_45([d, e, f, g], is_dual)
    } else {
        select_45([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_66([a, h, d, e, f, g, i], is_dual)
    } else {
        select_79([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_78([a, c, d, e, f, g, b, h, i], is_dual)
    } else {
        select_78([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_73([f, e, h, c, d, g, i, j], is_dual)
    } else {
        select_77([e, f, c, d, a, b, g, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_59([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_60([a, c, b, d, e, i, h, k, l, j], is_dual)
    } else {
        select_72([b, h, f, g, e, a, c, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_58([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_59([a, b, d, c, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_59([a, c, d, b, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_58([a, b, d, e, f, g, h, c, i, j, k, l], is_dual)
    } else {
        select_58([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < k) || (is_dual && d > k) {
        select_14([a, b, c, d, e, f, i, j, l, m], is_dual)
    } else {
        select_57([a, b, j, c, e, f, g, h, i, k, l, m], is_dual)
    }
}
/// n = 10, i = 1
fn select_86([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_79([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_79([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_86([e, f, g, a, b, c, d, h, i, j], is_dual)
    } else {
        select_78([h, c, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_50([a, b, c, d, e, f], is_dual)
    } else {
        select_30([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_89([a, c, b, h, g, f, i], is_dual)
    } else {
        select_55([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_50([b, c, d, e, g, f], is_dual)
    } else {
        select_56([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_90([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_91([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_66([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_88([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_90([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 2
fn select_84([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_85([g, a, b, c, d, e, i, h, j, k], is_dual)
    } else {
        select_87([g, f, k, a, b, c, h, i, j, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_95([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, e], is_dual)
    } else {
        select_23([a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_95([b, c, d, e, f, g], is_dual)
    } else {
        select_20([a, b, g, f, e, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_96([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_79([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_55([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_94([e, h, c, d, g, f, i, j], is_dual)
    } else {
        select_96([e, c, d, a, b, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_92([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_93([c, i, e, f, b, a, g, h, j, k], is_dual)
    } else {
        select_93([d, h, e, f, b, a, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 2
fn select_83([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_84([d, c, e, f, g, b, a, i, h, j, k, l], is_dual)
    } else {
        select_92([a, b, d, j, f, g, h, i, c, k, l], is_dual)
    }
}
/// n = 5, i = 0
fn select_101([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([a, b, c, d], is_dual)
    } else {
        select_45([a, b, c, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_101([c, d, e, f, g], is_dual)
    } else {
        select_69([a, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_64([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_64([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_99([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_100([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_102([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_104([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_49([a, b, c, j, h, k], is_dual)
    } else {
        select_101([d, e, f, g, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_106([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_21([a, b, c, e, d, f], is_dual)
    } else {
        select_26([d, c, a, e, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_105([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_101([c, d, e, f, h], is_dual)
    } else {
        select_106([a, i, b, j, g, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_103([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_104([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_105([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_99([a, i, d, e, f, g, h, b, j, l, k], is_dual)
    } else {
        select_103([a, c, b, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_110([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_33([d, f, g, a, e, b], !is_dual)
    } else {
        select_52([e, f, d, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_95([b, c, d, e, f, g], is_dual)
    } else {
        select_110([e, h, i, a, f, g, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_112([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_30([a, e, g, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_111([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_112([a, b, f, d, h, e, g], is_dual)
    } else {
        select_112([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_108([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_109([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_111([a, c, g, e, f, b, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_113([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_38([b, a, c, d, e, f, g], is_dual)
    } else {
        select_28([b, c, f, e, g, a], is_dual)
    }
}
/// n = 9, i = 3
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_108([a, b, d, c, e, g, f, h, i], is_dual)
    } else {
        select_113([g, a, c, b, e, h, f], is_dual)
    }
}
/// n = 12, i = 3
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_98([a, b, d, c, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_107([a, b, d, j, h, i, c, k, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_82([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_83([c, a, e, f, g, b, i, h, j, k, l, m], is_dual)
    } else {
        select_97([a, c, b, d, e, j, k, h, i, m, n, l], is_dual)
    }
}
/// n = 14, i = 3
fn select_81([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_82([e, f, d, g, c, a, b, j, l, h, i, k, m, n], is_dual)
    } else {
        select_82([e, g, d, f, c, a, b, j, k, h, i, l, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_80([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_81([g, h, f, e, a, c, d, k, l, b, i, j, m, n], is_dual)
    } else {
        select_81([g, h, f, e, b, c, d, k, l, a, i, j, m, n], is_dual)
    }
}
/// n = 14, i = 3
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_13([a, b, c, d, e, f, g, l, i, j, m, k, n], is_dual)
    } else {
        select_80([a, b, c, d, e, g, f, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_62([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_62([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_119([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_120([f, g, a, d, e, i, h, j, k], is_dual)
    } else {
        select_120([f, g, a, b, c, j, h, i, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_123([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_75([e, c, d, f, g], is_dual)
    } else {
        select_75([e, a, b, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_91([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_28([f, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_123([c, f, d, e, a, g, h], is_dual)
    } else {
        select_124([a, b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_121([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_85([f, a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_122([f, j, a, b, c, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_118([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_119([e, a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_121([h, a, b, c, d, f, g, e, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_130([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([a, c, d, g], is_dual)
    } else {
        select_22([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_129([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_21([d, b, c, e, f, h], is_dual)
    } else {
        select_130([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_129([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_27([d, g, h, e, a, f, c], !is_dual)
    }
}
/// n = 7, i = 1
fn select_132([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_50([a, f, c, d, e, g], is_dual)
    } else {
        select_50([b, e, c, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_131([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([a, e, c, f, h], is_dual)
    } else {
        select_132([b, d, a, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_127([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_128([f, b, d, a, e, g, h, i], is_dual)
    } else {
        select_131([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_134([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_55([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_55([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_134([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_78([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_126([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_127([a, i, c, d, b, g, h, j, k], is_dual)
    } else {
        select_133([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_125([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_126([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_126([a, b, d, e, f, g, c, h, i, k, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_117([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_118([c, i, g, h, e, a, b, d, k, j, l], is_dual)
    } else {
        select_125([a, b, c, d, f, e, j, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_116([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_117([a, b, d, c, f, e, g, h, j, k, i, l, n], is_dual)
    } else {
        select_117([a, c, e, b, f, d, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 15, i = 4
fn select_115([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_116([a, b, c, d, e, f, g, l, i, j, k, m, n, o], is_dual)
    } else {
        select_116([a, b, c, d, e, f, h, k, i, j, l, m, n, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_142([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_36([a, c, d, e, g, f, h], is_dual)
    } else {
        select_22([a, b, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_141([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_142([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_142([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_143([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_36([a, b, c, d, e, f, g], is_dual)
    } else {
        select_56([e, b, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_140([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_141([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_143([b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_47([a, b, c, f, e, g, h, i], is_dual)
    } else {
        select_140([a, c, e, d, g, h, f, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_147([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_30([a, b, c, e], is_dual)
    } else {
        select_30([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_146([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_147([a, b, e, d, f], is_dual)
    } else {
        select_25([a, c, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_149([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_36([a, b, c, e, f, g, h], is_dual)
    } else {
        select_29([b, c, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_148([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_149([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_63([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_146([a, b, g, f, c, h], is_dual)
    } else {
        select_148([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_93([c, d, e, f, b, a, g, h, i, j], is_dual)
    } else {
        select_145([a, b, c, d, i, g, j, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_138([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_139([a, c, d, h, e, b, g, i, j, k], is_dual)
    } else {
        select_144([a, c, d, e, b, f, g, i, h, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_153([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_132([a, c, d, e, b, f, g], is_dual)
    } else {
        select_132([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_152([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_74([a, b, d, e, f, h], is_dual)
    } else {
        select_153([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_155([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_95([g, d, e, f, a, h], is_dual)
    } else {
        select_79([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_156([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_55([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_44([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_154([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_155([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_156([a, g, d, e, f, b, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_151([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_152([a, c, f, b, e, g, h, i], is_dual)
    } else {
        select_154([a, b, d, c, e, h, g, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_74([a, b, c, d, f, h], is_dual)
    } else {
        select_153([a, c, e, b, d, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_161([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_43([a, e, f], is_dual)
    } else {
        select_34([a, b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_160([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_161([e, c, d, a, f, g], is_dual)
    } else {
        select_89([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_159([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_124([a, c, d, b, e, g, f, h], is_dual)
    } else {
        select_160([a, f, c, d, b, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_157([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_158([a, b, c, f, e, g, h, j], is_dual)
    } else {
        select_159([a, b, c, g, d, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_150([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_151([a, b, d, f, e, c, h, g, j, i, k], is_dual)
    } else {
        select_157([a, b, c, f, d, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_138([a, c, b, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_150([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_167([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_43([a, d, e], is_dual)
    } else {
        select_22([a, b, c, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_166([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_167([a, e, f, g, h], is_dual)
    } else {
        select_32([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_169([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_30([a, g, c, i], is_dual)
    } else {
        select_130([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_168([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_147([g, a, b, i, j], is_dual)
    } else {
        select_169([a, c, b, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_165([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_166([e, c, d, a, f, g, h, j], is_dual)
    } else {
        select_168([a, b, d, c, f, e, h, g, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_52([h, i, a, b, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_170([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_44([f, c, d, e, a, g, i], is_dual)
    } else {
        select_171([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_164([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_165([a, b, i, c, f, g, j, h, k, l], is_dual)
    } else {
        select_170([a, b, d, e, h, f, i, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_174([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([a, b, c, h, i], is_dual)
    } else {
        select_45([d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_176([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_52([e, f, a, b, c], !is_dual)
    } else {
        select_56([b, c, d, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_175([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_39([c, d, f, g, i], is_dual)
    } else {
        select_176([a, b, e, h, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_174([c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_175([a, b, c, d, i, e, h, j, k, l, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_178([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([a, b, d, e, c, f], is_dual)
    } else {
        select_49([a, c, d, e, b, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_177([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_178([b, c, g, d, h, j], is_dual)
    } else {
        select_96([a, e, f, b, c, d, g, h, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_173([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_177([h, c, d, e, f, g, a, i, j, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_164([b, c, d, f, g, h, i, a, j, k, l, m], is_dual)
    } else {
        select_172([b, c, a, d, e, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_183([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_21([a, b, c, f, e, g], is_dual)
    } else {
        select_21([a, b, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_91([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_183([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_182([b, a, g, d, e, f, i, h, k, l], is_dual)
    } else {
        select_48([a, f, h, c, j, g, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_153([b, g, c, f, i, h, k], is_dual)
    } else {
        select_96([a, d, e, b, f, h, g, i, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_180([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_181([a, b, c, e, f, h, i, j, g, k, m, l], is_dual)
    } else {
        select_184([b, c, a, e, f, d, h, g, j, i, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_130([a, d, b, g, e, f, h], is_dual)
    } else {
        select_130([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_187([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_188([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_71([c, e, d, g, a, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_21([a, c, b, f, g, h], is_dual)
    } else {
        select_30([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_189([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_55([a, c, d, e, f, g, h, j], is_dual)
    } else {
        select_190([a, b, c, h, g, f, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_186([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_187([a, b, h, c, f, g, j, k, l], is_dual)
    } else {
        select_189([a, b, g, d, e, f, i, h, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_186([a, b, c, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_184([b, c, a, e, f, d, h, g, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_179([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_180([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_185([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_162([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_163([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_179([a, c, b, e, f, g, h, d, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_137([a, d, c, e, b, j, f, i, l, m, k], is_dual)
    } else {
        select_162([a, c, b, d, e, i, g, h, k, j, m, l, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_135([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_136([a, b, c, d, f, e, g, h, i, k, j, l, m, n], is_dual)
    } else {
        select_136([a, b, d, c, e, f, g, h, j, k, i, l, n, m], is_dual)
    }
}
/// n = 15, i = 4
fn select_114([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_115([a, c, d, e, f, g, b, h, j, k, i, l, m, n, o], is_dual)
    } else {
        select_135([a, b, c, d, e, f, g, l, j, k, m, i, n, o], is_dual)
    }
}
/// n = 15, i = 4
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_12([a, d, b, f, g, h, c, e, i, k, l, j, m, n], is_dual)
    } else {
        select_114([a, b, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 4
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_11([h, d, e, f, a, b, g, c, l, i, j, k, m, o, n], is_dual)
    } else {
        select_11([h, d, e, g, a, c, f, b, l, i, k, j, m, n, o], is_dual)
    }
}
/// n = 15, i = 4
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_10([a, b, c, d, f, g, e, h, i, j, k, l, n, o, m], is_dual)
    } else {
        select_10([a, b, e, d, f, g, c, h, i, j, m, l, n, o, k], is_dual)
    }
}
/// n = 15, i = 4
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < j) || (is_dual && n > j) {
        select_9([a, c, d, e, f, b, g, h, i, j, k, l, m, o, n], is_dual)
    } else {
        select_9([a, g, d, e, f, b, c, h, i, n, k, l, m, o, j], is_dual)
    }
}
/// n = 15, i = 4
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_8([a, b, c, d, e, f, g, h, i, k, l, m, n, o, j], is_dual)
    } else {
        select_8([b, a, c, d, e, f, g, h, j, k, l, m, n, o, i], is_dual)
    }
}
/// n = 15, i = 4
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_7([a, b, c, d, e, f, g, h, j, k, l, m, n, o, i], is_dual)
    } else {
        select_7([a, b, c, d, e, f, i, h, j, k, l, m, n, o, g], is_dual)
    }
}
/// n = 15, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_6([a, b, c, d, e, f, g, h, i, k, l, m, n, o, j], is_dual)
    } else {
        select_6([a, b, c, d, e, j, g, h, i, k, l, m, n, o, f], is_dual)
    }
}
/// n = 15, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_5([a, b, c, d, e, f, g, h, i, j, l, m, n, o, k], is_dual)
    } else {
        select_5([a, b, c, d, k, f, g, h, i, j, l, m, n, o, e], is_dual)
    }
}
/// n = 15, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < d) || (is_dual && l > d) {
        select_4([a, b, c, d, e, f, g, h, i, j, k, m, n, o, l], is_dual)
    } else {
        select_4([a, b, c, l, e, f, g, h, i, j, k, m, n, o, d], is_dual)
    }
}
/// n = 15, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_3([a, l, m, b, c, d, e, f, g, h, i, j, k, n, o], is_dual)
    } else {
        select_3([k, l, m, b, c, d, e, f, g, h, i, j, a, n, o], is_dual)
    }
}
/// n = 15, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o], is_dual)
    }
}
/// n = 15, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
