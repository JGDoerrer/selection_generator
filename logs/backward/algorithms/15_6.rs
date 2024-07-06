/// n = 1, i = 0
fn select_26([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_25([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_26([a], is_dual)
    } else {
        select_26([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_24([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_25([a, b], is_dual)
    } else {
        select_25([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_27([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_25([a, d], is_dual)
    } else {
        select_25([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([b, c, e], is_dual)
    } else {
        select_27([a, f, d, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, b, d], is_dual)
    } else {
        select_27([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_23([b, a, c, d, e, f, g], is_dual)
    } else {
        select_28([a, e, c, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_31([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_25([a, b], is_dual)
    } else {
        select_26([c], is_dual)
    }
}
/// n = 5, i = 2
fn select_30([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_31([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_32([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_31([a, d, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_30([b, c, f, e, h], is_dual)
    } else {
        select_32([a, b, d, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_21([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([g, h, i, d, e, f, b, a], !is_dual)
    } else {
        select_29([b, c, a, f, d, g, e, i, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_24([b, c, d], is_dual)
    } else {
        select_31([a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_34([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, b, c, d, f, g], is_dual)
    } else {
        select_28([b, d, c, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_37([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([c, d, e], is_dual)
    } else {
        select_24([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_28([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_34([b, a, g, f, i, e, h], is_dual)
    } else {
        select_36([b, c, d, e, f, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_20([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_21([e, b, d, a, g, f, h, i, j], is_dual)
    } else {
        select_33([b, a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_40([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_35([h, e, i, f, c, a], !is_dual)
    } else {
        select_35([g, f, i, e, d, b], !is_dual)
    }
}
/// n = 5, i = 2
fn select_41([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_31([a, d, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_39([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_40([b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_41([a, g, c, e, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_43([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_35([a, b, c, d, f, g], is_dual)
    } else {
        select_28([a, e, g, f], !is_dual)
    }
}
/// n = 6, i = 2
fn select_45([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_31([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_46([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, b, e, f], is_dual)
    } else {
        select_27([a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([a, e, c, f, d, h], is_dual)
    } else {
        select_46([b, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_42([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_43([g, h, j, d, a, f, c], !is_dual)
    } else {
        select_44([i, g, j, h, d, e, a, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_38([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_39([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_42([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_19([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_20([d, j, g, k, h, i, e, a, b, f], !is_dual)
    } else {
        select_38([a, b, c, d, e, f, g, i, j, k, h], is_dual)
    }
}
/// n = 4, i = 0
fn select_51([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_24([a, b, c], is_dual)
    } else {
        select_24([b, c, d], is_dual)
    }
}
/// n = 7, i = 1
fn select_50([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([c, d, e, f], is_dual)
    } else {
        select_24([a, b, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_53([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([a, b, c], is_dual)
    } else {
        select_26([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([c, d, e, f], is_dual)
    } else {
        select_53([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_50([b, f, c, d, e, g, h], is_dual)
    } else {
        select_52([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([c, d, e, f], is_dual)
    } else {
        select_28([a, b, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_57([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_24([b, c, d], is_dual)
    } else {
        select_25([a, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_56([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_51([c, d, e, g], is_dual)
    } else {
        select_57([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_55([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_56([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_49([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_54([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_59([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_52([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_52([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_61([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_45([a, h, b, f, i, j], is_dual)
    } else {
        select_51([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_55([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_61([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_58([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_59([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_60([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_47([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_48([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_58([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_18([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_19([k, j, m, a, g, l, b, c, h, d, i], !is_dual)
    } else {
        select_47([c, a, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_66([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_28([c, d, b, e], is_dual)
    } else {
        select_27([a, e, c, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_67([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_31([d, b, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_66([b, c, d, f, e, g], is_dual)
    } else {
        select_67([g, a, b, d, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_69([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_57([a, c, d, b, e], is_dual)
    } else {
        select_57([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_70([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_35([a, c, d, b, e, f], is_dual)
    } else {
        select_35([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_68([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_69([a, f, d, g, b], !is_dual)
    } else {
        select_70([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_64([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_65([b, c, d, f, e, h, g], is_dual)
    } else {
        select_68([g, f, i, a, h, b, c], !is_dual)
    }
}
/// n = 5, i = 1
fn select_73([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_51([a, b, c, d], is_dual)
    } else {
        select_26([e], is_dual)
    }
}
/// n = 6, i = 2
fn select_74([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_30([e, f, a, b, c], !is_dual)
    } else {
        select_53([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_72([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_73([b, d, e, f, g], is_dual)
    } else {
        select_74([a, b, c, g, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_45([a, d, b, c, e, f], is_dual)
    } else {
        select_45([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_75([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_69([b, c, d, e, f], is_dual)
    } else {
        select_76([a, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_71([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_72([c, d, e, a, b, g, f, h], is_dual)
    } else {
        select_75([c, a, b, d, g, h, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_63([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_64([b, d, e, c, f, g, a, h, i], is_dual)
    } else {
        select_71([a, c, b, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_62([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_63([c, a, d, e, f, g, i, h, j], is_dual)
    } else {
        select_19([j, i, l, a, f, k, b, c, g, d, h], !is_dual)
    }
}
/// n = 13, i = 5
fn select_17([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_18([b, a, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_62([a, b, c, d, e, i, h, j, g, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_83([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_27([b, c, d, e], is_dual)
    } else {
        select_31([a, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_82([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_46([b, c, d, e, f, g], is_dual)
    } else {
        select_83([a, c, d, e, f, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_85([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_27([a, d, c, f], is_dual)
    } else {
        select_27([b, c, d, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_84([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([h, i, d, g, a, f], !is_dual)
    } else {
        select_53([e, b, c, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_81([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_82([a, b, e, d, f, g, h, i, k], is_dual)
    } else {
        select_84([a, c, f, d, g, e, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_86([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_73([b, c, d, e, f], is_dual)
    } else {
        select_30([e, g, a, b, f], !is_dual)
    }
}
/// n = 12, i = 5
fn select_80([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_81([j, k, l, i, h, g, a, b, e, f, c], !is_dual)
    } else {
        select_86([b, a, d, g, h, f, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_51([b, c, d, e], is_dual)
    } else {
        select_31([a, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_88([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_29([a, h, b, f, e, i, g, j, k], is_dual)
    } else {
        select_89([a, c, d, g, f, h, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_91([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_57([a, b, f, d, e], is_dual)
    } else {
        select_57([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_92([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([a, f, c, d, e, g], is_dual)
    } else {
        select_37([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_90([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_91([a, b, g, e, f, d], is_dual)
    } else {
        select_92([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 14, i = 5
fn select_87([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_88([a, c, b, h, f, g, i, k, l, m, n], is_dual)
    } else {
        select_90([d, e, g, b, i, h, j], is_dual)
    }
}
/// n = 15, i = 6
fn select_79([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_80([l, k, n, o, i, m, a, f, g, b, j, d], !is_dual)
    } else {
        select_87([b, c, d, a, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 3, i = 1
fn select_97([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([a, b, c], is_dual)
    } else {
        select_31([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_96([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_97([a, d, e], is_dual)
    } else {
        select_27([a, b, c, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_98([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_45([a, c, b, d, e, f], is_dual)
    } else {
        select_35([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_95([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_96([b, c, e, h, i], is_dual)
    } else {
        select_98([a, c, d, g, f, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_101([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, c, d, g], is_dual)
    } else {
        select_27([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_100([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_45([d, b, c, e, f, h], is_dual)
    } else {
        select_101([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_99([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_100([b, c, a, e, d, h, g, f], is_dual)
    } else {
        select_96([h, d, e, a, f], !is_dual)
    }
}
/// n = 11, i = 5
fn select_94([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_95([a, c, d, b, f, e, h, g, i, j, k], is_dual)
    } else {
        select_99([c, e, d, f, b, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_45([d, b, c, e, f, g], is_dual)
    } else {
        select_32([a, b, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_104([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_35([a, b, f, e, h, i], is_dual)
    } else {
        select_24([c, d, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_102([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_103([a, c, f, e, b, g, h, i], is_dual)
    } else {
        select_104([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 15, i = 6
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_94([a, b, c, j, f, g, l, m, i, n, o], is_dual)
    } else {
        select_102([c, d, i, e, g, h, k, j, l], is_dual)
    }
}
/// n = 15, i = 6
fn select_78([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_79([a, c, d, b, e, f, h, i, g, j, k, m, n, l, o], is_dual)
    } else {
        select_93([a, b, c, d, e, g, h, i, f, j, k, l, m, n, o], is_dual)
    }
}
/// n = 7, i = 2
fn select_109([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_45([a, c, b, f, g, e], is_dual)
    } else {
        select_45([a, d, b, e, g, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_110([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_51([b, c, d, e], is_dual)
    } else {
        select_25([a, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_108([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_109([a, c, b, h, g, i, f], is_dual)
    } else {
        select_110([c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_35([b, c, d, e, f, g], is_dual)
    } else {
        select_28([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_57([b, c, d, g, f], is_dual)
    } else {
        select_30([a, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_111([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_112([f, a, c, d, e, g, h, i], is_dual)
    } else {
        select_113([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_107([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_108([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_111([f, c, d, e, a, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_66([d, c, a, f, e, g], is_dual)
    } else {
        select_22([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_117([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_45([a, b, c, e, d, f], is_dual)
    } else {
        select_32([d, c, a, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_116([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_103([b, c, d, e, f, g, h, j], is_dual)
    } else {
        select_117([a, c, d, g, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_115([b, c, a, f, g, i, h, e], is_dual)
    } else {
        select_116([a, c, d, b, f, e, i, g, j, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_106([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_107([a, b, g, d, e, f, i, h, k, m], is_dual)
    } else {
        select_114([a, b, c, h, f, g, j, l, k, m], is_dual)
    }
}
/// n = 8, i = 1
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_92([a, b, c, h, e, f, g], is_dual)
    } else {
        select_92([a, b, d, g, e, f, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_119([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_120([a, b, c, d, f, g, h, e], is_dual)
    } else {
        select_120([a, b, c, e, f, g, h, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_123([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_45([f, g, e, c, d, a], !is_dual)
    } else {
        select_31([e, b, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_122([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_66([d, g, a, e, f, b], !is_dual)
    } else {
        select_123([e, g, d, f, a, b, c], !is_dual)
    }
}
/// n = 5, i = 0
fn select_125([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_51([a, b, c, d], is_dual)
    } else {
        select_51([a, b, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_125([b, c, d, e, f], is_dual)
    } else {
        select_53([a, h, i, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_121([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_122([a, b, h, f, i, g, j], is_dual)
    } else {
        select_124([a, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_118([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_119([d, e, g, c, f, h, i, j], is_dual)
    } else {
        select_121([a, b, c, h, i, f, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_105([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_106([a, b, c, d, i, g, h, k, j, f, m, l, n], is_dual)
    } else {
        select_118([b, c, d, a, e, g, h, f, i, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_77([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_78([b, c, d, e, f, a, g, h, i, k, j, l, m, n, o], is_dual)
    } else {
        select_105([b, a, d, e, f, g, c, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_16([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_17([b, a, d, f, c, h, g, i, j, k, n, l, o], is_dual)
    } else {
        select_77([a, b, c, e, d, f, g, h, i, j, l, m, o, k, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_131([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_96([a, f, e, b, c], !is_dual)
    } else {
        select_76([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_132([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_98([f, g, h, a, d, e, c], !is_dual)
    } else {
        select_26([b], is_dual)
    }
}
/// n = 9, i = 3
fn select_130([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_131([a, b, f, d, h, g], is_dual)
    } else {
        select_132([a, c, b, e, g, h, i, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_134([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_96([f, d, b, a, e], !is_dual)
    } else {
        select_45([b, c, a, d, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_136([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_30([b, c, f, g, e], is_dual)
    } else {
        select_53([h, d, i, a], !is_dual)
    }
}
/// n = 9, i = 4
fn select_135([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_34([e, h, i, d, a, f, c], !is_dual)
    } else {
        select_136([a, b, c, e, f, d, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_134([i, j, f, a, b, c], !is_dual)
    } else {
        select_135([h, i, j, a, f, g, c, d, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_130([b, d, c, e, f, a, g, h, j], is_dual)
    } else {
        select_133([b, c, a, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_140([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_37([b, c, d, e, g, f], is_dual)
    } else {
        select_53([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_50([b, c, d, e, i, g, h], is_dual)
    } else {
        select_140([f, a, h, b, c, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_141([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_36([a, c, b, h, g, f, i], is_dual)
    } else {
        select_52([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_139([c, a, d, e, f, b, g, i, h, j], is_dual)
    } else {
        select_141([b, g, c, e, f, a, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_144([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([g, h, f, d, a, b], !is_dual)
    } else {
        select_28([a, f, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_51([b, c, d, e], is_dual)
    } else {
        select_53([a, g, h, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_144([h, i, j, e, f, a, b, g], !is_dual)
    } else {
        select_145([a, c, d, f, e, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_143([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_143([c, b, d, e, f, a, g, i, j, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_137([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_138([b, c, e, f, a, d, g, i, h, j], is_dual)
    } else {
        select_142([a, b, c, d, g, i, j, h, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_128([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_129([a, b, c, d, k, i, g, l, j, m], is_dual)
    } else {
        select_137([a, b, c, g, e, f, h, j, i, k, m, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_150([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_28([b, c, e, g], is_dual)
    } else {
        select_53([a, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_149([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_55([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_150([a, b, c, h, f, g, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_152([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_45([a, d, c, e, f, g], is_dual)
    } else {
        select_28([a, b, d, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_153([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_45([a, c, b, d, e, f], is_dual)
    } else {
        select_32([a, b, e, f, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_151([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_152([b, c, d, e, f, g, h], is_dual)
    } else {
        select_153([a, d, g, f, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_149([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_151([b, c, d, g, f, a, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_147([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_148([c, a, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_133([a, b, d, c, g, h, i, j, f, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_157([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_45([a, c, b, e, f, g], is_dual)
    } else {
        select_53([a, c, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_156([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_157([a, c, b, h, f, g, i], is_dual)
    } else {
        select_110([c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_155([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_139([c, b, d, e, f, a, g, i, h, j], is_dual)
    } else {
        select_156([a, c, g, e, f, b, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_159([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_43([a, b, g, e, h, f, i], is_dual)
    } else {
        select_145([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_161([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_35([a, b, c, d, e, f], is_dual)
    } else {
        select_35([a, b, c, d, e, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_162([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_57([e, c, d, a, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_160([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_161([a, c, d, e, g, h, i], is_dual)
    } else {
        select_162([e, b, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_158([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_159([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_160([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_154([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_155([b, d, e, f, a, c, g, i, h, j], is_dual)
    } else {
        select_158([b, a, c, d, g, i, j, k, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_146([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_147([a, c, b, d, e, h, j, i, g, l, k], is_dual)
    } else {
        select_154([c, a, d, e, f, b, g, i, h, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_128([a, b, d, e, f, c, h, g, i, j, k, m, l], is_dual)
    } else {
        select_146([a, c, d, f, b, h, g, j, i, l, k, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_168([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_28([a, g, c, i], is_dual)
    } else {
        select_101([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_168([b, a, d, c, f, g, e, i, h], is_dual)
    } else {
        select_150([b, a, e, d, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_170([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, b, c, e, h, i], is_dual)
    } else {
        select_57([d, b, c, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_35([a, c, d, e, f, h], is_dual)
    } else {
        select_53([b, c, d, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_170([a, e, h, d, f, g, j, i, l], is_dual)
    } else {
        select_171([a, b, c, i, f, h, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_166([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_167([b, c, i, e, a, f, h, k, l], is_dual)
    } else {
        select_169([b, c, e, d, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_173([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_109([b, a, c, d, e, f, g], is_dual)
    } else {
        select_45([b, d, e, a, g, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_157([b, d, e, g, f, i, h], is_dual)
    } else {
        select_74([a, b, c, h, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_172([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_173([b, c, h, e, a, g, j], is_dual)
    } else {
        select_174([b, c, e, d, a, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_165([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_166([a, c, b, e, d, f, g, h, i, j, l, k], is_dual)
    } else {
        select_172([b, a, d, e, f, g, j, i, h, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_178([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_97([a, e, f], is_dual)
    } else {
        select_57([a, b, c, d, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_177([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_170([a, d, f, c, e, g, i, h, j], is_dual)
    } else {
        select_178([a, b, h, e, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_101([a, c, h, d, f, g, j], is_dual)
    } else {
        select_101([b, c, g, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_176([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_177([a, f, d, c, e, h, g, j, i, k], is_dual)
    } else {
        select_179([a, d, b, e, g, f, i, j, h, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_181([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_96([a, c, b, d, e], is_dual)
    } else {
        select_96([b, c, a, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_183([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([e, a, c, f, d, g], is_dual)
    } else {
        select_45([d, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_182([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_183([b, e, c, d, g, f, i], is_dual)
    } else {
        select_45([d, f, a, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_180([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_181([e, g, d, a, h], !is_dual)
    } else {
        select_182([b, c, a, e, f, d, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_175([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_176([b, c, a, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_180([b, c, i, e, a, f, h, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_164([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_165([a, c, d, b, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_175([a, d, c, e, g, f, h, b, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_98([a, d, c, g, f, e, h], is_dual)
    } else {
        select_43([a, b, e, f, h, d, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_186([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_122([a, b, d, f, h, g, i], is_dual)
    } else {
        select_187([a, d, c, g, e, f, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_189([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_178([e, c, d, a, f, g], is_dual)
    } else {
        select_36([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_191([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_30([d, f, e, a, b], !is_dual)
    } else {
        select_30([e, f, d, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_98([a, c, d, e, f, g, h], is_dual)
    } else {
        select_191([b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_188([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_189([a, d, e, g, f, j, h], is_dual)
    } else {
        select_190([a, b, c, h, f, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_185([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_186([b, c, f, e, a, i, h, g, j], is_dual)
    } else {
        select_188([b, c, e, a, d, g, h, f, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_195([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_46([d, f, g, a, e, b], !is_dual)
    } else {
        select_30([e, f, d, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_194([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_157([b, c, d, e, f, g, i], is_dual)
    } else {
        select_195([e, h, j, a, f, g, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_197([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_45([a, b, d, f, e, h], is_dual)
    } else {
        select_23([c, a, b, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_196([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([a, b, c, e, d, f, h, g], is_dual)
    } else {
        select_197([a, c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_193([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_194([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_196([i, e, j, a, h, g, f, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_76([e, a, d, c, g, f], is_dual)
    } else {
        select_171([g, h, d, i, e, f, b, a], !is_dual)
    }
}
/// n = 11, i = 4
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_194([a, b, c, e, g, f, i, j, h, k], is_dual)
    } else {
        select_199([j, g, k, a, h, i, f, b, d], !is_dual)
    }
}
/// n = 12, i = 4
fn select_192([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_193([a, b, e, i, h, g, f, k, l, j], is_dual)
    } else {
        select_198([a, c, e, d, f, g, h, j, i, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_185([a, b, c, g, e, i, h, j, f, k], is_dual)
    } else {
        select_192([a, b, c, e, d, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_163([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_164([a, b, h, d, c, i, g, f, l, j, n, k], is_dual)
    } else {
        select_184([a, c, b, e, g, f, h, j, i, k, m, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_126([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_127([a, c, d, b, e, f, g, i, h, j, l, k, m, n], is_dual)
    } else {
        select_163([a, c, d, e, f, g, i, h, b, j, k, l, n, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_16([a, b, d, c, f, e, g, i, h, k, m, l, j, o, n], is_dual)
    } else {
        select_126([a, c, d, b, f, e, g, h, i, k, m, j, l, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_207([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_45([a, c, b, f, g, h], is_dual)
    } else {
        select_28([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_206([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_191([f, b, c, h, e, g], is_dual)
    } else {
        select_207([i, h, j, f, e, d, g, a], !is_dual)
    }
}
/// n = 7, i = 3
fn select_209([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_83([a, b, c, d, f, e, g], is_dual)
    } else {
        select_83([a, b, c, e, f, d, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_208([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_153([b, d, c, e, f, h], is_dual)
    } else {
        select_209([a, b, d, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_205([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_206([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_208([g, i, j, e, d, f, h, a, c], !is_dual)
    }
}
/// n = 9, i = 2
fn select_211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_157([a, d, b, c, f, h, i], is_dual)
    } else {
        select_92([d, e, b, c, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_183([a, b, c, d, e, f, g], is_dual)
    } else {
        select_53([e, a, h, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_210([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_211([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_212([h, f, k, b, a, j, i, g], !is_dual)
    }
}
/// n = 13, i = 5
fn select_204([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_205([l, j, m, a, f, k, b, g, c, i], !is_dual)
    } else {
        select_210([b, a, d, g, e, f, h, j, i, k, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_216([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_45([a, c, d, f, e, h], is_dual)
    } else {
        select_45([b, d, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_215([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_216([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_153([c, e, d, a, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_218([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_45([g, i, h, d, f, a], !is_dual)
    } else {
        select_53([e, b, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_217([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_153([h, d, i, f, g, a], !is_dual)
    } else {
        select_218([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_214([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_215([h, f, g, i, d, a, b, e], !is_dual)
    } else {
        select_217([b, a, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_220([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_183([b, c, d, e, g, f, h], is_dual)
    } else {
        select_153([a, b, c, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_219([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_220([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_45([g, i, e, a, b, h], !is_dual)
    }
}
/// n = 11, i = 4
fn select_213([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_214([j, k, i, f, e, c, b, a, h], !is_dual)
    } else {
        select_219([c, b, e, d, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_203([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_204([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_213([a, c, b, e, i, f, h, j, g, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_225([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_23([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_224([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_162([d, c, b, e, f, g], is_dual)
    } else {
        select_225([a, b, f, d, e, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_226([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_183([b, g, c, e, a, f, h], is_dual)
    } else {
        select_92([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_223([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_224([a, b, c, e, f, g, i, j, k], is_dual)
    } else {
        select_226([e, b, c, d, i, g, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_229([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([a, e, c, f, d, h], is_dual)
    } else {
        select_45([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_228([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_229([f, i, j, d, h, g, a, b], !is_dual)
    } else {
        select_117([f, b, c, g, e, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_231([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_45([a, f, b, e, d, g], is_dual)
    } else {
        select_57([b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_230([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_117([d, b, c, e, f, g], is_dual)
    } else {
        select_231([d, h, i, a, e, g, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_227([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_228([j, g, k, d, i, a, h, e, b, f], !is_dual)
    } else {
        select_230([a, e, c, d, g, f, i, h, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_222([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_223([b, c, a, d, g, e, f, h, i, j, k], is_dual)
    } else {
        select_227([i, k, m, e, f, l, a, g, j, b, h], !is_dual)
    }
}
/// n = 7, i = 3
fn select_234([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_69([d, e, a, g, f], !is_dual)
    } else {
        select_191([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_233([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_234([f, e, i, a, b, h, g], !is_dual)
    } else {
        select_226([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_237([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_46([a, b, e, f, d, g], is_dual)
    } else {
        select_57([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_236([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_237([b, a, c, d, f, e, g], is_dual)
    } else {
        select_117([d, e, c, a, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_235([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_236([a, b, f, h, e, i, j, g], is_dual)
    } else {
        select_226([a, c, d, e, h, f, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_232([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_233([a, i, c, d, e, h, g, j, l], is_dual)
    } else {
        select_235([e, b, c, d, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_221([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_222([b, c, d, e, a, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_232([b, a, d, e, f, c, h, i, g, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_202([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_203([a, c, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    } else {
        select_221([a, c, b, e, f, g, d, h, i, j, l, k, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_241([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_220([a, e, c, d, h, f, g, i], is_dual)
    } else {
        select_35([a, b, i, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_243([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([a, e, b, d, g, h], is_dual)
    } else {
        select_25([c, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_242([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_76([b, a, f, d, h, g], is_dual)
    } else {
        select_243([b, a, c, g, e, i, h, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_240([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_241([b, c, e, d, f, g, j, h, i], is_dual)
    } else {
        select_242([a, b, c, i, f, h, e, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_246([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_162([b, d, a, e, f, g], is_dual)
    } else {
        select_162([a, c, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_247([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_162([a, b, f, g, e, i], is_dual)
    } else {
        select_92([c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_245([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_246([a, b, f, d, h, g, e, i], is_dual)
    } else {
        select_247([b, d, a, c, g, e, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_250([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_85([a, b, d, e, h, i], is_dual)
    } else {
        select_85([a, c, d, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_249([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_250([a, b, d, h, e, f, g, j, i], is_dual)
    } else {
        select_152([a, g, d, c, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_248([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_242([a, b, c, f, e, h, i, j, g], is_dual)
    } else {
        select_249([a, c, b, d, e, f, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_244([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_245([b, e, c, d, h, f, g, i, j], is_dual)
    } else {
        select_248([a, b, c, g, f, j, h, i, e, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_239([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_240([a, c, d, e, b, f, g, i, j, h, k], is_dual)
    } else {
        select_244([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_254([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_152([b, c, d, a, e, f, g], is_dual)
    } else {
        select_152([b, c, e, a, d, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_255([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_73([a, c, d, e, f], is_dual)
    } else {
        select_69([a, b, c, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_253([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_254([h, a, b, c, d, g, i], is_dual)
    } else {
        select_255([c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_257([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_55([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_55([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_258([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_152([a, b, c, h, f, g, i], is_dual)
    } else {
        select_110([c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_256([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_257([c, d, e, f, a, b, g, i, h, j], is_dual)
    } else {
        select_258([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_252([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_253([c, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_256([c, d, b, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_261([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_152([a, b, c, f, e, g, h], is_dual)
    } else {
        select_57([c, d, g, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_262([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_112([a, b, c, f, g, e, h, i], is_dual)
    } else {
        select_112([a, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_260([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_261([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_262([a, b, c, f, e, i, g, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_259([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_260([a, b, e, c, f, d, g, h, i, j], is_dual)
    } else {
        select_260([a, b, e, d, f, c, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_251([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_252([f, a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_259([b, c, d, e, g, j, h, a, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_239([a, c, d, e, f, g, h, i, b, j, k], is_dual)
    } else {
        select_251([a, c, d, b, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_202([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    } else {
        select_238([b, d, a, e, f, c, h, g, j, i, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_269([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_46([a, b, c, e, f, g], is_dual)
    } else {
        select_46([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_268([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_74([a, g, h, i, f, b], !is_dual)
    } else {
        select_269([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_270([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_109([b, c, a, d, e, f, g], is_dual)
    } else {
        select_74([a, f, h, i, g, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_267([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_268([a, c, d, f, g, e, i, h, j], is_dual)
    } else {
        select_270([a, b, c, e, g, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_266([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_233([a, b, f, d, e, h, g, i, j], is_dual)
    } else {
        select_267([e, h, k, l, j, f, a, b, c, i], !is_dual)
    }
}
/// n = 8, i = 3
fn select_273([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_216([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_45([b, d, e, c, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_272([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_134([c, d, e, f, a, g], is_dual)
    } else {
        select_273([c, b, d, a, f, e, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_275([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_76([e, a, g, h, b, f], !is_dual)
    } else {
        select_89([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_90([c, d, f, b, e, g, h], is_dual)
    } else {
        select_275([e, a, b, g, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_271([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_272([j, h, e, k, f, a, b, i], !is_dual)
    } else {
        select_274([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_265([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_266([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    } else {
        select_271([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_279([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_66([a, c, e, f, d, h], is_dual)
    } else {
        select_195([e, h, i, a, g, b, d], !is_dual)
    }
}
/// n = 8, i = 2
fn select_281([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_23([a, b, c, e, f, g, h], is_dual)
    } else {
        select_24([b, c, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_280([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_281([b, a, d, f, e, g, i, j], is_dual)
    } else {
        select_162([a, c, d, g, f, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_278([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_279([a, i, c, f, e, k, h, l, m], is_dual)
    } else {
        select_280([b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_284([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_28([a, g, i, h], !is_dual)
    } else {
        select_101([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_283([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_112([d, i, j, k, a, e, b, g], !is_dual)
    } else {
        select_284([a, e, c, d, f, h, j, g, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_69([b, d, c, e, f], is_dual)
    } else {
        select_112([d, e, h, i, a, g, b, f], !is_dual)
    }
}
/// n = 13, i = 5
fn select_282([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_283([a, b, c, e, i, f, h, k, j, l, m], is_dual)
    } else {
        select_285([a, b, d, e, j, g, i, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_277([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_278([b, c, d, e, a, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_282([a, c, d, e, b, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_288([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_153([b, e, d, a, f, h], is_dual)
    } else {
        select_207([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_290([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([a, b, c, f, i, j], is_dual)
    } else {
        select_37([b, c, d, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_289([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_170([a, c, d, g, f, b, h, i, j], is_dual)
    } else {
        select_290([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_287([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_288([a, b, c, j, g, l, i, m], is_dual)
    } else {
        select_289([c, d, f, i, e, g, h, k, j, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_293([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([c, d, e, f], is_dual)
    } else {
        select_30([h, i, a, b, g], !is_dual)
    }
}
/// n = 12, i = 4
fn select_292([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_293([a, h, c, d, e, f, i, k, l], is_dual)
    } else {
        select_61([f, b, c, d, e, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_294([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_92([a, c, d, e, b, f, g], is_dual)
    } else {
        select_92([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 14, i = 5
fn select_291([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_292([a, c, b, d, h, f, g, i, j, l, n, m], is_dual)
    } else {
        select_294([b, i, e, d, f, h, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_286([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_287([b, a, c, d, f, e, h, i, g, j, k, l, m], is_dual)
    } else {
        select_291([c, d, b, e, f, h, g, i, a, j, k, m, l, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_276([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_277([b, a, d, c, f, h, i, j, k, g, l, m, o], is_dual)
    } else {
        select_286([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_264([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_265([l, m, n, o, a, g, j, b, c, h, d, k], !is_dual)
    } else {
        select_276([a, b, d, e, c, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_298([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_294([a, c, e, b, f, g, h], is_dual)
    } else {
        select_294([a, b, d, c, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_297([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_298([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_298([a, c, d, b, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_301([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_216([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_36([b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_302([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_157([b, c, d, e, f, g, i], is_dual)
    } else {
        select_153([a, c, g, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_300([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_301([c, d, a, e, f, b, g, h, i], is_dual)
    } else {
        select_302([c, d, b, e, f, a, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_299([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_148([b, d, a, c, e, f, g, h, j, i], is_dual)
    } else {
        select_300([b, a, d, c, g, f, i, j, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_296([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_297([b, a, d, f, e, i, g, h, j], is_dual)
    } else {
        select_299([a, b, c, d, h, g, j, i, f, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_306([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_89([a, c, d, f, e, g, h], is_dual)
    } else {
        select_35([a, b, g, e, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_305([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_75([a, b, c, f, e, h, g], is_dual)
    } else {
        select_306([a, d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_304([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_241([b, d, c, a, g, e, h, i, f], is_dual)
    } else {
        select_305([b, a, f, d, h, g, e, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_310([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_101([a, d, b, g, e, f, h], is_dual)
    } else {
        select_101([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_309([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_157([a, c, b, i, h, g, k], is_dual)
    } else {
        select_310([c, d, g, e, f, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_312([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([a, b, d, g, f, i], is_dual)
    } else {
        select_27([c, d, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_311([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_312([a, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_36([a, d, c, h, g, e, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_308([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_309([b, c, a, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_311([b, e, f, c, a, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_314([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_46([a, c, g, f, d, i], is_dual)
    } else {
        select_101([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_315([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_45([f, c, b, e, h, g], is_dual)
    } else {
        select_85([a, c, d, g, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_313([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_314([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_315([c, a, d, e, b, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_307([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_308([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_313([a, b, i, d, c, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_303([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_304([d, a, f, e, g, b, h, j, i], is_dual)
    } else {
        select_307([a, c, d, b, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_295([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_296([b, d, c, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_303([b, d, a, e, f, c, g, h, j, i, k], is_dual)
    }
}
/// n = 15, i = 6
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_264([a, b, c, e, d, f, g, h, i, k, l, m, n, j, o], is_dual)
    } else {
        select_295([a, c, h, d, f, b, i, g, k, j, l], is_dual)
    }
}
/// n = 15, i = 6
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_201([c, a, b, d, h, f, g, i, j, k, m, l, n], is_dual)
    } else {
        select_263([a, b, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_15([a, b, d, c, f, e, g, j, i, l, k, m, h, o, n], is_dual)
    } else {
        select_200([a, b, d, e, f, c, g, j, h, l, m, i, n, k, o], is_dual)
    }
}
/// n = 7, i = 2
fn select_324([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([a, e, b, d, g, f], is_dual)
    } else {
        select_53([a, c, f, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_323([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_162([b, a, d, e, g, i], is_dual)
    } else {
        select_324([a, c, b, f, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_325([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([b, a, c, d, e, g, f], is_dual)
    } else {
        select_96([g, e, b, a, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_322([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_323([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_325([g, b, d, e, a, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_328([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_35([g, h, i, a, d, b], !is_dual)
    } else {
        select_85([b, c, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_329([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_45([a, b, c, d, e, f], is_dual)
    } else {
        select_31([a, f, b], is_dual)
    }
}
/// n = 9, i = 4
fn select_327([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_328([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_329([b, d, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_331([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_45([a, c, b, d, e, g], is_dual)
    } else {
        select_97([b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_330([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_331([b, e, c, a, f, g, h], is_dual)
    } else {
        select_207([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_326([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_327([a, b, d, f, h, e, i, g, j], is_dual)
    } else {
        select_330([b, a, c, e, g, f, i, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_322([b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_326([b, a, i, d, g, j, h, f, l, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_335([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_57([a, b, c, d, e], is_dual)
    } else {
        select_31([e, a, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_334([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_335([f, h, i, a, b, c], !is_dual)
    } else {
        select_162([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_333([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_75([a, c, d, b, f, g, h], is_dual)
    } else {
        select_334([a, b, e, c, d, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_337([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_157([a, b, c, d, e, f, g], is_dual)
    } else {
        select_32([a, b, f, g, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_73([a, e, f, h, j], is_dual)
    } else {
        select_69([b, c, d, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_336([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_337([d, g, e, f, a, h, i], is_dual)
    } else {
        select_338([d, b, c, a, e, f, h, g, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_332([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_333([a, b, d, e, h, g, j, i, l], is_dual)
    } else {
        select_336([b, d, e, c, f, g, i, h, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_320([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_321([b, c, e, d, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_332([b, c, e, a, d, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_342([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_170([a, d, e, b, f, c, g, h, i], is_dual)
    } else {
        select_170([a, d, e, c, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_343([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_103([g, i, e, f, h, a, d, b], !is_dual)
    } else {
        select_29([g, i, h, f, e, a, d, c, b], !is_dual)
    }
}
/// n = 13, i = 5
fn select_341([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_342([b, d, e, f, h, g, j, i, l], is_dual)
    } else {
        select_343([k, l, m, h, a, g, b, c, i], !is_dual)
    }
}
/// n = 11, i = 5
fn select_345([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_29([h, j, k, d, g, a, f, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_347([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_31([a, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_83([a, b, d, f, g, h, i], is_dual)
    } else {
        select_28([b, e, c, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_346([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_347([a, c, e, d, g, f, i, j], is_dual)
    } else {
        select_348([a, b, f, d, g, e, h, i, j], is_dual)
    }
}
/// n = 13, i = 6
fn select_344([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_345([j, k, m, g, i, l, e, a, b, f, c], !is_dual)
    } else {
        select_346([a, c, d, g, e, f, h, j, l, m], is_dual)
    }
}
/// n = 14, i = 6
fn select_340([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_341([b, d, c, a, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_344([j, k, l, n, a, m, h, b, g, c, d, i, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_352([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_46([b, c, d, e, f, g], is_dual)
    } else {
        select_85([a, c, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_351([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_352([c, b, d, f, e, g, i, h], is_dual)
    } else {
        select_22([a, c, e, f, g, d, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_350([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_115([g, a, c, d, f, i, j, h], is_dual)
    } else {
        select_351([a, f, b, d, h, e, j, k, i, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_354([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_162([a, b, d, e, c, f], is_dual)
    } else {
        select_162([a, c, d, e, b, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_356([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_51([c, d, e, i], is_dual)
    } else {
        select_101([a, b, h, f, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_355([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_356([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_55([a, h, c, d, e, f, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_353([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_354([b, c, h, e, f, j], is_dual)
    } else {
        select_355([a, d, b, c, e, f, g, h, i, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_350([b, d, c, f, g, i, h, a, j, k, l], is_dual)
    } else {
        select_353([b, a, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 14, i = 6
fn select_339([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_340([a, c, b, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_349([a, d, e, c, f, h, g, b, i, j, k, l], is_dual)
    }
}
/// n = 15, i = 6
fn select_319([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_320([a, c, d, f, b, h, g, i, j, l, m, k], is_dual)
    } else {
        select_339([a, b, c, e, d, i, g, h, k, m, n, o, j, l], is_dual)
    }
}
/// n = 5, i = 2
fn select_362([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_53([a, b, c, d], is_dual)
    } else {
        select_53([a, b, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_361([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_191([a, c, e, d, f, g], is_dual)
    } else {
        select_362([d, b, g, e, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_364([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([b, f, d, e, h, i], is_dual)
    } else {
        select_53([a, c, g, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_363([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_364([a, b, c, d, f, h, j, i, k, l], is_dual)
    } else {
        select_231([b, d, e, i, f, g, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_360([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_361([l, m, h, a, j, b, k, c], !is_dual)
    } else {
        select_363([a, b, c, d, e, f, g, h, i, j, k, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_366([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_96([e, b, a, d, f], is_dual)
    } else {
        select_153([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_368([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_23([b, c, d, e, g, f, h], is_dual)
    } else {
        select_28([a, h, b, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_367([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_144([b, e, d, a, f, g, h, i], is_dual)
    } else {
        select_368([b, d, a, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_365([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_366([h, e, j, a, b, i], !is_dual)
    } else {
        select_367([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_359([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_360([a, b, d, c, e, f, g, h, j, l, i, k, m], is_dual)
    } else {
        select_365([a, b, c, g, h, i, j, k, f, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_371([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_144([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_195([b, a, c, h, e, i, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_372([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_183([b, c, a, d, f, e, h], is_dual)
    } else {
        select_41([a, d, f, g, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_370([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_371([c, b, e, a, g, f, j, i, h], is_dual)
    } else {
        select_372([j, f, l, a, g, k, h, d, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_374([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_183([b, e, c, d, g, f, h], is_dual)
    } else {
        select_67([d, a, h, i, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_376([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_45([a, b, c, e, d, g], is_dual)
    } else {
        select_45([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_375([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([c, b, e, h, f, g], is_dual)
    } else {
        select_376([g, a, c, d, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_373([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_374([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_375([c, a, b, f, d, e, g, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_369([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_370([a, c, b, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_373([i, j, l, g, f, k, a, b, h, d], !is_dual)
    }
}
/// n = 13, i = 5
fn select_358([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_359([a, c, b, d, e, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_369([a, b, c, d, g, h, i, j, k, l, f, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_381([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([a, b, c, d, e, g], is_dual)
    } else {
        select_30([f, g, a, d, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_380([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_98([f, g, h, d, a, e, c], !is_dual)
    } else {
        select_381([a, b, e, g, f, h, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_379([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_380([b, e, d, a, f, h, g, j], is_dual)
    } else {
        select_313([b, c, d, a, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_383([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_178([a, c, e, d, f, h], is_dual)
    } else {
        select_34([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_382([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_383([d, h, i, e, g, f, a, b], !is_dual)
    } else {
        select_21([e, h, i, d, f, g, b, a, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_378([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_379([a, c, b, e, g, f, i, j, k, h], is_dual)
    } else {
        select_382([k, l, g, a, b, i, h, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_386([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_144([d, b, c, e, f, g, h, j], is_dual)
    } else {
        select_32([a, f, g, i, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_387([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_229([b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_136([i, f, j, g, h, a, c, b, e], !is_dual)
    }
}
/// n = 12, i = 5
fn select_385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_386([h, k, l, e, i, j, a, f, g, c, d], !is_dual)
    } else {
        select_387([e, a, b, d, g, h, f, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_389([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_144([d, b, c, e, f, h, g, i], is_dual)
    } else {
        select_243([a, b, f, e, g, h, d, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_390([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_67([e, g, b, f, d], is_dual)
    } else {
        select_195([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_388([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_389([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_390([h, d, j, i, g, a, e, f], !is_dual)
    }
}
/// n = 12, i = 5
fn select_384([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_385([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    } else {
        select_388([h, l, k, e, i, f, j, a, b, g], !is_dual)
    }
}
/// n = 13, i = 5
fn select_377([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_378([f, a, b, d, e, g, h, i, j, k, l, m], is_dual)
    } else {
        select_384([b, c, d, e, h, f, g, k, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_357([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_358([c, a, d, e, f, b, g, h, j, i, k, l, m], is_dual)
    } else {
        select_377([c, d, b, e, f, a, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_318([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_319([a, c, b, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    } else {
        select_357([b, a, c, d, h, f, i, j, k, l, m, g, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_396([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_157([a, b, c, d, e, f, g], is_dual)
    } else {
        select_45([a, h, g, e, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_395([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_189([a, b, d, e, c, f, g], is_dual)
    } else {
        select_396([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_398([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_74([a, d, e, g, f, i], is_dual)
    } else {
        select_76([d, b, c, f, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_399([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_328([a, b, c, g, e, i, h, f, j], is_dual)
    } else {
        select_76([a, b, f, d, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_397([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_398([d, a, c, b, e, g, f, i, h], is_dual)
    } else {
        select_399([b, d, c, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_394([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_395([a, c, b, e, f, h, j, g], is_dual)
    } else {
        select_397([a, c, d, b, e, f, g, j, i, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_393([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_394([a, b, c, d, i, h, g, k, j, l], is_dual)
    } else {
        select_47([a, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_403([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_237([a, f, b, d, e, h, i], is_dual)
    } else {
        select_237([a, e, c, d, f, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_404([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_109([a, b, c, f, g, e, h], is_dual)
    } else {
        select_162([a, c, d, e, g, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_403([b, a, g, e, f, i, j, k, h], is_dual)
    } else {
        select_404([h, b, d, c, e, g, j, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_407([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_101([b, c, d, e, f, g, h], is_dual)
    } else {
        select_53([a, j, k, i], !is_dual)
    }
}
/// n = 11, i = 4
fn select_408([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_101([c, d, g, e, f, i, j], is_dual)
    } else {
        select_53([a, b, h, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_406([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_407([a, c, d, g, e, f, i, j, k, h, l], is_dual)
    } else {
        select_408([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_410([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_57([b, a, c, d, e], is_dual)
    } else {
        select_27([b, e, d, a], is_dual)
    }
}
/// n = 10, i = 3
fn select_409([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_216([a, b, c, d, g, i, h, j], is_dual)
    } else {
        select_410([b, d, e, h, f], is_dual)
    }
}
/// n = 14, i = 5
fn select_405([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_406([a, c, b, g, f, j, i, h, k, l, m, n], is_dual)
    } else {
        select_409([a, h, c, e, d, g, i, k, j, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_401([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_402([c, f, e, d, a, g, h, i, j, k, l], is_dual)
    } else {
        select_405([b, c, a, e, d, g, h, i, f, j, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_413([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_103([a, d, b, e, c, f, g, h], is_dual)
    } else {
        select_103([a, d, c, e, b, f, g, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_412([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_413([a, b, c, j, g, l, i, m], is_dual)
    } else {
        select_289([a, d, f, i, e, g, h, k, j, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_416([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_83([a, c, d, e, f, g, h], is_dual)
    } else {
        select_28([b, f, c, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_417([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_23([b, c, d, f, h, g, i], is_dual)
    } else {
        select_83([a, b, e, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_415([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_416([a, b, d, e, h, i, g, k], is_dual)
    } else {
        select_417([a, c, b, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_414([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_415([a, b, d, c, e, f, h, g, i, j, k], is_dual)
    } else {
        select_416([a, b, g, e, c, i, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_411([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_412([b, c, d, a, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_414([b, c, d, h, g, j, a, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_400([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_401([b, c, d, e, f, g, h, i, a, j, k, l, m, n], is_dual)
    } else {
        select_411([b, c, a, d, f, e, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_392([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_393([a, c, d, f, b, h, g, i, j, l, n, k], is_dual)
    } else {
        select_400([a, b, c, e, d, f, g, h, i, k, l, m, j, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_422([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_69([a, c, b, d, e], is_dual)
    } else {
        select_67([a, c, e, b, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_423([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_73([b, c, d, e, f], is_dual)
    } else {
        select_191([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_421([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_422([a, i, j, h, e, c], !is_dual)
    } else {
        select_423([g, h, i, j, a, f, b, d], !is_dual)
    }
}
/// n = 11, i = 4
fn select_420([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_421([a, b, c, f, e, i, h, g, j, k], is_dual)
    } else {
        select_86([a, b, d, g, h, f, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_426([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_191([c, d, g, e, f, a], !is_dual)
    } else {
        select_117([c, e, g, d, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_427([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_162([a, c, d, e, f, g], is_dual)
    } else {
        select_225([b, a, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_425([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_426([g, c, a, i, f, e, j], is_dual)
    } else {
        select_427([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_429([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_229([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_229([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_430([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_183([b, c, d, a, e, f, h], is_dual)
    } else {
        select_183([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_428([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_429([b, e, a, d, g, f, i, h], is_dual)
    } else {
        select_430([a, b, f, c, h, e, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_424([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_425([a, b, c, f, e, i, j, h, k, g], is_dual)
    } else {
        select_428([a, b, e, d, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_419([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_420([a, c, d, b, f, e, h, i, g, k, j], is_dual)
    } else {
        select_424([a, b, d, c, f, g, h, i, j, e, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_435([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([a, f, d, e, h, j], is_dual)
    } else {
        select_28([b, c, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_436([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_28([c, d, e, g], is_dual)
    } else {
        select_28([a, b, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_434([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_435([a, c, d, b, f, g, e, i, h, j], is_dual)
    } else {
        select_436([b, e, c, d, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_437([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_96([f, e, a, c, d], !is_dual)
    } else {
        select_34([f, g, d, a, b, e, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_433([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_434([b, c, d, e, g, f, h, j, i, k], is_dual)
    } else {
        select_437([a, b, c, i, f, g, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_432([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_433([b, a, c, d, e, g, f, h, j, i, k], is_dual)
    } else {
        select_414([c, d, e, b, g, f, a, h, i, k, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_440([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_69([b, e, c, d, f], is_dual)
    } else {
        select_89([a, b, c, d, e, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_439([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_440([a, b, d, h, f, j, g, i], is_dual)
    } else {
        select_325([a, c, e, g, f, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_442([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_109([a, d, b, c, f, g, e], is_dual)
    } else {
        select_27([d, e, g, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_443([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_157([b, c, d, e, f, g, h], is_dual)
    } else {
        select_216([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_441([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_442([b, c, g, e, f, h, i], is_dual)
    } else {
        select_443([c, a, e, d, f, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_438([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_439([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_441([c, a, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_431([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_432([a, b, d, c, e, h, g, j, l, k, i], is_dual)
    } else {
        select_438([a, c, b, e, f, g, h, i, l, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_418([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_419([b, c, e, d, i, h, g, a, k, j, l], is_dual)
    } else {
        select_431([b, c, a, e, d, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_391([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_392([a, c, b, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    } else {
        select_418([a, b, c, d, f, h, i, j, k, l, g, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_317([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_318([a, c, b, e, d, f, h, i, j, g, l, m, k, n, o], is_dual)
    } else {
        select_391([a, c, d, e, b, f, h, g, j, k, i, l, n, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_451([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_97([a, g, h], is_dual)
    } else {
        select_101([a, c, b, d, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_450([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_451([a, e, c, d, f, h, g, i], is_dual)
    } else {
        select_34([a, b, g, d, j, e, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_449([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_115([a, c, d, e, g, i, j, h], is_dual)
    } else {
        select_450([c, b, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_453([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_67([g, b, h, a, j], is_dual)
    } else {
        select_168([b, d, a, c, e, f, g, i, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_455([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_45([a, b, c, e, d, f], is_dual)
    } else {
        select_27([c, d, e, b], is_dual)
    }
}
/// n = 8, i = 3
fn select_454([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_112([a, b, c, d, e, g, h, f], is_dual)
    } else {
        select_455([a, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_452([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_453([a, b, e, d, f, g, i, h, k, j], is_dual)
    } else {
        select_454([a, b, c, h, g, j, e, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_448([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_449([b, c, d, e, h, f, g, i, k, j, l], is_dual)
    } else {
        select_452([a, d, c, e, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_459([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_35([b, c, d, g, f, i], is_dual)
    } else {
        select_85([i, j, e, h, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_458([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_459([a, b, c, g, e, f, h, i, j, k], is_dual)
    } else {
        select_459([a, b, d, f, e, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_457([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_102([a, c, d, g, f, i, j, h, l], is_dual)
    } else {
        select_458([a, b, c, e, f, h, g, j, i, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_461([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_98([h, i, g, a, b, e, c], !is_dual)
    } else {
        select_98([g, i, h, a, b, f, d], !is_dual)
    }
}
/// n = 8, i = 2
fn select_463([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_23([a, c, d, e, g, f, h], is_dual)
    } else {
        select_27([a, b, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_462([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_237([f, d, b, a, e, h, g], is_dual)
    } else {
        select_463([b, d, a, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_460([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_461([a, b, c, d, h, f, g, i, j], is_dual)
    } else {
        select_462([a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_456([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_457([b, c, a, h, d, f, g, i, j, l, k, m], is_dual)
    } else {
        select_460([a, c, g, e, f, h, j, k, l, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_447([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_448([a, b, c, g, e, h, i, f, j, l, k, m], is_dual)
    } else {
        select_456([b, c, d, a, e, h, f, i, j, g, k, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_468([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_45([a, b, d, g, h, j], is_dual)
    } else {
        select_85([c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_467([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_66([f, d, a, g, e, i], is_dual)
    } else {
        select_468([b, c, d, a, e, f, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_466([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_467([a, b, d, c, h, g, i, f, j, k], is_dual)
    } else {
        select_409([b, c, d, e, a, f, g, i, h, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_470([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_66([e, b, a, g, d, f], is_dual)
    } else {
        select_23([b, a, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_469([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_470([a, c, d, f, g, h, i], is_dual)
    } else {
        select_429([a, h, b, d, e, g, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_465([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_466([a, c, b, d, g, f, h, j, k, l, i], is_dual)
    } else {
        select_469([b, c, a, e, h, f, g, i, k, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_473([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_281([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_66([g, b, a, h, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_474([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_70([a, b, c, f, e, g], is_dual)
    } else {
        select_70([a, b, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_472([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_473([b, c, a, f, e, g, h, j], is_dual)
    } else {
        select_474([a, g, b, d, i, f, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_475([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_90([a, d, e, b, h, i, g], is_dual)
    } else {
        select_473([a, c, b, g, f, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_471([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_472([a, b, d, c, g, h, i, f, j, k, l], is_dual)
    } else {
        select_475([b, c, d, e, a, g, f, j, i, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_464([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_465([a, b, c, h, e, f, i, g, k, j, l, m], is_dual)
    } else {
        select_471([a, b, d, e, g, f, i, h, l, j, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_446([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_447([b, a, e, d, c, g, h, j, i, f, l, m, k], is_dual)
    } else {
        select_464([b, c, d, e, a, g, h, j, f, i, l, k, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_481([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_53([e, b, c, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_480([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_281([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_481([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_482([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_183([a, f, b, e, i, g, j], is_dual)
    } else {
        select_231([e, c, d, h, f, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_479([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_480([c, b, d, f, e, g, i, h, j, k], is_dual)
    } else {
        select_482([b, h, c, a, e, g, f, j, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_483([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_122([f, b, c, a, e, g, h], is_dual)
    } else {
        select_190([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_478([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_479([c, b, d, a, f, g, h, e, i, j, k], is_dual)
    } else {
        select_483([i, l, e, m, a, f, k, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_486([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_237([b, d, c, f, e, g, h], is_dual)
    } else {
        select_98([a, d, b, h, f, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_485([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_486([a, e, d, c, g, h, f, i, k], is_dual)
    } else {
        select_399([a, b, d, f, g, i, e, j, h, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_488([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_43([a, c, d, b, e, f, g], is_dual)
    } else {
        select_43([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_489([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_183([b, c, d, a, e, f, g], is_dual)
    } else {
        select_91([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_487([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_488([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_489([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_485([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_487([i, k, l, e, b, a, j, f], !is_dual)
    }
}
/// n = 14, i = 5
fn select_477([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_478([a, b, g, d, e, f, i, h, l, k, j, n, m], is_dual)
    } else {
        select_484([a, f, h, c, e, j, g, k, l, i, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_493([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_281([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_23([a, b, h, e, g, f, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_495([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, b, f, d, g, e, h], is_dual)
    } else {
        select_23([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_494([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_70([a, g, j, k, i, f], !is_dual)
    } else {
        select_495([b, a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 14, i = 5
fn select_492([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_493([b, c, d, e, g, h, j, i, k], is_dual)
    } else {
        select_494([a, c, b, f, h, g, k, j, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_498([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_23([a, c, d, e, g, f, i], is_dual)
    } else {
        select_28([a, h, b, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_497([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_225([e, c, d, a, f, g, h], is_dual)
    } else {
        select_498([b, a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_496([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_422([a, h, i, k, j, b], !is_dual)
    } else {
        select_497([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_491([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_492([a, d, e, b, c, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_496([j, m, l, n, f, a, i, b, c, k, h], !is_dual)
    }
}
/// n = 9, i = 4
fn select_501([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_183([a, b, c, d, e, f, g], is_dual)
    } else {
        select_73([e, a, h, i, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_503([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_101([b, c, a, d, e, g, f], is_dual)
    } else {
        select_30([b, f, d, a, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_504([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_45([a, g, c, f, d, i], is_dual)
    } else {
        select_101([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_502([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_503([a, e, c, g, f, h, d, i], is_dual)
    } else {
        select_504([a, c, b, d, f, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_500([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_501([a, f, b, d, h, e, i, g, j], is_dual)
    } else {
        select_502([a, c, b, d, f, e, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_506([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_96([a, f, d, g, h], is_dual)
    } else {
        select_225([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([a, c, d, e, g, f, i], is_dual)
    } else {
        select_46([a, b, f, e, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_509([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, b, c, e], is_dual)
    } else {
        select_28([a, c, b, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_507([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_508([a, b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_509([h, a, b, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_505([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_506([e, c, d, a, f, g, h, j], is_dual)
    } else {
        select_507([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_499([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_500([a, b, d, e, g, f, i, j, h, l], is_dual)
    } else {
        select_505([j, k, i, l, e, a, g, b, c, h], !is_dual)
    }
}
/// n = 14, i = 5
fn select_490([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_491([a, b, c, d, e, f, g, h, i, k, j, l, m, n], is_dual)
    } else {
        select_499([a, b, c, d, f, j, i, g, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_476([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_477([b, d, c, e, f, h, g, i, a, j, k, l, n, m], is_dual)
    } else {
        select_490([b, c, a, d, e, f, h, i, g, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_445([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_446([b, c, a, d, e, g, h, i, f, j, l, m, k], is_dual)
    } else {
        select_476([a, b, c, d, e, g, h, i, j, k, l, m, f, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_515([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_41([a, g, f, i, j], is_dual)
    } else {
        select_495([b, a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_516([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_89([a, c, d, g, e, h, j], is_dual)
    } else {
        select_284([a, h, b, e, f, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_514([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_515([a, c, g, e, f, b, h, i, j, k], is_dual)
    } else {
        select_516([a, c, b, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_517([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_97([a, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_513([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_514([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_517([j, h, k, e, a, f, b, i], !is_dual)
    }
}
/// n = 8, i = 3
fn select_520([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_191([d, b, c, e, f, g], is_dual)
    } else {
        select_76([h, e, f, d, g, a], !is_dual)
    }
}
/// n = 8, i = 3
fn select_519([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_520([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_123([c, e, g, a, h, d, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_518([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_519([i, j, f, a, h, g, e, b], !is_dual)
    } else {
        select_396([a, e, c, d, f, h, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_512([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_513([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_518([a, b, c, g, f, h, i, j, e, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_525([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_57([a, c, d, e, h], is_dual)
    } else {
        select_101([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_157([a, d, c, i, g, h, k], is_dual)
    } else {
        select_525([b, d, e, h, f, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_526([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_183([b, d, c, e, h, g, i], is_dual)
    } else {
        select_237([b, d, a, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_523([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_524([b, d, a, c, e, h, g, f, i, j, k], is_dual)
    } else {
        select_526([d, f, c, e, a, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_528([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_410([a, c, i, f, k], is_dual)
    } else {
        select_162([b, d, e, h, g, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_529([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_162([b, e, d, h, g, j], is_dual)
    } else {
        select_117([a, i, c, k, f, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_527([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_528([b, c, a, e, d, f, h, g, i, j, k], is_dual)
    } else {
        select_529([a, c, b, d, e, g, h, f, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_522([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_523([b, a, e, d, g, f, i, h, j, l, k], is_dual)
    } else {
        select_527([a, c, b, i, d, f, g, h, k, l, m, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_532([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_66([a, c, e, f, d, h], is_dual)
    } else {
        select_153([a, d, b, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_531([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_413([f, g, h, i, a, d, e, c], !is_dual)
    } else {
        select_532([a, b, c, e, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_535([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, c, d, e, h, j], is_dual)
    } else {
        select_35([b, c, d, f, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_536([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_83([a, b, h, d, i, f, g], is_dual)
    } else {
        select_28([a, g, c, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_534([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_535([h, i, j, k, f, a, d, g, b, e], !is_dual)
    } else {
        select_536([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_537([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_481([e, b, c, g, a, f, h, i], is_dual)
    } else {
        select_368([a, d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_533([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_534([a, b, c, f, g, i, j, k, l, h, m], is_dual)
    } else {
        select_537([a, d, e, c, g, i, h, k, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_530([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_531([a, h, c, b, f, k, i, m, j], is_dual)
    } else {
        select_533([a, c, d, b, e, f, g, i, j, h, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_521([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_522([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_530([a, b, d, c, i, h, g, k, j, l, f, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_511([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_512([a, d, c, b, f, h, j, k, m, g, l], is_dual)
    } else {
        select_521([a, b, d, e, c, f, h, i, g, k, l, j, m, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_542([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([b, c, e, g, f, h], is_dual)
    } else {
        select_503([h, g, j, d, i, e, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_65([b, c, d, h, f, g, i], is_dual)
    } else {
        select_542([a, c, d, e, g, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_544([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_162([b, f, d, e, i, j], is_dual)
    } else {
        select_504([a, c, b, f, g, h, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_546([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_30([a, d, g, f, c], is_dual)
    } else {
        select_30([b, c, g, e, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_157([b, e, c, d, f, h, g], is_dual)
    } else {
        select_546([f, i, e, h, a, b, g], !is_dual)
    }
}
/// n = 12, i = 4
fn select_543([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_544([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_545([a, b, d, e, h, f, j, i, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_540([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_541([a, c, i, d, f, h, g, k, j, l, m], is_dual)
    } else {
        select_543([a, b, d, e, g, f, h, j, i, l, k, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_549([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_503([i, h, k, e, j, f, a, b], !is_dual)
    } else {
        select_225([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_548([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_549([a, b, c, h, e, f, g, i, j, k, l], is_dual)
    } else {
        select_549([a, b, d, g, e, f, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_550([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_442([a, b, c, f, g, h, e], is_dual)
    } else {
        select_86([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_547([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_548([a, c, b, d, e, g, f, h, i, j, k, l], is_dual)
    } else {
        select_550([e, b, c, h, g, j, i, f], is_dual)
    }
}
/// n = 13, i = 4
fn select_539([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_540([b, c, d, e, a, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_547([b, d, g, e, f, h, a, i, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_553([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_288([e, a, b, c, h, f, g, i], is_dual)
    } else {
        select_196([b, a, d, f, g, h, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_555([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_191([b, c, d, f, e, g], is_dual)
    } else {
        select_76([f, h, i, d, a, g], !is_dual)
    }
}
/// n = 6, i = 2
fn select_556([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_69([a, b, c, d, f], is_dual)
    } else {
        select_69([a, c, b, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_554([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_555([a, c, d, f, h, e, g, i, j], is_dual)
    } else {
        select_556([i, e, j, f, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_552([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_553([e, b, c, d, g, f, i, h, j], is_dual)
    } else {
        select_554([a, f, b, d, g, e, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_559([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_153([b, g, c, a, f, h], is_dual)
    } else {
        select_52([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_558([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_559([f, a, d, b, e, g, h, j], is_dual)
    } else {
        select_162([b, c, e, g, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_561([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_183([a, b, c, d, e, f, h], is_dual)
    } else {
        select_67([e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_560([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_131([e, b, f, d, g, h], is_dual)
    } else {
        select_561([a, c, b, e, g, h, i, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_557([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_558([c, b, d, e, f, g, i, h, k, j], is_dual)
    } else {
        select_560([a, c, h, e, g, f, j, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_551([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_552([b, d, g, e, f, a, h, i, j, l, k], is_dual)
    } else {
        select_557([b, d, c, e, a, f, h, i, g, k, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_539([b, c, d, a, e, g, h, f, i, j, l, k, m], is_dual)
    } else {
        select_551([a, c, d, b, i, g, h, j, k, l, f, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_510([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_511([a, b, d, e, c, f, i, j, h, g, l, m, k, n], is_dual)
    } else {
        select_538([a, c, d, e, b, f, i, j, g, k, h, m, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_444([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_445([a, b, e, d, f, g, h, i, c, j, l, k, m, n], is_dual)
    } else {
        select_510([a, b, e, c, f, g, h, i, d, j, k, m, l, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_316([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_317([a, b, c, d, e, f, g, h, i, j, l, m, n, o, k], is_dual)
    } else {
        select_444([c, b, e, a, f, d, h, g, j, i, l, k, n, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_14([a, b, c, d, f, e, g, h, k, j, m, l, i, n, o], is_dual)
    } else {
        select_316([a, b, c, f, e, d, g, h, k, i, m, l, j, o, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_570([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_237([d, b, c, g, f, e, h], is_dual)
    } else {
        select_352([c, a, e, f, d, h, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_569([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_570([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_45([i, j, d, a, e, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_572([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_237([d, b, c, g, f, e, h], is_dual)
    } else {
        select_22([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_571([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_181([a, h, e, i, j], is_dual)
    } else {
        select_572([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_568([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_569([g, k, j, d, i, e, a, h, b, f], !is_dual)
    } else {
        select_571([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_575([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_157([a, d, b, c, e, f, g], is_dual)
    } else {
        select_32([f, d, a, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_574([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_181([a, f, e, b, g], !is_dual)
    } else {
        select_575([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_577([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_191([b, c, d, g, f, i], is_dual)
    } else {
        select_123([g, j, e, h, a, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_579([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_97([a, f, g], is_dual)
    } else {
        select_85([a, b, c, d, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_578([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_579([a, c, d, f, g, h, i], is_dual)
    } else {
        select_117([g, b, c, e, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_576([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_577([a, b, d, f, e, g, i, h, k, j], is_dual)
    } else {
        select_578([a, b, c, e, h, f, i, g, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_573([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_574([g, k, e, l, h, a, j], !is_dual)
    } else {
        select_576([a, b, d, c, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 13, i = 6
fn select_567([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_568([j, i, m, d, h, k, a, e, g, f, c], !is_dual)
    } else {
        select_573([j, h, l, m, d, k, e, i, a, f, g, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_584([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, c, d, e, h, g, j], is_dual)
    } else {
        select_23([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_583([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_34([e, j, k, a, h, f, b], !is_dual)
    } else {
        select_584([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_586([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([f, b, a, e, h, i], is_dual)
    } else {
        select_53([g, c, d, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_585([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_586([h, i, j, k, e, f, a, b, g], !is_dual)
    } else {
        select_36([b, d, c, e, f, h, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_582([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_583([a, d, b, c, e, f, g, h, i, k, j], is_dual)
    } else {
        select_585([i, k, l, h, e, g, j, a, f, b, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_589([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([a, b, c, d, g, e, f], is_dual)
    } else {
        select_23([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_588([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([a, c, e, f, g, d, h, i], is_dual)
    } else {
        select_589([c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_587([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_422([a, g, i, k, j, e], !is_dual)
    } else {
        select_588([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_581([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_582([a, b, g, c, e, f, i, k, j, l, h, m], is_dual)
    } else {
        select_587([j, m, k, e, l, i, h, a, b, g, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_592([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([b, c, e, g, f, h], is_dual)
    } else {
        select_197([h, g, j, i, d, e, a, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_594([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_45([e, d, g, f, a, b], !is_dual)
    } else {
        select_30([b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_593([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_594([f, d, h, a, b, g, e], !is_dual)
    } else {
        select_455([b, d, c, f, e, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_591([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_592([g, j, k, d, h, i, a, e, f, c], !is_dual)
    } else {
        select_593([d, b, c, g, f, e, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_596([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_117([g, b, c, e, f, h], is_dual)
    } else {
        select_504([g, k, j, d, i, h, e, a, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_597([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_250([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_34([d, i, j, a, g, f, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_595([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_596([h, i, k, d, e, j, a, g, f, c, b], !is_dual)
    } else {
        select_597([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_590([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_591([j, k, l, d, g, h, a, i, e, f, b], !is_dual)
    } else {
        select_595([a, e, c, d, g, h, f, j, k, i, l], is_dual)
    }
}
/// n = 14, i = 6
fn select_580([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_581([b, a, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_590([b, g, c, e, i, f, a, j, h, k, l, n], is_dual)
    }
}
/// n = 14, i = 6
fn select_566([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_567([k, n, j, e, h, m, l, a, i, b, f, g, c], !is_dual)
    } else {
        select_580([a, b, d, c, e, g, f, i, h, j, k, m, n, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_601([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_226([e, b, d, g, f, h, i, j], is_dual)
    } else {
        select_226([f, a, c, h, e, g, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_602([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_90([c, d, f, e, g, h, i], is_dual)
    } else {
        select_92([a, b, g, h, e, f, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_600([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_601([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_602([c, e, a, d, f, h, b, g, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_604([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_49([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_212([b, h, c, f, a, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_606([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_145([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_145([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_607([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_91([a, c, d, b, e, f], is_dual)
    } else {
        select_91([b, c, d, a, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_605([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_606([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_607([h, i, f, j, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_603([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_604([b, a, e, c, d, f, g, h, i, j], is_dual)
    } else {
        select_605([a, b, c, d, g, f, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_599([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_600([a, c, d, e, b, g, h, f, j, i], is_dual)
    } else {
        select_603([b, a, c, h, d, f, g, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_611([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_509([a, b, h, i, j], is_dual)
    } else {
        select_324([c, a, d, f, e, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_612([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_96([h, b, j, d, i], is_dual)
    } else {
        select_536([a, b, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_610([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_611([a, b, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_612([c, b, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_615([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, b, e, f, g], is_dual)
    } else {
        select_23([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_614([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_481([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_615([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_613([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_614([g, i, h, j, d, f, a, c], !is_dual)
    } else {
        select_429([d, h, g, k, e, j, c, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_609([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_610([g, d, i, k, h, j, e, f, a, b], !is_dual)
    } else {
        select_613([b, c, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_618([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_281([b, c, d, e, g, i, h, j], is_dual)
    } else {
        select_416([j, k, m, f, l, a, g, b], !is_dual)
    }
}
/// n = 8, i = 2
fn select_619([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_281([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_85([b, g, e, a, f, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_617([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_618([a, c, b, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_619([b, c, d, e, g, i, j, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_621([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_250([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_329([a, g, c, f, d, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_622([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_416([g, i, h, d, e, a, f, b], !is_dual)
    } else {
        select_284([a, c, b, d, e, g, h, f, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_620([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_621([b, a, c, d, e, f, g, j, i], is_dual)
    } else {
        select_622([a, b, f, e, d, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_616([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_617([b, a, c, d, e, f, g, h, j, i, l, k, m], is_dual)
    } else {
        select_620([b, h, c, a, f, g, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_608([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_609([l, m, j, a, f, k, b, h, i, g, e], !is_dual)
    } else {
        select_616([a, b, e, c, d, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_598([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_599([a, b, c, g, e, f, i, h, l, j, k, n, m], is_dual)
    } else {
        select_608([a, b, c, h, d, f, g, j, k, l, i, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_565([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_566([b, c, h, d, f, j, g, a, i, k, l, m, n, o], is_dual)
    } else {
        select_598([b, c, a, d, e, f, g, h, i, k, j, l, n, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_628([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_183([c, d, a, e, f, b, g], is_dual)
    } else {
        select_183([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_627([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_628([b, c, d, g, f, a, h], is_dual)
    } else {
        select_294([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_629([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_606([a, f, e, h, i, g, b, c], !is_dual)
    } else {
        select_489([a, b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_626([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_627([b, c, d, e, a, g, f, h], is_dual)
    } else {
        select_629([g, i, f, j, a, b, h, c, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_632([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_183([a, e, c, d, g, f, i], is_dual)
    } else {
        select_117([d, f, b, h, e, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_631([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_632([e, c, d, a, f, g, h, i, k], is_dual)
    } else {
        select_249([b, c, a, d, f, g, h, e, j, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_630([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_631([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_487([h, k, l, e, a, b, j, g], !is_dual)
    }
}
/// n = 13, i = 5
fn select_625([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_626([b, a, c, h, e, f, j, i, m, l], is_dual)
    } else {
        select_630([a, b, d, c, f, g, i, j, k, h, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_636([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_69([a, h, i, j, b], !is_dual)
    } else {
        select_89([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_635([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_636([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_75([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_638([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_34([a, f, b, d, g, e, h], is_dual)
    } else {
        select_34([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_639([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_66([h, i, a, e, g, f], !is_dual)
    } else {
        select_183([a, c, b, d, f, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_637([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_638([f, d, i, a, g, h, b, e], !is_dual)
    } else {
        select_639([f, g, i, a, d, h, e, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_634([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_635([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_637([a, c, d, g, h, f, i, b, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_642([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_50([c, d, a, e, f, g, h], is_dual)
    } else {
        select_162([a, b, e, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_641([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_642([b, d, a, c, e, g, f, i, h], is_dual)
    } else {
        select_404([b, f, e, d, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_644([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_69([a, b, d, i, k], is_dual)
    } else {
        select_110([c, e, f, h, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_645([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_281([a, c, d, f, e, g, i, j], is_dual)
    } else {
        select_162([g, b, c, d, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_643([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_644([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_645([b, d, e, f, h, g, a, j, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_640([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_641([a, c, e, f, i, g, h, j, l], is_dual)
    } else {
        select_643([a, b, d, e, c, h, g, i, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_633([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_634([b, a, j, c, e, g, l, i, m, n], is_dual)
    } else {
        select_640([a, c, d, e, b, f, g, h, i, k, j, m, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_624([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_625([b, c, d, h, f, g, i, a, j, k, l, n, m], is_dual)
    } else {
        select_633([b, c, d, a, f, e, g, h, j, i, l, k, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_650([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_157([a, c, b, f, e, g, h], is_dual)
    } else {
        select_57([c, d, g, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_649([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_131([a, b, e, f, h, i], is_dual)
    } else {
        select_650([a, b, c, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_652([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_157([a, e, d, g, f, h, j], is_dual)
    } else {
        select_152([b, c, e, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_651([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_215([a, c, b, f, h, i, j, g], is_dual)
    } else {
        select_652([c, a, b, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_648([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_649([a, c, f, e, b, g, h, j, i], is_dual)
    } else {
        select_651([a, c, d, b, e, g, h, f, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_655([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_98([f, h, e, a, b, g, c], !is_dual)
    } else {
        select_117([a, d, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_657([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_45([a, b, c, f, e, g], is_dual)
    } else {
        select_45([a, b, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_656([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_98([f, e, h, a, b, g, c], !is_dual)
    } else {
        select_657([b, d, a, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_654([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_655([a, b, c, f, i, h, e, g], is_dual)
    } else {
        select_656([a, b, e, d, g, i, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_659([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_183([a, b, d, f, e, g, i], is_dual)
    } else {
        select_455([e, a, c, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_658([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_638([b, d, f, g, e, a, i, h], is_dual)
    } else {
        select_659([b, d, c, a, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_653([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_654([c, d, e, b, f, a, i, g, h], is_dual)
    } else {
        select_658([c, d, a, e, f, b, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_647([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_648([b, c, d, a, e, f, g, h, j, i], is_dual)
    } else {
        select_653([b, c, a, d, g, i, h, j, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_664([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_57([b, c, d, e, f], is_dual)
    } else {
        select_53([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_663([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_50([b, c, d, e, f, g, h], is_dual)
    } else {
        select_664([a, h, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_662([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_159([a, g, d, e, f, b, h, i, j], is_dual)
    } else {
        select_663([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_667([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_37([b, c, d, e, g, h], is_dual)
    } else {
        select_57([a, d, e, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_666([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_112([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_667([a, d, e, b, c, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_665([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_159([a, g, d, e, f, b, h, j, i], is_dual)
    } else {
        select_666([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_661([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_662([a, b, c, e, f, g, d, h, j, i], is_dual)
    } else {
        select_665([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_669([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_493([c, a, d, e, f, b, g, h, i], is_dual)
    } else {
        select_493([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_668([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_669([c, d, b, e, f, g, a, h, i], is_dual)
    } else {
        select_253([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_660([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_661([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_668([c, a, e, f, b, d, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_646([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_647([b, c, d, e, f, g, h, i, a, j], is_dual)
    } else {
        select_660([b, c, d, e, a, f, g, i, h, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_623([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_624([a, b, c, d, f, e, g, h, j, i, l, m, k, n], is_dual)
    } else {
        select_646([b, c, j, d, e, a, g, k, l, i], is_dual)
    }
}
/// n = 15, i = 6
fn select_564([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_565([a, c, d, e, f, g, h, i, b, j, k, l, m, n, o], is_dual)
    } else {
        select_623([a, c, d, e, b, f, g, i, h, j, k, m, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_675([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_215([b, c, a, d, e, f, h, g], is_dual)
    } else {
        select_437([b, f, c, a, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_677([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_157([b, d, c, e, f, g, h], is_dual)
    } else {
        select_45([a, g, d, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_676([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_167([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_677([b, c, e, a, f, d, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_674([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_675([a, b, d, c, h, g, f, i], is_dual)
    } else {
        select_676([a, b, d, c, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_680([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_100([b, a, c, d, e, g, f, h], is_dual)
    } else {
        select_27([b, f, d, a], is_dual)
    }
}
/// n = 7, i = 2
fn select_681([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_96([g, f, c, a, b], !is_dual)
    } else {
        select_455([b, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_679([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_680([c, d, b, f, e, a, g, h], is_dual)
    } else {
        select_681([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_683([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_96([b, c, d, e, f], is_dual)
    } else {
        select_117([d, f, g, a, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_684([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([a, b, c, d, e, g, f], is_dual)
    } else {
        select_178([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_683([g, a, b, e, h, f, i], is_dual)
    } else {
        select_684([f, a, c, d, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_678([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_679([e, g, i, j, a, f, b, h], !is_dual)
    } else {
        select_682([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_673([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_674([f, g, j, k, e, a, b, i, h], !is_dual)
    } else {
        select_678([a, b, c, d, f, e, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_687([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_149([b, a, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_135([j, g, k, a, f, i, b, c, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_690([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([a, c, d, e, g, h], is_dual)
    } else {
        select_57([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_689([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_331([a, b, g, f, e, i, j], is_dual)
    } else {
        select_690([b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_692([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_46([a, c, e, h, d, g], is_dual)
    } else {
        select_46([b, c, d, h, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_691([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_67([d, f, c, g, e], is_dual)
    } else {
        select_692([h, i, f, d, g, a, b, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_688([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_689([a, b, d, c, e, g, h, f, i, j], is_dual)
    } else {
        select_691([e, i, j, f, g, a, h, b, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_686([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_687([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_688([g, f, k, l, a, b, i, j, c, h], !is_dual)
    }
}
/// n = 9, i = 2
fn select_694([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_261([a, b, d, g, f, c, h, i], is_dual)
    } else {
        select_258([a, b, d, c, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_695([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_501([f, h, j, a, b, i, g, d, e], !is_dual)
    } else {
        select_112([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_694([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_695([b, a, c, d, g, h, i, f, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_685([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_686([a, b, c, d, e, f, h, i, g, j, k, l], is_dual)
    } else {
        select_693([b, a, c, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_672([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_673([f, i, l, k, a, g, b, j, c, d, h], !is_dual)
    } else {
        select_685([a, b, d, c, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_700([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_66([e, h, a, f, g, b], !is_dual)
    } else {
        select_171([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_699([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_700([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_700([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_702([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_157([e, d, b, c, f, g, h], is_dual)
    } else {
        select_45([g, a, d, f, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_701([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_702([b, a, c, d, e, g, f, h], is_dual)
    } else {
        select_91([b, d, f, e, g, a], is_dual)
    }
}
/// n = 10, i = 4
fn select_698([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_699([f, g, i, e, j, a, h, b], !is_dual)
    } else {
        select_701([b, c, d, a, f, g, e, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_704([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_131([a, c, d, e, f, g], is_dual)
    } else {
        select_556([a, b, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_705([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_191([b, a, c, d, e, g], is_dual)
    } else {
        select_67([e, a, b, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_703([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_704([a, b, d, e, f, h, g], is_dual)
    } else {
        select_705([i, f, j, a, b, c, g], !is_dual)
    }
}
/// n = 12, i = 5
fn select_697([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_698([a, c, d, f, e, h, g, i, l, j], is_dual)
    } else {
        select_703([h, j, k, g, l, e, f, a, b, i], !is_dual)
    }
}
/// n = 10, i = 4
fn select_710([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([f, c, d, e, h, i], is_dual)
    } else {
        select_30([i, j, a, g, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_709([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_150([a, b, e, d, g, f, h, i], is_dual)
    } else {
        select_710([a, d, c, b, f, g, e, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_708([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_301([c, d, a, e, f, b, g, h, i], is_dual)
    } else {
        select_709([d, c, b, e, a, f, g, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_711([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_422([a, f, b, c, g, e], is_dual)
    } else {
        select_489([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_707([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_708([a, b, e, d, c, g, f, i, h, j], is_dual)
    } else {
        select_711([a, e, b, f, g, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_713([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_131([a, c, e, d, f, g], is_dual)
    } else {
        select_68([a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_714([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_413([h, a, i, j, f, b, g, c], !is_dual)
    } else {
        select_342([b, a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_712([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_713([a, f, i, j, g, b, h, c], !is_dual)
    } else {
        select_714([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_706([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_707([e, k, j, l, i, g, f, a, b, h], !is_dual)
    } else {
        select_712([e, a, c, d, f, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_696([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_697([c, a, d, e, g, b, f, h, i, k, l, j], is_dual)
    } else {
        select_706([c, b, d, e, f, a, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_671([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_672([a, b, d, e, c, f, h, g, i, j, k, l], is_dual)
    } else {
        select_696([a, b, c, d, e, f, h, i, j, k, g, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_721([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_57([d, c, f, e, h], is_dual)
    } else {
        select_30([i, j, a, b, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_720([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_721([a, b, f, c, e, g, i, j, h, k], is_dual)
    } else {
        select_504([b, c, d, g, e, f, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_722([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_103([a, f, b, d, e, h, i, j], is_dual)
    } else {
        select_324([d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_719([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_720([a, b, c, d, f, g, e, h, j, i, k], is_dual)
    } else {
        select_722([e, d, c, a, g, f, i, h, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_725([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_45([d, f, g, e, a, b], !is_dual)
    } else {
        select_45([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_724([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_144([e, b, c, d, f, g, i, h], is_dual)
    } else {
        select_725([g, e, j, d, h, a, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_726([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([e, a, b, f, h, d, i, g], is_dual)
    } else {
        select_22([d, a, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_723([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_724([b, c, d, e, a, f, g, i, h, j], is_dual)
    } else {
        select_726([d, a, c, b, e, f, h, i, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_718([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_719([a, b, d, c, e, g, f, i, h, j, k], is_dual)
    } else {
        select_723([a, b, f, d, h, g, e, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_727([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_519([e, h, i, f, a, b, g, c], !is_dual)
    } else {
        select_135([f, h, i, a, e, g, b, c, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_717([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < e) || (is_dual && k > e) {
        select_718([a, b, c, d, e, f, g, h, i, j, l], is_dual)
    } else {
        select_727([h, i, k, l, a, b, j, f, g], !is_dual)
    }
}
/// n = 8, i = 2
fn select_730([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_702([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_92([a, e, c, d, b, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_729([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_730([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_483([g, i, f, j, a, b, h, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_733([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_66([b, f, d, e, a, g], is_dual)
    } else {
        select_225([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_734([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_237([a, g, b, d, f, e, h], is_dual)
    } else {
        select_162([d, b, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_732([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_733([a, b, g, e, f, h, d], is_dual)
    } else {
        select_734([b, a, c, e, f, d, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_731([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_732([b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_428([a, e, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_728([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_729([h, i, k, e, l, a, b, j, f, g], !is_dual)
    } else {
        select_731([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_716([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_717([a, b, e, c, f, g, d, i, h, j, k, l], is_dual)
    } else {
        select_728([a, b, d, e, f, c, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_737([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_215([f, h, i, d, e, g, a, b], !is_dual)
    } else {
        select_705([h, f, g, e, d, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_739([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_36([b, d, c, e, g, f, h], is_dual)
    } else {
        select_218([h, i, j, e, a, g, b, f, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_738([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_194([a, d, c, e, f, b, g, i, h, j], is_dual)
    } else {
        select_739([a, d, b, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_736([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_737([f, i, j, a, g, c, h, b, d], !is_dual)
    } else {
        select_738([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_741([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_367([a, b, c, d, f, g, e, h, j], is_dual)
    } else {
        select_367([b, c, d, a, g, e, h, f, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_743([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_98([d, c, b, f, e, g, h], is_dual)
    } else {
        select_195([d, f, h, e, g, a, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_742([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_743([b, c, a, e, d, g, f, h], is_dual)
    } else {
        select_396([d, i, g, h, e, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_740([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_741([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_742([h, j, e, a, f, i, b, g, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_735([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_736([e, h, i, k, f, a, b, j, c, g], !is_dual)
    } else {
        select_740([a, c, b, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_715([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_716([a, b, d, e, c, f, g, i, h, j, l, k], is_dual)
    } else {
        select_735([a, b, c, d, i, f, j, k, l, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_670([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_671([b, d, a, e, f, h, g, c, i, j, l, k], is_dual)
    } else {
        select_715([b, d, c, e, f, h, a, g, i, j, l, k], is_dual)
    }
}
/// n = 15, i = 6
fn select_563([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < g) || (is_dual && n > g) {
        select_564([a, b, d, e, c, f, h, g, i, j, k, l, m, n, o], is_dual)
    } else {
        select_670([a, b, c, e, d, j, k, h, m, n, g, o], is_dual)
    }
}
/// n = 10, i = 4
fn select_751([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([b, c, e, g, f, h], is_dual)
    } else {
        select_150([h, i, j, g, d, e, a, b], !is_dual)
    }
}
/// n = 12, i = 4
fn select_750([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_751([a, b, g, e, f, i, j, h, k, l], is_dual)
    } else {
        select_404([h, b, c, d, f, g, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_753([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_579([a, f, e, i, h, j, k], is_dual)
    } else {
        select_162([g, d, b, c, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_754([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_96([h, d, g, a, f], !is_dual)
    } else {
        select_481([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_752([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_753([a, d, f, c, e, g, h, j, i, k, l], is_dual)
    } else {
        select_754([a, b, i, e, h, k, f, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_749([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_750([c, a, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_752([c, b, e, d, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_757([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_36([b, c, a, d, f, e, g], is_dual)
    } else {
        select_381([b, c, e, a, f, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_756([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_757([a, d, e, b, c, f, g, h], is_dual)
    } else {
        select_330([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_758([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_337([b, f, d, e, a, g, h], is_dual)
    } else {
        select_140([b, d, e, a, c, f, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_755([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_756([a, b, c, d, i, k, h, l], is_dual)
    } else {
        select_758([d, c, f, e, h, g, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_748([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_749([c, d, b, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_755([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_762([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_96([a, e, f, g, h], is_dual)
    } else {
        select_237([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_763([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_178([b, c, d, g, f, h], is_dual)
    } else {
        select_101([h, k, j, e, i, a, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_761([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_762([e, j, k, f, i, a, b, g], !is_dual)
    } else {
        select_763([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_765([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_157([f, e, h, i, a, b, g], !is_dual)
    } else {
        select_36([b, d, a, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_766([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([b, a, c, d, e, g, f], is_dual)
    } else {
        select_153([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_764([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_765([e, h, i, f, a, b, g, c, d], !is_dual)
    } else {
        select_766([a, c, b, d, e, f, h, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_760([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_761([i, k, j, l, e, a, h, b, g, f, d], !is_dual)
    } else {
        select_764([b, a, c, h, e, j, f, l, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_769([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_91([b, c, d, f, g, e], is_dual)
    } else {
        select_43([g, i, j, a, b, h, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_768([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_561([e, j, k, f, a, i, g, b], !is_dual)
    } else {
        select_769([h, f, j, k, i, e, a, g, c, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_767([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_768([i, l, j, k, e, a, h, b, g, f, d], !is_dual)
    } else {
        select_764([b, a, c, h, e, j, f, k, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_759([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_760([c, b, d, e, a, f, g, h, i, j, k, l], is_dual)
    } else {
        select_767([c, a, d, e, b, f, g, h, i, j, l, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_747([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_748([b, a, d, c, e, f, g, h, j, i, k, m], is_dual)
    } else {
        select_759([a, b, c, d, f, i, h, j, g, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_773([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_683([a, b, c, d, f, e, g], is_dual)
    } else {
        select_134([a, d, g, e, f, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_776([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_45([b, d, c, e, f, h], is_dual)
    } else {
        select_31([a, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_775([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_331([g, a, b, e, d, i, j], is_dual)
    } else {
        select_776([a, d, c, e, f, h, g, j, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_778([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_45([f, c, g, e, d, a], !is_dual)
    } else {
        select_30([d, g, c, e, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_777([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([b, c, e, h, f, g], is_dual)
    } else {
        select_778([i, h, g, d, e, a, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_774([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_775([c, b, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_777([c, a, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_772([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_773([a, j, e, g, b, i, c], !is_dual)
    } else {
        select_774([a, c, b, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_780([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_739([a, b, c, d, f, g, j, i, k, l], is_dual)
    } else {
        select_404([b, c, d, e, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_782([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_98([a, b, c, d, e, f, g], is_dual)
    } else {
        select_98([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_783([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_157([a, b, d, e, f, i, h], is_dual)
    } else {
        select_162([a, c, d, e, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_781([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_782([a, c, i, g, f, k, l], is_dual)
    } else {
        select_783([b, c, e, d, f, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_779([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_780([b, d, a, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_781([b, d, c, e, f, g, a, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_771([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_772([a, c, d, h, g, i, b, j, k, l], is_dual)
    } else {
        select_779([a, c, d, b, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_787([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_96([i, j, a, b, g], !is_dual)
    } else {
        select_101([b, d, c, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_786([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_787([b, a, c, d, f, e, h, g, i, j], is_dual)
    } else {
        select_762([e, c, d, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_789([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_91([b, c, d, f, g, e], is_dual)
    } else {
        select_455([a, b, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_788([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_789([b, a, c, d, f, e, g, h, j], is_dual)
    } else {
        select_561([e, c, d, a, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_785([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_786([c, b, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_788([c, a, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_792([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_98([f, g, i, d, a, e, c], !is_dual)
    } else {
        select_207([f, h, i, d, a, g, b, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_791([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_792([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_792([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_793([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_112([a, b, c, e, f, h, i, g], is_dual)
    } else {
        select_231([a, c, d, f, g, e, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_790([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_791([a, b, c, g, h, f, e, i, j], is_dual)
    } else {
        select_793([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_784([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_785([a, c, b, f, e, g, i, h, k, j], is_dual)
    } else {
        select_790([j, l, i, k, a, b, c, f, d, h], !is_dual)
    }
}
/// n = 13, i = 5
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_771([b, d, a, c, e, f, g, h, j, i, k, m], is_dual)
    } else {
        select_784([a, c, d, b, f, i, h, j, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_746([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_747([a, b, d, c, i, e, g, h, k, l, m, j, n], is_dual)
    } else {
        select_770([a, b, d, c, h, f, g, i, j, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_799([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_237([b, d, c, e, f, g, h], is_dual)
    } else {
        select_191([a, b, d, e, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_798([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_799([a, b, d, i, e, g, h, k], is_dual)
    } else {
        select_462([e, c, d, h, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_800([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_179([a, c, b, d, f, e, g, h, i, j], is_dual)
    } else {
        select_314([a, c, e, d, f, i, h, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_797([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_798([b, a, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_800([f, d, c, a, h, g, e, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_803([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_310([a, b, g, c, f, h, e, i], is_dual)
    } else {
        select_92([b, c, d, e, h, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_802([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_236([a, b, g, h, f, i, e, j], is_dual)
    } else {
        select_803([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_805([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_43([a, b, f, d, h, e, g], is_dual)
    } else {
        select_43([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_804([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_122([a, b, d, e, h, g, i], is_dual)
    } else {
        select_805([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_801([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_802([a, c, f, d, e, g, h, j, i, k], is_dual)
    } else {
        select_804([a, b, c, i, e, g, f, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_796([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_797([a, c, e, b, f, g, h, d, i, j, k], is_dual)
    } else {
        select_801([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_809([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_66([f, h, a, b, g, c], !is_dual)
    } else {
        select_455([a, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_808([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_809([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_809([a, b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_811([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_281([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_70([a, f, c, d, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_810([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_811([i, j, k, l, f, a, g, b, h, c], !is_dual)
    } else {
        select_289([b, c, d, e, a, g, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_808([b, h, a, c, f, i, g, j], is_dual)
    } else {
        select_810([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_813([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_473([b, e, a, c, f, g, h, j], is_dual)
    } else {
        select_489([a, c, d, b, g, i, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_815([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_237([b, c, d, a, e, f, g], is_dual)
    } else {
        select_101([a, d, c, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_816([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_70([a, g, f, h, d, b], !is_dual)
    } else {
        select_70([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_815([b, c, e, a, g, j, k], is_dual)
    } else {
        select_816([a, b, d, h, f, i, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_812([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_813([c, b, d, e, a, f, h, i, g, k, j], is_dual)
    } else {
        select_814([a, c, d, b, e, g, f, i, k, j, h, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_807([a, c, e, f, i, g, h, k, j, l, m, n], is_dual)
    } else {
        select_812([a, c, b, d, j, g, h, l, i, m, k, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_795([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_796([a, d, c, f, b, g, i, h, k, l, j], is_dual)
    } else {
        select_806([a, c, b, e, d, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_821([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_335([e, c, d, a, f, g], is_dual)
    } else {
        select_657([b, a, c, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_820([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_134([c, a, e, f, b, g], is_dual)
    } else {
        select_821([a, c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_823([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_216([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_34([e, b, d, a, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_824([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_183([b, c, d, a, e, f, h], is_dual)
    } else {
        select_183([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_822([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_823([e, i, j, d, f, h, a, g], !is_dual)
    } else {
        select_824([d, f, b, c, e, g, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_819([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_820([e, i, d, k, f, a, b, h], !is_dual)
    } else {
        select_822([b, a, c, f, d, e, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_827([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_237([d, b, c, g, f, e, h], is_dual)
    } else {
        select_481([c, a, e, f, h, d, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_828([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_98([e, a, f, d, h, i, j], is_dual)
    } else {
        select_495([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_826([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_827([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_828([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_830([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_376([a, b, e, d, g, h, i], is_dual)
    } else {
        select_117([a, d, c, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_831([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_98([b, g, c, a, f, h, i], is_dual)
    } else {
        select_52([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_829([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_830([a, i, j, g, h, b, c, d, f], !is_dual)
    } else {
        select_831([a, b, d, c, e, g, f, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_825([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_826([a, c, e, b, f, g, i, j, h, k], is_dual)
    } else {
        select_829([a, f, b, d, c, h, i, g, l, k], is_dual)
    }
}
/// n = 13, i = 6
fn select_818([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_819([a, e, c, d, g, h, f, j, l, i, m], is_dual)
    } else {
        select_825([d, k, l, g, m, h, i, j, e, a, f, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_836([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_23([b, c, d, e, f, g, h], is_dual)
    } else {
        select_83([a, b, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_835([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_725([d, i, j, g, f, b, c], !is_dual)
    } else {
        select_836([g, k, i, j, h, d, e, a, f, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_837([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_281([b, a, c, d, e, f, g, i], is_dual)
    } else {
        select_69([a, f, c, d, h], is_dual)
    }
}
/// n = 14, i = 6
fn select_834([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_835([k, m, n, g, h, l, a, f, d, i, c], !is_dual)
    } else {
        select_837([b, c, e, h, f, g, i, j, k], is_dual)
    }
}
/// n = 14, i = 6
fn select_833([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_272([b, a, g, c, h, j, l, i], is_dual)
    } else {
        select_834([a, b, d, e, c, f, g, h, i, l, k, m, j, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_840([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_183([c, e, d, g, a, f, i], is_dual)
    } else {
        select_310([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_841([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_66([b, c, d, f, e, g], is_dual)
    } else {
        select_117([g, f, h, a, d, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_839([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_840([a, e, c, d, g, f, h, j, i], is_dual)
    } else {
        select_841([a, f, b, i, e, j, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_843([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_178([a, c, f, g, d, i], is_dual)
    } else {
        select_776([j, g, i, h, f, a, e, d, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_844([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_73([f, c, d, h, g], is_dual)
    } else {
        select_29([j, h, k, i, f, a, e, g, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_842([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_843([b, c, f, a, e, g, h, i, j, k], is_dual)
    } else {
        select_844([b, c, a, d, e, g, f, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_838([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_839([b, e, d, a, f, g, i, k, j, h], is_dual)
    } else {
        select_842([a, c, e, b, f, h, i, k, g, l, j], is_dual)
    }
}
/// n = 15, i = 6
fn select_832([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_833([a, c, d, e, g, h, i, f, j, m, k, l, n, o], is_dual)
    } else {
        select_838([c, b, d, f, e, h, j, g, i, k, l, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_817([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_818([m, l, o, i, f, n, a, k, g, j, b, c, d], !is_dual)
    } else {
        select_832([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_794([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_795([b, c, d, e, a, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_817([b, c, e, f, d, g, h, i, a, j, l, m, k, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_745([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_746([b, a, c, d, e, f, g, h, i, k, l, m, o, j], is_dual)
    } else {
        select_794([a, b, e, f, d, c, g, h, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_850([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_470([a, b, c, d, f, e, g], is_dual)
    } else {
        select_816([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_849([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_233([d, e, j, k, a, h, i, f, b], !is_dual)
    } else {
        select_850([d, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_848([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_785([a, b, f, c, e, h, g, i, j, k], is_dual)
    } else {
        select_849([g, j, l, a, b, h, k, f, i, d, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_855([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_23([b, c, d, e, f, g, h], is_dual)
    } else {
        select_53([a, c, d, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_854([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_281([b, d, e, g, f, h, i, j], is_dual)
    } else {
        select_855([a, b, c, i, f, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_856([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_207([g, a, c, d, e, h, j, k], is_dual)
    } else {
        select_364([a, e, d, b, f, h, g, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_853([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_854([b, c, d, a, e, f, i, h, g, j, k], is_dual)
    } else {
        select_856([b, c, g, d, a, f, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_852([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_172([b, a, c, e, f, g, j, i, h, k], is_dual)
    } else {
        select_853([a, b, e, c, d, g, f, i, h, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_858([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_247([a, d, b, f, g, e, i, j, h], is_dual)
    } else {
        select_247([a, d, c, e, g, f, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_860([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_183([b, c, d, e, g, f, h], is_dual)
    } else {
        select_98([a, b, c, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_859([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_212([f, b, d, a, e, g, h, i], is_dual)
    } else {
        select_860([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_857([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_858([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_859([a, e, f, d, h, i, g, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_851([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_852([a, c, b, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_857([a, c, e, d, f, h, g, b, i, j, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_847([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < l) || (is_dual && i > l) {
        select_848([a, b, g, c, h, f, j, k, l, m, i, n], is_dual)
    } else {
        select_851([a, b, f, d, e, i, g, h, k, j, m, l, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_864([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_470([h, f, j, e, i, a, b], !is_dual)
    } else {
        select_423([e, b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_866([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_73([a, c, d, f, g], is_dual)
    } else {
        select_586([k, j, f, i, h, e, a, b, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_865([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_866([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_237([e, j, k, h, i, a, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_863([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_864([a, c, b, d, e, f, g, i, h, j], is_dual)
    } else {
        select_865([a, b, c, d, e, f, h, j, g, k, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_869([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_376([a, b, c, d, e, f, g], is_dual)
    } else {
        select_117([a, g, d, e, f, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_868([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_830([a, b, d, g, e, h, i, f, j], is_dual)
    } else {
        select_869([a, g, j, f, i, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_870([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_173([c, b, d, f, e, g, h], is_dual)
    } else {
        select_190([a, b, d, g, e, f, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_867([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_868([a, b, d, c, e, f, g, i, h, j], is_dual)
    } else {
        select_870([i, j, f, k, a, g, e, b, h, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_862([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_863([j, k, i, l, e, a, h, f, c, b, g], !is_dual)
    } else {
        select_867([i, l, j, k, e, a, h, g, c, d, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_874([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_183([a, g, b, e, h, f, j], is_dual)
    } else {
        select_24([c, d, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_873([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_181([a, i, l, b, j], !is_dual)
    } else {
        select_874([c, d, a, b, e, f, g, h, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_877([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_57([c, d, f, e, h], is_dual)
    } else {
        select_28([a, b, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_876([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_877([b, c, e, d, f, g, i, h, j], is_dual)
    } else {
        select_117([a, d, e, h, f, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_878([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_66([b, a, c, e, d, f], is_dual)
    } else {
        select_509([c, a, e, d, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_875([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_876([b, a, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_878([k, g, a, j, i, b, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_872([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_873([a, c, d, b, f, i, h, j, l, g, m, k], is_dual)
    } else {
        select_875([a, b, c, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 13, i = 6
fn select_880([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_835([i, l, m, g, h, k, a, f, c, e, d], !is_dual)
    } else {
        select_532([b, h, d, f, e, i, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_882([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_546([a, c, f, e, d, h, g], is_dual)
    } else {
        select_692([a, c, b, d, e, f, h, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_881([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_830([d, h, k, e, j, a, g, f, c], !is_dual)
    } else {
        select_882([b, e, c, d, g, f, h, i], is_dual)
    }
}
/// n = 14, i = 6
fn select_879([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_880([a, e, b, c, i, f, g, h, k, l, j, m, n], is_dual)
    } else {
        select_881([a, f, d, g, h, j, e, l, k, i, n], is_dual)
    }
}
/// n = 14, i = 6
fn select_871([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_872([c, b, a, e, d, g, i, f, h, j, k, l, m], is_dual)
    } else {
        select_879([b, d, c, e, g, f, a, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 6
fn select_861([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_862([a, c, b, d, h, g, f, j, k, i, l, m], is_dual)
    } else {
        select_871([a, b, c, e, d, g, f, j, i, k, h, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_846([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_847([a, b, d, e, c, g, h, f, i, k, j, m, l, o], is_dual)
    } else {
        select_861([a, b, c, d, h, g, f, k, i, l, m, n, j, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_889([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_45([a, d, b, e, h, i], is_dual)
    } else {
        select_101([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_888([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_889([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_329([b, g, d, f, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_887([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_888([a, b, h, c, f, g, e, i, j], is_dual)
    } else {
        select_306([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_886([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_887([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_887([c, b, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_891([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_281([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_55([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_892([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_22([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_150([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_890([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_891([h, b, c, d, e, g, i, j], is_dual)
    } else {
        select_892([j, h, k, l, f, g, a, b], !is_dual)
    }
}
/// n = 14, i = 5
fn select_885([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_886([a, b, f, e, g, i, h, j, m, k], is_dual)
    } else {
        select_890([c, d, a, b, h, f, g, i, k, l, n, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_896([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_178([a, c, d, b, e, f], is_dual)
    } else {
        select_178([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_898([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_23([c, a, b, d, f, e, g], is_dual)
    } else {
        select_28([a, b, f, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_897([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_898([h, i, k, j, g, e, a, b], !is_dual)
    } else {
        select_70([a, f, c, d, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_895([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_896([a, g, d, h, f, j], is_dual)
    } else {
        select_897([a, b, c, f, e, g, i, k, l, h, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_901([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_28([g, h, d, a], !is_dual)
    } else {
        select_30([f, i, b, e, c], !is_dual)
    }
}
/// n = 12, i = 4
fn select_900([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_901([a, b, c, f, g, j, k, l, i], is_dual)
    } else {
        select_162([b, c, d, e, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_903([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_35([e, b, c, g, f, h], is_dual)
    } else {
        select_28([i, j, d, a], !is_dual)
    }
}
/// n = 12, i = 5
fn select_902([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_70([a, h, c, d, g, l], is_dual)
    } else {
        select_903([b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_899([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_900([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_902([a, b, d, e, f, i, h, g, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_894([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_895([a, c, d, g, f, b, h, i, j, k, l, m], is_dual)
    } else {
        select_899([a, c, d, b, e, f, h, g, i, j, l, m, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_906([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_416([k, j, h, i, f, a, e, b], !is_dual)
    } else {
        select_70([a, h, c, d, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_905([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_906([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_579([k, f, j, e, a, b, i], !is_dual)
    }
}
/// n = 11, i = 4
fn select_904([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_905([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_905([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_893([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_894([b, c, d, e, a, g, f, h, i, j, l, k, m], is_dual)
    } else {
        select_904([a, d, b, c, g, h, i, j, k, f, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_884([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_885([b, c, d, a, e, f, g, h, i, j, l, m, k, n], is_dual)
    } else {
        select_893([a, b, c, d, h, g, f, i, k, l, m, j, n], is_dual)
    }
}
/// n = 12, i = 5
fn select_910([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_40([i, l, f, j, e, h, a, c, g], !is_dual)
    } else {
        select_207([b, c, f, d, i, h, g, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_909([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_910([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_40([f, l, j, k, e, g, a, b, h], !is_dual)
    }
}
/// n = 11, i = 5
fn select_912([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_41([a, g, e, i, k], is_dual)
    } else {
        select_504([b, c, a, e, f, d, g, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_911([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_415([a, b, c, d, f, e, h, g, i, j, k], is_dual)
    } else {
        select_912([a, b, c, f, h, g, e, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_908([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_909([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    } else {
        select_911([a, c, b, d, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_915([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_22([g, i, f, h, a, d, b, e], !is_dual)
    } else {
        select_67([a, f, c, g, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_916([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_69([c, d, e, f, h], is_dual)
    } else {
        select_67([a, b, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_914([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_915([h, i, j, f, a, b, c, g, d], !is_dual)
    } else {
        select_916([b, c, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_917([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_159([e, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_217([a, b, h, e, f, i, j, k, g], is_dual)
    }
}
/// n = 13, i = 5
fn select_913([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_914([k, g, m, l, i, f, e, a, b, j], !is_dual)
    } else {
        select_917([b, e, c, d, f, g, i, h, l, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_907([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_908([a, e, c, d, g, f, i, j, h, m, k, l], is_dual)
    } else {
        select_913([a, b, c, d, f, e, g, h, i, k, l, m, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_883([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_884([b, a, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_907([b, d, c, h, f, g, i, k, j, l, a, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_845([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_846([a, b, d, e, c, i, g, h, j, l, k, f, n, m, o], is_dual)
    } else {
        select_883([a, d, c, e, b, g, i, f, j, l, h, m, k, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_744([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_745([a, c, d, b, e, f, g, i, j, h, k, l, n, m, o], is_dual)
    } else {
        select_845([a, c, e, f, d, g, h, i, j, b, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_562([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_563([a, b, d, c, e, f, h, i, j, k, m, n, o, g, l], is_dual)
    } else {
        select_744([a, b, d, e, f, c, h, i, j, g, m, l, n, k, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_13([b, c, d, a, f, e, h, i, g, j, k, n, m, o, l], is_dual)
    } else {
        select_562([b, a, c, d, f, e, h, i, j, k, l, n, m, o, g], is_dual)
    }
}
/// n = 15, i = 6
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, d, e, f, g, b, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_12([b, c, d, e, f, g, a, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_11([a, b, e, f, g, h, c, i, j, d, k, l, o, m, n], is_dual)
    } else {
        select_11([a, b, e, g, f, h, d, j, i, c, k, l, o, n, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_10([a, b, c, e, f, d, g, h, j, k, l, m, i, n, o], is_dual)
    } else {
        select_10([a, b, d, e, f, c, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_9([a, b, c, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_9([a, b, d, e, f, g, h, i, c, j, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_8([f, g, h, i, c, d, b, e, a, k, m, j, n, l, o], is_dual)
    } else {
        select_8([f, g, h, i, c, e, b, d, a, k, l, j, n, m, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_930([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_773([a, h, g, d, b, c, f], !is_dual)
    } else {
        select_395([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_933([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_331([f, g, h, e, a, b, d], !is_dual)
    } else {
        select_195([b, c, d, f, e, h, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_932([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_440([b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_933([e, i, j, k, f, a, b, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_935([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_331([a, b, g, f, e, h, j], is_dual)
    } else {
        select_171([b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_934([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_194([a, c, b, d, e, f, g, i, j, k], is_dual)
    } else {
        select_935([a, b, c, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_931([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_932([h, g, j, k, f, e, i, a, d, c, b], !is_dual)
    } else {
        select_934([h, e, j, k, f, g, i, d, b, a, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_929([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_930([e, f, g, j, k, a, b, h], !is_dual)
    } else {
        select_931([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_939([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_329([a, g, d, f, e, h], is_dual)
    } else {
        select_525([a, d, b, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_938([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_939([a, b, g, c, e, f, i, j], is_dual)
    } else {
        select_404([b, a, f, d, e, g, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_941([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_162([e, f, c, d, h, g], is_dual)
    } else {
        select_231([a, b, g, e, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_942([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_98([d, j, h, g, a, f, c], !is_dual)
    } else {
        select_503([h, i, j, d, g, e, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_940([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_941([b, c, a, d, e, f, g, h, i, k], is_dual)
    } else {
        select_942([b, g, c, e, a, f, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_937([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_938([b, c, d, a, f, h, e, i, g, j], is_dual)
    } else {
        select_940([a, b, d, c, f, h, g, i, e, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_945([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_152([b, e, d, c, f, g, h], is_dual)
    } else {
        select_243([a, b, d, g, f, e, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_944([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_888([a, b, g, d, f, h, e, j, k], is_dual)
    } else {
        select_945([b, c, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_947([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_331([d, g, h, e, a, f, b], !is_dual)
    } else {
        select_101([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_946([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_947([c, d, b, e, f, a, h, g], is_dual)
    } else {
        select_638([c, a, d, e, b, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_943([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_944([b, c, a, d, e, g, f, h, j, i, k], is_dual)
    } else {
        select_946([e, a, c, h, g, i, j, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_936([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_937([c, d, a, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_943([c, d, b, e, g, f, a, h, i, k, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_928([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_929([i, k, f, l, a, g, c, j, d, b, h], !is_dual)
    } else {
        select_936([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_952([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_50([a, g, c, d, e, f, h], is_dual)
    } else {
        select_50([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_951([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_952([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_952([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_955([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_46([b, c, d, f, g, h], is_dual)
    } else {
        select_27([a, d, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_954([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_237([e, c, d, a, f, g, i], is_dual)
    } else {
        select_955([d, a, b, c, f, e, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_953([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_840([b, a, d, e, c, f, g, h, i], is_dual)
    } else {
        select_954([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_950([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_951([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_953([a, b, c, i, d, h, g, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_957([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_559([a, b, d, c, e, g, f, h], is_dual)
    } else {
        select_815([a, b, d, f, c, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_956([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_951([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_957([a, b, c, i, d, h, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_949([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_950([a, b, c, e, d, i, g, h, k, j, l], is_dual)
    } else {
        select_956([c, a, d, f, b, h, g, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_961([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_324([a, c, b, f, e, g, d], is_dual)
    } else {
        select_324([b, c, a, f, d, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_962([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_144([a, c, b, e, f, g, d, h], is_dual)
    } else {
        select_324([b, c, a, e, d, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_960([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_961([b, d, a, e, f, c, g], is_dual)
    } else {
        select_962([b, d, c, e, a, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_964([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_191([i, j, k, a, b, h], !is_dual)
    } else {
        select_125([c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_965([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_125([b, c, d, e, g], is_dual)
    } else {
        select_30([a, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_963([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_964([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_965([b, c, d, e, f, g, a, h, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_959([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_960([g, i, k, j, a, b, c, h], !is_dual)
    } else {
        select_963([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_958([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_959([a, b, c, d, e, h, g, j, i, k, l], is_dual)
    } else {
        select_956([b, c, d, f, a, e, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_948([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_949([c, a, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_958([a, b, c, d, h, f, g, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_927([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_928([a, b, c, d, e, k, h, l, m, g, n, j], is_dual)
    } else {
        select_948([a, c, d, e, f, b, h, i, g, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_971([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_162([a, e, c, d, f, g], is_dual)
    } else {
        select_117([g, b, a, e, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_972([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_22([a, b, c, d, e, f, i, g], is_dual)
    } else {
        select_546([a, b, f, d, i, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_970([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_971([b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_972([a, c, h, f, g, e, k, j, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_973([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_246([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_324([b, e, d, g, f, a, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_969([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_970([b, a, c, g, e, f, h, i, k, l, m], is_dual)
    } else {
        select_973([a, e, f, d, h, g, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_976([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_183([b, c, a, e, f, g, i], is_dual)
    } else {
        select_410([a, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_975([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_945([b, a, e, d, h, f, g, j, i], is_dual)
    } else {
        select_976([b, d, g, c, f, e, h, i, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_979([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_101([b, d, c, f, e, g, h], is_dual)
    } else {
        select_32([a, b, h, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_978([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_979([b, e, g, c, f, h, l, k, i, j], is_dual)
    } else {
        select_207([a, b, i, d, j, h, g, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_977([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_978([b, c, e, d, a, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_592([b, h, e, g, a, f, j, l, i, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_974([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_975([a, c, b, e, i, f, k, h, m, j, n], is_dual)
    } else {
        select_977([a, f, b, c, d, g, j, h, k, l, m, i, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_968([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_969([b, f, d, e, a, g, h, j, k, l, i, n, m], is_dual)
    } else {
        select_974([b, a, c, e, d, g, h, f, i, j, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 5
fn select_982([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_199([b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_489([h, e, f, k, a, j, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_981([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_773([a, c, d, k, l, f, i], is_dual)
    } else {
        select_982([j, l, m, k, a, i, h, b, c, g, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_985([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_103([a, d, c, e, f, h, g, j], is_dual)
    } else {
        select_776([a, b, c, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_986([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_459([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_455([b, c, d, g, h, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_984([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_985([a, c, f, d, e, h, g, i, j, k], is_dual)
    } else {
        select_986([a, c, b, g, e, f, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_988([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_237([b, e, d, f, g, i, j], is_dual)
    } else {
        select_504([a, b, c, e, f, g, h, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_989([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_348([a, f, b, d, h, e, g, i, j], is_dual)
    } else {
        select_348([a, e, c, d, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_987([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_988([b, d, c, a, e, g, f, h, j, i, k], is_dual)
    } else {
        select_989([b, d, f, e, g, a, j, i, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_983([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_984([b, a, c, d, g, e, h, f, i, j, k], is_dual)
    } else {
        select_987([a, b, d, c, g, h, f, e, j, i, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_980([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_981([a, b, g, c, d, f, h, j, k, l, n, m, i], is_dual)
    } else {
        select_983([a, b, c, e, i, f, j, g, m, n, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_967([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_968([a, d, b, e, f, c, g, h, i, j, k, l, n, m], is_dual)
    } else {
        select_980([a, d, c, f, e, b, g, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_994([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_157([b, e, d, g, f, h, i], is_dual)
    } else {
        select_157([a, e, c, h, f, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_993([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_994([b, c, d, a, e, g, h, f, j, i], is_dual)
    } else {
        select_860([b, f, d, e, a, g, h, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_995([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_220([a, c, b, d, f, g, e, h], is_dual)
    } else {
        select_91([c, d, e, f, g, b], is_dual)
    }
}
/// n = 11, i = 4
fn select_992([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_993([a, b, d, c, e, g, h, f, j, i, k], is_dual)
    } else {
        select_995([d, a, f, e, g, c, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_998([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_157([b, c, f, h, g, j, k], is_dual)
    } else {
        select_73([a, d, e, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_997([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_337([c, h, e, f, a, i, k], is_dual)
    } else {
        select_998([c, b, d, e, f, a, g, i, h, j, l, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1000([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_237([b, a, c, d, e, f, g], is_dual)
    } else {
        select_98([a, b, e, d, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1001([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_100([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_237([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_999([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1000([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1001([h, e, g, j, i, a, b, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_996([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_997([c, b, d, e, a, f, g, i, h, j, k, l], is_dual)
    } else {
        select_999([b, c, h, e, a, g, j, i, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_991([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_992([a, b, d, c, f, i, j, h, m, k, n], is_dual)
    } else {
        select_996([a, b, c, d, e, h, g, k, i, l, m, j, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1005([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_98([g, f, i, a, b, h, c], !is_dual)
    } else {
        select_140([b, d, e, a, c, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1004([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1005([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_157([b, f, d, e, c, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1007([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_191([a, c, d, f, e, i], is_dual)
    } else {
        select_778([b, a, f, g, d, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1008([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_157([b, c, a, d, e, f, g], is_dual)
    } else {
        select_30([c, f, e, a, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1006([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1007([i, j, f, h, c, b, g, a, d], !is_dual)
    } else {
        select_1008([a, c, b, e, f, h, g, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1003([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1004([c, b, d, f, g, h, k, i, j], is_dual)
    } else {
        select_1006([c, b, a, i, e, j, h, g, l, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1010([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_194([a, c, d, b, e, g, f, h, j, i], is_dual)
    } else {
        select_805([a, d, f, e, g, b, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1012([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_76([a, b, e, d, g, f], is_dual)
    } else {
        select_725([g, e, h, a, f, c, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1013([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_335([f, d, e, a, g, i], is_dual)
    } else {
        select_140([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1011([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1012([a, c, f, e, b, g, h, j], is_dual)
    } else {
        select_1013([a, c, e, b, d, g, f, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1009([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1010([c, b, a, d, i, g, f, h, j, l], is_dual)
    } else {
        select_1011([a, f, c, b, e, h, i, g, l, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1002([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1003([c, d, a, e, f, b, i, h, g, j, k, l], is_dual)
    } else {
        select_1009([a, d, c, g, f, b, h, j, i, l, m, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_990([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_991([a, b, c, d, e, f, g, h, i, k, l, m, j, n], is_dual)
    } else {
        select_1002([a, b, d, c, e, h, g, j, l, k, m, i, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_966([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_967([a, b, c, e, d, f, h, i, k, g, l, j, m, n], is_dual)
    } else {
        select_990([a, c, d, e, b, f, g, i, j, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_926([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_927([a, d, e, b, f, g, h, c, i, j, k, l, m, n], is_dual)
    } else {
        select_966([a, d, e, c, f, g, h, b, i, j, k, l, n, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1021([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_35([d, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1020([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_216([f, g, i, h, d, a, b, e], !is_dual)
    } else {
        select_1021([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1022([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_144([c, b, d, a, e, f, g, h], is_dual)
    } else {
        select_324([c, a, d, b, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1019([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1020([f, g, i, e, h, a, b, d, c], !is_dual)
    } else {
        select_1022([a, g, f, i, h, e, b, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1023([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_555([b, c, a, d, f, g, e, h, i], is_dual)
    } else {
        select_215([h, f, i, g, d, a, b, e], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1018([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1019([h, k, i, j, e, a, c, g, d], !is_dual)
    } else {
        select_1023([a, c, b, e, f, i, j, h, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1026([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_889([b, a, d, c, e, f, g, h, i], is_dual)
    } else {
        select_34([e, h, j, a, g, f, d], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1025([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1026([a, b, f, d, h, g, e, j, k, i], is_dual)
    } else {
        select_486([a, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1028([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_503([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_503([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1029([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_229([a, b, d, f, g, e, i, j], is_dual)
    } else {
        select_117([a, e, c, h, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1027([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1028([a, d, c, e, g, f, j, i, h], is_dual)
    } else {
        select_1029([a, b, f, d, g, h, e, j, k, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1024([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1025([a, d, b, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_1027([a, d, c, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1017([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1018([c, b, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_1024([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1032([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_680([a, b, d, e, g, i, h, j], is_dual)
    } else {
        select_570([b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1031([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1032([a, b, c, d, f, g, h, e, i, j], is_dual)
    } else {
        select_1032([b, a, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1036([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_45([a, c, d, e, f, h], is_dual)
    } else {
        select_45([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1035([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_96([g, e, c, b, f], !is_dual)
    } else {
        select_1036([a, c, d, b, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_1037([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_66([g, d, a, e, f, b], !is_dual)
    } else {
        select_778([g, f, a, d, e, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1034([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1035([b, a, c, d, e, f, h, g], is_dual)
    } else {
        select_1037([a, b, c, f, g, h, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1033([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1034([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_1034([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1030([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1031([b, c, d, a, e, f, h, g, j, i], is_dual)
    } else {
        select_1033([e, f, g, k, l, a, h, i], !is_dual)
    }
}
/// n = 13, i = 6
fn select_1016([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1017([f, e, k, j, m, g, a, l, b, h, i], !is_dual)
    } else {
        select_1030([b, a, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1042([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_67([a, b, h, i, j], is_dual)
    } else {
        select_125([c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1041([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_965([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_1042([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_1044([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_73([a, c, d, e, f], is_dual)
    } else {
        select_362([a, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1043([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1044([a, c, d, e, f, g, i], is_dual)
    } else {
        select_823([a, b, c, g, f, e, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1041([a, c, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1043([a, i, k, l, b, g, j, c, h], !is_dual)
    }
}
/// n = 14, i = 6
fn select_1039([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1040([a, b, c, d, e, h, g, j, i, l, m, n], is_dual)
    } else {
        select_950([c, a, d, f, b, e, g, h, i, k, j], is_dual)
    }
}
/// n = 15, i = 6
fn select_1038([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1039([b, a, c, d, h, f, g, i, j, l, k, m, n, o], is_dual)
    } else {
        select_1039([a, b, c, d, i, e, g, h, k, l, j, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_1015([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1016([k, m, j, o, b, a, g, n, h, d, c, l, e], !is_dual)
    } else {
        select_1038([a, b, d, e, c, f, g, h, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_1051([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([b, c, d, f], is_dual)
    } else {
        select_30([a, g, e, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1050([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1051([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_455([i, j, k, a, f, b], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1049([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1050([a, b, c, d, e, g, f, h, i, j, l], is_dual)
    } else {
        select_1050([b, a, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1048([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1049([a, c, d, e, f, b, g, h, i, k, j, l], is_dual)
    } else {
        select_1049([b, c, d, e, f, a, g, h, j, k, i, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1047([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_956([b, c, d, f, a, e, g, h, i, j], is_dual)
    } else {
        select_1048([a, b, c, d, e, h, g, j, i, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1046([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_949([c, a, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1047([a, b, c, d, h, f, g, i, j, l, k, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1056([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_34([b, a, e, g, h, d, f], is_dual)
    } else {
        select_153([b, d, c, f, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1055([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1056([d, c, a, e, b, h, f, g], is_dual)
    } else {
        select_1056([d, c, b, e, a, h, g, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_1057([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_454([f, h, i, e, b, a, g, c], !is_dual)
    } else {
        select_330([c, b, d, a, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1054([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1055([e, g, h, i, a, b, c, f], !is_dual)
    } else {
        select_1057([b, c, a, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1060([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_451([e, d, c, a, f, h, g, i], is_dual)
    } else {
        select_168([a, c, b, d, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1061([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([a, b, d, f, g, e, h, i], is_dual)
    } else {
        select_481([b, c, e, f, g, d, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1059([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1060([a, e, d, c, g, f, i, h, j], is_dual)
    } else {
        select_1061([a, d, b, e, h, f, i, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1058([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1059([a, c, b, d, f, g, h, e, i, k, j], is_dual)
    } else {
        select_570([c, e, d, f, b, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1053([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_1054([a, f, b, g, h, j, i, k, e], is_dual)
    } else {
        select_1058([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1065([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_152([a, g, h, i, e, f, b], !is_dual)
    } else {
        select_171([b, a, c, d, e, f, h, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_1066([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_73([a, b, c, d, f], is_dual)
    } else {
        select_69([a, b, c, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1064([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1065([a, h, i, j, f, g, c, d, e], !is_dual)
    } else {
        select_1066([a, i, f, j, b, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1068([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_455([d, c, b, f, g, e], is_dual)
    } else {
        select_284([f, g, i, d, h, a, b, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1067([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_555([a, b, c, e, f, g, h, d, i], is_dual)
    } else {
        select_1068([a, c, b, e, d, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_1063([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1064([d, h, j, k, i, f, e, b, g, c], !is_dual)
    } else {
        select_1067([b, a, e, d, f, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1071([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_664([a, g, d, e, f, h, j, k], is_dual)
    } else {
        select_140([b, c, f, d, e, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1070([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1071([a, c, e, b, d, f, h, g, i, j, k], is_dual)
    } else {
        select_739([a, c, g, e, f, b, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1069([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_738([a, b, c, d, g, f, i, j, k, h], is_dual)
    } else {
        select_1070([a, d, c, e, b, f, g, h, j, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1062([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1063([f, k, l, a, h, j, i, d, c, b, g], !is_dual)
    } else {
        select_1069([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1052([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1053([a, d, c, e, f, g, h, b, j, i, k], is_dual)
    } else {
        select_1062([a, c, b, d, e, f, h, g, i, j, l, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_1045([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_1046([a, b, d, e, c, f, h, g, i, j, k, l, m, n], is_dual)
    } else {
        select_1052([b, a, d, c, e, k, h, l, m, g, n, j], is_dual)
    }
}
/// n = 15, i = 6
fn select_1014([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_1015([a, b, e, d, f, g, c, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1045([a, b, e, c, f, g, h, d, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_925([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_926([b, c, d, a, e, f, g, h, j, i, k, l, m, n], is_dual)
    } else {
        select_1014([a, b, c, d, e, f, g, i, j, h, k, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 2
fn select_1079([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_125([c, d, e, f, h], is_dual)
    } else {
        select_410([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1080([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_125([c, d, e, f, h], is_dual)
    } else {
        select_117([a, i, b, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1078([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1079([b, a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1080([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1081([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_122([a, b, e, d, g, f, h], is_dual)
    } else {
        select_43([a, c, f, d, h, e, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1077([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1078([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1081([a, c, i, g, b, h, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1083([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_572([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_572([c, b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1085([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_376([a, c, d, f, g, h, j], is_dual)
    } else {
        select_778([i, g, a, e, h, b, d], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1087([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_51([c, d, e, g], is_dual)
    } else {
        select_35([a, b, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1086([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1087([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_44([b, g, c, f, a, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1084([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1085([a, c, g, d, f, b, h, i, j, k], is_dual)
    } else {
        select_1086([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1082([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1083([a, c, d, b, g, h, i, j, f], is_dual)
    } else {
        select_1084([a, b, d, e, c, g, f, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1076([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1077([c, a, d, e, f, g, h, i, j, l, k], is_dual)
    } else {
        select_1082([a, b, c, d, j, h, i, g, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1090([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_559([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_1044([a, f, d, e, b, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1089([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_665([c, a, e, b, d, f, g, h, j, i], is_dual)
    } else {
        select_1090([a, b, c, d, g, f, i, h, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1092([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_173([b, a, c, e, f, i, h], is_dual)
    } else {
        select_404([b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1094([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_237([a, e, c, g, f, d, i], is_dual)
    } else {
        select_237([b, d, c, g, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1095([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_66([b, c, e, g, f, h], is_dual)
    } else {
        select_778([a, b, h, e, d, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1093([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1094([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_1095([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1091([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1092([b, c, d, a, e, g, f, i, h], is_dual)
    } else {
        select_1093([b, a, d, c, g, h, i, f, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1088([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1089([a, b, c, h, e, g, i, f, k, j, l], is_dual)
    } else {
        select_1091([c, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_1076([a, b, c, e, d, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1088([a, b, c, e, j, g, h, i, d, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1100([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_152([a, e, d, c, g, f, i], is_dual)
    } else {
        select_352([a, b, d, f, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1099([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1100([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_743([f, c, d, a, e, h, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1098([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_773([e, j, f, a, b, i, g], !is_dual)
    } else {
        select_1099([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_1104([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([c, d, g, f, a, e], !is_dual)
    } else {
        select_53([c, b, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1103([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_229([a, c, d, g, e, f, i, h], is_dual)
    } else {
        select_1104([g, j, a, b, h, f, d], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1105([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_103([a, c, d, e, g, h, i, k], is_dual)
    } else {
        select_231([b, c, d, f, h, g, j], is_dual)
    }
}
/// n = 13, i = 6
fn select_1102([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1103([g, h, k, m, i, l, d, f, a, c], !is_dual)
    } else {
        select_1105([j, k, h, m, g, i, l, d, e, a, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1108([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_83([a, b, d, e, h, i, j], is_dual)
    } else {
        select_85([b, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1107([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_103([a, c, d, h, f, g, j, k], is_dual)
    } else {
        select_1108([a, b, d, g, e, f, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1109([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_229([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_778([d, h, a, e, g, b, f], !is_dual)
    }
}
/// n = 13, i = 6
fn select_1106([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1107([h, l, i, m, j, k, e, a, b, g, d], !is_dual)
    } else {
        select_1109([a, c, d, h, f, g, i, k], is_dual)
    }
}
/// n = 13, i = 6
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1102([i, m, l, f, k, j, e, b, g, a, c, h, d], !is_dual)
    } else {
        select_1106([a, b, c, d, f, g, h, e, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 6
fn select_1097([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1098([b, c, a, h, e, f, i, j, k, g], is_dual)
    } else {
        select_1101([a, c, b, d, e, f, g, h, j, i, l, k, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_1113([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_66([a, b, d, f, g, e], is_dual)
    } else {
        select_237([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1112([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_470([d, b, c, e, g, f, i], is_dual)
    } else {
        select_1113([a, e, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([e, d, b, c, g, f, i], is_dual)
    } else {
        select_594([f, a, d, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_131([e, a, b, g, h, j], is_dual)
    } else {
        select_1115([c, a, b, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1111([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1112([c, b, d, e, f, a, g, h, i], is_dual)
    } else {
        select_1114([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1117([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_49([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1117([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_963([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1110([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1111([k, l, j, m, a, g, h, c, b, i], !is_dual)
    } else {
        select_1116([a, c, d, e, f, h, g, j, i, k, m], is_dual)
    }
}
/// n = 14, i = 6
fn select_1096([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_1097([j, k, l, n, a, g, b, m, h, c, d, i, e], !is_dual)
    } else {
        select_1110([a, d, c, b, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_1074([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1075([b, d, c, e, a, f, i, h, g, j, l, k, m], is_dual)
    } else {
        select_1096([b, a, c, d, e, j, h, i, k, l, g, m, n, o], is_dual)
    }
}
/// n = 7, i = 3
fn select_1123([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_98([e, d, g, a, b, f, c], !is_dual)
    } else {
        select_455([b, c, a, e, d, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_1122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_743([a, c, b, d, e, f, h, i], is_dual)
    } else {
        select_1123([e, a, b, g, d, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1121([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1122([j, f, k, a, g, b, c, i, h], !is_dual)
    } else {
        select_665([c, a, b, d, e, g, f, h, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_66([a, e, c, d, g, h], is_dual)
    } else {
        select_410([c, b, d, e, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1120([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1121([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1124([k, f, i, g, a, c, j, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1128([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_416([a, b, d, e, g, i, h, j], is_dual)
    } else {
        select_504([a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1129([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_503([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_324([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1127([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1128([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_1129([a, b, f, d, h, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_233([a, b, f, d, e, h, g, j, i], is_dual)
    } else {
        select_1127([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1125([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1126([a, d, c, e, f, g, h, b, i, j], is_dual)
    } else {
        select_147([a, c, d, b, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1119([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1120([a, b, c, d, e, f, g, i, h, k, j], is_dual)
    } else {
        select_1125([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1133([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_423([d, g, e, i, a, h, b, f], !is_dual)
    } else {
        select_1109([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1132([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1133([g, f, j, b, a, i, c, h, d], !is_dual)
    } else {
        select_957([c, b, d, e, a, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_1136([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_73([e, a, c, d, g], is_dual)
    } else {
        select_162([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1135([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_559([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_1136([b, f, d, e, a, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1137([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_131([d, b, e, c, f, g], is_dual)
    } else {
        select_68([f, a, b, d, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1134([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1135([b, c, a, d, e, f, g, i, h], is_dual)
    } else {
        select_1137([i, j, f, b, g, a, h, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1131([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1132([a, b, c, d, g, f, i, h, j, k], is_dual)
    } else {
        select_1134([a, b, c, e, h, f, g, j, i, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1140([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_423([d, h, g, i, a, e, b, f], !is_dual)
    } else {
        select_423([d, g, h, i, a, f, c, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1141([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_234([f, e, h, a, b, g, c], !is_dual)
    } else {
        select_489([b, c, d, a, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1140([e, h, j, a, f, i, b, c, g], !is_dual)
    } else {
        select_1141([a, c, d, f, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1139([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_1139([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1130([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1131([a, b, d, c, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1138([a, b, g, c, d, f, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1118([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_1119([a, b, d, c, f, g, h, j, i, l, k], is_dual)
    } else {
        select_1130([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 15, i = 6
fn select_1073([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1074([a, b, d, c, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    } else {
        select_1118([a, c, b, d, e, j, g, h, k, l, i, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1147([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_122([a, c, d, e, f, g, h], is_dual)
    } else {
        select_556([e, b, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1149([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_98([f, h, i, a, b, g, c], !is_dual)
    } else {
        select_410([c, a, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_1148([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1149([e, f, h, i, a, b, g, c, d], !is_dual)
    } else {
        select_123([b, c, f, g, a, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1146([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1147([a, f, h, i, e, b, g, d], !is_dual)
    } else {
        select_1148([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_1152([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_96([b, c, e, d, f], is_dual)
    } else {
        select_27([f, g, a, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1153([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_546([b, e, d, a, f, g, h], is_dual)
    } else {
        select_218([b, a, c, d, f, e, g, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1151([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_1152([e, b, a, g, i, h, d], is_dual)
    } else {
        select_1153([a, b, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1155([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_98([a, d, c, g, f, e, i], is_dual)
    } else {
        select_112([f, h, g, i, a, d, b, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1154([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1056([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_1155([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1150([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1151([g, f, i, e, h, a, b, d, c], !is_dual)
    } else {
        select_1154([g, e, i, a, f, h, b, d, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1145([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1146([f, g, i, j, b, c, h, d, e], !is_dual)
    } else {
        select_1150([f, g, i, j, a, b, c, h, e], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_216([a, b, h, c, f, g, i, j], is_dual)
    } else {
        select_52([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1158([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1159([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_167([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1157([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_675([a, b, c, g, h, i, f, j], is_dual)
    } else {
        select_1158([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_152([b, c, d, e, f, g, h], is_dual)
    } else {
        select_45([a, g, d, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1161([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1162([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_621([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1160([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1161([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_675([a, b, c, g, f, h, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1156([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_1157([a, b, c, e, d, f, h, g, i, j], is_dual)
    } else {
        select_1160([a, b, g, e, f, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1144([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1145([a, b, c, d, h, f, i, j, k, g], is_dual)
    } else {
        select_1156([b, a, d, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1143([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_948([a, b, d, e, c, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1144([b, a, d, c, e, k, g, l, m, h, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1168([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_195([e, a, b, g, d, i, h], is_dual)
    } else {
        select_195([d, a, c, f, e, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_1167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_215([f, h, j, d, g, i, a, e], !is_dual)
    } else {
        select_1168([d, b, c, e, g, f, h, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1169([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_72([a, c, f, d, e, h, g, i], is_dual)
    } else {
        select_67([a, b, i, j, f], is_dual)
    }
}
/// n = 12, i = 5
fn select_1166([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1167([j, k, l, a, h, b, i, c, f, d, g], !is_dual)
    } else {
        select_1169([b, d, c, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1172([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_50([b, c, a, d, e, f, g], is_dual)
    } else {
        select_335([g, b, c, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1172([a, c, d, b, e, g, f, h], is_dual)
    } else {
        select_189([a, f, c, d, b, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1170([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_300([b, a, d, c, g, f, i, j, k], is_dual)
    } else {
        select_1171([a, c, b, f, e, g, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1165([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1166([a, b, d, c, e, g, h, f, i, k, j, l], is_dual)
    } else {
        select_1170([a, c, d, b, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1175([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_680([i, j, e, f, h, a, b, g], !is_dual)
    } else {
        select_143([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1174([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1175([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_143([a, c, d, e, b, f, g, i, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_1178([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_509([f, a, b, d, e], is_dual)
    } else {
        select_509([e, a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_1179([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_195([a, b, c, d, f, g, e], is_dual)
    } else {
        select_195([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_1177([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1178([a, c, d, b, e, f], is_dual)
    } else {
        select_1179([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1180([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1044([a, c, d, e, b, f, h], is_dual)
    } else {
        select_1044([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1176([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1177([a, b, g, h, c, d, f], !is_dual)
    } else {
        select_1180([a, c, d, b, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_1173([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1174([e, d, j, i, k, f, h, b, g, c], !is_dual)
    } else {
        select_1176([d, f, e, i, k, g, a, h], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1164([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1165([b, a, c, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1173([f, k, l, b, a, g, i, j, d, c, h], !is_dual)
    }
}
/// n = 14, i = 5
fn select_1163([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_1046([a, b, d, e, c, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1164([b, a, d, c, e, k, g, l, m, h, j, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1142([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1143([c, b, a, d, e, f, i, g, j, h, k, l, m], is_dual)
    } else {
        select_1163([a, b, c, d, e, f, i, h, j, g, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_1072([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_1073([a, b, e, d, f, g, h, c, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1142([a, b, e, c, f, g, h, i, d, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_924([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_925([a, b, c, e, f, g, h, i, d, j, k, l, m, n, o], is_dual)
    } else {
        select_1072([a, b, c, e, d, g, h, i, f, j, k, l, m, n, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_1190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_28([e, f, b, d], is_dual)
    } else {
        select_85([a, d, c, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1189([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_40([a, b, d, e, c, g, f, h, i], is_dual)
    } else {
        select_1190([a, b, c, e, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1191([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_43([a, c, f, d, e, h, i], is_dual)
    } else {
        select_34([a, b, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1188([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1189([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_1191([d, h, i, a, e, g, b, f, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1193([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_98([f, i, g, a, d, e, c], !is_dual)
    } else {
        select_153([h, i, d, a, g, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1192([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1193([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_229([f, i, a, d, g, h, b, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1187([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1188([a, e, i, b, f, h, g, d, c], !is_dual)
    } else {
        select_1192([a, f, i, e, h, b, g, c, d], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_47([a, c, d, b, e, f, g, i, h, j], is_dual)
    } else {
        select_1187([a, c, h, d, i, b, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1185([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1186([a, b, c, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_1186([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1197([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_132([a, c, b, g, h, i, f, j], is_dual)
    } else {
        select_783([b, c, a, d, e, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_171([f, g, h, i, a, e, d, c], !is_dual)
    } else {
        select_589([c, a, b, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1200([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_34([e, h, i, a, g, f, b], !is_dual)
    } else {
        select_525([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1199([f, k, j, i, h, a, b, c, d], !is_dual)
    } else {
        select_1200([a, e, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1197([a, c, f, d, e, b, g, h, i, j], is_dual)
    } else {
        select_1198([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1202([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_131([a, b, c, d, i, h], is_dual)
    } else {
        select_559([a, b, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1203([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_131([a, b, c, e, g, h], is_dual)
    } else {
        select_1065([a, g, i, j, f, h, c, d, e], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1201([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1202([a, c, d, f, b, e, g, h, i], is_dual)
    } else {
        select_1203([a, c, d, g, f, b, i, h, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_1195([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1196([a, b, c, f, j, g, h, i, l, m, n], is_dual)
    } else {
        select_1201([b, a, d, e, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1207([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_183([a, b, c, d, e, f, g], is_dual)
    } else {
        select_26([h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1208([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_70([a, g, i, j, h, f], !is_dual)
    } else {
        select_92([a, d, b, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_1206([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1207([a, f, d, e, b, g, i, h], is_dual)
    } else {
        select_1208([a, b, c, d, e, g, f, h, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1210([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_183([a, b, c, d, e, f, h], is_dual)
    } else {
        select_117([e, h, i, a, g, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1209([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1210([f, b, d, a, e, g, h, i, j], is_dual)
    } else {
        select_860([a, b, c, d, e, g, f, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1205([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1206([j, i, l, m, a, f, k, h, b, d, e], !is_dual)
    } else {
        select_1209([i, j, l, m, a, f, k, g, b, c], !is_dual)
    }
}
/// n = 7, i = 1
fn select_1212([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_294([a, b, c, e, f, d, g], is_dual)
    } else {
        select_294([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1214([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_52([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_52([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_1216([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_51([d, e, f, g], is_dual)
    } else {
        select_51([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_1215([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_50([a, h, d, e, f, g, i], is_dual)
    } else {
        select_1216([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1213([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1214([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_1215([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1212([b, c, d, g, a, h, j], is_dual)
    } else {
        select_1213([a, e, f, b, c, d, g, h, i, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1204([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1205([b, e, f, c, d, g, h, i, a, j, k, l, m], is_dual)
    } else {
        select_1211([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 15, i = 6
fn select_1194([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_1195([a, b, d, e, f, c, g, i, h, j, k, l, m, n], is_dual)
    } else {
        select_1204([b, a, e, f, h, g, c, i, j, k, m, l, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1185([b, c, a, e, f, h, i, k, l, j, o], is_dual)
    } else {
        select_1194([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_1223([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([g, h, a, f, b], !is_dual)
    } else {
        select_53([e, c, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1222([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_98([h, i, j, a, f, e, b], !is_dual)
    } else {
        select_1223([a, e, c, d, g, f, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1224([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_125([c, d, e, f, g], is_dual)
    } else {
        select_53([a, b, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1221([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1222([g, m, k, l, j, h, a, b, i, c], !is_dual)
    } else {
        select_1224([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1226([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_50([b, c, d, e, f, g, h], is_dual)
    } else {
        select_52([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1227([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_362([a, b, g, h, i], is_dual)
    } else {
        select_51([c, d, e, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_1225([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1226([b, c, g, d, e, f, h, i, j], is_dual)
    } else {
        select_1227([a, h, d, e, f, g, i, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1220([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1221([b, c, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    } else {
        select_1225([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_1230([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_98([a, b, h, f, i, j, k], is_dual)
    } else {
        select_56([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1232([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_45([e, i, j, h, a, g], !is_dual)
    } else {
        select_51([b, c, d, f], is_dual)
    }
}
/// n = 12, i = 5
fn select_1231([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_324([i, l, k, j, f, a, b], !is_dual)
    } else {
        select_1232([b, c, d, e, f, g, h, j, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1229([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1230([a, b, c, d, e, f, g, h, i, j, l], is_dual)
    } else {
        select_1231([b, a, c, d, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_1234([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_125([b, c, d, e, f], is_dual)
    } else {
        select_31([a, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1233([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_680([i, k, j, g, f, a, b, h], !is_dual)
    } else {
        select_1234([b, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1228([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1229([c, b, d, e, f, a, g, h, i, j, k, l], is_dual)
    } else {
        select_1233([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1219([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1220([a, c, b, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_1228([a, c, h, e, f, g, b, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1237([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_54([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_54([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1239([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_293([a, g, c, d, e, f, h, i, j], is_dual)
    } else {
        select_52([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1238([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_49([a, c, d, e, f, b, g, h, j], is_dual)
    } else {
        select_1239([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1236([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_1237([a, c, b, e, f, g, d, h, i], is_dual)
    } else {
        select_1238([a, c, d, e, f, g, b, h, j, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_1242([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_546([a, b, c, e, d, g, f], is_dual)
    } else {
        select_546([a, b, d, e, c, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1241([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1242([h, e, b, f, g, c, a], !is_dual)
    } else {
        select_430([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1240([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1241([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_711([c, a, b, e, d, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1235([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1236([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1240([a, b, c, d, h, e, i, j], is_dual)
    }
}
/// n = 14, i = 6
fn select_1218([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1219([a, b, c, g, d, e, f, h, j, l, m, i, n], is_dual)
    } else {
        select_1235([c, a, d, e, f, b, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1248([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_28([a, b, h, k], is_dual)
    } else {
        select_101([c, d, g, e, f, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1247([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_152([c, i, e, d, f, h, k], is_dual)
    } else {
        select_1248([a, b, c, e, h, f, g, j, i, k, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_1249([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1248([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_329([c, h, d, f, e, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1246([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1247([b, c, a, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_1249([b, c, f, e, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 4, i = 1
fn select_1252([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_28([a, b, d, c], is_dual)
    }
}
/// n = 10, i = 3
fn select_1253([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_35([e, c, d, h, g, j], is_dual)
    } else {
        select_35([e, a, b, i, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1251([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1252([b, c, i, j], is_dual)
    } else {
        select_1253([b, c, d, e, a, f, g, h, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1250([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1251([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_35([h, d, e, a, g, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1245([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1246([b, c, d, e, a, g, f, h, i, j, l, k], is_dual)
    } else {
        select_1250([a, c, d, b, g, h, j, i, f, k, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_1256([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_183([a, b, c, d, e, f, g], is_dual)
    } else {
        select_31([e, g, a], is_dual)
    }
}
/// n = 8, i = 2
fn select_1255([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_220([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_1256([f, b, d, a, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1254([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_233([b, a, c, d, f, e, g, h, j], is_dual)
    } else {
        select_1255([a, c, d, b, e, g, f, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1244([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1245([b, a, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_1254([a, b, h, e, i, f, g, l, m, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1260([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_237([e, c, d, a, f, g, h], is_dual)
    } else {
        select_168([b, d, a, c, f, e, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1259([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_413([d, b, c, i, f, l, h, k], is_dual)
    } else {
        select_1260([a, d, h, e, f, g, j, k, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1262([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_109([a, b, c, d, e, f, g], is_dual)
    } else {
        select_76([a, e, f, h, g, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1261([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1262([a, d, b, f, i, h, e, g], is_dual)
    } else {
        select_1262([a, d, c, e, i, g, f, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1258([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1259([a, c, d, b, e, f, g, h, i, j, l, m], is_dual)
    } else {
        select_1261([a, b, e, h, f, g, j, l, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1265([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_144([a, c, b, d, e, f, h, g], is_dual)
    } else {
        select_455([d, b, a, f, g, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1264([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1028([a, c, b, d, e, f, h, i, j], is_dual)
    } else {
        select_1265([a, b, c, e, f, i, g, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1267([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_98([g, h, i, a, d, e, c], !is_dual)
    } else {
        select_195([a, e, b, f, d, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1266([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1267([h, j, i, g, e, a, b, f, c], !is_dual)
    } else {
        select_766([b, a, d, e, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_1263([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1264([i, e, j, k, g, h, f, a, c, b], !is_dual)
    } else {
        select_1266([b, c, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 14, i = 6
fn select_1257([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1258([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1263([i, f, k, n, a, l, g, j, b, e, h], !is_dual)
    }
}
/// n = 14, i = 6
fn select_1243([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_1244([b, a, d, e, f, c, g, h, j, i, k, l, m], is_dual)
    } else {
        select_1257([b, c, d, e, f, a, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 6
fn select_1217([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1218([a, c, d, b, e, f, g, h, j, i, k, l, m, n], is_dual)
    } else {
        select_1243([a, c, d, e, f, g, h, i, j, b, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_1183([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1184([a, b, d, c, e, f, g, h, i, k, j, l, n, o, m], is_dual)
    } else {
        select_1217([a, b, c, d, e, f, i, j, k, h, l, m, n, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_1273([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_805([a, c, d, g, e, f, h, i], is_dual)
    } else {
        select_1037([h, g, i, a, f, b, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1276([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_35([a, c, d, e, f, h], is_dual)
    } else {
        select_35([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1275([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_43([a, c, e, f, h, d, i], is_dual)
    } else {
        select_1276([f, g, h, i, a, e, b, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1277([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_98([a, b, e, f, h, d, i], is_dual)
    } else {
        select_98([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1274([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1275([e, f, i, h, g, a, c, b, d], !is_dual)
    } else {
        select_1277([e, g, i, f, h, b, a, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1272([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1273([b, c, d, a, f, e, g, i, h], is_dual)
    } else {
        select_1274([i, g, e, j, a, f, b, h, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1279([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1262([a, c, b, d, f, g, e, h], is_dual)
    } else {
        select_869([a, c, e, g, b, h, f], is_dual)
    }
}
/// n = 12, i = 5
fn select_1278([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_433([a, b, c, d, e, f, g, h, i, k, l], is_dual)
    } else {
        select_1279([a, b, h, c, f, i, j, l], is_dual)
    }
}
/// n = 13, i = 6
fn select_1271([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_1272([e, a, d, c, h, g, i, f, j, k], is_dual)
    } else {
        select_1278([h, j, i, l, m, d, e, k, g, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1283([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_96([a, g, e, h, i], is_dual)
    } else {
        select_73([b, c, d, f, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_1284([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_96([e, f, a, b, c], !is_dual)
    } else {
        select_1252([b, c, d, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_1282([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1283([a, b, c, d, e, f, h, i, j, k], is_dual)
    } else {
        select_1284([b, c, d, f, g, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1252([a, b, h, i], is_dual)
    } else {
        select_368([c, d, a, b, e, f, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1288([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_57([e, c, d, g, f], is_dual)
    } else {
        select_28([a, b, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1287([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_161([a, d, e, h, g, k, l], is_dual)
    } else {
        select_1288([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1285([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1286([c, d, b, e, f, h, j, g, i], is_dual)
    } else {
        select_1287([a, c, d, b, e, h, g, j, f, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1281([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1282([i, j, k, l, g, a, b, h, d, e, f], !is_dual)
    } else {
        select_1285([b, c, d, e, a, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_1291([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_195([d, g, h, e, f, a, c], !is_dual)
    } else {
        select_195([e, g, h, d, f, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1290([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1291([a, f, c, d, g, e, i, j], is_dual)
    } else {
        select_632([a, b, c, d, f, e, g, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1292([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_41([a, h, b, e, i], is_dual)
    } else {
        select_161([a, c, d, g, f, h, i], is_dual)
    }
}
/// n = 14, i = 6
fn select_1289([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1290([h, n, m, a, k, l, b, c, i, d], !is_dual)
    } else {
        select_1292([b, a, e, f, h, g, j, k, i], is_dual)
    }
}
/// n = 14, i = 6
fn select_1280([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1281([a, c, b, d, e, g, i, h, k, l, j, m], is_dual)
    } else {
        select_1289([a, b, c, f, d, e, h, i, j, k, l, g, n, m], is_dual)
    }
}
/// n = 14, i = 6
fn select_1270([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1271([l, k, n, a, i, j, m, b, g, c, h, d, e], !is_dual)
    } else {
        select_1280([b, a, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_1297([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_331([f, d, g, e, a, b, c], !is_dual)
    } else {
        select_331([f, e, g, d, a, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_1298([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_66([a, b, c, d, e, f], is_dual)
    } else {
        select_41([c, d, b, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1296([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1297([e, g, h, a, b, f, c], !is_dual)
    } else {
        select_1298([h, g, a, b, f, e, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1299([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_561([e, h, i, f, a, g, b, c], !is_dual)
    } else {
        select_561([f, h, i, e, a, g, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1295([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1296([e, h, i, j, a, f, b, c], !is_dual)
    } else {
        select_1299([e, h, i, j, a, f, g, c, d], !is_dual)
    }
}
/// n = 7, i = 1
fn select_1302([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_125([b, c, d, e, f], is_dual)
    } else {
        select_25([a, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1301([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_1302([b, c, d, e, f, h, i], is_dual)
    } else {
        select_1239([a, b, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1303([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_49([a, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1300([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1301([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_1303([a, c, d, e, f, g, b, h, i, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1294([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1295([a, j, l, m, b, h, k, c, i, d], !is_dual)
    } else {
        select_1300([b, c, d, a, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 14, i = 6
fn select_1293([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1294([a, b, c, g, d, e, f, h, j, l, m, i, n], is_dual)
    } else {
        select_1235([h, a, d, e, f, b, c, i, k, j], is_dual)
    }
}
/// n = 14, i = 6
fn select_1269([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_1270([a, c, d, e, f, g, h, i, j, b, k, l, m, n], is_dual)
    } else {
        select_1293([a, c, d, b, e, f, g, h, j, i, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_1308([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_422([a, b, c, d, f, g], is_dual)
    } else {
        select_70([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_1307([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_627([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_1308([a, g, b, c, d, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_1309([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_773([a, c, f, e, g, b, h], is_dual)
    } else {
        select_275([a, c, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1306([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1307([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_1309([g, b, c, d, f, h, a, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1313([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_74([a, e, g, h, f, b], !is_dual)
    } else {
        select_110([b, a, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_1312([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_559([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_1313([a, f, d, e, b, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1311([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_627([a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_1312([f, a, b, c, d, g, i, j, h, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1315([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_131([a, b, f, d, i, g], is_dual)
    } else {
        select_816([a, b, c, g, e, h, i, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_1316([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_830([a, i, j, e, h, f, g, b, d], !is_dual)
    } else {
        select_830([a, i, j, f, h, e, g, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1314([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1315([a, i, k, e, j, g, h, f, b], !is_dual)
    } else {
        select_1316([a, b, c, d, e, f, g, j, h, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1310([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1311([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1314([f, j, l, k, a, h, b, g, c, i, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1305([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1306([b, c, d, a, g, f, h, j, i], is_dual)
    } else {
        select_1310([b, a, e, c, d, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_1321([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_69([a, b, c, d, e], is_dual)
    } else {
        select_28([a, b, e, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_1320([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_131([a, c, d, e, b, f], is_dual)
    } else {
        select_1321([a, c, d, b, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_1319([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1320([a, b, c, d, e, f], is_dual)
    } else {
        select_70([a, b, d, e, f, c], is_dual)
    }
}
/// n = 11, i = 4
fn select_1318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1170([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1319([a, c, h, e, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1325([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_28([a, b, h, j], is_dual)
    } else {
        select_85([c, e, d, g, f, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1324([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_1325([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_436([a, b, c, g, f, i, h, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1327([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_45([a, g, b, d, i, f], is_dual)
    } else {
        select_45([a, f, c, e, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1327([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_536([a, b, e, d, g, f, h, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1323([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1324([a, b, c, d, f, e, g, h, i, j, k], is_dual)
    } else {
        select_1326([c, b, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1328([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_181([a, f, d, b, g], !is_dual)
    } else {
        select_132([a, c, b, e, d, f, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1322([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1323([b, c, a, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_1328([b, c, g, e, i, a, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1317([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1318([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1322([b, c, e, d, g, f, h, j, a, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1304([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1305([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    } else {
        select_1317([b, c, d, f, a, g, e, h, j, i, k], is_dual)
    }
}
/// n = 14, i = 6
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_1269([a, b, c, d, e, f, g, h, j, i, k, l, m, n], is_dual)
    } else {
        select_1304([b, a, h, e, f, c, i, d, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_1182([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < h) || (is_dual && l > h) {
        select_1183([b, c, a, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1268([a, c, b, d, e, f, i, j, l, k, h, m, n, o], is_dual)
    }
}
/// n = 10, i = 4
fn select_1335([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_422([b, c, a, g, h, j], is_dual)
    } else {
        select_1172([a, d, e, b, c, g, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1336([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1085([a, b, d, e, c, f, g, h, j, i], is_dual)
    } else {
        select_1085([a, c, d, e, b, f, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1334([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_1335([a, b, c, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1336([a, b, c, e, d, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1339([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_100([c, b, a, f, d, g, e, h], is_dual)
    } else {
        select_34([d, g, i, a, e, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1339([b, f, c, g, a, e, i, j, h], is_dual)
    } else {
        select_486([b, d, c, a, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1341([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_237([c, e, b, f, d, g, i], is_dual)
    } else {
        select_503([d, a, c, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1340([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1341([c, b, d, a, e, f, h, i, g, j], is_dual)
    } else {
        select_726([c, a, d, b, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1337([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1338([a, b, e, c, f, d, g, h, i, j], is_dual)
    } else {
        select_1340([a, b, e, d, f, c, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1333([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1334([a, c, d, f, b, e, g, i, h, k, j], is_dual)
    } else {
        select_1337([a, c, d, b, e, h, i, j, g, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1344([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_122([b, c, g, e, h, f, i], is_dual)
    } else {
        select_1199([e, j, h, i, g, b, a, d, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1347([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_28([a, g, i, h], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1346([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_416([f, h, i, d, a, b, g, e], !is_dual)
    } else {
        select_1347([f, h, i, d, b, g, a, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1348([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_416([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_347([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1345([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1346([d, g, i, e, h, a, f, c, b], !is_dual)
    } else {
        select_1348([g, d, i, e, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1343([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1344([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_1345([f, b, c, e, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1342([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_1343([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_1343([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1332([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1333([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1342([a, g, c, d, f, b, h, j, i, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1353([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_98([a, g, b, h, f, e, i], is_dual)
    } else {
        select_145([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1352([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_1037([h, f, e, a, i, b, g], !is_dual)
    } else {
        select_1353([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1355([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_162([b, a, e, f, i, k], is_dual)
    } else {
        select_368([a, b, c, d, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1354([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1355([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_325([h, b, e, f, a, k, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1351([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1352([b, g, e, f, a, j, h, i, k], is_dual)
    } else {
        select_1354([b, a, e, f, c, d, h, i, g, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_1358([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_73([a, d, f, g, b], !is_dual)
    } else {
        select_70([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1357([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1358([a, g, b, h, e, f, i], is_dual)
    } else {
        select_73([a, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_1359([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_131([a, b, c, d, e, f], is_dual)
    } else {
        select_70([a, e, b, g, f, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1356([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1357([a, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_1359([a, i, j, h, b, f, c], !is_dual)
    }
}
/// n = 11, i = 3
fn select_1350([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_1351([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1356([a, b, g, e, f, c, h, j, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1362([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_413([g, b, c, f, a, i, h, j], is_dual)
    } else {
        select_338([a, b, c, f, d, e, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1364([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_152([a, b, c, d, e, g, f], is_dual)
    } else {
        select_34([g, e, h, a, b, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1363([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1012([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1364([a, c, d, e, f, h, g, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1361([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1362([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_1363([c, b, d, g, f, a, h, i, l, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1367([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_140([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_140([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1366([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1367([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_141([c, g, d, e, f, a, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_1368([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_189([a, c, d, e, b, f, g], is_dual)
    } else {
        select_575([a, d, e, b, c, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_1365([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1366([a, c, b, d, e, f, g, h, i, k, l], is_dual)
    } else {
        select_1368([a, b, d, g, i, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_1360([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1361([a, b, c, d, i, e, j, k, h, m, l, n], is_dual)
    } else {
        select_1365([a, c, b, d, f, g, h, k, i, j, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1349([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_1350([a, c, e, h, f, g, d, i, j, k, l], is_dual)
    } else {
        select_1360([a, b, c, e, d, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1331([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1332([a, b, c, d, i, e, j, l, k, h, m, n], is_dual)
    } else {
        select_1349([b, a, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 10, i = 3
fn select_1374([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_362([a, b, h, i, j], is_dual)
    } else {
        select_125([c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_1375([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_50([a, b, c, d, e, g, h], is_dual)
    } else {
        select_50([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1373([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1374([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_1375([b, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1377([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_125([d, e, f, g, i], is_dual)
    } else {
        select_162([a, b, c, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1376([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1080([a, b, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_1377([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1372([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_1373([a, i, d, e, f, g, h, c, j, k, l], is_dual)
    } else {
        select_1376([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1371([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1372([a, b, d, e, f, g, h, i, c, j, k, l], is_dual)
    } else {
        select_1372([a, c, d, e, f, g, h, i, b, j, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1379([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_1188([f, e, j, a, h, i, g, c, b], !is_dual)
    } else {
        select_870([a, b, c, d, f, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1378([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1379([a, d, b, e, f, g, c, h, i, j], is_dual)
    } else {
        select_1379([a, d, c, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1370([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1371([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1378([b, c, d, e, j, i, a, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1369([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1370([h, a, c, d, b, e, f, g, i, k, m, j, l], is_dual)
    } else {
        select_1370([h, a, f, g, e, b, c, d, i, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1330([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_1331([a, c, f, g, h, d, e, i, j, b, k, l, n, m], is_dual)
    } else {
        select_1369([a, b, d, e, h, f, g, c, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_1385([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_606([a, b, e, h, i, g, c, f], !is_dual)
    } else {
        select_474([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_1386([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_90([a, d, e, b, c, f, g], is_dual)
    } else {
        select_226([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1384([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1385([a, b, f, j, c, i, h, d, g], !is_dual)
    } else {
        select_1386([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1383([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1384([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_1384([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1382([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1383([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_1383([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1391([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_70([a, f, h, i, g, b], !is_dual)
    } else {
        select_162([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_1390([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1391([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_1391([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1389([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_627([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_1390([a, g, i, f, j, b, h, c, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1393([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_246([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_371([c, a, d, b, f, g, h, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1392([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1393([b, d, c, e, a, f, g, h, j], is_dual)
    } else {
        select_1393([a, d, c, e, b, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1388([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1389([e, a, b, c, f, g, h, i, j, k], is_dual)
    } else {
        select_1392([e, j, k, h, l, g, f, a, d, i], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1387([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1388([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_1388([b, c, d, e, f, g, a, h, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1381([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1382([b, f, g, c, d, a, i, h, j, k], is_dual)
    } else {
        select_1387([a, b, c, d, e, f, g, i, j, k, h, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1400([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_45([e, i, j, h, a, b], !is_dual)
    } else {
        select_57([b, c, d, g, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_1399([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_416([a, b, d, e, f, h, g, j], is_dual)
    } else {
        select_1400([a, c, b, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1401([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1400([a, h, b, c, f, g, j, i, k, l], is_dual)
    } else {
        select_89([a, d, e, i, f, h, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1398([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1399([b, a, h, c, f, j, g, k, l, i], is_dual)
    } else {
        select_1401([b, a, c, d, e, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1403([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_408([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_329([b, h, d, f, e, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1402([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1403([c, a, d, e, b, f, g, h, i, j, k], is_dual)
    } else {
        select_1403([c, b, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1397([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1398([d, c, e, a, b, f, i, h, j, k, g, m], is_dual)
    } else {
        select_1402([a, b, d, e, f, g, i, j, h, l, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1406([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_103([b, d, c, e, f, g, h, j], is_dual)
    } else {
        select_117([a, c, d, f, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1405([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1406([a, c, d, b, e, f, g, i, h, j], is_dual)
    } else {
        select_366([a, d, g, f, b, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1409([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_37([c, f, d, e, h, g], is_dual)
    } else {
        select_28([a, b, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1408([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1409([a, b, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_877([a, b, f, d, c, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1407([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1408([b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1249([b, c, a, d, f, i, h, j, g, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1404([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1405([a, b, c, i, f, j, h, l, g, k], is_dual)
    } else {
        select_1407([c, a, b, d, e, f, h, g, i, k, l, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1396([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_1397([b, c, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1404([b, c, a, e, f, d, h, g, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_1413([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_40([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_40([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1412([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_181([e, f, c, a, g], !is_dual)
    } else {
        select_1413([b, a, d, c, e, f, h, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1411([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1405([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1412([g, j, h, i, a, b, f, e, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1415([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_131([f, b, c, d, g, h], is_dual)
    } else {
        select_69([i, j, e, k, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1417([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_73([f, b, c, d, g], is_dual)
    } else {
        select_69([h, i, e, j, a], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1418([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_103([a, c, g, f, b, h, i, j], is_dual)
    } else {
        select_1087([a, c, b, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1416([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1417([a, b, c, d, f, h, i, g, j, k], is_dual)
    } else {
        select_1418([a, b, e, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1414([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1415([a, b, c, j, g, i, l, h, k, m, n], is_dual)
    } else {
        select_1416([a, d, f, h, e, g, k, i, j, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1410([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1411([a, b, c, j, g, l, i, m, h, k], is_dual)
    } else {
        select_1414([c, a, b, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1395([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1396([g, a, b, c, d, f, h, i, j, l, k, m, n], is_dual)
    } else {
        select_1410([a, b, c, d, e, f, h, i, g, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1421([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1241([g, a, b, c, e, h, f, i], is_dual)
    } else {
        select_1241([f, a, b, d, e, h, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1423([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_733([a, c, b, f, h, g, e], is_dual)
    } else {
        select_404([b, c, d, a, f, e, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_1424([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_961([a, b, c, d, e, f, g], is_dual)
    } else {
        select_961([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_1422([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1423([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_1424([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1420([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1421([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_1422([b, a, d, e, c, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_1429([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_37([a, b, c, d, e, f], is_dual)
    } else {
        select_53([e, c, d, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1428([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_50([b, c, d, e, f, g, h], is_dual)
    } else {
        select_1429([a, h, b, c, f, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1427([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_702([b, c, d, g, f, a, h, i], is_dual)
    } else {
        select_1428([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1426([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_699([g, h, j, f, k, a, i, b], !is_dual)
    } else {
        select_1427([b, c, d, e, a, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1425([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1389([h, a, b, c, j, g, l, i, k, m], is_dual)
    } else {
        select_1426([a, d, e, f, i, g, h, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1419([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < m) || (is_dual && i > m) {
        select_1420([h, a, b, c, j, g, l, m, i], is_dual)
    } else {
        select_1425([c, a, b, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1394([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1395([c, d, b, e, f, g, h, a, i, j, k, l, m, n], is_dual)
    } else {
        select_1419([c, d, a, e, f, g, b, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1380([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1381([b, c, d, e, j, k, a, l, h, i, m, n], is_dual)
    } else {
        select_1394([b, c, d, e, f, a, g, h, i, j, m, k, l, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1329([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_1330([b, c, d, f, g, a, e, h, i, j, k, m, l, n], is_dual)
    } else {
        select_1380([b, c, d, f, g, e, h, i, j, k, a, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_1181([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_1182([a, b, d, e, g, h, f, i, j, c, k, l, m, n, o], is_dual)
    } else {
        select_1329([d, a, b, e, f, g, h, c, i, k, j, l, m, n], is_dual)
    }
}
/// n = 15, i = 6
fn select_923([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_924([a, c, d, b, e, f, g, h, i, j, l, m, o, k, n], is_dual)
    } else {
        select_1181([a, c, b, d, g, h, e, f, i, j, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_922([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_923([a, d, b, c, g, h, f, e, i, j, l, m, k, o, n], is_dual)
    } else {
        select_923([a, d, c, b, g, h, e, f, i, k, l, n, j, o, m], is_dual)
    }
}
/// n = 15, i = 6
fn select_921([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_922([f, e, g, c, d, h, a, b, j, i, l, m, n, k, o], is_dual)
    } else {
        select_922([f, e, h, c, d, g, a, b, j, i, k, m, n, l, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_920([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_921([h, i, f, g, e, a, c, d, l, b, j, k, m, n, o], is_dual)
    } else {
        select_921([h, i, f, g, e, b, c, d, l, a, j, k, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_919([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_920([a, b, c, e, f, g, h, i, j, d, k, l, m, n, o], is_dual)
    } else {
        select_920([a, b, d, e, f, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_918([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_919([g, h, i, j, d, e, c, f, a, b, k, m, n, l, o], is_dual)
    } else {
        select_919([g, h, i, j, d, f, c, e, a, b, k, l, n, m, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_7([a, b, c, d, e, f, g, h, i, k, l, m, n, j, o], is_dual)
    } else {
        select_918([a, j, b, c, d, e, f, g, h, i, l, m, n, k, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_6([k, j, a, h, i, b, c, d, e, f, n, g, l, m, o], is_dual)
    } else {
        select_6([k, j, g, h, i, b, c, d, e, f, n, a, l, m, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_5([a, b, c, d, e, f, g, h, j, k, l, i, m, n, o], is_dual)
    } else {
        select_5([a, b, c, d, e, f, g, i, j, k, l, h, m, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_4([a, b, c, d, e, f, g, h, i, j, l, m, k, n, o], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < o) || (is_dual && n > o) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, m, o], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o], is_dual)
    }
}
/// n = 15, i = 6
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && o < n) || (is_dual && o > n) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    }
}
