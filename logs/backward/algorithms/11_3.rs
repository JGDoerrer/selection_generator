/// n = 1, i = 0
fn select_17([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_16([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_17([a], is_dual)
    } else {
        select_17([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_15([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_16([a, d], is_dual)
    } else {
        select_16([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_18([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_16([a, b], is_dual)
    } else {
        select_17([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_14([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_15([b, c, e, d], is_dual)
    } else {
        select_18([a, b, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_19([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_18([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_14([a, b, c, e, d, f], is_dual)
    } else {
        select_19([d, c, a, e, f], is_dual)
    }
}
/// n = 3, i = 0
fn select_22([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_16([a, b], is_dual)
    } else {
        select_16([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_21([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_22([c, d, e], is_dual)
    } else {
        select_22([a, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_23([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([b, c, d], is_dual)
    } else {
        select_16([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_21([a, b, c, d, e, f], is_dual)
    } else {
        select_23([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_12([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_13([a, f, c, b, e, g], is_dual)
    } else {
        select_20([a, c, b, d, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_25([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_23([a, b, f, d, e], is_dual)
    } else {
        select_23([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_14([e, a, c, f, d, g], is_dual)
    } else {
        select_14([d, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_24([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_25([a, c, d, b, e, f], is_dual)
    } else {
        select_26([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_11([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_12([a, e, b, d, g, f, h], is_dual)
    } else {
        select_24([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_10([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_11([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_11([a, b, f, c, h, e, j, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_28([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_12([a, b, c, e, d, f, g], is_dual)
    } else {
        select_12([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_31([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_21([a, f, c, d, e, g], is_dual)
    } else {
        select_21([b, e, c, d, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_32([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_23([b, a, c, d, e], is_dual)
    } else {
        select_15([b, e, d, a], is_dual)
    }
}
/// n = 9, i = 2
fn select_30([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_31([b, f, c, d, e, g, h], is_dual)
    } else {
        select_32([e, a, h, f, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_35([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_22([b, c, e], is_dual)
    } else {
        select_15([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_34([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_35([a, b, c, e, f, g, h], is_dual)
    } else {
        select_22([b, c, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_37([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_22([a, b, c], is_dual)
    } else {
        select_17([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_36([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_35([a, b, c, d, e, f, g], is_dual)
    } else {
        select_37([e, b, c, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_33([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_34([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_36([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_29([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_30([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_33([c, b, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_27([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_28([b, g, c, d, a, i, h], is_dual)
    } else {
        select_29([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_9([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_10([b, c, d, e, f, g, a, h, i, j], is_dual)
    } else {
        select_27([c, b, a, d, e, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_8([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_9([a, b, d, c, e, g, h, i, f, j], is_dual)
    } else {
        select_9([a, c, d, b, e, f, h, i, g, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_40([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, d, e, g, h], is_dual)
    } else {
        select_28([a, b, c, d, f, g, h], is_dual)
    }
}
/// n = 4, i = 0
fn select_45([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_22([a, b, c], is_dual)
    } else {
        select_22([b, c, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_46([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_15([a, c, b, d], is_dual)
    } else {
        select_15([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 2
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_46([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_45([c, d, e, g], is_dual)
    } else {
        select_23([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_43([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_44([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_47([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_22([a, b, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_50([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_45([c, d, e, f], is_dual)
    } else {
        select_37([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([b, f, c, d, e, g, h], is_dual)
    } else {
        select_50([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_42([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_43([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_48([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_53([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, b, e], is_dual)
    } else {
        select_23([b, c, d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_55([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_15([a, b, c, d], is_dual)
    } else {
        select_18([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_54([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_55([b, c, a, d, e], is_dual)
    } else {
        select_15([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_53([a, c, b, d, e], is_dual)
    } else {
        select_54([a, c, e, b, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_56([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_26([b, c, d, a, e, f, h], is_dual)
    } else {
        select_26([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_52([b, e, c, a, g, f], is_dual)
    } else {
        select_56([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_41([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_42([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_51([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_39([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_40([a, c, e, f, b, g, i, h], is_dual)
    } else {
        select_41([a, c, b, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_38([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([a, b, d, c, e, f, h, i, g, j], is_dual)
    } else {
        select_39([a, c, d, b, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_7([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_8([a, c, d, e, f, g, h, i, b, j], is_dual)
    } else {
        select_38([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_6([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_7([a, b, c, e, f, g, d, h, i, j], is_dual)
    } else {
        select_7([a, b, d, e, f, g, c, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_5([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_6([a, b, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_6([a, b, c, d, f, g, h, e, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_4([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_5([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_5([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_60([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_8([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_8([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_59([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_60([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_60([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_69([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([b, c, d], is_dual)
    } else {
        select_18([a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_68([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_69([a, b, c, d, f, g], is_dual)
    } else {
        select_46([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_67([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_68([a, c, d, b, e, f, g], is_dual)
    } else {
        select_68([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_67([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_24([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_65([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_66([a, c, d, h, g, b, i, j], is_dual)
    } else {
        select_42([a, c, d, b, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_64([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_65([a, b, c, d, e, i, g, h, j, k], is_dual)
    } else {
        select_65([a, b, c, d, f, h, g, i, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_74([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_46([a, b, c, e], is_dual)
    } else {
        select_46([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_75([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, e], is_dual)
    } else {
        select_18([a, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_73([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_74([i, a, b, f, j], is_dual)
    } else {
        select_75([b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_77([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_14([b, d, c, e, g, f], is_dual)
    } else {
        select_55([d, h, a, b, f], !is_dual)
    }
}
/// n = 6, i = 2
fn select_79([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_15([a, b, e, f], is_dual)
    } else {
        select_15([a, c, d, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_78([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_14([a, e, c, f, d, h], is_dual)
    } else {
        select_79([b, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_76([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_77([b, a, c, e, d, f, h, g], is_dual)
    } else {
        select_78([b, c, a, e, d, g, h, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_72([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_73([b, c, a, d, e, f, h, g, i, j], is_dual)
    } else {
        select_76([c, b, g, a, f, i, j, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_82([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_35([a, c, d, e, g, f, i], is_dual)
    } else {
        select_46([a, h, b, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_81([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_82([f, a, c, d, e, g, h, j, i], is_dual)
    } else {
        select_82([e, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_84([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_14([f, c, g, e, d, a], !is_dual)
    } else {
        select_55([d, g, c, e, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_85([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_35([a, b, c, d, e, f, g], is_dual)
    } else {
        select_35([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_84([a, b, i, f, e, j, h], is_dual)
    } else {
        select_85([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_81([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_83([c, a, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_71([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_72([b, a, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_80([a, b, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_70([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_71([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_71([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_63([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_64([a, b, d, e, c, f, h, g, i, j, k], is_dual)
    } else {
        select_70([a, b, c, d, e, i, h, j, g, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_91([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_45([a, b, c, d], is_dual)
    } else {
        select_17([e], is_dual)
    }
}
/// n = 5, i = 2
fn select_92([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_37([a, b, c, d], is_dual)
    } else {
        select_37([a, b, c, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_90([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_91([a, c, d, e, f], is_dual)
    } else {
        select_92([a, b, f, e, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_94([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_46([a, b, c, d], is_dual)
    } else {
        select_18([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_93([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_13([a, b, c, d, e, f], is_dual)
    } else {
        select_94([a, d, c, f, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_90([a, c, b, d, e, f, g], is_dual)
    } else {
        select_93([a, c, f, e, b, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_96([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_75([f, c, d, e, a, g, h], is_dual)
    } else {
        select_50([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_98([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, e], is_dual)
    } else {
        select_16([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_99([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_45([d, e, f, g], is_dual)
    } else {
        select_45([a, b, c, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_97([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_98([g, d, e, f, a, h], is_dual)
    } else {
        select_99([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_96([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_97([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_89([a, b, h, d, c, g, i], is_dual)
    } else {
        select_95([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_101([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_24([a, c, d, b, f, e, g], is_dual)
    } else {
        select_52([a, f, b, c, g, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_100([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_42([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_101([a, c, d, h, g, b, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_88([a, b, c, d, e, f, g, h, j], is_dual)
    } else {
        select_100([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_105([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_31([b, c, a, d, e, f, g], is_dual)
    } else {
        select_26([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_104([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_105([a, c, b, d, f, e, g, h], is_dual)
    } else {
        select_52([a, f, b, g, h, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_108([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_14([a, b, c, f, e, g], is_dual)
    } else {
        select_14([a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_49([a, b, c, d, f, g, h], is_dual)
    } else {
        select_108([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_109([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_31([a, b, c, d, e, f, g], is_dual)
    } else {
        select_46([a, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_106([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_107([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_109([b, g, d, e, f, a, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_103([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_104([a, c, h, d, f, b, g, i], is_dual)
    } else {
        select_106([a, c, d, b, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_99([b, c, g, d, e, f, h, i], is_dual)
    } else {
        select_50([a, h, d, e, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_111([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_43([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_112([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_110([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_104([a, c, i, d, g, b, h, j], is_dual)
    } else {
        select_111([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_103([a, b, c, e, i, g, h, d, j], is_dual)
    } else {
        select_110([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_86([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_87([a, b, c, d, e, i, h, j, g, k], is_dual)
    } else {
        select_102([a, b, d, e, c, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_62([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_63([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    } else {
        select_86([a, b, d, c, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_61([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_62([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    } else {
        select_62([a, b, c, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_58([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_59([a, b, c, d, e, g, i, j, f, k], is_dual)
    } else {
        select_61([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_121([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_55([e, f, a, b, c], !is_dual)
    } else {
        select_37([b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_120([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_53([b, c, d, e, f], is_dual)
    } else {
        select_121([a, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_119([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_120([c, a, b, d, g, i, h], is_dual)
    } else {
        select_91([c, e, f, h, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_124([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_14([a, d, c, e, f, g], is_dual)
    } else {
        select_46([a, b, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_123([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_124([d, e, a, g, f, h, i], is_dual)
    } else {
        select_124([b, c, a, h, f, g, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([c, d, f, e, h], is_dual)
    } else {
        select_46([a, b, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_127([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_35([a, c, d, b, e, f, g], is_dual)
    } else {
        select_35([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_126([a, b, c, f, e, g, h, i, j], is_dual)
    } else {
        select_127([a, b, d, h, g, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_122([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_123([e, a, b, c, d, g, f, h, i, j], is_dual)
    } else {
        select_125([c, d, b, e, f, g, h, a, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_118([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_119([d, e, c, f, a, g, i, j, h], is_dual)
    } else {
        select_122([a, b, d, e, c, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_117([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_118([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_118([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_132([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_14([a, d, b, c, e, f], is_dual)
    } else {
        select_14([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_131([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_53([b, c, d, e, f], is_dual)
    } else {
        select_132([a, b, c, f, e, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_134([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_14([a, b, c, e, d, f], is_dual)
    } else {
        select_15([c, d, e, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_133([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_84([g, i, a, h, e, b, c], !is_dual)
    } else {
        select_134([a, b, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_131([a, b, c, e, h, g, i], is_dual)
    } else {
        select_133([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_138([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_15([b, c, d, e], is_dual)
    } else {
        select_18([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_137([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_138([a, c, d, e, f, g, h], is_dual)
    } else {
        select_46([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_136([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_77([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_137([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_131([b, c, d, e, h, g, i], is_dual)
    } else {
        select_136([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_129([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_130([a, c, d, b, e, g, h, f, i, j], is_dual)
    } else {
        select_135([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_128([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_129([a, c, d, e, b, f, h, g, i, j], is_dual)
    } else {
        select_71([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_116([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_117([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_128([b, c, i, d, e, g, h, a, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_115([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_116([a, b, d, e, f, g, c, h, i, j, k], is_dual)
    } else {
        select_116([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_114([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < j) || (is_dual && d > j) {
        select_60([a, b, c, e, i, f, h, j, d, k], is_dual)
    } else {
        select_115([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_113([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_114([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_114([a, b, c, d, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_58([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_113([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_3([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_4([a, b, c, d, e, f, i, j, g, k], is_dual)
    } else {
        select_57([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_14([a, b, d, f, e, h], is_dual)
    } else {
        select_35([c, a, b, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_146([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_34([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_147([a, h, b, f, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_145([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_131([a, b, c, g, e, i, h], is_dual)
    } else {
        select_146([a, d, b, c, e, f, g, h, j, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_149([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_13([a, b, c, d, e, f], is_dual)
    } else {
        select_18([a, f, b], is_dual)
    }
}
/// n = 8, i = 3
fn select_150([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_13([a, d, c, f, e, g], is_dual)
    } else {
        select_54([a, b, g, h, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_149([i, a, b, d, e, g], is_dual)
    } else {
        select_150([g, a, c, d, f, j, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_145([b, a, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_148([b, c, d, e, f, g, a, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_154([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_69([a, c, d, b, e, f], is_dual)
    } else {
        select_69([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_153([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_53([a, f, d, g, b], !is_dual)
    } else {
        select_154([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_153([a, b, f, g, h, i, e], is_dual)
    } else {
        select_24([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_151([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_66([a, b, c, h, e, j, f, i], is_dual)
    } else {
        select_152([a, b, f, d, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_144([a, b, d, e, c, f, g, h, j, i], is_dual)
    } else {
        select_151([a, c, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_142([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_143([a, b, c, d, e, g, h, i, f, j], is_dual)
    } else {
        select_143([a, b, d, c, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_141([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_142([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_142([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_140([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_141([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_141([a, b, c, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_139([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_58([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_140([a, b, c, d, e, f, i, j, k, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_2([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_3([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_139([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_165([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_53([a, b, c, d, f], is_dual)
    } else {
        select_53([a, b, d, e, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_166([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_91([a, c, d, e, g], is_dual)
    } else {
        select_20([a, b, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_164([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_165([a, b, d, e, f, h], is_dual)
    } else {
        select_166([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_168([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_74([f, a, b, d, e], is_dual)
    } else {
        select_74([e, a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_167([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_93([a, b, c, e, f, g], is_dual)
    } else {
        select_168([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_163([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_164([c, b, a, d, e, f, h, g], is_dual)
    } else {
        select_167([h, f, i, b, a, c, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_172([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_14([a, c, d, f, e, h], is_dual)
    } else {
        select_14([b, d, c, e, f, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_173([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_69([a, b, c, d, f, g], is_dual)
    } else {
        select_46([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_171([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_172([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_173([e, b, d, a, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_174([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_68([a, b, f, d, h, e, g], is_dual)
    } else {
        select_68([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_170([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_171([e, b, c, d, f, g, h, i], is_dual)
    } else {
        select_174([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_178([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_15([a, c, d, g], is_dual)
    } else {
        select_15([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_177([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_178([a, d, b, g, e, f, h], is_dual)
    } else {
        select_178([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_176([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_25([c, d, e, g, f, a], is_dual)
    } else {
        select_177([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_180([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_79([d, f, g, a, e, b], !is_dual)
    } else {
        select_55([e, f, d, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_179([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_53([a, e, c, d, f], is_dual)
    } else {
        select_180([g, d, h, a, e, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_175([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_176([b, c, d, a, f, e, g, h], is_dual)
    } else {
        select_179([a, c, b, g, f, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_169([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_170([a, d, c, e, f, b, h, g, i], is_dual)
    } else {
        select_175([a, d, b, e, f, c, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_162([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_163([a, c, b, e, h, f, g, j, i], is_dual)
    } else {
        select_169([a, b, c, d, g, f, i, h, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_35([b, c, d, e, g, f, h], is_dual)
    } else {
        select_46([a, h, b, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_184([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_185([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_185([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_187([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_14([a, f, b, e, d, g], is_dual)
    } else {
        select_23([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_186([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_187([a, b, c, d, e, f, g], is_dual)
    } else {
        select_134([a, b, f, d, g, e], is_dual)
    }
}
/// n = 8, i = 2
fn select_183([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_184([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_186([a, b, g, e, f, c, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_182([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_183([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_183([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 7, i = 1
fn select_190([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_31([a, c, d, e, b, f, g], is_dual)
    } else {
        select_31([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_191([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_20([a, b, d, e, c, f], is_dual)
    } else {
        select_20([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_189([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_190([a, d, e, b, c, f, g], is_dual)
    } else {
        select_191([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_189([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_189([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_181([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_182([b, c, a, d, e, f, g, i], is_dual)
    } else {
        select_188([d, a, e, b, c, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_161([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_162([a, d, c, b, e, f, h, g, j, i], is_dual)
    } else {
        select_181([a, d, h, b, c, f, i, j, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_196([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_31([b, c, d, e, g, f, h], is_dual)
    } else {
        select_14([a, b, c, f, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_198([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_21([a, b, c, d, e, f], is_dual)
    } else {
        select_46([a, f, b, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_197([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_50([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_198([a, c, b, h, g, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_195([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_196([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_197([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_200([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_180([a, b, c, d, f, g, e], is_dual)
    } else {
        select_180([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_165([b, c, d, e, f, g], is_dual)
    } else {
        select_200([f, h, i, a, b, c, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_194([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_195([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_199([a, b, c, d, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_193([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_194([a, b, c, e, f, g, h, j, i, k], is_dual)
    } else {
        select_169([a, c, b, d, i, h, k, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_192([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_193([c, a, d, b, e, f, h, i, j, g, k], is_dual)
    } else {
        select_193([c, b, d, a, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_161([a, b, e, c, i, g, h, d, k, j], is_dual)
    } else {
        select_192([a, b, c, e, f, d, g, h, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_159([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_160([c, d, a, b, e, f, g, h, j, k, i], is_dual)
    } else {
        select_160([c, d, a, b, f, e, g, h, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_158([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_159([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    } else {
        select_159([a, b, c, d, e, g, h, i, j, f, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_157([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_158([g, h, a, e, f, b, c, d, i, j, k], is_dual)
    } else {
        select_158([g, h, d, e, f, b, c, a, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_156([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([a, b, c, d, e, g, h, i, f, j, k], is_dual)
    } else {
        select_157([a, b, c, d, f, g, h, i, e, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_155([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_156([a, b, c, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_156([a, b, c, d, e, f, h, i, j, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_2([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_155([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_0([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, k, j], is_dual)
    }
}
