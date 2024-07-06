/// n = 1, i = 0
fn select_20([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_19([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_20([a], is_dual)
    } else {
        select_20([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_18([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_19([a, b], is_dual)
    } else {
        select_19([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_21([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_19([a, b], is_dual)
    } else {
        select_20([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_21([a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_17([a, b, c, d, e, f], is_dual)
    } else {
        select_17([a, b, c, d, e, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([c, d, e], is_dual)
    } else {
        select_18([a, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_24([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_19([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_22([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_23([a, b, c, d, e, f], is_dual)
    } else {
        select_24([e, c, d, a, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_15([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_16([a, c, d, e, g, h, i], is_dual)
    } else {
        select_22([e, b, c, d, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_19([a, d], is_dual)
    } else {
        select_19([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_27([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, b, e, f], is_dual)
    } else {
        select_28([a, c, d, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_29([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_21([d, a, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_27([d, f, g, a, e, b], !is_dual)
    } else {
        select_29([e, f, d, b, c], !is_dual)
    }
}
/// n = 4, i = 0
fn select_31([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_18([a, b, c], is_dual)
    } else {
        select_18([b, c, d], is_dual)
    }
}
/// n = 6, i = 1
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([b, c, d, e], is_dual)
    } else {
        select_19([a, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_25([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_26([e, h, i, a, f, g, b], !is_dual)
    } else {
        select_30([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_14([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_15([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_25([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, d, c, f], is_dual)
    } else {
        select_28([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_17([b, c, d, g, f, i], is_dual)
    } else {
        select_35([i, j, e, h, a, b], !is_dual)
    }
}
/// n = 4, i = 1
fn select_37([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, b, d], is_dual)
    } else {
        select_28([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_36([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_17([a, c, d, e, f, g], is_dual)
    } else {
        select_37([a, b, h, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_33([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_34([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_36([a, e, c, d, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_32([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_33([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_33([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_13([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_14([i, j, k, f, g, a, d, b, h], !is_dual)
    } else {
        select_32([f, g, i, j, k, a, h, e, d, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_42([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, e], is_dual)
    } else {
        select_28([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_42([b, a, c, d, e, f, g], is_dual)
    } else {
        select_37([a, e, c, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_40([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_26([b, f, c, h, e, g, j], is_dual)
    } else {
        select_41([h, k, f, i, e, d, a, g], !is_dual)
    }
}
/// n = 3, i = 1
fn select_45([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_21([a, b, c], is_dual)
    } else {
        select_21([a, c, b], is_dual)
    }
}
/// n = 6, i = 2
fn select_44([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_45([a, e, f], is_dual)
    } else {
        select_24([a, b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_47([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_28([b, c, e, d], is_dual)
    } else {
        select_21([a, b, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_46([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_47([a, f, b, d, i, j], is_dual)
    } else {
        select_28([c, g, e, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_43([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_44([b, c, f, g, e, i], is_dual)
    } else {
        select_46([g, i, k, f, j, h, d, a, b, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_39([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_40([j, k, h, i, g, d, f, a, e, b, c], !is_dual)
    } else {
        select_43([j, k, g, i, h, d, a, f, b, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_50([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_47([a, e, b, d, g, h], is_dual)
    } else {
        select_19([c, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_51([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_21([a, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_50([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_51([f, c, a, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_47([a, c, b, f, g, h], is_dual)
    } else {
        select_37([b, e, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_54([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_23([a, b, c, d, e, f], is_dual)
    } else {
        select_37([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_52([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_53([b, a, c, f, g, e, i, h], is_dual)
    } else {
        select_54([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_48([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_49([e, h, l, f, k, a, b, i], !is_dual)
    } else {
        select_52([b, c, f, d, e, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_38([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_39([h, e, l, i, k, j, b, g, f, a, c], !is_dual)
    } else {
        select_48([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_13([e, i, l, g, k, a, f, j, b, h, c], !is_dual)
    } else {
        select_38([a, b, c, d, e, g, h, i, f, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_28([b, c, d, e], is_dual)
    } else {
        select_21([a, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_59([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_60([a, c, d, f, g, i, k], is_dual)
    } else {
        select_60([b, c, e, f, g, h, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_62([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_18([a, b, c], is_dual)
    } else {
        select_20([d], is_dual)
    }
}
/// n = 7, i = 3
fn select_61([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_47([c, d, g, f, a, e], !is_dual)
    } else {
        select_62([c, b, d, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_59([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_61([d, k, a, g, i, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([a, d, c, g, e, h], is_dual)
    } else {
        select_37([e, f, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_47([a, e, c, f, d, h], is_dual)
    } else {
        select_47([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_63([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_64([i, j, g, h, a, b, e, c], !is_dual)
    } else {
        select_65([a, c, d, g, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_58([h, i, k, f, g, j, a, d, e, b, c], !is_dual)
    } else {
        select_63([a, i, h, k, f, j, g, d, e, b], !is_dual)
    }
}
/// n = 6, i = 2
fn select_68([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_47([a, b, c, e, d, f], is_dual)
    } else {
        select_51([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_69([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_47([a, b, c, e, d, g], is_dual)
    } else {
        select_47([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_67([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_68([a, d, c, f, e, h], is_dual)
    } else {
        select_69([a, b, e, d, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_71([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_17([a, c, d, b, e, f], is_dual)
    } else {
        select_17([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_71([a, e, d, g, f, b], !is_dual)
    } else {
        select_68([a, c, b, e, d, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_66([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_67([a, e, b, g, d, i, h, f, j], is_dual)
    } else {
        select_70([a, g, j, f, h, e, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_57([a, b, c, e, f, d, h, g, i, j, k], is_dual)
    } else {
        select_66([a, b, c, d, f, g, h, e, i, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_75([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, d, b, e], is_dual)
    } else {
        select_24([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_47([a, d, b, c, e, f], is_dual)
    } else {
        select_47([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_74([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_75([b, c, d, e, f], is_dual)
    } else {
        select_76([a, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_77([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_71([a, f, h, i, g, b], !is_dual)
    } else {
        select_22([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_74([e, h, i, f, a, b, g], !is_dual)
    } else {
        select_77([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_79([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_76([a, b, c, g, i, f], is_dual)
    } else {
        select_68([a, f, d, h, e, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_81([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_29([d, f, e, a, b], !is_dual)
    } else {
        select_29([e, f, d, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_80([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_81([c, d, f, e, h, g], is_dual)
    } else {
        select_76([e, a, b, g, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_78([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_79([a, b, c, d, e, h, g, i, f], is_dual)
    } else {
        select_80([b, c, a, d, e, h, f, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_73([g, a, i, j, e, f, h, b, c], !is_dual)
    } else {
        select_78([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_55([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_56([a, k, j, f, h, i, b, e, c, g, d], !is_dual)
    } else {
        select_72([a, e, j, k, f, h, b, i, c, g], !is_dual)
    }
}
/// n = 12, i = 5
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_12([b, c, a, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_55([f, b, c, h, e, g, i, a, k, l, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_11([a, b, c, d, g, h, f, i, j, e, k, l], is_dual)
    } else {
        select_11([b, a, c, d, g, h, e, i, j, f, k, l], is_dual)
    }
}
/// n = 7, i = 1
fn select_88([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, f, c, d, e, g], is_dual)
    } else {
        select_23([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_87([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_88([a, c, d, e, b, f, g], is_dual)
    } else {
        select_88([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_90([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_47([e, a, c, f, d, g], is_dual)
    } else {
        select_47([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_90([c, d, a, e, f, b, g], is_dual)
    } else {
        select_90([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_87([b, c, d, a, e, f, g], is_dual)
    } else {
        select_89([b, c, d, g, f, a, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_77([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_77([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_86([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_91([a, g, i, f, j, b, h, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_84([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_85([a, f, i, j, k, g, b, h, c, d], !is_dual)
    } else {
        select_85([a, g, i, j, k, f, b, h, c, e], !is_dual)
    }
}
/// n = 6, i = 2
fn select_96([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_35([a, b, c, e, d, f], is_dual)
    } else {
        select_35([a, b, d, e, c, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_97([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_42([a, b, c, d, e, f, g], is_dual)
    } else {
        select_42([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_95([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_96([b, f, d, e, a, g], is_dual)
    } else {
        select_97([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_99([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, b, f, d, e], is_dual)
    } else {
        select_24([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_101([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, c, d, g], is_dual)
    } else {
        select_28([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_100([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_101([a, d, b, g, e, f, h], is_dual)
    } else {
        select_101([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_98([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_99([c, d, e, g, f, a], is_dual)
    } else {
        select_100([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_95([a, c, b, f, h, g, e], is_dual)
    } else {
        select_98([b, c, d, a, f, e, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([c, d, e, f], is_dual)
    } else {
        select_37([a, b, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_105([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_47([a, b, c, d, e, f], is_dual)
    } else {
        select_21([a, f, b], is_dual)
    }
}
/// n = 8, i = 2
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_104([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_105([a, b, g, c, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_107([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_29([e, f, a, b, c], !is_dual)
    } else {
        select_62([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_108([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_47([e, d, g, f, a, b], !is_dual)
    } else {
        select_29([b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_106([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_107([a, f, c, d, i, g], is_dual)
    } else {
        select_108([h, f, i, g, a, e, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_102([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_103([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_106([c, b, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_93([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_94([a, b, c, f, g, e, i, h], is_dual)
    } else {
        select_102([e, a, b, d, h, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_92([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_93([c, a, d, b, g, f, h, i, e, j], is_dual)
    } else {
        select_93([c, b, d, a, g, e, h, i, f, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_83([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_84([h, a, i, b, c, f, g, j, k, l, e], is_dual)
    } else {
        select_92([b, c, a, d, f, g, h, e, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_113([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_99([a, b, g, e, f, d], is_dual)
    } else {
        select_88([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_114([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_90([b, g, c, e, a, f, h], is_dual)
    } else {
        select_88([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_113([a, d, e, b, c, f, g], is_dual)
    } else {
        select_114([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_117([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_31([a, b, c, d], is_dual)
    } else {
        select_20([e], is_dual)
    }
}
/// n = 7, i = 2
fn select_116([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_71([a, b, c, d, f, g], is_dual)
    } else {
        select_117([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_99([a, c, d, b, e, f], is_dual)
    } else {
        select_90([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_116([a, e, b, h, i, g, f], !is_dual)
    } else {
        select_118([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_111([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_112([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_115([a, b, f, j, c, i, h, d, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_110([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_111([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_111([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_109([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_110([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_110([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_82([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_83([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_109([g, h, e, k, l, f, a, b, j, i], !is_dual)
    }
}
/// n = 12, i = 5
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_10([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_82([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_9([a, b, c, d, e, f, g, h, j, l, i, k], is_dual)
    } else {
        select_9([a, b, d, c, e, f, h, g, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_8([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_8([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_7([e, f, b, a, c, i, h, j, g, d, l, k], is_dual)
    } else {
        select_7([e, f, b, a, d, i, h, k, g, c, l, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_6([g, a, e, f, b, c, k, j, d, h, i, l], is_dual)
    } else {
        select_6([g, d, e, f, b, c, k, j, a, h, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_5([a, b, c, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_5([a, b, c, d, f, g, h, e, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_4([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_4([a, b, c, d, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_3([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, j, l, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_2([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, l, k], is_dual)
    }
}
