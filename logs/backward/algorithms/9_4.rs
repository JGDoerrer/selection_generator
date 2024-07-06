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
/// n = 4, i = 1
fn select_17([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, b, d], is_dual)
    } else {
        select_12([b, c, a, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_16([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_17([a, b, c, e], is_dual)
    } else {
        select_17([a, c, b, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_9([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_10([a, b, d, f, e], is_dual)
    } else {
        select_16([a, b, c, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_20([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_12([a, b, e, f], is_dual)
    } else {
        select_12([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_19([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_20([d, f, g, a, e, b], !is_dual)
    } else {
        select_11([e, f, d, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_18([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_19([a, b, c, d, f, g, e], is_dual)
    } else {
        select_19([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_8([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_9([d, g, i, e, a, h], !is_dual)
    } else {
        select_18([b, c, e, d, g, f, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_24([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_15([a, b, c], is_dual)
    } else {
        select_15([a, c, b], is_dual)
    }
}
/// n = 3, i = 0
fn select_26([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_13([a, b], is_dual)
    } else {
        select_13([b, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_25([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_26([b, c, d], is_dual)
    } else {
        select_13([a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_23([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_24([a, e, f], is_dual)
    } else {
        select_25([a, b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_26([b, c, d], is_dual)
    } else {
        select_15([a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, b, c, d, f, g], is_dual)
    } else {
        select_17([b, d, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_23([a, c, e, d, f, h], is_dual)
    } else {
        select_27([a, b, f, d, g, e, h], is_dual)
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
/// n = 7, i = 3
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([f, g, e, c, d, a], !is_dual)
    } else {
        select_15([e, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_29([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_23([b, c, d, e, f, g], is_dual)
    } else {
        select_30([h, g, e, d, b, a, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_22([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_29([c, a, d, e, b, f, h, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_7([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_8([c, b, a, d, f, i, g, e, h], is_dual)
    } else {
        select_21([d, h, i, g, f, a, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_6([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_7([g, e, i, a, f, h, c, b, d], !is_dual)
    } else {
        select_7([g, f, i, b, e, h, c, a, d], !is_dual)
    }
}
/// n = 6, i = 2
fn select_36([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_28([a, c, d, b, e, f], is_dual)
    } else {
        select_28([b, c, d, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_37([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_31([e, a, c, f, d, g], is_dual)
    } else {
        select_31([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_35([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_36([e, f, d, h, g, a], !is_dual)
    } else {
        select_37([b, c, a, e, f, d, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([a, b, f, d, e], is_dual)
    } else {
        select_25([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_38([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_39([a, c, d, b, e, f], is_dual)
    } else {
        select_37([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_34([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_35([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_38([e, h, i, d, f, a, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_41([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_31([b, d, c, e, g, f], is_dual)
    } else {
        select_11([d, h, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_43([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_12([a, c, d, g], is_dual)
    } else {
        select_12([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_42([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_11([b, f, d, a, h], is_dual)
    } else {
        select_43([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_40([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_41([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_42([a, b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_34([a, b, c, d, e, f, h, i, g], is_dual)
    } else {
        select_40([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_47([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_31([a, d, b, c, e, f], is_dual)
    } else {
        select_31([a, d, c, b, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_48([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_25([a, c, d, b, e], is_dual)
    } else {
        select_25([b, c, d, a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_46([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_47([a, b, c, f, e, g], is_dual)
    } else {
        select_48([b, c, d, e, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_50([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_25([b, a, c, d, e], is_dual)
    } else {
        select_12([b, e, d, a], is_dual)
    }
}
/// n = 8, i = 3
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_36([b, e, c, d, f, g], is_dual)
    } else {
        select_50([g, e, h, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_45([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_46([a, c, d, b, e, f, g], is_dual)
    } else {
        select_49([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_53([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_24([a, d, e], is_dual)
    } else {
        select_12([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_53([b, e, d, a, f], is_dual)
    } else {
        select_28([b, a, c, d, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_31([a, b, c, e, d, f], is_dual)
    } else {
        select_12([c, d, e, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_31([a, f, b, e, d, g], is_dual)
    } else {
        select_25([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_54([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_55([a, b, f, d, g, e], is_dual)
    } else {
        select_56([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_51([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_52([a, b, f, e, g, h], is_dual)
    } else {
        select_54([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_45([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_51([a, b, d, e, f, g, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_32([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_33([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_44([d, h, i, e, a, f, b, g], !is_dual)
    }
}
/// n = 9, i = 4
fn select_5([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_6([e, f, g, i, b, c, a, h, d], !is_dual)
    } else {
        select_32([b, c, d, e, f, g, h, a, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_4([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_5([e, a, b, c, f, g, h, d, i], is_dual)
    } else {
        select_5([e, a, b, d, f, g, i, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_3([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_4([a, b, c, d, e, g, h, i, f], is_dual)
    } else {
        select_4([a, b, c, f, e, g, h, i, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_2([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_3([a, f, g, b, c, d, e, h, i], is_dual)
    } else {
        select_3([e, f, g, b, c, d, a, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_2([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_2([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_0([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, i, h], is_dual)
    }
}
