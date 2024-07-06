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
/// n = 3, i = 1
fn select_24([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_20([a, b], is_dual)
    } else {
        select_21([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_19([b, c, d], is_dual)
    } else {
        select_24([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_22([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, b, e, f], is_dual)
    } else {
        select_23([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_22([a, b, d, e, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_20([a, d], is_dual)
    } else {
        select_20([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_27([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_28([b, c, e, d], is_dual)
    } else {
        select_24([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_29([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_24([d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_27([f, c, g, e, d, a], !is_dual)
    } else {
        select_29([d, g, c, e, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([a, e, c, f, d, h], is_dual)
    } else {
        select_27([b, d, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_26([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_30([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_16([a, h, d, e, f, i, k], is_dual)
    } else {
        select_25([j, l, h, a, k, g, b, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_33([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_29([d, f, e, a, b], !is_dual)
    } else {
        select_29([e, f, d, a, c], !is_dual)
    }
}
/// n = 5, i = 2
fn select_35([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_24([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_34([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([a, b, c, e, d, f], is_dual)
    } else {
        select_35([d, c, a, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_33([h, d, i, b, a, g], !is_dual)
    } else {
        select_34([a, d, c, f, e, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_38([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_19([a, b, c], is_dual)
    } else {
        select_21([d], is_dual)
    }
}
/// n = 7, i = 2
fn select_37([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_27([a, c, b, e, f, g], is_dual)
    } else {
        select_38([a, c, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_39([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_27([a, c, b, f, g, e], is_dual)
    } else {
        select_27([a, d, b, e, g, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_36([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_37([a, e, c, g, h, f, j], is_dual)
    } else {
        select_39([b, e, d, f, h, i, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_31([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_32([a, b, c, g, h, i, j, k, f], is_dual)
    } else {
        select_36([b, a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_15([a, b, c, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_31([a, b, i, c, d, k, h, g, j, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([b, c, e], is_dual)
    } else {
        select_28([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_44([a, b, c, e, f, g, h], is_dual)
    } else {
        select_19([b, c, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_45([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_44([a, b, c, d, g, e, f], is_dual)
    } else {
        select_44([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_42([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_43([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_45([a, b, h, f, e, i, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_47([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_20([a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, c, d, g], is_dual)
    } else {
        select_28([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_49([a, d, b, g, e, f, h], is_dual)
    } else {
        select_49([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_47([c, d, e, i, g, h], is_dual)
    } else {
        select_48([c, a, h, b, f, g, j, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_42([a, c, d, e, h, g, f, i, j], is_dual)
    } else {
        select_46([b, a, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_53([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_19([c, d, e], is_dual)
    } else {
        select_19([a, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_54([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_19([b, c, d], is_dual)
    } else {
        select_20([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_53([a, b, c, d, e, f], is_dual)
    } else {
        select_54([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_51([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_17([a, c, d, e, g], is_dual)
    } else {
        select_52([a, b, c, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, b, e, f], is_dual)
    } else {
        select_28([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([d, f, g, a, e, b], !is_dual)
    } else {
        select_29([e, f, d, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_56([g, f, h, d, e, c, a], !is_dual)
    } else {
        select_24([e, b, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_50([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_51([f, b, c, d, e, g, h], is_dual)
    } else {
        select_55([a, b, h, g, f, e, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_40([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_41([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_50([b, g, d, e, h, f, a, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_14([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    } else {
        select_40([a, d, g, e, f, h, c, j, i, k, l], is_dual)
    }
}
/// n = 6, i = 1
fn select_61([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_52([a, b, d, e, c, f], is_dual)
    } else {
        select_52([a, c, d, e, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_63([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_19([a, b, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_38([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_62([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_63([b, f, c, d, e, g, h], is_dual)
    } else {
        select_64([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_61([b, c, g, e, f, i], is_dual)
    } else {
        select_62([a, d, b, c, e, f, g, h, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_61([a, b, c, d, e, g], is_dual)
    } else {
        select_61([a, b, c, e, f, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_59([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_60([a, b, c, d, i, g, h, k, j, l], is_dual)
    } else {
        select_65([b, c, h, e, f, j, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_69([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_53([a, f, c, d, e, g], is_dual)
    } else {
        select_53([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_68([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_69([a, c, d, e, b, f, g], is_dual)
    } else {
        select_69([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_71([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([e, a, c, f, d, g], is_dual)
    } else {
        select_27([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_71([c, d, a, e, f, b, g], is_dual)
    } else {
        select_71([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_67([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_68([b, c, d, a, e, f, g], is_dual)
    } else {
        select_70([b, c, d, g, f, a, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_74([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_53([b, c, d, e, g, f], is_dual)
    } else {
        select_38([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_74([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_53([c, g, d, e, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_54([a, b, c, d, e], is_dual)
    } else {
        select_24([e, a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_75([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_47([a, c, d, f, e, g], is_dual)
    } else {
        select_76([a, b, g, e, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_73([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_75([c, f, d, e, g, a, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_66([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_67([g, b, c, e, f, h, i, j], is_dual)
    } else {
        select_72([h, a, d, b, c, f, g, j, k, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_58([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_59([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    } else {
        select_66([b, c, d, e, i, g, h, a, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_13([a, b, c, f, d, e, h, i, j, k, g, l, m], is_dual)
    } else {
        select_58([c, b, d, e, f, a, h, i, g, k, j, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_82([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_54([a, c, d, b, e], is_dual)
    } else {
        select_54([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_81([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_82([a, b, c, f, e], is_dual)
    } else {
        select_82([a, b, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_84([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_38([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_83([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_84([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_84([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_81([a, h, f, j, b, i], !is_dual)
    } else {
        select_83([a, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_86([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_22([a, b, c, f, e, g], is_dual)
    } else {
        select_22([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_87([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_82([d, e, a, g, f], !is_dual)
    } else {
        select_33([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_86([a, c, b, d, e, f, g], is_dual)
    } else {
        select_87([a, e, h, b, c, g, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_79([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_80([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_85([a, f, h, j, b, i, c, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_90([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_22([a, g, i, j, h, f], !is_dual)
    } else {
        select_69([a, d, b, c, e, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_89([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_90([a, b, i, c, f, h, g, j, k, l], is_dual)
    } else {
        select_62([a, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_47([b, c, d, e, f, g], is_dual)
    } else {
        select_27([a, g, b, f, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_91([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_92([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_90([a, b, h, c, e, g, f, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_88([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_89([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_91([a, c, d, i, f, g, h, b, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_78([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_79([g, a, b, c, i, f, j, k, h, l], is_dual)
    } else {
        select_88([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_97([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_44([a, c, d, e, g, f, h], is_dual)
    } else {
        select_28([a, b, e, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_98([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_53([a, b, c, d, e, f], is_dual)
    } else {
        select_38([e, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_96([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_97([a, e, c, d, f, g, h, i], is_dual)
    } else {
        select_98([b, h, c, d, e, g, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_101([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, b, d], is_dual)
    } else {
        select_28([b, c, a, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_100([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_101([c, d, b, e], is_dual)
    } else {
        select_28([a, e, c, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_99([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_100([b, a, d, h, e, g], is_dual)
    } else {
        select_34([g, b, c, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_95([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_96([d, b, a, c, e, g, f, h, i, j], is_dual)
    } else {
        select_99([d, f, b, a, g, e, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_69([b, c, a, d, e, f, g], is_dual)
    } else {
        select_71([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_103([a, c, e, f, h, g, i, j], is_dual)
    } else {
        select_103([a, b, d, g, h, f, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_94([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_95([b, c, g, e, f, a, h, j, i, k], is_dual)
    } else {
        select_102([b, a, d, c, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_103([e, b, d, g, f, h, i, j], is_dual)
    } else {
        select_103([f, a, c, h, e, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_107([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_37([a, c, b, i, h, g, k], is_dual)
    } else {
        select_48([c, d, g, e, f, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_44([b, c, d, e, f, g, h], is_dual)
    } else {
        select_38([a, c, d, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_108([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_43([b, d, e, g, f, h, i, j], is_dual)
    } else {
        select_109([a, b, c, i, f, h, g, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_106([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_107([a, c, h, b, d, g, i, f, j, k, l], is_dual)
    } else {
        select_108([a, d, c, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_104([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_105([d, f, e, c, g, a, i, h, j, k], is_dual)
    } else {
        select_106([b, d, a, e, c, g, i, h, f, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_94([a, b, h, d, e, j, g, i, f, k, l], is_dual)
    } else {
        select_104([b, a, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_77([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_78([a, b, c, i, d, f, j, g, h, l, m, k], is_dual)
    } else {
        select_93([a, c, e, d, b, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_12([b, c, d, a, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_77([b, c, d, f, e, g, h, j, i, a, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_116([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([a, b, c, d, e, g], is_dual)
    } else {
        select_29([f, g, a, d, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_47([b, c, d, e, f, g], is_dual)
    } else {
        select_116([a, g, b, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_73([a, b, c, d, e, g, f, h, j], is_dual)
    } else {
        select_115([f, c, d, e, a, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_119([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_54([a, b, f, d, e], is_dual)
    } else {
        select_54([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_69([a, b, c, d, e, f, g], is_dual)
    } else {
        select_119([a, b, g, e, f, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_118([a, d, e, b, c, f, g], is_dual)
    } else {
        select_103([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_114([a, c, d, b, h, f, g, j, i, k], is_dual)
    } else {
        select_117([a, g, b, e, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_113([a, b, d, e, c, f, i, h, j, g, k], is_dual)
    } else {
        select_113([a, c, d, e, b, f, i, g, j, h, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_123([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_33([b, c, f, e, a, g], is_dual)
    } else {
        select_52([b, c, a, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_122([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_123([a, c, e, b, h, f, g], is_dual)
    } else {
        select_123([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_82([b, c, d, g, i], is_dual)
    } else {
        select_17([a, e, f, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_43([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_22([a, f, c, d, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_124([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_125([b, a, c, f, d, e, h, g, i, j], is_dual)
    } else {
        select_126([b, c, d, e, g, a, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_121([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_122([a, c, d, h, f, g, i, j], is_dual)
    } else {
        select_124([a, b, d, e, g, f, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_130([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_27([a, f, b, e, h, i], is_dual)
    } else {
        select_19([c, d, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_129([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_74([g, b, c, d, e, i, h, k], is_dual)
    } else {
        select_130([h, a, d, e, f, i, g, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_132([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_18([c, d, e, f], is_dual)
    } else {
        select_101([a, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_132([e, f, c, d, g, i, h, j], is_dual)
    } else {
        select_132([e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_128([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_129([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_131([c, d, e, f, a, b, h, i, g, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_135([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_54([b, a, c, d, e], is_dual)
    } else {
        select_28([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_134([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_135([b, a, d, c, e], is_dual)
    } else {
        select_33([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_134([a, b, g, h, c, f], !is_dual)
    } else {
        select_16([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_127([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_128([a, c, e, f, b, d, g, i, h, k, j], is_dual)
    } else {
        select_133([a, i, b, d, g, j, l, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_120([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_121([a, b, c, d, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_127([a, b, c, f, e, i, h, j, g, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_111([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_112([a, d, e, f, g, h, i, j, c, k, l], is_dual)
    } else {
        select_120([a, b, d, e, f, c, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 5, i = 2
fn select_141([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_101([a, b, c, d], is_dual)
    } else {
        select_24([d, b, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_140([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_82([a, c, b, d, e], is_dual)
    } else {
        select_141([a, c, e, b, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_139([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_140([a, e, g, d, b, f], !is_dual)
    } else {
        select_87([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_143([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_71([b, c, d, a, e, f, h], is_dual)
    } else {
        select_71([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_142([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_140([b, e, c, a, g, f], is_dual)
    } else {
        select_143([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_138([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_139([f, g, e, b, a, h, c], !is_dual)
    } else {
        select_142([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_147([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_23([a, b, c, d, f, g], is_dual)
    } else {
        select_101([a, e, g, f], !is_dual)
    }
}
/// n = 6, i = 2
fn select_148([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_27([a, b, c, e, d, f], is_dual)
    } else {
        select_28([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 3
fn select_146([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_147([d, e, g, a, b, f, c], !is_dual)
    } else {
        select_148([a, c, b, d, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_150([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_27([a, c, b, d, e, f], is_dual)
    } else {
        select_23([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_149([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_147([a, b, e, f, h, d, g], is_dual)
    } else {
        select_150([a, d, c, g, f, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_146([a, h, g, d, f, e, c], !is_dual)
    } else {
        select_149([a, c, b, e, f, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_152([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_150([a, e, b, g, h, d, f], is_dual)
    } else {
        select_150([a, d, c, f, h, e, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_154([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_101([a, b, c, e], is_dual)
    } else {
        select_101([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_153([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_154([f, a, b, d, e], is_dual)
    } else {
        select_154([e, a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_151([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_152([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_153([a, f, g, h, d, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_144([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_145([a, b, d, e, f, c, h, g], is_dual)
    } else {
        select_151([a, c, d, b, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_137([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_138([h, a, e, i, f, b, g, c], !is_dual)
    } else {
        select_144([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_159([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_23([a, c, d, e, f, g], is_dual)
    } else {
        select_101([a, b, h, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_160([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_23([a, b, c, f, i, j], is_dual)
    } else {
        select_53([b, c, d, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_158([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_159([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_160([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_162([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_101([a, b, c, d], is_dual)
    } else {
        select_24([a, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_161([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_69([a, d, b, c, e, f, g], is_dual)
    } else {
        select_162([a, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_157([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_158([a, c, d, b, e, f, h, g, i, j], is_dual)
    } else {
        select_161([a, c, d, g, f, b, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_44([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([e, b, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_163([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_100([d, c, a, f, e, g], is_dual)
    } else {
        select_164([c, a, b, e, g, d, f, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_156([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_157([b, c, d, f, e, g, h, i, j, k], is_dual)
    } else {
        select_163([k, l, i, g, f, b, a, j], !is_dual)
    }
}
/// n = 7, i = 2
fn select_168([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_44([a, b, c, d, e, f, g], is_dual)
    } else {
        select_44([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_169([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_44([a, c, d, e, g, f, i], is_dual)
    } else {
        select_101([a, h, b, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_167([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_168([e, c, d, a, f, g, h], is_dual)
    } else {
        select_169([b, a, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_166([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_118([a, d, e, c, h, i, g], is_dual)
    } else {
        select_167([a, b, c, g, f, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_171([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_52([b, e, d, h, g, j], is_dual)
    } else {
        select_34([a, i, c, k, f, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_170([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_171([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_162([a, k, c, l, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_165([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_166([b, a, c, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_170([a, c, b, d, e, f, h, g, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_155([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_156([c, a, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_165([c, b, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_136([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_137([a, k, l, i, c, g, b, d, j], !is_dual)
    } else {
        select_155([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_111([a, c, d, b, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_136([a, i, b, c, d, e, h, j, k, l, m, g], is_dual)
    }
}
/// n = 13, i = 4
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_11([a, b, d, e, g, c, h, i, j, f, k, l, m], is_dual)
    } else {
        select_110([a, b, d, e, g, f, h, i, c, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_10([g, d, e, f, b, c, a, i, k, h, j, l, m], is_dual)
    } else {
        select_10([g, d, f, e, b, c, a, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_9([h, f, g, a, d, e, b, k, c, i, j, l, m], is_dual)
    } else {
        select_9([h, f, g, c, d, e, b, k, a, i, j, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_8([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_8([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_7([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_7([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_183([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_27([f, g, e, c, d, a], !is_dual)
    } else {
        select_24([e, b, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_182([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_100([d, g, a, e, f, b], !is_dual)
    } else {
        select_183([e, g, d, f, a, b, c], !is_dual)
    }
}
/// n = 9, i = 2
fn select_184([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_74([a, b, c, d, g, f, h, i], is_dual)
    } else {
        select_74([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_181([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_182([h, j, f, b, a, i, g], !is_dual)
    } else {
        select_184([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_23([b, c, d, e, f, g], is_dual)
    } else {
        select_101([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_186([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_164([f, h, i, e, d, g, a, b], !is_dual)
    } else {
        select_187([f, d, h, i, e, g, a, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_189([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_27([a, d, b, c, e, f], is_dual)
    } else {
        select_27([a, d, c, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_189([a, c, d, f, e, h], is_dual)
    } else {
        select_189([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_185([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_186([b, c, a, d, e, g, f, h, i], is_dual)
    } else {
        select_188([d, g, h, i, a, e, f, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_180([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_181([b, a, d, e, c, f, g, i, h, j], is_dual)
    } else {
        select_185([i, j, k, a, f, g, b, c, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_192([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_22([a, e, c, f, h, i], is_dual)
    } else {
        select_69([b, d, a, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_194([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_27([a, d, c, e, f, g], is_dual)
    } else {
        select_101([a, b, d, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_195([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, b, c, d, f, g], is_dual)
    } else {
        select_101([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_193([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_194([a, b, c, d, e, g, f], is_dual)
    } else {
        select_195([g, e, h, a, b, f, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_191([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_192([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_193([a, b, c, d, f, i, h, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_198([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, d, e], is_dual)
    } else {
        select_24([a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_197([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_198([f, c, d, e, a, g, h], is_dual)
    } else {
        select_64([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_196([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_51([a, b, d, e, f, c, g], is_dual)
    } else {
        select_197([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_190([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_191([a, b, c, i, d, g, h, k, j, l], is_dual)
    } else {
        select_196([a, g, b, e, f, j, i, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_179([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_180([a, b, c, d, i, g, h, k, l, m, j], is_dual)
    } else {
        select_190([a, d, c, b, e, f, h, g, i, j, l, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_202([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([a, b, c, e, g], is_dual)
    } else {
        select_52([a, d, b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_201([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_182([f, e, h, a, b, g, c], !is_dual)
    } else {
        select_202([b, a, d, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_204([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_147([f, g, h, a, c, e, d], !is_dual)
    } else {
        select_195([f, g, h, a, b, e, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_205([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([a, b, c, e, g], is_dual)
    } else {
        select_52([c, d, a, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_203([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_204([a, c, b, d, f, g, e, h], is_dual)
    } else {
        select_205([a, c, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_200([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_201([b, a, c, d, f, g, h, e], is_dual)
    } else {
        select_203([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_208([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_82([a, b, c, d, f], is_dual)
    } else {
        select_82([a, b, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_209([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_194([b, e, d, c, f, g, h], is_dual)
    } else {
        select_35([a, b, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_207([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_208([b, f, d, e, i, h], is_dual)
    } else {
        select_209([a, b, h, c, f, g, j, i, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_212([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([a, e, b, d, g, f], is_dual)
    } else {
        select_38([a, c, f, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_211([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_82([a, f, c, d, g], is_dual)
    } else {
        select_212([a, b, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_214([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_27([b, d, c, e, g, f], is_dual)
    } else {
        select_29([d, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_215([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_27([b, f, d, e, h, i], is_dual)
    } else {
        select_38([a, c, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_213([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_214([a, c, d, f, e, h, g, j], is_dual)
    } else {
        select_215([a, b, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_210([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_211([c, d, e, f, g, j, i, k], is_dual)
    } else {
        select_213([a, c, b, d, g, k, h, l, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_206([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_207([g, c, d, e, f, a, h, i, j, k, l, m], is_dual)
    } else {
        select_210([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_199([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_200([a, b, c, k, l, h, g, m], is_dual)
    } else {
        select_206([a, c, d, b, e, f, h, g, i, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_178([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_179([b, d, a, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_199([b, d, c, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_220([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_154([a, b, e, d, f], is_dual)
    } else {
        select_24([a, c, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_221([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_141([a, e, b, f, d], is_dual)
    } else {
        select_141([a, d, c, f, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_219([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_220([a, b, c, d, e, g], is_dual)
    } else {
        select_221([a, b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_223([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_198([b, a, c, d, e, f, g], is_dual)
    } else {
        select_189([e, a, g, h, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_224([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_141([a, b, f, g, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_222([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_223([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_224([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_218([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_219([a, c, f, e, b, h, g], is_dual)
    } else {
        select_222([a, c, b, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_227([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_82([b, d, c, e, f], is_dual)
    } else {
        select_195([a, b, f, d, e, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_228([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([d, b, c, e, f], is_dual)
    } else {
        select_189([d, g, h, e, a, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_226([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_227([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_228([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_230([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_194([b, c, d, a, e, f, g], is_dual)
    } else {
        select_29([d, f, e, a, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_232([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_29([b, f, d, a, h], is_dual)
    } else {
        select_49([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_233([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([c, d, g, f, a, e], !is_dual)
    } else {
        select_38([c, b, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_231([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_232([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_233([g, f, a, d, h, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_229([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_230([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_231([b, d, c, f, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_225([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_226([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_229([e, d, h, i, a, f, b, g], !is_dual)
    }
}
/// n = 9, i = 4
fn select_217([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_218([f, d, i, e, a, h, b, g], !is_dual)
    } else {
        select_225([a, b, c, d, f, e, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_237([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_44([c, a, b, d, f, e, g], is_dual)
    } else {
        select_101([a, b, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_238([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([h, e, i, f, c, a], !is_dual)
    } else {
        select_23([g, f, i, e, d, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_236([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_237([d, g, i, h, f, e, a, b], !is_dual)
    } else {
        select_238([g, i, f, h, d, e, a, b, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_239([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_147([a, c, f, d, e, h, i], is_dual)
    } else {
        select_195([a, b, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_235([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_236([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_239([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_241([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_44([a, c, d, b, e, f, g], is_dual)
    } else {
        select_44([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_240([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_164([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_241([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_234([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_235([i, k, h, b, a, j, g, f, c], !is_dual)
    } else {
        select_240([b, a, d, e, h, j, k, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_217([j, k, i, b, h, a, f, c, g], !is_dual)
    } else {
        select_234([a, b, c, d, e, f, h, i, k, g, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_177([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_178([b, a, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_216([b, c, d, e, i, h, k, l, j, a, m], is_dual)
    }
}
/// n = 5, i = 0
fn select_248([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_18([a, b, c, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_247([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_141([a, b, h, i, j], is_dual)
    } else {
        select_248([c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_250([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([c, d, e, g], is_dual)
    } else {
        select_54([a, b, h, f, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_249([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_250([a, b, c, d, e, g, h, i, j], is_dual)
    } else {
        select_250([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_246([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_247([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_249([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_252([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_147([a, b, f, d, h, e, g], is_dual)
    } else {
        select_147([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_251([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_146([d, h, e, a, f, g, b], !is_dual)
    } else {
        select_252([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_245([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_246([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_251([b, i, c, g, a, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_256([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([g, h, f, d, a, b], !is_dual)
    } else {
        select_101([a, f, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_255([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_212([b, c, a, e, d, g, f], is_dual)
    } else {
        select_256([a, c, b, e, f, g, d, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_257([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_256([h, i, g, d, e, a, f, b], !is_dual)
    } else {
        select_256([g, i, h, d, f, a, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_254([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_255([h, d, i, f, e, g, a, b], !is_dual)
    } else {
        select_257([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_253([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_246([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_254([b, i, c, a, g, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_244([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_245([a, c, b, e, f, g, h, d, i, j, k], is_dual)
    } else {
        select_253([a, c, d, e, f, g, h, b, i, k, j, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_261([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([a, c, d, e, f], is_dual)
    } else {
        select_76([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_262([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_82([b, e, c, d, f], is_dual)
    } else {
        select_23([a, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_260([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_261([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_262([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_259([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_67([b, c, d, e, a, g, f, h], is_dual)
    } else {
        select_260([g, a, b, c, d, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_132([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_250([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 1
fn select_267([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_18([d, e, f, g], is_dual)
    } else {
        select_18([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_266([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_64([a, h, d, e, f, g, i, j], is_dual)
    } else {
        select_267([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_264([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_265([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_266([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_263([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_264([a, b, c, e, f, g, h, d, i, j], is_dual)
    } else {
        select_264([a, b, d, e, f, g, h, c, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_258([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_259([b, c, d, e, i, h, a, j, k], is_dual)
    } else {
        select_263([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_243([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_244([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_258([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_272([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_147([a, c, d, b, e, f, g], is_dual)
    } else {
        select_147([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_271([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_272([a, d, c, e, h, f, g], is_dual)
    } else {
        select_143([d, g, i, h, a, e, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_189([a, c, e, d, g, f], is_dual)
    } else {
        select_195([g, h, e, a, b, f, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_276([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, d, c, f], is_dual)
    } else {
        select_28([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_275([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_154([c, a, e, d, g], is_dual)
    } else {
        select_276([a, b, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_273([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_274([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_275([c, b, e, g, f, h, a], is_dual)
    }
}
/// n = 10, i = 4
fn select_270([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_271([h, j, i, a, f, e, b, c, g], !is_dual)
    } else {
        select_273([a, c, d, e, f, i, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_279([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_101([e, f, b, d], is_dual)
    } else {
        select_276([a, d, c, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_278([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_182([i, f, h, g, a, d, b], !is_dual)
    } else {
        select_279([b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_281([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_189([a, b, c, g, i, f], is_dual)
    } else {
        select_34([a, f, d, h, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_282([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_135([c, d, f, e, g], is_dual)
    } else {
        select_187([a, b, c, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_280([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_281([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_282([a, c, b, d, e, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_277([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_278([c, d, a, f, e, g, b, i, h], is_dual)
    } else {
        select_280([a, c, b, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_269([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_270([a, c, b, d, f, e, g, h, i, j], is_dual)
    } else {
        select_277([c, b, a, d, f, g, h, i, j, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_287([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_27([d, f, g, e, a, b], !is_dual)
    } else {
        select_27([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_47([b, c, d, e, f, g], is_dual)
    } else {
        select_287([e, h, i, a, f, g, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_288([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_56([e, h, i, a, f, g, b], !is_dual)
    } else {
        select_47([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_286([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_288([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_291([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_27([a, c, b, f, g, h], is_dual)
    } else {
        select_101([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_290([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_291([a, b, c, h, g, f, i, j], is_dual)
    } else {
        select_64([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_289([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_290([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_290([c, b, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_284([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_285([a, b, e, c, d, g, i, k, h], is_dual)
    } else {
        select_289([c, d, a, f, b, e, g, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_294([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_291([b, a, d, f, h, e, i, g], is_dual)
    } else {
        select_39([b, e, c, d, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_296([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([a, b, e, f, d, g], is_dual)
    } else {
        select_54([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_297([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_44([b, a, c, d, e, f, g], is_dual)
    } else {
        select_101([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_295([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_296([d, b, c, g, f, e, h], is_dual)
    } else {
        select_297([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_293([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_294([a, b, f, c, e, h, i, j, g], is_dual)
    } else {
        select_295([a, b, d, e, g, f, i, j, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_292([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_293([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_293([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_283([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_284([c, b, d, e, a, f, h, g, i, j, k], is_dual)
    } else {
        select_292([d, e, c, f, b, g, h, a, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_268([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_269([b, d, c, e, h, g, i, a, j, k], is_dual)
    } else {
        select_283([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_242([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_243([a, b, d, e, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_268([a, b, d, e, c, j, i, l, h, m, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_176([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < m) || (is_dual && h > m) {
        select_177([a, c, b, d, e, f, g, i, j, k, l, m, h], is_dual)
    } else {
        select_242([a, c, d, b, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_175([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_176([g, c, d, e, f, a, b, h, j, k, i, l, m], is_dual)
    } else {
        select_176([g, c, e, d, f, a, b, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_174([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_175([g, h, a, d, e, f, b, c, i, j, k, l, m], is_dual)
    } else {
        select_175([g, h, c, d, e, f, b, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_173([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_174([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_174([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_172([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_173([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    } else {
        select_173([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_6([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_172([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_310([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_29([e, f, a, b, c], !is_dual)
    } else {
        select_38([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_309([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_141([a, b, f, g, e], is_dual)
    } else {
        select_310([a, c, d, e, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_308([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_274([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_309([b, a, d, e, c, f, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_313([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_53([a, b, c, d, e, f], is_dual)
    } else {
        select_101([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_312([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_64([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_313([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_311([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_73([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_312([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_307([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_308([a, b, c, d, k, l, h, m], is_dual)
    } else {
        select_311([d, a, e, h, f, g, j, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_306([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_307([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_307([a, b, c, e, d, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_316([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_123([b, c, e, a, g, f, h], is_dual)
    } else {
        select_143([a, b, d, c, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_317([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_62([b, a, d, e, f, g, c, h, i], is_dual)
    } else {
        select_265([b, c, d, e, f, g, a, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_315([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_316([a, b, c, i, d, j, g, h, k], is_dual)
    } else {
        select_317([b, c, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_320([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_82([a, e, c, f, h], is_dual)
    } else {
        select_69([b, d, a, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_321([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_17([a, b, e, f, h], is_dual)
    } else {
        select_69([c, d, a, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_319([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_320([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_321([b, d, a, e, c, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_323([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_164([h, i, j, e, a, f, b, g], !is_dual)
    } else {
        select_198([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_325([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_27([a, c, b, d, e, f], is_dual)
    } else {
        select_35([a, b, e, f, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_324([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_56([d, b, c, f, e, g, i], is_dual)
    } else {
        select_325([h, i, d, e, g, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_322([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_323([b, c, a, d, e, f, g, h, i, j], is_dual)
    } else {
        select_324([h, i, j, e, a, b, f, c, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_319([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_322([a, b, c, d, h, f, i, j, g, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_314([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_315([b, c, a, g, e, f, h, j, i, k, l], is_dual)
    } else {
        select_318([a, b, d, c, i, g, h, k, l, m, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_305([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_306([b, a, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_314([b, d, e, c, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_304([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_305([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_305([a, c, b, d, e, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_303([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_304([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    } else {
        select_304([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_302([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_303([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    } else {
        select_303([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_334([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_27([a, c, d, f, e, h], is_dual)
    } else {
        select_27([b, d, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_333([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_334([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_27([b, d, e, c, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_332([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_333([a, c, b, d, e, g, f, h], is_dual)
    } else {
        select_26([e, h, a, g, f, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_331([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_332([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_332([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_336([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_252([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_252([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_338([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_325([b, g, c, a, f, h], is_dual)
    } else {
        select_64([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_337([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_223([a, f, d, e, b, g, h, i], is_dual)
    } else {
        select_338([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_335([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_336([a, b, d, g, f, c, i, h], is_dual)
    } else {
        select_337([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_330([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_331([a, b, i, c, j, f, h, g], !is_dual)
    } else {
        select_335([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_343([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_23([a, c, d, e, f, h], is_dual)
    } else {
        select_38([b, c, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_342([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_100([e, h, a, f, g, b], !is_dual)
    } else {
        select_343([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_344([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([a, b, c, d, e, f], is_dual)
    } else {
        select_23([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_341([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_342([a, b, d, f, e, h, g, i], is_dual)
    } else {
        select_344([a, c, g, e, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_346([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_82([a, e, c, d, f], is_dual)
    } else {
        select_56([g, d, h, a, e, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_348([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_44([a, b, c, d, e, f, g], is_dual)
    } else {
        select_23([d, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_347([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_348([h, d, i, e, a, g, b, f], !is_dual)
    } else {
        select_30([a, b, c, e, d, f, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_345([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_346([e, d, i, a, g, h, b, f], !is_dual)
    } else {
        select_347([a, b, c, e, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_340([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_341([b, a, d, c, e, g, f, h, i], is_dual)
    } else {
        select_345([b, f, d, e, a, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_339([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_340([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_340([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_329([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_330([a, b, c, e, f, d, g, i, h, j], is_dual)
    } else {
        select_339([a, b, c, d, f, e, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_354([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_57([b, c, d, e, f, g], is_dual)
    } else {
        select_276([a, c, d, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_353([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_100([d, c, a, f, e, h], is_dual)
    } else {
        select_354([c, a, b, e, d, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_355([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_325([b, d, c, f, g, e], is_dual)
    } else {
        select_195([b, a, e, g, h, d, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_352([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_353([a, b, c, d, e, f, h, i], is_dual)
    } else {
        select_355([b, a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_357([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_187([f, h, g, i, a, d, b, e], !is_dual)
    } else {
        select_150([a, d, c, g, f, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_356([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_355([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_357([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_351([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_352([g, f, i, e, h, a, b, d, c], !is_dual)
    } else {
        select_356([g, e, i, a, f, h, b, d, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_350([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_351([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_351([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_360([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_220([a, b, d, c, e, f], is_dual)
    } else {
        select_220([a, c, d, b, e, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_362([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_141([a, b, c, d, e], is_dual)
    } else {
        select_101([a, d, b, e], !is_dual)
    }
}
/// n = 7, i = 2
fn select_361([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_362([a, b, f, c, g], is_dual)
    } else {
        select_16([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_359([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_360([a, b, c, f, d, g], is_dual)
    } else {
        select_361([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_364([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_61([a, b, e, c, d, g], is_dual)
    } else {
        select_61([c, d, e, a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_365([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_220([a, b, d, f, e, g], is_dual)
    } else {
        select_362([a, g, f, b, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_363([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_364([a, c, d, e, b, f, g], is_dual)
    } else {
        select_365([b, d, e, c, f, g, a], is_dual)
    }
}
/// n = 8, i = 3
fn select_358([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_359([a, b, c, d, e, f, h], is_dual)
    } else {
        select_363([a, b, e, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_349([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_350([a, b, g, h, i, c, d, f, e], !is_dual)
    } else {
        select_358([a, b, g, i, c, h, d, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_328([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_329([a, b, c, e, d, f, g, i, h, j], is_dual)
    } else {
        select_349([a, b, c, e, g, h, i, d, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_371([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_82([b, c, d, e, f], is_dual)
    } else {
        select_189([a, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_370([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_371([a, b, c, f, e, g, h], is_dual)
    } else {
        select_286([a, d, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_373([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_296([a, b, c, d, f, e, g], is_dual)
    } else {
        select_343([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_374([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_56([d, g, h, a, e, f, b], !is_dual)
    } else {
        select_54([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_372([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_373([a, f, b, d, h, e, i, g, j], is_dual)
    } else {
        select_374([a, e, c, d, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_369([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_370([b, a, c, d, e, g, h, f, i], is_dual)
    } else {
        select_372([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_376([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_252([a, c, g, e, f, b, i, h], is_dual)
    } else {
        select_288([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_375([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_332([e, i, a, j, b, f, h, g], !is_dual)
    } else {
        select_376([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_368([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_369([a, b, c, d, e, g, f, h, j, i], is_dual)
    } else {
        select_375([a, b, d, c, e, f, g, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_367([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_368([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_368([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_380([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_81([a, c, b, e, d, f], is_dual)
    } else {
        select_123([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_382([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_84([e, h, i, j, a, g, c, d], !is_dual)
    } else {
        select_52([e, b, c, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_381([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_382([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_382([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_379([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_380([a, b, c, d, f, g, h], is_dual)
    } else {
        select_381([a, b, e, c, d, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_384([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_123([a, c, d, b, f, e, g], is_dual)
    } else {
        select_272([a, f, e, h, b, g, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_385([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_140([a, f, b, c, g, e], is_dual)
    } else {
        select_123([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_383([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_384([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_385([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_378([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_379([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_383([a, b, c, d, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_389([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_47([b, a, c, d, e, f], is_dual)
    } else {
        select_310([a, e, g, h, f, b], !is_dual)
    }
}
/// n = 7, i = 1
fn select_390([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([e, c, d, f, g], is_dual)
    } else {
        select_17([e, a, b, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_388([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_389([a, f, d, e, b, g, h, i], is_dual)
    } else {
        select_390([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_387([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_388([a, b, e, c, d, f, h, g, i], is_dual)
    } else {
        select_380([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_391([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_380([a, b, c, d, f, g, h], is_dual)
    } else {
        select_337([a, b, e, c, d, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_386([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_387([a, b, e, f, c, d, g, h, i], is_dual)
    } else {
        select_391([a, b, e, f, d, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_377([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_378([a, b, d, f, e, g, c, h, i, j], is_dual)
    } else {
        select_386([a, b, d, f, c, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_366([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_367([a, b, c, f, e, g, h, d, i, j], is_dual)
    } else {
        select_377([a, b, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_327([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_328([a, b, c, d, e, f, h, i, j, g], is_dual)
    } else {
        select_366([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 3, i = 1
fn select_400([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_24([a, b, c], is_dual)
    } else {
        select_24([a, c, b], is_dual)
    }
}
/// n = 6, i = 2
fn select_399([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_400([a, e, f], is_dual)
    } else {
        select_54([a, b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_398([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_399([a, c, g, e, f, h], is_dual)
    } else {
        select_343([a, b, d, f, e, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_401([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_141([d, a, g, h, e], is_dual)
    } else {
        select_33([b, c, e, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_397([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_398([a, b, d, c, e, f, g, i, h], is_dual)
    } else {
        select_401([i, j, e, a, g, b, h, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_396([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_397([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_397([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_404([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_17([a, b, c, d, e], is_dual)
    } else {
        select_17([a, b, f, g, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_403([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_404([a, b, d, e, f, g, h], is_dual)
    } else {
        select_272([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_402([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_360([a, b, f, c, g, e], is_dual)
    } else {
        select_403([a, b, d, c, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_395([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_396([a, b, c, d, e, g, f, i, h, j], is_dual)
    } else {
        select_402([a, b, e, f, g, d, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_408([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_37([b, a, d, e, f, h, g], is_dual)
    } else {
        select_189([b, a, g, c, h, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_407([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_338([a, b, g, d, e, f, h, j], is_dual)
    } else {
        select_408([c, a, f, d, e, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_410([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_82([a, b, c, d, f], is_dual)
    } else {
        select_82([a, c, b, d, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_411([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_56([a, b, c, d, f, g, e], is_dual)
    } else {
        select_56([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_409([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_410([d, g, i, e, a, h], !is_dual)
    } else {
        select_411([b, c, e, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_406([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_407([c, a, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_409([j, f, k, a, g, b, h, i, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_405([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_406([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_406([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_394([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_395([a, b, c, d, h, g, e, j, i, k], is_dual)
    } else {
        select_405([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_416([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_82([b, c, d, e, f], is_dual)
    } else {
        select_24([a, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_417([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_399([a, b, c, d, f, g], is_dual)
    } else {
        select_325([b, a, c, d, g, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_415([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_416([a, c, d, f, e, h, g, j], is_dual)
    } else {
        select_417([a, b, g, e, i, f, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_420([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_27([a, c, b, d, e, g], is_dual)
    } else {
        select_400([b, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_419([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_420([a, b, g, f, e, h, i], is_dual)
    } else {
        select_198([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_421([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_82([a, b, c, d, e], is_dual)
    } else {
        select_101([a, b, e, c], is_dual)
    }
}
/// n = 10, i = 4
fn select_418([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_419([a, b, c, d, e, f, g, i, j], is_dual)
    } else {
        select_421([c, d, f, e, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_414([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_415([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_418([c, a, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_424([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_334([a, b, f, d, e, g, h, i], is_dual)
    } else {
        select_313([b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_423([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_398([c, d, b, e, f, a, g, i, h], is_dual)
    } else {
        select_424([c, d, a, e, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_422([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_423([b, a, e, d, c, g, f, i, h], is_dual)
    } else {
        select_385([b, e, a, f, g, c, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_413([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_414([b, a, e, c, d, g, h, i, f, j], is_dual)
    } else {
        select_422([b, c, d, e, a, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_412([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_413([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_413([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_393([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_394([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_412([a, b, c, d, e, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_392([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_393([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_393([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_326([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_327([a, b, c, d, h, f, i, j, e, k], is_dual)
    } else {
        select_392([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_301([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_302([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_326([a, b, c, d, e, g, j, k, f, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_430([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_384([a, e, h, i, f, b, g, c], !is_dual)
    } else {
        select_384([a, f, h, i, e, b, g, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_434([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_27([a, b, c, f, e, g], is_dual)
    } else {
        select_27([a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_433([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_63([a, b, c, d, f, g, h], is_dual)
    } else {
        select_434([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_432([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_371([a, b, c, h, f, i, g], is_dual)
    } else {
        select_433([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_436([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_56([a, f, b, d, e, g, h], is_dual)
    } else {
        select_150([d, h, g, e, a, f, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_435([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_115([a, f, c, d, e, g, h, j, i], is_dual)
    } else {
        select_436([a, h, b, e, i, f, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_431([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_432([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_435([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_429([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_430([a, b, c, h, g, i, j, k, f], is_dual)
    } else {
        select_431([a, c, b, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_428([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_429([a, b, d, e, f, c, g, h, i, j, k], is_dual)
    } else {
        select_429([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_439([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_380([a, b, d, e, c, f, g], is_dual)
    } else {
        select_133([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_438([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_439([a, b, c, e, f, d, g, h], is_dual)
    } else {
        select_439([a, b, d, e, f, c, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_442([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_182([d, f, g, b, a, e, c], !is_dual)
    } else {
        select_148([d, f, a, b, g, e], !is_dual)
    }
}
/// n = 7, i = 3
fn select_441([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_442([a, b, d, c, e, f, g], is_dual)
    } else {
        select_442([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_443([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_380([a, b, c, d, e, f, g], is_dual)
    } else {
        select_404([a, b, d, f, g, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_440([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_441([a, b, c, f, g, d, h], is_dual)
    } else {
        select_443([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_437([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_438([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_440([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_427([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_428([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_437([a, b, c, i, k, g, d, j], !is_dual)
    }
}
/// n = 11, i = 4
fn select_426([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_427([a, b, c, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_427([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_450([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_248([c, d, e, f, h], is_dual)
    } else {
        select_23([a, b, i, g, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_451([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_248([c, d, e, f, g], is_dual)
    } else {
        select_162([a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_449([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_450([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_451([a, h, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_448([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_449([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_449([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 1
fn select_454([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_248([e, f, g, h, i], is_dual)
    } else {
        select_248([a, b, c, d, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_455([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_47([a, b, c, d, f, g], is_dual)
    } else {
        select_18([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 1
fn select_453([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_454([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_455([i, e, f, g, h, a, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_457([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_17([a, b, c, d, f], is_dual)
    } else {
        select_17([a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_456([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_457([a, c, d, e, f, h], is_dual)
    } else {
        select_197([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_452([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_453([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_456([a, i, e, f, g, h, b, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_447([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < i) || (is_dual && c > i) {
        select_448([a, b, i, e, f, g, h, c, j, k, l], is_dual)
    } else {
        select_452([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_446([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_447([a, b, c, d, f, g, h, i, e, j, k, l], is_dual)
    } else {
        select_447([a, b, c, e, f, g, h, i, d, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_460([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_86([a, b, c, d, e, f, g], is_dual)
    } else {
        select_83([a, b, e, h, i, g, c, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_459([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_67([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_460([a, g, f, j, b, i, h, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_458([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_459([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_459([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_445([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_446([a, b, c, e, f, d, g, h, i, j, k, l], is_dual)
    } else {
        select_458([a, b, c, e, f, j, d, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_467([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_27([a, h, b, f, i, j], is_dual)
    } else {
        select_18([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_466([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_132([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_467([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_468([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_64([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_64([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_465([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_466([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_468([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_464([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_384([a, j, k, g, c, b, i, h], !is_dual)
    } else {
        select_465([a, c, b, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_471([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_22([a, b, c, e, g, h], is_dual)
    } else {
        select_52([a, d, b, c, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_470([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_51([a, c, d, e, f, g, h], is_dual)
    } else {
        select_471([a, b, h, c, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_469([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_470([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_470([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_463([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_464([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_469([a, b, d, e, c, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_462([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_463([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_463([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_475([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_25([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_25([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_476([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_140([a, b, c, f, g, e], is_dual)
    } else {
        select_86([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_474([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_475([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_476([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_473([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_474([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_476([a, b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_472([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_458([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_473([a, b, c, g, j, d, i, h], !is_dual)
    }
}
/// n = 11, i = 4
fn select_461([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_462([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_472([a, b, c, h, e, f, d, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_444([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < e) || (is_dual && j > e) {
        select_445([a, b, c, d, f, g, e, h, i, j, k, l, m], is_dual)
    } else {
        select_461([a, b, c, d, f, g, j, e, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_425([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_426([a, b, c, d, e, g, j, k, f, l, m], is_dual)
    } else {
        select_444([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_300([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_301([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_425([a, b, c, d, e, f, g, h, i, j, l, m, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_486([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_74([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_63([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_485([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_312([b, g, c, e, f, a, h, i, j], is_dual)
    } else {
        select_486([c, a, d, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_484([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_485([a, b, e, c, f, g, d, h, i, j], is_dual)
    } else {
        select_485([a, b, e, d, f, g, c, h, i, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_489([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_47([g, d, e, f, a, h], is_dual)
    } else {
        select_267([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_488([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_489([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_51([a, g, d, e, f, b, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_487([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_488([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_81([a, b, d, h, g, c], is_dual)
    }
}
/// n = 10, i = 2
fn select_483([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_484([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_487([a, c, d, h, f, g, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_493([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_22([e, f, d, h, g, a], !is_dual)
    } else {
        select_71([b, c, a, e, f, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_494([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_212([a, c, b, f, e, g, d], is_dual)
    } else {
        select_212([b, c, a, f, d, g, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_492([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_493([a, b, c, d, e, f, h, i], is_dual)
    } else {
        select_494([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_491([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_319([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_492([f, i, j, a, b, g, c, h, d], !is_dual)
    }
}
/// n = 11, i = 4
fn select_497([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_248([c, d, e, f, g], is_dual)
    } else {
        select_34([a, j, k, b, i, h], !is_dual)
    }
}
/// n = 12, i = 4
fn select_496([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_450([a, b, c, d, e, f, g, h, i, j, l], is_dual)
    } else {
        select_497([a, h, c, d, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_495([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_246([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_496([a, c, d, e, f, g, h, b, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_490([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_491([a, b, c, d, j, h, k, i, l, m], is_dual)
    } else {
        select_495([a, b, c, e, f, g, i, h, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_483([c, a, e, f, g, b, i, h, j, k], is_dual)
    } else {
        select_490([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 11, i = 2
fn select_501([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_62([a, c, d, e, f, i, h, j, k], is_dual)
    } else {
        select_212([a, c, b, h, g, k, i], is_dual)
    }
}
/// n = 9, i = 1
fn select_503([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_63([a, h, d, e, f, g, i], is_dual)
    } else {
        select_267([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_502([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_81([a, g, b, j, h, k], is_dual)
    } else {
        select_503([c, a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_500([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_501([b, d, a, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_502([b, d, c, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_506([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_48([b, a, c, d, f, e, g, h], is_dual)
    } else {
        select_71([c, e, d, g, a, f, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_505([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_506([a, c, j, b, g, h, k, i, l], is_dual)
    } else {
        select_455([c, d, e, f, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_508([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_467([f, a, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_64([h, b, c, d, e, f, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_507([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_353([a, c, b, g, h, k, i, l], is_dual)
    } else {
        select_508([b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_504([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_505([a, d, b, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_507([a, d, c, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_499([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_500([c, d, b, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_504([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_498([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_483([c, a, e, f, g, b, i, h, j, k], is_dual)
    } else {
        select_499([a, c, b, d, e, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_481([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_482([a, b, d, c, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_498([a, c, d, b, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_480([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_481([a, c, d, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_481([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_479([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_480([a, b, c, e, f, g, h, i, d, j, k, l, m], is_dual)
    } else {
        select_480([a, b, d, e, f, g, h, i, c, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_515([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_100([d, c, a, f, e, g], is_dual)
    } else {
        select_297([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_514([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_515([c, a, d, b, e, f, g, h], is_dual)
    } else {
        select_515([c, b, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_516([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_51([c, e, a, b, d, f, g], is_dual)
    } else {
        select_16([a, b, c, d, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_513([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_514([b, c, a, d, g, h, i, f], is_dual)
    } else {
        select_516([b, c, d, e, a, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_512([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_513([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_513([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_518([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_134([a, c, d, e, f, g], is_dual)
    } else {
        select_141([a, b, g, h, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_520([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_22([a, g, f, h, d, b], !is_dual)
    } else {
        select_22([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_519([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_520([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_362([a, g, d, b, h], !is_dual)
    }
}
/// n = 8, i = 3
fn select_517([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_518([a, c, b, d, e, g, f, h], is_dual)
    } else {
        select_519([a, c, d, e, f, g, b, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_511([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_512([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_517([a, b, d, e, g, h, c, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_523([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_342([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_342([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_522([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_359([a, b, f, c, i, h, g], !is_dual)
    } else {
        select_523([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_521([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_473([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_522([a, b, c, f, i, d, h, g, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_510([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_511([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_521([a, b, c, g, e, d, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_525([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_380([a, b, c, g, i, h, f], !is_dual)
    } else {
        select_83([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_524([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < d) || (is_dual && k > d) {
        select_446([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_525([a, b, c, e, j, k, d, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_509([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_510([a, b, c, d, f, j, e, k, l, m], is_dual)
    } else {
        select_524([a, b, c, d, f, e, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_478([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_479([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_509([a, b, c, d, e, f, g, h, i, k, l, m, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_477([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_478([a, b, c, d, e, g, h, i, j, f, k, l, m], is_dual)
    } else {
        select_478([a, b, c, d, f, g, h, i, j, e, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_299([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_300([a, b, c, d, e, f, h, i, j, k, g, l, m], is_dual)
    } else {
        select_477([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_298([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_299([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_6([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_5([a, b, c, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_298([a, b, c, d, e, f, g, h, j, k, i, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_536([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_475([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_133([a, b, g, e, i, h, c, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_535([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_359([a, b, g, i, e, c, h], !is_dual)
    } else {
        select_536([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_539([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_90([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_90([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_538([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_380([a, b, i, j, g, c, h], !is_dual)
    } else {
        select_539([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_537([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_538([a, b, c, f, d, e, g, h, i, j], is_dual)
    } else {
        select_538([a, b, c, f, e, d, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_534([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_535([a, b, c, e, g, h, i, d, j], is_dual)
    } else {
        select_537([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_544([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_82([b, c, d, e, f], is_dual)
    } else {
        select_310([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_543([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_68([b, c, d, e, f, g, h], is_dual)
    } else {
        select_544([a, b, c, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_545([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_81([g, h, e, i, a, b], !is_dual)
    } else {
        select_404([b, e, c, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_542([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_543([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_545([b, a, c, d, h, i, j, k, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_542([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_542([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_540([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_535([a, b, c, e, h, i, j, d, k], is_dual)
    } else {
        select_541([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_533([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_534([a, b, d, c, e, i, k, h, l, j], is_dual)
    } else {
        select_540([a, b, c, d, f, g, h, j, i, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_551([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_74([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_434([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_553([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_101([b, c, e, g], is_dual)
    } else {
        select_38([a, d, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_554([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_101([a, g, c, i], is_dual)
    } else {
        select_49([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_552([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_553([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_554([b, a, d, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_550([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_551([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_552([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_549([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_229([a, b, c, g, h, i, f, j], is_dual)
    } else {
        select_550([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_548([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_549([a, b, h, e, c, f, i, g, j, k], is_dual)
    } else {
        select_549([a, b, g, e, d, f, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_558([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_194([a, b, c, h, f, g, i], is_dual)
    } else {
        select_47([c, d, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_560([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_27([a, e, b, d, g, h], is_dual)
    } else {
        select_20([c, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_559([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_37([g, d, b, c, f, h, j], is_dual)
    } else {
        select_560([h, a, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_557([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_558([a, b, e, c, d, g, f, h, i], is_dual)
    } else {
        select_559([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_562([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_82([a, f, d, g, b], !is_dual)
    } else {
        select_22([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_561([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_220([a, b, c, d, e, f], is_dual)
    } else {
        select_562([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_556([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_557([a, b, d, e, c, f, g, h, j, i], is_dual)
    } else {
        select_561([a, j, f, h, b, i, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_563([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_557([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_222([a, j, f, k, h, b, i, g], !is_dual)
    }
}
/// n = 12, i = 4
fn select_555([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_556([a, b, e, d, g, f, i, h, j, k], is_dual)
    } else {
        select_563([a, b, c, h, e, f, i, g, k, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_547([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_548([a, c, b, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_555([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_566([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_488([a, c, e, b, f, g, h, i], is_dual)
    } else {
        select_488([a, b, d, c, f, h, g, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_568([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_182([h, g, i, a, b, e, c], !is_dual)
    } else {
        select_87([g, h, i, a, b, f, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_569([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_353([b, c, d, a, f, e, g, h], is_dual)
    } else {
        select_346([b, a, d, e, c, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_567([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_568([a, c, e, d, g, f, i, h, j], is_dual)
    } else {
        select_569([a, b, c, e, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_565([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_566([c, b, d, f, a, e, h, g, j, i], is_dual)
    } else {
        select_567([a, b, c, d, f, i, h, g, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_573([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_297([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_297([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_575([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_27([a, f, b, e, d, g], is_dual)
    } else {
        select_54([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_574([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_71([a, f, b, e, i, g, j], is_dual)
    } else {
        select_575([e, c, d, h, f, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_572([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_573([d, b, f, e, a, g, i, h, j], is_dual)
    } else {
        select_574([b, a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_578([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_44([a, b, f, d, g, e, h], is_dual)
    } else {
        select_44([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_577([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_296([a, f, b, e, g, i, j], is_dual)
    } else {
        select_578([a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_579([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_232([e, b, a, d, f, h, i, g], is_dual)
    } else {
        select_232([d, c, a, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_576([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_577([b, a, d, c, e, g, f, h, j, i], is_dual)
    } else {
        select_579([d, b, f, e, a, g, j, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_571([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_572([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_576([a, d, c, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_582([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_56([a, b, c, d, e, f, g], is_dual)
    } else {
        select_28([a, g, f, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_581([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_582([a, b, c, h, e, g, i], is_dual)
    } else {
        select_346([a, c, d, g, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_584([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_150([h, i, g, a, b, e, c], !is_dual)
    } else {
        select_150([g, i, h, a, b, f, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_585([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_17([a, b, c, e, g], is_dual)
    } else {
        select_52([b, d, a, c, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_583([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_584([a, c, b, d, g, f, i, j, h], is_dual)
    } else {
        select_585([c, b, e, a, h, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_580([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_581([a, d, f, e, b, g, h, i, j], is_dual)
    } else {
        select_583([a, c, d, e, b, g, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_570([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_571([a, b, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_580([b, c, a, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_564([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_565([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    } else {
        select_570([b, c, d, h, f, g, a, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_546([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_547([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_564([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_532([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_533([a, b, c, d, f, e, i, h, k, l, m, j], is_dual)
    } else {
        select_546([a, b, c, d, e, h, g, j, i, l, k, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_592([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_71([a, b, c, d, e, f, h], is_dual)
    } else {
        select_34([e, h, i, a, g, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_591([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_192([a, b, c, d, e, g, f, h, j], is_dual)
    } else {
        select_592([f, b, d, a, e, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_64([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_64([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_593([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_594([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_503([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_590([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_591([a, i, c, d, b, g, h, j, k, l], is_dual)
    } else {
        select_593([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_597([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_74([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_23([f, d, e, a, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_596([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_489([f, b, c, d, e, g, h, i], is_dual)
    } else {
        select_597([f, a, i, b, c, g, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_600([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_54([b, c, d, g, f], is_dual)
    } else {
        select_29([a, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_599([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_600([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_187([f, a, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_598([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_312([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_599([g, b, d, e, a, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_595([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_596([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_598([b, h, c, e, f, a, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_589([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_590([a, b, d, e, f, g, c, h, i, k, j, l], is_dual)
    } else {
        select_595([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_604([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_399([a, b, f, d, e, g], is_dual)
    } else {
        select_23([a, c, e, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_603([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_92([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_604([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_602([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_603([a, b, d, e, c, f, g, h], is_dual)
    } else {
        select_603([a, c, d, e, b, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_606([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_265([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_265([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_607([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_266([a, c, d, e, f, g, b, h, i, j], is_dual)
    } else {
        select_266([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_605([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_606([a, b, h, e, f, g, c, i, j], is_dual)
    } else {
        select_607([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_601([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_602([a, b, c, e, i, h, d, j], is_dual)
    } else {
        select_605([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_588([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_589([a, b, c, d, f, e, i, h, k, j, l, m], is_dual)
    } else {
        select_601([a, b, d, e, g, c, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_613([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([a, b, c, e, d, g], is_dual)
    } else {
        select_27([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_612([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_613([a, b, e, g, h, i, j], is_dual)
    } else {
        select_575([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_614([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_232([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_232([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_611([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_612([a, f, c, d, g, h, e, j, i, k], is_dual)
    } else {
        select_614([a, d, b, e, g, f, j, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_616([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_135([a, c, f, h, i], is_dual)
    } else {
        select_578([b, a, d, e, g, f, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_617([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_296([b, c, d, h, f, g, i], is_dual)
    } else {
        select_354([a, b, c, e, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_615([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_616([a, d, f, c, h, g, e, j, k, i], is_dual)
    } else {
        select_617([c, e, b, d, g, h, f, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_610([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_611([a, g, b, d, f, i, h, e, j, k, l], is_dual)
    } else {
        select_615([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_609([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_610([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_610([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_622([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_54([a, c, d, e, h], is_dual)
    } else {
        select_49([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_621([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_622([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_195([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_620([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_621([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_621([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_619([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_620([a, b, e, c, g, h, f, j, i], is_dual)
    } else {
        select_620([a, b, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_618([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_619([a, b, d, g, e, f, i, h, k, j], is_dual)
    } else {
        select_335([a, b, f, h, c, j, g, k, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_608([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_609([a, b, d, f, e, c, h, g, i, j, k, l], is_dual)
    } else {
        select_618([a, b, f, c, e, d, h, g, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_587([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_588([a, b, c, e, d, f, g, h, i, k, j, l, m], is_dual)
    } else {
        select_608([a, b, c, e, f, i, h, j, d, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_628([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_22([a, f, h, i, g, b], !is_dual)
    } else {
        select_52([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_627([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_371([e, h, i, f, a, b, g], !is_dual)
    } else {
        select_628([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_630([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_64([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_39([a, g, b, c, f, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_629([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_389([f, g, d, e, a, h, j, k], is_dual)
    } else {
        select_630([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_626([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_627([b, a, c, d, j, g, h, k, l], is_dual)
    } else {
        select_629([b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_633([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([c, d, e, f, g], is_dual)
    } else {
        select_189([c, a, b, g, h, f], is_dual)
    }
}
/// n = 10, i = 2
fn select_632([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_371([c, a, b, d, j, g, i], is_dual)
    } else {
        select_633([a, b, c, e, f, i, h, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_631([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_632([c, d, a, f, b, e, g, i, j, h], is_dual)
    } else {
        select_632([c, d, b, f, a, e, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_625([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_626([b, a, e, f, c, d, h, j, i, g, k, l], is_dual)
    } else {
        select_631([b, e, c, d, f, a, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_637([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_56([e, h, j, a, f, g, c], !is_dual)
    } else {
        select_37([b, c, d, e, f, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_636([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_582([a, b, g, e, h, i, f], is_dual)
    } else {
        select_637([a, c, b, d, e, f, g, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_639([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_26([f, j, a, h, g, b, c], !is_dual)
    } else {
        select_215([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_640([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_52([b, f, d, e, g, h], is_dual)
    } else {
        select_575([a, b, c, h, i, f, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_638([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_639([a, b, h, e, g, f, k, i, j, l], is_dual)
    } else {
        select_640([b, c, e, d, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_635([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_636([b, h, c, d, f, a, i, k, l, j], is_dual)
    } else {
        select_638([b, a, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_643([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_56([d, g, i, a, f, e, c], !is_dual)
    } else {
        select_27([b, d, c, f, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_642([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_582([a, c, f, d, g, h, e], is_dual)
    } else {
        select_643([a, b, c, d, f, e, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_645([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_334([a, b, e, c, h, g, f, i], is_dual)
    } else {
        select_30([a, b, d, f, g, e, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_644([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_645([b, c, e, d, g, h, f, i, j], is_dual)
    } else {
        select_639([a, b, f, d, g, e, j, i, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_641([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_642([b, c, h, e, a, f, j, k, i], is_dual)
    } else {
        select_644([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_634([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_635([b, a, c, d, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_641([a, b, c, e, h, j, g, i, f, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_624([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_625([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_634([a, b, e, f, d, g, h, i, c, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_623([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_624([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    } else {
        select_624([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_586([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_587([a, b, c, d, e, g, f, i, h, j, k, l, m], is_dual)
    } else {
        select_623([a, b, c, d, f, e, i, h, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_532([a, b, d, c, e, g, f, j, i, k, h, m, l], is_dual)
    } else {
        select_586([a, b, c, d, f, g, e, j, h, k, l, i, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_530([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_531([a, b, c, e, d, f, g, i, j, k, l, h, m], is_dual)
    } else {
        select_531([a, b, d, e, c, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_529([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_530([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_530([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_528([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_529([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_529([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_655([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_398([a, b, c, e, g, f, i, j, h], is_dual)
    } else {
        select_309([a, f, b, d, h, i, j, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_656([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_194([a, f, b, d, e, h, g], is_dual)
    } else {
        select_49([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_654([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_655([a, b, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_656([b, d, f, e, g, c, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_659([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_194([a, b, c, d, g, h, f], is_dual)
    } else {
        select_194([a, b, c, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_658([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_68([a, b, d, e, f, h, g], is_dual)
    } else {
        select_659([a, b, c, d, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_661([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([e, c, d, f, g], is_dual)
    } else {
        select_116([a, g, b, e, h, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_660([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_261([e, f, c, d, a, g, i, j], is_dual)
    } else {
        select_661([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_657([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_658([b, c, d, e, a, g, f, h, i], is_dual)
    } else {
        select_660([a, d, b, c, g, h, i, f, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_653([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_654([a, d, e, b, f, g, c, h, i, j], is_dual)
    } else {
        select_657([a, d, e, c, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_665([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_56([d, a, c, f, e, g, h], is_dual)
    } else {
        select_29([d, h, g, e, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_664([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_220([f, d, c, a, e, g], is_dual)
    } else {
        select_665([a, c, b, e, f, g, d, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_666([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_411([b, c, d, a, e, f, g], is_dual)
    } else {
        select_153([a, c, d, b, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_663([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_664([a, c, b, d, e, f, g, i], is_dual)
    } else {
        select_666([d, a, b, f, e, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_670([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_27([d, b, c, e, f, g], is_dual)
    } else {
        select_35([a, b, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_27([a, g, c, f, d, i], is_dual)
    } else {
        select_49([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_669([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_670([a, f, b, d, e, h, i, g], is_dual)
    } else {
        select_671([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_668([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_669([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_231([a, f, b, h, e, g, d, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_673([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_297([h, i, j, g, a, e, b, c], !is_dual)
    } else {
        select_30([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_672([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_230([a, b, c, f, g, h, e, j], is_dual)
    } else {
        select_673([b, a, c, d, e, f, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_667([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_668([c, b, d, f, e, g, a, h, i], is_dual)
    } else {
        select_672([c, b, a, d, f, g, h, e, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_662([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_663([h, j, e, a, f, i, c, b, g], !is_dual)
    } else {
        select_667([b, c, a, d, f, e, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_652([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_653([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_662([b, c, d, e, g, h, i, a, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_677([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_134([a, c, d, e, f, g], is_dual)
    } else {
        select_410([a, b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_676([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_677([a, b, c, d, h, g, i], is_dual)
    } else {
        select_65([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_680([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_56([g, h, i, a, e, b, c], !is_dual)
    } else {
        select_71([b, c, d, g, e, f, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_681([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_63([a, g, c, d, e, f, h], is_dual)
    } else {
        select_63([b, f, c, d, e, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_679([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_680([a, i, b, c, g, h, f, j, k], is_dual)
    } else {
        select_681([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_684([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_53([b, c, d, e, g, h], is_dual)
    } else {
        select_54([a, d, e, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_683([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_187([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_684([a, d, e, b, c, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_683([a, b, h, d, e, f, g, i, k, j], is_dual)
    } else {
        select_683([a, c, g, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_678([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_679([a, d, b, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_682([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_675([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_676([b, a, d, e, c, h, g, j, i], is_dual)
    } else {
        select_678([a, b, c, f, d, e, g, h, i, k, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_688([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_195([a, f, b, d, g, e, h], is_dual)
    } else {
        select_195([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_689([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_97([b, d, a, c, e, f, h, g], is_dual)
    } else {
        select_256([a, b, c, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_687([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_688([b, d, f, g, e, a, i, h], is_dual)
    } else {
        select_689([b, d, c, a, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_691([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_74([b, d, e, a, c, f, g, h], is_dual)
    } else {
        select_150([g, f, i, a, b, h, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_692([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_26([h, e, a, f, g, b, c], !is_dual)
    } else {
        select_26([h, f, a, e, g, b, d], !is_dual)
    }
}
/// n = 10, i = 3
fn select_690([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_691([a, b, f, d, e, h, i, g, j], is_dual)
    } else {
        select_692([a, b, c, g, i, j, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_686([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_687([a, c, g, d, f, b, i, h, j], is_dual)
    } else {
        select_690([a, c, d, b, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_696([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_54([b, d, e, h, i], is_dual)
    } else {
        select_49([a, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_695([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_696([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_54([g, d, e, a, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_694([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_645([b, a, h, c, g, j, f, k, i], is_dual)
    } else {
        select_695([b, c, f, d, e, g, i, h, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_698([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_195([b, a, g, f, i, e, h], is_dual)
    } else {
        select_313([b, c, d, e, f, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_697([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_419([b, a, d, f, h, e, g, j, i], is_dual)
    } else {
        select_698([a, b, c, g, e, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_693([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_694([b, c, e, a, d, g, h, f, j, i, k], is_dual)
    } else {
        select_697([b, c, f, e, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_685([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_686([a, b, c, h, d, i, g, f, j, k], is_dual)
    } else {
        select_693([b, a, c, d, e, g, f, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_674([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_675([b, c, e, a, d, f, g, h, i, j, k], is_dual)
    } else {
        select_685([b, c, e, f, d, g, h, i, a, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_651([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_652([a, b, c, i, d, e, g, k, j, h, l], is_dual)
    } else {
        select_674([a, b, d, c, e, f, g, i, h, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_650([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_651([a, b, d, e, f, g, c, h, i, j, k, l], is_dual)
    } else {
        select_651([a, c, d, e, f, g, b, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_649([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_650([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    } else {
        select_650([a, b, c, e, f, g, h, d, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_648([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_326([a, b, c, d, e, i, g, j, f, k, l], is_dual)
    } else {
        select_649([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_705([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_134([a, c, e, d, f, g], is_dual)
    } else {
        select_471([a, f, e, i, h, b, g, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_708([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_28([b, c, d, e], is_dual)
    } else {
        select_24([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_707([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_708([a, c, d, e, f, g, h], is_dual)
    } else {
        select_101([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_706([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_416([f, a, h, i, d, b, g, e], !is_dual)
    } else {
        select_707([b, a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_704([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_705([a, e, h, i, b, c, g, f, d], !is_dual)
    } else {
        select_706([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_711([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_296([a, g, b, d, f, e, h], is_dual)
    } else {
        select_52([d, b, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_710([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_346([a, b, g, e, f, h, d, i], is_dual)
    } else {
        select_711([b, a, c, e, f, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_709([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_710([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_710([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_703([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_704([a, b, c, g, e, h, i, j, f], is_dual)
    } else {
        select_709([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_714([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_232([a, e, c, g, f, h, d, i], is_dual)
    } else {
        select_671([a, c, b, d, f, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_713([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_182([d, j, e, a, b, i, h], !is_dual)
    } else {
        select_714([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_712([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_713([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_713([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_702([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_703([a, b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_712([a, b, c, e, d, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_701([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_702([a, b, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_702([a, b, c, e, f, d, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_700([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_701([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_701([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_720([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_51([a, b, c, d, e, f, g], is_dual)
    } else {
        select_162([a, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_719([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_85([a, f, b, j, c, i, h, g], !is_dual)
    } else {
        select_720([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_718([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_719([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_719([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_724([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_22([a, f, i, j, h, g], !is_dual)
    } else {
        select_69([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_723([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_724([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_724([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_722([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_380([a, b, i, j, f, c, h], !is_dual)
    } else {
        select_723([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_721([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_722([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_722([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_717([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_718([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_721([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_727([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_470([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_470([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_726([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_727([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_727([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_725([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_458([a, b, c, e, f, h, d, i, j, k], is_dual)
    } else {
        select_726([a, b, c, e, f, d, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_716([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_717([a, b, c, d, f, h, i, e, j, k], is_dual)
    } else {
        select_725([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_715([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_701([a, b, c, d, e, h, i, f, j, k], is_dual)
    } else {
        select_716([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_699([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_700([a, b, c, d, e, h, i, j, f, k], is_dual)
    } else {
        select_715([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_647([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_648([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_699([a, b, c, d, e, f, i, k, g, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_737([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_212([a, b, f, e, h, g, d], is_dual)
    } else {
        select_578([b, a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_736([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_737([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_560([a, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_739([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_194([b, e, d, c, f, g, h], is_dual)
    } else {
        select_560([a, b, d, g, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_738([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_739([a, b, g, c, e, f, i, j, h], is_dual)
    } else {
        select_739([a, b, f, d, e, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_735([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_736([c, a, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_738([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_734([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_735([a, b, d, c, e, j, g, l, m, i], is_dual)
    } else {
        select_567([a, d, c, b, f, i, h, k, m, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_742([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_286([a, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_621([a, c, b, h, e, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_741([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_742([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_742([a, c, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_743([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_319([a, b, c, h, d, g, j, k], is_dual)
    } else {
        select_311([b, a, c, g, e, f, i, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_740([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_741([a, b, d, c, f, i, k, h, j, m], is_dual)
    } else {
        select_743([a, d, c, e, b, h, g, j, i, l, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_733([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_734([a, b, d, e, c, f, h, i, g, k, j, m, l], is_dual)
    } else {
        select_740([a, d, c, e, b, f, g, i, j, k, h, m, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_749([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_27([f, c, b, e, h, g], is_dual)
    } else {
        select_276([a, c, d, g, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_748([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_100([f, c, a, h, e, g], is_dual)
    } else {
        select_749([c, b, a, e, d, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_747([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_281([b, a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_748([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_751([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_135([b, d, g, f, i], is_dual)
    } else {
        select_325([a, c, h, e, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_750([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_751([a, c, b, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_751([b, c, a, d, e, g, f, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_746([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_747([e, a, c, d, f, g, h, j, i], is_dual)
    } else {
        select_750([b, c, a, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_755([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_27([e, d, g, f, a, b], !is_dual)
    } else {
        select_29([b, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_754([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_399([a, c, d, f, e, h], is_dual)
    } else {
        select_755([a, b, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_757([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([a, c, e, h, d, g], is_dual)
    } else {
        select_57([b, c, d, h, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_756([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([d, e, c, g, f], is_dual)
    } else {
        select_757([a, b, f, d, e, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_753([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_754([a, b, f, e, h, g, d, i], is_dual)
    } else {
        select_756([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_760([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_44([b, c, d, e, f, g, h], is_dual)
    } else {
        select_708([a, b, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_759([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_760([a, c, b, e, f, h, g, i, j, k], is_dual)
    } else {
        select_45([c, d, g, f, e, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_761([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_325([b, c, d, f, g, e], is_dual)
    } else {
        select_195([a, b, e, f, d, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_758([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_759([a, c, d, b, e, f, g, h, j, i, k], is_dual)
    } else {
        select_761([a, b, f, g, i, h, j, e, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_752([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_753([c, e, d, f, a, g, h, i, j], is_dual)
    } else {
        select_758([b, c, a, d, f, g, e, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_745([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_746([c, d, b, e, f, a, g, i, j, h, k], is_dual)
    } else {
        select_752([c, d, a, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_765([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_56([a, b, c, d, e, g, f], is_dual)
    } else {
        select_287([a, b, c, d, f, g, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_764([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_123([a, d, g, e, h, f, i], is_dual)
    } else {
        select_765([j, k, g, a, b, c, i], !is_dual)
    }
}
/// n = 11, i = 3
fn select_766([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_123([a, d, h, f, j, g, i], is_dual)
    } else {
        select_544([a, b, c, e, i, k, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_763([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_764([a, b, d, c, e, g, i, j, k, h, l], is_dual)
    } else {
        select_766([c, b, d, a, f, e, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_769([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_296([b, d, c, g, h, f, j], is_dual)
    } else {
        select_44([a, c, f, e, g, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_768([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_769([c, b, a, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_100([i, c, a, j, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_773([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, f, c, h], is_dual)
    } else {
        select_28([b, e, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_772([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_276([a, c, d, g, h, j], is_dual)
    } else {
        select_773([a, b, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_771([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_772([a, c, b, d, e, f, h, i, g, j], is_dual)
    } else {
        select_279([a, c, f, e, d, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_770([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_346([e, d, c, g, a, f, i, h], is_dual)
    } else {
        select_771([b, a, c, e, f, d, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_767([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_768([b, a, d, c, f, e, i, j, g, h, k], is_dual)
    } else {
        select_770([b, e, d, g, a, f, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_762([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_763([a, b, c, d, f, e, g, i, h, j, k, l], is_dual)
    } else {
        select_767([a, c, d, f, h, g, b, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_744([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_745([b, a, c, e, d, i, h, k, l, g, j], is_dual)
    } else {
        select_762([a, b, d, e, f, c, g, i, j, k, h, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_732([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_733([b, c, a, e, d, f, h, g, i, j, l, k, m], is_dual)
    } else {
        select_744([b, a, c, e, d, i, h, l, k, j, g, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_731([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_732([d, e, f, b, c, a, h, j, g, i, l, k, m], is_dual)
    } else {
        select_732([d, f, e, b, c, a, h, i, g, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_730([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_731([g, e, f, a, c, d, j, b, h, i, l, k, m], is_dual)
    } else {
        select_731([g, e, f, b, c, d, j, a, h, i, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_729([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_730([a, b, c, e, f, g, h, d, i, j, k, l, m], is_dual)
    } else {
        select_730([a, b, d, e, f, g, h, c, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_728([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_729([a, b, c, d, e, g, h, i, f, j, k, l, m], is_dual)
    } else {
        select_729([a, b, c, d, f, g, h, i, e, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_646([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_647([a, b, c, d, e, f, h, j, k, l, g, m], is_dual)
    } else {
        select_728([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_527([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_528([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    } else {
        select_646([a, b, c, d, e, f, g, h, i, k, l, m, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_526([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_527([a, b, c, d, e, f, g, i, j, h, k, l, m], is_dual)
    } else {
        select_527([a, b, c, d, e, f, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < i) || (is_dual && l > i) {
        select_4([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_526([a, b, c, d, e, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_2([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, m, l], is_dual)
    }
}
