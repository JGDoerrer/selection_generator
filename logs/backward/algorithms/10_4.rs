/// n = 1, i = 0
fn select_16([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_15([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_16([a], is_dual)
    } else {
        select_16([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_14([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_15([a, d], is_dual)
    } else {
        select_15([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_17([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_15([a, b], is_dual)
    } else {
        select_16([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_14([b, c, e, d], is_dual)
    } else {
        select_17([a, b, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_12([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_13([a, b, c, e, d, f], is_dual)
    } else {
        select_14([c, d, e, b], is_dual)
    }
}
/// n = 3, i = 0
fn select_20([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_15([a, b], is_dual)
    } else {
        select_15([b, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_19([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_20([b, c, d], is_dual)
    } else {
        select_15([a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_18([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_13([a, f, b, e, d, g], is_dual)
    } else {
        select_19([b, c, d, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_11([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_12([a, b, f, d, g, e], is_dual)
    } else {
        select_18([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 5, i = 2
fn select_23([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_14([a, b, c, d], is_dual)
    } else {
        select_17([d, a, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_13([b, d, c, e, g, f], is_dual)
    } else {
        select_23([d, h, a, b, f], !is_dual)
    }
}
/// n = 7, i = 2
fn select_25([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_14([a, c, d, g], is_dual)
    } else {
        select_14([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_24([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_23([b, f, d, a, h], is_dual)
    } else {
        select_25([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_22([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_24([a, b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_10([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_11([a, b, d, c, e, f, g], is_dual)
    } else {
        select_21([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 4, i = 1
fn select_28([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_14([a, c, b, d], is_dual)
    } else {
        select_14([b, c, a, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_27([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_28([a, b, c, d], is_dual)
    } else {
        select_17([a, d, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_29([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_19([b, a, c, d, e], is_dual)
    } else {
        select_14([b, e, d, a], is_dual)
    }
}
/// n = 7, i = 3
fn select_26([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_27([a, d, c, e, f], is_dual)
    } else {
        select_29([a, d, g, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_9([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_10([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_26([a, c, f, e, g, h, b], is_dual)
    }
}
/// n = 3, i = 1
fn select_34([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, b, c], is_dual)
    } else {
        select_17([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_33([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_34([a, d, e], is_dual)
    } else {
        select_14([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_35([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_13([a, d, b, c, e, f], is_dual)
    } else {
        select_13([a, d, c, b, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_32([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_33([a, f, e, b, c], !is_dual)
    } else {
        select_35([a, b, c, d, e, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_38([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_20([c, d, e], is_dual)
    } else {
        select_20([a, b, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_37([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_38([a, b, c, d, e, f], is_dual)
    } else {
        select_19([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_36([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_37([a, b, d, e, c, f], is_dual)
    } else {
        select_37([a, c, d, e, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_31([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_32([a, c, d, f, b, g], is_dual)
    } else {
        select_36([a, c, d, b, e, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_41([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([b, c, a, d, e], is_dual)
    } else {
        select_14([a, c, b, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_42([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_19([a, c, d, b, e], is_dual)
    } else {
        select_19([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_40([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_41([a, c, e, b, f], is_dual)
    } else {
        select_42([a, c, b, d, e], is_dual)
    }
}
/// n = 4, i = 0
fn select_45([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_20([a, b, c], is_dual)
    } else {
        select_20([b, c, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_46([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_20([a, b, c], is_dual)
    } else {
        select_16([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_44([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([b, c, d, e], is_dual)
    } else {
        select_46([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_43([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_44([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_44([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_39([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_40([a, b, g, h, c, f], !is_dual)
    } else {
        select_43([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_31([a, b, d, e, c, f, g], is_dual)
    } else {
        select_39([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_8([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_9([a, b, c, e, f, g, d, h], is_dual)
    } else {
        select_30([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_52([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_14([a, d, c, f], is_dual)
    } else {
        select_14([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_51([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_52([a, b, c, e, d, f], is_dual)
    } else {
        select_52([a, b, d, e, c, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_50([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_51([f, h, a, b, g, c], !is_dual)
    } else {
        select_12([a, c, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_49([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_50([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_50([a, b, d, c, e, g, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_20([b, c, d], is_dual)
    } else {
        select_17([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_54([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_33([b, e, d, a, f], is_dual)
    } else {
        select_55([b, a, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_56([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_51([b, a, d, f, g, h], is_dual)
    } else {
        select_29([a, c, d, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_53([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_54([g, d, i, a, f, c], !is_dual)
    } else {
        select_56([g, h, i, d, a, f, e, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_48([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_49([a, g, f, i, d, h, e, b], !is_dual)
    } else {
        select_53([f, g, i, d, e, h, a, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_60([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_13([f, c, g, e, d, a], !is_dual)
    } else {
        select_23([d, g, c, e, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_59([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_51([g, d, a, e, f, b], !is_dual)
    } else {
        select_60([g, f, a, d, e, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_62([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_55([a, b, c, d, e, f], is_dual)
    } else {
        select_55([a, b, c, d, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_63([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_55([b, c, d, e, f, g], is_dual)
    } else {
        select_28([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_62([a, c, e, d, f, h, i], is_dual)
    } else {
        select_63([d, g, h, i, a, e, b, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_59([b, a, f, d, g, h, e], is_dual)
    } else {
        select_61([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_66([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_23([a, d, g, f, c], is_dual)
    } else {
        select_23([b, c, g, e, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_65([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_33([g, f, b, c, d], !is_dual)
    } else {
        select_66([g, h, e, f, a, d, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_67([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_33([g, f, a, b, c], !is_dual)
    } else {
        select_60([g, h, a, f, e, b, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_64([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_65([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_67([a, c, d, b, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_57([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_58([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_64([c, b, a, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_47([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_48([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_57([a, e, g, i, b, f, h, c, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_7([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_8([a, b, h, i, e, f, c, g], !is_dual)
    } else {
        select_47([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_6([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_7([a, b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_7([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 5, i = 2
fn select_75([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_14([a, b, c, d], is_dual)
    } else {
        select_17([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_74([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_13([a, b, c, e, d, f], is_dual)
    } else {
        select_75([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_73([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_41([a, b, e, f, g], is_dual)
    } else {
        select_74([a, b, c, d, e, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_77([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_55([a, b, c, d, f, g], is_dual)
    } else {
        select_28([a, e, g, f], !is_dual)
    }
}
/// n = 7, i = 3
fn select_76([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_77([a, c, d, b, e, f, g], is_dual)
    } else {
        select_77([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_72([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_73([a, b, f, d, g, h, e], is_dual)
    } else {
        select_76([a, d, g, i, e, f, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_81([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_14([a, b, e, f], is_dual)
    } else {
        select_14([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_80([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_81([d, f, g, a, e, b], !is_dual)
    } else {
        select_23([e, f, d, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_79([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_80([a, b, c, d, f, g, e], is_dual)
    } else {
        select_80([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_78([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_73([d, f, c, h, e, i, g], is_dual)
    } else {
        select_79([a, b, g, d, i, j, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_71([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_72([d, e, c, g, a, f, h, j, i], is_dual)
    } else {
        select_78([b, a, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_70([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_71([c, a, d, e, b, g, f, h, i, j], is_dual)
    } else {
        select_71([c, b, d, f, a, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_69([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_70([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_70([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_68([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_69([a, b, c, e, f, d, g, h, i, j], is_dual)
    } else {
        select_69([a, b, d, e, f, c, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_6([a, b, c, d, g, h, i, e, j], is_dual)
    } else {
        select_68([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_5([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_5([a, b, c, d, e, f, h, j, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_89([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_74([a, d, g, e, f, b], !is_dual)
    } else {
        select_74([a, e, g, d, f, c], !is_dual)
    }
}
/// n = 7, i = 1
fn select_91([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_38([a, f, c, d, e, g], is_dual)
    } else {
        select_38([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_92([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_13([e, a, c, f, d, g], is_dual)
    } else {
        select_13([d, b, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_90([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_91([b, c, a, d, e, f, g], is_dual)
    } else {
        select_92([b, g, c, e, a, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_88([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_89([f, e, i, a, b, h, g], !is_dual)
    } else {
        select_90([b, c, a, d, f, e, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_93([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_66([e, g, d, f, a, c, b], !is_dual)
    } else {
        select_60([d, g, a, f, e, c, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_87([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_88([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_93([e, i, f, a, c, b, h], !is_dual)
    }
}
/// n = 9, i = 3
fn select_86([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_87([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_87([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_96([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_11([a, b, c, d, e, f, g], is_dual)
    } else {
        select_89([a, b, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_95([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_96([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_64([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_99([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_35([f, a, b, c, e, g], is_dual)
    } else {
        select_35([e, a, b, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_100([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_91([a, c, d, e, b, f, g], is_dual)
    } else {
        select_91([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_98([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_99([b, c, d, g, f, a, h], is_dual)
    } else {
        select_100([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_104([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_20([b, c, e], is_dual)
    } else {
        select_14([a, f, d, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_103([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_104([a, b, c, d, e, f, g], is_dual)
    } else {
        select_55([d, b, c, e, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_102([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_103([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_63([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_101([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_102([a, e, h, i, f, g, b, c], !is_dual)
    } else {
        select_102([a, f, h, i, e, g, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_97([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_98([a, c, d, e, b, g, f, h], is_dual)
    } else {
        select_101([a, i, f, j, b, g, h, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_94([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_95([a, f, j, i, b, g, c, h, d], !is_dual)
    } else {
        select_97([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_85([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_86([a, c, b, d, e, f, g, h, i], is_dual)
    } else {
        select_94([a, b, c, d, e, f, g, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_85([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_85([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_111([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_81([a, b, e, f, d, g], is_dual)
    } else {
        select_19([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_112([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_104([b, a, c, d, e, f, g], is_dual)
    } else {
        select_28([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_110([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_111([d, b, c, g, f, e, h], is_dual)
    } else {
        select_112([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_109([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_110([c, a, d, e, b, f, g, h, i], is_dual)
    } else {
        select_110([c, b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_114([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_51([h, f, a, g, d, b], !is_dual)
    } else {
        select_27([a, f, c, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_40([a, g, c, d, f, h], is_dual)
    } else {
        select_114([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_108([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_109([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_113([b, c, d, a, f, e, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_108([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_108([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_119([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_19([a, b, f, d, e], is_dual)
    } else {
        select_19([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_119([a, c, d, b, e, f], is_dual)
    } else {
        select_92([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_76([a, f, e, h, b, g, c], !is_dual)
    } else {
        select_118([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_12([a, c, d, e, f, g], is_dual)
    } else {
        select_63([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_123([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_23([b, c, f, e, h], is_dual)
    } else {
        select_75([a, b, d, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_13([a, e, c, f, d, h], is_dual)
    } else {
        select_81([b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_122([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_123([h, i, f, g, e, a, d, c, b], !is_dual)
    } else {
        select_124([h, e, i, g, f, d, a, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_121([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_122([c, b, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_116([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_117([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_120([a, c, b, g, f, i, e, h, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_128([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_13([a, c, b, d, e, f], is_dual)
    } else {
        select_75([a, b, e, f, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_13([g, h, i, d, a, b], !is_dual)
    } else {
        select_75([a, c, f, e, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_127([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_128([h, d, i, f, b, a], !is_dual)
    } else {
        select_129([b, a, c, d, e, g, h, i, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_65([g, h, a, i, e, f, b, c], !is_dual)
    } else {
        select_127([g, h, i, e, f, a, b, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_126([a, c, b, g, f, e, j, i, h], is_dual)
    } else {
        select_117([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_115([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_116([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_125([a, d, c, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_107([a, c, d, e, b, g, f, i, h], is_dual)
    } else {
        select_115([a, b, c, e, d, g, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_106([a, b, d, e, f, c, g, h, i, j], is_dual)
    } else {
        select_106([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_83([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_84([a, b, c, d, e, f, h, j, i, g], is_dual)
    } else {
        select_105([a, b, c, e, d, f, h, g, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_82([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_83([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_83([a, b, c, e, f, g, d, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_4([a, b, c, d, e, g, h, i, f, j], is_dual)
    } else {
        select_82([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_3([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_3([a, b, c, d, e, f, h, j, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_2([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_2([a, b, c, d, e, f, h, i, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
