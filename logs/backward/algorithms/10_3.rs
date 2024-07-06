/// n = 1, i = 0
fn select_15([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_14([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_15([a], is_dual)
    } else {
        select_15([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_13([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_14([a, b], is_dual)
    } else {
        select_14([b, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_12([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_13([b, c, d], is_dual)
    } else {
        select_14([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_11([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_12([a, b, f, d, e], is_dual)
    } else {
        select_12([a, c, e, d, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_18([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_14([a, d], is_dual)
    } else {
        select_14([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_19([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_14([a, b], is_dual)
    } else {
        select_15([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_17([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_18([b, c, e, d], is_dual)
    } else {
        select_19([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_16([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_17([e, a, c, f, d, g], is_dual)
    } else {
        select_17([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_10([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_11([a, c, d, b, e, f], is_dual)
    } else {
        select_16([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_22([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_18([a, b, c, d], is_dual)
    } else {
        select_19([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_21([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_17([a, b, c, e, d, f], is_dual)
    } else {
        select_22([d, c, a, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([c, d, e], is_dual)
    } else {
        select_13([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_24([a, b, c, d, e, f], is_dual)
    } else {
        select_12([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_20([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_21([a, f, c, b, e, g], is_dual)
    } else {
        select_23([a, c, b, d, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_9([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_10([a, b, c, f, e, h, g], is_dual)
    } else {
        select_20([a, e, b, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_9([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_9([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_27([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_11([a, c, d, b, e, f], is_dual)
    } else {
        select_11([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_10([a, c, d, e, b, f, g], is_dual)
    } else {
        select_27([a, c, b, e, d, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_31([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_18([a, b, e, f], is_dual)
    } else {
        select_18([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([a, b, e, f, d, g], is_dual)
    } else {
        select_12([b, c, d, f, e], is_dual)
    }
}
/// n = 4, i = 0
fn select_33([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_13([a, b, c], is_dual)
    } else {
        select_13([b, c, d], is_dual)
    }
}
/// n = 6, i = 1
fn select_32([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_33([b, c, d, e], is_dual)
    } else {
        select_14([a, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_29([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_30([b, h, a, e, f, i, g], is_dual)
    } else {
        select_32([b, c, d, g, e, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([b, c, e], is_dual)
    } else {
        select_18([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_36([a, b, c, e, f, g, h], is_dual)
    } else {
        select_13([b, c, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_38([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_13([a, b, c], is_dual)
    } else {
        select_15([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_37([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_36([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([e, b, c, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_34([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_35([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_37([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_28([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_34([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_25([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_26([b, g, c, d, a, i, h], is_dual)
    } else {
        select_28([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_8([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_25([c, b, a, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_7([a, b, d, c, e, g, h, i, f, j], is_dual)
    } else {
        select_7([a, c, d, b, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_6([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_6([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_44([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_21([a, d, g, e, f, b], !is_dual)
    } else {
        select_21([a, e, g, d, f, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_10([b, c, d, a, f, e, g], is_dual)
    } else {
        select_44([f, e, h, a, b, g, c], !is_dual)
    }
}
/// n = 9, i = 2
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_33([c, d, e, g], is_dual)
    } else {
        select_12([a, b, h, f, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_49([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_18([a, c, b, d], is_dual)
    } else {
        select_18([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_48([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([c, d, e, f], is_dual)
    } else {
        select_49([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_46([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_47([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_48([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_51([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([c, d, e, f], is_dual)
    } else {
        select_13([a, b, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([c, d, e, f], is_dual)
    } else {
        select_38([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_50([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_51([b, f, c, d, e, g, h], is_dual)
    } else {
        select_52([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_45([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_46([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_50([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_42([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_43([b, c, d, h, g, a, i, j], is_dual)
    } else {
        select_45([c, b, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_42([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_42([a, b, d, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_57([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, f, c, d, e, g], is_dual)
    } else {
        select_24([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_57([a, c, d, e, b, f, g], is_dual)
    } else {
        select_57([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_55([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_10([a, c, g, d, b, f, h], is_dual)
    } else {
        select_56([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_61([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_13([b, c, d], is_dual)
    } else {
        select_19([a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_61([a, c, d, e, f, h], is_dual)
    } else {
        select_61([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_59([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_60([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_61([e, c, d, a, f, h], is_dual)
    }
}
/// n = 5, i = 1
fn select_63([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, d, b, e], is_dual)
    } else {
        select_12([b, c, d, a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_62([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_63([b, e, c, d, f], is_dual)
    } else {
        select_61([a, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_58([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_59([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_62([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_54([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_55([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_58([a, b, c, d, g, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_53([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_42([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_54([a, b, c, d, h, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_40([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_41([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_53([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_39([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_40([a, b, d, e, f, g, c, h, i, j], is_dual)
    } else {
        select_40([a, c, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_5([a, b, c, e, f, g, h, i, d, j], is_dual)
    } else {
        select_39([a, b, c, e, f, d, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_4([a, b, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_4([a, b, c, d, f, g, h, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_3([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_3([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_2([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_2([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
