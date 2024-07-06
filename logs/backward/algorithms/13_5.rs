/// n = 1, i = 0
fn select_22([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_21([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_22([a], is_dual)
    } else {
        select_22([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_20([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_21([a, b], is_dual)
    } else {
        select_21([b, c], is_dual)
    }
}
/// n = 4, i = 0
fn select_19([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_20([a, b, c], is_dual)
    } else {
        select_20([b, c, d], is_dual)
    }
}
/// n = 5, i = 0
fn select_18([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_19([a, b, c, d], is_dual)
    } else {
        select_19([a, b, c, e], is_dual)
    }
}
/// n = 4, i = 1
fn select_23([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_20([a, b, c], is_dual)
    } else {
        select_22([d], is_dual)
    }
}
/// n = 9, i = 3
fn select_17([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([b, c, d, e, f], is_dual)
    } else {
        select_23([a, h, i, g], !is_dual)
    }
}
/// n = 4, i = 1
fn select_27([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_21([a, d], is_dual)
    } else {
        select_21([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_27([a, d, c, f], is_dual)
    } else {
        select_27([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_25([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_26([a, b, c, e, d, f], is_dual)
    } else {
        select_26([a, b, d, e, c, f], is_dual)
    }
}
/// n = 3, i = 1
fn select_30([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_21([a, b], is_dual)
    } else {
        select_22([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_20([b, c, d], is_dual)
    } else {
        select_30([a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_28([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_29([a, c, d, e, f, h], is_dual)
    } else {
        select_23([b, c, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_24([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([e, h, a, f, g, b], !is_dual)
    } else {
        select_28([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_16([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_17([a, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_24([a, b, c, i, g, j, h, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_34([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, b, e, f], is_dual)
    } else {
        select_27([a, c, d, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_35([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_20([b, c, d], is_dual)
    } else {
        select_21([a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_34([a, b, e, f, d, g], is_dual)
    } else {
        select_35([b, c, d, f, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_37([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_30([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_38([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_30([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_36([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_37([a, b, c, e, d, f], is_dual)
    } else {
        select_38([d, c, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_33([b, a, c, d, f, e, g], is_dual)
    } else {
        select_36([d, e, c, a, f, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, b, f, d, e], is_dual)
    } else {
        select_35([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, c, d, g], is_dual)
    } else {
        select_27([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_42([a, d, b, g, e, f, h], is_dual)
    } else {
        select_42([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_39([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_40([c, d, e, g, f, a], is_dual)
    } else {
        select_41([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_31([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_32([a, c, b, g, f, h, e, i], is_dual)
    } else {
        select_39([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_15([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_16([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_31([a, c, i, d, g, b, j, h, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_46([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_37([a, d, b, c, e, f], is_dual)
    } else {
        select_37([a, d, c, b, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_45([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_46([b, a, c, d, e, f], is_dual)
    } else {
        select_38([b, c, e, f, a], is_dual)
    }
}
/// n = 4, i = 1
fn select_49([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, b, d], is_dual)
    } else {
        select_27([b, c, a, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_48([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_37([a, d, c, e, f, g], is_dual)
    } else {
        select_49([a, b, d, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_50([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_37([f, g, e, c, d, a], !is_dual)
    } else {
        select_30([e, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_47([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([a, b, c, d, e, f, g], is_dual)
    } else {
        select_50([c, b, e, f, g, a, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_44([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_45([g, d, e, i, a, b], !is_dual)
    } else {
        select_47([d, g, h, i, e, f, b, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_52([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, d, e], is_dual)
    } else {
        select_30([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([c, d, e, f], is_dual)
    } else {
        select_23([a, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_52([f, c, d, e, a, g, h], is_dual)
    } else {
        select_53([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_43([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_44([i, j, k, a, g, f, b, c, h], !is_dual)
    } else {
        select_51([a, c, d, e, f, g, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_15([c, b, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_43([b, c, h, e, f, g, a, i, k, j, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_59([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_20([c, d, e], is_dual)
    } else {
        select_20([a, b, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_58([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_59([c, d, e, f, h, i], is_dual)
    } else {
        select_59([a, b, e, f, g, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_57([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_58([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_35([i, d, e, a, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_61([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([a, c, b, f, g, e], is_dual)
    } else {
        select_37([a, d, b, e, g, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_60([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_61([a, g, b, c, f, i, h], is_dual)
    } else {
        select_53([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_56([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_57([b, d, e, a, c, f, h, g, j, i], is_dual)
    } else {
        select_60([b, g, f, d, e, a, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_64([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([e, a, c, f, d, g], is_dual)
    } else {
        select_37([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_64([b, c, d, a, e, f, h], is_dual)
    } else {
        select_64([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_67([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_30([d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_66([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([a, b, c, d, e, g], is_dual)
    } else {
        select_67([f, g, a, d, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_65([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_66([b, c, g, a, f, i, h], is_dual)
    } else {
        select_53([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_62([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_63([a, b, g, c, i, f, j, k], is_dual)
    } else {
        select_65([a, b, f, d, e, h, g, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_55([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_56([a, c, b, e, f, d, g, i, h, j], is_dual)
    } else {
        select_62([a, b, c, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_48([a, b, d, c, e, f, g], is_dual)
    } else {
        select_27([d, f, e, c], is_dual)
    }
}
/// n = 8, i = 3
fn select_71([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_46([e, a, g, h, b, f], !is_dual)
    } else {
        select_52([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_69([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_70([e, h, i, j, f, a, b], !is_dual)
    } else {
        select_71([e, b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_74([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([b, c, d, e, f, g], is_dual)
    } else {
        select_49([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 2
fn select_75([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([b, c, d, e, g, h], is_dual)
    } else {
        select_35([a, d, e, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_73([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_74([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_75([a, d, e, b, c, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_77([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_37([a, c, b, e, f, g], is_dual)
    } else {
        select_23([a, c, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_76([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_77([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([a, b, f, g, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_72([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_73([a, b, c, d, e, f, g, h, j, k], is_dual)
    } else {
        select_76([b, a, c, h, f, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_68([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_69([b, g, d, e, f, a, h, j, k, i], is_dual)
    } else {
        select_72([b, a, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_54([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_55([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_68([b, c, h, e, f, g, a, i, k, l, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_14([b, c, a, e, f, g, d, h, i, j, k, l], is_dual)
    } else {
        select_54([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 1
fn select_83([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_59([a, f, c, d, e, g], is_dual)
    } else {
        select_59([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_82([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_83([a, c, d, e, b, f, g], is_dual)
    } else {
        select_83([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_85([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_59([a, b, c, d, e, f], is_dual)
    } else {
        select_35([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_84([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_85([a, b, d, e, c, f], is_dual)
    } else {
        select_85([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_81([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_82([c, d, e, a, b, f, g], is_dual)
    } else {
        select_84([a, b, f, c, d, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_87([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([b, a, d, h, e, g], is_dual)
    } else {
        select_36([g, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_87([d, c, a, e, f, b, g, h], is_dual)
    } else {
        select_87([d, c, b, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_80([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_81([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_86([c, d, e, b, g, f, a, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([c, d, e, f], is_dual)
    } else {
        select_49([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_92([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_19([c, d, e, g], is_dual)
    } else {
        select_35([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_91([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_92([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_19([d, e, f, g], is_dual)
    } else {
        select_19([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_94([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_53([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_89([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_90([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_93([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_88([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_89([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_89([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_79([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_80([a, c, d, e, i, h, b, j], is_dual)
    } else {
        select_88([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_97([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_82([c, f, e, a, b, g, h], is_dual)
    } else {
        select_82([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_97([c, d, a, e, b, h, g, i, f], is_dual)
    } else {
        select_97([c, d, b, e, a, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_96([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_96([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_79([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_95([a, b, d, e, f, i, h, c, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_13([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_78([a, c, d, e, f, g, b, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_105([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_20([b, c, e], is_dual)
    } else {
        select_27([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_105([b, a, c, d, e, f, g], is_dual)
    } else {
        select_49([a, e, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_25([d, c, a, f, e, g], is_dual)
    } else {
        select_104([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_37([a, f, b, e, h, i], is_dual)
    } else {
        select_20([c, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_107([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_74([a, f, c, d, e, g, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_102([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_103([i, j, k, f, g, a, b, c], !is_dual)
    } else {
        select_106([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_110([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_105([a, c, d, e, h, g, j], is_dual)
    } else {
        select_105([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_74([a, g, c, d, e, h, j, k], is_dual)
    } else {
        select_110([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_37([d, b, c, e, f, h], is_dual)
    } else {
        select_42([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_113([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_37([a, c, b, d, e, f], is_dual)
    } else {
        select_38([a, b, e, f, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_111([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_112([i, h, f, g, d, b, a, e], !is_dual)
    } else {
        select_113([b, d, c, f, g, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_108([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_109([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_111([j, k, g, e, i, a, f, h, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_101([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_102([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_108([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_100([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_101([a, d, b, e, f, g, c, h, i, k, j], is_dual)
    } else {
        select_101([a, d, c, e, f, g, b, h, j, k, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_64([b, g, c, e, a, f, h], is_dual)
    } else {
        select_83([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_119([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_67([d, f, e, a, b], !is_dual)
    } else {
        select_67([e, f, d, a, c], !is_dual)
    }
}
/// n = 5, i = 1
fn select_120([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_35([a, c, d, b, e], is_dual)
    } else {
        select_35([b, c, d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_119([a, b, c, d, e, f], is_dual)
    } else {
        select_120([d, e, a, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_116([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_117([b, c, a, d, f, e, g, h], is_dual)
    } else {
        select_118([f, e, i, a, b, h, g], !is_dual)
    }
}
/// n = 11, i = 3
fn select_115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_116([b, c, i, d, g, a, h, j, k], is_dual)
    } else {
        select_89([c, b, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_123([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_64([a, b, c, d, e, f, g], is_dual)
    } else {
        select_23([e, a, h, g], !is_dual)
    }
}
/// n = 7, i = 1
fn select_125([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([c, d, e, f], is_dual)
    } else {
        select_20([a, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_125([b, f, c, d, e, g, h], is_dual)
    } else {
        select_53([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_122([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_123([b, h, c, f, a, g, i, j], is_dual)
    } else {
        select_124([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_117([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_118([a, e, i, b, f, h, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_122([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_126([a, b, h, d, f, c, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_115([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_121([a, b, c, e, i, g, h, d, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_99([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_100([a, b, c, d, e, i, h, j, g, k, l], is_dual)
    } else {
        select_114([a, d, c, e, b, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_98([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_99([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_99([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_12([b, a, c, f, g, d, e, h, i, j, m, k], is_dual)
    } else {
        select_98([c, a, b, d, e, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_134([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_23([e, b, c, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_104([a, b, d, f, g, e, h, i], is_dual)
    } else {
        select_134([b, c, e, f, g, d, h, j], is_dual)
    }
}
/// n = 3, i = 1
fn select_137([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_30([a, b, c], is_dual)
    } else {
        select_30([a, c, b], is_dual)
    }
}
/// n = 7, i = 3
fn select_136([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_137([a, f, g], is_dual)
    } else {
        select_26([a, b, c, d, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_135([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_104([a, c, b, e, f, d, g, h], is_dual)
    } else {
        select_136([d, c, a, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_132([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_133([a, c, b, e, d, f, h, g, i, j], is_dual)
    } else {
        select_135([a, e, c, g, f, h, d, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_139([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([b, c, d, a, e, f, g], is_dual)
    } else {
        select_67([d, f, e, a, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_141([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([a, e, c, f, d, h], is_dual)
    } else {
        select_37([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_140([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_104([h, i, j, g, a, e, b, c], !is_dual)
    } else {
        select_141([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_139([a, b, c, f, g, h, e, j], is_dual)
    } else {
        select_140([b, a, c, d, e, f, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_132([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_138([b, a, c, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_145([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_29([a, b, c, d, f, g], is_dual)
    } else {
        select_49([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_144([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_145([a, c, d, b, e, f, g], is_dual)
    } else {
        select_145([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_143([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_144([a, b, c, f, h, e, g], is_dual)
    } else {
        select_144([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_147([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_64([b, c, d, a, e, f, g], is_dual)
    } else {
        select_40([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_144([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_147([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_142([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_143([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_146([a, h, e, i, f, b, g, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_131([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_142([a, b, g, c, h, f, i, j, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_130([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_130([a, b, d, e, f, c, g, h, j, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_153([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_19([a, b, c, d], is_dual)
    } else {
        select_22([e], is_dual)
    }
}
/// n = 6, i = 2
fn select_154([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, b, e, f], is_dual)
    } else {
        select_29([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_152([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_153([a, d, f, g, b], !is_dual)
    } else {
        select_154([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 7, i = 1
fn select_155([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_153([a, c, d, e, g], is_dual)
    } else {
        select_85([a, b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_151([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_152([a, g, c, b, f, h, i], is_dual)
    } else {
        select_155([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_73([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_73([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_150([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_151([a, b, g, e, f, c, h, i, j], is_dual)
    } else {
        select_156([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_149([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_150([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_150([a, b, e, d, f, g, c, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_161([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([c, d, e, f], is_dual)
    } else {
        select_67([h, i, a, b, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_160([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_52([f, c, d, e, a, g, i], is_dual)
    } else {
        select_161([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_160([e, f, a, b, c, d, g, i, j], is_dual)
    } else {
        select_160([e, d, a, b, c, f, g, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_163([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_153([a, c, d, e, f], is_dual)
    } else {
        select_154([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_162([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_163([a, b, c, d, e, g, h], is_dual)
    } else {
        select_163([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_158([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_159([a, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_162([f, g, a, b, c, d, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_167([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_34([d, f, g, a, e, b], !is_dual)
    } else {
        select_67([e, f, d, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_166([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_167([a, b, c, d, f, g, e], is_dual)
    } else {
        select_167([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_169([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_49([a, b, c, e], is_dual)
    } else {
        select_49([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_168([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_169([f, a, b, d, e], is_dual)
    } else {
        select_169([e, a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_165([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_166([b, c, d, a, e, f, g], is_dual)
    } else {
        select_168([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_165([g, d, a, b, e, f, h], is_dual)
    } else {
        select_165([f, d, a, c, e, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_158([a, b, c, g, d, e, f, h, i, j, k], is_dual)
    } else {
        select_164([i, j, k, g, d, e, f, h], !is_dual)
    }
}
/// n = 12, i = 5
fn select_148([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_149([b, c, a, e, f, d, g, h, j, i], is_dual)
    } else {
        select_157([e, f, g, b, c, d, a, i, k, l, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_129([c, e, a, b, f, h, i, d, j, k], is_dual)
    } else {
        select_148([c, a, b, e, f, d, g, h, i, l, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_176([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_37([a, h, b, f, i, j], is_dual)
    } else {
        select_19([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_91([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_176([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_175([c, a, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_175([c, b, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_178([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_53([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_53([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_177([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_178([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_178([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_173([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_174([c, d, b, e, f, g, a, h, i, j], is_dual)
    } else {
        select_177([c, d, a, e, f, g, b, h, j, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_182([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_137([a, e, f], is_dual)
    } else {
        select_35([a, b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_183([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_59([a, b, c, d, e, f], is_dual)
    } else {
        select_49([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_181([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_182([e, c, d, a, f, g], is_dual)
    } else {
        select_183([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_77([a, b, c, d, e, f, g], is_dual)
    } else {
        select_37([a, h, g, e, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_181([a, b, d, e, c, f, g], is_dual)
    } else {
        select_184([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 5, i = 2
fn select_187([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_137([a, d, e], is_dual)
    } else {
        select_27([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_186([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_187([f, d, b, a, e], !is_dual)
    } else {
        select_37([b, c, a, d, f, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_185([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_186([b, a, c, d, e, f], is_dual)
    } else {
        select_45([e, a, d, g, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_179([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_180([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_185([a, h, g, d, b, c, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_172([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_173([g, c, e, f, a, b, d, h, i, j], is_dual)
    } else {
        select_179([g, e, f, c, h, d, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_191([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_33([a, b, c, d, f, e, g], is_dual)
    } else {
        select_50([e, c, a, f, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_192([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_64([b, c, d, g, e, f, h], is_dual)
    } else {
        select_167([g, h, i, a, e, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_190([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_191([e, c, d, a, f, g, h, i], is_dual)
    } else {
        select_192([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_195([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_37([a, c, b, d, e, f], is_dual)
    } else {
        select_29([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_194([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_64([b, c, d, e, g, f, h], is_dual)
    } else {
        select_195([a, b, c, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_196([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_64([a, b, c, d, e, f, h], is_dual)
    } else {
        select_36([e, h, i, a, g, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_193([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_194([a, b, c, d, e, g, f, h, j], is_dual)
    } else {
        select_196([f, b, d, a, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_189([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_190([a, d, b, e, f, c, g, h, i], is_dual)
    } else {
        select_193([a, d, c, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_198([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_93([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_124([a, h, d, e, f, g, b, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_200([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_53([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_161([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_199([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_93([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_200([b, h, d, e, f, g, a, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_198([a, c, d, e, f, g, b, h, i, k], is_dual)
    } else {
        select_199([c, b, d, e, f, g, a, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_188([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_189([a, c, d, i, e, b, h, j, k, l], is_dual)
    } else {
        select_197([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_171([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_172([c, h, a, b, e, f, d, j, k, i], is_dual)
    } else {
        select_188([a, b, c, d, g, e, f, h, i, j, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_206([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_37([a, f, b, e, d, g], is_dual)
    } else {
        select_35([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_205([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_64([a, f, b, e, i, g, j], is_dual)
    } else {
        select_206([e, c, d, h, f, g, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_207([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_104([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_104([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_204([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_205([b, a, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_207([d, b, f, e, a, g, i, h, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_210([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_37([a, b, c, e, d, g], is_dual)
    } else {
        select_37([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_209([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_210([a, b, e, g, h, i, j], is_dual)
    } else {
        select_206([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_67([b, f, d, a, h], is_dual)
    } else {
        select_42([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_212([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_212([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_208([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_209([a, f, c, d, g, h, e, j, i, k], is_dual)
    } else {
        select_211([a, d, b, e, g, f, j, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_204([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_208([a, d, c, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_216([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_49([a, g, c, i], is_dual)
    } else {
        select_42([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_215([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_41([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_216([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_214([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_215([b, c, e, d, f, g, h, i, k, j], is_dual)
    } else {
        select_50([l, k, f, e, b, a, j], !is_dual)
    }
}
/// n = 8, i = 3
fn select_219([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_37([a, c, d, f, e, h], is_dual)
    } else {
        select_37([b, d, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_218([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_219([a, b, g, d, e, h, j, k], is_dual)
    } else {
        select_216([b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_220([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_113([a, b, i, g, e, j], is_dual)
    } else {
        select_41([b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_217([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_218([a, b, d, e, f, h, i, g, j, k, l], is_dual)
    } else {
        select_220([a, g, c, d, f, h, e, j, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_213([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_214([c, d, a, e, f, g, b, h, j, i, k, l], is_dual)
    } else {
        select_217([c, d, b, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_202([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_203([a, b, c, g, e, i, h, f, j, k, m], is_dual)
    } else {
        select_213([c, b, d, a, e, g, f, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_202([a, c, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    } else {
        select_202([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_170([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_171([a, b, d, e, c, f, g, i, h, j, l, k, m], is_dual)
    } else {
        select_201([a, b, d, e, f, g, h, i, c, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_128([a, b, c, d, f, e, i, k, j, h, l, m], is_dual)
    } else {
        select_170([c, d, b, f, e, a, g, h, i, k, j, m, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_11([a, e, d, b, c, f, g, h, i, j, l, m, k], is_dual)
    } else {
        select_127([a, e, b, c, f, d, g, h, i, l, k, m, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_10([a, b, c, d, e, g, f, h, i, k, l, j, m], is_dual)
    } else {
        select_10([a, b, c, e, d, f, g, h, j, k, m, i, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_9([e, c, d, b, f, a, g, i, h, k, l, m, j], is_dual)
    } else {
        select_9([e, c, d, b, g, a, f, i, h, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_8([c, d, a, b, e, f, g, h, j, k, l, m, i], is_dual)
    } else {
        select_8([d, c, a, b, e, f, g, i, j, k, l, m, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_7([a, b, c, d, e, f, g, i, j, k, l, h, m], is_dual)
    } else {
        select_7([a, b, c, d, e, f, h, i, j, k, l, g, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_6([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    } else {
        select_6([a, b, c, d, e, i, g, h, j, k, l, f, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_5([i, j, a, g, h, b, c, d, e, f, k, l, m], is_dual)
    } else {
        select_5([i, j, f, g, h, b, c, d, e, a, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_4([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_4([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_3([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_237([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_67([a, d, g, f, c], is_dual)
    } else {
        select_67([b, c, g, e, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_238([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_37([f, c, g, e, d, a], !is_dual)
    } else {
        select_67([d, g, c, e, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_236([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_237([e, g, d, f, a, c, b], !is_dual)
    } else {
        select_238([d, g, a, f, e, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_240([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([g, h, f, d, a, b], !is_dual)
    } else {
        select_49([a, f, c, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_239([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_145([a, c, d, e, g, f, h], is_dual)
    } else {
        select_240([h, i, g, e, d, a, b, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_235([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_236([e, h, f, a, d, g, c], !is_dual)
    } else {
        select_239([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_242([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_219([e, h, g, i, d, f, b, c], !is_dual)
    } else {
        select_74([e, g, h, i, d, f, a, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_244([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_37([a, b, c, e, d, f], is_dual)
    } else {
        select_27([c, d, e, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_243([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_107([a, c, b, d, e, f, g, i, h], is_dual)
    } else {
        select_244([a, b, d, g, h, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_241([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_242([i, h, j, e, a, g, f, c, d], !is_dual)
    } else {
        select_243([b, c, a, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_234([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_235([b, c, g, f, e, a, i, j, h], is_dual)
    } else {
        select_241([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_248([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([g, i, h, d, f, a], !is_dual)
    } else {
        select_23([e, b, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_247([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_237([b, e, d, a, f, g, h], is_dual)
    } else {
        select_248([b, a, c, d, f, e, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_247([b, a, d, e, g, h, k, j, f], is_dual)
    } else {
        select_247([a, b, c, f, g, i, j, k, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_251([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_105([a, b, f, d, g, e, h], is_dual)
    } else {
        select_105([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_250([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_169([a, b, i, j, k], is_dual)
    } else {
        select_251([a, c, d, g, e, f, h, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_253([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_67([b, c, a, d, e], is_dual)
    } else {
        select_27([a, c, b, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_254([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_105([a, c, d, e, g, f, i], is_dual)
    } else {
        select_49([a, h, b, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_252([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_253([h, b, i, a, j], is_dual)
    } else {
        select_254([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_249([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_250([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_252([a, b, c, f, g, i, h, j, e, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_245([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_246([a, b, f, c, i, g, j, k, h, e, l], is_dual)
    } else {
        select_249([b, c, a, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_233([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_234([j, e, l, i, a, k, g, h, b, c], !is_dual)
    } else {
        select_245([a, b, c, d, e, f, g, i, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_232([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_233([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_233([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_231([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_232([a, b, c, d, e, f, g, h, i, l, k, j], is_dual)
    } else {
        select_232([a, b, d, c, e, f, g, h, i, l, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_260([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_46([a, e, f, h, g, b], !is_dual)
    } else {
        select_61([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_261([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_46([a, d, f, g, e, b], !is_dual)
    } else {
        select_113([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_259([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_260([a, c, b, d, f, g, e, h], is_dual)
    } else {
        select_261([a, c, e, g, b, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_258([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_259([e, b, a, c, d, f, g, h], is_dual)
    } else {
        select_259([e, b, a, d, c, f, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_257([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_258([e, a, c, d, b, f, g, h], is_dual)
    } else {
        select_258([i, a, g, h, b, f, c, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_266([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_34([b, c, d, e, f, g], is_dual)
    } else {
        select_26([a, c, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_265([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_266([a, b, c, h, f, j, k, i], is_dual)
    } else {
        select_120([a, g, d, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_267([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_266([c, a, b, d, f, h, i, g], is_dual)
    } else {
        select_169([e, a, b, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_264([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_265([c, a, b, d, e, g, h, i, f, j, k], is_dual)
    } else {
        select_267([j, k, h, g, f, i, c, a, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_269([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_77([a, b, d, e, c, f, g], is_dual)
    } else {
        select_77([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_268([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_166([f, h, i, a, b, c, g], !is_dual)
    } else {
        select_269([d, b, c, e, f, g, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_264([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    } else {
        select_268([f, j, k, i, l, a, g, b, c, h], !is_dual)
    }
}
/// n = 13, i = 5
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_263([g, a, b, c, d, f, h, j, i, k, l, m], is_dual)
    } else {
        select_263([f, a, b, c, e, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_256([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_257([f, g, a, b, j, k, l, m, h], is_dual)
    } else {
        select_262([a, b, c, d, e, f, g, h, i, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_273([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_45([h, a, b, i, f, m], is_dual)
    } else {
        select_265([b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_272([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_273([a, c, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    } else {
        select_273([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 6, i = 1
fn select_276([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_153([a, c, d, e, f], is_dual)
    } else {
        select_120([a, b, c, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_275([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_166([f, h, i, a, b, c, g], !is_dual)
    } else {
        select_276([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_279([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_105([b, c, d, e, g, f, h], is_dual)
    } else {
        select_49([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_278([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_279([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_279([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_277([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_278([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_278([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_274([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_275([a, b, c, d, e, g, i, k, l], is_dual)
    } else {
        select_277([d, e, f, b, c, g, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_272([c, d, e, a, b, f, h, i, g, k, l, m, j], is_dual)
    } else {
        select_274([g, a, b, c, d, e, j, h, k, i, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_271([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_271([a, b, c, d, f, e, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_255([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_256([b, c, d, e, f, g, h, i, j, k, a, l, m], is_dual)
    } else {
        select_270([b, c, a, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_230([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_231([a, h, c, d, e, j, i, b, k, l, m, g], is_dual)
    } else {
        select_255([a, c, d, e, b, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_154([a, f, h, i, g, b], !is_dual)
    } else {
        select_85([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_286([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_286([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_287([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_117([a, d, b, e, f, c, g, h], is_dual)
    } else {
        select_117([a, d, c, e, f, b, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_284([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_285([a, g, i, f, j, b, h, c, d], !is_dual)
    } else {
        select_287([a, c, d, e, b, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_290([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_244([a, b, f, d, g, e], is_dual)
    } else {
        select_206([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_289([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_290([a, b, g, e, f, c, h], is_dual)
    } else {
        select_278([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_292([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_85([b, d, a, e, f, g], is_dual)
    } else {
        select_85([a, c, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_293([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_240([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_167([b, a, c, h, e, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_291([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_292([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_293([c, a, d, b, f, g, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_288([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_289([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_291([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_283([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_284([a, c, d, e, b, g, f, h, i, j], is_dual)
    } else {
        select_288([a, i, f, j, k, g, b, c, h], !is_dual)
    }
}
/// n = 7, i = 2
fn select_297([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_64([c, d, a, e, f, b, g], is_dual)
    } else {
        select_64([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_163([e, f, a, h, i, g, b], !is_dual)
    } else {
        select_297([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_295([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_296([g, i, f, j, a, b, h, c, d], !is_dual)
    } else {
        select_287([b, c, d, e, a, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_300([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_46([a, b, c, f, e, g], is_dual)
    } else {
        select_120([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_302([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_29([a, b, c, d, e, f], is_dual)
    } else {
        select_29([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_301([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_302([a, c, d, e, g, h, i], is_dual)
    } else {
        select_85([e, b, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_299([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_300([a, b, c, f, e, g, h], is_dual)
    } else {
        select_301([a, d, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_305([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_37([a, g, c, f, d, i], is_dual)
    } else {
        select_42([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_304([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_305([a, c, b, d, f, e, h, g, i], is_dual)
    } else {
        select_212([a, e, c, g, f, h, d, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_306([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_64([a, b, c, d, e, f, g], is_dual)
    } else {
        select_153([e, a, h, i, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_303([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_304([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_306([a, e, c, d, g, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_298([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_299([b, a, c, d, e, g, h, f, i], is_dual)
    } else {
        select_303([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_294([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_295([e, h, f, j, k, a, b, i, c, g], !is_dual)
    } else {
        select_298([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_282([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_283([f, e, k, j, l, a, g, i, b, h, d], !is_dual)
    } else {
        select_294([a, g, c, b, e, f, i, j, h, l, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_313([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_27([b, c, d, e], is_dual)
    } else {
        select_30([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_312([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_313([a, c, d, e, f, g, h], is_dual)
    } else {
        select_49([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_311([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_312([b, a, c, d, e, f, g, i], is_dual)
    } else {
        select_50([h, i, d, g, b, a, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_310([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_311([a, c, b, f, e, h, d, i, g], is_dual)
    } else {
        select_103([b, c, a, f, d, h, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_315([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_154([a, g, f, h, d, b], !is_dual)
    } else {
        select_154([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_314([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_315([e, h, j, f, i, a, b, g], !is_dual)
    } else {
        select_181([b, c, d, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_309([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_310([h, e, j, f, a, i, b, c, g], !is_dual)
    } else {
        select_314([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_318([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_154([a, b, c, e, g, h], is_dual)
    } else {
        select_85([a, d, b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_319([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_33([b, d, c, e, f, g, h], is_dual)
    } else {
        select_119([a, b, d, e, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_317([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_318([e, a, h, d, g, f, j, k], is_dual)
    } else {
        select_319([b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_321([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_25([b, c, a, d, e, f], is_dual)
    } else {
        select_26([a, c, b, e, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_323([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([a, e, c, f, d, h], is_dual)
    } else {
        select_34([b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_322([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_305([a, b, c, e, d, f, g, i, h], is_dual)
    } else {
        select_323([a, b, f, e, d, h, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_320([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_321([a, d, f, g, h, j], is_dual)
    } else {
        select_322([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_316([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_317([b, c, a, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_320([b, g, d, a, f, h, e, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_308([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_309([a, b, c, h, e, f, i, j, g, k], is_dual)
    } else {
        select_316([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_328([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_37([d, b, c, e, f, g], is_dual)
    } else {
        select_38([a, b, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_327([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_328([a, c, b, d, e, g, f, h], is_dual)
    } else {
        select_136([g, h, d, e, a, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_330([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_37([b, d, c, e, g, f], is_dual)
    } else {
        select_67([d, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_329([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_113([i, f, j, a, c, b], !is_dual)
    } else {
        select_330([b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_326([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_327([a, d, c, g, e, f, i, h], is_dual)
    } else {
        select_329([a, b, c, d, e, g, h, i, j, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_333([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_37([a, g, b, d, i, f], is_dual)
    } else {
        select_37([a, f, c, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_334([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_37([f, c, b, e, h, g], is_dual)
    } else {
        select_26([a, c, d, g, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_332([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_333([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_334([c, b, a, e, d, g, h, i, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_331([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_186([a, g, b, d, h, j], is_dual)
    } else {
        select_332([b, c, a, e, d, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_325([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_326([b, a, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_331([d, c, b, f, e, h, a, i, j, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_337([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([a, b, d, g, f, i], is_dual)
    } else {
        select_27([c, d, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_336([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_33([e, c, d, a, f, g, h], is_dual)
    } else {
        select_337([b, a, d, c, f, e, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_338([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_141([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_206([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_335([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_336([f, a, c, d, e, g, h, i, j], is_dual)
    } else {
        select_338([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_324([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_325([a, b, h, c, f, g, e, i, j, k], is_dual)
    } else {
        select_335([c, b, d, a, g, f, e, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_307([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_308([a, c, b, d, e, g, f, h, i, k, j], is_dual)
    } else {
        select_324([a, c, b, d, e, g, f, h, j, k, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_281([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_282([a, b, g, c, e, i, f, j, k, l, h, m], is_dual)
    } else {
        select_307([a, b, f, d, e, k, h, g, i, l, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_344([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_119([c, d, g, e, f, a], !is_dual)
    } else {
        select_36([c, e, g, d, f, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_343([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_71([d, e, b, c, f, g, h, j], is_dual)
    } else {
        select_344([g, a, h, i, d, e, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_346([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_36([a, c, d, e, f, g], is_dual)
    } else {
        select_195([e, h, f, a, b, g, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_348([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_105([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_347([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_36([h, f, i, a, e, b], !is_dual)
    } else {
        select_348([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_345([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_346([h, f, e, j, a, b, i, g], !is_dual)
    } else {
        select_347([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_342([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_343([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_345([b, a, c, d, e, f, g, h, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_351([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_33([a, c, b, e, f, g, h], is_dual)
    } else {
        select_35([b, c, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_352([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_183([f, a, c, d, e, h, g], is_dual)
    } else {
        select_183([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_350([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_351([c, a, d, e, f, b, h, g], is_dual)
    } else {
        select_352([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_353([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_71([a, e, c, d, f, g, i, j], is_dual)
    } else {
        select_261([a, g, b, h, e, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_350([b, c, d, e, g, f, i, h], is_dual)
    } else {
        select_353([f, i, k, l, g, a, j, b, c, h], !is_dual)
    }
}
/// n = 12, i = 5
fn select_341([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_342([g, f, k, l, a, h, j, b, i, c, d], !is_dual)
    } else {
        select_349([a, b, c, d, e, g, f, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_340([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_341([a, c, d, e, b, f, g, i, h, j, k, l], is_dual)
    } else {
        select_283([g, f, i, k, l, a, b, j, c, d, h], !is_dual)
    }
}
/// n = 10, i = 3
fn select_358([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_28([a, b, c, d, f, i, h, j], is_dual)
    } else {
        select_85([b, e, c, d, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_357([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_103([a, b, c, g, h, i, j, f], is_dual)
    } else {
        select_358([b, c, d, e, a, g, f, i, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_360([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_240([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_240([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_359([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_32([g, i, h, a, f, e, d, c], !is_dual)
    } else {
        select_360([h, i, g, a, f, d, e, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_356([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_357([b, c, d, a, e, g, f, h, j, i], is_dual)
    } else {
        select_359([i, k, j, g, f, a, b, c, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_363([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_64([c, e, d, g, a, f, i], is_dual)
    } else {
        select_41([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_362([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_363([b, d, c, a, f, e, g, h, i], is_dual)
    } else {
        select_32([a, d, b, g, f, h, e, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_365([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_25([d, c, a, f, e, h], is_dual)
    } else {
        select_266([c, a, b, e, d, g, h, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_366([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_48([a, f, b, d, e, h, g], is_dual)
    } else {
        select_42([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_364([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_365([a, c, b, f, g, h, e, i], is_dual)
    } else {
        select_366([b, c, a, d, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_361([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_362([b, a, g, d, f, h, i, e, k, j], is_dual)
    } else {
        select_364([b, c, d, e, f, h, g, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_355([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_356([a, b, g, c, e, i, f, j, k, h, l], is_dual)
    } else {
        select_361([a, b, c, d, h, f, j, g, k, i, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_354([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < l) || (is_dual && f > l) {
        select_294([b, a, g, c, h, i, j, k, l, m, f], is_dual)
    } else {
        select_355([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_339([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_340([a, b, c, d, g, f, i, j, h, k, l, m], is_dual)
    } else {
        select_354([a, b, c, e, d, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_280([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_281([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    } else {
        select_339([b, a, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_229([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_230([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_280([a, b, e, f, d, g, h, i, j, c, k, m, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_374([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_77([a, d, b, c, e, f, g], is_dual)
    } else {
        select_38([f, d, a, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_373([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_374([a, c, d, b, e, f, g], is_dual)
    } else {
        select_163([a, f, e, h, i, g, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_372([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_287([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_373([g, i, f, j, a, b, h, c, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_376([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_145([g, h, i, a, b, f, d], !is_dual)
    } else {
        select_195([h, g, i, a, b, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_375([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_292([a, c, d, b, g, e, h, f], is_dual)
    } else {
        select_376([a, b, d, c, g, f, h, e, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_371([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_372([f, h, j, k, e, b, a, i, c, g], !is_dual)
    } else {
        select_375([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_380([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_77([b, c, d, e, f, g, i], is_dual)
    } else {
        select_113([a, c, g, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_381([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_219([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_183([b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_379([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_380([c, d, b, e, f, a, g, i, h], is_dual)
    } else {
        select_381([c, d, a, e, f, b, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_383([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_253([a, c, e, b, f], is_dual)
    } else {
        select_120([a, c, b, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_382([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_383([a, f, b, c, g, e], is_dual)
    } else {
        select_147([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_378([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_379([b, a, e, d, c, g, f, i, h], is_dual)
    } else {
        select_382([b, e, a, f, g, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_377([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_378([a, c, d, b, e, g, h, f, i], is_dual)
    } else {
        select_378([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_370([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_371([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_377([e, i, j, k, f, a, c, b, h], !is_dual)
    }
}
/// n = 7, i = 3
fn select_388([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_25([d, g, a, e, f, b], !is_dual)
    } else {
        select_50([e, g, d, f, a, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_387([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_24([a, c, d, f, e, h, g, i], is_dual)
    } else {
        select_388([a, b, g, e, i, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_386([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_387([b, c, d, a, f, e, g, i, h], is_dual)
    } else {
        select_359([h, j, i, f, e, a, b, c, g], !is_dual)
    }
}
/// n = 9, i = 4
fn select_390([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_32([g, i, f, a, h, d, e, b], !is_dual)
    } else {
        select_32([f, i, g, a, h, e, d, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_392([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_25([d, c, a, f, e, g], is_dual)
    } else {
        select_134([c, a, b, e, g, d, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_391([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_392([i, j, h, f, e, b, a, g], !is_dual)
    } else {
        select_24([b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_389([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_390([h, i, j, e, f, a, b, c, g], !is_dual)
    } else {
        select_391([a, b, d, c, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_385([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_386([a, c, b, d, f, g, h, i, e, j], is_dual)
    } else {
        select_389([a, c, b, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_384([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_385([a, c, b, d, e, g, f, h, i, j], is_dual)
    } else {
        select_294([a, b, c, d, e, g, h, i, j, k, f], is_dual)
    }
}
/// n = 11, i = 5
fn select_369([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_370([f, g, h, k, b, c, j, i, d, a, e], !is_dual)
    } else {
        select_384([b, c, d, e, f, g, h, i, a, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_398([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_40([a, b, g, e, f, d], is_dual)
    } else {
        select_83([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_397([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_398([a, d, e, b, c, f, g], is_dual)
    } else {
        select_117([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_399([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_163([a, e, b, h, i, g, f], !is_dual)
    } else {
        select_147([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_396([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_397([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_399([a, b, f, j, c, i, h, d, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_395([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_396([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_396([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_394([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_395([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_395([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_400([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_283([a, f, g, j, k, b, h, i, c, d, e], !is_dual)
    } else {
        select_283([a, f, h, j, k, b, g, i, c, e, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_393([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_394([a, h, f, j, k, g, b, c, i, d], !is_dual)
    } else {
        select_400([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_368([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_369([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_393([a, b, c, e, d, f, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_407([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_25([f, h, a, b, g, c], !is_dual)
    } else {
        select_244([a, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_406([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_300([a, c, d, e, g, f, h], is_dual)
    } else {
        select_407([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_409([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_253([a, h, b, i, e], is_dual)
    } else {
        select_74([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_410([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_64([f, j, i, a, h, e, b], !is_dual)
    } else {
        select_348([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_408([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_409([a, b, d, e, c, g, f, h, i], is_dual)
    } else {
        select_410([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_405([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_406([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_408([b, a, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_413([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_153([a, h, i, j, g], !is_dual)
    } else {
        select_18([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_412([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_300([f, h, i, j, a, b, g], !is_dual)
    } else {
        select_413([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_416([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_105([a, b, c, e, f, g, h], is_dual)
    } else {
        select_20([b, c, d], is_dual)
    }
}
/// n = 9, i = 2
fn select_415([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_416([a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_92([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_417([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([b, c, d, f], is_dual)
    } else {
        select_67([a, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_414([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_415([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_417([g, c, d, e, a, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_411([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_412([a, b, c, d, g, f, i, k, l, h], is_dual)
    } else {
        select_414([a, e, b, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_404([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_405([f, a, i, k, l, b, h, j, g, e], !is_dual)
    } else {
        select_411([b, a, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_403([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_283([a, f, h, k, l, b, i, j, g, c, d], !is_dual)
    } else {
        select_404([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_402([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_394([a, i, f, k, l, h, b, c, j, g], !is_dual)
    } else {
        select_403([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_423([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_219([f, h, i, c, d, g, a, e], !is_dual)
    } else {
        select_195([f, g, i, c, d, e, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_425([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([c, d, g, f, a, e], !is_dual)
    } else {
        select_23([c, b, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_424([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_425([g, j, a, b, h, f, d], !is_dual)
    } else {
        select_141([a, c, d, g, e, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_422([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_423([g, j, a, h, i, b, e, c, f], !is_dual)
    } else {
        select_424([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_427([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_119([i, f, j, a, b, c], !is_dual)
    } else {
        select_212([b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_428([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_37([d, h, i, g, a, f], !is_dual)
    } else {
        select_29([d, b, c, e, f, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_426([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_427([g, h, j, k, i, d, e, a, b, f], !is_dual)
    } else {
        select_428([a, c, e, d, g, f, i, h, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_421([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_422([a, b, c, d, f, g, e, h, i, k], is_dual)
    } else {
        select_426([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_430([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_300([a, c, d, e, b, f, g], is_dual)
    } else {
        select_300([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_432([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_49([a, b, c, d], is_dual)
    } else {
        select_30([a, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_431([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_163([a, b, c, d, e, f, g], is_dual)
    } else {
        select_432([a, b, f, c, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_429([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_430([a, b, d, e, c, f, g], is_dual)
    } else {
        select_431([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 11, i = 5
fn select_420([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_421([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    } else {
        select_429([a, f, c, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_435([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_186([i, c, d, h, g, a], !is_dual)
    } else {
        select_423([a, b, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_438([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_67([e, f, a, b, c], !is_dual)
    } else {
        select_23([b, c, d, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_437([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_48([g, h, i, d, e, f, a], !is_dual)
    } else {
        select_438([d, e, b, c, f, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_436([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_147([d, b, e, c, g, f, k], is_dual)
    } else {
        select_437([a, b, f, d, e, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_434([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_435([b, f, d, e, h, g, i, a, j], is_dual)
    } else {
        select_436([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_441([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_119([b, c, d, e, f, g], is_dual)
    } else {
        select_195([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_440([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_441([g, i, j, e, a, b, h, f], !is_dual)
    } else {
        select_147([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_443([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_64([b, c, d, a, e, f, h], is_dual)
    } else {
        select_64([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_444([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_237([a, b, c, e, d, g, f], is_dual)
    } else {
        select_237([a, b, d, e, c, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_442([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_443([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_444([h, e, b, f, g, c, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_439([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_440([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_442([e, i, j, g, b, a, c, h], !is_dual)
    }
}
/// n = 11, i = 5
fn select_433([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_434([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_439([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_419([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_420([a, c, b, d, f, g, e, h, i, k, j], is_dual)
    } else {
        select_433([a, b, c, d, f, g, h, i, j, e, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_450([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_67([b, c, f, e, h], is_dual)
    } else {
        select_38([a, b, d, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_451([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([b, c, e, g], is_dual)
    } else {
        select_23([a, d, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_449([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_450([a, c, b, e, d, g, f, i, h], is_dual)
    } else {
        select_451([g, d, h, i, e, f, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_453([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_137([a, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_452([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_312([a, b, c, e, d, f, g, h], is_dual)
    } else {
        select_453([a, b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_448([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_449([b, e, c, h, f, d, i, g, j], is_dual)
    } else {
        select_452([b, a, d, g, f, i, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_454([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_118([f, e, h, a, b, g, c], !is_dual)
    } else {
        select_147([b, c, d, a, f, e, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_447([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_448([b, a, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_454([g, h, k, d, e, a, j, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_457([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_25([a, c, d, e, f, h], is_dual)
    } else {
        select_26([a, b, d, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_458([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_25([a, b, d, f, g, e], is_dual)
    } else {
        select_33([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_456([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_457([b, a, f, e, h, d, i, g], is_dual)
    } else {
        select_458([b, d, c, e, g, h, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_460([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_77([a, d, c, e, f, h, g], is_dual)
    } else {
        select_36([a, e, j, b, i, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_462([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_34([a, c, e, h, d, g], is_dual)
    } else {
        select_34([b, c, d, h, e, f], is_dual)
    }
}
/// n = 11, i = 5
fn select_461([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_195([h, j, g, d, f, e, c], !is_dual)
    } else {
        select_462([j, k, f, d, i, a, g, b], !is_dual)
    }
}
/// n = 12, i = 5
fn select_459([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_460([a, e, c, d, g, f, i, h, k, l], is_dual)
    } else {
        select_461([g, j, l, e, k, h, i, a, f, b, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_455([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_456([c, d, e, f, g, b, h, j, i], is_dual)
    } else {
        select_459([a, c, b, d, f, g, h, e, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_446([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_447([b, g, c, e, f, i, h, a, j, k, l], is_dual)
    } else {
        select_455([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_466([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_36([a, d, c, f, e, h], is_dual)
    } else {
        select_210([a, b, e, d, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_465([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_466([a, h, j, e, i, b, f, g, c], !is_dual)
    } else {
        select_181([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_469([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_49([a, g, i, h], !is_dual)
    } else {
        select_42([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_468([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_136([b, e, d, g, f, a, h], is_dual)
    } else {
        select_469([f, i, h, d, g, b, a, e, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_467([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_468([a, i, k, f, j, b, g, c, h], !is_dual)
    } else {
        select_178([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_464([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_465([a, b, c, i, f, g, j, k, h, l], is_dual)
    } else {
        select_467([a, b, g, d, e, f, h, i, k, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_473([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_59([b, c, d, e, g, f], is_dual)
    } else {
        select_23([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_472([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_153([a, b, i, j, h], !is_dual)
    } else {
        select_473([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_474([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_37([a, b, c, f, e, g], is_dual)
    } else {
        select_37([a, b, d, f, e, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_471([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_472([a, b, f, d, e, h, g, i, k, l], is_dual)
    } else {
        select_474([c, g, d, e, f, h, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_477([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_29([d, b, c, e, f, g], is_dual)
    } else {
        select_35([d, h, i, a, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_478([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([b, c, d, f], is_dual)
    } else {
        select_29([e, i, j, a, h, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_476([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_477([a, b, h, e, g, f, i, j, k], is_dual)
    } else {
        select_478([a, c, d, f, e, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_480([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_19([b, c, d, f], is_dual)
    } else {
        select_38([a, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_481([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_313([a, b, d, f, g, h, i], is_dual)
    } else {
        select_49([b, e, c, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_479([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_480([a, c, d, f, e, g, h, j, k], is_dual)
    } else {
        select_481([a, b, h, e, g, f, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_475([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_476([a, b, c, d, e, f, g, h, j, k, l], is_dual)
    } else {
        select_479([a, b, c, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_470([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_471([a, b, c, d, e, g, f, h, j, i, k, l], is_dual)
    } else {
        select_475([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_463([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_464([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_470([a, b, g, d, e, f, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_445([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_446([a, c, h, e, f, b, g, j, i, k, l, m], is_dual)
    } else {
        select_463([a, c, e, b, d, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_418([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_419([a, j, f, m, c, b, k, l, i, g, h], !is_dual)
    } else {
        select_445([a, c, b, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_401([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_402([a, b, c, h, d, f, g, j, k, l, i, m], is_dual)
    } else {
        select_418([a, b, c, e, d, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_367([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_368([a, b, c, d, j, f, k, l, m, g, i], is_dual)
    } else {
        select_401([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_228([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_229([a, b, d, e, f, c, g, h, i, k, j, l, m], is_dual)
    } else {
        select_367([a, b, d, e, f, g, i, j, k, c, l, m, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_163([e, f, c, d, g, h, i], is_dual)
    } else {
        select_163([e, f, a, b, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_492([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_35([b, c, d, g, f], is_dual)
    } else {
        select_67([a, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_491([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_74([f, a, c, d, e, g, h, i], is_dual)
    } else {
        select_492([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_490([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_491([b, c, d, e, a, f, g, h, j], is_dual)
    } else {
        select_491([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_488([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_489([a, d, e, f, b, c, g, h, i], is_dual)
    } else {
        select_490([b, c, g, e, f, a, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_496([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_37([a, c, b, d, e, g], is_dual)
    } else {
        select_137([b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_497([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([a, c, b, f, g, h], is_dual)
    } else {
        select_49([b, e, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_495([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_496([b, e, c, a, f, g, h], is_dual)
    } else {
        select_497([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_494([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_163([a, f, d, e, g, h, i], is_dual)
    } else {
        select_495([a, b, h, c, f, i, g, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_499([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([a, b, c, d, e, f], is_dual)
    } else {
        select_432([c, d, b, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_498([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, b, d, e, f, g, i], is_dual)
    } else {
        select_499([h, i, a, b, f, c, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_493([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_494([b, a, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_498([b, g, f, d, e, a, h, j, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_487([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_488([a, b, f, e, c, g, h, j, l, i, m], is_dual)
    } else {
        select_493([a, c, d, b, h, f, g, i, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_503([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_219([a, b, f, c, e, g, h, i], is_dual)
    } else {
        select_474([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_505([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([a, e, b, d, g, h], is_dual)
    } else {
        select_21([c, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_504([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([b, e, d, c, f, g, h], is_dual)
    } else {
        select_505([a, b, d, g, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_502([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_503([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_504([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_506([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_191([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_191([a, c, d, e, b, f, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_501([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_502([a, b, d, e, c, f, h, g, j], is_dual)
    } else {
        select_506([d, a, c, b, h, g, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_509([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_33([d, b, c, g, f, e, h], is_dual)
    } else {
        select_266([c, a, e, f, d, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_510([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_33([d, b, c, g, f, e, h], is_dual)
    } else {
        select_134([c, a, e, f, h, d, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_509([c, b, d, e, a, f, g, i, h], is_dual)
    } else {
        select_510([c, a, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_507([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_502([a, b, f, d, c, h, i, g, k], is_dual)
    } else {
        select_508([a, c, e, b, f, g, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_500([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_501([a, c, f, g, d, b, h, i, j, k], is_dual)
    } else {
        select_507([a, c, b, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_486([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_487([a, b, c, d, e, f, g, h, j, i, l, m, k], is_dual)
    } else {
        select_500([a, c, b, h, d, g, f, j, k, l, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_514([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_346([a, b, d, f, g, e, h, j], is_dual)
    } else {
        select_47([a, c, d, e, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_516([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_238([j, g, d, e, h, a, c], !is_dual)
    } else {
        select_238([b, c, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_517([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_105([c, b, d, e, f, g, h], is_dual)
    } else {
        select_30([a, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_516([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_517([b, d, a, e, c, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_513([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_514([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_515([b, a, d, e, c, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_519([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_117([a, c, d, e, b, f, g, i], is_dual)
    } else {
        select_117([a, b, d, e, c, f, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_521([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_187([b, c, d, e, f], is_dual)
    } else {
        select_36([d, f, g, a, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_520([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_521([a, c, g, f, b, h, i], is_dual)
    } else {
        select_160([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_518([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_519([a, c, f, b, e, h, g, i, k], is_dual)
    } else {
        select_520([a, b, d, c, g, f, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_512([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_513([b, a, c, d, g, f, i, j, k, h], is_dual)
    } else {
        select_518([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_525([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_154([a, b, c, f, e, g], is_dual)
    } else {
        select_154([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_524([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_525([a, e, b, d, g, f, h], is_dual)
    } else {
        select_147([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_523([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_524([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_524([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_528([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_48([a, b, d, c, e, f, g], is_dual)
    } else {
        select_26([b, d, f, e, a, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_527([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_528([a, b, f, d, g, e, i], is_dual)
    } else {
        select_528([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_531([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_67([f, h, a, e, c], !is_dual)
    } else {
        select_23([g, d, i, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_530([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_25([c, d, e, h, f, j], is_dual)
    } else {
        select_531([a, c, b, e, i, g, j, k, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_533([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_42([c, d, g, e, f, i, j], is_dual)
    } else {
        select_23([a, b, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_532([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_533([a, d, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_104([a, b, d, h, e, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_529([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_530([c, d, a, e, b, f, g, i, h, k, j], is_dual)
    } else {
        select_532([c, b, e, d, a, f, g, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_526([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_527([a, b, c, e, j, h, g, l, i], is_dual)
    } else {
        select_529([a, b, c, g, d, f, i, j, k, l, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_523([a, b, d, e, h, g, f, i, k, j], is_dual)
    } else {
        select_526([a, b, c, e, d, g, f, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_511([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_512([a, g, c, f, e, b, h, i, k, j, l], is_dual)
    } else {
        select_522([a, c, b, d, e, g, h, f, j, k, i, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_485([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_486([a, b, c, e, d, g, i, h, k, f, l, j, m], is_dual)
    } else {
        select_511([c, a, d, e, b, i, g, f, k, j, h, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_538([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_374([a, d, e, b, c, f, g], is_dual)
    } else {
        select_181([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_541([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_29([a, b, c, d, f, g], is_dual)
    } else {
        select_49([b, d, c, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_542([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_37([g, h, i, d, a, b], !is_dual)
    } else {
        select_38([a, c, f, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_540([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_541([e, g, i, d, b, f, c], !is_dual)
    } else {
        select_542([a, b, c, e, f, d, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_543([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_119([b, d, e, f, a, h], is_dual)
    } else {
        select_61([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_539([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_540([b, a, c, e, d, f, h, g, i], is_dual)
    } else {
        select_543([e, h, d, i, g, a, b, f], !is_dual)
    }
}
/// n = 11, i = 4
fn select_537([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_538([a, b, e, f, g, i, h], is_dual)
    } else {
        select_539([j, k, i, a, b, g, c, d, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_546([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_451([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_216([b, a, d, c, f, g, e, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_503([b, c, e, a, f, d, g, h, i], is_dual)
    } else {
        select_546([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_544([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_382([a, d, e, f, g, b, h], is_dual)
    } else {
        select_545([a, d, c, e, b, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_536([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_537([b, a, d, e, c, f, h, i, k, j, g], is_dual)
    } else {
        select_544([b, c, d, a, f, h, g, k, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_551([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_137([a, g, h], is_dual)
    } else {
        select_42([a, c, b, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_550([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_551([a, e, c, d, f, h, g, i], is_dual)
    } else {
        select_541([a, b, g, d, j, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_552([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_182([a, c, e, d, f, h], is_dual)
    } else {
        select_541([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_549([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_550([b, a, c, e, g, f, i, j, h, k], is_dual)
    } else {
        select_552([b, a, d, e, h, i, k, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_554([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_541([a, b, e, f, d, h, i], is_dual)
    } else {
        select_113([b, c, d, f, g, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_556([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_105([a, b, c, d, g, e, f], is_dual)
    } else {
        select_105([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_557([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_105([b, c, d, e, f, g, h], is_dual)
    } else {
        select_313([a, b, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_555([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_556([c, d, g, f, e, i, h], is_dual)
    } else {
        select_557([a, c, b, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_553([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_554([a, b, f, g, i, h, j, e, k], is_dual)
    } else {
        select_555([a, c, d, b, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_548([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_549([c, b, e, d, a, f, g, h, i, j, k], is_dual)
    } else {
        select_553([c, a, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_560([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_145([a, b, e, f, h, d, g], is_dual)
    } else {
        select_195([a, d, c, g, f, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_559([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_560([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_560([a, b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_561([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_25([b, c, d, f, e, g], is_dual)
    } else {
        select_36([g, f, h, a, d, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_558([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_559([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_561([b, d, a, f, c, g, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_547([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_548([a, b, d, c, e, f, g, i, h, j, k], is_dual)
    } else {
        select_558([a, b, d, h, i, c, k, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_536([a, c, e, d, b, f, h, g, i, k, j], is_dual)
    } else {
        select_547([c, a, e, f, d, g, i, b, j, k, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_565([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_318([a, b, c, d, f, h, i, j], is_dual)
    } else {
        select_117([a, c, d, e, h, f, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_567([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_36([d, b, c, e, f, g], is_dual)
    } else {
        select_206([d, h, i, a, e, g, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_566([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_191([d, e, i, a, g, h, b, f], !is_dual)
    } else {
        select_567([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_564([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_565([b, a, d, c, e, f, g, h, i, k], is_dual)
    } else {
        select_566([h, j, k, a, b, f, c, i, g], !is_dual)
    }
}
/// n = 7, i = 3
fn select_569([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_186([g, c, d, f, e, a], !is_dual)
    } else {
        select_238([a, b, c, f, d, g, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_571([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_119([f, d, g, e, a, b], !is_dual)
    } else {
        select_119([f, e, g, d, a, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_570([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_571([e, g, i, d, f, b, c], !is_dual)
    } else {
        select_47([e, h, i, d, f, g, a, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_568([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_569([b, f, e, g, h, a, d], is_dual)
    } else {
        select_570([b, c, a, e, d, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_563([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_564([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_568([j, h, k, c, a, f, b, d, i], !is_dual)
    }
}
/// n = 9, i = 3
fn select_574([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_123([a, e, c, d, g, f, h, i], is_dual)
    } else {
        select_388([a, b, h, d, i, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_573([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_574([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_574([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_577([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_153([a, c, d, e, f], is_dual)
    } else {
        select_36([a, e, h, b, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_576([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_168([e, f, i, a, b, h], !is_dual)
    } else {
        select_577([e, b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_580([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_35([b, a, c, d, e], is_dual)
    } else {
        select_27([b, e, d, a], is_dual)
    }
}
/// n = 7, i = 3
fn select_579([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_580([a, b, c, d, e], is_dual)
    } else {
        select_432([a, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_578([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_168([d, e, h, a, b, g], !is_dual)
    } else {
        select_579([e, a, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_575([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_576([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_578([a, b, g, e, f, c, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_572([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_573([a, b, c, e, f, g, h, i, j], is_dual)
    } else {
        select_575([a, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_562([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_563([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_572([d, a, b, c, e, f, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_534([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_535([a, b, c, f, e, d, h, i, g, k, j], is_dual)
    } else {
        select_562([a, b, c, f, e, h, g, i, d, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_485([a, b, d, c, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_534([d, i, a, c, b, g, f, j, l, k, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_586([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_90([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_124([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_587([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_383([b, e, c, a, g, f], is_dual)
    } else {
        select_63([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_585([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_586([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_587([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_589([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_106([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_106([c, b, d, e, f, a, g, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_588([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_589([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_589([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_584([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_585([a, d, e, f, b, c, g, h, j, i], is_dual)
    } else {
        select_588([a, d, b, c, e, g, i, k, l, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_594([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, d, e], is_dual)
    } else {
        select_21([a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_593([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_594([f, c, d, e, b, g], is_dual)
    } else {
        select_53([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_592([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_186([b, c, g, f, a, h], is_dual)
    } else {
        select_593([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_591([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_592([c, a, b, e, f, d, g, h], is_dual)
    } else {
        select_520([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_590([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_585([b, a, e, f, c, d, g, h, j, i], is_dual)
    } else {
        select_591([a, b, c, d, e, g, i, k, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_583([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_584([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    } else {
        select_590([d, e, b, c, a, f, g, h, i, k, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_599([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_25([b, a, c, e, d, f], is_dual)
    } else {
        select_169([c, a, e, d, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_598([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_321([g, f, i, a, e, b], !is_dual)
    } else {
        select_599([b, c, f, e, d, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_601([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_254([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_348([e, c, d, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_600([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_236([a, c, b, i, j, f, h], is_dual)
    } else {
        select_601([a, b, d, e, f, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_597([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_598([b, c, g, i, f, j, a, h, k], is_dual)
    } else {
        select_600([a, c, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_603([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_175([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_178([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_604([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_82([a, b, c, e, f, d, g], is_dual)
    } else {
        select_82([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_602([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_603([a, b, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_604([a, d, e, f, b, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_596([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_597([a, c, d, e, f, g, h, i, b, j, k], is_dual)
    } else {
        select_602([a, c, d, b, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_608([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_187([a, c, b, d, e], is_dual)
    } else {
        select_187([b, c, a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_607([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_608([a, b, e, c, f], is_dual)
    } else {
        select_154([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_610([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_416([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_91([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_609([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_610([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_610([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_606([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_607([a, b, c, g, d, h], is_dual)
    } else {
        select_609([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_605([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_588([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_606([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_595([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_596([a, e, d, f, b, c, g, h, i, k, j], is_dual)
    } else {
        select_605([d, a, b, c, e, g, i, j, l, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_582([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_583([a, e, f, d, c, b, g, i, j, h, l, k], is_dual)
    } else {
        select_595([a, e, f, d, b, c, h, i, k, g, l, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_615([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_163([a, b, c, d, f, h, i], is_dual)
    } else {
        select_82([a, c, e, b, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_614([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_615([b, a, d, e, c, f, g, h, j], is_dual)
    } else {
        select_566([g, i, j, a, b, f, c, h, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_617([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_186([c, b, g, e, a, i], is_dual)
    } else {
        select_336([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_619([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_28([c, b, e, f, h, g, j, i], is_dual)
    } else {
        select_497([a, c, g, d, i, h, f, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_620([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_219([b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_497([a, c, f, d, e, g, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_618([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_619([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_620([f, c, b, e, a, h, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_616([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_617([a, b, c, d, h, f, i, j, g], is_dual)
    } else {
        select_618([a, c, b, d, e, f, h, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_613([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_614([f, a, g, c, e, b, i, h, j, k], is_dual)
    } else {
        select_616([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_624([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_25([g, d, a, e, f, b], !is_dual)
    } else {
        select_238([g, f, a, d, e, b, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_625([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_210([i, a, b, e, f, h, j], is_dual)
    } else {
        select_348([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_623([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_624([i, h, e, f, j, a, g], !is_dual)
    } else {
        select_625([b, a, c, d, f, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_628([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([h, i, j, e, a, b], !is_dual)
    } else {
        select_29([a, c, d, g, f, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_627([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_113([i, e, j, h, b, a], !is_dual)
    } else {
        select_628([b, a, c, d, e, f, g, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_626([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_186([a, h, b, e, j, f], is_dual)
    } else {
        select_627([a, f, c, d, e, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_622([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_623([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_626([b, c, d, e, a, f, g, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_630([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_90([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_90([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_629([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_630([a, b, f, c, d, e, g, i, j], is_dual)
    } else {
        select_604([a, c, d, e, b, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_621([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_622([b, c, f, d, e, g, h, i, a, j, k], is_dual)
    } else {
        select_629([c, b, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_612([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_613([a, b, c, d, h, f, g, j, k, l, i], is_dual)
    } else {
        select_621([a, f, c, b, g, e, h, i, k, m, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_636([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_49([a, b, c, d], is_dual)
    } else {
        select_49([a, b, d, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_635([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_187([e, f, a, b, c], !is_dual)
    } else {
        select_636([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, d, e], is_dual)
    } else {
        select_23([a, g, h, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_637([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_154([a, h, i, j, e, b], !is_dual)
    } else {
        select_638([g, h, i, j, a, f, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_634([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_635([a, b, c, f, h, g], is_dual)
    } else {
        select_637([a, d, b, c, e, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_640([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_120([b, c, a, h, j], is_dual)
    } else {
        select_120([e, f, d, g, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_642([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_30([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_641([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, b, c, d, e, g, f], is_dual)
    } else {
        select_642([a, b, c, f, g, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_639([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_640([a, b, c, f, d, e, g, h, i, j], is_dual)
    } else {
        select_641([d, e, g, f, a, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_633([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_634([b, d, e, c, h, a, g, i, j, k], is_dual)
    } else {
        select_639([b, d, e, a, c, f, g, h, j, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_646([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_35([a, b, c, d, e], is_dual)
    } else {
        select_30([e, a, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_645([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_646([g, e, h, a, f, b], !is_dual)
    } else {
        select_438([a, e, c, d, g, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_644([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_645([a, h, i, j, b, f, g, c], !is_dual)
    } else {
        select_155([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_643([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_430([a, f, c, d, b, g, h], is_dual)
    } else {
        select_644([a, b, e, c, d, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_632([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_633([a, b, d, e, f, c, h, g, i, j, k], is_dual)
    } else {
        select_643([a, b, e, f, g, c, i, h, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_650([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_496([a, b, g, f, e, h, j], is_dual)
    } else {
        select_28([b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_649([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_620([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_650([b, c, d, e, a, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_652([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_328([a, e, c, d, f, h, g, i], is_dual)
    } else {
        select_182([a, b, g, d, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_653([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([b, c, d, e, f, g, h], is_dual)
    } else {
        select_113([a, d, g, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_651([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_652([c, d, b, f, g, e, a, i, h], is_dual)
    } else {
        select_653([c, a, b, d, f, g, e, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_648([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_649([a, f, b, c, e, h, i, g, j, k], is_dual)
    } else {
        select_651([b, a, c, d, g, i, f, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_647([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_614([a, f, g, c, e, b, h, i, j, k], is_dual)
    } else {
        select_648([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_631([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_632([f, a, c, d, b, g, h, j, i, l, k], is_dual)
    } else {
        select_647([a, b, c, e, h, f, g, i, k, m, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_611([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_612([a, c, b, e, d, h, i, g, k, f, l, j, m], is_dual)
    } else {
        select_631([a, c, d, b, e, h, i, f, j, k, g, m, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_581([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_582([a, b, h, c, d, i, g, j, l, k, f, m], is_dual)
    } else {
        select_611([a, b, d, c, e, g, f, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_483([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_484([b, d, a, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_581([b, d, c, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_483([a, b, d, c, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_483([a, c, d, b, e, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_227([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_228([a, b, c, d, f, e, g, h, j, k, l, m, i], is_dual)
    } else {
        select_482([a, b, c, d, e, f, g, h, j, i, l, k, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_226([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_227([f, b, c, d, e, a, i, j, h, g, l, m, k], is_dual)
    } else {
        select_227([f, b, c, e, d, a, i, j, h, g, k, m, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_225([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_226([a, b, c, d, e, f, h, i, j, k, l, g, m], is_dual)
    } else {
        select_226([a, b, c, d, g, f, h, i, j, k, l, e, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_224([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_225([h, a, f, g, b, c, d, l, k, e, i, j, m], is_dual)
    } else {
        select_225([h, e, f, g, b, c, d, l, k, a, i, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_223([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_224([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_224([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_222([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_223([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_223([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_667([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_46([e, a, d, c, g, f], is_dual)
    } else {
        select_28([g, h, d, i, e, f, b, a], !is_dual)
    }
}
/// n = 8, i = 3
fn select_668([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_25([b, a, d, f, g, h], is_dual)
    } else {
        select_580([a, c, d, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_666([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_667([c, b, d, e, a, g, f, h, i], is_dual)
    } else {
        select_668([c, a, d, b, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_670([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_113([g, h, f, a, b, c], !is_dual)
    } else {
        select_195([f, h, g, a, b, e, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_669([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_24([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_670([i, f, j, h, e, a, b, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_665([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_666([e, f, c, d, a, h, g, i, k], is_dual)
    } else {
        select_669([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_673([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_25([b, f, d, e, a, g], is_dual)
    } else {
        select_348([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_672([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_673([a, c, b, f, h, g, e], is_dual)
    } else {
        select_39([b, c, d, a, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_676([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_29([g, h, i, a, d, b], !is_dual)
    } else {
        select_26([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_675([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_676([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_29([a, h, i, f, e, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_677([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_187([g, e, b, a, f], !is_dual)
    } else {
        select_77([b, a, c, d, e, g, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_674([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_675([f, i, k, g, j, a, b, d, h], !is_dual)
    } else {
        select_677([b, c, e, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_671([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_672([c, d, g, e, f, b, h, i], is_dual)
    } else {
        select_674([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_664([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_665([f, i, k, l, g, a, j, b, c, d, h], !is_dual)
    } else {
        select_671([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_681([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_219([e, i, j, k, d, h, a, g], !is_dual)
    } else {
        select_154([e, f, b, c, g, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_683([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_49([a, g, i, h], !is_dual)
    }
}
/// n = 11, i = 5
fn select_682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_580([j, i, k, a, b], !is_dual)
    } else {
        select_683([b, c, d, e, g, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_680([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_681([i, k, l, g, a, f, j, h, c, d, e], !is_dual)
    } else {
        select_682([f, i, k, l, g, j, a, h, b, c, e], !is_dual)
    }
}
/// n = 12, i = 5
fn select_679([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_672([c, d, g, e, f, b, h, i], is_dual)
    } else {
        select_680([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_686([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_77([a, c, d, e, f, g, h], is_dual)
    } else {
        select_85([b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_685([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_686([f, a, e, c, d, g, h, j], is_dual)
    } else {
        select_670([a, h, b, e, g, f, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_687([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_441([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_388([f, b, c, a, e, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_684([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_685([b, c, d, e, a, f, g, h, j, i], is_dual)
    } else {
        select_687([f, k, g, l, a, h, i, b], !is_dual)
    }
}
/// n = 13, i = 5
fn select_678([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_679([a, b, c, d, h, f, g, i, j, k, l, m], is_dual)
    } else {
        select_684([a, b, c, e, i, f, g, h, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_664([a, h, b, c, d, g, j, i, f, k, l, m], is_dual)
    } else {
        select_678([a, b, c, d, e, g, f, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_662([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_663([a, c, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    } else {
        select_663([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_693([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_432([a, g, f, h, i], is_dual)
    } else {
        select_83([a, d, b, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_694([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_77([a, d, b, c, e, f, g], is_dual)
    } else {
        select_27([d, f, e, b], is_dual)
    }
}
/// n = 10, i = 4
fn select_692([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_693([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_694([e, i, j, g, a, b, h], !is_dual)
    }
}
/// n = 11, i = 4
fn select_691([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_692([g, a, b, d, e, h, f, i, j, k], is_dual)
    } else {
        select_692([f, a, c, d, e, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_691([b, a, c, d, g, e, f, h, i, j, k], is_dual)
    } else {
        select_288([f, j, e, k, l, g, a, b, i], !is_dual)
    }
}
/// n = 7, i = 3
fn select_698([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_145([a, b, c, d, f, e, g], is_dual)
    } else {
        select_29([d, f, g, a, b, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_699([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_33([a, b, c, d, f, e, g], is_dual)
    } else {
        select_28([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_697([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_698([a, b, f, d, g, h, e], is_dual)
    } else {
        select_699([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_701([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_77([b, a, c, d, e, g, f], is_dual)
    } else {
        select_113([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_700([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_701([a, b, d, f, e, g, h, i], is_dual)
    } else {
        select_699([e, h, j, a, f, i, b, c, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_696([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_697([e, f, j, a, h, i, g, b, c], !is_dual)
    } else {
        select_700([a, c, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_704([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([a, e, b, d, g, f], is_dual)
    } else {
        select_23([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_703([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_240([a, c, b, e, f, g, d, h], is_dual)
    } else {
        select_704([b, c, a, e, d, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_702([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_672([c, b, a, d, f, e, h, g], is_dual)
    } else {
        select_703([b, c, g, h, a, f, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_695([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_696([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_702([b, d, a, e, f, c, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_689([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_690([a, b, c, d, e, g, f, h, i, j, l, k], is_dual)
    } else {
        select_695([g, e, j, l, f, a, k, b, i, h], !is_dual)
    }
}
/// n = 12, i = 5
fn select_688([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_689([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_689([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_661([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < c) || (is_dual && k > c) {
        select_662([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_688([a, b, h, d, e, j, g, i, k, c, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_709([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_430([b, f, d, e, a, g, h], is_dual)
    } else {
        select_520([b, a, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_712([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_153([b, c, d, e, f], is_dual)
    } else {
        select_67([e, g, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_711([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_247([e, j, i, h, g, a, b, c, f], !is_dual)
    } else {
        select_712([c, a, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_714([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_46([h, e, f, d, g, a], !is_dual)
    } else {
        select_119([d, b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_713([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_443([g, h, f, i, a, d, e, b], !is_dual)
    } else {
        select_714([b, a, c, d, f, h, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_710([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_711([c, a, b, d, e, f, g, i, h, j], is_dual)
    } else {
        select_713([e, g, j, h, i, a, c, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_708([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_709([c, d, b, a, e, f, h, g, i], is_dual)
    } else {
        select_710([b, c, d, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_707([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_708([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_708([a, b, c, e, d, f, g, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_719([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_25([e, b, a, g, d, f], is_dual)
    } else {
        select_105([b, a, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_718([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_315([a, b, c, e, f, d, g, h], is_dual)
    } else {
        select_719([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_717([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_718([a, b, f, d, e, h, i, g], is_dual)
    } else {
        select_525([a, d, c, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_716([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_717([a, b, d, e, c, g, f, h, i], is_dual)
    } else {
        select_717([a, c, d, f, b, g, e, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_723([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_187([a, f, e, b, c], !is_dual)
    } else {
        select_46([a, b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_722([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_723([a, d, e, f, h, g], is_dual)
    } else {
        select_261([a, f, j, b, i, c, g], !is_dual)
    }
}
/// n = 8, i = 2
fn select_725([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_416([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_25([g, b, a, h, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_724([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_725([a, d, b, c, e, g, f, h], is_dual)
    } else {
        select_383([a, b, c, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_721([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_722([a, b, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_724([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_720([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_721([a, b, c, e, d, g, h, i, f, j], is_dual)
    } else {
        select_721([a, b, d, e, c, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_715([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_716([a, c, d, b, g, h, f, e, i], is_dual)
    } else {
        select_720([a, b, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_706([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_707([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_715([a, b, e, f, d, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_732([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_42([b, d, c, f, e, h, g], is_dual)
    } else {
        select_23([a, i, j, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_731([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_33([e, c, d, b, f, g, h], is_dual)
    } else {
        select_732([a, b, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_734([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_105([a, b, c, d, e, f, g], is_dual)
    } else {
        select_29([d, b, c, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_733([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_580([b, c, d, g, f], is_dual)
    } else {
        select_734([g, i, j, e, a, h, b, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_730([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_731([g, h, j, k, d, i, a, e, b, f], !is_dual)
    } else {
        select_733([g, h, j, k, d, i, a, f, c, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_736([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_74([f, h, g, i, a, d, b, e], !is_dual)
    } else {
        select_195([a, d, c, g, f, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_735([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_624([a, b, e, g, h, i, d], is_dual)
    } else {
        select_736([b, a, c, d, e, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_729([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_730([a, b, c, d, e, f, g, i, h, k, j], is_dual)
    } else {
        select_735([a, b, f, d, h, g, i, j, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_739([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_113([h, d, i, f, g, a], !is_dual)
    } else {
        select_248([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_738([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_701([a, b, d, f, e, g, i, j], is_dual)
    } else {
        select_739([h, i, j, e, a, b, f, c, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_741([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_328([b, d, a, e, c, g, f, h], is_dual)
    } else {
        select_323([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_740([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_186([c, b, f, e, a, h], is_dual)
    } else {
        select_741([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_737([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_738([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_740([c, g, b, a, h, e, f, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_728([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_729([b, a, c, e, d, f, g, h, i, k, j], is_dual)
    } else {
        select_737([g, i, k, d, e, h, j, a, b, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_745([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_33([b, d, c, f, e, g, h], is_dual)
    } else {
        select_195([a, d, b, h, f, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_744([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_745([a, b, d, c, f, g, e, h, i], is_dual)
    } else {
        select_135([a, e, d, b, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_747([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_238([j, i, a, g, h, b, e], !is_dual)
    } else {
        select_206([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_746([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_554([g, i, j, a, h, b, e, f, d], !is_dual)
    } else {
        select_747([b, a, c, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_743([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_744([a, c, b, e, d, f, g, h, i], is_dual)
    } else {
        select_746([a, c, d, e, b, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_750([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_28([f, g, h, i, a, e, d, c], !is_dual)
    } else {
        select_556([c, a, b, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_749([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_750([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_24([a, g, h, i, d, e, f, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_748([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_615([e, d, i, h, k, j, a, g, c], !is_dual)
    } else {
        select_749([d, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_742([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_743([d, e, i, j, k, f, a, b, h, g], !is_dual)
    } else {
        select_748([a, b, c, d, e, f, g, i, k, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_727([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_728([c, b, d, f, a, g, e, h, j, i, k], is_dual)
    } else {
        select_742([c, a, d, f, e, b, g, h, j, k, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_726([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_727([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_727([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_705([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_706([a, b, f, j, c, k, h, i, d, g], !is_dual)
    } else {
        select_726([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_660([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_661([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_705([a, b, c, h, i, j, k, l, d, m, g], is_dual)
    }
}
/// n = 13, i = 5
fn select_659([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_660([a, b, c, d, f, g, e, h, i, j, k, l, m], is_dual)
    } else {
        select_660([a, b, c, e, f, g, d, h, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_758([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_82([a, d, e, b, c, f, g], is_dual)
    } else {
        select_84([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_757([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_758([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_758([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_761([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_64([a, b, c, d, e, f, h], is_dual)
    } else {
        select_187([g, h, e, a, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_763([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_29([a, c, d, e, f, h], is_dual)
    } else {
        select_29([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_762([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_474([e, b, c, d, f, g, h], is_dual)
    } else {
        select_763([a, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_760([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_761([b, c, g, e, f, a, h, i], is_dual)
    } else {
        select_762([b, c, a, d, e, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_765([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_438([b, e, c, d, g, f], is_dual)
    } else {
        select_34([e, g, h, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_764([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_186([b, c, f, e, a, g], is_dual)
    } else {
        select_765([b, c, a, d, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_759([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_760([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_764([a, b, c, g, e, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_756([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_757([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_759([b, c, d, e, f, g, h, a, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_768([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_413([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_413([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_767([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_768([a, b, f, i, j, k, h, c, d, g], !is_dual)
    } else {
        select_397([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_771([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_154([a, g, i, j, h, f], !is_dual)
    } else {
        select_83([a, d, b, c, e, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_124([a, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_771([a, b, i, c, f, h, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_769([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_770([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_770([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_766([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_767([a, g, k, l, m, b, i, j, d, e, h], !is_dual)
    } else {
        select_769([a, b, c, f, d, e, g, h, i, j, k, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_755([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_756([a, k, l, m, g, c, i, b, j], !is_dual)
    } else {
        select_766([a, c, b, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_754([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_755([a, b, d, e, f, g, c, h, i, j, k, l, m], is_dual)
    } else {
        select_755([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_776([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_374([e, a, b, d, f, h, i], is_dual)
    } else {
        select_297([a, b, c, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_775([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_81([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_776([c, d, e, b, f, g, a, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_774([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_775([a, c, d, e, i, b, h, j, k], is_dual)
    } else {
        select_88([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_773([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_79([a, b, e, f, g, c, d, h, i, j], is_dual)
    } else {
        select_774([a, b, e, c, d, f, g, h, j, i, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_782([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([a, b, c, d, e, f], is_dual)
    } else {
        select_23([e, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_781([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_473([a, c, f, d, e, h, g, j], is_dual)
    } else {
        select_782([b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_783([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_46([a, c, d, f, e, h], is_dual)
    } else {
        select_46([b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_780([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_781([c, d, e, a, b, f, h, g, j, i], is_dual)
    } else {
        select_783([c, f, a, b, h, i, j, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_785([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_187([a, f, d, g, h], is_dual)
    } else {
        select_348([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_786([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_48([a, b, c, f, e, g, h], is_dual)
    } else {
        select_646([c, d, g, e, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_784([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_785([h, g, j, e, f, i, a, b], !is_dual)
    } else {
        select_786([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_779([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_780([b, c, d, e, a, f, h, g, i, j], is_dual)
    } else {
        select_784([b, c, e, g, f, a, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_789([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_136([h, j, g, i, a, b, c], !is_dual)
    } else {
        select_154([a, f, d, e, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_790([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_105([a, c, d, b, e, f, g], is_dual)
    } else {
        select_105([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_788([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_789([a, b, c, d, f, g, h, i, j, k], is_dual)
    } else {
        select_790([b, c, e, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_792([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_77([a, c, b, h, f, g, i], is_dual)
    } else {
        select_594([c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_791([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_300([c, a, b, f, h, i, g], is_dual)
    } else {
        select_792([c, d, e, a, b, f, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_787([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_788([a, b, c, g, e, h, i, f, j, k, l], is_dual)
    } else {
        select_791([b, c, d, f, e, h, i, g, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_779([a, b, c, d, h, j, g, i, f, k, l], is_dual)
    } else {
        select_787([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_778([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_778([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_772([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_773([a, b, d, e, f, c, g, h, j, i, k], is_dual)
    } else {
        select_777([a, b, d, e, f, g, h, i, j, c, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_753([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_754([a, b, c, d, e, f, g, i, j, k, l, h, m], is_dual)
    } else {
        select_772([a, b, c, e, f, d, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_752([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_753([a, b, c, d, f, g, h, e, i, j, k, l, m], is_dual)
    } else {
        select_753([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_798([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_163([a, c, b, d, e, f, g], is_dual)
    } else {
        select_723([a, b, g, h, c, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_797([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_397([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_798([a, b, c, d, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_796([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_797([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_797([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_801([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_117([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_144([a, f, e, i, b, h, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_800([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_801([a, c, d, e, b, f, g, h, j], is_dual)
    } else {
        select_801([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_803([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_673([b, c, a, e, f, d, g], is_dual)
    } else {
        select_360([a, c, b, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_805([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_52([a, b, c, d, f, g, h], is_dual)
    } else {
        select_52([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_804([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_186([a, b, h, f, i, g], is_dual)
    } else {
        select_805([a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_802([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_803([b, c, h, f, a, i, g, j], is_dual)
    } else {
        select_804([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_799([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_800([a, g, i, f, l, b, k, j, c, h], !is_dual)
    } else {
        select_802([a, c, b, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_795([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_796([a, i, f, g, l, b, c, k, j], !is_dual)
    } else {
        select_799([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_794([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_795([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_795([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_793([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_794([a, b, c, d, f, g, e, h, i, j, k, l], is_dual)
    } else {
        select_794([a, b, c, e, f, g, d, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_751([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_752([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_793([a, b, c, d, e, g, i, k, l, f, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_658([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_659([a, b, c, d, e, h, g, i, j, f, k, l, m], is_dual)
    } else {
        select_751([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_814([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_374([a, c, e, d, g, f, i], is_dual)
    } else {
        select_528([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_813([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_814([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_801([a, i, e, k, f, b, j, h, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_817([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_312([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_237([g, i, e, h, a, c, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_818([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_238([g, i, a, h, e, b, c], !is_dual)
    } else {
        select_244([a, b, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_816([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_817([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_818([a, c, b, d, f, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_815([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_697([b, c, g, e, f, h, a, i, j], is_dual)
    } else {
        select_816([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_812([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_813([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_815([a, e, f, k, b, h, j, i, c, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_811([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_812([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_812([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_820([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_797([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_797([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_819([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_820([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_820([a, b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_810([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_811([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_819([a, b, c, h, f, k, d, j, i], !is_dual)
    }
}
/// n = 11, i = 5
fn select_809([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_810([a, b, c, d, f, e, g, h, i, j, k], is_dual)
    } else {
        select_810([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_827([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_154([a, f, i, j, h, g], !is_dual)
    } else {
        select_83([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_826([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_827([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_827([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_825([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_397([a, b, i, f, k, c, j, h], !is_dual)
    } else {
        select_826([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_824([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_825([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    } else {
        select_825([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_823([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_824([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_824([a, b, c, d, e, f, g, i, k, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_822([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_823([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_823([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_833([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_153([a, b, c, d, e], is_dual)
    } else {
        select_153([a, b, f, g, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_832([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_833([a, g, b, c, h, i, j], is_dual)
    } else {
        select_82([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_834([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_124([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_124([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_831([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_832([a, b, c, d, i, g, h, j, k, l], is_dual)
    } else {
        select_834([a, b, c, e, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_835([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_300([a, b, g, c, e, f, h], is_dual)
    } else {
        select_300([a, b, f, d, e, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_830([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_831([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_835([g, k, h, m, a, b, l, j], !is_dual)
    }
}
/// n = 13, i = 5
fn select_829([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_830([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_830([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_828([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_829([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_829([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_821([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_822([a, b, c, d, f, i, j, e, k, l, m], is_dual)
    } else {
        select_828([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_808([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_809([a, b, c, d, e, i, j, k, f, l, m], is_dual)
    } else {
        select_821([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 5
fn select_836([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_809([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_809([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_808([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_836([a, b, c, d, e, i, j, k, l, f, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_845([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_251([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_704([a, b, f, e, h, g, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_844([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_304([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_845([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_846([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_118([a, e, h, b, f, g, c], !is_dual)
    } else {
        select_147([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_843([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_844([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_846([e, a, g, c, d, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_842([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_843([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_843([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_850([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_240([a, b, f, e, h, g, d, i], is_dual)
    } else {
        select_251([b, a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_849([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_845([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_850([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_848([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_846([a, f, b, c, h, e, i, g], is_dual)
    } else {
        select_849([a, b, d, c, f, e, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_852([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_236([a, h, g, d, f, e, c], !is_dual)
    } else {
        select_560([a, c, b, e, f, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_854([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_195([a, e, b, g, h, d, f], is_dual)
    } else {
        select_195([a, d, c, f, h, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_853([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_168([a, f, g, h, d, e], !is_dual)
    } else {
        select_854([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_851([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_852([a, b, d, e, f, c, h, g], is_dual)
    } else {
        select_853([a, c, d, b, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_847([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_848([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_851([a, g, b, j, e, i, h, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_841([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_842([a, b, c, e, f, g, d, i, h], is_dual)
    } else {
        select_847([a, b, d, e, c, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_840([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_841([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_841([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_860([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_541([f, h, d, a, b, g, e], !is_dual)
    } else {
        select_244([a, d, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_859([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_423([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_860([a, b, c, d, f, e, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_862([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_46([d, a, f, g, b, e], !is_dual)
    } else {
        select_29([b, a, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_861([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_166([a, b, g, d, i, j, f], is_dual)
    } else {
        select_862([d, f, c, h, e, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_858([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_859([d, e, c, g, a, f, h, j, i], is_dual)
    } else {
        select_861([b, a, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_857([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_858([c, a, d, e, b, g, f, h, i, j], is_dual)
    } else {
        select_858([c, b, d, f, a, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_856([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_857([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_857([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_855([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_856([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_856([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_839([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_840([a, b, c, d, e, k, j, h, i, g], !is_dual)
    } else {
        select_855([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_838([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_839([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_839([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_868([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_448([k, h, l, f, j, e, a, b, c, i], !is_dual)
    } else {
        select_116([a, b, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_871([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_240([d, h, i, e, g, a, b, f], !is_dual)
    } else {
        select_29([a, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_870([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_871([g, d, k, a, e, j, h, c, f], !is_dual)
    } else {
        select_322([b, c, a, e, f, d, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_869([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_870([g, i, k, d, e, j, a, h, b, f, c], !is_dual)
    } else {
        select_870([g, j, k, d, e, i, a, h, b, f, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_867([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_868([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_869([b, c, g, e, f, i, h, j, a, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_873([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_435([e, c, g, a, f, d, i, h, j], is_dual)
    } else {
        select_861([a, b, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_876([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_104([a, e, b, g, i, f, h, d], is_dual)
    } else {
        select_469([h, j, i, f, g, a, d, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_878([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_29([a, b, c, e, h, i], is_dual)
    } else {
        select_35([d, b, c, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_877([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_104([h, i, j, e, a, f, b, g], !is_dual)
    } else {
        select_878([b, c, d, a, f, e, h, g, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_875([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_876([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_877([i, h, d, k, g, f, j, a, e, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_874([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_875([a, b, c, d, f, e, h, g, i, j, k], is_dual)
    } else {
        select_435([e, c, g, a, f, d, j, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_872([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_873([a, c, d, b, e, f, g, h, j, i], is_dual)
    } else {
        select_874([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_866([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_867([a, b, c, d, f, e, g, h, i, j, l, k], is_dual)
    } else {
        select_872([a, b, c, g, f, i, h, j, l, k, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_882([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_24([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_883([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_71([a, b, c, g, e, f, h, i], is_dual)
    } else {
        select_71([a, b, d, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_881([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_882([a, e, j, f, k, b, h, g], !is_dual)
    } else {
        select_883([a, c, b, d, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_880([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_881([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_544([a, i, j, e, k, f, b, c, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_887([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_638([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_638([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_886([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_887([a, f, e, i, j, h, b, g], !is_dual)
    } else {
        select_117([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_885([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_397([a, f, i, e, k, b, j, h], !is_dual)
    } else {
        select_886([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_884([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_813([a, b, c, h, e, f, i, g, j, k, l], is_dual)
    } else {
        select_885([a, e, j, l, b, f, k, i, g, h, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_879([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_880([a, b, c, h, e, g, i, j, k, l, f], is_dual)
    } else {
        select_884([a, b, c, d, e, g, f, h, i, l, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_865([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_866([a, b, d, e, f, c, h, g, i, j, k, l], is_dual)
    } else {
        select_879([a, b, c, e, f, g, d, h, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_864([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_865([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_865([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_863([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_864([a, b, c, d, f, g, e, h, i, j, k, l], is_dual)
    } else {
        select_864([a, b, c, e, f, g, d, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_837([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_838([a, b, c, d, e, h, i, j, k, f, l], is_dual)
    } else {
        select_863([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_807([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_837([a, b, c, d, e, f, i, g, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_657([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_658([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_806([a, b, c, d, e, f, g, h, j, k, l, m, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_898([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([b, c, d, e, f, g, h], is_dual)
    } else {
        select_167([e, i, j, a, f, g, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_899([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_145([a, b, g, d, j, e, i], is_dual)
    } else {
        select_469([a, e, c, d, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_897([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_898([b, a, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_899([b, d, c, e, g, f, a, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_896([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_897([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_740([c, g, a, b, h, e, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_895([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_896([a, b, c, d, f, g, h, e, i, j], is_dual)
    } else {
        select_290([b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_903([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_330([b, c, d, e, f, h, g, j], is_dual)
    } else {
        select_195([g, h, i, a, c, f, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_902([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_903([b, a, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_818([a, b, c, d, e, g, f, h, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_906([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_23([a, b, c, d], is_dual)
    } else {
        select_23([a, b, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_905([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_119([a, c, e, d, f, g], is_dual)
    } else {
        select_906([d, b, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_904([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_905([a, c, b, d, e, g, f, h], is_dual)
    } else {
        select_47([d, g, h, i, a, e, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_901([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_902([a, b, c, d, f, e, g, i, h, j], is_dual)
    } else {
        select_904([f, e, j, a, i, h, b, c, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_900([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_901([a, b, d, c, e, f, h, j, g, i], is_dual)
    } else {
        select_901([a, c, d, b, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_894([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_895([b, c, d, a, f, g, h, e, i, j], is_dual)
    } else {
        select_900([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_893([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_894([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_894([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_911([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_191([a, d, c, e, g, f, h, i], is_dual)
    } else {
        select_528([e, b, d, c, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_910([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_911([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_911([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_913([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_318([a, b, c, d, f, e, g, i], is_dual)
    } else {
        select_318([a, b, h, i, g, e, f, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_914([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_191([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_191([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_912([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_913([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_914([a, b, e, i, g, h, c, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_909([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_910([a, b, c, e, d, g, f, h, i], is_dual)
    } else {
        select_912([a, b, f, e, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_915([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_813([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_813([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_908([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_909([a, b, c, f, k, i, j, d, h, g], !is_dual)
    } else {
        select_915([a, b, c, i, k, f, j, h, d, g, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_917([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_396([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_396([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_918([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_885([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_885([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_916([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_917([a, b, c, f, i, k, j, g, h, e], !is_dual)
    } else {
        select_918([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_907([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_908([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_916([a, b, c, d, e, f, g, h, k, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_892([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_893([a, b, c, f, k, j, d, i, g, h], !is_dual)
    } else {
        select_907([a, b, c, d, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_891([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_892([a, b, c, d, f, e, g, h, i, j, k], is_dual)
    } else {
        select_892([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_890([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_838([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    } else {
        select_891([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_889([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_890([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_890([a, b, c, d, e, f, g, i, k, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_927([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, e, c, d, f, g, i], is_dual)
    } else {
        select_195([a, b, g, e, h, f, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_929([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_40([a, c, d, b, e, f], is_dual)
    } else {
        select_40([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_928([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_363([a, c, b, d, f, g, e, h, i], is_dual)
    } else {
        select_929([a, f, d, e, g, b], is_dual)
    }
}
/// n = 10, i = 4
fn select_926([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_927([a, d, b, c, e, g, h, f, j], is_dual)
    } else {
        select_928([b, c, d, a, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_932([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_119([f, b, d, e, h, i], is_dual)
    } else {
        select_141([a, e, c, d, g, f, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_933([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_312([a, b, c, d, e, f, g, i], is_dual)
    } else {
        select_119([b, c, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_931([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_932([a, b, c, e, d, f, g, h, j, i], is_dual)
    } else {
        select_933([a, f, b, e, d, h, i, j, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_934([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_561([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_135([a, c, d, b, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_930([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_931([a, c, b, f, e, g, h, d, i, j], is_dual)
    } else {
        select_934([a, d, g, c, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_925([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_926([b, a, c, d, e, f, h, g, j, i], is_dual)
    } else {
        select_930([b, d, c, e, f, g, h, a, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_937([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_168([a, c, d, b, e, f], is_dual)
    } else {
        select_168([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_938([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_383([a, b, c, d, e, f], is_dual)
    } else {
        select_154([a, b, c, g, f, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_936([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_937([a, b, e, h, c, g], !is_dual)
    } else {
        select_938([a, b, e, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_941([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_74([f, d, h, i, e, g, a, c], !is_dual)
    } else {
        select_134([f, h, i, e, d, g, a, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_940([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_941([b, c, a, d, e, g, f, h, i], is_dual)
    } else {
        select_783([d, g, h, i, a, e, f, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_939([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_940([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_940([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_935([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_936([a, b, g, i, e, f, h, c], !is_dual)
    } else {
        select_939([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_924([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_925([a, c, b, d, f, g, e, h, i, j], is_dual)
    } else {
        select_935([a, f, e, j, g, i, c, b, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_946([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_541([g, f, h, a, e, d, b], !is_dual)
    } else {
        select_195([f, g, h, a, d, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_945([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_70([a, c, b, d, f, e, g], is_dual)
    } else {
        select_946([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_944([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_945([a, f, e, i, b, h, c, g], !is_dual)
    } else {
        select_146([a, c, d, b, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_943([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_295([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_944([g, f, i, j, a, b, c, h, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_942([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_943([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_943([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_923([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_924([a, c, b, d, f, g, e, h, i, j], is_dual)
    } else {
        select_942([a, g, f, i, k, e, b, j, c, h], !is_dual)
    }
}
/// n = 11, i = 5
fn select_922([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_923([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_923([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_948([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_394([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_394([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_947([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_948([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_917([a, b, c, d, f, g, h, i, e, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_921([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_922([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_947([a, b, c, g, i, k, f, d, j, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_955([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_409([a, c, d, e, b, g, f, h, i], is_dual)
    } else {
        select_347([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_954([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_955([b, c, a, d, e, g, h, i, f], is_dual)
    } else {
        select_406([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_953([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_573([a, b, c, e, f, g, h, d, i], is_dual)
    } else {
        select_954([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_958([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_71([f, a, c, d, e, h, j, g], is_dual)
    } else {
        select_71([e, b, c, d, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_957([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_379([h, f, e, j, k, a, i, g, b], !is_dual)
    } else {
        select_958([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_961([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_474([a, f, c, d, e, g, h], is_dual)
    } else {
        select_59([b, e, c, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_960([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_441([f, i, e, j, a, b, h, g], !is_dual)
    } else {
        select_961([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_959([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_576([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_960([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_956([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_957([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_959([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_952([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_953([e, j, k, i, f, a, b, h, g], !is_dual)
    } else {
        select_956([a, b, c, d, e, f, h, j, k, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_967([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_67([b, c, f, g, e], is_dual)
    } else {
        select_49([h, i, d, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_966([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_169([a, b, c, i, j], is_dual)
    } else {
        select_967([h, i, j, g, f, b, c, d, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_968([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_34([c, d, e, f, h, i], is_dual)
    } else {
        select_49([a, b, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_965([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_966([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_968([d, e, b, c, g, f, a, i, j, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_970([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_636([a, b, c, e], is_dual)
    } else {
        select_49([b, c, d, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_969([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_71([a, b, c, d, e, g, h, i], is_dual)
    } else {
        select_970([c, d, e, a, f], is_dual)
    }
}
/// n = 11, i = 5
fn select_964([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_965([e, f, i, j, k, g, h, a, b, c], !is_dual)
    } else {
        select_969([e, g, j, k, f, a, h, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_963([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_964([b, a, c, d, f, e, g, h, k, i, j], is_dual)
    } else {
        select_378([f, i, j, k, e, g, a, b, h], !is_dual)
    }
}
/// n = 9, i = 4
fn select_974([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_551([h, i, f, g, d, b, a, e], !is_dual)
    } else {
        select_104([b, a, c, d, f, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_975([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_36([a, e, d, h, g, j], is_dual)
    } else {
        select_141([a, b, c, f, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_973([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_974([i, j, h, g, a, b, e, c, f], !is_dual)
    } else {
        select_975([b, c, a, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_977([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_219([a, b, f, c, e, h, g, i], is_dual)
    } else {
        select_61([b, e, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_976([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_186([b, f, g, e, a, i], is_dual)
    } else {
        select_977([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_972([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_973([a, c, b, d, f, g, h, e, j, i], is_dual)
    } else {
        select_976([b, a, c, d, e, g, h, f, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_971([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_309([a, b, d, c, f, g, e, i, h, j], is_dual)
    } else {
        select_972([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_962([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_963([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_971([e, f, i, j, k, a, b, h, g, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_951([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_952([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_962([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_983([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_104([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_451([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_984([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_46([b, a, e, d, g, f], is_dual)
    } else {
        select_37([b, c, a, f, g, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_982([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_983([e, a, b, c, f, h, i, j], is_dual)
    } else {
        select_984([b, c, d, f, e, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_986([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_541([b, a, e, g, h, d, f], is_dual)
    } else {
        select_113([b, d, c, f, g, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_985([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_986([c, b, a, f, d, h, g, e], is_dual)
    } else {
        select_736([a, c, b, f, e, g, h, d, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_981([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_982([h, j, i, k, f, d, e, g, a, c], !is_dual)
    } else {
        select_985([d, b, c, h, f, g, e, i, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_989([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_187([h, d, a, b, f], !is_dual)
    } else {
        select_676([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_988([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_989([a, b, c, e, f, g, d, h, i], is_dual)
    } else {
        select_135([b, a, c, d, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_991([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_187([a, b, g, d, h], is_dual)
    } else {
        select_38([a, c, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_990([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_991([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_392([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_987([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_988([d, a, c, g, e, f, i, h, k], is_dual)
    } else {
        select_990([i, j, k, g, e, d, a, h, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_980([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_981([b, a, c, e, d, f, g, h, j, i, k], is_dual)
    } else {
        select_987([a, b, c, e, f, g, h, i, j, d, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_995([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_33([d, b, c, g, f, e, h], is_dual)
    } else {
        select_104([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_994([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_736([a, b, f, e, h, i, g, d, j], is_dual)
    } else {
        select_995([b, a, c, e, d, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_997([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_145([a, c, e, d, h, f, i], is_dual)
    } else {
        select_74([d, g, h, i, a, e, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_996([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_191([a, e, c, d, g, f, i, h], is_dual)
    } else {
        select_997([a, b, f, d, e, h, j, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_993([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_994([a, b, c, d, g, e, h, f, i, j], is_dual)
    } else {
        select_996([a, b, c, d, g, f, h, e, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_999([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_723([a, c, e, d, f, g], is_dual)
    } else {
        select_862([a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_1001([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_328([g, i, h, d, e, a, f, b], !is_dual)
    } else {
        select_551([a, c, b, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1000([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1001([b, a, c, f, d, g, h, e, i], is_dual)
    } else {
        select_552([d, h, i, e, f, g, a, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_998([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_999([d, b, c, e, f, i, g, h], is_dual)
    } else {
        select_1000([a, b, g, d, h, f, e, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_992([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_993([a, c, b, d, f, g, h, i, e, j], is_dual)
    } else {
        select_998([a, b, c, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_979([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_980([c, b, d, f, a, g, h, e, i, j, k], is_dual)
    } else {
        select_992([c, a, d, f, g, e, b, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_1006([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_153([a, c, d, e, f], is_dual)
    } else {
        select_906([a, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1005([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_571([b, c, f, e, a, g, h], is_dual)
    } else {
        select_1006([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_1008([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_119([b, a, c, d, e, g], is_dual)
    } else {
        select_253([e, a, b, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_1007([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_186([a, c, d, b, e, f], is_dual)
    } else {
        select_1008([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1004([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1005([a, b, d, e, g, f, h, i], is_dual)
    } else {
        select_1007([a, b, c, d, h, g, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_1003([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1004([e, i, j, k, g, f, a, b, h], !is_dual)
    } else {
        select_687([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1012([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_646([f, d, e, a, g, i], is_dual)
    } else {
        select_473([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1011([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_374([a, d, e, f, c, g, h], is_dual)
    } else {
        select_1012([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1015([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_35([b, c, d, e, f], is_dual)
    } else {
        select_23([a, g, h, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1014([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1015([a, g, d, e, f, h, j, k], is_dual)
    } else {
        select_473([b, c, f, d, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1016([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_77([b, c, d, e, f, g, i], is_dual)
    } else {
        select_167([e, h, j, a, f, g, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1013([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1014([a, c, d, b, e, f, h, g, j, i, k], is_dual)
    } else {
        select_1016([a, c, g, d, f, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1010([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1011([b, a, d, c, g, f, i, k, h], is_dual)
    } else {
        select_1013([a, c, b, d, e, f, g, h, j, k, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1019([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_312([a, b, d, e, g, i, h, j], is_dual)
    } else {
        select_305([a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1018([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1019([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_818([a, b, f, d, h, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1017([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1018([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_116([a, b, f, d, e, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1009([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1010([c, b, d, a, e, g, h, f, i, j, k], is_dual)
    } else {
        select_1017([c, d, b, e, g, f, h, a, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1002([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_1003([i, j, l, f, a, g, b, k, c, d, h], !is_dual)
    } else {
        select_1009([c, b, a, d, e, g, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_978([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_979([f, j, g, l, a, b, i, k, c, d, h], !is_dual)
    } else {
        select_1002([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_950([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_951([b, c, d, h, f, g, i, j, k, a, l], is_dual)
    } else {
        select_978([b, c, a, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_949([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_950([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_950([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_920([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_921([a, b, c, d, h, g, i, j, l, e, k], is_dual)
    } else {
        select_949([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_919([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_920([a, b, c, d, f, g, e, h, i, j, k, l], is_dual)
    } else {
        select_920([a, b, c, e, f, g, d, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_888([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_889([a, b, c, d, e, h, i, j, k, f, l], is_dual)
    } else {
        select_919([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_656([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_657([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_888([a, b, c, d, e, f, j, k, l, m, g, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_655([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_656([a, b, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_656([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1031([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_124([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_200([b, c, d, e, f, g, a, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1030([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1031([c, d, e, a, b, f, g, h, i, j], is_dual)
    } else {
        select_177([a, b, d, c, e, f, g, i, h, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1034([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_113([b, c, e, d, g, h], is_dual)
    } else {
        select_323([a, c, b, e, d, f, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1035([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_46([a, c, e, d, g, f], is_dual)
    } else {
        select_541([g, h, e, a, b, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1033([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1034([i, e, j, f, h, g, a, c, b], !is_dual)
    } else {
        select_1035([e, b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1038([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_37([b, d, c, g, f, i], is_dual)
    } else {
        select_38([a, c, e, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1037([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_328([a, d, f, e, c, g, h, j], is_dual)
    } else {
        select_1038([a, b, d, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1039([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_67([f, i, b, e, c], !is_dual)
    } else {
        select_49([g, h, d, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1036([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1037([b, a, c, d, f, e, h, g, j, i], is_dual)
    } else {
        select_1039([b, a, c, f, h, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1032([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1033([c, b, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_1036([c, a, d, e, f, b, h, g, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1029([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1030([c, d, a, e, f, h, g, i, j, k, m], is_dual)
    } else {
        select_1032([a, b, e, f, i, g, h, k, l, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1043([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_154([i, j, e, k, c, a], !is_dual)
    } else {
        select_432([g, h, b, d, f], is_dual)
    }
}
/// n = 11, i = 5
fn select_1042([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1043([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_425([e, b, g, h, d, k, f], is_dual)
    }
}
/// n = 12, i = 5
fn select_1044([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1037([c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_407([a, b, d, e, h, g, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1041([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1042([k, l, i, j, f, h, a, g, b, c, d], !is_dual)
    } else {
        select_1044([b, c, a, d, e, f, h, g, j, i, l, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1047([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_473([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_473([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1046([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_792([b, d, g, e, f, c, h, i, j], is_dual)
    } else {
        select_1047([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_1049([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_496([f, d, g, e, a, b, c], !is_dual)
    } else {
        select_496([f, e, g, d, a, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1048([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1049([f, h, i, a, b, c, g], !is_dual)
    } else {
        select_163([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1045([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1046([c, d, e, f, a, b, g, i, h, j, l], is_dual)
    } else {
        select_1048([a, b, d, e, g, i, j, k, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1041([b, c, d, f, h, g, a, i, j, l, k, m], is_dual)
    } else {
        select_1045([c, d, b, e, f, a, g, i, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1028([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1029([b, d, e, f, c, g, a, h, i, j, k, m, l], is_dual)
    } else {
        select_1040([b, d, e, f, a, g, h, c, i, j, l, k, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1052([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_382([c, a, b, e, d, f, g], is_dual)
    } else {
        select_442([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1055([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_119([a, c, d, f, e, g], is_dual)
    } else {
        select_438([f, h, d, i, b, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1054([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1012([c, b, e, a, d, g, f, h, j], is_dual)
    } else {
        select_1055([i, j, f, g, b, a, h, c, d], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1057([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_153([a, e, f, h, j], is_dual)
    } else {
        select_120([b, c, d, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1056([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1057([b, c, d, f, a, e, g, h, i, j], is_dual)
    } else {
        select_641([c, d, g, f, a, i, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1053([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1054([b, c, a, i, f, g, h, k, l, m], is_dual)
    } else {
        select_1056([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1051([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1052([a, g, d, e, h, j, k, i], is_dual)
    } else {
        select_1053([a, b, c, d, e, f, g, h, i, k, l, m, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1050([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1051([c, a, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_1051([c, b, d, e, f, g, a, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1027([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1028([b, c, d, a, e, f, g, h, j, i, l, k, m], is_dual)
    } else {
        select_1050([a, b, d, c, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1063([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_145([f, g, h, a, c, e, d], !is_dual)
    } else {
        select_541([f, g, h, a, b, e, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1064([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_48([b, c, d, e, f, g, h], is_dual)
    } else {
        select_37([a, g, d, f, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1062([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1063([a, d, c, f, h, g, e, j], is_dual)
    } else {
        select_1064([b, a, d, c, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1061([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1062([c, a, d, e, f, b, h, g, j, i], is_dual)
    } else {
        select_1062([c, b, d, e, f, a, h, g, j, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_1067([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_187([e, b, a, d, f], is_dual)
    } else {
        select_113([a, b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_1069([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_37([d, f, g, e, a, b], !is_dual)
    } else {
        select_37([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1068([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_46([a, b, e, d, g, f], is_dual)
    } else {
        select_1069([g, e, h, a, f, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1066([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1067([a, d, f, e, b, h], is_dual)
    } else {
        select_1068([a, c, d, b, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1065([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1066([a, b, c, f, h, e, g, i], is_dual)
    } else {
        select_29([c, d, g, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1060([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1061([c, d, b, e, a, g, h, f, j, i], is_dual)
    } else {
        select_1065([c, d, e, b, g, f, h, a, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1072([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_269([a, b, c, d, h, g, i], is_dual)
    } else {
        select_276([b, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1071([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1072([c, d, e, f, a, b, h, g, i], is_dual)
    } else {
        select_1048([a, b, c, d, e, h, i, j, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1075([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_145([e, f, h, a, b, g, d], !is_dual)
    } else {
        select_474([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1074([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_624([a, c, e, g, b, h, f], is_dual)
    } else {
        select_1075([a, c, b, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1073([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_744([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_1074([a, b, c, f, h, g, i, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1070([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1071([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_1073([b, c, d, e, h, i, g, a, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1059([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1060([b, c, d, e, g, h, a, i, j, k], is_dual)
    } else {
        select_1070([b, d, e, a, c, f, i, g, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1080([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_125([b, h, c, d, e, g, i], is_dual)
    } else {
        select_92([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_1079([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1080([c, a, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_1080([c, b, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1082([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_64([a, b, f, d, e, g, i], is_dual)
    } else {
        select_580([d, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1081([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_87([d, c, a, e, f, b, g, h], is_dual)
    } else {
        select_1082([c, b, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1078([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1079([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_1081([b, c, i, d, a, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1085([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_53([a, c, d, e, g, h, i, j], is_dual)
    } else {
        select_216([a, b, c, i, f, h, g, k, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1086([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_416([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_104([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1084([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1085([a, b, c, d, e, g, h, f, i, k, j], is_dual)
    } else {
        select_1086([f, b, d, e, a, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1088([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_279([h, a, d, e, f, i, g, j], is_dual)
    } else {
        select_75([f, d, e, b, c, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1089([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_416([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_134([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1087([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1088([b, a, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_1089([b, f, d, e, a, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1083([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1084([a, d, c, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_1087([a, d, b, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1077([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1078([a, b, e, f, c, d, g, j, i, h, l], is_dual)
    } else {
        select_1083([c, d, a, f, b, e, g, j, h, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1093([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_104([e, a, b, f, h, d, i, g], is_dual)
    } else {
        select_104([d, a, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1094([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_33([a, f, b, e, g, i, j], is_dual)
    } else {
        select_251([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1092([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1093([d, b, f, e, a, g, j, i, h], is_dual)
    } else {
        select_1094([b, a, d, c, e, g, f, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1091([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1092([a, b, g, c, e, i, f, j, h, k], is_dual)
    } else {
        select_1092([a, b, f, d, e, h, g, j, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1098([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_35([a, c, d, e, h], is_dual)
    } else {
        select_42([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1097([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_541([e, h, i, a, g, f, b], !is_dual)
    } else {
        select_1098([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1096([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1097([a, d, c, f, h, g, e, j, i], is_dual)
    } else {
        select_745([a, b, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1095([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1096([a, b, g, c, j, e, f, h, i, k], is_dual)
    } else {
        select_1096([a, b, f, d, i, e, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1090([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1091([a, c, d, e, b, f, g, i, j, h, k], is_dual)
    } else {
        select_1095([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1076([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1077([a, c, d, e, b, f, h, g, i, j, k, l], is_dual)
    } else {
        select_1090([d, e, a, c, f, g, h, i, b, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1058([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1059([a, c, b, d, e, h, j, k, i, g, l], is_dual)
    } else {
        select_1076([b, c, a, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1026([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1027([a, b, c, d, f, g, e, h, i, j, k, l, m], is_dual)
    } else {
        select_1058([a, b, d, f, g, c, h, i, j, e, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1025([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1026([e, f, g, c, d, a, b, h, j, k, i, l, m], is_dual)
    } else {
        select_1026([e, g, f, c, d, a, b, h, i, k, j, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_1106([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_253([a, b, e, g, d], is_dual)
    } else {
        select_253([a, c, d, f, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_1105([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_983([h, g, j, k, d, f, a, c], !is_dual)
    } else {
        select_1106([g, b, c, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_104([g, i, f, h, a, d, b, e], !is_dual)
    } else {
        select_253([a, f, c, g, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_1104([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_1105([b, a, c, e, d, f, h, g, j, i, k], is_dual)
    } else {
        select_1107([a, b, f, e, i, h, j, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1109([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_571([a, g, h, b, c, d, f], !is_dual)
    } else {
        select_84([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_1111([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_125([b, c, d, e, f, g, h], is_dual)
    } else {
        select_782([a, h, b, c, f, g, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_1112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_125([b, c, d, e, f, g, h], is_dual)
    } else {
        select_59([a, h, b, c, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1110([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1111([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_1112([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1108([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1109([h, a, b, c, d, i, g, j], is_dual)
    } else {
        select_1110([b, d, e, f, a, c, h, g, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1103([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1104([j, l, m, g, h, k, a, b, c, d, i], !is_dual)
    } else {
        select_1108([b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1116([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_33([b, g, d, e, f, i, h], is_dual)
    } else {
        select_77([a, b, c, h, e, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_723([a, b, h, e, j, k], is_dual)
    } else {
        select_1116([a, c, b, d, e, f, g, h, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1118([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_25([c, b, e, h, f, g], is_dual)
    } else {
        select_210([g, a, c, d, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1117([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1118([c, a, f, d, h, e, g, j, i], is_dual)
    } else {
        select_32([d, e, b, g, i, f, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1115([b, a, c, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_1117([b, d, c, f, e, g, a, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_945([a, e, b, d, h, f, g, i], is_dual)
    } else {
        select_47([a, b, f, c, g, h, e, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1113([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1114([a, b, d, e, f, i, g, h, j, k, l], is_dual)
    } else {
        select_1119([a, c, d, e, j, g, h, i, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1102([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_1103([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1113([c, a, e, f, b, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, b, d, e, f, g, i], is_dual)
    } else {
        select_862([a, h, i, b, f, c, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_430([a, b, d, e, f, g, h], is_dual)
    } else {
        select_1123([a, b, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_937([a, h, f, i, b, c], !is_dual)
    } else {
        select_798([a, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1122([a, b, c, d, e, f, g, h, j], is_dual)
    } else {
        select_1124([a, c, b, d, e, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1127([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_571([a, c, f, d, g, e, i], is_dual)
    } else {
        select_862([d, h, g, a, f, b, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_146([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_1127([i, h, j, a, f, e, b, c, g], !is_dual)
    }
}
/// n = 8, i = 2
fn select_1129([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_147([a, c, e, b, h, f, g], is_dual)
    } else {
        select_147([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1131([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_119([a, b, e, d, g, h], is_dual)
    } else {
        select_36([d, a, c, f, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_328([g, i, e, f, h, a, d, b], !is_dual)
    } else {
        select_450([g, i, h, f, e, a, d, c, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1130([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1131([b, d, c, g, f, e, h, j], is_dual)
    } else {
        select_1132([a, c, e, f, h, d, g, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1128([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1129([b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_1130([i, j, l, f, h, k, a, b, c, g, e], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1125([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_1126([b, a, g, d, f, c, h, i, j, k], is_dual)
    } else {
        select_1128([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1120([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1121([h, b, a, c, d, f, i, j, l, g], is_dual)
    } else {
        select_1125([a, c, d, b, e, f, h, g, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1102([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1120([c, b, d, e, i, h, g, a, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1100([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1101([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_1101([b, a, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_1138([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_236([d, g, e, a, b, f, c], !is_dual)
    } else {
        select_236([d, g, f, a, b, e, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1139([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_152([d, g, c, i, e, j, k], is_dual)
    } else {
        select_941([a, e, b, d, f, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1138([j, d, h, g, a, f, b], !is_dual)
    } else {
        select_1139([a, b, c, d, e, g, h, j, i, f, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_29([a, c, d, e, h, j], is_dual)
    } else {
        select_29([b, c, d, f, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1144([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_26([a, d, c, g, e, h], is_dual)
    } else {
        select_49([e, f, b, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_1142([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1143([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1144([i, j, e, h, a, g, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1145([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_474([a, f, c, d, e, g, i], is_dual)
    } else {
        select_474([b, e, c, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1141([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1142([a, b, c, h, e, g, f, i, j, k], is_dual)
    } else {
        select_1145([a, b, d, f, e, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1142([a, b, c, h, e, f, g, j, i, l], is_dual)
    } else {
        select_1142([b, a, d, g, f, e, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1140([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1141([c, b, e, a, g, h, f, d, i, j, k], is_dual)
    } else {
        select_1146([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1136([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1137([j, f, l, a, k, b, h, i, g, c, d], !is_dual)
    } else {
        select_1140([b, c, a, d, e, h, f, g, k, i, l, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1149([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1142([a, b, c, i, f, h, g, j, k, l], is_dual)
    } else {
        select_178([a, b, d, e, g, f, h, i, k, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1148([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1149([c, b, d, a, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_1146([b, c, i, d, f, g, a, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1147([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1137([k, f, m, a, l, b, h, j, g, c, i], !is_dual)
    } else {
        select_1148([b, c, a, d, e, h, f, g, i, l, j, m, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1135([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1136([a, b, c, d, i, f, j, h, g, k, l, m], is_dual)
    } else {
        select_1147([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1153([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_761([b, c, h, f, g, a, i, j], is_dual)
    } else {
        select_200([b, c, a, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1152([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_285([f, g, k, h, l, a, i, b, e], !is_dual)
    } else {
        select_1153([b, c, d, a, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_240([b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_74([e, a, b, d, g, h, j, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1157([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_119([e, a, b, g, d, h], is_dual)
    } else {
        select_244([d, a, c, f, e, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1155([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1156([b, c, d, a, f, e, h, i, g, j], is_dual)
    } else {
        select_1157([c, d, e, f, a, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1158([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_736([h, j, g, i, e, a, b, f, c], !is_dual)
    } else {
        select_736([g, j, h, i, f, a, b, e, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1154([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1155([b, a, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_1158([a, b, c, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1151([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1152([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1154([h, b, c, d, a, f, g, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_123([a, b, f, e, d, h, i, j], is_dual)
    } else {
        select_845([b, a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1164([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_29([b, c, d, g, f, i], is_dual)
    } else {
        select_26([i, j, e, h, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1165([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_29([a, c, d, e, f, g], is_dual)
    } else {
        select_49([a, b, h, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1163([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1164([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_1165([a, e, c, d, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1162([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_300([b, c, d, f, e, g, h], is_dual)
    } else {
        select_1163([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1160([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1161([a, c, d, e, f, g, h, b, i, j], is_dual)
    } else {
        select_1162([a, c, b, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1160([a, b, c, d, g, e, h, f, i, j], is_dual)
    } else {
        select_1160([a, b, c, d, g, f, h, e, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1150([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1151([b, c, a, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1159([j, k, l, f, g, h, b, a, i, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1134([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1135([a, c, b, d, e, f, h, g, i, j, l, k, m], is_dual)
    } else {
        select_1150([a, b, c, d, i, f, h, j, k, l, m, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1170([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1163([a, b, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_1163([a, b, d, g, e, f, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1171([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_152([e, g, d, a, f, i, j], is_dual)
    } else {
        select_319([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1169([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1170([a, b, c, d, f, e, g, h, k, i, l], is_dual)
    } else {
        select_1171([f, j, e, l, a, k, i, b, g, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1173([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_186([e, g, h, a, b, c], !is_dual)
    } else {
        select_383([b, e, c, d, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1174([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_145([e, g, h, a, b, f, c], !is_dual)
    } else {
        select_145([e, g, h, a, b, f, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1172([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1173([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_1174([e, g, a, i, b, h, c, f], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1168([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1169([a, b, c, d, e, f, g, h, j, l, i, k], is_dual)
    } else {
        select_1172([f, a, c, h, e, i, k, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1178([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_551([b, d, c, e, f, h, g, i], is_dual)
    } else {
        select_50([j, i, e, d, b, a, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_42([b, c, d, e, f, g, h], is_dual)
    } else {
        select_23([a, j, k, i], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1179([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1180([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_453([i, j, k, d, a, e, b, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1177([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1178([a, c, b, d, f, e, g, j, i, h], is_dual)
    } else {
        select_1179([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_328([a, e, c, d, f, h, g, j], is_dual)
    } else {
        select_50([i, j, d, e, a, b, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1181([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_327([c, b, d, f, e, g, i, h], is_dual)
    } else {
        select_1182([a, c, b, d, f, e, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1176([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_1177([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_1181([c, b, a, g, d, e, h, f, j, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1185([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_46([f, h, i, d, a, g], !is_dual)
    } else {
        select_119([b, c, d, f, e, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_1184([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1185([a, b, d, f, i, h, e, g, k], is_dual)
    } else {
        select_620([j, i, k, h, g, f, d, a, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1186([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_253([c, a, g, i, d], is_dual)
    } else {
        select_238([b, d, c, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_1183([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1184([b, c, a, e, f, d, h, i, g, k, j], is_dual)
    } else {
        select_1186([c, d, g, f, a, h, e, k, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_1175([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1176([i, e, k, a, h, g, j, f, b, d, c], !is_dual)
    } else {
        select_1183([i, g, k, e, h, j, a, f, b, d, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1167([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1168([a, f, c, d, e, i, g, h, j, l, k, m], is_dual)
    } else {
        select_1175([a, j, c, b, e, g, k, f, i, l, m], is_dual)
    }
}
/// n = 11, i = 5
fn select_1191([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_154([a, f, c, d, i, k], is_dual)
    } else {
        select_417([b, a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1190([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1191([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_817([h, i, f, k, a, j, g, e, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1194([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_37([b, d, c, e, f, h], is_dual)
    } else {
        select_30([a, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1193([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_312([a, b, d, e, g, h, j, k], is_dual)
    } else {
        select_1194([a, b, c, e, f, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1193([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_407([a, h, b, d, g, e, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1189([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_1190([a, c, b, g, f, h, i, e, j, k, l], is_dual)
    } else {
        select_1192([a, b, c, d, e, f, g, h, i, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1197([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_145([a, b, g, e, h, f, i], is_dual)
    } else {
        select_638([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1196([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_301([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_1197([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1199([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_125([b, c, d, e, f, g, h], is_dual)
    } else {
        select_1015([a, h, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1198([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1199([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_301([a, g, d, e, f, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1195([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1196([a, b, c, d, g, f, i, j, h], is_dual)
    } else {
        select_1198([a, d, e, b, c, f, g, h, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1188([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1189([b, d, c, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_1195([b, a, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1202([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1035([d, h, e, i, f, g, a, b], !is_dual)
    } else {
        select_552([a, b, c, d, e, f, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_1204([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_642([a, g, d, f, e, h], is_dual)
    } else {
        select_1098([a, d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1203([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1204([a, c, f, d, e, h, g, j], is_dual)
    } else {
        select_504([a, b, g, d, e, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1201([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_1202([j, e, k, f, a, g, i, b, h], !is_dual)
    } else {
        select_1203([b, a, c, d, g, e, f, h, j, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1206([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_191([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_302([a, b, f, d, h, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1205([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_697([e, i, j, a, g, h, f, b, d], !is_dual)
    } else {
        select_1206([a, c, b, e, g, h, i, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1200([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1201([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1205([a, b, c, h, e, g, i, j, f, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1187([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1188([f, a, c, g, d, e, h, i, j, k, l, m], is_dual)
    } else {
        select_1200([a, b, c, i, e, f, h, j, k, g, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1166([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1167([a, c, d, e, f, b, h, i, g, j, k, l, m], is_dual)
    } else {
        select_1187([a, b, d, e, f, g, c, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1133([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_1134([a, b, d, e, f, g, h, c, i, j, l, k, m], is_dual)
    } else {
        select_1166([a, b, d, c, f, g, h, e, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1099([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1100([d, e, f, b, c, a, h, i, j, g, k, l, m], is_dual)
    } else {
        select_1133([d, e, f, b, c, a, h, i, g, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1024([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1025([a, g, b, c, d, e, f, i, j, k, h, l, m], is_dual)
    } else {
        select_1099([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1023([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_1024([h, f, g, a, d, e, b, k, c, i, j, l, m], is_dual)
    } else {
        select_1024([h, f, g, c, d, e, b, k, a, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1022([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1023([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_1023([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1021([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1022([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_1022([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 11, i = 5
fn select_1213([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_824([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_824([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1217([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_154([d, f, b, c, g, h], is_dual)
    } else {
        select_279([d, f, j, k, e, i, a, h], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1217([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_1217([b, c, d, f, a, e, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1215([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1216([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_1216([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1214([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1215([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    } else {
        select_1215([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1212([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1213([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_1214([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1221([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_122([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_122([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1220([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_295([b, c, d, e, h, g, a, i, j, k], is_dual)
    } else {
        select_1221([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1225([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_141([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_141([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1224([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_443([a, b, f, c, h, e, g, i], is_dual)
    } else {
        select_1225([b, e, a, d, g, f, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1227([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_238([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_141([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1226([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1227([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_1227([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1223([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1224([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_1226([e, b, f, d, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1230([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_438([a, f, c, d, i, g], is_dual)
    } else {
        select_195([h, i, f, a, g, e, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1229([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1230([a, d, b, c, e, f, g, h, i], is_dual)
    } else {
        select_593([h, f, a, j, k, g, i, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1228([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_718([a, e, d, g, b, f, h, i], is_dual)
    } else {
        select_1229([a, b, c, d, f, h, e, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1222([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1223([a, h, f, k, b, i, j, c, g], !is_dual)
    } else {
        select_1228([a, h, i, k, f, j, b, c, g, d, e], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1219([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1220([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1222([a, c, d, e, h, g, i, b, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1219([a, b, c, e, f, g, d, h, i, j, k, l], is_dual)
    } else {
        select_1219([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1211([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < k) || (is_dual && e > k) {
        select_1212([a, b, c, d, f, h, i, j, k, e, l], is_dual)
    } else {
        select_1218([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1210([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1211([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1211([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_1234([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_283([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_283([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1239([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_496([g, a, b, f, e, h, i], is_dual)
    } else {
        select_28([a, e, c, d, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1240([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_125([a, b, c, d, f, g, h], is_dual)
    } else {
        select_474([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1238([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1239([b, g, c, d, a, f, h, i, j], is_dual)
    } else {
        select_1240([a, e, c, d, b, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1241([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_383([a, f, b, g, h, e], is_dual)
    } else {
        select_117([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1237([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1238([a, c, b, d, e, g, h, f, i, j], is_dual)
    } else {
        select_1241([a, c, f, e, g, b, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1236([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_284([a, i, k, l, f, g, b, j, c, h], !is_dual)
    } else {
        select_1237([a, c, b, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1235([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1236([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    } else {
        select_1236([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1233([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1234([a, b, c, g, k, l, i, j, d, e, h], !is_dual)
    } else {
        select_1235([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_1245([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1079([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_86([b, c, i, d, a, g, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1246([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_154([a, f, c, d, g, h], is_dual)
    } else {
        select_36([h, i, f, a, e, b], !is_dual)
    }
}
/// n = 11, i = 3
fn select_1244([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1245([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1246([b, d, c, i, g, h, j, a, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_1249([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_147([e, b, d, a, f, g, h], is_dual)
    } else {
        select_147([f, a, c, b, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1248([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_146([e, g, b, d, a, f, h, i], is_dual)
    } else {
        select_1249([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1252([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_195([b, g, c, a, f, h, i], is_dual)
    } else {
        select_53([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1251([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_723([a, b, i, j, c, h], !is_dual)
    } else {
        select_1252([c, a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1254([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_438([a, e, g, h, f, b], !is_dual)
    } else {
        select_594([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1255([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_541([f, a, b, h, e, g, i], is_dual)
    } else {
        select_28([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1253([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1254([f, h, k, l, a, j, d, e], !is_dual)
    } else {
        select_1255([b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1250([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1251([a, i, f, k, l, g, j, h, b, e], !is_dual)
    } else {
        select_1253([a, b, e, c, d, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1247([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1248([a, c, g, e, h, f, b, i, j], is_dual)
    } else {
        select_1250([a, c, b, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1243([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1244([a, c, d, e, b, f, g, i, h, j, k], is_dual)
    } else {
        select_1247([a, c, d, e, h, b, g, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 1
fn select_1259([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_84([a, b, c, d, e, g], is_dual)
    } else {
        select_84([a, b, c, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1261([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_74([e, b, c, d, f, g, h, j], is_dual)
    } else {
        select_628([a, b, c, d, f, g, e, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1260([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1261([b, a, c, d, f, e, g, i, h, j], is_dual)
    } else {
        select_543([f, i, e, j, h, a, b, g], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1258([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1259([b, c, d, e, f, g, h], is_dual)
    } else {
        select_1260([g, i, k, l, a, b, j, c, d, h], !is_dual)
    }
}
/// n = 8, i = 1
fn select_1264([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_125([a, g, c, d, e, f, h], is_dual)
    } else {
        select_125([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_1263([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1264([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_1264([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1265([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_45([a, b, c, d, g, h], is_dual)
    } else {
        select_147([a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1262([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1263([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1265([a, b, c, i, d, h, g, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1257([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1258([a, b, c, d, e, h, g, j, i, k, l, m], is_dual)
    } else {
        select_1262([c, a, e, f, b, d, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1256([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1257([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_1257([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_1243([a, b, d, f, e, g, h, i, c, j, k, l, m], is_dual)
    } else {
        select_1256([a, b, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1232([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_1233([a, b, c, e, f, i, h, j, d, k, l, m], is_dual)
    } else {
        select_1242([a, b, c, e, f, d, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1271([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_152([a, h, i, b, f, c, g], !is_dual)
    } else {
        select_163([a, b, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1270([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_397([a, f, c, b, e, h, g, i], is_dual)
    } else {
        select_1271([a, b, d, c, g, f, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1274([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_36([a, c, d, g, f, h], is_dual)
    } else {
        select_210([a, b, c, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1273([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_147([a, b, d, e, h, g, i], is_dual)
    } else {
        select_1274([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1275([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_163([a, b, c, d, e, f, h], is_dual)
    } else {
        select_36([a, g, h, b, c, f], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1272([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1273([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_1275([a, b, f, k, l, j, i, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1269([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1270([a, b, e, c, f, d, g, h, i, j], is_dual)
    } else {
        select_1272([a, b, e, d, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 5
fn select_1277([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1246([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_593([f, h, a, j, k, e, i, g], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1276([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_397([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1277([a, h, k, l, f, b, j, c, i, d, g], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1269([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_1276([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1281([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_123([b, g, c, e, a, f, h, i], is_dual)
    } else {
        select_961([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1280([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1281([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_1281([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1283([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_123([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_123([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_1282([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_151([a, b, g, j, k, e, i, c, h], !is_dual)
    } else {
        select_1283([a, b, e, d, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1279([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1280([a, b, i, d, e, g, h, j, k], is_dual)
    } else {
        select_1282([a, b, c, d, f, g, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1287([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_41([a, b, g, c, f, h, e, i], is_dual)
    } else {
        select_83([b, c, d, e, h, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1286([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_32([a, b, g, h, f, i, e, j], is_dual)
    } else {
        select_1287([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1285([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1286([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_1286([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_1290([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_182([a, c, d, b, e, f], is_dual)
    } else {
        select_182([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_1289([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, b, d, e, f, g, h], is_dual)
    } else {
        select_1290([a, b, c, g, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1288([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_489([a, d, e, f, b, c, g, h, i], is_dual)
    } else {
        select_1289([b, c, g, e, f, a, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1284([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1285([a, b, c, i, d, g, h, f, j, k], is_dual)
    } else {
        select_1288([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1278([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_1279([a, b, d, e, f, c, h, i, g, j, k, l, m], is_dual)
    } else {
        select_1284([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1267([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1268([a, b, c, d, e, i, g, j, h, k, l, m], is_dual)
    } else {
        select_1278([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1266([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1267([a, b, c, e, f, g, d, h, i, j, k, l, m], is_dual)
    } else {
        select_1267([a, b, d, e, f, g, c, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1231([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1232([a, b, c, d, f, g, h, e, i, j, k, l, m], is_dual)
    } else {
        select_1266([a, b, c, d, f, e, h, g, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1209([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_1210([a, b, c, d, e, g, i, j, k, f, l, m], is_dual)
    } else {
        select_1231([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1299([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_253([a, b, h, i, j], is_dual)
    } else {
        select_18([c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1300([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_417([a, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_417([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1298([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1299([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1300([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1302([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_416([a, b, c, d, f, g, h, i], is_dual)
    } else {
        select_416([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1303([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([c, d, e, f, g], is_dual)
    } else {
        select_49([a, b, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1301([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1302([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_1303([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1297([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1298([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_1301([a, b, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1304([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_146([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_382([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1296([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1297([a, b, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1304([a, b, c, d, i, j, h, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1307([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_290([a, c, d, b, f, e, g], is_dual)
    } else {
        select_703([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1306([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_937([a, h, b, c, f, g], is_dual)
    } else {
        select_1307([a, g, b, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1309([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_887([a, b, d, e, f, g, i, j], is_dual)
    } else {
        select_71([a, h, i, j, b, f, c, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1308([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_146([a, b, c, f, e, h, g, i], is_dual)
    } else {
        select_1309([a, e, h, j, k, f, i, b, d, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1305([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1306([a, b, j, k, f, h, i, c, g], !is_dual)
    } else {
        select_1308([a, h, j, k, b, f, i, c, g, d, e], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1295([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_1296([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_1305([a, b, j, l, m, c, k, d, h, e, i], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1294([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_78([a, b, d, e, f, g, c, i, h, j], is_dual)
    } else {
        select_1295([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_1314([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_71([a, f, i, j, b, h, c, g], !is_dual)
    } else {
        select_276([a, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1313([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1314([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_1314([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1315([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_430([a, b, c, g, i, h, f], !is_dual)
    } else {
        select_887([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1312([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1313([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1315([a, b, c, i, j, h, d, e, g], !is_dual)
    }
}
/// n = 9, i = 2
fn select_1319([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_53([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_53([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1318([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1264([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_1319([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1317([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_1263([a, c, b, e, f, g, d, h], is_dual)
    } else {
        select_1318([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_1321([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_887([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_82([h, i, f, j, k, a, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1320([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1321([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_1321([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1316([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1317([a, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_1320([a, b, h, k, l, m, j, c, d, e, i], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1311([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_1312([a, b, h, k, l, m, c, j, d, i], !is_dual)
    } else {
        select_1316([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_1325([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_290([a, c, d, b, e, f, g], is_dual)
    } else {
        select_290([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_1324([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_937([a, h, b, c, e, f], is_dual)
    } else {
        select_1325([a, f, b, d, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1323([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_1324([b, a, d, e, c, f, g, h], is_dual)
    } else {
        select_759([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1326([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_757([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_757([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1322([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1323([a, c, d, e, f, g, h, b, i], is_dual)
    } else {
        select_1326([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1310([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < c) || (is_dual && j > c) {
        select_1311([a, b, d, e, c, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1322([a, b, k, l, m, h, d, c, j], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1293([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1294([a, b, c, e, d, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1310([a, b, c, d, e, f, g, h, j, k, l, m, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1292([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1293([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_1293([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1291([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1292([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_1292([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1208([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1209([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_1291([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1333([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_588([e, f, k, l, i, m, h, a, b, j], !is_dual)
    } else {
        select_767([e, f, c, h, d, i, g, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_1335([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_235([d, h, i, f, e, g, a, b, c], !is_dual)
    } else {
        select_235([e, h, i, f, d, g, a, b, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1334([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_1335([a, b, c, f, g, e, h, d, i], is_dual)
    } else {
        select_1335([b, a, c, f, g, d, h, e, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1332([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < h) || (is_dual && m > h) {
        select_1333([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1334([a, b, j, e, f, k, l, m, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1339([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_118([c, b, d, e, g, f, h], is_dual)
    } else {
        select_63([c, h, i, g, e, d, a, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1338([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1339([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_1339([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1341([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, b, d, e, f, g, h], is_dual)
    } else {
        select_144([a, b, c, g, f, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1342([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_163([a, b, d, e, f, g, h], is_dual)
    } else {
        select_154([a, b, c, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1340([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_1341([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_1342([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1337([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_1338([i, j, a, b, c, d, h, e, g], !is_dual)
    } else {
        select_1340([a, b, c, e, d, f, g, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1336([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1337([k, l, m, d, g, i, h, e, j, b], !is_dual)
    } else {
        select_767([d, g, a, h, c, i, f, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1331([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1332([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    } else {
        select_1336([c, b, d, f, g, h, i, a, e, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1330([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1331([a, c, d, e, b, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1331([b, c, d, e, a, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1329([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1330([a, b, c, e, f, g, d, i, h, j, k, l, m], is_dual)
    } else {
        select_1330([a, b, d, e, f, h, c, i, g, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1328([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1329([a, b, c, e, f, g, d, h, i, j, k, l, m], is_dual)
    } else {
        select_1329([a, b, d, e, f, g, c, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1327([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1328([a, b, c, d, e, g, h, f, i, j, k, l, m], is_dual)
    } else {
        select_1328([a, b, c, d, f, g, h, e, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1207([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1208([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1327([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1020([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_1021([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1207([a, b, c, d, e, f, g, i, j, k, l, h, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_654([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_655([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    } else {
        select_1020([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_221([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_222([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_654([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_221([a, b, c, d, e, f, g, h, i, j, l, m, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    }
}
