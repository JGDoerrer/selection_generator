/// n = 1, i = 0
fn select_23([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_22([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([a], is_dual)
    } else {
        select_23([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_21([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_22([a, b], is_dual)
    } else {
        select_22([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_24([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_22([a, d], is_dual)
    } else {
        select_22([b, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_20([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([b, c, e], is_dual)
    } else {
        select_24([a, f, d, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_25([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_24([a, c, b, d], is_dual)
    } else {
        select_24([b, c, a, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_19([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_20([b, a, c, d, e, f, g], is_dual)
    } else {
        select_25([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_18([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_19([e, a, b, f, g, d, h, i], is_dual)
    } else {
        select_19([d, a, c, f, g, e, h, i], is_dual)
    }
}
/// n = 3, i = 1
fn select_29([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_22([a, b], is_dual)
    } else {
        select_23([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([b, c, e, d], is_dual)
    } else {
        select_29([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_27([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([e, a, c, f, d, g], is_dual)
    } else {
        select_28([d, b, c, f, e, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_31([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_21([b, c, d], is_dual)
    } else {
        select_22([a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_30([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_28([a, f, b, e, d, g], is_dual)
    } else {
        select_31([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_26([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_27([a, f, b, e, i, g, j], is_dual)
    } else {
        select_30([e, c, d, h, f, g, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_17([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_18([d, b, f, e, a, g, i, h, j], is_dual)
    } else {
        select_26([b, a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_35([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_29([d, a, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_36([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, c, d, g], is_dual)
    } else {
        select_24([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_34([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_35([b, f, d, a, h], is_dual)
    } else {
        select_36([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_33([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_34([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_34([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, b, e, f], is_dual)
    } else {
        select_24([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_38([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([a, b, e, f, d, g], is_dual)
    } else {
        select_31([b, c, d, f, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_41([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_21([b, c, d], is_dual)
    } else {
        select_29([a, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_40([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_28([a, c, b, d, e, f], is_dual)
    } else {
        select_41([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_37([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_38([b, d, c, f, e, g, h], is_dual)
    } else {
        select_40([a, d, b, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_32([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_33([a, d, b, f, g, e, j, i, h], is_dual)
    } else {
        select_37([a, e, d, c, g, h, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_16([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_17([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_32([a, d, c, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_45([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_36([a, d, b, g, e, f, h], is_dual)
    } else {
        select_36([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_46([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_25([a, g, c, i], is_dual)
    } else {
        select_36([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_44([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_45([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_46([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_43([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_44([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_44([c, b, d, e, f, a, g, h, j, i], is_dual)
    }
}
/// n = 4, i = 0
fn select_49([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_21([a, b, c], is_dual)
    } else {
        select_21([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_48([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_49([a, b, c, d], is_dual)
    } else {
        select_23([e], is_dual)
    }
}
/// n = 7, i = 2
fn select_47([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([b, c, d, e, f], is_dual)
    } else {
        select_35([e, g, a, b, f], !is_dual)
    }
}
/// n = 11, i = 3
fn select_42([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_43([a, b, c, h, d, f, g, j, i, k], is_dual)
    } else {
        select_47([c, b, e, i, f, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_15([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_16([a, b, c, g, e, i, h, f, j, k], is_dual)
    } else {
        select_42([b, c, a, e, d, f, h, g, j, i, k], is_dual)
    }
}
/// n = 6, i = 1
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_21([c, d, e], is_dual)
    } else {
        select_21([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_54([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_55([a, f, c, d, e, g], is_dual)
    } else {
        select_55([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_53([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_54([a, c, d, e, b, f, g], is_dual)
    } else {
        select_54([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_57([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_55([a, b, c, d, e, f], is_dual)
    } else {
        select_31([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_56([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_48([a, c, d, e, g], is_dual)
    } else {
        select_57([a, b, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_52([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_53([a, b, c, d, e, g, h], is_dual)
    } else {
        select_56([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_28([d, b, c, e, f, h], is_dual)
    } else {
        select_36([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_59([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_60([e, b, d, a, f, g, h, i], is_dual)
    } else {
        select_45([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_58([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_59([a, d, b, e, c, f, g, h, i], is_dual)
    } else {
        select_59([a, d, c, e, b, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_51([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_52([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_58([a, b, c, i, d, h, g, j, k], is_dual)
    }
}
/// n = 5, i = 1
fn select_64([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_31([a, c, d, b, e], is_dual)
    } else {
        select_31([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_65([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([a, d, b, c, e, f], is_dual)
    } else {
        select_28([a, d, c, b, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_63([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_64([b, c, d, e, f], is_dual)
    } else {
        select_65([a, b, c, f, e, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_66([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([b, c, d, e], is_dual)
    } else {
        select_29([a, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_62([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_63([a, b, c, d, e, f, g], is_dual)
    } else {
        select_66([a, b, c, d, e, g, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_69([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([c, d, e, f], is_dual)
    } else {
        select_21([a, b, g], is_dual)
    }
}
/// n = 4, i = 1
fn select_71([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_21([a, b, c], is_dual)
    } else {
        select_23([d], is_dual)
    }
}
/// n = 8, i = 2
fn select_70([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([c, d, e, f], is_dual)
    } else {
        select_71([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_68([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_69([b, f, c, d, e, g, h], is_dual)
    } else {
        select_70([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_67([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_68([a, b, d, e, f, g, c, h, i], is_dual)
    } else {
        select_68([a, c, d, e, f, g, b, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_61([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_62([b, c, d, h, g, a, i], is_dual)
    } else {
        select_67([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_50([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_51([a, b, c, f, d, e, g, h, i, j, k], is_dual)
    } else {
        select_61([c, a, d, e, b, h, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_14([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_15([b, c, d, e, f, g, h, i, a, j, k], is_dual)
    } else {
        select_50([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_78([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, d, c, f], is_dual)
    } else {
        select_24([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_77([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_78([a, b, c, e, d, f], is_dual)
    } else {
        select_78([a, b, d, e, c, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_76([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_77([d, c, a, f, e, g], is_dual)
    } else {
        select_19([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_80([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([b, c, d, e], is_dual)
    } else {
        select_71([a, g, h, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_81([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([g, h, f, d, a, b], !is_dual)
    } else {
        select_25([a, f, c, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_79([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_80([a, c, d, f, e, g, h, j], is_dual)
    } else {
        select_81([h, i, j, e, f, a, b, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_75([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_76([i, j, k, f, g, a, b, c], !is_dual)
    } else {
        select_79([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_84([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_20([a, c, d, e, h, g, j], is_dual)
    } else {
        select_20([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_85([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_41([b, c, d, e, f, g], is_dual)
    } else {
        select_25([a, g, h, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_83([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_84([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_85([a, g, c, d, e, h, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_88([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_24([a, b, c, d], is_dual)
    } else {
        select_29([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_87([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_28([a, c, b, d, e, f], is_dual)
    } else {
        select_88([a, b, e, f, d], is_dual)
    }
}
/// n = 9, i = 4
fn select_86([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_60([i, h, f, g, d, b, a, e], !is_dual)
    } else {
        select_87([b, d, c, f, g, e], is_dual)
    }
}
/// n = 11, i = 4
fn select_82([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_83([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_86([j, k, g, e, i, a, f, h, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_74([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_75([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_82([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_91([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_27([b, g, c, e, a, f, h], is_dual)
    } else {
        select_54([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_93([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_35([d, f, e, a, b], !is_dual)
    } else {
        select_35([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_92([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_64([d, e, a, g, f], !is_dual)
    } else {
        select_93([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_91([b, c, a, d, f, e, g, h], is_dual)
    } else {
        select_92([f, e, i, a, b, h, g], !is_dual)
    }
}
/// n = 6, i = 2
fn select_95([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_77([b, c, a, d, e, f], is_dual)
    } else {
        select_78([a, c, b, e, d, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_97([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_28([a, b, c, e, d, f], is_dual)
    } else {
        select_88([d, c, a, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_96([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_57([a, c, b, d, e, f], is_dual)
    } else {
        select_97([a, f, c, b, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_94([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_95([a, b, c, e, f, h], is_dual)
    } else {
        select_96([a, e, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_89([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_90([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_94([c, b, a, e, f, d, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_73([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_74([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_89([a, b, g, d, e, f, h, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_102([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_25([b, c, e, g], is_dual)
    } else {
        select_71([a, d, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_101([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_19([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_102([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_95([g, b, c, e, f, h], is_dual)
    } else {
        select_101([h, g, i, j, d, e, a, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_105([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([f, g, e, c, d, a], !is_dual)
    } else {
        select_29([e, b, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_104([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_77([d, g, a, e, f, b], !is_dual)
    } else {
        select_105([e, g, d, f, a, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_103([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_95([b, a, c, d, e, f], is_dual)
    } else {
        select_104([d, e, g, a, b, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_99([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_100([b, a, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_103([d, a, f, g, h, i, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_108([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_41([a, c, d, b, e, f], is_dual)
    } else {
        select_41([b, c, d, a, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_53([b, c, a, d, e, f, g], is_dual)
    } else {
        select_108([a, g, b, c, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_106([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_107([a, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_81([j, k, h, g, f, a, b, i], !is_dual)
    }
}
/// n = 11, i = 4
fn select_98([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_99([b, c, d, f, g, h, i, a, j, k], is_dual)
    } else {
        select_106([b, c, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_72([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_73([a, d, b, e, f, g, c, h, i, k, j], is_dual)
    } else {
        select_98([a, d, c, e, f, b, g, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_14([a, d, c, e, b, f, h, g, i, j, k], is_dual)
    } else {
        select_72([a, c, b, d, e, i, h, j, k, g, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_115([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_31([b, a, c, d, e], is_dual)
    } else {
        select_24([b, e, d, a], is_dual)
    }
}
/// n = 8, i = 2
fn select_114([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_57([a, e, c, d, f, g], is_dual)
    } else {
        select_115([g, a, b, h, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_114([a, b, c, f, e, h, i, g], is_dual)
    } else {
        select_20([b, d, g, e, h, f, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_117([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_57([e, c, a, d, f, g], is_dual)
    } else {
        select_57([a, b, d, f, e, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_116([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_95([e, g, d, a, f, h], is_dual)
    } else {
        select_117([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_113([a, b, f, d, e, g, i, h, k], is_dual)
    } else {
        select_116([a, e, g, c, h, f, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_121([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_55([b, c, d, e, g, f], is_dual)
    } else {
        select_71([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_120([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_121([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_55([c, g, d, e, f, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_122([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_48([a, b, c, e, g], is_dual)
    } else {
        select_57([a, d, b, c, e, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_119([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_120([a, f, c, d, e, g, i, h, k], is_dual)
    } else {
        select_122([a, g, h, b, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_118([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_119([a, d, e, b, c, f, g, h, i, k, j], is_dual)
    } else {
        select_63([a, b, c, g, i, j, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_112([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_118([b, a, c, d, e, g, h, f, i, k, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_127([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_28([f, c, g, e, d, a], !is_dual)
    } else {
        select_35([d, g, c, e, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_126([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_127([f, h, d, g, e, a, b], !is_dual)
    } else {
        select_105([d, h, f, g, e, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_129([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_28([d, b, c, e, f, g], is_dual)
    } else {
        select_88([a, b, d, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_128([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_129([a, d, b, g, e, i, f, h], is_dual)
    } else {
        select_19([a, b, c, f, h, g, i, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_126([b, e, d, f, a, h, g, j], is_dual)
    } else {
        select_128([b, c, d, a, f, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_131([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_27([b, f, d, e, a, g, i], is_dual)
    } else {
        select_45([c, a, d, b, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_133([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_28([a, g, c, f, d, i], is_dual)
    } else {
        select_36([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_133([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_133([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_130([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_131([a, c, b, g, f, e, i, j, h], is_dual)
    } else {
        select_132([a, f, b, d, e, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_124([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < j) || (is_dual && e > j) {
        select_125([a, b, c, g, f, i, j, h, e, k], is_dual)
    } else {
        select_130([b, c, a, d, f, e, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_137([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_39([a, c, g, f, d, i], is_dual)
    } else {
        select_36([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_136([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_137([b, a, c, e, f, i, h, j, g], is_dual)
    } else {
        select_137([b, a, d, e, f, i, g, j, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_138([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_77([b, c, d, f, e, g], is_dual)
    } else {
        select_97([g, f, h, a, d, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_135([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_136([b, c, a, d, e, f, h, i, g, j], is_dual)
    } else {
        select_138([e, g, b, a, f, j, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_140([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_77([e, b, a, g, d, f], is_dual)
    } else {
        select_20([b, a, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_140([a, c, d, e, g, f, h], is_dual)
    } else {
        select_97([f, b, a, d, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_134([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_135([b, d, a, c, e, g, f, i, h, j], is_dual)
    } else {
        select_139([b, f, d, a, g, h, e, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_124([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    } else {
        select_134([a, c, d, e, b, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_110([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_111([a, c, b, e, d, f, h, g, j, i, k], is_dual)
    } else {
        select_123([a, b, c, g, e, i, h, f, k, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_145([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_66([f, c, d, e, a, g, h], is_dual)
    } else {
        select_70([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_147([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([b, c, d, e], is_dual)
    } else {
        select_22([a, f], is_dual)
    }
}
/// n = 8, i = 1
fn select_148([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_49([d, e, f, g], is_dual)
    } else {
        select_49([a, b, c, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_147([g, d, e, f, a, h], is_dual)
    } else {
        select_148([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_144([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_145([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_146([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_144([a, b, c, d, e, f, h, i, j], is_dual)
    } else {
        select_144([a, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_152([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_25([a, g, i, h], !is_dual)
    } else {
        select_36([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_151([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_60([b, c, d, f, e, h, g, i], is_dual)
    } else {
        select_152([a, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_154([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_41([a, b, c, d, f, g], is_dual)
    } else {
        select_25([a, e, g, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_153([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_38([a, b, c, d, e, f, g], is_dual)
    } else {
        select_154([a, b, e, d, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_150([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_151([a, b, c, d, f, e, h, g, i, j], is_dual)
    } else {
        select_153([e, c, d, a, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_157([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_35([e, f, a, b, c], !is_dual)
    } else {
        select_71([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_156([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_147([b, a, c, d, e, f], is_dual)
    } else {
        select_157([a, e, g, h, f, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_159([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_28([a, c, b, f, g, e], is_dual)
    } else {
        select_28([a, d, b, e, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_159([a, b, c, d, e, f, g], is_dual)
    } else {
        select_65([a, e, f, h, g, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_155([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_156([a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_158([a, c, b, f, g, h, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_149([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_150([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_155([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_142([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_143([a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_149([a, b, c, j, d, i, h, k, l, m], is_dual)
    }
}
/// n = 5, i = 0
fn select_164([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_49([a, b, c, d], is_dual)
    } else {
        select_49([a, b, c, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_165([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_35([b, c, a, d, e], is_dual)
    } else {
        select_24([a, c, b, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_163([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_164([c, d, e, f, g], is_dual)
    } else {
        select_165([a, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 2
fn select_166([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_164([c, d, e, f, h], is_dual)
    } else {
        select_31([a, b, i, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_162([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_163([a, h, c, d, e, f, g, i, j, k], is_dual)
    } else {
        select_166([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 1
fn select_168([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_69([a, b, c, d, e, g, h], is_dual)
    } else {
        select_69([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 5, i = 2
fn select_170([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_71([a, b, c, d], is_dual)
    } else {
        select_71([a, b, c, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_169([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_164([c, d, e, f, g], is_dual)
    } else {
        select_170([a, b, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_168([b, g, c, d, e, f, h, i], is_dual)
    } else {
        select_169([a, h, c, d, e, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_161([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_162([a, c, d, e, f, g, h, b, i, j, k], is_dual)
    } else {
        select_167([a, b, d, e, f, g, h, c, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_174([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_41([a, c, d, e, f, h], is_dual)
    } else {
        select_71([b, c, d, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_173([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_38([a, b, c, d, f, e, g], is_dual)
    } else {
        select_174([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_176([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([d, f, g, a, e, b], !is_dual)
    } else {
        select_35([e, f, d, b, c], !is_dual)
    }
}
/// n = 7, i = 3
fn select_175([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_176([a, b, c, d, e, f, g], is_dual)
    } else {
        select_24([a, g, f, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_172([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_173([a, b, c, d, e, f, h, g, i], is_dual)
    } else {
        select_175([a, b, f, d, g, h, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_178([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_48([a, c, d, e, f], is_dual)
    } else {
        select_165([a, b, f, g, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_177([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_95([d, b, c, e, f, g], is_dual)
    } else {
        select_178([g, d, h, i, a, e, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_172([e, g, i, a, f, h, b, c, d], !is_dual)
    } else {
        select_177([e, f, i, a, g, h, b, c, d], !is_dual)
    }
}
/// n = 12, i = 4
fn select_160([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_161([b, c, d, a, e, f, g, h, i, j, k], is_dual)
    } else {
        select_171([b, c, d, i, h, a, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_141([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_142([a, b, d, g, c, e, f, h, i, j, l, k, m], is_dual)
    } else {
        select_160([a, b, c, e, d, f, i, h, k, m, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < j) || (is_dual && c > j) {
        select_110([a, b, e, d, g, h, j, i, c, k, l, m], is_dual)
    } else {
        select_141([a, b, d, e, c, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_13([a, b, c, f, e, d, h, k, j, l, i, m], is_dual)
    } else {
        select_109([a, b, c, f, d, g, e, h, i, k, j, m, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_185([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_121([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_29([a, h, b], is_dual)
    }
}
/// n = 9, i = 2
fn select_186([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_121([a, e, f, c, d, g, h, i], is_dual)
    } else {
        select_71([a, b, i, f], is_dual)
    }
}
/// n = 12, i = 3
fn select_184([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_185([a, b, j, e, f, i, h, l], is_dual)
    } else {
        select_186([c, d, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_189([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_20([a, b, c, e, f, g, h], is_dual)
    } else {
        select_21([b, c, d], is_dual)
    }
}
/// n = 10, i = 2
fn select_188([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_189([e, c, d, g, f, i, h, j], is_dual)
    } else {
        select_189([e, a, b, h, f, i, g, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_187([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_96([a, h, c, i, g, k, m], is_dual)
    } else {
        select_188([a, g, d, e, b, f, j, i, h, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_183([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_184([a, c, b, d, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_187([b, c, d, e, f, g, h, a, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_193([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_55([b, c, d, e, g, h], is_dual)
    } else {
        select_31([a, d, e, f, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_194([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_20([b, c, d, e, g, f, h], is_dual)
    } else {
        select_25([a, h, b, e], is_dual)
    }
}
/// n = 10, i = 2
fn select_192([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_193([f, d, e, b, c, g, i, h, j], is_dual)
    } else {
        select_194([h, a, d, e, f, i, g, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_195([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_57([a, c, d, e, g, f], is_dual)
    } else {
        select_159([a, b, c, f, g, e, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_191([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_192([b, a, g, d, e, f, h, j, i, l], is_dual)
    } else {
        select_195([a, f, i, c, h, g, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_20([e, c, d, f, i, h, k], is_dual)
    } else {
        select_20([e, a, b, f, j, g, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_198([b, c, d, e, a, g, h, i, j, k, l], is_dual)
    } else {
        select_194([f, a, b, c, g, h, k, l], is_dual)
    }
}
/// n = 4, i = 1
fn select_200([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_25([a, b, c, d], is_dual)
    } else {
        select_25([a, b, d, c], is_dual)
    }
}
/// n = 7, i = 2
fn select_201([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_20([a, c, d, b, e, f, g], is_dual)
    } else {
        select_20([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_199([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_200([a, b, c, h], is_dual)
    } else {
        select_201([a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_196([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_197([d, b, c, e, f, a, g, h, i, j, l, k], is_dual)
    } else {
        select_199([b, c, j, e, f, i, h, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_190([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_191([c, b, d, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_196([c, a, b, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_182([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, b, d, c, e, f, i, h, j, g, k, l, m], is_dual)
    } else {
        select_190([a, d, c, b, e, f, i, g, j, h, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_206([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_55([a, b, c, d, e, f], is_dual)
    } else {
        select_25([a, f, b, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_207([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_41([a, b, c, d, f, g], is_dual)
    } else {
        select_25([b, d, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_205([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_206([b, c, d, e, f, h, g], is_dual)
    } else {
        select_207([b, a, g, f, i, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_209([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_41([a, c, d, e, f, h], is_dual)
    } else {
        select_41([b, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_210([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_28([a, e, c, f, d, h], is_dual)
    } else {
        select_28([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_208([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_209([a, c, d, e, f, h, g, j], is_dual)
    } else {
        select_210([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_204([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_205([b, e, c, d, g, f, h, i, j], is_dual)
    } else {
        select_208([a, b, f, d, g, e, h, i, j, k], is_dual)
    }
}
/// n = 3, i = 1
fn select_214([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_29([a, b, c], is_dual)
    } else {
        select_29([a, c, b], is_dual)
    }
}
/// n = 7, i = 3
fn select_213([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_214([a, f, g], is_dual)
    } else {
        select_78([a, b, c, d, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_129([a, c, b, e, d, g, f, h], is_dual)
    } else {
        select_213([a, b, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_215([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_27([b, c, a, e, f, d, g], is_dual)
    } else {
        select_30([a, b, c, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_212([a, c, d, e, f, i, h, j], is_dual)
    } else {
        select_215([b, c, d, g, e, h, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_204([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    } else {
        select_211([c, a, d, e, f, g, b, i, h, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_217([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_96([a, c, b, d, e, f, g], is_dual)
    } else {
        select_92([a, e, h, b, c, g, f], !is_dual)
    }
}
/// n = 6, i = 2
fn select_219([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_64([a, c, b, d, e], is_dual)
    } else {
        select_165([a, c, e, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_220([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_57([b, c, a, d, e, f], is_dual)
    } else {
        select_93([b, c, f, e, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_218([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_219([a, f, b, c, g, e], is_dual)
    } else {
        select_220([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_216([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_217([d, g, e, i, a, h, f, c], !is_dual)
    } else {
        select_218([e, i, d, g, h, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_202([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_203([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_216([a, j, k, b, f, i, h, g, c], !is_dual)
    }
}
/// n = 13, i = 4
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_182([a, b, c, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_202([a, b, d, c, j, h, i, k, l, g, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_225([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_108([a, g, f, h, d, b], !is_dual)
    } else {
        select_108([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_224([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_140([a, b, c, d, f, e, g], is_dual)
    } else {
        select_225([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_228([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_28([a, b, c, f, e, g], is_dual)
    } else {
        select_28([a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_227([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_69([a, b, c, d, f, g, h], is_dual)
    } else {
        select_228([e, h, a, b, f, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_229([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_174([a, e, c, d, f, g, i, h], is_dual)
    } else {
        select_81([e, i, j, f, h, a, g, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_226([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_227([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_229([b, g, d, e, a, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_223([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_224([a, f, k, b, i, j, h, g], !is_dual)
    } else {
        select_226([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_232([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_48([e, c, d, f, g], is_dual)
    } else {
        select_48([e, a, b, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_233([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_70([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_40([b, g, c, a, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_231([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_232([c, f, d, e, a, g, h], is_dual)
    } else {
        select_233([a, b, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_235([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_48([e, a, h, i, g], !is_dual)
    } else {
        select_27([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_234([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_235([b, h, c, f, a, g, i, j, k], is_dual)
    } else {
        select_68([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_230([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_231([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_234([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_222([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_223([a, b, d, c, i, f, h, g, j, k, l], is_dual)
    } else {
        select_230([a, b, d, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_240([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_28([a, d, c, e, f, g], is_dual)
    } else {
        select_25([a, b, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_241([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_36([a, b, d, e, f, i, j], is_dual)
    } else {
        select_36([b, c, d, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_239([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_240([a, i, e, d, g, h, k], is_dual)
    } else {
        select_241([a, b, e, c, h, f, g, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_243([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_28([a, c, b, e, f, g], is_dual)
    } else {
        select_71([a, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_242([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_243([a, e, d, g, f, h, j], is_dual)
    } else {
        select_240([b, c, e, h, f, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_238([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_239([b, a, c, d, e, g, h, i, f, k, j], is_dual)
    } else {
        select_242([b, a, f, d, e, h, g, i, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_245([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_121([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_121([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_246([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_121([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_69([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_244([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_245([a, b, h, c, e, f, g, i, j, k, l], is_dual)
    } else {
        select_246([c, d, g, e, f, b, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_237([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_238([b, c, i, d, e, a, g, h, j, k, l], is_dual)
    } else {
        select_244([b, c, d, e, a, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 5, i = 2
fn select_250([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_214([a, d, e], is_dual)
    } else {
        select_24([a, b, c, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_251([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_20([a, b, c, d, e, f, g], is_dual)
    } else {
        select_41([d, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_249([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_250([a, b, h, e, i], is_dual)
    } else {
        select_251([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_253([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_28([a, b, c, e, d, f], is_dual)
    } else {
        select_24([c, d, e, b], is_dual)
    }
}
/// n = 10, i = 4
fn select_252([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_154([a, c, d, g, h, f, j], is_dual)
    } else {
        select_253([i, j, h, a, e, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_248([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_249([b, a, c, d, e, g, f, j, i], is_dual)
    } else {
        select_252([a, b, c, d, f, g, e, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_256([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_20([a, b, c, d, e, f, g], is_dual)
    } else {
        select_71([e, b, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_257([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_28([a, f, b, e, h, i], is_dual)
    } else {
        select_21([c, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_255([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_256([b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_257([a, b, c, d, e, g, f, h, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_259([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_35([a, d, g, f, c], is_dual)
    } else {
        select_35([b, c, g, e, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_258([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_259([a, b, d, f, e, h, g], is_dual)
    } else {
        select_22([b, c], is_dual)
    }
}
/// n = 10, i = 4
fn select_254([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_255([b, a, c, d, f, e, g, i, h, j], is_dual)
    } else {
        select_258([e, c, d, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_247([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_248([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_254([a, c, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_236([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_237([a, b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_247([a, b, c, h, i, j, k, l, g, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_221([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < h) || (is_dual && c > h) {
        select_222([a, b, h, e, f, g, c, i, j, l, k, m], is_dual)
    } else {
        select_236([a, b, d, c, e, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_180([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_181([a, b, e, d, c, j, g, h, i, l, m, k, n], is_dual)
    } else {
        select_221([a, b, c, e, f, i, g, h, j, k, l, n, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_266([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([c, d, e, f], is_dual)
    } else {
        select_25([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_267([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_49([c, d, e, g], is_dual)
    } else {
        select_31([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_266([a, g, c, d, e, f, h, i], is_dual)
    } else {
        select_267([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_268([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_70([a, h, d, e, f, g, i, j], is_dual)
    } else {
        select_148([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_264([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_265([b, h, d, e, f, g, a, i, j], is_dual)
    } else {
        select_268([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 4
fn select_263([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_264([a, c, e, f, g, i, h, j, k, m], is_dual)
    } else {
        select_264([a, b, d, f, h, j, g, i, l, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_272([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_31([a, c, d, e, h], is_dual)
    } else {
        select_36([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_273([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_28([a, e, b, d, g, f], is_dual)
    } else {
        select_71([a, c, f, e], is_dual)
    }
}
/// n = 9, i = 2
fn select_271([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_272([d, e, b, c, f, g, i, h], is_dual)
    } else {
        select_273([h, d, a, f, e, i, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_270([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_271([b, a, f, d, e, g, h, i, k], is_dual)
    } else {
        select_195([a, e, g, c, h, f, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_275([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_84([b, g, d, e, f, a, h, i, j, k], is_dual)
    } else {
        select_198([a, c, d, e, b, f, g, h, i, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_277([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_28([a, b, c, d, e, f], is_dual)
    } else {
        select_29([a, f, b], is_dual)
    }
}
/// n = 7, i = 2
fn select_276([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_200([a, b, c, f], is_dual)
    } else {
        select_277([b, c, d, e, g, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_274([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_275([c, e, d, a, b, g, f, h, k, j, i], is_dual)
    } else {
        select_276([a, b, i, e, g, k, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_269([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_270([c, b, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_274([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_262([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_263([a, b, d, c, f, e, h, i, g, j, k, l, m, n], is_dual)
    } else {
        select_269([a, d, c, h, f, i, k, j, g, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_280([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_63([c, a, b, f, h, i, g], is_dual)
    } else {
        select_120([c, d, e, a, b, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_279([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_113([c, b, d, e, f, g, h, a, i], is_dual)
    } else {
        select_280([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_283([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_121([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_228([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_284([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_102([b, a, e, d, g, f, i, h], is_dual)
    } else {
        select_46([b, a, d, c, f, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_282([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_283([b, c, d, a, e, f, h, g, i, j], is_dual)
    } else {
        select_284([b, c, g, d, a, f, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_243([b, d, c, e, f, g, h], is_dual)
    } else {
        select_28([a, g, d, f, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_285([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_286([b, c, e, a, f, d, g, h, i], is_dual)
    } else {
        select_284([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_281([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_282([a, d, b, c, e, f, g, h, j, i], is_dual)
    } else {
        select_285([a, d, b, c, g, f, i, j, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_278([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_279([a, c, f, d, e, g, i, h, j], is_dual)
    } else {
        select_281([a, b, h, c, e, g, i, f, j, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < b) || (is_dual && k > b) {
        select_262([a, c, d, e, b, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_278([a, d, e, k, f, g, i, b, j, m, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_292([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_214([a, e, f], is_dual)
    } else {
        select_31([a, b, c, d, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_291([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_292([a, b, f, d, e, g], is_dual)
    } else {
        select_41([a, c, e, d, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_290([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_95([f, a, b, d, g, e], is_dual)
    } else {
        select_291([a, c, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_295([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_28([f, c, b, e, h, g], is_dual)
    } else {
        select_78([a, c, d, g, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_294([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_137([a, d, b, f, e, g, i, h, j], is_dual)
    } else {
        select_295([a, d, c, g, e, h, i, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_296([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_57([a, f, d, e, g, h], is_dual)
    } else {
        select_38([b, c, a, h, i, f, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_293([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_294([b, c, h, d, f, k, g, j, i, l], is_dual)
    } else {
        select_296([a, c, b, e, i, g, j, l, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_289([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < m) || (is_dual && g > m) {
        select_290([a, h, i, f, j, k, m], is_dual)
    } else {
        select_293([b, c, d, a, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 5, i = 2
fn select_300([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_25([a, b, c, e], is_dual)
    } else {
        select_25([a, c, b, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_301([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_35([i, j, a, b, g], !is_dual)
    } else {
        select_31([d, c, f, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_299([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_300([a, b, c, j, k], is_dual)
    } else {
        select_301([b, c, e, d, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_302([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_87([a, d, g, f, e, i], is_dual)
    } else {
        select_240([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_299([a, b, d, c, e, f, h, i, j, g, k], is_dual)
    } else {
        select_302([c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_304([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_48([c, d, e, f, g], is_dual)
    } else {
        select_65([c, a, b, g, h, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_306([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([c, d, e, f], is_dual)
    } else {
        select_35([h, i, a, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_305([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_300([a, b, c, i, j], is_dual)
    } else {
        select_306([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_303([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_304([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_305([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_297([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_298([a, c, b, e, i, g, k, j, h, m, l], is_dual)
    } else {
        select_303([a, e, c, d, f, h, j, l, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_288([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_289([b, e, c, d, f, g, h, i, j, a, k, l, m], is_dual)
    } else {
        select_297([a, b, c, e, d, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_309([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_63([b, a, c, f, e, g, i], is_dual)
    } else {
        select_57([a, d, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_311([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_48([b, d, e, f, h], is_dual)
    } else {
        select_70([a, c, b, d, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_312([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_159([b, c, d, e, f, h, g], is_dual)
    } else {
        select_87([a, c, g, f, e, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_310([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_311([c, b, e, d, g, i, h, l, j], is_dual)
    } else {
        select_312([a, c, h, e, f, j, i, g, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_308([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_309([a, d, c, f, i, g, j, k, h], is_dual)
    } else {
        select_310([b, d, a, e, c, f, g, i, h, k, l, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_316([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_39([a, c, e, h, d, g], is_dual)
    } else {
        select_39([b, c, d, h, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_315([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_316([b, d, c, e, f, h, i, g], is_dual)
    } else {
        select_207([e, a, g, h, j, d, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_314([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_315([a, b, f, d, e, g, i, j, h, k], is_dual)
    } else {
        select_31([b, c, h, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_319([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_20([a, b, f, d, g, e, h], is_dual)
    } else {
        select_20([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_318([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_300([i, a, j, d, k], is_dual)
    } else {
        select_319([d, b, c, g, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_320([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_57([b, d, c, g, f, h], is_dual)
    } else {
        select_250([a, i, e, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_317([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_318([a, b, c, e, i, f, g, j, k, h, l], is_dual)
    } else {
        select_320([a, b, d, f, e, h, g, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_313([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_314([a, c, b, e, f, i, h, j, k, l, g], is_dual)
    } else {
        select_317([a, b, c, d, e, f, h, g, i, j, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_307([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_308([a, b, c, d, e, g, h, f, i, j, k, l], is_dual)
    } else {
        select_313([b, d, c, e, g, f, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_287([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_288([a, c, b, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_307([a, c, g, e, f, b, h, i, j, l, m, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_260([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_261([a, c, d, e, b, f, h, i, g, j, k, l, m, n], is_dual)
    } else {
        select_287([a, b, d, e, c, j, h, i, k, l, g, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_179([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_180([a, b, e, f, c, g, h, d, i, j, k, l, m, n], is_dual)
    } else {
        select_260([a, b, e, f, d, g, h, c, i, j, k, l, n, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_12([a, b, d, f, e, c, j, h, i, m, l, n, k], is_dual)
    } else {
        select_179([a, b, d, c, f, e, g, h, i, j, k, m, n, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_11([a, b, c, d, e, g, f, h, i, j, l, m, k, n], is_dual)
    } else {
        select_11([a, b, d, c, e, f, g, h, i, k, l, n, j, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_10([g, d, b, e, c, a, f, j, i, h, l, m, n, k], is_dual)
    } else {
        select_10([g, d, b, f, c, a, e, j, i, h, k, m, n, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_9([b, c, a, d, e, f, g, i, h, k, l, m, n, j], is_dual)
    } else {
        select_9([c, b, a, d, e, f, g, j, h, k, l, m, n, i], is_dual)
    }
}
/// n = 14, i = 4
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_8([a, b, c, d, e, f, g, h, j, k, l, m, n, i], is_dual)
    } else {
        select_8([a, b, c, d, e, f, g, i, j, k, l, m, n, h], is_dual)
    }
}
/// n = 14, i = 4
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_7([a, b, c, d, e, f, g, i, j, k, l, m, n, h], is_dual)
    } else {
        select_7([a, b, c, d, e, h, g, i, j, k, l, m, n, f], is_dual)
    }
}
/// n = 14, i = 4
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_6([a, b, c, d, e, f, g, h, j, k, l, m, n, i], is_dual)
    } else {
        select_6([a, b, c, d, i, f, g, h, j, k, l, m, n, e], is_dual)
    }
}
/// n = 14, i = 4
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_5([j, a, h, i, b, c, d, e, f, m, n, g, k, l], is_dual)
    } else {
        select_5([j, g, h, i, b, c, d, e, f, m, n, a, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_4([a, b, c, d, e, f, g, h, j, k, i, l, m, n], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, i, j, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_3([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_325([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_8([a, b, c, d, e, f, g, i, j, k, l, m, h, n], is_dual)
    } else {
        select_8([a, b, c, d, e, h, g, i, j, k, l, m, f, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_324([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_325([a, b, c, d, e, f, g, h, j, k, l, m, i, n], is_dual)
    } else {
        select_325([a, b, c, d, i, f, g, h, j, k, l, m, e, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_323([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_324([j, a, h, i, b, c, d, e, f, m, g, k, l, n], is_dual)
    } else {
        select_324([j, g, h, i, b, c, d, e, f, m, a, k, l, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_322([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_323([a, b, c, d, e, f, g, h, j, k, i, l, m, n], is_dual)
    } else {
        select_323([a, b, c, d, e, f, g, i, j, k, h, l, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_321([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_322([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_322([a, b, c, d, e, f, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_321([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
