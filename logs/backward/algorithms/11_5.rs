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
/// n = 6, i = 1
fn select_15([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([c, d, e], is_dual)
    } else {
        select_16([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_14([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_15([a, f, c, d, e, g], is_dual)
    } else {
        select_15([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_13([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_14([a, c, d, e, b, f, g], is_dual)
    } else {
        select_14([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_21([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_17([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_15([a, b, c, d, e, f], is_dual)
    } else {
        select_21([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_19([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_20([a, b, d, e, c, f], is_dual)
    } else {
        select_20([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_12([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_13([a, d, e, b, c, f, g], is_dual)
    } else {
        select_19([a, b, c, d, f, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_26([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_17([a, b], is_dual)
    } else {
        select_18([c], is_dual)
    }
}
/// n = 3, i = 1
fn select_25([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_26([a, b, c], is_dual)
    } else {
        select_26([a, c, b], is_dual)
    }
}
/// n = 4, i = 1
fn select_27([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_17([a, d], is_dual)
    } else {
        select_17([b, c], is_dual)
    }
}
/// n = 5, i = 2
fn select_24([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_25([a, d, e], is_dual)
    } else {
        select_27([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_16([b, c, d], is_dual)
    } else {
        select_26([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_24([b, e, d, a, f], is_dual)
    } else {
        select_28([b, a, c, d, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_27([b, c, e, d], is_dual)
    } else {
        select_26([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_32([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_26([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_32([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_33([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c, e, d, g], is_dual)
    } else {
        select_31([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_30([a, g, d, e, f, b], !is_dual)
    } else {
        select_33([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_22([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([a, b, d, h, f, g], is_dual)
    } else {
        select_29([g, i, h, a, e, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_11([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_12([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_22([b, d, e, c, f, g, a, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_37([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([a, d, b, c, e, f], is_dual)
    } else {
        select_31([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_37([f, a, b, c, e, g], is_dual)
    } else {
        select_37([e, a, b, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_36([b, c, d, g, f, a, h], is_dual)
    } else {
        select_13([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, d, b, e, f], is_dual)
    } else {
        select_28([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_39([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_40([a, f, h, i, g, b], !is_dual)
    } else {
        select_20([b, a, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_38([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_39([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_39([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_35([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_38([a, g, i, f, j, b, h, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_10([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_11([a, i, f, j, k, g, b, c, h], !is_dual)
    } else {
        select_34([a, c, d, e, b, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_9([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_10([a, f, g, j, k, b, h, i, c, d, e], !is_dual)
    } else {
        select_10([a, f, h, j, k, b, g, i, c, e, d], !is_dual)
    }
}
/// n = 4, i = 0
fn select_47([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_16([a, b, c], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_46([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_47([a, b, c, d], is_dual)
    } else {
        select_18([e], is_dual)
    }
}
/// n = 7, i = 2
fn select_45([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_40([a, b, c, d, f, g], is_dual)
    } else {
        select_46([b, c, d, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_49([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_21([a, b, f, d, e], is_dual)
    } else {
        select_21([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_50([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([e, a, c, f, d, g], is_dual)
    } else {
        select_31([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_48([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_49([a, c, d, b, e, f], is_dual)
    } else {
        select_50([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_44([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_45([a, e, b, h, i, g, f], !is_dual)
    } else {
        select_48([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_52([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_49([a, b, g, e, f, d], is_dual)
    } else {
        select_14([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_14([b, c, a, d, e, f, g], is_dual)
    } else {
        select_50([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_52([a, d, e, b, c, f, g], is_dual)
    } else {
        select_53([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_43([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_44([a, b, f, j, c, i, h, d, g], !is_dual)
    } else {
        select_51([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_42([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_43([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_43([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_42([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_42([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_8([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_9([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    } else {
        select_41([a, h, f, j, k, g, b, c, i, d], !is_dual)
    }
}
/// n = 6, i = 2
fn select_61([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_27([a, d, c, f], is_dual)
    } else {
        select_27([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_60([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_61([a, b, c, e, d, f], is_dual)
    } else {
        select_61([a, b, d, e, c, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_63([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_16([a, b, c], is_dual)
    } else {
        select_18([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_62([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_28([a, c, d, e, f, h], is_dual)
    } else {
        select_63([b, c, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_59([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_60([e, h, a, f, g, b], !is_dual)
    } else {
        select_62([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_65([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([f, g, e, c, d, a], !is_dual)
    } else {
        select_26([e, b, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_67([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, b, d], is_dual)
    } else {
        select_27([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([g, h, f, d, a, b], !is_dual)
    } else {
        select_67([a, f, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_64([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_65([a, b, d, e, g, h, i], is_dual)
    } else {
        select_66([e, a, c, d, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_59([g, j, e, k, d, a, h, f], !is_dual)
    } else {
        select_64([b, a, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_70([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_67([b, d, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_71([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_31([a, f, b, e, d, g], is_dual)
    } else {
        select_21([b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_70([g, h, i, a, b, e, c], !is_dual)
    } else {
        select_71([b, c, d, g, e, f, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_68([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_59([a, c, d, f, e, h, g, j], is_dual)
    } else {
        select_69([e, i, h, k, f, j, a, b, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_58([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    } else {
        select_68([e, i, j, k, a, f, h, b, c, g, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_75([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([a, e, b, d, g, h], is_dual)
    } else {
        select_17([c, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_74([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_75([b, a, c, g, e, i, h, f], is_dual)
    } else {
        select_37([b, a, f, d, h, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_78([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_27([a, b, c, d], is_dual)
    } else {
        select_26([d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_77([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_78([e, f, a, b, c], !is_dual)
    } else {
        select_63([b, c, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_76([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_77([a, d, e, g, f, i], is_dual)
    } else {
        select_37([d, b, c, f, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_73([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_74([c, d, b, e, f, a, g, h, i], is_dual)
    } else {
        select_76([c, a, b, d, e, g, f, i, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_81([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_21([b, a, c, d, e], is_dual)
    } else {
        select_27([b, e, d, a], is_dual)
    }
}
/// n = 6, i = 2
fn select_80([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_30([a, c, d, b, e, f], is_dual)
    } else {
        select_81([a, b, d, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_79([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_80([a, f, b, c, g, e], is_dual)
    } else {
        select_48([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_73([a, d, c, e, b, g, f, i, h], is_dual)
    } else {
        select_79([a, d, e, f, g, b, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_56([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_57([a, c, b, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_72([a, i, j, e, k, f, b, c, h], !is_dual)
    }
}
/// n = 6, i = 2
fn select_85([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_24([a, f, e, b, c], !is_dual)
    } else {
        select_37([a, b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_84([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_85([a, c, d, f, b, g], is_dual)
    } else {
        select_19([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_88([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_47([b, c, d, e], is_dual)
    } else {
        select_63([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_87([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_88([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_88([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_86([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_53([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_87([a, e, f, i, j, h, c, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_84([a, f, i, j, e, b, h], !is_dual)
    } else {
        select_86([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_92([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_31([a, c, d, f, e, h], is_dual)
    } else {
        select_31([b, d, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_31([a, b, c, f, e, g], is_dual)
    } else {
        select_31([a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_92([a, b, f, c, e, g, h, i], is_dual)
    } else {
        select_93([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_95([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_31([a, c, b, d, e, g], is_dual)
    } else {
        select_25([b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_95([a, b, f, e, d, g, h], is_dual)
    } else {
        select_28([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_91([b, a, c, d, e, g, f, i, h], is_dual)
    } else {
        select_94([f, b, d, a, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_98([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_67([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_97([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_98([a, c, d, b, e, f, g], is_dual)
    } else {
        select_98([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_97([a, f, e, i, b, h, g], !is_dual)
    } else {
        select_53([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_89([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_90([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_96([a, i, e, k, f, b, j, h, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_82([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_83([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_89([a, e, k, j, b, f, i, h, c, g, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_55([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_56([a, b, c, d, e, g, h, i, j, k, f], is_dual)
    } else {
        select_82([a, e, k, j, b, g, i, f, h, d, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_103([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_30([a, d, g, e, f, b], !is_dual)
    } else {
        select_30([a, e, g, d, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_102([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_97([a, d, c, f, h, e, g], is_dual)
    } else {
        select_103([a, b, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_104([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_45([a, f, c, d, e, g, h], is_dual)
    } else {
        select_29([a, g, b, h, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_102([a, c, g, e, b, f, h, i], is_dual)
    } else {
        select_104([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_30([a, e, b, h, g, i], is_dual)
    } else {
        select_71([a, c, d, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_108([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_30([d, b, c, e, f, g], is_dual)
    } else {
        select_71([d, h, i, a, e, g, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_107([a, c, d, b, f, g, e, h, i], is_dual)
    } else {
        select_108([a, e, d, g, b, f, i, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_111([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_27([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_110([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_111([a, b, f, d, g, e], is_dual)
    } else {
        select_71([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_110([a, b, c, d, e, f, g], is_dual)
    } else {
        select_103([a, b, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_106([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_109([a, h, j, e, b, i, f, g, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_100([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_101([a, i, k, g, b, j, e, h, c], !is_dual)
    } else {
        select_105([a, g, k, i, b, j, h, e, f, d], !is_dual)
    }
}
/// n = 6, i = 2
fn select_116([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_78([d, f, e, a, b], !is_dual)
    } else {
        select_78([e, f, d, a, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_115([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_77([f, h, d, i, b, g], !is_dual)
    } else {
        select_116([a, c, d, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_119([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([b, c, e], is_dual)
    } else {
        select_27([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_118([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_119([a, b, c, d, e, f, g], is_dual)
    } else {
        select_63([e, b, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_120([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_28([b, c, d, e, f, g], is_dual)
    } else {
        select_67([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_117([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_118([f, h, i, e, d, g, a, b], !is_dual)
    } else {
        select_120([f, d, h, i, e, g, a, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_115([a, c, f, d, g, i, e, h, j], is_dual)
    } else {
        select_117([b, c, e, h, d, g, f, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_103([f, e, h, a, b, g, c], !is_dual)
    } else {
        select_48([b, c, d, a, f, e, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_114([b, a, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_121([g, h, k, d, e, a, j, f], !is_dual)
    }
}
/// n = 5, i = 2
fn select_125([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_78([b, c, a, d, e], is_dual)
    } else {
        select_27([a, c, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_126([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_119([b, a, c, d, e, f, g], is_dual)
    } else {
        select_67([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_124([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_125([a, f, c, g, e], is_dual)
    } else {
        select_126([g, i, f, h, a, d, b, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_128([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_61([a, d, c, g, e, h], is_dual)
    } else {
        select_67([e, f, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_127([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_128([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_128([a, b, c, d, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_124([g, f, i, d, h, b, a, e, c], !is_dual)
    } else {
        select_127([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_131([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_119([a, c, d, e, f, g, i], is_dual)
    } else {
        select_61([a, b, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_130([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_60([f, d, a, g, e, h], is_dual)
    } else {
        select_131([d, b, a, c, e, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_66([d, h, i, e, g, a, b, f], !is_dual)
    } else {
        select_28([a, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_129([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_130([i, j, k, g, d, e, b, a, h], !is_dual)
    } else {
        select_132([b, d, c, g, e, f, h, j, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_122([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_123([a, b, f, d, h, g, i, j, e], is_dual)
    } else {
        select_129([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_113([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_122([h, i, k, e, f, j, b, g, a, c, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_99([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_100([a, b, c, d, f, g, h, i, k, j, e], is_dual)
    } else {
        select_112([a, h, k, j, f, e, i, b, c, g, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_54([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([a, f, g, k, b, c, h, j, i, e, d], !is_dual)
    } else {
        select_99([a, f, h, k, b, g, j, c, i, d, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_7([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_8([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_54([a, b, c, e, d, f, g, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_138([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_85([a, h, i, e, b, g], !is_dual)
    } else {
        select_85([a, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_137([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_138([a, b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_138([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_141([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, b, d, e], is_dual)
    } else {
        select_24([b, c, a, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_143([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_67([b, c, e, g], is_dual)
    } else {
        select_63([a, d, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_142([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_126([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_143([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_140([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_141([a, f, g, c, d], !is_dual)
    } else {
        select_142([a, f, h, i, e, g, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_140([a, g, i, j, f, b, h, c, e], !is_dual)
    } else {
        select_138([a, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_136([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_137([a, f, i, j, b, g, h, c, d], !is_dual)
    } else {
        select_139([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_147([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([f, d, b, a, e], !is_dual)
    } else {
        select_31([b, c, a, d, f, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_149([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_31([f, c, g, e, d, a], !is_dual)
    } else {
        select_78([d, g, c, e, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_148([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_149([h, g, a, e, f, b, c], !is_dual)
    } else {
        select_149([h, g, a, e, f, b, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_147([c, a, e, f, b, g], is_dual)
    } else {
        select_148([a, c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_152([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_78([a, d, g, f, c], is_dual)
    } else {
        select_78([b, c, g, e, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_151([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_152([g, h, e, f, a, b, c], !is_dual)
    } else {
        select_78([f, g, b, c, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_150([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_147([c, d, e, f, a, g], is_dual)
    } else {
        select_151([c, b, d, a, f, e, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_146([a, b, d, c, f, g, e, h], is_dual)
    } else {
        select_150([b, a, d, c, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_154([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_110([a, b, c, e, f, d, g], is_dual)
    } else {
        select_110([a, b, d, e, f, c, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_156([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_30([a, c, d, f, e, g], is_dual)
    } else {
        select_125([a, b, c, g, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_155([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_147([a, b, c, e, h, g], is_dual)
    } else {
        select_156([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_153([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_154([a, c, b, g, e, h, f], is_dual)
    } else {
        select_155([a, c, d, b, e, f, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_144([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_145([b, g, h, c, e, a, f, i], is_dual)
    } else {
        select_153([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_135([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_136([a, e, j, k, f, b, c, i, g, h], !is_dual)
    } else {
        select_144([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_161([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_152([g, i, e, h, a, c, b], !is_dual)
    } else {
        select_111([b, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_111([a, b, d, g, f, h], is_dual)
    } else {
        select_149([g, i, a, h, e, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_160([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_161([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_162([a, c, b, d, f, g, e, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_160([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_109([b, c, g, e, f, h, a, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_158([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_159([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_83([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_166([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_98([a, b, g, e, h, f, i], is_dual)
    } else {
        select_88([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_167([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_98([a, b, f, d, h, e, g], is_dual)
    } else {
        select_98([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_165([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_166([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_167([a, c, g, e, f, b, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_169([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_111([a, b, c, e, f, g], is_dual)
    } else {
        select_111([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_168([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_147([b, c, f, e, a, g], is_dual)
    } else {
        select_169([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_164([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_165([a, b, d, c, e, g, f, h, i], is_dual)
    } else {
        select_168([g, a, c, b, e, h, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_172([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_152([e, g, d, f, a, c, b], !is_dual)
    } else {
        select_149([d, g, a, f, e, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_23([g, d, h, a, f, b], !is_dual)
    } else {
        select_172([d, h, g, a, f, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_175([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([a, e, c, f, d, h], is_dual)
    } else {
        select_31([b, d, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_174([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_149([d, h, a, e, g, b, f], !is_dual)
    } else {
        select_175([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_173([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_110([a, b, c, d, f, e, g], is_dual)
    } else {
        select_174([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_170([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_171([a, d, c, e, b, g, f, h], is_dual)
    } else {
        select_173([a, d, b, e, f, g, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_163([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_164([b, a, c, d, e, g, h, f, i], is_dual)
    } else {
        select_170([b, f, h, c, e, a, g, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_158([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_163([a, e, f, k, b, h, c, j, i], !is_dual)
    }
}
/// n = 11, i = 5
fn select_134([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_135([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_157([a, e, f, k, b, c, j, g, i, h, d], !is_dual)
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
        select_21([b, c, d, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_181([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_182([a, b, c, d, f, e, g], is_dual)
    } else {
        select_62([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_184([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_125([d, b, c, e, f], is_dual)
    } else {
        select_111([d, f, g, a, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_180([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_181([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_184([a, b, f, d, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_160([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_180([b, c, g, e, f, h, a, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_178([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_179([a, e, f, k, b, h, j, i, c, g], !is_dual)
    } else {
        select_89([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_187([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_85([a, b, g, h, c, f], !is_dual)
    } else {
        select_45([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_186([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_51([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_187([a, b, c, d, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_185([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_186([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_186([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_177([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_178([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_185([a, h, e, f, k, b, c, j, i], !is_dual)
    }
}
/// n = 11, i = 5
fn select_176([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_177([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_177([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_133([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_134([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_176([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_6([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_7([e, b, c, a, d, h, i, g, j, f, k], is_dual)
    } else {
        select_133([e, b, c, a, h, i, g, f, k, d, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_5([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_6([f, a, d, e, b, j, i, c, g, h, k], is_dual)
    } else {
        select_6([f, c, d, e, b, j, i, a, g, h, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_4([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_5([a, b, c, d, f, g, e, h, i, j, k], is_dual)
    } else {
        select_5([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_3([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_4([a, b, c, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_4([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_2([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_3([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, i, k, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_2([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_0([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
