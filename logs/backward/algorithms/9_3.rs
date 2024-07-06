/// n = 1, i = 0
fn select_14([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_13([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_14([a], is_dual)
    } else {
        select_14([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_12([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_13([a, d], is_dual)
    } else {
        select_13([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_15([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_13([a, b], is_dual)
    } else {
        select_14([c], is_dual)
    }
}
/// n = 5, i = 2
fn select_11([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_12([a, b, c, d], is_dual)
    } else {
        select_15([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_10([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_11([b, c, a, d, e], is_dual)
    } else {
        select_12([a, c, b, d], is_dual)
    }
}
/// n = 3, i = 0
fn select_18([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_13([a, b], is_dual)
    } else {
        select_13([b, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_17([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_13([a, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_16([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_17([a, c, d, b, e], is_dual)
    } else {
        select_17([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_9([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_10([a, c, e, b, f], is_dual)
    } else {
        select_16([a, c, b, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_21([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([b, c, e], is_dual)
    } else {
        select_12([a, f, d, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_22([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_18([b, c, d], is_dual)
    } else {
        select_15([a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_21([a, b, c, d, e, f, g], is_dual)
    } else {
        select_22([d, b, c, e, f, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, b, d], is_dual)
    } else {
        select_12([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_23([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_22([b, c, d, e, f, g], is_dual)
    } else {
        select_24([a, g, h, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_20([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_23([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_8([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_9([a, e, c, d, f, g], is_dual)
    } else {
        select_19([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([a, b, c, d, f, g], is_dual)
    } else {
        select_24([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_27([a, c, d, b, e, f, g], is_dual)
    } else {
        select_27([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_17([a, b, f, d, e], is_dual)
    } else {
        select_17([a, c, e, d, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_12([b, c, e, d], is_dual)
    } else {
        select_15([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([e, a, c, f, d, g], is_dual)
    } else {
        select_31([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_28([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, b, e, f], is_dual)
    } else {
        select_30([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_25([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_26([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_28([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_7([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_8([a, c, b, g, e, i, f, h], is_dual)
    } else {
        select_25([a, c, d, f, e, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_6([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_7([a, b, c, d, g, e, h, f, i], is_dual)
    } else {
        select_7([a, b, c, d, g, f, h, e, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_22([a, c, d, b, e, f], is_dual)
    } else {
        select_22([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_34([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_9([a, b, c, d, f, g], is_dual)
    } else {
        select_35([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([a, e, c, d, f, g, h], is_dual)
    } else {
        select_8([a, b, c, g, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_33([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_33([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_5([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_6([a, d, c, e, f, g, b, h, i], is_dual)
    } else {
        select_32([a, d, b, e, f, g, c, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_41([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_12([a, b, c, d], is_dual)
    } else {
        select_15([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_41([d, c, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_39([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_40([a, d, c, f, e, g], is_dual)
    } else {
        select_11([d, h, b, a, g], !is_dual)
    }
}
/// n = 9, i = 3
fn select_38([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_9([a, g, c, d, f, h], is_dual)
    } else {
        select_39([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_45([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_12([a, b, e, f], is_dual)
    } else {
        select_12([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([a, b, e, f, d, g], is_dual)
    } else {
        select_17([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_46([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_21([b, a, c, d, e, f, g], is_dual)
    } else {
        select_24([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_43([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_44([d, b, c, g, f, e, h], is_dual)
    } else {
        select_46([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_42([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_43([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_43([c, b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_38([b, c, d, a, f, e, h, g, i], is_dual)
    } else {
        select_42([a, b, c, d, f, g, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_37([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_37([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_4([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_5([a, b, e, d, f, c, g, h, i], is_dual)
    } else {
        select_36([a, b, e, c, f, d, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_3([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_4([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_4([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_54([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_11([d, f, e, a, b], !is_dual)
    } else {
        select_11([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_53([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_10([e, a, b, f, g], is_dual)
    } else {
        select_54([b, a, c, d, e, g], is_dual)
    }
}
/// n = 4, i = 0
fn select_57([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_18([a, b, c], is_dual)
    } else {
        select_18([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_56([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_57([a, b, c, d], is_dual)
    } else {
        select_14([e], is_dual)
    }
}
/// n = 6, i = 1
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_56([a, c, d, e, f], is_dual)
    } else {
        select_16([a, b, c, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_52([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_53([h, f, i, a, b, c, g], !is_dual)
    } else {
        select_55([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_59([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([a, b, c, f, e, g], is_dual)
    } else {
        select_35([a, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_58([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_59([a, b, c, e, d, f, g], is_dual)
    } else {
        select_59([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_51([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_52([b, a, c, d, e, f, h, i, g], is_dual)
    } else {
        select_58([b, f, d, e, a, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_63([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_18([c, d, e], is_dual)
    } else {
        select_18([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_62([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_63([a, b, c, d, e, f], is_dual)
    } else {
        select_17([e, c, d, a, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_61([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_9([a, e, b, c, g, h], is_dual)
    } else {
        select_62([c, d, a, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([a, e, c, f, d, h], is_dual)
    } else {
        select_31([b, d, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_66([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_66([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_67([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_30([b, c, d, a, e, f, h], is_dual)
    } else {
        select_30([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_64([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_65([b, e, a, d, g, f, i, h], is_dual)
    } else {
        select_67([a, b, f, c, h, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_60([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_61([a, c, b, f, h, g, i, e], is_dual)
    } else {
        select_64([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_51([b, c, d, a, e, f, h, g, i], is_dual)
    } else {
        select_60([b, c, d, e, f, g, h, a, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_49([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_50([a, d, b, e, c, h, g, i, f], is_dual)
    } else {
        select_50([a, d, c, e, b, h, f, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_49([a, b, d, e, f, c, g, h, i], is_dual)
    } else {
        select_49([a, c, d, e, f, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_48([a, b, c, d, f, g, e, h, i], is_dual)
    } else {
        select_48([a, b, c, e, f, g, d, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_2([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_3([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_47([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_75([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_63([a, f, c, d, e, g], is_dual)
    } else {
        select_63([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_74([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_75([a, c, d, e, b, f, g], is_dual)
    } else {
        select_75([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_73([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_28([a, c, g, d, b, f, h], is_dual)
    } else {
        select_74([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_79([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_18([a, b, c], is_dual)
    } else {
        select_14([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_78([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_57([b, c, d, e], is_dual)
    } else {
        select_79([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_78([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_78([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_76([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_77([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_9([a, b, g, h, c, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_73([a, b, d, c, e, f, g, h], is_dual)
    } else {
        select_76([a, b, c, d, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_71([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_72([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_72([a, b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_82([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_26([a, b, e, g, c, f, d], !is_dual)
    } else {
        select_9([a, b, d, c, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_83([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_9([a, b, c, d, e, f], is_dual)
    } else {
        select_35([a, b, c, g, f, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_81([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_82([a, b, i, g, h, e, c], !is_dual)
    } else {
        select_83([a, b, g, d, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_26([a, b, e, h, c, g, f], !is_dual)
    } else {
        select_59([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_86([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_59([a, e, b, d, g, f, h], is_dual)
    } else {
        select_28([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_84([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_85([a, e, f, d, b, g, h, i], is_dual)
    } else {
        select_86([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_80([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_81([a, b, d, e, c, g, f, h, i], is_dual)
    } else {
        select_84([a, b, c, e, f, d, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_70([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_71([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_80([a, b, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_69([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_70([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_70([a, b, d, e, f, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_68([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_69([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_69([a, b, c, d, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_2([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_68([a, b, c, d, e, f, h, i, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_0([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
