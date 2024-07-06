/// n = 1, i = 0
fn select_27([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_26([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_27([a], is_dual)
    } else {
        select_27([b], is_dual)
    }
}
/// n = 4, i = 1
fn select_25([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_26([a, d], is_dual)
    } else {
        select_26([b, c], is_dual)
    }
}
/// n = 3, i = 1
fn select_28([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_26([a, b], is_dual)
    } else {
        select_27([c], is_dual)
    }
}
/// n = 6, i = 2
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_25([b, c, e, d], is_dual)
    } else {
        select_28([a, b, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([e, a, c, f, d, g], is_dual)
    } else {
        select_24([d, b, c, f, e, g], is_dual)
    }
}
/// n = 3, i = 0
fn select_31([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_26([a, b], is_dual)
    } else {
        select_26([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_30([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([c, d, e], is_dual)
    } else {
        select_31([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_29([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_30([a, f, c, d, e, g], is_dual)
    } else {
        select_30([b, e, c, d, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_22([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_23([b, g, c, e, a, f, h], is_dual)
    } else {
        select_29([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_22([a, d, b, e, f, c, g, h], is_dual)
    } else {
        select_22([a, d, c, e, f, b, g, h], is_dual)
    }
}
/// n = 4, i = 0
fn select_34([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_31([a, b, c], is_dual)
    } else {
        select_31([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_33([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_34([a, b, c, d], is_dual)
    } else {
        select_27([e], is_dual)
    }
}
/// n = 7, i = 3
fn select_32([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_33([a, b, c, d, e], is_dual)
    } else {
        select_33([a, b, f, g, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_20([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_21([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_32([a, f, c, g, h, b, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_39([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_31([b, c, d], is_dual)
    } else {
        select_28([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_38([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_39([a, c, d, b, e, f], is_dual)
    } else {
        select_39([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_37([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_38([a, g, f, h, d, b], !is_dual)
    } else {
        select_38([a, f, g, h, e, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_41([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_24([a, d, b, c, e, f], is_dual)
    } else {
        select_24([a, d, c, b, e, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_43([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_25([a, b, c, d], is_dual)
    } else {
        select_28([a, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_42([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_24([a, c, b, d, e, f], is_dual)
    } else {
        select_43([a, b, e, f, d], is_dual)
    }
}
/// n = 7, i = 3
fn select_40([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_41([a, d, f, g, e, b], !is_dual)
    } else {
        select_42([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_36([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_37([a, b, c, d, e, f, g, i], is_dual)
    } else {
        select_40([a, e, b, g, d, h, i], is_dual)
    }
}
/// n = 4, i = 1
fn select_47([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_31([a, b, c], is_dual)
    } else {
        select_27([d], is_dual)
    }
}
/// n = 8, i = 3
fn select_46([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_39([a, c, d, e, f, h], is_dual)
    } else {
        select_47([b, c, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_49([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_31([b, c, e], is_dual)
    } else {
        select_25([a, f, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_48([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_49([a, b, c, d, g, e, f], is_dual)
    } else {
        select_49([a, b, c, d, g, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_45([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_46([f, g, h, i, a, e, d, c], !is_dual)
    } else {
        select_48([c, a, b, e, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_44([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_45([e, a, c, h, f, d, g, i, j], is_dual)
    } else {
        select_45([d, b, c, h, f, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_35([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_36([e, f, c, a, g, d, h, i, j], is_dual)
    } else {
        select_44([a, b, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_19([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_20([a, b, e, h, f, i, g, j, l], is_dual)
    } else {
        select_35([k, m, i, a, b, l, h, c, d, j], !is_dual)
    }
}
/// n = 6, i = 2
fn select_55([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([a, d, c, f], is_dual)
    } else {
        select_25([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_54([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_55([a, b, c, e, d, f], is_dual)
    } else {
        select_55([a, b, d, e, c, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_56([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([a, b, c, e, d, f], is_dual)
    } else {
        select_43([d, c, a, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_53([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_54([b, c, d, f, e, g], is_dual)
    } else {
        select_56([g, f, h, a, d, b], !is_dual)
    }
}
/// n = 7, i = 2
fn select_58([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_34([b, c, d, e], is_dual)
    } else {
        select_28([a, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_59([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_24([a, h, b, f, i, j], is_dual)
    } else {
        select_34([c, d, e, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_57([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_58([g, c, d, e, a, h, j], is_dual)
    } else {
        select_59([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_52([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_53([a, h, c, b, f, i, g, j], is_dual)
    } else {
        select_57([a, c, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 5, i = 1
fn select_63([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_31([b, c, d], is_dual)
    } else {
        select_26([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_62([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_63([e, c, d, a, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_61([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_62([d, a, b, g, e, f], is_dual)
    } else {
        select_62([d, a, c, f, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_60([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_61([a, d, e, b, c, f, g], is_dual)
    } else {
        select_22([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_51([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_52([b, a, d, c, g, f, h, j, k, i], is_dual)
    } else {
        select_60([b, h, c, a, e, i, g, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_67([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_63([a, b, c, d, e], is_dual)
    } else {
        select_28([e, a, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_66([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_67([a, b, g, e, h, f], is_dual)
    } else {
        select_33([e, c, d, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_69([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_34([b, c, d, e], is_dual)
    } else {
        select_26([a, f], is_dual)
    }
}
/// n = 5, i = 2
fn select_71([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_25([a, b, c, d], is_dual)
    } else {
        select_28([d, a, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_70([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_71([e, f, a, b, c], !is_dual)
    } else {
        select_47([b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_68([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_69([b, a, c, d, e, f], is_dual)
    } else {
        select_70([a, e, g, h, f, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_65([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_66([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_68([e, f, c, d, a, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_64([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_65([a, c, b, g, e, f, i, j, h, k], is_dual)
    } else {
        select_60([a, e, b, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_50([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_51([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_64([b, c, f, e, h, i, g, a, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_18([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_19([a, b, c, d, e, f, h, g, i, j, k, l, m], is_dual)
    } else {
        select_50([b, a, e, c, f, g, h, i, d, j, k, l], is_dual)
    }
}
/// n = 5, i = 1
fn select_77([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_63([a, c, d, b, e], is_dual)
    } else {
        select_63([b, c, d, a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_76([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_77([a, b, c, f, e], is_dual)
    } else {
        select_77([a, b, d, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_78([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_67([b, c, d, a, e, f], is_dual)
    } else {
        select_63([a, c, d, b, e], is_dual)
    }
}
/// n = 12, i = 4
fn select_75([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_76([c, d, f, h, g, i], is_dual)
    } else {
        select_78([a, b, e, j, k, l], is_dual)
    }
}
/// n = 4, i = 1
fn select_82([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_25([a, c, b, d], is_dual)
    } else {
        select_25([b, c, a, d], is_dual)
    }
}
/// n = 5, i = 2
fn select_81([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_82([a, b, c, e], is_dual)
    } else {
        select_82([a, c, b, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_83([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_63([e, c, d, g, f], is_dual)
    } else {
        select_82([a, b, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_80([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_81([a, b, h, i, j], is_dual)
    } else {
        select_83([a, b, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_85([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_24([a, b, c, i, j, k], is_dual)
    } else {
        select_49([c, d, e, g, h, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_84([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_42([b, h, c, a, j, k], is_dual)
    } else {
        select_85([b, c, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_79([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_80([a, b, d, e, h, f, i, j, k, g], is_dual)
    } else {
        select_84([a, b, c, d, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_74([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_75([b, c, a, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_79([b, c, e, d, g, i, a, h, j, k, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_89([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_71([d, f, e, a, b], !is_dual)
    } else {
        select_71([e, f, d, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_88([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_89([a, b, e, d, g, h], is_dual)
    } else {
        select_56([d, a, c, f, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_91([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_34([c, d, e, g], is_dual)
    } else {
        select_63([a, b, h, f, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_90([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_69([g, c, d, e, a, h], is_dual)
    } else {
        select_91([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_87([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_88([b, h, c, a, f, i, g, j], is_dual)
    } else {
        select_90([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_86([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_60([a, h, c, b, e, i, g, j], is_dual)
    } else {
        select_87([a, b, d, c, g, f, h, j, k, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_73([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_74([a, b, g, d, e, f, h, i, k, j, l, m], is_dual)
    } else {
        select_86([a, c, d, b, f, i, h, j, g, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_95([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_33([a, d, f, g, b], !is_dual)
    } else {
        select_38([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_94([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_76([c, d, f, h, g, i], is_dual)
    } else {
        select_95([a, l, m, b, k, e, j], !is_dual)
    }
}
/// n = 7, i = 3
fn select_98([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_24([a, c, b, d, e, f], is_dual)
    } else {
        select_39([e, d, g, a, f, b], !is_dual)
    }
}
/// n = 12, i = 5
fn select_97([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_85([b, c, a, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_98([b, h, c, a, j, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_100([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([h, e, j, i, a, b], !is_dual)
    } else {
        select_47([g, c, d, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_101([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_24([a, b, c, e, d, f], is_dual)
    } else {
        select_25([c, d, e, b], is_dual)
    }
}
/// n = 11, i = 5
fn select_99([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_100([h, f, j, k, d, i, a, b, g, e], !is_dual)
    } else {
        select_101([b, f, c, h, e, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_96([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_97([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    } else {
        select_99([i, k, l, g, h, a, j, b, f, d, e], !is_dual)
    }
}
/// n = 13, i = 5
fn select_93([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_94([b, c, a, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_96([b, c, e, d, g, i, h, a, j, k, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_34([c, d, e, g], is_dual)
    } else {
        select_39([a, b, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_104([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_69([g, c, d, e, b, h], is_dual)
    } else {
        select_105([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_107([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_24([f, c, g, e, d, a], !is_dual)
    } else {
        select_71([d, g, c, e, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_106([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_89([h, f, i, a, d, c], !is_dual)
    } else {
        select_107([b, c, d, f, e, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_103([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_104([b, c, a, d, e, f, g, h, i, k], is_dual)
    } else {
        select_106([g, k, j, a, i, f, b, c, h], !is_dual)
    }
}
/// n = 12, i = 4
fn select_102([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_60([a, h, d, c, e, i, g, j], is_dual)
    } else {
        select_103([a, b, c, d, g, f, h, j, k, i, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_92([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_93([a, b, g, d, e, f, h, i, k, j, l, m, n], is_dual)
    } else {
        select_102([a, b, c, d, f, i, h, j, g, k, l, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_72([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_73([b, c, a, e, f, g, d, h, i, j, k, l, m], is_dual)
    } else {
        select_92([b, c, d, e, f, g, a, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_17([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_18([b, a, c, d, e, g, i, h, j, l, n, k, m], is_dual)
    } else {
        select_72([a, b, c, e, d, f, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_114([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_24([a, d, c, e, f, g], is_dual)
    } else {
        select_82([a, b, d, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_116([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_25([a, c, d, g], is_dual)
    } else {
        select_25([b, c, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_115([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_63([a, c, d, e, h], is_dual)
    } else {
        select_116([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_113([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_114([a, h, e, b, f, g, i], is_dual)
    } else {
        select_115([a, e, c, d, g, f, h, i], is_dual)
    }
}
/// n = 7, i = 1
fn select_118([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([c, d, e, f], is_dual)
    } else {
        select_31([a, b, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_119([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_34([d, e, f, g], is_dual)
    } else {
        select_34([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 1
fn select_117([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_118([a, h, d, e, f, g, i], is_dual)
    } else {
        select_119([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_112([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_113([a, j, b, c, d, h, g, i, k], is_dual)
    } else {
        select_117([d, b, c, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_111([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_112([a, c, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_112([b, c, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 2
fn select_110([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_111([a, b, e, f, c, g, h, d, i, j, k], is_dual)
    } else {
        select_111([a, b, e, f, d, g, h, c, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_123([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_67([g, b, c, a, f, h], is_dual)
    } else {
        select_118([b, c, a, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 1
fn select_124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_69([g, d, e, f, a, h], is_dual)
    } else {
        select_119([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_122([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_123([f, a, h, d, e, g, i, j], is_dual)
    } else {
        select_124([f, d, e, b, c, g, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_127([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_39([b, c, d, e, f, g], is_dual)
    } else {
        select_82([a, g, h, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_128([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_63([b, c, d, g, f], is_dual)
    } else {
        select_71([a, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_126([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_127([f, a, c, d, e, g, h, i], is_dual)
    } else {
        select_128([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_130([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_82([a, f, b, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_131([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([c, d, e, f], is_dual)
    } else {
        select_47([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_129([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_130([a, c, b, h, g, f, i], is_dual)
    } else {
        select_131([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_125([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_126([g, b, d, e, a, f, h, i, j], is_dual)
    } else {
        select_129([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_121([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_122([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_125([b, h, c, e, f, a, g, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_134([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_23([a, b, c, d, e, f, h], is_dual)
    } else {
        select_56([e, h, i, a, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_135([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_23([b, c, d, e, g, f, h], is_dual)
    } else {
        select_98([a, b, c, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_133([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_134([f, b, d, a, e, g, h, i, j], is_dual)
    } else {
        select_135([a, b, c, d, e, g, f, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_137([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_131([g, a, c, d, e, f, h, i], is_dual)
    } else {
        select_131([f, b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_136([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_117([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_137([b, h, d, e, f, g, a, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_132([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < i) || (is_dual && b > i) {
        select_133([a, i, c, d, b, g, h, j, k, l], is_dual)
    } else {
        select_136([a, c, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_120([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_121([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_132([a, b, d, e, f, g, c, h, i, k, j, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_109([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_110([a, c, d, e, g, h, b, i, j, k, l], is_dual)
    } else {
        select_120([a, j, b, c, f, d, e, i, l, k, m, n], is_dual)
    }
}
/// n = 7, i = 3
fn select_143([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_39([a, b, c, d, f, g], is_dual)
    } else {
        select_82([b, d, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_142([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_115([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_143([e, h, i, a, g, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_141([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_142([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_142([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_140([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_141([a, b, e, c, g, h, f, j, i], is_dual)
    } else {
        select_141([a, b, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_146([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_62([b, c, a, d, e, f], is_dual)
    } else {
        select_38([a, e, b, d, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_148([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_24([d, b, c, e, f, h], is_dual)
    } else {
        select_116([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 6, i = 2
fn select_150([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_25([a, b, e, f], is_dual)
    } else {
        select_25([a, c, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_149([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_150([a, b, e, f, d, g], is_dual)
    } else {
        select_63([b, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_147([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_148([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_149([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_145([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_146([a, b, c, d, e, f, g, i], is_dual)
    } else {
        select_147([a, e, h, i, g, f, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_141([a, c, e, b, d, g, f, h, i], is_dual)
    } else {
        select_145([a, i, j, h, c, b, g, f, e], !is_dual)
    }
}
/// n = 12, i = 4
fn select_139([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_140([a, b, d, g, e, f, i, h, k, j], is_dual)
    } else {
        select_144([a, f, b, h, c, j, g, k, i, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_155([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_24([a, b, c, e, d, g], is_dual)
    } else {
        select_24([a, f, g, e, d, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_156([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_24([a, f, b, e, d, g], is_dual)
    } else {
        select_63([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_154([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_155([a, b, e, g, h, i, j], is_dual)
    } else {
        select_156([a, c, d, f, g, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_158([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_71([b, f, d, a, h], is_dual)
    } else {
        select_116([b, c, a, d, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_157([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_158([a, c, b, f, e, h, i, g], is_dual)
    } else {
        select_158([a, d, b, f, e, g, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_153([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_154([a, f, c, d, g, h, e, j, i, k], is_dual)
    } else {
        select_157([a, d, b, e, g, f, j, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_161([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_49([b, a, c, d, e, f, g], is_dual)
    } else {
        select_82([a, e, c, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_160([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_149([d, b, c, g, f, e, h], is_dual)
    } else {
        select_161([a, c, e, f, h, d, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_163([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_55([a, b, d, e, h, i], is_dual)
    } else {
        select_55([a, c, d, f, g, i], is_dual)
    }
}
/// n = 6, i = 2
fn select_164([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_24([a, b, c, d, e, f], is_dual)
    } else {
        select_28([a, f, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_162([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_163([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_164([a, g, c, f, d, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_159([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_160([a, b, d, e, f, h, i, k, j], is_dual)
    } else {
        select_162([a, d, c, e, h, g, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_152([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_153([a, g, b, d, f, i, h, e, j, k, l], is_dual)
    } else {
        select_159([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_151([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_152([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_152([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_138([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_139([a, b, e, c, f, d, g, h, i, j, k, l], is_dual)
    } else {
        select_151([a, b, d, e, f, c, g, h, i, j, k, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_108([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < b) || (is_dual && l > b) {
        select_109([a, c, d, b, e, h, f, g, i, j, k, l, m, o], is_dual)
    } else {
        select_138([j, a, c, d, e, h, l, i, b, k, m, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_16([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_17([a, b, c, d, e, k, h, j, i, l, m, n, o, p], is_dual)
    } else {
        select_108([a, b, d, e, f, g, i, h, j, l, m, k, n, o, p], is_dual)
    }
}
/// n = 8, i = 2
fn select_172([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([c, d, e, f], is_dual)
    } else {
        select_82([a, b, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_171([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_91([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_172([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_170([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_171([a, c, d, e, f, b, g, h, i], is_dual)
    } else {
        select_171([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_174([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_131([a, h, c, d, e, f, i, k], is_dual)
    } else {
        select_91([b, f, c, d, e, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_176([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_34([c, d, e, i], is_dual)
    } else {
        select_116([a, b, h, f, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_175([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_176([a, b, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_172([a, h, c, d, e, f, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_173([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_174([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_175([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_169([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_170([a, f, d, e, g, h, i, k, m], is_dual)
    } else {
        select_173([b, c, a, e, i, g, h, f, j, n, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_180([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, e, c, f, d, h], is_dual)
    } else {
        select_150([b, c, d, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_179([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_56([a, g, b, h, f, j], is_dual)
    } else {
        select_180([a, c, d, e, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_178([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_157([a, d, c, e, g, f, j, i, h], is_dual)
    } else {
        select_179([a, f, b, d, h, e, g, j, k, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_183([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_24([d, b, c, e, f, g], is_dual)
    } else {
        select_43([a, b, d, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_182([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_183([b, d, c, e, f, g, h, j], is_dual)
    } else {
        select_56([a, c, d, f, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_184([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_183([a, g, c, e, f, i, h, k], is_dual)
    } else {
        select_46([a, b, d, h, e, g, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_181([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_182([a, b, c, e, g, f, k, j, n, m], is_dual)
    } else {
        select_184([b, a, d, e, g, h, j, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_177([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_178([a, c, d, g, f, h, j, k, i, l, m], is_dual)
    } else {
        select_181([a, b, d, c, e, h, g, f, i, k, j, m, n, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_168([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_169([c, d, a, e, f, g, b, h, i, k, j, m, l, n], is_dual)
    } else {
        select_177([c, d, b, e, f, a, h, i, g, j, k, l, n, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_189([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_24([f, g, e, c, d, a], !is_dual)
    } else {
        select_28([e, b, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_114([a, b, c, d, e, f, g], is_dual)
    } else {
        select_189([c, b, e, f, g, a, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_187([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_188([a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_89([i, f, j, b, a, d], !is_dual)
    }
}
/// n = 6, i = 1
fn select_192([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_63([a, b, f, d, e], is_dual)
    } else {
        select_63([a, c, e, d, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_191([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([b, c, d, a, e, f, g], is_dual)
    } else {
        select_192([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_190([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_191([a, c, e, b, h, f, g], is_dual)
    } else {
        select_191([a, b, d, c, h, g, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_186([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_187([a, b, d, g, e, f, h, i, j, k], is_dual)
    } else {
        select_190([a, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 5, i = 1
fn select_196([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_63([b, a, c, d, e], is_dual)
    } else {
        select_25([b, e, d, a], is_dual)
    }
}
/// n = 11, i = 4
fn select_195([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_196([b, d, g, f, i], is_dual)
    } else {
        select_56([a, h, c, j, e, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_197([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_62([b, d, e, h, g, j], is_dual)
    } else {
        select_196([a, c, i, f, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_194([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_195([a, c, b, e, h, g, f, i, j, k, l], is_dual)
    } else {
        select_197([c, b, e, a, d, g, f, i, h, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_62([e, b, c, d, f, g], is_dual)
    } else {
        select_39([e, h, i, a, f, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_198([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([b, f, d, e, g, h], is_dual)
    } else {
        select_199([a, c, b, g, e, f, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_193([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_194([a, c, d, b, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_198([a, d, g, e, f, b, h, j, i, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_185([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_186([b, a, e, c, g, f, h, j, k, m, i], is_dual)
    } else {
        select_193([a, c, d, b, e, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_167([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_168([a, b, d, e, c, f, g, i, h, k, j, m, l, n], is_dual)
    } else {
        select_185([a, b, c, d, e, g, j, i, k, h, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_205([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_49([a, b, c, e, f, g, h], is_dual)
    } else {
        select_31([b, c, d], is_dual)
    }
}
/// n = 9, i = 2
fn select_204([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_205([a, b, c, d, f, g, h, i], is_dual)
    } else {
        select_205([a, b, c, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_206([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_54([a, c, e, f, d, h], is_dual)
    } else {
        select_42([a, d, b, g, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_204([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_206([a, i, b, g, f, j, h, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_208([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_172([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_59([a, b, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 4, i = 1
fn select_210([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_82([a, b, c, d], is_dual)
    } else {
        select_82([a, b, d, c], is_dual)
    }
}
/// n = 7, i = 3
fn select_209([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_210([b, c, d, e], is_dual)
    } else {
        select_39([a, b, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_207([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_208([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_209([a, b, c, g, j, h, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_202([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_203([b, c, d, e, f, g, h, a, i, j, k], is_dual)
    } else {
        select_207([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 1
fn select_213([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_69([a, c, d, e, f, h], is_dual)
    } else {
        select_29([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 1
fn select_212([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_213([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_213([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_216([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_116([a, d, b, g, e, f, h], is_dual)
    } else {
        select_116([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_215([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_148([e, b, d, a, f, g, h, i], is_dual)
    } else {
        select_216([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_214([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_215([a, d, b, e, c, f, g, h, i], is_dual)
    } else {
        select_215([a, d, c, e, b, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_211([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_212([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_214([a, b, c, i, d, h, g, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_201([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_202([a, b, d, c, e, i, g, h, k, l, j], is_dual)
    } else {
        select_211([b, a, c, f, e, h, g, i, j, m, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_221([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_30([b, c, d, e, g, f], is_dual)
    } else {
        select_47([a, b, c, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_220([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_221([a, b, c, d, e, h, g, i], is_dual)
    } else {
        select_131([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_222([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_62([b, a, d, e, f, g], is_dual)
    } else {
        select_56([a, c, b, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_219([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_220([b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_222([a, b, h, c, d, g, j, i, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_224([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_150([d, f, g, a, e, b], !is_dual)
    } else {
        select_71([e, f, d, b, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_223([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_69([b, c, d, e, f, g], is_dual)
    } else {
        select_224([e, h, i, a, f, g, b], !is_dual)
    }
}
/// n = 11, i = 3
fn select_218([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_219([a, c, b, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_223([a, h, e, f, g, b, i, k, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_228([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_30([b, c, d, e, g, h], is_dual)
    } else {
        select_63([a, d, e, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_227([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_228([a, d, e, b, c, f, h, g, i], is_dual)
    } else {
        select_127([a, g, d, e, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_226([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_223([a, g, d, e, f, b, h, j, i], is_dual)
    } else {
        select_227([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 6, i = 2
fn select_230([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_54([b, c, a, d, e, f], is_dual)
    } else {
        select_55([a, c, b, e, d, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_231([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_205([b, c, d, e, f, a, g, h], is_dual)
    } else {
        select_172([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_229([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_230([a, b, g, f, c, h], is_dual)
    } else {
        select_231([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_225([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_226([a, b, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_229([g, a, c, b, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_217([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_218([a, b, d, e, c, f, g, i, h, j, k], is_dual)
    } else {
        select_225([a, b, d, h, e, g, c, i, j, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_200([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_201([a, b, c, d, i, f, g, h, j, k, l, n, m], is_dual)
    } else {
        select_217([b, a, c, e, k, f, g, j, i, m, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_166([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_167([a, c, d, b, e, j, g, h, i, l, m, k, n, o], is_dual)
    } else {
        select_200([b, d, c, a, e, f, h, g, i, j, k, m, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_238([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_82([b, c, e, g], is_dual)
    } else {
        select_47([a, d, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_237([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_172([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_238([a, b, c, h, f, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_236([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_237([a, b, c, e, f, g, h, i, j, l], is_dual)
    } else {
        select_46([a, d, e, f, g, i, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_241([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, c, b, f, g, e], is_dual)
    } else {
        select_24([a, d, b, e, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_240([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_241([b, c, d, e, f, h, g], is_dual)
    } else {
        select_42([a, c, g, f, e, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_243([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_25([b, c, d, e], is_dual)
    } else {
        select_28([a, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_242([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_243([a, c, d, e, f, g, h], is_dual)
    } else {
        select_82([b, f, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_239([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_240([a, b, d, c, f, e, g, h, i], is_dual)
    } else {
        select_242([a, b, e, f, c, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_235([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_236([b, d, e, c, a, f, g, i, h, j, l, k], is_dual)
    } else {
        select_239([b, d, e, h, a, g, i, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_247([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_82([a, b, c, d], is_dual)
    } else {
        select_28([d, b, e], is_dual)
    }
}
/// n = 6, i = 2
fn select_246([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_77([a, c, b, d, e], is_dual)
    } else {
        select_247([a, c, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_245([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_123([a, d, e, b, c, g, f, h], is_dual)
    } else {
        select_246([a, g, b, c, h, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_244([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_187([a, b, c, g, e, f, h, i, j, k], is_dual)
    } else {
        select_245([a, c, e, d, f, h, g, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_234([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_235([b, d, c, a, h, f, g, i, j, l, m, k], is_dual)
    } else {
        select_244([a, c, b, e, g, i, h, k, l, n, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_251([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_216([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_98([a, b, e, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_252([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_54([d, g, a, e, f, b], !is_dual)
    } else {
        select_189([e, g, d, f, a, b, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_250([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_251([a, e, c, d, f, g, h, j, i, k], is_dual)
    } else {
        select_252([a, b, i, j, k, e, h], is_dual)
    }
}
/// n = 3, i = 1
fn select_256([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_28([a, b, c], is_dual)
    } else {
        select_28([a, c, b], is_dual)
    }
}
/// n = 5, i = 2
fn select_255([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_256([a, d, e], is_dual)
    } else {
        select_25([a, b, c, d], is_dual)
    }
}
/// n = 6, i = 2
fn select_254([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_255([e, f, a, b, c], !is_dual)
    } else {
        select_210([b, c, d, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_258([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, d, e, g, f, h], is_dual)
    } else {
        select_39([h, i, j, a, e, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_257([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_255([i, e, a, b, h], !is_dual)
    } else {
        select_258([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_253([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_254([a, c, d, f, h, g], is_dual)
    } else {
        select_257([a, b, c, d, e, g, h, i, j, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_249([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_250([a, b, d, c, f, g, h, i, e, j, k], is_dual)
    } else {
        select_253([a, b, c, e, f, i, g, h, k, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_261([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_33([b, d, e, f, k], is_dual)
    } else {
        select_59([a, c, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_263([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, e, b, d, g, f], is_dual)
    } else {
        select_47([a, c, f, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_262([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_196([b, a, d, g, h], is_dual)
    } else {
        select_263([a, c, b, e, f, h, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_260([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_261([c, b, d, e, f, g, h, i, j, l, m, k], is_dual)
    } else {
        select_262([c, a, d, j, h, g, i, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_265([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_161([a, c, e, f, g, d, h, i], is_dual)
    } else {
        select_48([c, b, d, f, e, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_267([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_256([a, g, h], is_dual)
    } else {
        select_116([a, c, b, d, e, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_266([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_267([a, h, c, e, f, i, g, j], is_dual)
    } else {
        select_46([a, b, d, g, e, h, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_264([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_265([c, e, d, f, a, g, h, i, j], is_dual)
    } else {
        select_266([c, b, d, a, f, g, h, e, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_259([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_260([c, d, a, b, e, f, h, g, i, j, l, m, k], is_dual)
    } else {
        select_264([a, d, b, c, j, g, h, k, l, m, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_248([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_249([b, c, d, i, a, g, h, j, l, m, k], is_dual)
    } else {
        select_259([b, c, d, e, a, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_233([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_234([a, b, c, e, d, f, i, h, j, g, k, m, l, n], is_dual)
    } else {
        select_248([b, d, a, e, c, f, g, i, j, h, k, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_273([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_39([a, c, d, e, h, j], is_dual)
    } else {
        select_39([b, c, d, f, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_274([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_55([a, d, c, g, e, h], is_dual)
    } else {
        select_82([e, f, b, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_272([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_273([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_274([i, j, e, h, a, g, f, b], !is_dual)
    }
}
/// n = 12, i = 4
fn select_271([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_272([a, b, d, e, f, g, j, i, k, l], is_dual)
    } else {
        select_142([a, c, d, e, f, h, i, j, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_276([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_54([e, h, a, f, g, b], !is_dual)
    } else {
        select_46([a, b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_275([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_276([a, b, d, e, f, i, h, j], is_dual)
    } else {
        select_142([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 4
fn select_270([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_271([a, b, f, d, j, g, h, i, k, l, n, m], is_dual)
    } else {
        select_275([a, c, f, e, k, g, i, j, m, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_280([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_24([a, c, b, e, f, g], is_dual)
    } else {
        select_47([a, c, d, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_279([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_280([b, e, d, g, f, h, i], is_dual)
    } else {
        select_280([a, e, c, h, f, g, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_278([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_113([b, d, c, f, e, h, i, g, j], is_dual)
    } else {
        select_279([a, b, g, d, e, h, f, i, j, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_282([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_89([c, d, g, e, f, a], !is_dual)
    } else {
        select_56([c, e, g, d, f, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_281([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_230([b, c, d, f, e, g], is_dual)
    } else {
        select_282([h, g, b, f, a, e, d], !is_dual)
    }
}
/// n = 11, i = 3
fn select_277([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_278([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_281([a, g, b, h, j, i, f, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_269([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_270([a, c, d, b, e, f, g, h, i, k, j, m, l, n], is_dual)
    } else {
        select_277([a, d, j, e, f, g, b, i, k, l, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_286([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_114([a, f, b, d, e, h, g], is_dual)
    } else {
        select_116([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_285([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_61([c, d, e, a, i, g, h], is_dual)
    } else {
        select_286([a, b, e, c, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_287([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_205([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_161([a, b, h, f, g, e, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_284([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_285([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_287([a, c, b, h, f, g, j, k, i, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_290([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, h, d, e, f, g, i, k], is_dual)
    } else {
        select_280([b, d, c, g, f, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_292([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([b, d, c, g, f, i], is_dual)
    } else {
        select_43([a, c, e, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_291([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_130([b, d, c, g, h, f, i], is_dual)
    } else {
        select_292([a, b, f, d, e, h, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_289([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_290([b, d, e, a, f, c, g, h, j, i, k], is_dual)
    } else {
        select_291([b, d, c, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_294([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_23([b, c, d, e, g, f, h], is_dual)
    } else {
        select_42([a, b, c, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_296([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_39([a, b, c, e, h, i], is_dual)
    } else {
        select_63([d, b, c, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_295([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_280([b, e, d, i, g, h, k], is_dual)
    } else {
        select_296([a, c, h, e, f, g, i, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_293([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_294([c, h, d, e, a, g, i, j], is_dual)
    } else {
        select_295([b, c, a, d, e, f, g, i, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_288([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_289([a, c, d, b, g, f, h, j, k, i, l], is_dual)
    } else {
        select_293([b, c, a, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_283([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_284([c, b, a, d, e, h, f, i, j, g, k, m], is_dual)
    } else {
        select_288([a, b, c, g, e, h, i, j, f, k, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_268([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_269([b, c, d, e, a, f, g, h, j, k, i, m, l, n], is_dual)
    } else {
        select_283([b, c, d, i, f, g, a, h, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_232([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_233([a, c, b, d, e, j, g, h, i, l, m, k, n, o], is_dual)
    } else {
        select_268([b, d, c, a, e, f, h, g, i, j, k, m, l, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_165([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_166([c, d, e, b, f, g, h, a, i, j, k, l, m, o, n], is_dual)
    } else {
        select_232([c, d, e, a, f, g, h, b, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_15([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_16([b, c, d, a, e, g, h, f, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_165([a, c, d, b, e, f, g, i, j, k, l, n, o, m, p], is_dual)
    }
}
/// n = 7, i = 2
fn select_305([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_49([a, b, c, d, e, f, g], is_dual)
    } else {
        select_49([a, b, c, e, d, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_304([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_38([a, d, c, e, h, i], is_dual)
    } else {
        select_305([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_303([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_282([g, j, b, f, a, i, c], !is_dual)
    } else {
        select_304([h, j, f, a, i, g, b, e, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_307([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_247([a, f, c, g, e], is_dual)
    } else {
        select_161([g, i, f, h, a, d, b, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_308([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_54([d, c, a, f, e, g], is_dual)
    } else {
        select_161([b, c, a, e, g, d, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_306([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_307([f, h, j, g, i, a, c, e, d], !is_dual)
    } else {
        select_308([b, c, d, g, e, h, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_302([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_303([a, b, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_306([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_311([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_224([a, b, c, d, f, g, e], is_dual)
    } else {
        select_224([a, c, b, d, e, g, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_312([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_81([f, a, b, d, e], is_dual)
    } else {
        select_81([e, a, c, d, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_310([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_311([b, c, d, a, e, f, g], is_dual)
    } else {
        select_312([a, c, d, b, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_315([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_150([b, c, d, e, f, g], is_dual)
    } else {
        select_55([a, c, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_314([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_81([e, a, b, h, i], is_dual)
    } else {
        select_315([c, a, b, d, f, h, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_317([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_71([c, d, g, h, f], is_dual)
    } else {
        select_71([a, b, i, j, e], is_dual)
    }
}
/// n = 11, i = 5
fn select_316([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_81([g, a, b, j, k], is_dual)
    } else {
        select_317([a, b, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_313([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_314([i, j, h, f, e, g, d, b, c], !is_dual)
    } else {
        select_316([a, d, b, c, g, f, h, i, j, e, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_309([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_310([a, c, e, f, i, j, h], is_dual)
    } else {
        select_313([k, i, j, l, a, g, h, b, e, f, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_301([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_302([a, d, f, c, g, b, h, l, i, k], is_dual)
    } else {
        select_309([a, d, c, f, b, e, h, g, j, l, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_322([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_24([a, c, d, f, e, h], is_dual)
    } else {
        select_24([b, d, c, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_321([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_322([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_189([g, h, a, f, e, b, d], !is_dual)
    }
}
/// n = 7, i = 3
fn select_324([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_24([d, f, g, e, a, b], !is_dual)
    } else {
        select_24([d, f, g, e, a, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_323([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_41([a, b, e, d, g, f], is_dual)
    } else {
        select_324([g, e, h, a, f, c, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_320([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_321([d, h, e, i, f, g, a, b], !is_dual)
    } else {
        select_323([d, h, f, i, e, g, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_326([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_114([b, c, d, a, e, f, g], is_dual)
    } else {
        select_114([b, c, e, a, d, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_325([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_326([e, a, d, b, c, f, g], is_dual)
    } else {
        select_143([g, a, h, b, c, f, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_319([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_320([e, b, c, f, d, h, g, j, k], is_dual)
    } else {
        select_325([d, i, j, h, k, e, g, a], !is_dual)
    }
}
/// n = 8, i = 3
fn select_329([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_241([a, b, c, d, e, f, g], is_dual)
    } else {
        select_41([a, e, f, h, g, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_328([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_329([a, c, b, d, f, g, e, h], is_dual)
    } else {
        select_40([a, c, e, g, b, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_327([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_328([e, b, a, c, d, f, g, h], is_dual)
    } else {
        select_328([e, b, a, d, c, f, h, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_318([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_319([l, i, j, a, k, b, h, e, c, f, g], !is_dual)
    } else {
        select_327([h, a, d, e, b, i, g, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_300([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_301([a, b, c, e, d, f, h, i, g, k, j, l], is_dual)
    } else {
        select_318([b, c, e, a, d, f, g, i, k, l, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_335([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_49([a, b, c, d, e, f, g], is_dual)
    } else {
        select_47([e, b, c, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_334([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_54([d, c, a, f, e, g], is_dual)
    } else {
        select_335([c, a, b, e, g, d, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_333([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_334([a, e, b, g, d, i, f, h], is_dual)
    } else {
        select_47([f, c, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_338([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_243([a, b, d, e, h, i, j], is_dual)
    } else {
        select_55([b, c, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_337([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_143([d, h, j, a, g, f, c], !is_dual)
    } else {
        select_338([b, a, c, e, d, f, g, h, i, k], is_dual)
    }
}
/// n = 7, i = 3
fn select_340([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_24([e, d, g, f, a, b], !is_dual)
    } else {
        select_71([b, c, d, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_339([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_340([i, f, j, g, d, e, b], !is_dual)
    } else {
        select_224([f, a, c, h, d, g, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_336([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_337([a, b, c, e, d, f, g, i, j, h, k], is_dual)
    } else {
        select_339([i, k, g, h, j, e, d, a, b, f], !is_dual)
    }
}
/// n = 11, i = 5
fn select_332([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_333([i, g, k, d, j, h, e, a, b], !is_dual)
    } else {
        select_336([b, a, c, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_344([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_82([g, h, d, a], !is_dual)
    } else {
        select_71([f, i, b, e, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_343([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_344([i, g, j, h, a, f, b, c, e], !is_dual)
    } else {
        select_83([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_346([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, d, e, g, f, h], is_dual)
    } else {
        select_82([a, h, b, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_345([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_210([a, b, h, i], is_dual)
    } else {
        select_346([c, d, a, b, e, f, i, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_342([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_343([a, c, d, b, f, g, i, e, h, j], is_dual)
    } else {
        select_345([c, d, b, a, e, g, i, f, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_348([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_89([b, c, d, f, e, g], is_dual)
    } else {
        select_41([f, h, i, d, a, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_347([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_230([h, e, j, a, f, b], !is_dual)
    } else {
        select_348([i, h, j, e, a, f, g, c, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_341([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_342([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_347([b, a, d, e, f, c, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_331([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < d) || (is_dual && j > d) {
        select_332([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_341([h, g, i, d, k, e, j, a, b, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_352([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_77([b, c, d, e, g], is_dual)
    } else {
        select_46([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_353([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_89([b, a, c, d, e, g], is_dual)
    } else {
        select_247([e, a, b, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_351([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_352([a, c, d, e, g, f, j, h, i], is_dual)
    } else {
        select_353([a, b, h, f, i, g, k], is_dual)
    }
}
/// n = 11, i = 5
fn select_354([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_337([h, i, k, g, d, j, e, a, f, b, c], !is_dual)
    } else {
        select_337([h, i, k, g, e, j, d, b, f, a, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_350([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_351([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_354([b, c, d, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_357([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_89([b, c, d, a, e, f], is_dual)
    } else {
        select_196([b, a, d, c, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_359([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_39([a, b, c, d, f, g], is_dual)
    } else {
        select_82([a, e, g, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_358([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_359([g, h, i, a, b, f, d], !is_dual)
    } else {
        select_98([h, g, i, a, b, e, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_356([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_357([a, b, h, e, j, g], is_dual)
    } else {
        select_358([a, c, b, d, g, f, i, j, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_362([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_82([a, g, i, h], !is_dual)
    } else {
        select_116([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_361([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_183([i, j, g, h, f, a, d, b], !is_dual)
    } else {
        select_362([g, j, i, f, h, d, a, e, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_363([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_77([a, b, d, h, j], is_dual)
    } else {
        select_196([c, e, g, f, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_360([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_361([c, b, e, g, f, a, i, h, j, k], is_dual)
    } else {
        select_363([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_355([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_356([a, d, c, b, e, f, h, g, j, i], is_dual)
    } else {
        select_360([a, d, b, e, c, g, h, f, j, i, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_350([b, c, d, e, f, a, g, h, j, i, k, l], is_dual)
    } else {
        select_355([b, d, a, e, f, g, c, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_330([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_331([g, j, l, b, h, k, a, c, d, i, e], !is_dual)
    } else {
        select_349([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_299([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_300([a, b, c, e, d, f, l, n, i, o, m, k], is_dual)
    } else {
        select_330([b, a, c, e, g, h, k, m, j, l, n, o], is_dual)
    }
}
/// n = 9, i = 2
fn select_369([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_118([b, f, c, d, e, g, h], is_dual)
    } else {
        select_131([a, g, c, d, e, f, h, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_368([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_171([b, c, d, e, f, g, a, h, i], is_dual)
    } else {
        select_369([b, a, d, e, f, g, c, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_371([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([b, c, d, a, e, f, h], is_dual)
    } else {
        select_23([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_370([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_246([b, e, c, a, g, f], is_dual)
    } else {
        select_371([a, b, d, c, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_367([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_368([c, d, b, a, e, f, g, h, i], is_dual)
    } else {
        select_370([b, c, d, h, a, g, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_375([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_24([a, b, c, f, e, g], is_dual)
    } else {
        select_24([a, b, d, f, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_374([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_375([b, c, a, d, f, e, g], is_dual)
    } else {
        select_98([b, e, c, a, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_376([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_23([b, c, d, a, e, f, h], is_dual)
    } else {
        select_23([a, d, c, b, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_373([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_374([a, b, d, e, f, g, h, j], is_dual)
    } else {
        select_376([a, c, d, e, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_377([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_213([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_137([b, c, d, e, f, a, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_372([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_373([a, b, c, i, d, h, g, j, k, l], is_dual)
    } else {
        select_377([c, a, d, e, f, g, h, i, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_366([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_367([h, c, d, e, a, g, i, k, l, j], is_dual)
    } else {
        select_372([a, b, c, f, d, i, g, h, j, m, k, n], is_dual)
    }
}
/// n = 11, i = 4
fn select_381([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_156([b, d, e, f, g, h, j], is_dual)
    } else {
        select_156([a, c, f, e, g, i, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_382([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_183([a, e, c, d, f, g, i, j], is_dual)
    } else {
        select_42([b, c, g, f, e, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_380([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_381([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    } else {
        select_382([b, c, g, e, f, a, i, j, h, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_384([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_130([a, b, c, d, e, f, g], is_dual)
    } else {
        select_63([e, c, d, b, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_385([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_322([f, h, i, c, d, g, a, e], !is_dual)
    } else {
        select_98([f, g, i, c, d, e, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_383([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_384([c, a, d, e, f, g, h], is_dual)
    } else {
        select_385([h, j, a, e, i, b, f, c, g], !is_dual)
    }
}
/// n = 13, i = 5
fn select_379([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_380([c, b, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_383([a, b, c, e, f, i, h, j, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_389([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_39([a, b, c, f, i, j], is_dual)
    } else {
        select_30([b, c, d, e, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_388([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_67([h, d, e, a, g, j], is_dual)
    } else {
        select_389([b, a, c, d, e, f, g, h, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_390([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_255([b, f, e, a, g], is_dual)
    } else {
        select_46([b, c, a, d, e, f, h, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_387([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_388([b, c, f, d, e, g, h, i, j, l, k], is_dual)
    } else {
        select_390([a, c, b, f, g, j, k, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_392([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_77([d, e, a, g, f], !is_dual)
    } else {
        select_89([a, b, c, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_391([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_123([a, d, e, b, c, g, f, h], is_dual)
    } else {
        select_392([a, f, i, b, g, h, c], !is_dual)
    }
}
/// n = 14, i = 5
fn select_386([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_387([c, a, d, e, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_391([a, b, j, e, f, g, i, l, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_378([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_379([a, b, c, e, i, g, h, d, j, k, m, l, n], is_dual)
    } else {
        select_386([a, b, c, e, d, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_365([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < d) || (is_dual && b > d) {
        select_366([a, c, e, f, g, d, h, b, i, j, k, l, m, n], is_dual)
    } else {
        select_378([a, c, f, e, g, b, h, i, d, j, k, m, l, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_397([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_280([b, c, d, e, f, g, i], is_dual)
    } else {
        select_42([a, c, g, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_398([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([b, c, d, e, g, f, h], is_dual)
    } else {
        select_322([a, b, f, c, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_396([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_397([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_398([b, c, e, a, f, d, g, i, h], is_dual)
    }
}
/// n = 7, i = 2
fn select_400([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_77([b, c, d, e, f], is_dual)
    } else {
        select_70([a, b, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_399([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_400([b, d, e, f, g, i, h], is_dual)
    } else {
        select_353([j, k, g, b, a, c, h], !is_dual)
    }
}
/// n = 14, i = 5
fn select_395([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_396([c, e, b, f, g, h, j, l, k], is_dual)
    } else {
        select_399([a, b, d, c, f, g, i, k, j, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_403([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_359([a, b, e, f, h, d, g], is_dual)
    } else {
        select_98([a, d, c, g, f, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_402([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_53([b, d, a, e, c, g, f, h], is_dual)
    } else {
        select_403([b, c, d, a, e, g, f, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_405([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_67([b, c, d, e, f, g], is_dual)
    } else {
        select_359([a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_404([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_405([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_101([e, h, f, a, b, g], !is_dual)
    }
}
/// n = 8, i = 3
fn select_401([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_402([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_404([a, b, c, d, f, g, h, e], is_dual)
    }
}
/// n = 14, i = 5
fn select_394([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_395([b, d, c, f, e, g, a, h, i, k, j, n, l, m], is_dual)
    } else {
        select_401([b, d, f, j, a, i, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_409([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_54([b, c, d, f, e, g], is_dual)
    } else {
        select_247([g, a, b, d, f], is_dual)
    }
}
/// n = 9, i = 2
fn select_408([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_409([b, h, c, a, f, i, g], is_dual)
    } else {
        select_90([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_411([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_54([b, c, d, f, e, g], is_dual)
    } else {
        select_89([d, f, h, a, g, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_410([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_90([b, c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_411([b, h, c, a, f, i, g, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_407([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_408([c, d, f, a, b, h, j, i, l], is_dual)
    } else {
        select_410([a, b, e, c, d, g, j, k, m, n], is_dual)
    }
}
/// n = 11, i = 4
fn select_414([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_77([c, d, f, g, i], is_dual)
    } else {
        select_38([a, b, e, h, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_415([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_241([a, c, b, d, e, f, g], is_dual)
    } else {
        select_196([a, d, e, f, b], is_dual)
    }
}
/// n = 11, i = 4
fn select_413([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_414([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    } else {
        select_415([d, e, g, f, c, i, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_416([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_54([f, h, a, b, g, c], !is_dual)
    } else {
        select_101([a, c, d, f, e, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_412([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_413([a, b, c, e, f, d, g, h, i, j, k], is_dual)
    } else {
        select_416([a, b, c, h, d, g, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_406([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_407([a, b, c, e, f, d, h, g, j, i, k, l, m, n], is_dual)
    } else {
        select_412([a, b, c, e, d, i, j, g, l, k, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_393([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_394([b, c, e, d, f, a, g, i, h, k, j, n, m, l], is_dual)
    } else {
        select_406([b, d, e, f, g, c, i, h, a, j, k, l, n, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_364([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_365([b, c, d, e, f, a, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_393([a, b, d, e, c, g, f, h, i, k, l, m, j, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_298([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_299([b, a, d, c, f, e, g, h, j, k, l, m, i, n, o], is_dual)
    } else {
        select_364([b, c, d, a, e, f, g, i, k, m, j, n, l, o], is_dual)
    }
}
/// n = 7, i = 3
fn select_423([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_255([b, c, e, d, f], is_dual)
    } else {
        select_25([f, g, a, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_424([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_62([c, b, d, e, f, g], is_dual)
    } else {
        select_101([a, b, c, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_422([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_423([c, b, g, a, f, i, h], is_dual)
    } else {
        select_424([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_426([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_42([g, h, f, a, b, c], !is_dual)
    } else {
        select_98([f, h, g, a, b, e, d], !is_dual)
    }
}
/// n = 7, i = 2
fn select_427([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_375([b, c, a, d, f, e, g], is_dual)
    } else {
        select_42([b, e, c, a, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_425([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_426([a, b, c, d, g, h, f, i], is_dual)
    } else {
        select_427([a, c, e, d, f, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_421([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_422([a, d, c, b, g, i, f, j, h], is_dual)
    } else {
        select_425([a, c, d, f, e, h, g, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_430([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_322([e, h, g, i, d, f, b, c], !is_dual)
    } else {
        select_127([e, g, h, i, d, f, a, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_432([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_49([a, c, d, e, g, f, i], is_dual)
    } else {
        select_82([a, h, b, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_431([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_242([a, b, d, f, e, h, g, i], is_dual)
    } else {
        select_432([a, c, b, d, f, g, h, i, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_429([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_430([c, e, d, a, h, g, f, i, j], is_dual)
    } else {
        select_431([b, c, a, d, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_434([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_161([c, b, d, e, a, f, h, g], is_dual)
    } else {
        select_238([c, a, b, d, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_433([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_230([h, e, j, a, f, b], !is_dual)
    } else {
        select_434([e, b, c, d, f, g, i, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_428([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_429([a, b, d, e, f, j, h, i, l, k], is_dual)
    } else {
        select_433([a, c, d, e, i, g, h, k, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_420([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_421([b, d, a, e, f, g, h, j, k, i], is_dual)
    } else {
        select_428([a, d, c, b, e, g, f, i, j, k, l, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_438([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_54([b, c, d, i, f, h], is_dual)
    } else {
        select_158([h, b, a, d, e, g, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_440([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_150([a, c, g, f, d, i], is_dual)
    } else {
        select_116([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_441([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_49([a, b, g, e, f, i, j], is_dual)
    } else {
        select_30([b, f, c, d, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_439([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_440([f, b, d, a, e, h, g, j, i], is_dual)
    } else {
        select_441([b, d, a, c, e, g, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_437([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_438([a, b, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_439([e, a, d, c, f, g, h, j, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_444([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, b, f, d, g, e, h], is_dual)
    } else {
        select_49([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_445([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_24([f, c, b, e, h, g], is_dual)
    } else {
        select_55([a, c, d, g, f, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_443([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_444([b, c, d, f, h, g, i, k], is_dual)
    } else {
        select_445([a, b, g, e, f, h, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_446([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_205([c, a, b, d, e, f, g, h], is_dual)
    } else {
        select_114([a, b, e, d, f, g, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_442([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_443([b, a, c, d, f, h, g, i, k, j, m, l], is_dual)
    } else {
        select_446([a, j, b, e, f, i, h, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_436([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_437([c, f, d, e, a, g, h, k, i, j, l], is_dual)
    } else {
        select_442([b, c, d, e, a, g, h, f, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_450([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_24([b, d, c, e, g, f], is_dual)
    } else {
        select_71([d, h, a, b, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_449([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_101([a, b, d, i, f, h], is_dual)
    } else {
        select_450([b, a, c, h, e, g, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_448([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_449([c, a, d, b, g, f, h, e, i, j], is_dual)
    } else {
        select_449([c, b, d, a, g, e, h, f, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_452([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_267([a, i, b, e, f, k, h, l], is_dual)
    } else {
        select_296([a, d, h, c, e, g, j, i, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_453([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_183([a, b, c, d, e, g, h, f], is_dual)
    } else {
        select_450([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_451([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < b) || (is_dual && j > b) {
        select_452([a, c, d, e, f, g, h, i, j, l, m, k], is_dual)
    } else {
        select_453([a, b, c, f, g, k, m, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_447([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_448([a, b, c, e, f, i, h, k, j, l], is_dual)
    } else {
        select_451([a, b, d, e, c, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_435([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_436([a, b, e, d, c, f, h, g, k, j, l, i, m], is_dual)
    } else {
        select_447([c, a, d, b, e, g, f, h, i, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_419([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_420([b, a, e, c, g, i, h, k, f, j, m, l], is_dual)
    } else {
        select_435([b, c, e, d, a, h, i, f, j, k, g, l, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_458([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_77([b, c, d, e, f], is_dual)
    } else {
        select_41([a, b, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_459([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_375([a, e, c, d, g, f, i], is_dual)
    } else {
        select_56([a, f, b, h, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_457([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_458([a, b, g, d, h, f, i], is_dual)
    } else {
        select_459([a, c, b, f, e, i, h, j, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_461([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_205([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_54([g, b, a, h, e, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_460([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_61([a, d, e, b, h, i, g], is_dual)
    } else {
        select_461([a, c, b, g, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_456([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_457([a, b, d, c, g, h, j, i, f, k], is_dual)
    } else {
        select_460([a, c, d, e, b, g, f, i, j, h, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_465([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_116([b, d, c, f, e, g, h], is_dual)
    } else {
        select_43([a, b, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_464([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_149([e, c, d, a, f, g, h], is_dual)
    } else {
        select_465([b, a, c, d, f, e, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_463([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_308([a, c, b, f, g, h, i, e], is_dual)
    } else {
        select_464([b, c, d, a, f, e, h, j, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_468([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_24([b, f, d, e, h, i], is_dual)
    } else {
        select_47([a, c, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_467([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_468([a, b, c, d, e, g, h, f, i, j], is_dual)
    } else {
        select_158([a, c, d, f, e, h, g, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_469([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([a, e, c, f, d, h], is_dual)
    } else {
        select_24([b, d, c, f, e, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_466([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_467([a, b, c, d, f, e, g, h, i, j], is_dual)
    } else {
        select_469([a, e, d, g, b, f, i, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_462([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_463([b, a, c, d, j, f, h, i, k, l], is_dual)
    } else {
        select_466([a, b, d, e, i, g, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_455([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_456([b, c, d, e, f, g, i, h, j, l, k], is_dual)
    } else {
        select_462([a, b, c, h, e, g, i, f, k, j, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_473([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_241([b, c, d, e, f, h, g], is_dual)
    } else {
        select_224([e, i, j, a, f, g, c], !is_dual)
    }
}
/// n = 8, i = 2
fn select_474([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_241([a, b, c, f, g, e, h], is_dual)
    } else {
        select_62([a, c, d, e, g, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_472([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_473([a, b, c, d, f, h, j, i, k, l], is_dual)
    } else {
        select_474([b, c, d, e, i, g, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_471([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_194([a, b, c, g, e, f, i, h, k, j, l, m], is_dual)
    } else {
        select_472([a, c, h, d, e, f, i, g, j, k, m, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_477([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_241([a, d, b, c, f, g, e], is_dual)
    } else {
        select_25([d, e, g, b], is_dual)
    }
}
/// n = 9, i = 3
fn select_479([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, b, d, g, f, i], is_dual)
    } else {
        select_25([c, d, e, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_478([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_192([c, d, e, i, f, g], is_dual)
    } else {
        select_479([a, b, d, c, f, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_476([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_477([a, b, j, c, f, i, l], is_dual)
    } else {
        select_478([b, c, f, d, e, g, h, i, k, j, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_481([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_62([f, e, c, d, h, g], is_dual)
    } else {
        select_156([a, b, g, e, f, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_480([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_481([a, b, c, h, e, f, g, i, j, k], is_dual)
    } else {
        select_481([a, b, d, g, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_475([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_476([b, c, a, d, e, f, h, i, g, k, j, l], is_dual)
    } else {
        select_480([b, g, d, e, f, a, h, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_470([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_471([a, c, b, d, e, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_475([b, a, c, d, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_454([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_455([a, b, d, e, c, f, g, i, h, j, m, k, l], is_dual)
    } else {
        select_470([a, b, c, d, e, f, g, h, i, k, m, l, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_418([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_419([c, d, f, a, e, g, b, i, h, k, j, m, l], is_dual)
    } else {
        select_454([c, d, f, b, e, g, a, i, h, k, m, l, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_487([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_359([a, b, c, d, f, e, g], is_dual)
    } else {
        select_39([d, f, g, a, b, e], !is_dual)
    }
}
/// n = 9, i = 3
fn select_486([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_215([a, c, b, d, e, f, g, i, h], is_dual)
    } else {
        select_487([a, b, f, g, h, i, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_490([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_39([a, b, c, d, e, f], is_dual)
    } else {
        select_39([a, b, c, d, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_491([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([g, h, f, d, a, b], !is_dual)
    } else {
        select_82([a, f, c, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_489([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_490([a, c, e, d, f, g, i], is_dual)
    } else {
        select_491([g, h, i, d, e, a, b, f], !is_dual)
    }
}
/// n = 11, i = 5
fn select_488([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_265([a, c, b, d, f, e, g, h, i], is_dual)
    } else {
        select_489([h, i, k, d, e, j, f, a, b], !is_dual)
    }
}
/// n = 12, i = 5
fn select_485([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_486([a, b, e, d, f, g, i, h, k], is_dual)
    } else {
        select_488([j, l, k, f, i, g, a, b, c, h, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_495([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, c, b, f, g, h], is_dual)
    } else {
        select_82([b, e, d, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_494([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_224([a, f, c, h, e, g, j], is_dual)
    } else {
        select_495([i, h, k, f, e, d, g, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_497([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, e, b, d, g, h], is_dual)
    } else {
        select_26([c, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_496([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_359([h, f, j, a, g, e, c], !is_dual)
    } else {
        select_497([a, b, e, d, f, h, i, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_493([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_494([a, c, b, d, e, h, g, i, f, k, j], is_dual)
    } else {
        select_496([a, c, b, d, e, f, h, i, g, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_500([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([g, h, i, d, a, b], !is_dual)
    } else {
        select_43([a, c, f, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_499([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_143([f, a, c, g, e, h, j], is_dual)
    } else {
        select_500([i, j, g, h, f, a, e, b, d], !is_dual)
    }
}
/// n = 8, i = 3
fn select_502([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_39([a, c, d, e, g, h], is_dual)
    } else {
        select_63([b, c, d, e, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_501([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_502([b, d, e, f, h, j, g, i], is_dual)
    } else {
        select_495([a, b, g, c, i, h, f, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_498([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_499([c, d, f, e, a, g, i, j, h, l], is_dual)
    } else {
        select_501([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_492([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_493([j, l, k, f, i, a, h, g, b, e, c], !is_dual)
    } else {
        select_498([a, b, c, e, d, f, h, i, j, g, l, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_484([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_485([c, b, d, e, f, a, g, h, i, j, k, l], is_dual)
    } else {
        select_492([c, a, d, e, f, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_507([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_256([a, e, f], is_dual)
    } else {
        select_63([a, b, c, d, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_506([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_280([a, b, c, d, e, g, f], is_dual)
    } else {
        select_507([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_505([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_506([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_384([a, b, d, e, c, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_509([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_131([b, c, a, d, e, f, g, h], is_dual)
    } else {
        select_42([b, g, c, a, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_508([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_509([a, b, c, d, g, h, f, i], is_dual)
    } else {
        select_509([a, b, c, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_504([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_505([a, c, b, f, g, i, j, k], is_dual)
    } else {
        select_508([b, a, c, d, e, f, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_512([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_444([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_98([e, a, f, d, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_513([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_469([a, b, d, f, g, e, i, j], is_dual)
    } else {
        select_56([a, e, c, h, g, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_511([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_512([b, a, c, e, d, f, g, h, i, k], is_dual)
    } else {
        select_513([g, d, j, k, i, e, h, a, b, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_515([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_54([h, i, a, e, g, f], !is_dual)
    } else {
        select_23([a, c, b, d, f, e, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_514([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_157([a, d, b, c, f, e, h, i, g], is_dual)
    } else {
        select_515([g, h, j, a, b, i, f, e, d], !is_dual)
    }
}
/// n = 11, i = 5
fn select_510([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_511([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_514([g, d, h, k, i, j, e, a, f, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_503([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_504([c, a, d, b, e, f, g, h, i, k, j], is_dual)
    } else {
        select_510([j, l, k, a, h, f, c, b, d, i, g], !is_dual)
    }
}
/// n = 13, i = 5
fn select_483([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_484([b, a, c, d, g, e, i, k, j, l, h, m], is_dual)
    } else {
        select_503([b, a, c, d, f, h, g, j, k, l, i, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_521([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([c, d, e, f], is_dual)
    } else {
        select_71([h, i, a, b, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_520([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_33([c, d, e, f, i], is_dual)
    } else {
        select_521([a, b, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_519([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_426([a, g, i, l, j, b, c, h], !is_dual)
    } else {
        select_520([b, c, d, a, e, f, g, h, k, i, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_523([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_149([a, b, c, d, e, f, g], is_dual)
    } else {
        select_359([a, b, e, d, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_524([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_221([a, d, e, b, c, f, g, h], is_dual)
    } else {
        select_280([a, f, d, e, b, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_522([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_523([i, g, k, a, j, b, c, h, d], !is_dual)
    } else {
        select_524([a, d, c, e, f, g, i, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_518([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_519([a, h, b, d, e, f, g, i, k, l, j, m], is_dual)
    } else {
        select_522([a, g, c, d, e, f, h, i, j, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_527([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_33([c, d, e, f, j], is_dual)
    } else {
        select_172([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_529([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_24([a, c, b, d, e, g], is_dual)
    } else {
        select_256([b, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_528([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_529([f, d, g, e, a, b, c], !is_dual)
    } else {
        select_529([f, e, g, d, a, b, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_526([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_527([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_528([b, c, h, a, i, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_530([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_524([a, c, b, e, f, h, j, i], is_dual)
    } else {
        select_358([a, b, d, c, g, i, j, k, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_525([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_526([a, b, h, d, e, f, g, i, k, j, l], is_dual)
    } else {
        select_530([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_517([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_518([a, d, c, e, f, g, h, b, i, j, k, l, m], is_dual)
    } else {
        select_525([a, d, b, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_516([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_331([h, k, m, b, i, l, a, c, d, j, e], !is_dual)
    } else {
        select_517([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_482([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_483([a, b, c, d, j, f, h, l, k, n, i, m, o], is_dual)
    } else {
        select_516([b, a, c, d, e, g, i, k, m, j, l, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_417([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_418([b, c, a, d, f, e, i, k, h, m, l, n, j], is_dual)
    } else {
        select_482([b, a, c, d, f, e, g, h, j, k, l, m, i, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_297([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_298([a, b, d, e, c, f, g, h, j, i, k, l, m, n, o], is_dual)
    } else {
        select_417([a, b, e, f, d, g, h, i, j, c, k, l, m, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_14([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_15([a, b, c, d, f, h, e, g, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_297([a, c, b, d, e, g, f, j, i, m, n, l, p, k, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_13([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_14([a, c, b, f, g, e, h, d, k, i, l, o, m, j, p, n], is_dual)
    } else {
        select_14([b, c, a, f, g, d, h, e, k, j, l, o, n, i, p, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_13([a, c, d, b, e, f, g, h, j, k, l, m, i, n, o, p], is_dual)
    } else {
        select_13([b, c, d, a, e, f, g, h, i, k, l, m, j, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_12([e, f, g, c, h, d, b, a, j, k, m, i, n, l, o, p], is_dual)
    } else {
        select_12([e, f, h, c, g, d, b, a, j, k, l, i, n, m, o, p], is_dual)
    }
}
/// n = 11, i = 3
fn select_542([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_221([b, d, g, e, f, i, h, j], is_dual)
    } else {
        select_221([a, c, h, e, f, i, g, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_542([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_129([c, g, d, e, f, a, h, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_543([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_458([a, b, c, d, f, g, h], is_dual)
    } else {
        select_77([b, c, d, e, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_540([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_541([a, c, b, f, d, e, g, h, i, k, j], is_dual)
    } else {
        select_543([c, d, e, a, h, g, j, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_546([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_62([a, c, b, d, e, f], is_dual)
    } else {
        select_56([a, f, c, b, e, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_545([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_546([a, e, b, d, g, f, h], is_dual)
    } else {
        select_191([a, b, c, f, e, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_548([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_130([a, d, c, h, g, e, j], is_dual)
    } else {
        select_479([a, d, b, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_549([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_280([a, b, c, h, e, g, j], is_dual)
    } else {
        select_149([b, g, d, e, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_547([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_548([c, e, b, d, a, f, g, h, i, j], is_dual)
    } else {
        select_549([c, a, d, e, b, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_544([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_545([b, a, f, e, c, g, i, h], is_dual)
    } else {
        select_547([a, b, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_539([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_540([c, e, d, a, b, f, g, i, h, k, j], is_dual)
    } else {
        select_544([c, a, b, e, d, i, g, j, k, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_554([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_24([a, b, f, h, e, i], is_dual)
    } else {
        select_63([e, c, d, g, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_555([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_24([a, b, d, f, e, h], is_dual)
    } else {
        select_49([c, a, b, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_553([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_554([a, c, b, d, f, e, g, h, i], is_dual)
    } else {
        select_555([a, c, e, b, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_552([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_553([a, b, d, c, e, f, h, g, i], is_dual)
    } else {
        select_432([a, d, b, c, f, g, h, e, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_557([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_148([c, b, a, f, e, g, d, h], is_dual)
    } else {
        select_164([a, d, c, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_556([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_557([a, b, c, e, h, g, f, i], is_dual)
    } else {
        select_188([e, a, c, d, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_551([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_552([a, b, d, e, h, f, i, g, j], is_dual)
    } else {
        select_556([b, f, c, d, g, e, h, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_560([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_54([b, a, e, g, h, i], is_dual)
    } else {
        select_62([a, e, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_561([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_62([a, e, c, d, f, g], is_dual)
    } else {
        select_56([g, b, a, e, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_559([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_560([a, c, d, i, f, g, k, h, j], is_dual)
    } else {
        select_561([b, c, e, h, f, g, j, i, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_558([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_559([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_243([b, k, h, a, j, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_550([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_551([b, d, g, e, a, f, h, j, i, k, l], is_dual)
    } else {
        select_558([b, c, d, a, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_538([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_539([a, c, b, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_550([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_567([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_71([a, c, d, g, i], is_dual)
    } else {
        select_71([b, c, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_566([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_241([b, e, c, d, g, f, h], is_dual)
    } else {
        select_567([g, j, i, e, h, a, b, f, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_568([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_149([a, b, c, d, f, e, g], is_dual)
    } else {
        select_46([d, g, h, i, a, f, e, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_565([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_566([a, b, c, f, e, g, h, j, i, l], is_dual)
    } else {
        select_568([e, b, d, h, i, f, j, g, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_570([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_131([b, c, d, e, f, h, g, j], is_dual)
    } else {
        select_500([h, j, k, f, i, a, b, c, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_569([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_458([b, c, d, f, i, h, g], is_dual)
    } else {
        select_570([a, b, e, c, d, f, g, i, j, h, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_564([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_565([b, d, e, c, f, g, a, h, i, j, l, k], is_dual)
    } else {
        select_569([b, d, a, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 5, i = 2
fn select_574([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_82([a, b, c, d], is_dual)
    } else {
        select_28([a, d, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_575([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_256([a, f, g], is_dual)
    } else {
        select_55([a, b, c, d, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_573([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_574([a, f, c, e, i], is_dual)
    } else {
        select_575([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_576([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_62([b, e, c, d, g, h], is_dual)
    } else {
        select_296([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_572([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_573([a, b, c, g, h, i, f, j, k], is_dual)
    } else {
        select_576([b, c, d, e, a, g, f, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_578([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_221([b, d, e, a, c, f, g, h], is_dual)
    } else {
        select_131([a, c, b, d, e, f, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_577([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_578([b, c, f, d, e, g, h, i, j], is_dual)
    } else {
        select_222([a, c, g, d, e, f, i, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_571([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_572([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_577([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_563([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_564([a, b, d, c, j, g, h, k, i, m, n, l], is_dual)
    } else {
        select_571([a, g, d, e, f, i, h, j, l, k, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_582([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_468([b, a, d, c, e, g, f, h, j, i], is_dual)
    } else {
        select_189([i, j, a, g, f, b, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_583([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_340([b, c, d, f, g, e, h], is_dual)
    } else {
        select_189([i, h, f, d, b, a, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_581([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_582([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_583([b, a, c, g, e, h, i, j, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_585([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_101([a, c, d, e, f, g], is_dual)
    } else {
        select_127([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_586([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_340([g, e, h, a, b, f, c], !is_dual)
    } else {
        select_41([b, c, e, d, g, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_584([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_585([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_586([b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_580([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_581([b, d, c, e, f, a, g, h, i, j], is_dual)
    } else {
        select_584([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_590([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, c, d, g, f, j], is_dual)
    } else {
        select_71([b, d, e, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_591([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_24([a, b, c, i, f, j], is_dual)
    } else {
        select_63([c, d, e, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_589([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_590([b, f, c, d, a, g, i, h, j, k], is_dual)
    } else {
        select_591([b, c, d, a, e, g, f, h, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_592([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_224([h, f, g, d, e, c, a], !is_dual)
    } else {
        select_107([a, b, f, d, g, h, e], is_dual)
    }
}
/// n = 12, i = 5
fn select_588([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_589([c, a, b, d, e, f, g, i, j, h, l], is_dual)
    } else {
        select_592([g, l, k, j, b, a, h, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_23([c, e, d, g, a, f, i], is_dual)
    } else {
        select_216([b, a, c, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_595([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_224([a, e, b, f, d, h, i], is_dual)
    } else {
        select_98([g, h, i, a, d, e, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_593([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_594([a, e, c, d, g, f, h, j, i], is_dual)
    } else {
        select_595([a, b, f, e, i, j, h, k, g], is_dual)
    }
}
/// n = 12, i = 5
fn select_587([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_588([b, a, c, d, e, g, f, i, h, j, k, l], is_dual)
    } else {
        select_593([c, b, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_579([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_580([a, c, b, d, i, f, g, k, l, j], is_dual)
    } else {
        select_587([a, b, c, g, e, f, h, j, i, l, k, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_562([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_563([a, b, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_579([a, b, c, e, d, g, j, h, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_537([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_538([a, h, g, d, e, f, b, i, j, k, l, m], is_dual)
    } else {
        select_562([a, c, b, d, e, f, h, i, g, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 2
fn select_601([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_130([a, h, b, c, f, g, i], is_dual)
    } else {
        select_118([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_602([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_375([a, f, c, d, e, g, h], is_dual)
    } else {
        select_30([b, e, c, d, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_600([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_601([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_602([a, g, d, e, f, b, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_604([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_69([f, c, d, e, b, g], is_dual)
    } else {
        select_131([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_605([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_42([h, d, i, f, e, a], !is_dual)
    } else {
        select_567([a, c, b, d, e, g, h, f, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_603([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_604([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_605([a, h, b, f, g, e, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_599([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_600([a, b, d, e, g, f, h, i, j], is_dual)
    } else {
        select_603([a, c, b, e, h, f, g, j, k, i, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_608([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_89([a, c, d, f, e, i], is_dual)
    } else {
        select_107([b, a, f, g, d, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_607([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_608([a, c, h, f, g, j, e, i, k], is_dual)
    } else {
        select_561([b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_609([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_230([e, f, h, a, b, g], !is_dual)
    } else {
        select_146([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_606([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_607([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_609([e, a, f, d, g, i, k, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_598([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_599([b, c, d, f, e, g, h, j, k, i, l, m], is_dual)
    } else {
        select_606([a, b, d, i, g, h, k, f, l, j, m], is_dual)
    }
}
/// n = 6, i = 2
fn select_613([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_81([e, a, b, d, f], is_dual)
    } else {
        select_247([d, b, c, e, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_615([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_71([a, d, g, f, c], is_dual)
    } else {
        select_71([b, c, g, e, d], is_dual)
    }
}
/// n = 8, i = 3
fn select_614([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_322([a, c, b, d, e, g, f, h], is_dual)
    } else {
        select_615([e, h, f, g, a, c, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_612([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_613([e, c, d, a, f, g], is_dual)
    } else {
        select_614([b, a, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_617([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_258([a, b, d, e, f, g, i, j, h, k], is_dual)
    } else {
        select_41([a, b, h, c, j, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_618([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_242([a, b, c, e, d, f, g, h], is_dual)
    } else {
        select_242([a, b, d, e, c, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_616([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_617([a, d, e, b, c, h, g, i, j, f, k], is_dual)
    } else {
        select_618([d, b, c, f, h, j, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_611([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_612([a, b, c, h, f, j, k, g], is_dual)
    } else {
        select_616([b, a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_621([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_62([a, e, c, d, f, g], is_dual)
    } else {
        select_196([g, a, b, h, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_622([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_41([b, a, f, d, h, g], is_dual)
    } else {
        select_497([b, a, c, g, e, i, h, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_620([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_621([b, c, d, e, g, h, f, i], is_dual)
    } else {
        select_622([a, b, c, f, g, h, e, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_624([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_280([b, d, e, g, f, i, h], is_dual)
    } else {
        select_70([a, b, c, h, g, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_626([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_39([g, h, i, a, d, b], !is_dual)
    } else {
        select_55([b, c, d, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_625([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_626([a, b, c, g, e, i, h, f, j], is_dual)
    } else {
        select_41([a, b, f, d, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_623([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_624([b, a, d, c, e, f, g, h, i, j], is_dual)
    } else {
        select_625([a, b, c, d, h, f, g, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_619([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_620([c, b, d, a, i, g, f, h, k, j], is_dual)
    } else {
        select_623([a, c, f, e, b, h, i, j, k, g, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_610([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_611([a, b, g, d, e, i, f, j, h, k, l], is_dual)
    } else {
        select_619([b, c, d, a, e, f, i, h, j, g, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_597([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_598([a, b, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    } else {
        select_610([a, h, b, c, e, i, g, f, k, j, m, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_631([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_33([a, c, d, e, f], is_dual)
    } else {
        select_247([a, b, f, g, e], is_dual)
    }
}
/// n = 5, i = 1
fn select_632([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_77([a, b, c, d, e], is_dual)
    } else {
        select_82([a, b, e, c], is_dual)
    }
}
/// n = 8, i = 3
fn select_630([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_631([a, b, c, d, e, g, h], is_dual)
    } else {
        select_632([a, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_634([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_255([g, f, b, c, d], !is_dual)
    } else {
        select_615([g, h, e, f, a, d, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_633([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_634([b, d, a, c, e, f, g, h], is_dual)
    } else {
        select_453([b, d, c, e, f, a, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_629([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_630([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_633([a, b, e, c, f, g, h, i], is_dual)
    }
}
/// n = 6, i = 1
fn select_637([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_62([a, b, d, e, c, f], is_dual)
    } else {
        select_62([a, c, d, e, b, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_636([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_171([a, d, b, c, e, f, g, h, j], is_dual)
    } else {
        select_637([b, c, f, e, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_639([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_114([b, c, d, e, f, g, h], is_dual)
    } else {
        select_42([a, d, g, f, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_638([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_397([a, b, d, e, f, g, h, j, i], is_dual)
    } else {
        select_639([a, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_635([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_636([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_638([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_628([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_629([a, b, c, d, e, h, i, g, j], is_dual)
    } else {
        select_635([a, b, d, e, c, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_642([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_474([b, a, f, d, e, g, h, j], is_dual)
    } else {
        select_142([a, c, b, g, e, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_641([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_642([a, b, d, c, e, h, g, i, f, k, j], is_dual)
    } else {
        select_642([a, c, d, b, e, h, f, i, g, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_646([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_150([a, c, e, h, d, g], is_dual)
    } else {
        select_150([b, c, d, h, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_645([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_77([d, e, c, g, f], is_dual)
    } else {
        select_646([a, b, f, d, e, h, i, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_648([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_39([a, c, d, e, f, h], is_dual)
    } else {
        select_39([b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_647([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_359([a, c, e, f, h, d, i], is_dual)
    } else {
        select_648([f, g, h, i, a, e, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_644([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_645([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_647([a, b, f, h, e, g, d, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_650([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_216([a, c, f, d, g, h, e, j], is_dual)
    } else {
        select_216([b, c, e, d, g, h, f, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_649([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_650([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_647([a, b, g, i, f, h, e, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_643([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_644([a, d, b, e, g, h, i, f, k, j], is_dual)
    } else {
        select_649([a, d, c, b, e, g, f, i, h, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_640([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_641([a, b, g, d, e, f, i, h, k, l, j], is_dual)
    } else {
        select_643([a, b, h, c, e, f, g, j, k, l, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_627([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_628([f, h, b, c, i, e, g, k, j, l], is_dual)
    } else {
        select_640([a, b, d, c, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_596([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_597([c, b, d, e, f, g, a, h, i, j, k, l, m], is_dual)
    } else {
        select_627([c, a, d, e, f, g, b, h, i, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_536([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_537([a, b, d, e, c, i, j, g, h, l, m, o, k, n], is_dual)
    } else {
        select_596([a, c, d, b, f, h, g, i, j, k, m, n, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_657([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_67([g, e, h, a, b, f], !is_dual)
    } else {
        select_33([a, c, d, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_656([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_458([b, c, d, a, f, e, g], is_dual)
    } else {
        select_657([a, b, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_660([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_24([a, g, c, f, d, i], is_dual)
    } else {
        select_116([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_659([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_56([g, b, c, e, f, h], is_dual)
    } else {
        select_660([g, k, j, d, i, h, e, a, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_662([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_71([b, c, f, e, h], is_dual)
    } else {
        select_43([a, b, d, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_661([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_149([d, b, c, g, f, e, h], is_dual)
    } else {
        select_662([g, i, j, d, h, a, f, e, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_658([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_659([i, h, k, d, g, j, a, f, e, c, b], !is_dual)
    } else {
        select_661([i, g, k, d, h, j, a, e, f, b], !is_dual)
    }
}
/// n = 11, i = 5
fn select_655([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_656([a, f, b, d, e, h, g, j], is_dual)
    } else {
        select_658([a, c, b, e, f, g, h, d, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_666([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_243([a, b, h, d, i, f, g], is_dual)
    } else {
        select_82([a, g, c, e], is_dual)
    }
}
/// n = 12, i = 4
fn select_665([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_114([a, h, c, d, f, k, i], is_dual)
    } else {
        select_666([a, b, d, e, i, g, h, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_667([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_280([g, c, d, h, f, k, i], is_dual)
    } else {
        select_335([a, b, i, e, g, j, l, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_664([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_665([a, b, e, d, f, g, h, i, k, l, j, m], is_dual)
    } else {
        select_667([b, a, e, c, f, g, h, j, k, l, i, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_670([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_24([b, d, c, e, f, h], is_dual)
    } else {
        select_28([a, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_669([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_242([a, d, c, f, e, h, g, j], is_dual)
    } else {
        select_670([h, i, j, f, d, a, g, b, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_671([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_46([f, h, c, i, d, g, a, e], !is_dual)
    } else {
        select_101([d, c, b, f, e, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_668([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_669([h, j, k, d, i, e, f, a, g, b], !is_dual)
    } else {
        select_671([j, k, d, e, i, a, g, f, c], !is_dual)
    }
}
/// n = 14, i = 5
fn select_663([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_664([b, c, a, d, e, f, g, h, j, k, i, l, m], is_dual)
    } else {
        select_668([m, j, n, a, l, h, k, b, f, i, c], !is_dual)
    }
}
/// n = 15, i = 5
fn select_654([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < j) || (is_dual && b > j) {
        select_655([n, o, l, a, m, i, k, b, c, g, d], !is_dual)
    } else {
        select_663([a, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    }
}
/// n = 10, i = 4
fn select_676([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_39([b, c, d, g, f, i], is_dual)
    } else {
        select_55([i, j, e, h, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_675([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_529([h, a, b, f, e, j, k], is_dual)
    } else {
        select_676([a, e, c, d, f, g, i, h, k, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_674([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_481([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_675([c, b, d, e, a, f, g, i, h, j, k], is_dual)
    }
}
/// n = 6, i = 2
fn select_678([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_507([a, c, d, b, e, f], is_dual)
    } else {
        select_507([b, c, d, a, e, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_677([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_678([a, b, c, g, e, i], is_dual)
    } else {
        select_134([h, j, i, a, b, g, f, e, d], !is_dual)
    }
}
/// n = 14, i = 5
fn select_673([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_674([b, c, a, e, f, i, h, k, l, n, j], is_dual)
    } else {
        select_677([b, a, c, d, j, g, l, m, n, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_680([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_507([a, c, f, g, d, i], is_dual)
    } else {
        select_670([j, g, i, h, f, a, e, d, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_679([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_680([a, c, b, d, e, g, h, f, j, i], is_dual)
    } else {
        select_274([d, c, f, e, a, i, g, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_672([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_673([b, a, c, d, e, f, g, h, i, j, k, m, l, n], is_dual)
    } else {
        select_679([b, c, d, j, g, l, m, i, a, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_653([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_654([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n], is_dual)
    } else {
        select_672([a, b, c, d, e, h, g, j, i, m, l, o, n, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_684([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_62([b, a, c, d, e, f], is_dual)
    } else {
        select_38([a, f, h, i, g, b], !is_dual)
    }
}
/// n = 9, i = 4
fn select_683([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_458([e, h, i, f, a, b, g], !is_dual)
    } else {
        select_684([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_686([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_33([b, c, d, e, f], is_dual)
    } else {
        select_89([g, e, h, a, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_685([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_458([a, c, d, b, e, f, g], is_dual)
    } else {
        select_686([a, b, c, d, e, g, h, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_682([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_683([b, a, c, d, e, f, g, h, i], is_dual)
    } else {
        select_685([a, e, h, i, f, b, g, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_688([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_24([d, h, i, g, a, f], !is_dual)
    } else {
        select_39([d, b, c, e, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_687([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_458([e, b, d, c, g, f, h], is_dual)
    } else {
        select_688([a, b, f, e, g, h, i, d, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_681([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_682([a, j, m, k, i, b, h, c, d], !is_dual)
    } else {
        select_687([l, m, j, a, k, i, b, g, f, e], !is_dual)
    }
}
/// n = 15, i = 5
fn select_652([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_653([a, b, c, d, e, f, g, h, j, k, i, l, m, n, o], is_dual)
    } else {
        select_681([a, b, d, i, f, h, k, g, m, o, j, l, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_694([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_131([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_58([f, c, d, e, a, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_693([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_694([a, g, d, e, f, b, h, i], is_dual)
    } else {
        select_124([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_692([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_693([a, b, e, c, f, g, h, i, l], is_dual)
    } else {
        select_368([c, d, a, b, f, h, g, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_698([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_150([b, c, d, f, g, h], is_dual)
    } else {
        select_25([a, d, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_697([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_149([e, c, d, a, f, g, i], is_dual)
    } else {
        select_698([d, a, b, c, f, e, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_696([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_697([a, d, c, b, f, h, i, g, j], is_dual)
    } else {
        select_286([b, d, a, e, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_699([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_594([a, d, c, b, f, h, i, g, j], is_dual)
    } else {
        select_474([b, d, e, a, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_695([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_696([a, b, d, e, g, h, i, f, k, j], is_dual)
    } else {
        select_699([a, c, d, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 16, i = 5
fn select_691([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_692([a, b, i, d, e, l, h, j, n, k, m, p], is_dual)
    } else {
        select_695([a, c, d, k, f, g, h, i, m, l, o], is_dual)
    }
}
/// n = 9, i = 3
fn select_703([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_62([b, e, c, d, f, g], is_dual)
    } else {
        select_101([a, g, b, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_704([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_660([a, c, d, b, e, f, g, h, i], is_dual)
    } else {
        select_660([b, c, d, a, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_702([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_703([c, a, b, f, e, g, i, j, k], is_dual)
    } else {
        select_704([a, b, e, d, g, f, h, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_706([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_69([c, d, e, h, g, j], is_dual)
    } else {
        select_105([a, b, d, e, g, f, h, i, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_707([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([e, b, c, d, f, g, i], is_dual)
    } else {
        select_502([f, a, c, d, e, h, g, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_705([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_706([b, a, c, d, e, f, g, h, i, j, k, m], is_dual)
    } else {
        select_707([h, c, d, e, a, g, j, i, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_701([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_702([a, j, b, c, f, g, i, k, h, l, n, m], is_dual)
    } else {
        select_705([a, b, g, d, e, f, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_711([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_30([a, b, c, d, e, f], is_dual)
    } else {
        select_47([e, c, d, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_712([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_39([a, b, f, e, h, i], is_dual)
    } else {
        select_31([c, d, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_710([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_711([b, g, c, d, f, h, j], is_dual)
    } else {
        select_712([a, f, c, d, e, h, g, i, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_713([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_105([a, i, d, e, f, g, h, j, k, l], is_dual)
    } else {
        select_119([b, c, h, d, e, f, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_709([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_710([b, j, c, d, g, a, h, i, k, l, m], is_dual)
    } else {
        select_713([b, c, d, a, e, f, g, h, i, j, k, m], is_dual)
    }
}
/// n = 7, i = 2
fn select_714([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_458([a, c, d, e, b, f, g], is_dual)
    } else {
        select_458([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 13, i = 4
fn select_708([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_709([a, b, e, f, c, d, g, h, j, i, l, k, m], is_dual)
    } else {
        select_714([a, j, c, d, h, k, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_700([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_701([b, c, d, e, f, g, h, a, i, j, k, l, n, m], is_dual)
    } else {
        select_708([b, c, a, d, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_690([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < k) || (is_dual && m > k) {
        select_691([a, b, e, d, c, f, j, h, i, g, k, l, n, o, m, p], is_dual)
    } else {
        select_700([a, b, e, c, h, i, j, g, l, m, o, k, n, p], is_dual)
    }
}
/// n = 10, i = 4
fn select_719([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_163([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_143([d, i, j, a, g, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_718([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_719([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_719([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_721([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_224([d, i, k, a, e, g, c], !is_dual)
    } else {
        select_180([b, c, d, f, e, h, g, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_720([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_721([e, j, k, f, g, i, h, d, a, b, c], !is_dual)
    } else {
        select_721([f, j, k, e, g, i, h, d, a, b, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_717([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_718([a, b, e, d, h, f, i, j, k, g], is_dual)
    } else {
        select_720([k, l, i, j, a, b, g, h, e, c, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_725([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_243([f, a, h, d, i, e, j], is_dual)
    } else {
        select_47([g, b, c, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_726([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([b, c, d, f], is_dual)
    } else {
        select_43([a, g, e, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_724([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_725([l, j, k, i, e, g, a, f, b, h], !is_dual)
    } else {
        select_726([b, c, d, e, f, g, h, i, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_723([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_458([a, c, d, g, e, i, h], is_dual)
    } else {
        select_724([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_722([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_723([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_723([b, c, d, e, a, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_716([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_717([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    } else {
        select_722([b, c, d, a, e, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_730([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_69([a, d, f, g, e, i], is_dual)
    } else {
        select_29([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_732([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_49([a, c, d, e, g, f, h], is_dual)
    } else {
        select_25([a, b, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_731([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_148([f, c, b, a, e, h, g, i], is_dual)
    } else {
        select_732([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_729([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_730([b, d, e, c, f, g, h, i, j], is_dual)
    } else {
        select_731([a, d, c, b, g, f, j, i, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_733([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_68([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_40([a, c, f, e, b, g, h], is_dual)
    }
}
/// n = 16, i = 5
fn select_728([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_729([a, d, k, e, f, h, i, j, m, o, l], is_dual)
    } else {
        select_733([a, b, l, c, n, g, p, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_734([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_62([a, b, f, g, e, i], is_dual)
    } else {
        select_29([c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 16, i = 5
fn select_727([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_728([a, b, d, c, e, f, h, g, i, j, k, m, l, n, o, p], is_dual)
    } else {
        select_734([c, a, e, f, l, i, j, g, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_715([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_716([a, b, l, d, c, h, g, n, o, k, p, m], is_dual)
    } else {
        select_727([a, b, d, c, e, f, h, g, i, j, k, m, n, o, l, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_689([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < k) || (is_dual && o > k) {
        select_690([a, b, c, e, d, f, g, i, j, h, l, k, m, n, o, p], is_dual)
    } else {
        select_715([a, b, c, d, e, f, g, h, i, j, l, m, n, o, k, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_651([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < n) || (is_dual && g > n) {
        select_652([a, b, c, d, i, f, h, j, k, l, n, o, g, p, m], is_dual)
    } else {
        select_689([a, b, d, e, f, c, h, i, j, g, k, l, m, o, n, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_535([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_536([a, c, b, d, f, i, h, j, k, l, m, g, p, o, n], is_dual)
    } else {
        select_651([a, c, d, f, e, b, h, j, i, g, l, m, n, k, o, p], is_dual)
    }
}
/// n = 7, i = 2
fn select_742([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_67([e, b, c, a, f, g], is_dual)
    } else {
        select_30([a, d, b, c, e, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_743([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_130([a, b, c, d, e, f, g], is_dual)
    } else {
        select_507([e, c, d, a, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_741([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_742([a, d, e, b, c, f, g], is_dual)
    } else {
        select_743([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_745([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_502([d, i, g, h, e, a, f, c], !is_dual)
    } else {
        select_324([e, g, h, d, f, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_744([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_745([a, f, b, d, h, e, i, j, g], is_dual)
    } else {
        select_745([a, e, c, d, g, f, i, j, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_740([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_741([a, c, d, f, g, i, h], is_dual)
    } else {
        select_744([j, i, l, a, g, k, b, c, e, h], !is_dual)
    }
}
/// n = 7, i = 2
fn select_747([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_246([a, f, b, c, g, e], is_dual)
    } else {
        select_191([a, c, d, b, f, e, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_750([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([a, g, c, i], is_dual)
    } else {
        select_116([a, b, d, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_749([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_750([b, a, d, c, f, g, e, i, h], is_dual)
    } else {
        select_238([b, a, e, d, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_748([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_398([b, c, e, a, f, d, g, h, i], is_dual)
    } else {
        select_749([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_746([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_747([a, d, e, f, g, b, h], is_dual)
    } else {
        select_748([a, d, c, e, b, g, f, i, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_739([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_740([a, b, c, d, f, e, h, i, j, g, k, l], is_dual)
    } else {
        select_746([c, d, b, a, e, h, g, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_752([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_545([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_747([a, c, b, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_755([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([e, b, c, d, f, g, h], is_dual)
    } else {
        select_648([a, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_756([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_69([b, c, d, e, f, g], is_dual)
    } else {
        select_42([a, b, g, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_754([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_755([c, b, d, e, f, a, g, i, h], is_dual)
    } else {
        select_756([c, a, d, e, f, b, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_753([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_21([b, d, e, f, a, h, g, i], is_dual)
    } else {
        select_754([b, a, c, d, e, h, i, j, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_751([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_752([b, c, d, e, h, i, g, a, j], is_dual)
    } else {
        select_753([b, c, d, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_738([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_739([a, b, c, d, e, h, k, g, i, j, l, m], is_dual)
    } else {
        select_751([c, d, a, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_760([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_326([e, a, b, c, d, f, g], is_dual)
    } else {
        select_164([c, d, b, f, g, a], is_dual)
    }
}
/// n = 8, i = 2
fn select_761([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_509([a, b, d, c, e, g, f, h], is_dual)
    } else {
        select_427([a, b, d, f, c, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_759([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_760([a, b, c, d, i, k, h], is_dual)
    } else {
        select_761([b, a, e, h, f, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_758([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_51([b, c, e, d, f, g, h, a, i, j, k], is_dual)
    } else {
        select_759([b, c, a, d, e, f, h, g, j, i, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_765([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_216([a, b, g, c, f, h, e, i], is_dual)
    } else {
        select_29([b, c, d, e, h, f, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_764([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_61([c, d, e, f, g, h, a], is_dual)
    } else {
        select_765([b, a, c, d, f, g, h, e, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_768([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_24([i, j, k, d, a, b], !is_dual)
    } else {
        select_243([a, c, g, e, h, f, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_769([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_25([a, f, c, h], is_dual)
    } else {
        select_25([b, e, d, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_767([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_768([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    } else {
        select_769([b, c, d, e, h, g, i, f], is_dual)
    }
}
/// n = 13, i = 5
fn select_766([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_104([b, a, d, e, f, g, i, h, k, m], is_dual)
    } else {
        select_767([j, l, m, i, k, f, a, g, b, h, c], !is_dual)
    }
}
/// n = 14, i = 5
fn select_763([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_764([c, d, a, e, f, j, i, h, k], is_dual)
    } else {
        select_766([a, b, d, c, h, f, g, k, j, i, l, m, n], is_dual)
    }
}
/// n = 9, i = 4
fn select_772([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_495([f, h, i, d, a, g, b, e], !is_dual)
    } else {
        select_98([f, g, i, d, a, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_771([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_772([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_772([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_774([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_221([a, b, c, d, e, g, f, h], is_dual)
    } else {
        select_63([f, d, e, b, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_773([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_774([b, c, d, a, e, g, f, h], is_dual)
    } else {
        select_146([b, c, f, d, a, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_770([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_771([k, h, m, a, l, c, b, d, i], !is_dual)
    } else {
        select_773([b, d, e, h, f, g, j, i, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_762([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_763([b, a, c, d, e, h, f, i, j, g, k, m, l, n], is_dual)
    } else {
        select_770([a, h, g, b, c, e, i, f, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_757([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_758([h, b, c, a, g, e, f, i, j, l, k], is_dual)
    } else {
        select_762([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_737([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_738([g, a, b, h, c, d, f, i, l, j, m, k, n], is_dual)
    } else {
        select_757([a, c, b, e, d, f, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 7, i = 2
fn select_780([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_54([a, b, d, f, g, e], is_dual)
    } else {
        select_149([a, c, b, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_781([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_101([a, b, f, d, g, e], is_dual)
    } else {
        select_156([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_779([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_780([a, b, f, d, e, h, i], is_dual)
    } else {
        select_781([a, e, c, d, g, f, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_783([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_148([b, i, d, f, g, h, j, l], is_dual)
    } else {
        select_296([a, c, h, d, e, g, i, k, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_784([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_46([a, c, d, g, f, i, h, k], is_dual)
    } else {
        select_70([b, c, e, h, g, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_782([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_783([a, c, b, d, f, g, h, i, j, l, m, k, n], is_dual)
    } else {
        select_784([a, g, b, h, e, f, i, k, m, j, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_778([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_779([i, d, e, a, g, h, j, l, k], is_dual)
    } else {
        select_782([b, a, d, e, c, f, g, h, j, i, k, l, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_786([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_458([c, a, b, d, g, h, f], is_dual)
    } else {
        select_774([c, d, e, a, b, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_785([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_656([b, f, c, d, g, a, h, i], is_dual)
    } else {
        select_786([c, d, a, b, e, h, f, g], is_dual)
    }
}
/// n = 14, i = 5
fn select_777([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_778([a, b, e, d, c, f, h, g, j, k, i, l, m, n], is_dual)
    } else {
        select_785([a, h, c, i, e, j, k, g, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_790([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_255([b, c, d, e, f], is_dual)
    } else {
        select_56([d, f, g, a, e, c], !is_dual)
    }
}
/// n = 9, i = 3
fn select_789([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_594([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_790([a, b, f, g, h, i, e], is_dual)
    }
}
/// n = 7, i = 1
fn select_792([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([e, c, d, f, g], is_dual)
    } else {
        select_33([e, a, b, g, f], is_dual)
    }
}
/// n = 7, i = 2
fn select_793([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_375([a, b, c, d, e, f, g], is_dual)
    } else {
        select_39([e, c, d, a, f, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_791([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_792([a, c, d, e, b, f, g], is_dual)
    } else {
        select_793([b, f, d, e, a, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_788([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_789([a, b, h, c, e, g, f, i, j], is_dual)
    } else {
        select_791([a, e, c, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_797([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_49([a, c, d, e, g, f, h], is_dual)
    } else {
        select_26([a, b], is_dual)
    }
}
/// n = 10, i = 3
fn select_796([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_797([b, e, c, d, f, g, i, h], is_dual)
    } else {
        select_432([a, f, c, d, e, g, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_795([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_796([a, b, c, d, e, f, h, i, g, j], is_dual)
    } else {
        select_346([b, e, c, d, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_799([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([e, c, d, f, g], is_dual)
    } else {
        select_324([a, b, g, e, h, f, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_798([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_509([e, b, f, c, d, g, h, i], is_dual)
    } else {
        select_799([a, g, c, d, e, f, h, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_794([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_795([b, c, d, e, g, h, j, i, k, l], is_dual)
    } else {
        select_798([a, c, d, e, j, f, h, i, k, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_787([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_788([a, c, e, h, f, j, i, l, n, k], is_dual)
    } else {
        select_794([c, b, d, a, i, f, g, h, k, j, m, o, l, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_776([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_777([a, c, e, b, h, g, i, f, j, m, l, o, k, n], is_dual)
    } else {
        select_787([a, c, b, d, e, f, g, h, i, j, k, m, n, o, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_805([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_49([b, c, d, e, f, g, h], is_dual)
    } else {
        select_47([a, c, d, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_804([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_54([f, d, a, h, e, g], is_dual)
    } else {
        select_805([b, d, a, c, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_803([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_804([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_243([b, e, f, g, h, i, a], is_dual)
    }
}
/// n = 12, i = 4
fn select_807([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_77([a, b, e, i, l], is_dual)
    } else {
        select_176([c, d, a, b, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_806([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_443([a, d, c, e, i, g, h, f, j, k, l, m], is_dual)
    } else {
        select_807([a, f, d, e, b, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_802([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_803([f, a, e, d, j, i, g, k, m], is_dual)
    } else {
        select_806([a, e, b, c, d, f, h, g, i, k, j, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_810([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_77([a, e, c, d, f], is_dual)
    } else {
        select_224([g, d, h, a, e, f, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_809([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_191([a, b, d, e, g, f, h], is_dual)
    } else {
        select_810([a, c, b, g, e, h, i, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_812([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_33([b, d, e, g, k], is_dual)
    } else {
        select_42([a, c, h, f, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_813([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_205([b, a, c, d, e, f, g, i], is_dual)
    } else {
        select_77([a, f, c, d, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_811([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_812([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    } else {
        select_813([b, f, d, e, g, a, h, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_808([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_809([a, c, d, i, f, h, j, g, k], is_dual)
    } else {
        select_811([a, b, d, e, g, f, h, i, l, k, j], is_dual)
    }
}
/// n = 15, i = 5
fn select_801([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_802([a, c, e, d, g, f, h, i, j, l, m, k, o], is_dual)
    } else {
        select_808([a, f, b, e, h, i, g, l, k, n, j, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_817([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_183([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_243([a, c, f, e, d, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_818([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_495([a, c, f, d, e, g, i, j], is_dual)
    } else {
        select_495([c, b, d, e, g, f, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_816([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_817([c, i, d, f, g, k, h, j], is_dual)
    } else {
        select_818([a, c, b, h, e, f, j, i, l, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_820([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_242([a, g, c, d, f, i, h, k], is_dual)
    } else {
        select_338([a, b, f, d, e, h, i, j, g, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_821([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_54([h, f, a, g, d, b], !is_dual)
    } else {
        select_574([a, f, c, e, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_819([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_820([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_821([g, a, c, i, f, d, h, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_815([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_816([b, a, c, d, e, f, g, i, h, k, j, l, m], is_dual)
    } else {
        select_819([c, e, d, f, h, g, a, i, j, l, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_824([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_444([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_263([a, b, f, e, h, g, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_826([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_49([a, c, d, e, h, g, j], is_dual)
    } else {
        select_49([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_825([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_196([a, d, i, g, j], is_dual)
    } else {
        select_826([b, c, a, g, e, f, h, i, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_823([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_824([b, d, e, f, h, i, k, l], is_dual)
    } else {
        select_825([a, d, e, c, h, i, g, j, m, n, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_828([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_77([a, f, c, g, e], is_dual)
    } else {
        select_161([a, b, e, d, f, h, i, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_827([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_828([b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_82([a, j, c, e], is_dual)
    }
}
/// n = 14, i = 4
fn select_822([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_823([c, b, e, a, d, g, i, f, h, j, m, n, k, l], is_dual)
    } else {
        select_827([a, c, g, e, k, i, f, l, j, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_814([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_815([a, b, c, d, f, g, h, k, l, n, j, o, m], is_dual)
    } else {
        select_822([c, b, a, e, d, g, f, i, h, j, k, n, m, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_800([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_801([a, b, d, e, c, g, h, i, f, j, m, k, n, o, l], is_dual)
    } else {
        select_814([a, d, c, b, e, g, f, h, i, j, m, l, n, o, k], is_dual)
    }
}
/// n = 16, i = 5
fn select_775([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_776([a, d, f, c, e, g, j, h, i, m, l, n, k, p, o], is_dual)
    } else {
        select_800([a, e, d, b, h, g, f, i, k, l, n, o, j, p, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_736([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && f < l) || (is_dual && f > l) {
        select_737([g, a, b, e, h, j, k, l, i, n, m, f, o, p], is_dual)
    } else {
        select_775([a, c, d, b, e, g, f, h, i, j, k, m, n, l, o, p], is_dual)
    }
}
/// n = 8, i = 2
fn select_835([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([e, c, d, f, g], is_dual)
    } else {
        select_89([a, b, g, e, h, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_834([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([b, e, d, g, f, h], is_dual)
    } else {
        select_835([a, c, b, f, e, g, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_837([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_23([b, c, d, g, e, f, h], is_dual)
    } else {
        select_24([a, b, d, f, g, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_836([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_191([f, b, j, e, k, h, i], is_dual)
    } else {
        select_837([a, i, c, d, f, g, l, j, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_833([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_834([b, c, j, e, l, h, f, i, k, m], is_dual)
    } else {
        select_836([a, c, f, d, e, i, g, h, k, j, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_841([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_63([c, d, f, e, h], is_dual)
    } else {
        select_82([a, b, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_840([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_172([a, b, d, e, h, g, i, k], is_dual)
    } else {
        select_841([a, b, c, i, f, g, h, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_839([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_639([a, b, c, i, f, j, h, g, k], is_dual)
    } else {
        select_840([b, c, a, d, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_843([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_183([a, f, b, d, e, g, h, i], is_dual)
    } else {
        select_63([b, c, g, e, f], is_dual)
    }
}
/// n = 10, i = 3
fn select_842([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_76([b, h, c, d, f, g], is_dual)
    } else {
        select_843([a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_838([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_839([a, b, c, d, g, f, h, i, k, l, j], is_dual)
    } else {
        select_842([a, d, e, i, f, g, j, h, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_832([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_833([a, c, d, b, e, h, g, i, j, f, k, l, m, n], is_dual)
    } else {
        select_838([c, b, f, d, e, h, i, j, g, l, k, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_845([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_22([a, d, b, e, h, g, f, i], is_dual)
    } else {
        select_191([a, b, c, f, h, i, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_847([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_67([d, f, g, e, a, b], !is_dual)
    } else {
        select_67([e, f, g, d, a, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_846([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_742([a, d, g, e, f, i, h], is_dual)
    } else {
        select_847([j, g, k, b, a, c, h], !is_dual)
    }
}
/// n = 12, i = 4
fn select_844([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_845([a, c, f, d, e, h, g, i, j], is_dual)
    } else {
        select_846([a, b, g, d, e, h, f, j, i, k, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_831([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_832([b, d, e, a, c, f, h, i, g, j, k, l, m, n], is_dual)
    } else {
        select_844([a, b, c, h, i, f, g, j, l, n, o, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_852([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_183([a, e, c, d, f, h, g, i], is_dual)
    } else {
        select_507([a, b, g, d, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_853([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_440([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_445([c, a, d, e, b, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_851([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_852([a, c, b, g, e, f, i, j, h], is_dual)
    } else {
        select_853([e, a, b, d, f, g, h, j, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_850([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_630([a, b, f, i, e, j, k, m], is_dual)
    } else {
        select_851([a, c, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_857([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_49([a, b, c, d, e, f, g], is_dual)
    } else {
        select_28([a, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_856([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_149([d, b, c, f, e, g, h], is_dual)
    } else {
        select_857([a, b, e, d, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_858([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_662([a, g, b, e, d, j, k, h, l], is_dual)
    } else {
        select_56([d, h, c, i, f, g], is_dual)
    }
}
/// n = 14, i = 5
fn select_855([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_856([a, b, d, e, g, k, i, j, m, n], is_dual)
    } else {
        select_858([a, b, c, j, e, f, h, i, l, k, m, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_860([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_55([b, c, g, e, h, f], is_dual)
    } else {
        select_71([a, b, d, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_859([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_247([d, a, i, l, e], is_dual)
    } else {
        select_860([b, e, c, d, f, g, h, j, k, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_854([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_855([b, a, d, c, e, g, f, i, j, h, k, l, m, n], is_dual)
    } else {
        select_859([b, h, d, a, e, g, i, j, m, l, k, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_849([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_850([a, b, g, c, e, f, i, j, k, l, h, m, n], is_dual)
    } else {
        select_854([a, b, c, d, e, f, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_865([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_39([a, c, d, e, f, g], is_dual)
    } else {
        select_82([a, b, h, g], !is_dual)
    }
}
/// n = 9, i = 4
fn select_866([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_55([h, i, d, g, a, f], !is_dual)
    } else {
        select_47([e, b, c, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_864([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_865([a, f, h, i, d, g, e, c], !is_dual)
    } else {
        select_866([f, h, i, d, a, g, e, b, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_863([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_864([a, c, d, h, f, i, g, j, k], is_dual)
    } else {
        select_40([a, c, b, j, e, k, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_867([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_77([a, f, d, g, b], !is_dual)
    } else {
        select_38([a, d, f, g, e, c], !is_dual)
    }
}
/// n = 13, i = 5
fn select_862([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_863([a, c, f, d, e, g, h, i, k, l, j], is_dual)
    } else {
        select_867([e, m, l, a, f, b, j], !is_dual)
    }
}
/// n = 11, i = 4
fn select_871([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_39([a, b, h, e, j, k], is_dual)
    } else {
        select_63([d, c, g, f, i], is_dual)
    }
}
/// n = 11, i = 5
fn select_872([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_39([a, c, f, d, i, k], is_dual)
    } else {
        select_71([b, g, e, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_870([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_871([a, b, d, c, e, f, g, h, i, j, l], is_dual)
    } else {
        select_872([a, c, b, e, f, h, i, g, j, k, l], is_dual)
    }
}
/// n = 11, i = 5
fn select_873([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_107([d, b, g, e, h, i, j], is_dual)
    } else {
        select_180([a, e, c, d, f, i, g, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_869([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_870([a, c, b, d, f, e, h, g, i, j, k, l], is_dual)
    } else {
        select_873([a, b, c, f, g, e, h, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_875([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_62([d, c, b, e, f, g], is_dual)
    } else {
        select_305([a, b, f, d, e, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_876([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_54([a, e, c, d, g, h], is_dual)
    } else {
        select_196([c, b, d, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_874([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_875([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_876([a, c, d, g, f, e, h, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_868([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_869([a, b, i, c, e, f, h, l, j, m, k, n], is_dual)
    } else {
        select_874([b, j, d, e, h, g, k, i, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_861([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_862([e, b, f, d, a, g, h, i, j, m, k, l, n], is_dual)
    } else {
        select_868([b, c, a, d, e, g, h, f, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_848([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_849([a, d, b, e, g, c, h, f, i, j, k, m, l, n], is_dual)
    } else {
        select_861([a, d, c, e, f, b, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_830([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_831([b, c, d, a, e, g, h, f, i, j, k, m, n, l, o], is_dual)
    } else {
        select_848([b, a, c, d, i, g, h, j, l, k, f, n, o, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_882([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_183([i, j, g, f, e, a, h, b], !is_dual)
    } else {
        select_750([a, b, d, c, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_884([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_116([c, d, g, e, f, i, j], is_dual)
    } else {
        select_47([a, b, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_883([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_884([a, e, b, c, g, f, h, i, j, k, l], is_dual)
    } else {
        select_750([a, b, e, d, i, h, g, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_881([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_882([b, i, c, d, a, h, g, j, l, m], is_dual)
    } else {
        select_883([b, a, e, c, d, f, g, h, i, k, l, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_886([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_149([a, f, b, d, e, h, i], is_dual)
    } else {
        select_149([a, e, c, d, f, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_887([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_241([b, c, a, d, e, f, g], is_dual)
    } else {
        select_89([b, d, e, f, a, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_885([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_886([b, a, d, e, g, f, i, j, h], is_dual)
    } else {
        select_887([a, h, b, c, e, j, g, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_880([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_881([a, c, d, b, e, g, h, f, i, l, j, k, m], is_dual)
    } else {
        select_885([a, f, d, e, b, g, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_890([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([b, e, c, d, g, f, i], is_dual)
    } else {
        select_107([j, i, d, e, h, a, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_891([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, h, b, e, f, g, i, j], is_dual)
    } else {
        select_69([b, c, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_889([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_890([b, g, c, a, e, f, h, i, j, k], is_dual)
    } else {
        select_891([b, c, a, d, e, f, h, g, i, k], is_dual)
    }
}
/// n = 7, i = 1
fn select_893([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_29([a, c, d, e, b, f, g], is_dual)
    } else {
        select_29([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_892([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_604([b, a, c, d, g, f, i, j], is_dual)
    } else {
        select_893([a, c, e, d, f, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_888([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_889([a, b, c, e, f, i, g, j, h, l, k], is_dual)
    } else {
        select_892([a, b, c, d, g, f, h, j, i, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_879([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_880([a, b, c, h, e, g, i, f, j, k, l, m, n], is_dual)
    } else {
        select_888([b, c, d, f, e, g, i, j, h, k, m, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_897([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_56([a, c, d, e, f, g], is_dual)
    } else {
        select_98([e, h, f, a, b, g, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_896([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_506([a, b, d, f, g, e, i, h], is_dual)
    } else {
        select_897([a, b, c, e, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_899([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_648([g, h, f, i, a, d, e, b], !is_dual)
    } else {
        select_98([a, d, c, f, h, e, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_898([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_899([h, j, g, i, e, a, f, b, c], !is_dual)
    } else {
        select_899([g, j, h, i, f, a, e, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_895([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_896([b, d, a, e, f, c, g, h, i], is_dual)
    } else {
        select_898([b, d, c, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_901([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_241([a, d, b, c, e, f, g], is_dual)
    } else {
        select_56([a, e, c, b, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_900([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_901([a, b, c, f, g, e, h, i], is_dual)
    } else {
        select_865([a, b, d, e, h, f, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_894([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_895([a, c, d, b, e, g, f, h, i, j], is_dual)
    } else {
        select_900([c, d, e, a, g, f, b, i, h], is_dual)
    }
}
/// n = 14, i = 5
fn select_878([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < g) || (is_dual && l > g) {
        select_879([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_894([a, c, b, j, h, f, l, g, m, n], is_dual)
    }
}
/// n = 6, i = 2
fn select_906([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_77([a, b, c, d, f], is_dual)
    } else {
        select_77([a, c, b, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_907([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_148([c, b, a, f, g, h, e, i], is_dual)
    } else {
        select_114([a, e, c, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_905([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_906([e, a, b, i, f, j], is_dual)
    } else {
        select_907([b, c, d, f, e, g, h, i, k], is_dual)
    }
}
/// n = 8, i = 2
fn select_909([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_346([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_346([b, a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_908([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_909([b, c, a, g, h, f, j, k], is_dual)
    } else {
        select_22([a, d, e, f, h, g, i, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_904([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_905([c, b, j, d, m, g, f, k, i, l, o], is_dual)
    } else {
        select_908([a, b, c, i, e, g, h, l, k, j, m, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_912([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_62([a, b, d, f, e, h], is_dual)
    } else {
        select_62([a, c, d, e, f, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_913([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_189([e, b, a, d, g, f, h], is_dual)
    } else {
        select_49([b, a, c, d, f, e, g], is_dual)
    }
}
/// n = 13, i = 5
fn select_911([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_912([b, a, e, d, h, g, j, k], is_dual)
    } else {
        select_913([a, c, b, f, i, k, l, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_915([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_258([a, b, c, f, e, i, h, g, j, k], is_dual)
    } else {
        select_444([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_916([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_23([b, d, c, f, e, g, h], is_dual)
    } else {
        select_98([f, i, g, e, a, h, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_914([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_915([a, c, b, d, f, e, g, h, i, j, k], is_dual)
    } else {
        select_916([a, b, f, d, e, g, h, j, i], is_dual)
    }
}
/// n = 15, i = 5
fn select_910([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_911([a, b, e, i, c, h, g, j, k, m, o, l, n], is_dual)
    } else {
        select_914([a, k, e, d, f, h, l, j, i, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_903([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_904([b, f, c, d, e, g, h, i, j, l, k, m, n, o, p], is_dual)
    } else {
        select_910([a, b, e, d, h, g, i, k, j, f, l, n, m, p, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_920([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_130([f, a, c, d, e, h, g], is_dual)
    } else {
        select_130([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_919([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_920([a, b, c, g, e, f, h, i], is_dual)
    } else {
        select_793([e, b, d, h, f, g, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_922([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_62([b, d, a, e, f, g], is_dual)
    } else {
        select_62([a, c, b, f, e, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_923([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_242([a, b, d, e, g, i, h, j], is_dual)
    } else {
        select_660([a, d, c, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_921([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_922([b, c, e, d, h, g, f, i], is_dual)
    } else {
        select_923([a, c, b, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_918([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_919([c, b, f, d, g, e, h, i, k], is_dual)
    } else {
        select_921([a, b, g, d, f, e, h, i, j, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_926([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_359([f, g, h, a, c, e, d], !is_dual)
    } else {
        select_143([f, g, h, a, b, e, d], !is_dual)
    }
}
/// n = 9, i = 4
fn select_925([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_252([a, c, e, d, g, f, i], is_dual)
    } else {
        select_926([d, h, i, g, e, a, b, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_928([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_247([e, g, b, f, d], is_dual)
    } else {
        select_224([a, d, c, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_929([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_529([e, g, h, f, b, d, c], !is_dual)
    } else {
        select_497([a, b, c, e, f, d, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_927([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_928([h, d, i, g, e, a, f, b], !is_dual)
    } else {
        select_929([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_924([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_925([a, c, b, i, e, f, j, h, k], is_dual)
    } else {
        select_927([a, c, d, h, e, g, k, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_917([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_918([b, c, a, d, f, g, e, h, j, i, k, l], is_dual)
    } else {
        select_924([a, b, c, h, f, g, i, j, e, l, k], is_dual)
    }
}
/// n = 16, i = 5
fn select_902([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < j) || (is_dual && o > j) {
        select_903([a, b, d, c, e, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_917([a, b, c, h, f, g, l, k, o, n, p, j], is_dual)
    }
}
/// n = 16, i = 5
fn select_877([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_878([g, a, b, e, h, j, f, i, k, m, n, p, l, o], is_dual)
    } else {
        select_902([b, a, c, d, e, f, g, h, i, j, k, l, n, o, p, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_829([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_830([g, a, b, e, d, j, f, i, h, l, m, k, n, o, p], is_dual)
    } else {
        select_877([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_735([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_736([c, a, d, e, f, b, g, h, i, j, k, l, m, n, p, o], is_dual)
    } else {
        select_829([c, b, d, e, f, a, g, h, i, j, k, l, m, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_534([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_535([b, a, c, d, e, f, g, h, i, j, l, m, n, k, o, p], is_dual)
    } else {
        select_735([a, b, c, d, e, f, h, i, j, k, l, m, n, g, o, p], is_dual)
    }
}
/// n = 10, i = 3
fn select_938([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_131([a, g, c, d, e, f, h, j], is_dual)
    } else {
        select_131([b, f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_937([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_237([a, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_938([a, c, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_940([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_241([b, c, a, d, e, f, g], is_dual)
    } else {
        select_98([a, c, b, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_939([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_940([a, b, e, h, f, j, g, k, l], is_dual)
    } else {
        select_415([c, d, e, g, f, i, h], is_dual)
    }
}
/// n = 14, i = 5
fn select_936([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_937([b, c, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_939([a, b, c, d, j, g, h, i, k, l, m, n], is_dual)
    }
}
/// n = 11, i = 4
fn select_943([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_750([a, e, b, d, g, f, j, i, h], is_dual)
    } else {
        select_362([i, k, j, f, g, a, h, e, c], !is_dual)
    }
}
/// n = 12, i = 4
fn select_944([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_59([f, b, c, d, e, g, h, i, j, k], is_dual)
    } else {
        select_521([a, h, c, d, e, f, i, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_942([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_943([a, b, c, j, g, h, i, f, k, l, m], is_dual)
    } else {
        select_944([a, c, d, e, f, h, g, i, j, k, m, l], is_dual)
    }
}
/// n = 11, i = 5
fn select_946([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_296([b, c, d, a, f, e, h, g, i], is_dual)
    } else {
        select_750([h, i, k, j, e, f, a, b, g], !is_dual)
    }
}
/// n = 11, i = 5
fn select_945([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_946([a, b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_43([a, h, e, c, g], is_dual)
    }
}
/// n = 14, i = 5
fn select_941([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < f) || (is_dual && m > f) {
        select_942([b, a, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_945([l, k, n, f, i, h, m, a, g, j, b], !is_dual)
    }
}
/// n = 14, i = 5
fn select_935([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_936([b, c, a, d, e, f, g, i, h, j, k, l, m, n], is_dual)
    } else {
        select_941([b, c, d, e, f, g, h, i, a, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_950([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_127([a, b, c, e, f, h, i, g], is_dual)
    } else {
        select_156([a, c, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_952([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_34([b, c, d, f], is_dual)
    } else {
        select_71([a, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_951([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_205([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_952([b, c, d, e, a, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_949([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_950([a, b, i, c, f, g, j, h, k], is_dual)
    } else {
        select_951([a, b, d, e, h, g, i, k, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_955([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([h, i, j, e, a, b], !is_dual)
    } else {
        select_39([a, c, d, g, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_954([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_375([g, b, c, d, f, h, j], is_dual)
    } else {
        select_955([f, a, c, d, e, h, g, i, k, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_956([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_27([h], is_dual)
    }
}
/// n = 12, i = 4
fn select_953([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_954([a, f, c, d, e, h, g, i, k, l, j], is_dual)
    } else {
        select_956([e, i, b, g, j, f, l, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_948([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_949([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_953([b, c, d, e, f, g, a, h, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_959([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_131([a, h, d, e, f, g, i, j], is_dual)
    } else {
        select_119([b, c, g, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_958([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_126([a, h, d, e, g, f, i, j, k], is_dual)
    } else {
        select_959([a, d, e, b, c, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_957([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_958([b, c, d, e, f, a, g, h, i, k, j], is_dual)
    } else {
        select_714([a, g, c, d, i, j, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_947([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_948([a, c, d, e, f, g, h, b, i, j, k, l], is_dual)
    } else {
        select_957([a, c, b, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_934([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_935([a, c, b, d, e, f, g, i, h, j, k, l, m, n], is_dual)
    } else {
        select_947([g, a, h, d, e, f, b, i, j, k, l, m], is_dual)
    }
}
/// n = 7, i = 3
fn select_965([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_243([a, b, c, d, f, e, g], is_dual)
    } else {
        select_243([a, b, c, e, f, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_964([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_375([a, e, c, d, f, h, j], is_dual)
    } else {
        select_965([a, b, g, e, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_963([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_964([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_965([f, b, a, e, h, i, j], is_dual)
    }
}
/// n = 6, i = 1
fn select_967([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_33([a, c, d, e, f], is_dual)
    } else {
        select_77([a, b, c, e, f], is_dual)
    }
}
/// n = 8, i = 3
fn select_969([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, b, c, e, g, h], is_dual)
    } else {
        select_25([c, d, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_968([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_969([b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_495([a, b, f, d, e, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_966([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_967([b, f, d, e, i, h], is_dual)
    } else {
        select_968([a, b, c, h, f, g, j, k, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_962([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_963([a, c, b, h, g, j, i, k, f, l], is_dual)
    } else {
        select_966([a, b, c, d, e, f, g, h, i, k, j, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_971([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_621([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_621([a, b, c, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_973([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_221([a, b, f, d, e, h, g, i], is_dual)
    } else {
        select_30([c, g, d, e, f, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_972([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_129([b, c, d, e, f, a, g, h, i], is_dual)
    } else {
        select_973([b, d, a, e, f, c, g, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_970([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_971([a, c, b, h, f, g, j, k], is_dual)
    } else {
        select_972([f, g, b, a, d, e, i, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_961([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_962([a, c, b, d, e, f, h, i, g, j, k, l], is_dual)
    } else {
        select_970([b, c, a, d, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_977([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_241([b, a, c, d, e, f, g], is_dual)
    } else {
        select_359([a, b, d, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_976([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_977([a, b, c, f, g, e, h, i, j], is_dual)
    } else {
        select_62([b, c, d, e, g, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_975([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_212([b, f, c, d, e, g, h, i], is_dual)
    } else {
        select_976([a, b, i, c, f, h, g, j, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_979([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_726([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_726([a, b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 7, i = 3
fn select_980([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_24([c, d, g, f, a, e], !is_dual)
    } else {
        select_47([c, b, d, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_978([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_979([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_980([a, g, e, f, b, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_974([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_975([a, c, b, d, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_978([a, b, c, i, f, g, j, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_960([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_961([b, c, d, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_974([b, a, d, e, f, g, h, c, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_933([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_934([a, b, c, e, d, f, g, h, i, j, l, n, k, m], is_dual)
    } else {
        select_960([a, c, d, f, b, i, g, h, k, m, j, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_985([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_276([a, c, d, f, e, h, g, j], is_dual)
    } else {
        select_926([e, i, j, h, f, a, b, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_984([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_985([b, a, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_164([e, j, f, a, b, i], !is_dual)
    }
}
/// n = 7, i = 2
fn select_989([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_49([a, c, d, b, e, f, g], is_dual)
    } else {
        select_49([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_988([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_258([a, b, c, d, f, g, e, h, i, j], is_dual)
    } else {
        select_989([b, e, c, d, f, g, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_990([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_54([g, d, a, e, f, b], !is_dual)
    } else {
        select_107([g, f, a, d, e, b, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_987([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_988([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_990([b, a, g, e, h, i, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_992([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_183([a, c, f, e, b, g, h, i], is_dual)
    } else {
        select_712([a, c, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_991([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_992([a, b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_416([a, f, b, d, g, e, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_986([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_987([a, b, c, d, h, f, i, j, g, k], is_dual)
    } else {
        select_991([b, c, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_983([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_984([b, a, c, i, h, f, g, k, l, j], is_dual)
    } else {
        select_986([a, b, d, e, g, f, h, j, i, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_995([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_938([a, c, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_938([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_994([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_995([c, d, a, b, e, f, g, i, h, k], is_dual)
    } else {
        select_995([e, f, a, b, c, d, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_999([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_63([b, c, d, e, f], is_dual)
    } else {
        select_47([a, g, h, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_998([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([b, e, c, d, f, g, h], is_dual)
    } else {
        select_999([a, f, c, d, e, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_997([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_998([a, b, c, d, g, f, i, j, h, k], is_dual)
    } else {
        select_774([a, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1000([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_307([a, b, c, e, f, h, g, i, j], is_dual)
    } else {
        select_307([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_996([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_997([a, b, c, e, g, f, i, j, h, l, k], is_dual)
    } else {
        select_1000([a, b, d, h, f, g, j, k, i, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_993([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_994([c, f, a, h, b, g, j, i, k, l, m], is_dual)
    } else {
        select_996([b, a, c, d, e, i, g, j, k, h, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_982([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_983([a, b, c, d, f, i, j, h, k, g, l, m], is_dual)
    } else {
        select_993([b, c, d, a, f, e, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_1004([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_912([a, c, e, h, f, g, i, j], is_dual)
    } else {
        select_524([b, a, e, d, i, g, h, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1005([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_334([i, j, h, f, e, b, a, g], !is_dual)
    } else {
        select_276([b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1003([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1004([c, b, a, d, e, f, g, i, h, k, j], is_dual)
    } else {
        select_1005([b, a, c, g, i, f, j, k, l, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1002([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1003([a, b, c, d, e, f, h, g, i, j, k, l], is_dual)
    } else {
        select_1003([a, b, c, d, e, g, h, f, i, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_1008([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_637([a, b, c, d, f, h], is_dual)
    } else {
        select_893([a, d, e, b, c, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1007([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1008([e, b, f, c, d, h, g, i], is_dual)
    } else {
        select_642([a, c, b, d, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 7, i = 2
fn select_1011([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_48([a, b, c, d, f, g, e], is_dual)
    } else {
        select_48([a, b, c, e, f, g, d], is_dual)
    }
}
/// n = 9, i = 3
fn select_1012([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_554([a, d, b, c, f, e, h, g, i], is_dual)
    } else {
        select_432([a, d, b, c, f, g, h, e, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1010([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1011([e, c, d, a, f, g, h], is_dual)
    } else {
        select_1012([b, c, d, a, g, e, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1009([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1010([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1010([a, b, c, d, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1006([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1007([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1009([b, a, c, h, e, f, j, i, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_1001([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1002([a, c, b, e, d, f, g, h, j, i, l, k], is_dual)
    } else {
        select_1006([a, c, d, b, f, g, j, i, k, h, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_981([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_982([b, c, d, a, f, e, g, i, j, k, h, l, m], is_dual)
    } else {
        select_1001([b, c, d, h, f, g, i, a, j, k, l, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_932([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_933([a, c, d, b, e, i, j, g, h, m, l, n, o, k], is_dual)
    } else {
        select_981([b, d, c, a, f, h, g, i, j, l, k, o, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1018([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_458([a, b, c, e, h, g, i], is_dual)
    } else {
        select_922([a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1020([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_114([b, c, d, e, f, g, h], is_dual)
    } else {
        select_24([a, g, d, f, e, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1021([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_114([a, b, c, f, e, h, g], is_dual)
    } else {
        select_77([a, b, d, g, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_1019([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1020([a, b, c, d, f, g, j, i, k], is_dual)
    } else {
        select_1021([b, c, d, e, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1017([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1018([b, c, d, f, e, i, h, g, j], is_dual)
    } else {
        select_1019([a, c, d, b, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1024([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_221([a, e, f, c, d, h, i, j], is_dual)
    } else {
        select_62([a, b, e, f, g, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1023([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_796([a, b, e, f, g, j, i, k, h, l], is_dual)
    } else {
        select_1024([b, g, e, f, c, d, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1026([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_258([e, a, c, d, f, h, g, i, j, k], is_dual)
    } else {
        select_999([g, b, c, d, e, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1025([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1026([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_255([j, f, e, a, i], !is_dual)
    }
}
/// n = 13, i = 4
fn select_1022([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1023([b, c, a, d, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1025([b, i, e, f, a, g, h, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1016([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1017([b, e, c, d, a, g, k, h, i, j, l], is_dual)
    } else {
        select_1022([a, b, e, f, c, d, g, i, k, j, h, l, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_1030([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_242([a, b, d, e, g, h, j, k], is_dual)
    } else {
        select_670([a, b, c, e, f, i, j, h, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1031([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_149([e, c, d, a, f, g, h], is_dual)
    } else {
        select_750([b, d, a, c, f, e, h, g, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1029([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1030([b, c, d, a, f, g, h, j, l, i, k], is_dual)
    } else {
        select_1031([a, b, e, c, f, i, j, k, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1033([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_46([a, b, d, h, f, g, j, k], is_dual)
    } else {
        select_46([a, c, e, g, f, h, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1032([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_398([c, b, f, e, h, g, j, k, i], is_dual)
    } else {
        select_1033([a, b, c, g, d, f, i, h, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1028([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1029([a, b, c, e, h, g, i, f, j, l, m, k], is_dual)
    } else {
        select_1032([b, c, d, f, e, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_1035([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_397([a, b, f, d, e, g, h, j, k], is_dual)
    } else {
        select_549([b, a, d, c, e, f, h, g, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1037([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_732([c, d, a, e, g, h, j, k], is_dual)
    } else {
        select_263([a, b, e, f, i, j, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1038([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_62([b, f, d, e, g, h], is_dual)
    } else {
        select_156([a, b, c, h, i, f, j], is_dual)
    }
}
/// n = 14, i = 4
fn select_1036([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1037([a, e, c, i, d, h, g, k, j, m, n, l], is_dual)
    } else {
        select_1038([a, b, e, f, j, h, k, l, i, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1034([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_1035([k, b, c, f, a, g, j, i, n, m, l], is_dual)
    } else {
        select_1036([a, b, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1027([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1028([a, b, c, d, f, i, g, j, h, l, k, m, n], is_dual)
    } else {
        select_1034([a, d, b, c, f, e, g, h, j, k, l, i, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1015([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1016([a, c, d, g, e, f, h, j, l, i, k, m, n], is_dual)
    } else {
        select_1027([a, c, b, e, f, d, h, i, j, k, l, g, n, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1043([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_149([g, c, b, a, f, i, h], is_dual)
    } else {
        select_91([b, c, a, d, e, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1042([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_66([a, h, c, d, e, f, i, k], is_dual)
    } else {
        select_1043([e, b, f, c, d, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1041([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_410([e, a, d, c, f, g, i, j, h, l], is_dual)
    } else {
        select_1042([b, d, a, c, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1040([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1041([a, b, g, d, e, f, h, k, j, i, l, m], is_dual)
    } else {
        select_410([e, a, d, c, i, h, j, g, k, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1047([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_155([a, b, c, e, h, g, i], is_dual)
    } else {
        select_56([a, c, d, g, f, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1046([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_920([a, c, d, i, e, f, h, k], is_dual)
    } else {
        select_1047([e, c, h, b, f, g, j, i, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1050([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([g, h, c, e], is_dual)
    } else {
        select_116([a, e, b, d, f, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1049([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_205([b, c, d, g, f, h, i, j], is_dual)
    } else {
        select_1050([a, i, b, e, f, h, g, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1052([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_49([a, c, d, e, g, h, j], is_dual)
    } else {
        select_63([b, c, d, f, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1051([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_205([a, d, e, j, f, h, i, l], is_dual)
    } else {
        select_1052([a, b, c, i, f, g, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1048([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1049([a, b, d, e, h, g, f, i, j, k, l], is_dual)
    } else {
        select_1051([b, a, c, d, e, g, f, i, j, h, l, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1045([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1046([b, c, h, d, f, a, g, i, j, k, l], is_dual)
    } else {
        select_1048([b, c, d, a, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1054([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_252([g, i, f, b, a, h, c], !is_dual)
    } else {
        select_524([b, c, a, d, e, f, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1056([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_62([g, e, c, d, f, h], is_dual)
    } else {
        select_143([h, j, k, a, b, i, f], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1055([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1056([a, b, c, d, e, g, f, h, i, j, k], is_dual)
    } else {
        select_1052([a, e, c, d, f, g, b, i, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1053([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1054([a, b, c, d, j, g, l, i, m], is_dual)
    } else {
        select_1055([a, b, f, i, e, g, h, k, j, m, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1044([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1045([c, a, f, b, e, g, i, h, j, l, k, m], is_dual)
    } else {
        select_1053([a, b, c, h, f, d, g, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1039([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1040([c, b, e, f, g, j, h, i, a, l, k, n, m], is_dual)
    } else {
        select_1044([b, a, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1014([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < l) || (is_dual && n > l) {
        select_1015([b, c, d, e, a, j, g, h, i, m, l, k, o, n], is_dual)
    } else {
        select_1039([a, c, b, h, f, e, g, j, i, k, m, n, l, p], is_dual)
    }
}
/// n = 9, i = 3
fn select_1061([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_22([a, b, c, d, e, f, g, i], is_dual)
    } else {
        select_78([a, b, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1060([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_889([a, b, d, h, f, g, j, i, l, k, n], is_dual)
    } else {
        select_1061([a, c, e, g, i, h, k, m, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1064([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_118([b, h, c, d, e, g, i], is_dual)
    } else {
        select_105([a, g, c, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1063([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_890([b, i, c, a, f, g, h, j, k, l], is_dual)
    } else {
        select_1064([b, c, a, d, e, f, g, h, i, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1062([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_21([i, b, c, e, h, f, k, j], is_dual)
    } else {
        select_1063([f, a, d, b, c, g, h, i, j, l, k, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1059([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1060([f, a, b, e, c, h, i, j, k, g, l, m, n, o], is_dual)
    } else {
        select_1062([a, c, g, e, d, f, h, i, k, j, l, m, o], is_dual)
    }
}
/// n = 9, i = 4
fn select_1068([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_280([a, b, c, d, e, f, g], is_dual)
    } else {
        select_58([f, e, h, i, a, g, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1067([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1068([a, e, c, d, f, h, g, i, j], is_dual)
    } else {
        select_686([h, i, f, j, a, e, b, g], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1069([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_594([a, b, h, c, f, g, e, i, j], is_dual)
    } else {
        select_893([a, f, c, d, e, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1066([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1067([a, b, f, g, e, h, j, k, l, i], is_dual)
    } else {
        select_1069([a, e, c, d, h, i, f, g, k, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_1072([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_529([d, f, g, e, a, b, c], !is_dual)
    } else {
        select_55([b, c, d, e, a, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1073([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_205([b, a, c, d, e, f, g, h], is_dual)
    } else {
        select_189([g, b, a, e, h, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1071([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1072([h, i, j, a, e, b, g], !is_dual)
    } else {
        select_1073([a, b, c, d, e, f, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1070([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_21([a, c, h, d, e, i, g, j], is_dual)
    } else {
        select_1071([a, b, c, g, f, i, j, k, l, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_1065([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1066([a, b, c, e, f, j, g, h, k, i, l, m], is_dual)
    } else {
        select_1070([a, b, c, d, g, f, i, h, k, j, m, l], is_dual)
    }
}
/// n = 16, i = 5
fn select_1058([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1059([c, b, a, h, e, j, f, g, i, l, k, n, m, p, o], is_dual)
    } else {
        select_1065([a, c, f, i, d, g, h, k, j, m, n, l, o], is_dual)
    }
}
/// n = 7, i = 2
fn select_1078([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([c, d, a, e, f, b, g], is_dual)
    } else {
        select_23([c, d, b, e, f, a, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_1077([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_76([b, c, a, e, d, f], is_dual)
    } else {
        select_1078([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1076([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1071([a, b, c, d, f, h, i, j, k, g], is_dual)
    } else {
        select_1077([a, c, d, g, e, h, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1081([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_118([a, b, c, d, i, g, h], is_dual)
    } else {
        select_712([e, h, a, b, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1080([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1081([b, c, d, e, a, f, g, i, h, j, k], is_dual)
    } else {
        select_891([a, g, d, e, f, b, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1082([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1072([i, j, k, a, f, b, h], !is_dual)
    } else {
        select_57([a, b, c, d, e, f, g, h, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1079([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1080([b, c, e, a, d, f, g, i, h, j, k], is_dual)
    } else {
        select_1082([a, b, c, d, g, f, i, j, k, h, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1075([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1076([a, b, c, d, k, g, i, m, j, n, l], is_dual)
    } else {
        select_1079([a, b, e, j, f, g, h, l, i, k, n, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_1086([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_205([b, d, e, g, f, h, i, j], is_dual)
    } else {
        select_805([a, b, c, i, f, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1085([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_458([c, a, b, g, i, j, h], is_dual)
    } else {
        select_1086([c, d, e, a, b, f, i, g, h, k, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1088([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_114([a, b, c, i, h, g, k], is_dual)
    } else {
        select_216([c, d, g, e, f, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1089([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_375([c, f, d, e, i, g, h], is_dual)
    } else {
        select_41([c, a, b, h, j, f], is_dual)
    }
}
/// n = 11, i = 3
fn select_1087([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1088([b, c, a, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1089([b, c, a, d, f, h, i, k, j, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1084([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1085([a, b, f, e, d, h, j, i, g, l, k], is_dual)
    } else {
        select_1087([g, a, b, c, e, h, f, i, j, k, l], is_dual)
    }
}
/// n = 7, i = 3
fn select_1093([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_24([a, b, c, d, e, g], is_dual)
    } else {
        select_71([f, g, a, d, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1092([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_33([e, c, d, f, g], is_dual)
    } else {
        select_1093([a, g, b, e, h, f, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1091([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1092([a, b, c, f, i, g, j, h, k], is_dual)
    } else {
        select_765([b, d, c, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1095([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_114([b, e, c, d, f, g, h], is_dual)
    } else {
        select_670([h, e, j, f, i, a, g, c, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1094([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_1095([a, b, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_357([f, b, c, e, i, h], is_dual)
    }
}
/// n = 14, i = 4
fn select_1090([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1091([a, e, d, j, f, g, i, h, l, k, n], is_dual)
    } else {
        select_1094([a, b, c, e, k, h, i, j, m, l, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1083([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1084([c, d, e, g, f, i, a, h, j, k, l, m], is_dual)
    } else {
        select_1090([b, c, d, e, a, f, h, i, g, j, k, l, m, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1074([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < l) || (is_dual && n > l) {
        select_1075([a, c, d, e, g, f, h, i, k, j, l, m, n, o], is_dual)
    } else {
        select_1083([a, b, d, e, h, f, g, i, k, j, n, m, l, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1057([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1058([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o, p], is_dual)
    } else {
        select_1074([b, c, d, a, e, f, g, h, j, i, k, m, l, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1013([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < k) || (is_dual && n > k) {
        select_1014([a, b, c, e, f, d, g, i, j, h, l, k, m, o, n, p], is_dual)
    } else {
        select_1057([a, b, c, e, d, f, g, i, h, j, l, m, n, o, k, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_931([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_932([a, b, c, d, f, i, h, j, k, m, l, n, g, o, p], is_dual)
    } else {
        select_1013([a, c, d, f, e, b, h, j, i, g, l, m, n, o, k, p], is_dual)
    }
}
/// n = 10, i = 4
fn select_1102([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_509([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_68([a, f, d, e, b, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1104([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_375([e, g, c, d, f, h, i], is_dual)
    } else {
        select_118([c, d, a, b, f, h, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1103([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_126([b, g, d, e, f, a, h, i, j], is_dual)
    } else {
        select_1104([a, c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1101([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1102([f, a, c, d, h, g, j, k, i, m], is_dual)
    } else {
        select_1103([a, b, e, d, g, f, h, i, l, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1107([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([b, c, d, e, g, f, h], is_dual)
    } else {
        select_495([a, b, f, c, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1106([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_279([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_1107([f, b, d, e, a, g, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1108([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_557([b, c, d, a, e, f, g, h], is_dual)
    } else {
        select_156([b, a, d, e, c, f, g], is_dual)
    }
}
/// n = 10, i = 3
fn select_1105([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1106([a, c, b, e, d, f, g, h, i, j], is_dual)
    } else {
        select_1108([a, c, b, g, h, i, f, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1100([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1101([a, c, b, h, e, f, g, i, j, l, m, k, n], is_dual)
    } else {
        select_1105([a, c, d, e, j, f, i, h, k, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_1111([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_54([d, c, a, f, e, h], is_dual)
    } else {
        select_315([c, a, b, e, d, g, h, f], is_dual)
    }
}
/// n = 7, i = 3
fn select_1112([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_54([b, a, c, e, d, f], is_dual)
    } else {
        select_81([c, a, e, d, g], is_dual)
    }
}
/// n = 11, i = 5
fn select_1110([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1111([a, d, c, f, e, g, h, j], is_dual)
    } else {
        select_1112([b, a, h, d, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1115([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_24([a, d, f, h, e, j], is_dual)
    } else {
        select_82([b, c, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1116([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_49([c, a, b, d, f, e, g], is_dual)
    } else {
        select_82([a, b, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1114([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1115([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_1116([c, d, e, b, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1117([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_255([d, j, a, b, h], !is_dual)
    } else {
        select_1116([b, d, c, e, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1113([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1114([b, d, a, c, e, f, g, h, i, j], is_dual)
    } else {
        select_1117([b, d, c, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1109([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1110([a, b, c, d, e, f, i, h, g, k, j], is_dual)
    } else {
        select_1113([j, i, h, k, f, a, d, g, b, e], !is_dual)
    }
}
/// n = 14, i = 5
fn select_1099([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < a) || (is_dual && k > a) {
        select_1100([b, c, a, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_1109([b, c, h, g, k, j, a, m, l, f, n], is_dual)
    }
}
/// n = 6, i = 1
fn select_1122([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_33([a, b, c, d, f], is_dual)
    } else {
        select_33([a, c, d, e, f], is_dual)
    }
}
/// n = 12, i = 4
fn select_1121([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_608([k, l, h, j, b, a, g, c, i], !is_dual)
    } else {
        select_1122([b, d, e, f, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1124([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_70([b, e, c, d, g, f], is_dual)
    } else {
        select_150([e, g, h, a, f, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1123([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1124([i, j, g, k, a, f, h, b], !is_dual)
    } else {
        select_1122([a, c, d, e, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1120([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1121([a, b, c, d, e, f, g, h, i, j, k, m], is_dual)
    } else {
        select_1123([b, g, d, e, f, h, a, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1127([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_216([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_750([a, d, f, c, g, e, i, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1126([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1127([c, a, d, e, f, b, g, h, i, j], is_dual)
    } else {
        select_1127([c, b, d, e, f, a, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1125([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_212([a, b, d, e, f, i, h, j], is_dual)
    } else {
        select_1126([a, b, c, j, d, g, h, i, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_1119([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1120([a, b, d, c, e, h, g, j, k, i, m, n, l], is_dual)
    } else {
        select_1125([a, c, d, f, b, e, g, h, j, i, l, k], is_dual)
    }
}
/// n = 15, i = 5
fn select_1118([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1119([b, a, c, d, h, f, g, i, k, j, m, l, n, o], is_dual)
    } else {
        select_1119([a, b, c, d, i, e, g, h, l, j, m, k, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_1098([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1099([a, b, e, d, c, l, h, g, m, j, k, o, n, p], is_dual)
    } else {
        select_1118([a, b, c, e, f, g, h, i, k, j, l, n, m, p, o], is_dual)
    }
}
/// n = 8, i = 3
fn select_1133([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_41([e, a, g, h, b, f], !is_dual)
    } else {
        select_58([b, a, c, d, e, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1132([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1133([a, e, h, i, f, g, b, c], !is_dual)
    } else {
        select_1133([a, f, h, i, e, g, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1131([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_21([b, c, d, e, a, g, f, h], is_dual)
    } else {
        select_1132([g, i, f, j, a, b, h, c, d], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1136([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_54([a, e, d, g, h, i], is_dual)
    } else {
        select_192([b, c, d, g, e, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1135([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_875([a, c, d, e, b, f, g, h, i], is_dual)
    } else {
        select_1136([b, c, d, a, f, e, g, h, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1138([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_41([e, a, i, j, b, g], !is_dual)
    } else {
        select_296([b, a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1139([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_280([b, d, c, e, f, h, g], is_dual)
    } else {
        select_89([i, e, j, a, b, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1137([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1138([a, b, c, e, h, f, i, g, j, k], is_dual)
    } else {
        select_1139([a, b, d, e, g, f, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1134([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1135([f, a, d, e, b, g, h, i, j], is_dual)
    } else {
        select_1137([a, b, d, c, e, g, h, f, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1130([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1131([a, b, c, d, k, h, l, i, j, m], is_dual)
    } else {
        select_1134([a, b, i, e, f, j, g, h, k, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_1143([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_149([d, b, c, g, f, e, h], is_dual)
    } else {
        select_670([g, h, j, d, i, a, f, e, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1144([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_183([a, b, c, e, f, h, i, j], is_dual)
    } else {
        select_192([b, c, d, h, f, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_1142([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1143([a, b, g, e, f, j, i, k, h, l], is_dual)
    } else {
        select_1144([a, c, d, f, e, g, h, i, j, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_1145([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1143([a, f, b, d, i, e, g, j, h, k], is_dual)
    } else {
        select_1143([a, e, c, d, h, f, g, j, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1141([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1142([a, f, c, d, e, h, g, j, i, k, m, l], is_dual)
    } else {
        select_1145([a, b, g, e, f, k, i, h, l, m, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1148([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_29([b, d, c, e, g, f, h], is_dual)
    } else {
        select_305([a, c, f, e, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1147([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_950([f, a, c, d, g, e, h, i, j], is_dual)
    } else {
        select_1148([a, c, b, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1149([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_188([e, i, j, d, f, h, a, g], !is_dual)
    } else {
        select_371([d, f, b, c, e, g, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1146([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1147([a, i, c, d, e, j, g, h, k, l], is_dual)
    } else {
        select_1149([a, k, b, h, l, e, f, i, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1140([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1141([b, c, d, e, a, f, g, i, h, j, k, l, m], is_dual)
    } else {
        select_1146([a, c, d, e, b, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1129([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1130([a, c, b, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_1140([a, c, d, e, f, g, h, i, b, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1154([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_101([a, b, d, g, f, h], is_dual)
    } else {
        select_107([g, i, a, h, e, b, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1153([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_923([a, b, c, d, e, f, g, i, h, j], is_dual)
    } else {
        select_1154([a, b, f, d, h, g, e, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1156([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([a, b, c, d, e, f, g], is_dual)
    } else {
        select_38([d, e, f, h, g, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1155([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_22([b, c, a, d, f, e, g, h], is_dual)
    } else {
        select_1156([e, i, j, a, b, f, h, g], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1152([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1153([a, b, d, c, e, g, f, h, i, j], is_dual)
    } else {
        select_1155([a, b, g, c, e, h, f, j, i, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1159([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_221([f, a, g, d, e, h, i, j], is_dual)
    } else {
        select_118([d, e, b, c, i, h, g], is_dual)
    }
}
/// n = 10, i = 2
fn select_1158([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1159([c, b, d, e, f, a, g, i, h, j], is_dual)
    } else {
        select_973([a, c, g, e, f, b, h, i, j], is_dual)
    }
}
/// n = 7, i = 1
fn select_1161([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_33([a, c, d, e, g], is_dual)
    } else {
        select_62([a, b, c, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1160([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1161([b, c, d, e, f, g, h], is_dual)
    } else {
        select_134([f, j, k, a, b, i, g, h, c], !is_dual)
    }
}
/// n = 13, i = 4
fn select_1157([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1158([a, c, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_1160([a, b, d, c, h, i, g, k, l, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1151([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && c < k) || (is_dual && c > k) {
        select_1152([a, b, d, h, i, g, k, c, j, l, m], is_dual)
    } else {
        select_1157([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1166([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_24([a, b, d, g, e, h], is_dual)
    } else {
        select_82([e, f, c, d], is_dual)
    }
}
/// n = 10, i = 3
fn select_1165([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_205([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1166([a, h, b, f, e, i, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1167([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_205([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_662([i, j, k, e, g, a, f, h, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1164([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1165([a, c, d, e, b, f, g, h, i, k], is_dual)
    } else {
        select_1167([b, c, d, e, a, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1163([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_21([b, c, d, f, a, i, h, j], is_dual)
    } else {
        select_1164([b, a, e, c, d, g, i, j, k, h, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1170([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_149([b, d, c, f, e, g, h], is_dual)
    } else {
        select_98([a, d, b, h, f, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1171([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_56([d, b, c, e, f, g], is_dual)
    } else {
        select_156([d, h, i, a, e, g, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1169([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1170([a, c, d, b, f, g, e, h, i], is_dual)
    } else {
        select_1171([a, e, d, g, b, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1168([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1169([a, b, e, d, g, f, i, j, h, k], is_dual)
    } else {
        select_1169([b, a, f, c, h, e, j, i, g, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1162([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_1163([a, b, c, d, e, f, g, i, j, h, k, l], is_dual)
    } else {
        select_1168([a, b, e, d, g, h, j, c, i, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1150([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1151([a, b, c, e, g, f, h, i, k, l, j, m, n], is_dual)
    } else {
        select_1162([b, a, c, d, e, j, h, g, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1128([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_1129([b, a, c, d, i, e, g, h, l, k, m, j, n], is_dual)
    } else {
        select_1150([a, b, c, d, f, e, g, i, h, j, k, l, m, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1097([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < m) || (is_dual && o > m) {
        select_1098([a, b, c, d, f, e, g, i, h, j, k, l, m, n, p, o], is_dual)
    } else {
        select_1128([a, b, c, e, f, g, h, i, k, j, l, n, o, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1178([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_183([b, d, a, e, c, g, f, h], is_dual)
    } else {
        select_180([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1177([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_477([a, c, e, d, h, g, f], is_dual)
    } else {
        select_1178([a, c, b, f, g, e, i, h], is_dual)
    }
}
/// n = 7, i = 3
fn select_1180([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_359([a, c, d, b, e, f, g], is_dual)
    } else {
        select_359([b, c, d, a, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1179([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_742([a, c, d, e, g, f, h], is_dual)
    } else {
        select_1180([a, f, g, i, h, e, b], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1176([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1177([b, c, a, f, e, g, j, h, k], is_dual)
    } else {
        select_1179([a, d, b, e, f, h, g, i, l], is_dual)
    }
}
/// n = 6, i = 2
fn select_1182([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_246([a, b, c, d, e, f], is_dual)
    } else {
        select_246([a, b, c, e, d, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1183([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_123([b, a, f, d, e, h, g, k], is_dual)
    } else {
        select_390([a, c, b, g, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1181([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1182([b, h, d, e, i, g], is_dual)
    } else {
        select_1183([a, b, c, d, e, f, g, h, j, k, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1175([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1176([b, c, d, e, f, g, a, h, j, i, k, l], is_dual)
    } else {
        select_1181([c, b, d, a, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1187([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_221([b, c, f, d, e, h, g, i], is_dual)
    } else {
        select_375([a, g, d, e, f, h, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1186([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_749([b, c, g, d, a, f, h, i, j], is_dual)
    } else {
        select_1187([b, c, d, a, e, f, h, g, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1188([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_613([a, b, c, e, f, h], is_dual)
    } else {
        select_191([e, b, c, d, f, g, h], is_dual)
    }
}
/// n = 11, i = 3
fn select_1185([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1186([f, a, c, e, b, g, h, i, j, k], is_dual)
    } else {
        select_1188([c, b, d, g, i, f, j, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1191([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_432([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_305([e, c, d, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1192([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_183([a, c, g, f, b, h, i, j], is_dual)
    } else {
        select_105([a, c, b, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1190([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1191([a, b, c, d, f, h, i, j, g], is_dual)
    } else {
        select_1192([b, c, e, a, d, f, g, h, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1193([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_992([b, c, d, a, f, e, h, g, i], is_dual)
    } else {
        select_308([a, b, c, f, g, h, i, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1189([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1190([a, b, c, d, e, g, f, h, i, j], is_dual)
    } else {
        select_1193([f, b, c, e, a, g, i, h, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1184([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1185([c, d, a, e, g, f, h, i, j, k, n], is_dual)
    } else {
        select_1189([a, b, f, d, h, i, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1174([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1175([i, a, b, g, c, j, h, m, k, f, n, l], is_dual)
    } else {
        select_1184([a, c, b, d, e, g, h, f, i, k, l, j, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1198([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_42([b, e, d, a, f, h], is_dual)
    } else {
        select_495([b, c, d, a, f, e, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1197([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_586([c, a, d, e, f, g, h, i], is_dual)
    } else {
        select_1198([a, b, c, g, f, e, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1200([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_143([a, f, b, d, g, e, h], is_dual)
    } else {
        select_143([a, e, c, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1201([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_529([e, h, i, f, a, g, b], !is_dual)
    } else {
        select_115([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1199([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1200([b, c, g, e, f, a, i, h], is_dual)
    } else {
        select_1201([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1196([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1197([a, c, d, b, i, f, h, j, g, k], is_dual)
    } else {
        select_1199([b, d, a, e, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1204([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([b, c, e, f, g, h], is_dual)
    } else {
        select_70([a, c, d, g, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1205([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_62([a, c, d, e, f, g], is_dual)
    } else {
        select_305([b, a, d, e, f, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1203([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1204([a, g, b, e, d, f, h, i, k], is_dual)
    } else {
        select_1205([b, c, f, d, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1206([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_742([a, d, g, c, h, f, k], is_dual)
    } else {
        select_1205([a, b, c, f, e, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1202([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1203([a, b, c, e, i, g, f, j, h, k, l], is_dual)
    } else {
        select_1206([a, c, d, b, f, h, j, i, g, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1195([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1196([g, a, b, c, e, j, f, h, i, k, l], is_dual)
    } else {
        select_1202([a, c, b, e, d, f, h, j, i, g, l, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1210([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_42([a, c, g, d, f, j], is_dual)
    } else {
        select_646([b, c, f, e, d, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1211([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_247([d, c, f, h, e], is_dual)
    } else {
        select_224([a, b, e, d, g, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1209([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1210([a, b, c, d, e, f, g, i, j, h], is_dual)
    } else {
        select_1211([i, j, g, d, h, f, e, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1213([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_148([c, h, b, e, f, i, g, k], is_dual)
    } else {
        select_280([a, c, d, g, e, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1214([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_529([a, b, g, f, e, h, j], is_dual)
    } else {
        select_46([b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1212([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1213([b, a, c, e, g, f, h, j, k, i, l], is_dual)
    } else {
        select_1214([g, a, b, d, i, f, j, k, h, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1208([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1209([b, d, h, a, g, f, j, k, i, l], is_dual)
    } else {
        select_1212([b, c, d, a, e, f, g, i, j, h, k, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_1216([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_749([c, d, f, e, a, g, h, i, j], is_dual)
    } else {
        select_1033([c, b, d, a, e, g, f, h, i, k, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1219([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([g, i, h, d, f, a], !is_dual)
    } else {
        select_47([e, b, c, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1218([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_130([b, d, c, e, g, f, h], is_dual)
    } else {
        select_1219([h, i, j, e, a, g, b, f, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1220([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_280([b, c, d, e, f, g, i], is_dual)
    } else {
        select_224([e, h, j, a, f, g, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1217([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1218([a, d, b, e, f, c, g, h, i, j], is_dual)
    } else {
        select_1220([a, d, c, e, f, b, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1215([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1216([a, d, e, b, c, f, g, h, k, j, i], is_dual)
    } else {
        select_1217([d, c, b, a, g, f, i, k, l, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1207([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1208([a, d, c, b, e, h, f, i, j, g, k, m], is_dual)
    } else {
        select_1215([a, g, b, c, d, i, h, f, j, k, m, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_1194([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1195([a, b, d, e, h, f, g, k, j, i, l, n], is_dual)
    } else {
        select_1207([a, b, g, c, d, f, i, h, l, k, m, n, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1173([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1174([c, b, d, e, f, a, g, h, i, k, l, j, m, n], is_dual)
    } else {
        select_1194([c, a, d, e, f, b, g, h, i, j, l, k, m, n], is_dual)
    }
}
/// n = 9, i = 4
fn select_1226([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_242([b, a, c, d, e, f, g, i], is_dual)
    } else {
        select_189([h, i, d, g, b, a, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1225([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_88([b, d, c, g, f, e, h, j], is_dual)
    } else {
        select_1226([h, g, k, d, i, a, f, e, c], !is_dual)
    }
}
/// n = 6, i = 2
fn select_1227([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_613([a, c, d, b, e, f], is_dual)
    } else {
        select_613([b, c, d, a, e, f], is_dual)
    }
}
/// n = 12, i = 5
fn select_1224([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1225([j, l, k, h, g, i, a, b, f, e, d], !is_dual)
    } else {
        select_1227([a, g, c, i, k, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1230([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_77([b, c, a, h, j], is_dual)
    } else {
        select_77([e, f, d, g, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1229([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1230([f, d, e, g, a, c, h, i, l, k], is_dual)
    } else {
        select_434([a, b, c, h, g, j, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1231([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_76([a, b, d, e, g, h], is_dual)
    } else {
        select_205([c, a, b, e, f, g, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1228([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1229([a, b, g, c, d, f, h, j, i, l, k, m, n], is_dual)
    } else {
        select_1231([c, d, a, e, f, i, g, k, j, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1223([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1224([b, g, c, d, h, j, a, i, l, k, m, n], is_dual)
    } else {
        select_1228([b, c, a, d, e, f, g, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1234([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_384([b, a, c, f, e, h, i], is_dual)
    } else {
        select_1021([a, c, b, d, e, f, g, i], is_dual)
    }
}
/// n = 7, i = 2
fn select_1236([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_375([a, b, c, d, e, f, g], is_dual)
    } else {
        select_63([e, c, d, b, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1235([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_76([a, c, d, e, f, g], is_dual)
    } else {
        select_1236([b, a, c, f, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1233([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1234([a, b, c, d, f, g, h, e, i], is_dual)
    } else {
        select_1235([a, b, d, c, f, e, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1239([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_114([a, b, d, c, e, f, h], is_dual)
    } else {
        select_114([a, c, d, b, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1238([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_326([e, a, d, c, f, h, g], is_dual)
    } else {
        select_1239([a, b, d, c, g, h, f, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1237([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_747([a, c, e, g, f, b, h], is_dual)
    } else {
        select_1238([a, c, d, e, b, f, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1232([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_1233([a, b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_1237([a, h, c, b, e, f, g, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1222([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1223([a, b, d, c, e, f, g, h, j, k, l, m, n, i], is_dual)
    } else {
        select_1232([c, b, e, a, f, g, j, i, l, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1244([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_241([e, a, b, f, g, i, j], is_dual)
    } else {
        select_444([a, c, d, g, e, f, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1245([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_340([a, b, e, h, g, d, f], is_dual)
    } else {
        select_42([a, d, c, f, h, e], is_dual)
    }
}
/// n = 10, i = 3
fn select_1243([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1244([c, b, a, d, e, f, g, h, j, i], is_dual)
    } else {
        select_1245([a, c, f, g, i, h, e, j], is_dual)
    }
}
/// n = 5, i = 2
fn select_1248([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_47([a, b, c, d], is_dual)
    } else {
        select_47([a, b, c, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1247([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_67([a, c, e, d, g, f], is_dual)
    } else {
        select_1248([d, b, f, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1249([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_23([b, d, c, e, g, f, h], is_dual)
    } else {
        select_71([d, i, a, e, h], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1246([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1247([d, b, f, g, e, i, j, k], is_dual)
    } else {
        select_1249([a, e, c, d, h, f, g, j, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_1242([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1243([b, d, e, c, h, i, g, j, l, k], is_dual)
    } else {
        select_1246([a, d, c, f, g, k, h, j, i, m, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1252([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_89([d, b, c, f, e, g], is_dual)
    } else {
        select_42([f, h, d, e, a, g], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1251([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_230([d, b, c, e, f, h], is_dual)
    } else {
        select_1252([a, c, b, d, e, f, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1254([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_54([a, c, d, e, f, h], is_dual)
    } else {
        select_55([a, b, d, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1255([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_89([a, b, e, d, f, h], is_dual)
    } else {
        select_155([d, a, c, g, e, i, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1253([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1254([c, d, b, f, h, e, i, g], is_dual)
    } else {
        select_1255([a, e, c, g, f, i, d, j, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1250([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1251([j, l, g, d, h, e, i, a, f], !is_dual)
    } else {
        select_1253([b, c, e, d, f, g, h, j, k, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1241([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1242([b, a, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    } else {
        select_1250([b, g, d, a, f, h, j, k, m, i, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1258([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_374([a, b, e, c, d, f, g, h], is_dual)
    } else {
        select_374([a, b, e, d, c, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1260([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_42([a, d, c, e, f, h], is_dual)
    } else {
        select_107([a, b, f, d, g, h, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1259([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_634([f, h, g, i, a, e, d, c], !is_dual)
    } else {
        select_1260([a, c, b, e, g, f, d, i, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1257([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1258([a, f, b, c, g, j, h, i], is_dual)
    } else {
        select_1259([a, h, d, i, e, k, f, l, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1263([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_89([a, c, e, d, f, g], is_dual)
    } else {
        select_1248([d, b, g, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1264([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_322([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_24([b, d, e, c, f, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1262([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1263([b, c, a, d, e, g, f, h], is_dual)
    } else {
        select_1264([d, g, h, i, a, e, f, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1266([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1166([d, a, c, f, e, g, h, j], is_dual)
    } else {
        select_670([g, i, j, d, h, e, f, b, c], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1267([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_440([d, a, b, g, e, f, i, j, h], is_dual)
    } else {
        select_440([d, a, c, f, e, g, h, j, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1265([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1266([e, d, c, g, a, f, j, i, h, k], is_dual)
    } else {
        select_1267([c, a, d, b, f, e, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1261([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1262([a, j, b, h, e, k, i, m, g], is_dual)
    } else {
        select_1265([a, e, c, d, g, f, i, h, j, l, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1256([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1257([a, b, e, c, f, h, j, g, k, l, i, m], is_dual)
    } else {
        select_1261([a, c, b, d, f, g, h, i, j, e, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1240([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1241([a, c, b, e, d, g, f, h, i, j, k, m, l, n], is_dual)
    } else {
        select_1256([a, c, e, f, b, h, g, k, i, l, j, n, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1221([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1222([a, c, b, d, e, f, g, h, j, k, i, m, l, n], is_dual)
    } else {
        select_1240([a, c, d, e, f, g, h, i, j, k, b, l, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1172([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1173([a, b, c, d, h, f, j, i, g, k, m, l, o, n], is_dual)
    } else {
        select_1221([a, b, c, d, e, g, j, i, h, k, l, m, n, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_1096([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < j) || (is_dual && m > j) {
        select_1097([a, b, c, d, f, e, g, i, h, k, j, l, n, o, m, p], is_dual)
    } else {
        select_1172([a, b, c, f, d, h, k, g, i, m, l, n, j, p, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_930([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_931([b, a, c, d, e, f, g, h, i, j, l, m, n, o, k, p], is_dual)
    } else {
        select_1096([b, a, c, d, e, f, h, i, j, k, l, m, n, o, g, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_533([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_534([a, b, c, e, f, d, g, i, k, j, l, m, o, h, n, p], is_dual)
    } else {
        select_930([a, b, d, e, f, c, g, i, k, h, l, m, n, o, j, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_532([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_533([d, e, f, b, c, a, j, l, i, h, g, n, m, k, o, p], is_dual)
    } else {
        select_533([d, f, e, b, c, a, j, k, i, h, g, n, m, l, o, p], is_dual)
    }
}
/// n = 10, i = 4
fn select_1277([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_634([g, i, a, j, e, h, b, f], !is_dual)
    } else {
        select_464([a, b, c, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1278([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_191([a, c, d, b, f, e, g], is_dual)
    } else {
        select_1180([a, f, e, h, b, g, c], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1276([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1277([a, b, f, d, e, g, j, h, k, i], is_dual)
    } else {
        select_1278([a, b, c, h, e, i, f, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1281([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_23([b, d, c, g, f, e, j], is_dual)
    } else {
        select_107([a, e, g, h, d, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1280([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_950([a, b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_1281([b, c, d, e, f, a, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1279([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1278([a, b, c, h, e, j, f, i], is_dual)
    } else {
        select_1280([a, b, f, d, e, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1275([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1276([a, c, d, e, b, f, g, h, j, i, k], is_dual)
    } else {
        select_1279([a, b, d, e, c, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1285([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_216([c, a, d, b, e, f, g, h], is_dual)
    } else {
        select_980([h, j, a, e, i, g, c], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1284([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_1285([d, i, j, k, e, f, h, a, g, c], !is_dual)
    } else {
        select_134([f, i, k, d, e, h, g, b, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1287([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_33([d, b, c, e, f], is_dual)
    } else {
        select_280([d, i, e, h, g, a, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1286([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_246([d, e, b, c, f, g], is_dual)
    } else {
        select_1287([a, b, c, d, e, g, h, f, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1283([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1284([a, f, c, d, e, h, g, i, k, l, j], is_dual)
    } else {
        select_1286([a, b, i, d, e, j, f, h, l], is_dual)
    }
}
/// n = 11, i = 4
fn select_1289([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, h, b, d, e, j, g, k], is_dual)
    } else {
        select_183([a, g, c, d, f, i, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1288([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_852([b, d, c, e, g, f, i, h, j], is_dual)
    } else {
        select_1289([a, b, c, d, e, f, h, i, g, j, k], is_dual)
    }
}
/// n = 12, i = 5
fn select_1282([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1283([b, c, d, a, e, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1288([a, c, d, b, f, g, e, h, i, j, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1274([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1275([a, b, f, d, e, g, h, i, j, k, l], is_dual)
    } else {
        select_1282([a, c, d, e, f, g, h, j, i, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_1294([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_148([b, c, a, e, d, h, g, f], is_dual)
    } else {
        select_507([h, d, i, e, a, f], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1295([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_507([a, c, e, d, f, h], is_dual)
    } else {
        select_143([a, b, f, d, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1293([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1294([b, a, c, f, d, g, h, e, i], is_dual)
    } else {
        select_1295([d, h, i, e, f, g, a, b], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1297([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_54([b, a, d, f, g, h], is_dual)
    } else {
        select_196([a, c, d, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1296([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_585([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1297([c, b, d, e, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1292([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1293([a, b, g, d, h, f, e, j, i], is_dual)
    } else {
        select_1296([d, b, c, e, f, i, g, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1300([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_149([a, b, c, d, f, e, g], is_dual)
    } else {
        select_189([e, c, a, f, d, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1301([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_359([a, c, e, d, h, f, i], is_dual)
    } else {
        select_127([d, g, h, i, a, e, b, f], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1299([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1300([a, e, c, d, g, f, i, h], is_dual)
    } else {
        select_1301([a, b, f, d, e, h, j, i, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1303([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_127([f, h, g, i, a, d, b, e], !is_dual)
    } else {
        select_98([a, d, c, g, f, e, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1302([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1127([b, a, c, d, f, e, g, h, j, i], is_dual)
    } else {
        select_1303([a, b, g, f, i, j, h, e, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1298([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1299([a, b, c, e, g, h, i, f, k, j], is_dual)
    } else {
        select_1302([a, b, d, c, e, g, f, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1291([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1292([a, b, c, e, h, i, g, j, f, k], is_dual)
    } else {
        select_1298([a, c, b, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1307([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_344([a, b, c, d, e, f, h, i, g], is_dual)
    } else {
        select_1116([h, i, e, g, f, d, b, a], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1306([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_307([a, b, c, e, d, f, i, g, h], is_dual)
    } else {
        select_1307([b, a, c, e, f, g, i, d, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1309([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_267([h, i, f, g, d, b, a, e], !is_dual)
    } else {
        select_161([b, a, c, d, f, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1308([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1309([i, j, g, d, h, a, e, b, f], !is_dual)
    } else {
        select_719([a, b, c, d, e, f, g, i, j, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1305([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_1306([a, b, f, h, e, g, i, d, j], is_dual)
    } else {
        select_1308([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1312([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_865([a, g, d, e, f, h, i, j], is_dual)
    } else {
        select_389([a, b, c, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1313([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_29([a, d, b, c, e, f, g], is_dual)
    } else {
        select_574([a, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1311([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1312([a, c, d, b, e, f, h, g, i, j], is_dual)
    } else {
        select_1313([a, c, d, g, f, b, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1310([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_572([a, b, c, d, e, g, h, k, j, l, m], is_dual)
    } else {
        select_1311([a, c, d, e, f, g, i, j, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1304([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < k) || (is_dual && a > k) {
        select_1305([b, c, i, g, h, k, j, a, l, m], is_dual)
    } else {
        select_1310([b, c, a, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1290([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < k) || (is_dual && h > k) {
        select_1291([a, b, c, k, f, g, i, m, h, n, l], is_dual)
    } else {
        select_1304([g, a, c, d, h, e, f, i, j, l, k, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1273([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1274([f, a, b, i, e, h, k, j, l, g, m, o, n], is_dual)
    } else {
        select_1290([b, c, a, d, e, h, f, i, g, j, l, k, m, n], is_dual)
    }
}
/// n = 9, i = 4
fn select_1319([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_148([h, f, i, d, g, a, b, e], !is_dual)
    } else {
        select_47([d, c, f, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_1318([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1319([b, a, c, e, f, d, g, h, i], is_dual)
    } else {
        select_101([d, e, c, a, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1320([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_601([a, g, h, d, e, f, k, i, l], is_dual)
    } else {
        select_765([a, i, b, c, f, g, h, j, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1317([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < l) || (is_dual && f > l) {
        select_1318([a, b, h, k, g, l, f, m, j], is_dual)
    } else {
        select_1320([b, a, c, d, e, g, f, h, i, k, j, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1322([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_66([b, d, a, c, e, g, f, h], is_dual)
    } else {
        select_810([a, b, c, g, e, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1323([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_481([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_49([g, c, d, b, f, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1321([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1322([a, b, c, h, f, g, j, k, l], is_dual)
    } else {
        select_1323([a, b, d, e, g, f, i, h, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1316([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1317([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1321([a, b, h, d, e, g, j, i, f, k, l, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1327([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_115([a, b, d, e, f, g, i, j], is_dual)
    } else {
        select_29([b, c, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1328([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_180([a, b, f, e, d, h, i, g], is_dual)
    } else {
        select_660([a, b, c, e, d, f, g, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1326([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1327([b, c, f, d, e, g, h, i, j, k], is_dual)
    } else {
        select_1328([a, b, h, g, f, k, j, l, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1330([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_149([e, c, d, a, f, g, h], is_dual)
    } else {
        select_479([b, a, d, c, f, e, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1329([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1330([f, a, d, c, e, g, i, h, j], is_dual)
    } else {
        select_160([a, b, c, e, f, h, i, g, j], is_dual)
    }
}
/// n = 13, i = 3
fn select_1325([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1326([a, c, d, e, g, f, h, i, j, l, k, m], is_dual)
    } else {
        select_1329([a, c, k, b, f, h, m, g, j, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1333([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_258([a, c, b, d, e, f, g, i, h, j], is_dual)
    } else {
        select_101([a, b, d, g, h, f], is_dual)
    }
}
/// n = 10, i = 4
fn select_1332([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_634([c, a, b, e, g, f, i, h], is_dual)
    } else {
        select_1333([a, c, b, d, f, g, h, e, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1335([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_192([c, d, e, g, f, h], is_dual)
    } else {
        select_469([a, b, f, e, g, i, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1336([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_54([j, i, a, h, e, b], !is_dual)
    } else {
        select_127([a, e, c, d, g, f, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1334([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1335([b, a, c, d, e, f, g, h, i, k, j], is_dual)
    } else {
        select_1336([a, b, c, f, e, i, h, j, k, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_1331([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1332([a, b, c, h, k, e, f, i, j, l], is_dual)
    } else {
        select_1334([a, b, f, d, e, g, j, i, h, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1324([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1325([b, a, c, e, d, g, f, h, i, j, l, k, m], is_dual)
    } else {
        select_1331([a, b, h, e, g, k, i, l, j, f, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1315([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < n) || (is_dual && k > n) {
        select_1316([a, b, c, d, i, f, g, h, l, j, m, n, k], is_dual)
    } else {
        select_1324([a, c, b, d, e, f, h, g, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 4
fn select_1341([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_41([c, h, i, d, a, g], !is_dual)
    } else {
        select_56([c, d, b, f, e, g], is_dual)
    }
}
/// n = 12, i = 4
fn select_1340([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1133([i, a, c, d, e, h, l, j], is_dual)
    } else {
        select_1341([a, b, j, e, f, k, g, i, l], is_dual)
    }
}
/// n = 9, i = 4
fn select_1343([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_39([h, e, i, f, c, a], !is_dual)
    } else {
        select_39([g, f, i, e, d, b], !is_dual)
    }
}
/// n = 12, i = 4
fn select_1342([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_62([i, b, c, d, f, h], is_dual)
    } else {
        select_1343([a, b, e, f, g, j, i, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_1339([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1340([a, b, c, g, e, f, h, k, i, l, j, m], is_dual)
    } else {
        select_1342([b, d, c, e, f, g, h, j, i, k, l, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1347([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_24([a, c, d, e, f, h], is_dual)
    } else {
        select_24([b, c, d, e, f, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1346([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_56([a, h, d, i, f, g], is_dual)
    } else {
        select_1347([a, b, g, c, e, j, k, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1348([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_495([a, b, i, c, j, h, g, l], is_dual)
    } else {
        select_296([b, e, g, d, h, f, k, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1345([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1346([a, c, f, d, g, h, j, k, l, i, m], is_dual)
    } else {
        select_1348([b, a, d, f, e, g, h, i, k, l, j, n], is_dual)
    }
}
/// n = 9, i = 4
fn select_1350([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_626([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_39([a, h, i, f, e, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1349([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1200([b, d, g, f, e, i, h, k], is_dual)
    } else {
        select_1350([f, l, i, j, g, e, a, c, h], !is_dual)
    }
}
/// n = 15, i = 5
fn select_1344([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < f) || (is_dual && j > f) {
        select_1345([g, a, f, c, d, e, h, i, k, l, j, m, n, o], is_dual)
    } else {
        select_1349([a, j, c, b, g, m, i, f, l, k, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1338([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1339([a, b, e, d, i, g, h, j, f, l, k, m, n, o], is_dual)
    } else {
        select_1344([a, b, c, e, d, f, g, h, i, k, j, l, m, o, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1354([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_54([b, c, d, f, e, g], is_dual)
    } else {
        select_24([g, f, h, d, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1356([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_55([a, c, e, h, i, k], is_dual)
    } else {
        select_116([b, c, d, f, g, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1355([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_54([g, d, a, i, f, h], is_dual)
    } else {
        select_1356([d, b, c, a, f, e, h, g, j, k, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1353([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1354([b, g, d, a, f, i, k, h], is_dual)
    } else {
        select_1355([b, d, a, c, f, e, h, g, j, k, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1358([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_615([a, c, d, f, g, i, j], is_dual)
    } else {
        select_107([b, c, g, f, e, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1357([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1358([a, b, f, d, e, h, i, g, k, j], is_dual)
    } else {
        select_645([b, d, c, e, g, f, i, j, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1352([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_1353([d, e, c, a, g, f, i, h, k, j, l], is_dual)
    } else {
        select_1357([f, b, c, h, e, g, j, i, d, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1360([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_804([a, b, g, c, f, j, i, k, l], is_dual)
    } else {
        select_22([b, a, d, e, h, i, g, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1362([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_107([a, c, g, f, e, i, h], is_dual)
    } else {
        select_107([c, b, h, d, f, g, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1361([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_645([b, d, c, e, g, f, i, j, h], is_dual)
    } else {
        select_1362([b, a, f, d, e, h, i, j, g, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1359([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < e) || (is_dual && i > e) {
        select_1360([f, e, d, c, b, h, g, k, j, i, l, m], is_dual)
    } else {
        select_1361([g, a, d, i, f, h, l, j, e, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1351([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1352([d, a, b, e, g, h, i, j, f, l, k, m], is_dual)
    } else {
        select_1359([a, d, c, b, e, g, h, f, j, i, k, l, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1337([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1338([b, c, a, e, d, f, h, i, g, j, l, k, m, o, n], is_dual)
    } else {
        select_1351([c, g, d, e, f, a, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1314([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1315([b, c, a, d, e, f, i, g, j, h, k, l, m, n], is_dual)
    } else {
        select_1337([a, c, b, d, e, f, i, h, j, g, k, l, m, o, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1272([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1273([a, c, b, j, e, f, i, h, g, k, l, n, o, p, m], is_dual)
    } else {
        select_1314([a, b, c, k, d, f, i, g, h, j, m, n, o, p, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1369([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_131([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_495([a, b, h, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1368([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1369([b, c, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_1187([a, c, d, e, f, b, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1367([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_226([a, c, d, b, f, h, g, i, j, k], is_dual)
    } else {
        select_1368([g, b, a, e, c, f, h, j, i, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1371([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_22([a, c, d, e, b, f, g, i], is_dual)
    } else {
        select_22([a, b, d, e, c, f, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1372([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_509([a, c, b, d, e, f, g, h], is_dual)
    } else {
        select_793([a, f, d, e, b, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1370([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1371([a, b, c, e, f, g, h, k, i], is_dual)
    } else {
        select_1372([b, a, d, c, g, f, i, j, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_1366([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1367([a, c, b, d, h, f, g, i, l, j, m, k], is_dual)
    } else {
        select_1370([g, a, c, b, e, j, h, k, l, m, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1377([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_24([a, f, b, e, h, i], is_dual)
    } else {
        select_31([c, d, g], is_dual)
    }
}
/// n = 10, i = 4
fn select_1376([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1377([b, a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_127([a, f, c, d, e, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1375([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1376([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_1376([c, b, d, e, f, a, g, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1374([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1375([a, b, c, d, g, f, i, j, k, h], is_dual)
    } else {
        select_60([a, b, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1378([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_60([a, b, d, e, f, g, h, i], is_dual)
    } else {
        select_1372([a, b, c, d, g, f, i, j, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1373([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1374([a, b, c, d, e, f, g, h, j, i, k], is_dual)
    } else {
        select_1378([b, d, c, a, e, f, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1365([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < b) || (is_dual && i > b) {
        select_1366([a, b, c, d, f, e, g, h, j, i, k, l, m], is_dual)
    } else {
        select_1373([a, g, i, c, f, b, h, k, j, l, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1383([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_107([a, b, i, f, e, j, h], is_dual)
    } else {
        select_305([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1384([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_432([f, a, c, d, e, g, h, j, i], is_dual)
    } else {
        select_432([e, b, c, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1382([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1383([c, a, d, e, f, b, g, h, j, i], is_dual)
    } else {
        select_1384([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 5
fn select_1386([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_375([e, d, b, c, f, g, i], is_dual)
    } else {
        select_258([e, i, j, k, g, h, a, f, b, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1385([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1386([h, j, k, l, a, i, f, g, c, d, e], !is_dual)
    } else {
        select_684([h, j, i, k, a, b, g, d, e], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1381([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1382([a, c, d, b, e, f, g, i, h, j], is_dual)
    } else {
        select_1385([a, b, d, c, e, f, h, i, g, k, l, j], is_dual)
    }
}
/// n = 9, i = 3
fn select_1389([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_54([c, b, e, h, f, g], is_dual)
    } else {
        select_155([g, a, c, d, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1388([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < i) || (is_dual && d > i) {
        select_990([g, h, d, e, i, a, f], !is_dual)
    } else {
        select_1389([b, c, a, e, d, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1391([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_46([b, a, c, d, e, f, h, g], is_dual)
    } else {
        select_164([a, g, h, e, f, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1390([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_458([a, c, d, e, f, g, j], is_dual)
    } else {
        select_1391([a, b, c, d, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1387([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1388([b, c, a, h, e, g, j, i, f], is_dual)
    } else {
        select_1390([e, b, a, f, d, i, h, g, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1380([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1381([e, a, f, d, b, g, h, j, l, k, m, i], is_dual)
    } else {
        select_1387([b, g, c, a, e, f, h, i, l, k, j], is_dual)
    }
}
/// n = 7, i = 3
fn select_1395([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_41([g, c, f, d, e, a], !is_dual)
    } else {
        select_247([c, d, b, f, e], is_dual)
    }
}
/// n = 8, i = 3
fn select_1394([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_376([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1395([h, e, b, f, g, c, a], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1396([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_282([g, c, a, i, f, e, j], is_dual)
    } else {
        select_1205([b, c, d, a, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1393([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1394([a, b, c, e, f, g, i, h], is_dual)
    } else {
        select_1396([a, b, d, c, e, f, h, i, j, g], is_dual)
    }
}
/// n = 7, i = 3
fn select_1399([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_359([d, e, g, a, b, f, c], !is_dual)
    } else {
        select_101([a, c, b, d, e, f], is_dual)
    }
}
/// n = 9, i = 4
fn select_1398([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_37([a, b, c, d, e, g, h, i], is_dual)
    } else {
        select_1399([a, e, b, f, h, d, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1400([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_928([a, c, d, e, f, i, h, g], is_dual)
    } else {
        select_1303([a, e, b, g, d, i, j, h, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1397([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_1398([g, k, j, i, d, e, f, h, c], !is_dual)
    } else {
        select_1400([d, a, b, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1392([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_1393([f, e, a, b, g, h, i, d, k, j], is_dual)
    } else {
        select_1397([b, a, c, f, g, d, i, h, e, k, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1379([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1380([b, a, d, c, f, g, h, e, j, k, i, l, m], is_dual)
    } else {
        select_1392([e, c, d, a, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 15, i = 5
fn select_1364([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1365([g, a, f, c, e, d, h, i, j, l, m, k, n], is_dual)
    } else {
        select_1379([a, b, i, c, g, h, j, k, l, f, m, n, o], is_dual)
    }
}
/// n = 10, i = 4
fn select_1405([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_307([f, i, j, g, h, a, b, e, c], !is_dual)
    } else {
        select_40([f, g, j, a, h, b, d], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1407([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_805([a, b, c, d, e, f, g, i, h], is_dual)
    } else {
        select_180([i, f, j, g, e, h, a, b], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1409([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_24([f, b, a, e, h, i], is_dual)
    } else {
        select_47([g, c, d, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1408([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_216([e, b, c, d, g, f, h, i], is_dual)
    } else {
        select_1409([d, c, a, f, g, e, i, h, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1406([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1407([k, m, h, l, i, e, f, j, a, g], !is_dual)
    } else {
        select_1408([b, f, c, d, e, h, g, i, k, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1404([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1405([h, l, m, k, i, e, f, a, b, j], !is_dual)
    } else {
        select_1406([b, c, a, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 4
fn select_1412([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_67([b, c, d, e, g, f], is_dual)
    } else {
        select_41([e, h, i, d, a, f], !is_dual)
    }
}
/// n = 9, i = 3
fn select_1413([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_183([a, d, b, g, e, i, f, h], is_dual)
    } else {
        select_161([a, b, c, f, h, g, i, e], is_dual)
    }
}
/// n = 13, i = 5
fn select_1411([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1412([a, e, c, f, k, h, i, l, m], is_dual)
    } else {
        select_1413([b, c, d, e, i, h, g, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1415([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, c, d, h, f, g, j, k], is_dual)
    } else {
        select_338([a, b, d, g, e, f, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1416([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_149([a, b, d, h, f, g, i], is_dual)
    } else {
        select_149([b, a, c, i, e, g, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1414([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1415([a, d, f, c, h, g, e, j, i, k, l], is_dual)
    } else {
        select_1416([b, e, d, c, h, g, f, j, i], is_dual)
    }
}
/// n = 15, i = 5
fn select_1410([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1411([a, b, g, c, h, f, i, k, l, n, m, j, o], is_dual)
    } else {
        select_1414([a, i, d, e, f, j, g, h, l, m, k, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1403([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && f < k) || (is_dual && f > k) {
        select_1404([a, b, g, e, k, i, h, j, f, l, m, o, n], is_dual)
    } else {
        select_1410([a, b, c, d, e, f, g, h, i, j, l, k, m, n, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_1419([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_909([a, c, d, e, b, f, g, h], is_dual)
    } else {
        select_909([b, c, d, e, a, f, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1421([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_205([a, c, d, e, g, f, h, i], is_dual)
    } else {
        select_91([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1420([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1247([b, h, c, f, a, g, i, j], is_dual)
    } else {
        select_1421([b, c, a, d, e, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1418([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1419([b, c, d, a, h, g, j, k], is_dual)
    } else {
        select_1420([a, e, f, b, c, g, h, i, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1424([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_23([b, d, c, e, h, g, i], is_dual)
    } else {
        select_149([b, d, a, e, f, i, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_1423([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_213([b, c, d, e, i, f, h, j], is_dual)
    } else {
        select_1424([a, b, c, j, f, g, h, i, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_1422([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1419([a, b, c, g, i, j, l, n], is_dual)
    } else {
        select_1423([d, e, f, a, b, g, h, i, j, k, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1417([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < l) || (is_dual && g > l) {
        select_1418([a, b, c, d, h, i, j, l, k, g, n, m, o], is_dual)
    } else {
        select_1422([b, c, d, e, a, f, g, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1402([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_1403([b, c, d, e, f, g, h, i, j, a, k, l, m, n, o], is_dual)
    } else {
        select_1417([b, a, c, d, e, f, g, h, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 4
fn select_1429([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_155([g, b, c, e, f, h, i], is_dual)
    } else {
        select_107([c, a, h, d, f, g, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1428([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_990([g, i, d, f, h, a, e], !is_dual)
    } else {
        select_1429([b, c, a, e, f, d, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 4
fn select_1430([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < h) || (is_dual && d > h) {
        select_990([g, f, d, a, h, b, e], !is_dual)
    } else {
        select_595([a, c, b, e, d, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1427([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1428([a, d, b, f, g, e, j, k, h, i], is_dual)
    } else {
        select_1430([a, e, c, i, g, h, j, k, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1432([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_22([b, a, d, e, g, h, f, k], is_dual)
    } else {
        select_390([a, c, b, f, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1431([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1432([b, c, a, f, e, g, h, i, k, l, j], is_dual)
    } else {
        select_1432([a, c, b, g, d, f, h, j, k, l, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1426([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && e < l) || (is_dual && e > l) {
        select_1427([a, b, g, h, f, j, l, i, k, m, e], is_dual)
    } else {
        select_1431([a, f, e, c, d, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1437([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_116([c, d, i, g, h, k, l], is_dual)
    } else {
        select_116([a, b, j, e, f, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1436([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < j) || (is_dual && a > j) {
        select_263([a, d, e, h, i, l, m], is_dual)
    } else {
        select_1437([b, c, d, e, f, g, h, i, j, k, l, m, n, o], is_dual)
    }
}
/// n = 11, i = 4
fn select_1438([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_148([a, g, c, d, f, i, k, h], is_dual)
    } else {
        select_24([d, h, b, e, j, g], is_dual)
    }
}
/// n = 15, i = 5
fn select_1435([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1436([a, b, c, e, d, f, g, i, h, j, k, m, l, n, o], is_dual)
    } else {
        select_1438([b, e, c, f, i, g, k, l, n, j, o], is_dual)
    }
}
/// n = 12, i = 5
fn select_1440([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_555([a, f, c, e, i, h, j, l], is_dual)
    } else {
        select_469([a, b, d, g, h, f, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1442([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_24([a, b, d, g, h, j], is_dual)
    } else {
        select_55([c, d, e, f, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1441([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_67([d, c, g, h, e, k], is_dual)
    } else {
        select_1442([a, e, b, d, f, i, h, g, j, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1439([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1440([a, e, b, d, f, g, h, i, k, l, m, j], is_dual)
    } else {
        select_1441([a, b, c, g, e, f, h, j, k, l, m, i], is_dual)
    }
}
/// n = 16, i = 5
fn select_1434([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1435([f, b, d, c, e, g, i, h, j, l, k, m, p, n, o], is_dual)
    } else {
        select_1439([a, h, b, d, g, k, i, f, n, o, l, p, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_1445([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_444([b, a, c, e, d, f, g, h], is_dual)
    } else {
        select_574([a, g, f, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1446([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_676([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_670([a, b, f, e, c, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1444([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1445([a, c, d, g, f, e, h, j, i, l], is_dual)
    } else {
        select_1446([j, k, l, h, g, f, a, i, b, c], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1447([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_230([b, a, d, f, g, e], is_dual)
    } else {
        select_147([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 5
fn select_1443([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1444([b, c, d, e, f, g, h, j, i, m, k, l], is_dual)
    } else {
        select_1447([k, l, h, j, g, b, a, c], !is_dual)
    }
}
/// n = 16, i = 5
fn select_1433([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_1434([a, b, c, d, e, f, g, h, i, j, k, m, l, n, o, p], is_dual)
    } else {
        select_1443([a, g, i, h, e, j, l, m, k, f, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1425([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1426([b, a, d, e, f, k, i, j, g, m, n, l, p], is_dual)
    } else {
        select_1433([a, b, d, c, e, f, g, i, h, j, l, m, k, o, p, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1401([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1402([f, b, c, g, d, e, h, j, k, i, l, m, n, o, p], is_dual)
    } else {
        select_1425([a, b, c, e, d, f, h, i, k, j, g, m, n, l, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1363([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1364([a, b, c, e, j, f, g, i, k, l, n, m, h, p, o], is_dual)
    } else {
        select_1401([b, c, a, e, d, f, g, i, h, k, j, m, n, l, p, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_1271([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1272([c, d, b, f, e, g, h, a, i, k, j, m, l, n, o, p], is_dual)
    } else {
        select_1363([c, d, a, e, f, g, h, i, b, j, k, l, m, n, p, o], is_dual)
    }
}
/// n = 8, i = 2
fn select_1453([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_741([a, b, c, e, f, h, g], is_dual)
    } else {
        select_741([a, b, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1455([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_604([a, c, d, e, f, b, g, h], is_dual)
    } else {
        select_604([b, c, d, e, f, a, g, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1454([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_693([a, d, f, b, c, e, g, h, j], is_dual)
    } else {
        select_1455([b, c, a, d, e, g, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1452([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1453([c, d, a, e, g, j, l, i], is_dual)
    } else {
        select_1454([a, c, d, b, i, f, h, k, j, m, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1457([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_541([c, a, d, e, f, g, b, h, i, j, k], is_dual)
    } else {
        select_541([c, b, d, e, f, g, a, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1459([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_557([b, c, h, f, g, a, i, j], is_dual)
    } else {
        select_171([b, c, a, d, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1460([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_604([f, e, c, d, h, g, i, j], is_dual)
    } else {
        select_604([f, e, a, b, i, g, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1458([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1459([b, c, h, e, f, a, g, i, j, k], is_dual)
    } else {
        select_1460([a, d, e, f, b, c, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1456([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1457([b, c, a, e, f, d, h, i, k, j, l], is_dual)
    } else {
        select_1458([a, d, e, g, b, c, i, h, j, k, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1451([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1452([a, b, c, d, e, g, i, j, h, k, l, m, n], is_dual)
    } else {
        select_1456([a, c, d, b, f, h, g, j, i, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1450([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1451([a, b, c, d, g, e, f, j, h, i, l, m, k, n], is_dual)
    } else {
        select_1451([b, a, c, d, g, f, e, j, i, h, k, m, l, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1466([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_375([a, i, d, e, f, h, k], is_dual)
    } else {
        select_228([b, d, e, c, f, g, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1467([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_183([a, c, d, e, g, i, h, k], is_dual)
    } else {
        select_469([a, b, c, e, f, h, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1465([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1466([b, c, f, d, e, g, h, i, j, l, k], is_dual)
    } else {
        select_1467([a, b, i, c, f, g, h, k, l, j, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1468([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_205([b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_555([a, h, b, f, e, g, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1464([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1465([a, b, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1468([b, c, d, e, g, j, h, i, l, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1471([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_495([a, b, i, c, f, g, k, l], is_dual)
    } else {
        select_591([b, c, f, d, e, g, h, j, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1470([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < j) || (is_dual && l > j) {
        select_1471([b, a, d, e, f, g, h, i, j, k, m, l], is_dual)
    } else {
        select_591([b, c, h, e, f, g, i, k, l, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1473([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_241([a, f, b, c, i, j, g], is_dual)
    } else {
        select_797([a, b, d, e, g, f, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1474([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_62([f, a, d, e, h, g], is_dual)
    } else {
        select_62([a, g, b, c, i, f], is_dual)
    }
}
/// n = 12, i = 3
fn select_1472([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1473([a, c, h, e, f, i, g, j, k, l], is_dual)
    } else {
        select_1474([b, d, g, e, f, j, i, h, k], is_dual)
    }
}
/// n = 14, i = 4
fn select_1469([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1470([a, b, d, h, e, f, i, g, j, k, l, m, n], is_dual)
    } else {
        select_1472([b, c, d, g, e, f, i, h, j, l, k, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1463([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1464([c, d, g, e, f, h, i, a, j, k, l, m, n], is_dual)
    } else {
        select_1469([c, b, d, a, e, f, h, i, g, j, l, k, m, n], is_dual)
    }
}
/// n = 12, i = 4
fn select_1477([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1030([a, b, d, c, f, g, h, j, k, i, l], is_dual)
    } else {
        select_992([a, c, e, b, f, i, j, h, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1479([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_114([b, c, e, h, f, g, i], is_dual)
    } else {
        select_280([a, e, d, g, f, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1478([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_237([a, c, d, b, e, f, h, g, i, k], is_dual)
    } else {
        select_1479([b, c, d, e, a, f, h, g, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1476([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1477([a, c, d, b, e, g, h, i, f, j, k, l], is_dual)
    } else {
        select_1478([a, d, b, f, e, g, i, h, k, j, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1482([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_241([a, g, b, c, f, i, h], is_dual)
    } else {
        select_131([a, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 2
fn select_1481([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_509([f, h, b, d, e, g, i, j], is_dual)
    } else {
        select_1482([g, a, c, d, e, f, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1484([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_131([a, c, d, e, g, h, i, j], is_dual)
    } else {
        select_750([a, b, c, i, f, h, g, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1485([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_221([a, c, j, e, f, i, h, l], is_dual)
    } else {
        select_228([b, e, f, d, h, g, i, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1483([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1484([a, b, c, e, f, i, g, h, j, l, k], is_dual)
    } else {
        select_1485([a, b, c, d, e, f, g, h, j, i, k, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1480([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1481([c, h, d, e, f, g, a, i, j, k], is_dual)
    } else {
        select_1483([b, c, a, d, e, f, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1475([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1476([b, d, c, a, j, g, h, i, l, m, k, n], is_dual)
    } else {
        select_1480([a, d, c, h, e, f, g, i, k, j, m, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1462([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1463([c, b, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1475([c, d, a, e, f, g, b, h, i, j, k, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1489([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1047([a, d, e, b, f, c, g, h, i], is_dual)
    } else {
        select_1047([a, d, e, c, f, b, g, h, i], is_dual)
    }
}
/// n = 8, i = 2
fn select_1491([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_67([a, c, d, e, h, g], is_dual)
    } else {
        select_149([a, c, b, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1490([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_920([b, c, d, e, a, f, g, h], is_dual)
    } else {
        select_1491([c, a, d, e, f, b, h, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1488([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1489([a, b, c, e, i, g, k, h, j], is_dual)
    } else {
        select_1490([a, d, e, f, h, g, i, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1493([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_123([a, c, d, e, f, h, g, i], is_dual)
    } else {
        select_509([a, b, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1494([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_509([a, b, d, e, f, h, g, i], is_dual)
    } else {
        select_357([a, b, c, d, i, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1492([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1493([a, b, e, h, f, g, j, i, l], is_dual)
    } else {
        select_1494([a, c, d, i, f, g, j, h, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1487([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1488([a, c, d, e, h, f, b, i, j, l, k], is_dual)
    } else {
        select_1492([a, e, c, d, f, b, g, i, j, h, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1498([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_322([b, c, e, d, f, g, h, i], is_dual)
    } else {
        select_495([a, c, f, d, e, g, h, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1497([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1498([a, c, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_1214([b, c, d, e, a, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1496([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_1258([b, a, d, e, f, i, h, k], is_dual)
    } else {
        select_1497([a, b, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1500([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_208([b, a, c, d, e, f, g, h, j, i], is_dual)
    } else {
        select_604([b, c, d, e, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1502([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_130([b, e, c, d, f, g, h], is_dual)
    } else {
        select_375([a, f, c, d, e, g, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1501([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_384([b, c, d, e, g, i, h], is_dual)
    } else {
        select_1502([a, b, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1499([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1500([c, b, d, e, f, a, g, h, i, j], is_dual)
    } else {
        select_1501([c, a, d, e, f, g, b, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1495([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1496([a, b, c, e, f, j, h, i, k, m, l], is_dual)
    } else {
        select_1499([a, b, c, d, g, i, h, j, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1486([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1487([b, c, e, f, a, h, g, i, j, k, l, m], is_dual)
    } else {
        select_1495([a, b, d, c, e, f, i, h, j, l, k, m, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1461([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1462([a, b, c, d, e, f, g, h, i, j, k, m, n, l], is_dual)
    } else {
        select_1486([a, c, b, d, f, g, e, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1449([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_1450([b, c, d, e, f, g, a, i, j, h, l, m, k, n], is_dual)
    } else {
        select_1461([b, c, a, f, g, d, e, i, j, k, l, m, h, n], is_dual)
    }
}
/// n = 12, i = 4
fn select_1508([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1330([a, b, h, c, f, g, j, k, l], is_dual)
    } else {
        select_1043([a, c, f, d, e, g, h, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1507([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1508([a, c, d, b, e, g, h, i, k, l, j, m], is_dual)
    } else {
        select_1126([a, b, c, f, e, g, j, k, m, i], is_dual)
    }
}
/// n = 14, i = 4
fn select_1506([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1507([a, b, c, f, e, h, g, i, j, k, m, l, n], is_dual)
    } else {
        select_888([a, c, d, j, f, g, i, k, h, l, n, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1512([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_216([h, a, e, b, g, f, j, i], is_dual)
    } else {
        select_91([b, e, c, d, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1511([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && l < f) || (is_dual && l > f) {
        select_1512([b, c, d, e, f, g, h, i, j, k], is_dual)
    } else {
        select_28([a, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1514([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_826([b, c, d, e, g, h, i, k, j, l], is_dual)
    } else {
        select_660([a, b, c, f, g, h, j, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1513([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_286([b, f, g, e, h, k, l, i], is_dual)
    } else {
        select_1514([a, f, b, c, d, g, h, i, j, l, m, k, n], is_dual)
    }
}
/// n = 14, i = 4
fn select_1510([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1511([b, h, d, e, f, g, a, i, j, l, k, m, n], is_dual)
    } else {
        select_1513([b, c, e, f, a, d, g, i, h, j, k, m, l, n], is_dual)
    }
}
/// n = 10, i = 4
fn select_1517([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_54([b, c, e, g, f, h], is_dual)
    } else {
        select_555([h, g, j, i, d, e, a, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1519([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_49([b, c, d, f, h, g, i], is_dual)
    } else {
        select_243([a, b, e, f, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1518([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_242([a, b, d, e, h, i, g, k], is_dual)
    } else {
        select_1519([a, c, b, d, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1516([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1517([b, f, d, e, a, g, i, j, h, k], is_dual)
    } else {
        select_1518([b, c, d, a, e, g, f, h, j, i, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1520([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_308([a, b, c, e, f, h, i, j], is_dual)
    } else {
        select_546([a, h, c, d, f, g, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1515([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1516([a, b, c, d, f, g, h, k, j, l, m], is_dual)
    } else {
        select_1520([a, c, d, e, g, h, i, j, l, k], is_dual)
    }
}
/// n = 15, i = 4
fn select_1509([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1510([b, c, d, f, a, i, g, h, j, k, m, n, l, o], is_dual)
    } else {
        select_1515([a, d, c, b, e, h, g, l, i, n, k, o, m], is_dual)
    }
}
/// n = 15, i = 4
fn select_1505([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1506([a, b, e, c, j, d, i, g, h, l, m, o, n, k], is_dual)
    } else {
        select_1509([b, a, e, d, f, c, i, h, j, g, l, k, o, n, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1525([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_148([b, a, f, e, g, h, d, i], is_dual)
    } else {
        select_263([d, b, c, e, f, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1524([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1525([f, c, d, a, e, g, h, i, k], is_dual)
    } else {
        select_548([b, d, c, a, e, g, f, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1528([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_116([b, d, c, f, e, h, g], is_dual)
    } else {
        select_47([a, i, j, b], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1527([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1528([a, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_247([a, b, c, j, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1530([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_24([f, b, c, e, h, g], is_dual)
    } else {
        select_47([i, d, j, a], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1531([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_24([h, i, e, c, f, a], !is_dual)
    } else {
        select_28([g, b, d], is_dual)
    }
}
/// n = 10, i = 4
fn select_1529([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1530([a, b, c, d, e, f, h, g, i, j], is_dual)
    } else {
        select_1531([a, c, d, e, g, h, f, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1526([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1527([a, b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_1529([b, d, e, g, f, a, i, h, j, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_1523([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1524([a, c, e, f, i, g, h, j, l, m, k], is_dual)
    } else {
        select_1526([a, b, d, g, f, h, i, k, l, n, j], is_dual)
    }
}
/// n = 15, i = 5
fn select_1522([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1523([a, b, c, e, h, f, g, i, j, k, m, l, n, o], is_dual)
    } else {
        select_888([a, c, d, j, f, g, i, k, h, l, n, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_1535([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_183([a, b, d, e, g, h, i, k], is_dual)
    } else {
        select_23([b, c, d, h, f, g, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1534([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1535([a, i, c, e, g, f, h, j, m, k, n], is_dual)
    } else {
        select_78([f, b, d, k, l, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1537([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_884([a, c, d, e, g, h, i, k, j, l, n], is_dual)
    } else {
        select_660([b, d, e, f, g, h, j, l, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1536([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_286([c, f, g, e, h, k, l, j], is_dual)
    } else {
        select_1537([b, a, d, f, c, g, h, j, i, l, m, k, o, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1533([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1534([b, c, h, e, f, a, g, i, j, k, l, n, m, o], is_dual)
    } else {
        select_1536([b, c, d, e, a, f, g, i, j, h, k, m, l, n, o], is_dual)
    }
}
/// n = 11, i = 4
fn select_1541([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_24([a, b, h, i, f, k], is_dual)
    } else {
        select_116([c, f, d, e, g, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1540([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_554([a, b, d, h, g, f, i, j, l], is_dual)
    } else {
        select_1541([a, b, c, f, e, g, i, h, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1542([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_114([b, c, e, i, f, h, k], is_dual)
    } else {
        select_479([a, d, e, h, f, g, j, i, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1539([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1540([b, d, c, e, f, g, h, a, i, j, k, l], is_dual)
    } else {
        select_1542([b, a, c, d, e, g, h, f, i, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1544([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_114([b, c, e, i, f, g, l], is_dual)
    } else {
        select_750([a, e, d, g, f, h, j, i, k], is_dual)
    }
}
/// n = 10, i = 4
fn select_1546([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_82([a, g, c, i], is_dual)
    } else {
        select_769([a, b, f, d, h, e, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1545([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1546([a, b, c, e, h, g, j, i, l, k], is_dual)
    } else {
        select_263([i, b, d, e, f, h, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1543([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1544([c, a, b, d, e, g, f, h, i, j, k, l], is_dual)
    } else {
        select_1545([c, b, d, e, f, g, h, a, i, j, l, k], is_dual)
    }
}
/// n = 14, i = 5
fn select_1538([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1539([a, b, d, g, e, h, i, f, j, k, m, n], is_dual)
    } else {
        select_1543([a, d, c, f, e, h, i, g, j, k, l, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_1532([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1533([a, c, b, d, i, f, g, h, j, k, m, n, l, p, o], is_dual)
    } else {
        select_1538([b, d, c, a, e, h, g, l, i, n, k, o, p, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_1521([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1522([a, b, e, c, j, d, i, g, h, l, m, n, o, k, p], is_dual)
    } else {
        select_1532([a, b, e, d, f, c, i, h, j, g, l, k, n, o, m, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1504([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1505([a, c, d, e, f, b, i, j, k, g, l, h, m, o, n], is_dual)
    } else {
        select_1521([a, b, d, f, e, c, i, k, j, h, l, g, n, m, o, p], is_dual)
    }
}
/// n = 12, i = 4
fn select_1552([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_444([b, c, d, f, g, h, i, j], is_dual)
    } else {
        select_338([a, b, c, e, f, g, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1551([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1552([a, c, b, e, f, g, i, h, k, l, j, m], is_dual)
    } else {
        select_992([a, b, d, h, f, j, k, i, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1554([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_114([b, c, d, e, f, h, g], is_dual)
    } else {
        select_41([a, b, c, g, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1555([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_221([a, c, f, d, e, h, g, j], is_dual)
    } else {
        select_130([b, g, d, e, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1553([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1554([d, b, c, a, h, f, i, j, g], is_dual)
    } else {
        select_1555([d, a, e, b, c, f, h, g, j, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1550([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1551([a, c, d, e, b, g, h, f, j, i, k, l, m], is_dual)
    } else {
        select_1553([a, b, i, c, e, g, f, j, k, m], is_dual)
    }
}
/// n = 10, i = 3
fn select_1558([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_242([a, h, b, e, f, i, g, j], is_dual)
    } else {
        select_296([a, c, d, f, e, g, i, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1557([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1558([a, c, b, g, e, f, h, i, k, l], is_dual)
    } else {
        select_474([b, f, e, d, h, g, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1559([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_174([a, c, b, d, h, f, g, i, j, l, m], is_dual)
    } else {
        select_893([b, i, e, d, f, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1556([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1557([a, b, d, c, g, h, j, i, f, k, l, m], is_dual)
    } else {
        select_1559([a, c, d, e, b, g, h, f, i, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1549([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1550([c, b, d, e, f, a, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1556([c, a, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1563([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_205([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_49([a, b, h, e, g, f, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1562([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_61([d, e, g, a, i, h, j], is_dual)
    } else {
        select_1563([b, c, a, h, f, g, i, k, l], is_dual)
    }
}
/// n = 10, i = 2
fn select_1565([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_118([b, h, c, d, e, g, i], is_dual)
    } else {
        select_91([a, g, c, d, e, f, h, i, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1564([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1565([b, c, a, e, h, f, g, i, j, l], is_dual)
    } else {
        select_893([a, i, d, e, g, h, k], is_dual)
    }
}
/// n = 13, i = 3
fn select_1561([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1562([a, c, f, d, b, g, h, j, k, l, i, m], is_dual)
    } else {
        select_1564([b, c, f, a, e, g, h, i, k, j, l, m], is_dual)
    }
}
/// n = 9, i = 2
fn select_1568([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_115([d, e, b, c, f, g, i, h], is_dual)
    } else {
        select_263([h, d, a, f, e, i, g], is_dual)
    }
}
/// n = 11, i = 3
fn select_1567([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1568([b, a, f, d, e, g, h, i, k], is_dual)
    } else {
        select_474([a, e, g, c, h, f, j, k], is_dual)
    }
}
/// n = 12, i = 3
fn select_1566([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < j) || (is_dual && f > j) {
        select_1567([a, c, b, e, g, j, h, i, f, k, l], is_dual)
    } else {
        select_1564([b, c, e, a, d, g, h, f, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1560([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1561([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_1566([g, b, c, e, f, a, h, i, k, j, l, m], is_dual)
    }
}
/// n = 15, i = 4
fn select_1548([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1549([a, b, c, h, e, j, g, i, l, m, n, k, o], is_dual)
    } else {
        select_1560([b, a, e, d, g, f, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 10, i = 2
fn select_1572([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_912([a, b, c, i, f, g, h, j], is_dual)
    } else {
        select_1161([a, c, d, e, h, g, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1571([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_52([a, b, c, d, g, f, i, j, h, k], is_dual)
    } else {
        select_1572([b, c, e, a, d, f, g, i, h, j], is_dual)
    }
}
/// n = 9, i = 2
fn select_1575([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_196([a, b, f, h, i], is_dual)
    } else {
        select_29([a, c, d, e, g, f, h], is_dual)
    }
}
/// n = 11, i = 2
fn select_1574([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1575([a, c, b, d, i, g, h, k, j], is_dual)
    } else {
        select_131([a, c, e, f, j, h, i, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1576([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_474([a, b, c, g, f, j, i, h], is_dual)
    } else {
        select_765([b, e, a, d, f, h, g, i, j], is_dual)
    }
}
/// n = 11, i = 2
fn select_1573([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1574([b, c, d, f, a, e, g, i, h, j, k], is_dual)
    } else {
        select_1576([b, h, d, c, f, a, g, i, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1570([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1571([a, b, c, d, f, j, h, i, g, k, l], is_dual)
    } else {
        select_1573([b, a, c, d, f, e, g, h, i, j, k], is_dual)
    }
}
/// n = 10, i = 2
fn select_1578([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_368([a, b, c, e, f, h, g, i, j], is_dual)
    } else {
        select_229([a, b, c, d, i, g, h, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1579([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1459([a, d, g, e, f, b, h, i, j, l], is_dual)
    } else {
        select_541([a, d, c, b, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1577([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1578([b, d, e, f, a, g, h, i, j, k], is_dual)
    } else {
        select_1579([a, b, c, d, e, i, g, h, k, j, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1569([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1570([a, b, e, h, f, d, g, i, k, j, m, l], is_dual)
    } else {
        select_1577([a, g, c, d, b, f, h, i, j, k, l, n, m], is_dual)
    }
}
/// n = 15, i = 4
fn select_1547([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_1548([a, b, c, e, f, d, h, g, j, i, k, l, m, o, n], is_dual)
    } else {
        select_1569([a, b, c, d, e, j, g, h, i, l, k, n, o, m], is_dual)
    }
}
/// n = 16, i = 5
fn select_1503([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < h) || (is_dual && m > h) {
        select_1504([a, c, b, d, f, e, g, h, i, k, j, l, n, m, o, p], is_dual)
    } else {
        select_1547([a, b, d, f, c, e, g, i, k, j, l, m, n, h, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_1448([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1449([b, a, h, d, g, e, f, i, k, l, j, m, n, o], is_dual)
    } else {
        select_1503([a, b, c, e, d, f, h, i, k, j, l, m, g, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1270([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < o) || (is_dual && g > o) {
        select_1271([b, a, c, d, e, f, h, i, j, k, l, m, n, o, g, p], is_dual)
    } else {
        select_1448([b, c, d, a, e, f, h, i, j, g, k, l, m, n, o, p], is_dual)
    }
}
/// n = 7, i = 1
fn select_1587([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_76([a, b, d, g, f, c], is_dual)
    } else {
        select_893([a, b, d, c, e, f, g], is_dual)
    }
}
/// n = 9, i = 3
fn select_1589([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_58([f, c, d, e, a, g, i], is_dual)
    } else {
        select_521([a, b, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1588([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_613([i, j, g, b, a, c], !is_dual)
    } else {
        select_1589([b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1586([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1587([a, d, e, f, h, g, i], is_dual)
    } else {
        select_1588([b, a, c, d, e, g, h, j, k, l], is_dual)
    }
}
/// n = 8, i = 2
fn select_1591([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_246([a, f, b, g, h, e], is_dual)
    } else {
        select_22([a, c, b, d, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1590([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1591([a, f, d, e, b, h, g, i], is_dual)
    } else {
        select_845([a, b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1585([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_1586([a, b, d, c, g, f, h, i, j, l, m, n], is_dual)
    } else {
        select_1590([a, c, e, i, f, g, h, j, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1594([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_45([a, b, c, e, f, d, g, h, i], is_dual)
    } else {
        select_276([a, g, h, i, d, e, f, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1593([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_1594([a, b, d, e, c, f, g, h, i], is_dual)
    } else {
        select_36([a, c, d, b, f, e, g, h, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1597([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_183([a, c, b, h, f, i, j, k], is_dual)
    } else {
        select_38([a, g, d, e, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1596([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_523([g, d, e, a, f, i, h, j, k], is_dual)
    } else {
        select_1597([a, e, d, b, c, f, h, g, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1598([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_594([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_107([b, f, g, h, e, a, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1595([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1596([a, b, c, d, e, g, f, i, h, j, k], is_dual)
    } else {
        select_1598([a, f, d, e, b, g, h, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1592([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1593([g, m, n, l, a, i, b, d, j], !is_dual)
    } else {
        select_1595([a, c, e, i, f, g, h, k, l, j, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1584([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_1585([b, c, d, f, a, g, e, h, i, j, l, k, m, n], is_dual)
    } else {
        select_1592([b, c, d, f, e, g, a, h, i, k, j, l, m, n], is_dual)
    }
}
/// n = 13, i = 5
fn select_1600([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1131([i, k, l, m, j, a, b, h, d, f], !is_dual)
    } else {
        select_1131([j, l, k, m, i, b, a, g, c, e], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1604([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < g) || (is_dual && d > g) {
        select_42([h, i, d, a, g, b], !is_dual)
    } else {
        select_98([f, i, g, a, d, e, c], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1603([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1604([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1604([a, b, c, d, e, f, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1605([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_469([a, c, d, b, e, f, g, h], is_dual)
    } else {
        select_469([b, c, d, a, e, f, g, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1602([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && i < a) || (is_dual && i > a) {
        select_1603([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1605([g, h, f, a, d, i, e, b], !is_dual)
    }
}
/// n = 7, i = 3
fn select_1607([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_230([a, b, c, d, e, f], is_dual)
    } else {
        select_867([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 4
fn select_1606([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1594([a, b, c, d, e, f, g, h, i], is_dual)
    } else {
        select_1607([a, b, f, d, g, h, e], is_dual)
    }
}
/// n = 9, i = 4
fn select_1601([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < c) || (is_dual && b > c) {
        select_1602([a, c, d, b, f, e, g, h, i], is_dual)
    } else {
        select_1606([a, b, d, e, c, f, g, h, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1599([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_1600([a, b, c, e, d, f, g, h, j, k, l, m, i], is_dual)
    } else {
        select_1601([a, k, m, l, b, g, c, i, d], !is_dual)
    }
}
/// n = 14, i = 5
fn select_1583([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < a) || (is_dual && j > a) {
        select_1584([b, c, d, a, e, f, g, h, i, k, j, m, l, n], is_dual)
    } else {
        select_1599([b, c, d, f, e, h, i, j, a, k, l, m, n], is_dual)
    }
}
/// n = 12, i = 4
fn select_1613([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_205([b, c, d, e, g, i, h, j], is_dual)
    } else {
        select_189([a, b, f, g, j, k, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1612([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1613([a, b, c, d, e, f, g, i, h, j, k, l], is_dual)
    } else {
        select_205([b, c, d, e, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1615([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_118([b, c, d, e, f, g, h], is_dual)
    } else {
        select_999([a, h, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1614([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1615([a, b, c, d, e, f, g, h, i, j], is_dual)
    } else {
        select_223([a, g, d, e, f, b, h, i, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1611([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1612([b, c, a, d, h, f, g, i, k, l, n, j], is_dual)
    } else {
        select_1614([f, a, e, d, g, i, h, j, m, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1618([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_183([a, b, c, d, e, f, h, i], is_dual)
    } else {
        select_575([g, i, d, h, a, b, e], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1619([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_77([c, d, e, f, h], is_dual)
    } else {
        select_71([i, j, a, b, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1617([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_1618([h, i, j, f, a, b, c, g, d], !is_dual)
    } else {
        select_1619([b, c, a, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1620([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_397([b, c, g, d, f, a, h, i, j], is_dual)
    } else {
        select_1187([b, c, d, a, e, f, h, g, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1616([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < j) || (is_dual && h > j) {
        select_1617([a, d, c, b, g, i, j, k, l, h], is_dual)
    } else {
        select_1620([b, a, d, e, f, h, g, i, k, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1610([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1611([a, b, c, d, e, g, h, i, f, j, k, l, m, n], is_dual)
    } else {
        select_1616([f, a, b, h, d, e, i, g, j, k, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1624([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < d) || (is_dual && h > d) {
        select_149([a, b, c, d, e, f, g], is_dual)
    } else {
        select_255([a, e, f, g, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_1623([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1624([a, j, c, f, g, l, i, m], is_dual)
    } else {
        select_576([a, b, e, i, d, f, h, k, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1625([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_817([a, j, c, f, g, l, i, m], is_dual)
    } else {
        select_576([a, b, e, i, d, f, h, k, j, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1622([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1623([a, b, c, d, e, f, h, i, g, j, k, l, m], is_dual)
    } else {
        select_1625([a, b, c, d, e, f, g, i, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1626([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_149([c, h, d, e, g, j, l], is_dual)
    } else {
        select_56([a, i, b, k, f, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1621([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1622([b, a, c, d, e, g, f, h, i, j, k, l, m], is_dual)
    } else {
        select_1626([a, d, b, c, g, i, h, j, k, l, f, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1609([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1610([c, b, d, e, f, g, a, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1621([c, a, d, f, e, g, b, h, i, k, j, l, n, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1631([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_224([d, f, h, e, g, a, c], !is_dual)
    } else {
        select_98([d, c, b, f, e, g, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1630([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_329([d, c, b, e, g, h, f, i], is_dual)
    } else {
        select_1631([c, a, f, g, d, i, e, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1633([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_359([a, b, f, d, h, e, g], is_dual)
    } else {
        select_359([a, c, e, d, g, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1632([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_403([e, a, b, d, h, i, f, g], is_dual)
    } else {
        select_1633([d, b, c, f, g, e, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1629([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1630([a, d, c, e, b, f, h, g, i], is_dual)
    } else {
        select_1632([b, c, d, a, f, e, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1635([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_546([a, c, b, d, e, f, g], is_dual)
    } else {
        select_392([a, e, h, b, c, g, f], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1636([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_594([a, d, c, b, i, h, g, f, j], is_dual)
    } else {
        select_474([b, d, e, a, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1634([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1635([h, a, b, d, f, i, j, g], is_dual)
    } else {
        select_1636([a, b, c, e, d, f, h, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1628([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1629([f, b, c, e, a, i, g, h, j], is_dual)
    } else {
        select_1634([b, a, c, d, e, g, f, h, i, j], is_dual)
    }
}
/// n = 8, i = 3
fn select_1640([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_98([a, e, b, g, h, d, f], is_dual)
    } else {
        select_98([a, d, c, f, h, e, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1639([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_1640([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_312([a, f, g, h, d, e], !is_dual)
    }
}
/// n = 8, i = 3
fn select_1642([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_491([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_491([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1641([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_594([a, c, d, b, f, e, g, h, i], is_dual)
    } else {
        select_1642([b, c, a, f, g, h, e, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1638([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < i) || (is_dual && e > i) {
        select_1639([g, e, j, i, h, f, b, a], !is_dual)
    } else {
        select_1641([b, a, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1637([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1638([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_1629([a, c, e, d, b, f, h, g, i], is_dual)
    }
}
/// n = 11, i = 4
fn select_1627([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1628([b, c, e, d, a, f, i, h, g, j], is_dual)
    } else {
        select_1637([f, b, c, g, e, a, j, i, h, k], is_dual)
    }
}
/// n = 15, i = 5
fn select_1608([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < h) || (is_dual && k > h) {
        select_1609([a, b, c, e, h, f, g, i, j, m, k, l, o, n], is_dual)
    } else {
        select_1627([a, b, d, e, k, g, i, l, h, n, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1582([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < k) || (is_dual && g > k) {
        select_1583([h, a, b, c, d, i, f, j, k, l, g, m, n, o], is_dual)
    } else {
        select_1608([b, c, d, a, e, f, h, g, i, j, l, m, k, o, n], is_dual)
    }
}
/// n = 8, i = 2
fn select_1649([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_241([a, f, b, d, e, h, g], is_dual)
    } else {
        select_375([a, d, c, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 5
fn select_1648([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1649([b, c, d, e, g, f, h, i], is_dual)
    } else {
        select_523([a, h, d, g, f, e, j, k, l], is_dual)
    }
}
/// n = 12, i = 5
fn select_1650([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_621([b, d, c, e, f, i, h, g], is_dual)
    } else {
        select_1446([h, i, j, l, e, k, a, f, g, d], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1647([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1648([a, c, d, e, b, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1650([b, c, d, e, a, f, h, g, i, j, k, l], is_dual)
    }
}
/// n = 9, i = 2
fn select_1653([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_375([a, g, b, c, i, f, h], is_dual)
    } else {
        select_33([a, d, e, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1652([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1653([a, d, e, b, c, h, g, i, f], is_dual)
    } else {
        select_1653([b, d, e, a, c, h, f, i, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_1654([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_409([a, c, d, g, e, f, h], is_dual)
    } else {
        select_781([a, b, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 2
fn select_1651([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1652([a, b, d, c, e, g, h, f, i], is_dual)
    } else {
        select_1654([a, b, d, f, c, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1646([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1647([b, a, d, e, c, g, h, i, f, j, k, l], is_dual)
    } else {
        select_1651([b, d, c, e, a, f, h, i, g], is_dual)
    }
}
/// n = 5, i = 0
fn select_1659([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_34([a, b, c, d], is_dual)
    } else {
        select_34([a, b, c, e], is_dual)
    }
}
/// n = 9, i = 3
fn select_1658([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1659([b, c, d, e, f], is_dual)
    } else {
        select_47([a, h, i, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1657([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_458([f, a, h, j, b, i, g], !is_dual)
    } else {
        select_1658([b, a, c, d, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1660([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_191([a, j, f, l, b, k, i], !is_dual)
    } else {
        select_524([a, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1656([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1657([a, b, d, e, j, g, i, l, m, n], is_dual)
    } else {
        select_1660([a, b, c, i, f, g, h, k, j, l, m, n], is_dual)
    }
}
/// n = 11, i = 5
fn select_1663([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_23([b, d, c, g, f, e, j], is_dual)
    } else {
        select_158([g, j, k, d, i, h, a, e], !is_dual)
    }
}
/// n = 11, i = 5
fn select_1662([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < b) || (is_dual && d > b) {
        select_1663([a, b, c, e, f, g, d, h, i, j, k], is_dual)
    } else {
        select_156([e, c, d, g, f, b, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1666([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_39([d, b, c, e, f, g], is_dual)
    } else {
        select_63([d, h, i, a, g], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1667([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_49([b, c, d, e, g, f, h], is_dual)
    } else {
        select_47([a, i, j, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1665([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_1666([e, c, d, a, g, f, h, i, j], is_dual)
    } else {
        select_1667([a, b, c, d, e, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 5
fn select_1664([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1665([a, c, d, e, f, g, i, h, j, k], is_dual)
    } else {
        select_191([a, j, f, l, b, k, c], !is_dual)
    }
}
/// n = 12, i = 5
fn select_1661([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_1662([i, j, l, a, g, k, b, h, c, d, f], !is_dual)
    } else {
        select_1664([a, b, d, c, e, g, f, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 5
fn select_1655([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1656([b, a, c, d, e, f, g, h, i, j, k, l, m, n], is_dual)
    } else {
        select_1661([a, b, c, i, f, h, k, j, g, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1645([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < l) || (is_dual && j > l) {
        select_1646([a, b, c, d, i, g, h, k, l, n, j, m], is_dual)
    } else {
        select_1655([a, b, d, e, f, c, g, h, i, j, k, m, l, n], is_dual)
    }
}
/// n = 11, i = 3
fn select_1672([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_33([f, d, e, i, h], is_dual)
    } else {
        select_128([a, h, b, c, f, g, j, k, i], is_dual)
    }
}
/// n = 13, i = 5
fn select_1671([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_473([a, b, h, c, f, j, g, k, l, m], is_dual)
    } else {
        select_1672([a, b, c, d, e, f, g, h, i, k, l], is_dual)
    }
}
/// n = 10, i = 4
fn select_1674([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_161([h, i, j, e, a, f, b, g], !is_dual)
    } else {
        select_296([b, c, d, a, f, e, h, g, i], is_dual)
    }
}
/// n = 12, i = 5
fn select_1673([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_474([c, a, f, d, e, g, h, j], is_dual)
    } else {
        select_1674([a, b, c, g, e, f, i, j, k, l], is_dual)
    }
}
/// n = 13, i = 5
fn select_1670([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1671([a, c, e, b, d, f, i, h, j, g, k, m, l], is_dual)
    } else {
        select_1673([a, c, b, d, f, i, g, j, h, l, k, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_1677([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_127([e, a, b, d, g, h, j, f], is_dual)
    } else {
        select_491([b, c, d, e, f, g, i, h], is_dual)
    }
}
/// n = 11, i = 5
fn select_1676([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_585([h, g, i, j, a, b, f, d], !is_dual)
    } else {
        select_1677([h, j, k, i, g, e, a, f, c, d], !is_dual)
    }
}
/// n = 12, i = 4
fn select_1679([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_70([a, f, c, i, g, l], is_dual)
    } else {
        select_128([b, g, d, e, f, h, j, k, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1678([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1679([b, c, e, a, d, f, h, g, j, k, l, i], is_dual)
    } else {
        select_1518([b, a, c, d, h, f, g, i, k, l, j], is_dual)
    }
}
/// n = 13, i = 5
fn select_1675([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1676([a, g, b, c, h, i, f, j, m, l, k], is_dual)
    } else {
        select_1678([b, c, a, d, e, f, h, i, g, j, k, m], is_dual)
    }
}
/// n = 13, i = 5
fn select_1669([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1670([b, c, d, e, f, a, g, h, i, k, j, l, m], is_dual)
    } else {
        select_1675([a, c, d, e, f, b, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 3
fn select_1682([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_548([a, b, c, e, f, g, i, h, k, j], is_dual)
    } else {
        select_853([f, a, b, d, g, i, j, k, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1684([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_148([a, f, b, d, e, g, h, i], is_dual)
    } else {
        select_63([b, c, g, e, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1683([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_697([a, b, c, f, e, h, i, g, j], is_dual)
    } else {
        select_1684([b, a, d, e, g, f, i, h, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1681([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1682([b, d, c, e, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1683([a, c, i, d, g, h, f, j, l, m, k], is_dual)
    }
}
/// n = 10, i = 3
fn select_1687([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_42([b, h, c, a, f, j], is_dual)
    } else {
        select_591([b, c, a, d, e, f, g, i, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1686([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1653([a, d, e, c, f, h, g, i, j], is_dual)
    } else {
        select_1687([a, b, g, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1689([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_241([a, c, b, e, g, i, h], is_dual)
    } else {
        select_216([b, c, e, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1688([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_215([a, h, c, d, g, i, f, k, l], is_dual)
    } else {
        select_1689([b, e, d, c, f, g, i, h, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1685([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < k) || (is_dual && j > k) {
        select_1686([g, a, e, b, c, f, h, i, k, l, j], is_dual)
    } else {
        select_1688([a, b, d, h, c, f, g, i, j, l, k, m], is_dual)
    }
}
/// n = 13, i = 4
fn select_1680([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1681([b, d, c, f, e, g, a, h, i, j, l, k, m], is_dual)
    } else {
        select_1685([b, d, e, a, f, g, c, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1668([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1669([a, b, c, d, h, e, k, j, i, g, l, n, m], is_dual)
    } else {
        select_1680([b, a, c, d, e, f, g, h, i, k, l, j, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1644([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && j < m) || (is_dual && j > m) {
        select_1645([a, b, d, c, e, f, g, h, i, k, l, m, n, j], is_dual)
    } else {
        select_1668([a, b, c, e, f, d, g, h, i, k, j, l, n, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_1694([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_123([a, c, d, b, e, g, f, h], is_dual)
    } else {
        select_743([a, f, c, d, b, g, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1693([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_693([a, b, d, e, f, g, h, i, j], is_dual)
    } else {
        select_1694([a, b, c, i, d, h, g, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_1696([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < e) || (is_dual && b > e) {
        select_230([a, e, d, b, f, g], is_dual)
    } else {
        select_427([a, b, c, d, f, e, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1695([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_693([a, b, d, c, e, f, g, h, i], is_dual)
    } else {
        select_1696([a, b, h, d, c, g, i], is_dual)
    }
}
/// n = 11, i = 2
fn select_1692([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1693([a, b, d, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1695([a, b, g, c, f, i, h, k, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1700([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_118([b, c, d, e, j, h, i], is_dual)
    } else {
        select_389([f, a, i, b, c, g, h, j, k, l], is_dual)
    }
}
/// n = 11, i = 3
fn select_1701([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_183([a, b, c, f, g, i, j, k], is_dual)
    } else {
        select_29([b, c, d, e, i, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1699([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < b) || (is_dual && h > b) {
        select_1700([c, b, d, e, f, a, g, h, i, j, k, l], is_dual)
    } else {
        select_1701([a, c, h, e, f, g, b, i, j, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1702([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_1198([a, c, b, h, f, i, j, g], is_dual)
    } else {
        select_967([a, c, d, e, g, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1698([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1699([b, d, e, f, a, c, g, h, i, j, k, l], is_dual)
    } else {
        select_1702([a, b, c, d, h, g, j, k, l, i], is_dual)
    }
}
/// n = 9, i = 2
fn select_1705([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_33([a, d, e, h, g], is_dual)
    } else {
        select_280([a, b, c, g, f, i, h], is_dual)
    }
}
/// n = 10, i = 2
fn select_1704([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_792([c, h, d, e, a, g, i], is_dual)
    } else {
        select_1705([a, b, g, d, e, f, i, h, j], is_dual)
    }
}
/// n = 12, i = 3
fn select_1703([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1704([b, c, e, d, f, g, h, i, j, k], is_dual)
    } else {
        select_174([a, c, b, d, h, f, g, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 3
fn select_1697([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1698([a, c, b, d, e, f, h, i, g, j, k, l], is_dual)
    } else {
        select_1703([c, d, a, e, f, h, g, i, b, j, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1691([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && m < l) || (is_dual && m > l) {
        select_1692([a, c, f, e, b, h, i, j, g, k, l], is_dual)
    } else {
        select_1697([b, a, d, c, j, e, g, h, i, k, m, n], is_dual)
    }
}
/// n = 11, i = 4
fn select_1710([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_155([a, b, c, e, i, h, k], is_dual)
    } else {
        select_158([a, c, d, h, f, g, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1712([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_49([a, b, h, f, g, j, k], is_dual)
    } else {
        select_34([c, d, e, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1711([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_440([h, b, c, a, f, j, g, k, i], is_dual)
    } else {
        select_1712([b, c, a, d, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1709([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_1710([a, g, e, c, b, f, h, i, j, k, l], is_dual)
    } else {
        select_1711([a, c, e, b, d, f, h, g, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1708([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < c) || (is_dual && i > c) {
        select_1709([a, b, d, e, c, f, g, h, j, i, k, l], is_dual)
    } else {
        select_1696([a, b, i, e, c, g, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1715([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_221([e, a, b, c, d, g, f, i], is_dual)
    } else {
        select_324([f, c, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1714([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1715([b, c, d, e, a, f, g, h, i], is_dual)
    } else {
        select_743([a, f, d, e, b, g, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1717([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_479([b, c, d, a, e, f, g, h, i], is_dual)
    } else {
        select_98([b, g, c, a, f, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1716([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1717([a, b, c, e, f, g, i, j, h, k], is_dual)
    } else {
        select_1717([a, b, d, e, f, g, h, j, i, k], is_dual)
    }
}
/// n = 12, i = 4
fn select_1713([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_1714([a, b, e, d, h, i, f, l, j], is_dual)
    } else {
        select_1716([a, b, e, f, c, g, j, i, h, k, l], is_dual)
    }
}
/// n = 13, i = 4
fn select_1707([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1708([a, b, f, c, e, g, h, i, j, l, m, k], is_dual)
    } else {
        select_1713([a, b, c, d, h, f, g, i, k, l, m, j], is_dual)
    }
}
/// n = 10, i = 4
fn select_1722([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_49([b, c, d, e, f, g, h], is_dual)
    } else {
        select_243([a, b, g, e, h, i, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1721([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1166([g, a, d, f, e, i, j, k], is_dual)
    } else {
        select_1722([e, d, b, c, f, h, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1720([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_645([b, e, d, f, a, g, h, i, j], is_dual)
    } else {
        select_1721([b, a, c, d, f, g, e, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1719([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1720([a, b, c, d, f, e, g, i, h, j, k], is_dual)
    } else {
        select_898([a, b, c, g, f, h, i, e, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1725([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_70([a, f, c, d, i, g], is_dual)
    } else {
        select_98([h, i, f, a, g, e, b], !is_dual)
    }
}
/// n = 10, i = 3
fn select_1724([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_1725([a, b, c, g, f, h, e, i, j], is_dual)
    } else {
        select_461([a, b, d, e, f, h, g, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1726([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_742([a, b, f, c, e, h, i], is_dual)
    } else {
        select_191([a, b, e, d, g, f, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1723([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1724([a, e, d, b, f, h, i, j, l, k], is_dual)
    } else {
        select_1726([f, b, c, a, g, k, j, m, i], is_dual)
    }
}
/// n = 14, i = 5
fn select_1718([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && a < i) || (is_dual && a > i) {
        select_1719([b, i, d, e, a, f, h, j, l, k, n], is_dual)
    } else {
        select_1723([b, a, c, d, e, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1706([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1707([b, c, e, a, f, d, g, h, i, j, k, l, m], is_dual)
    } else {
        select_1718([b, c, e, d, f, a, g, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1690([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1691([b, a, c, d, e, f, g, h, i, j, l, k, m, n], is_dual)
    } else {
        select_1706([a, b, c, i, d, f, h, j, k, g, l, m, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1643([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < l) || (is_dual && h > l) {
        select_1644([a, b, c, e, d, j, g, i, l, m, k, n, h, o], is_dual)
    } else {
        select_1690([a, b, d, e, c, f, g, i, h, j, k, m, l, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1581([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1582([a, b, c, e, d, f, g, h, j, k, i, l, n, m, o], is_dual)
    } else {
        select_1643([a, b, d, e, c, f, g, h, i, k, j, l, m, n, o], is_dual)
    }
}
/// n = 10, i = 3
fn select_1735([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_63([b, d, e, h, i], is_dual)
    } else {
        select_116([a, b, c, f, g, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1734([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < g) || (is_dual && a > g) {
        select_149([g, e, b, a, f, i, j], is_dual)
    } else {
        select_1735([b, a, e, c, d, f, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1733([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_1734([a, e, b, c, d, g, f, i, h, j], is_dual)
    } else {
        select_594([a, f, d, e, b, g, h, i, j], is_dual)
    }
}
/// n = 7, i = 2
fn select_1736([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_906([a, b, c, d, f, g], is_dual)
    } else {
        select_906([a, b, c, e, f, g], is_dual)
    }
}
/// n = 13, i = 4
fn select_1732([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1733([a, b, d, i, f, g, h, k, l, j], is_dual)
    } else {
        select_1736([a, c, g, e, j, i, m], is_dual)
    }
}
/// n = 9, i = 3
fn select_1738([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_230([a, c, d, f, g, h], is_dual)
    } else {
        select_1605([a, f, b, d, e, g, i, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1740([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_241([a, d, c, f, h, i, g], is_dual)
    } else {
        select_116([b, d, g, e, h, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1741([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_241([c, e, d, f, h, i, g], is_dual)
    } else {
        select_114([a, b, e, g, h, f, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1739([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_1740([c, b, e, d, f, a, g, h, i, j], is_dual)
    } else {
        select_1741([a, b, c, e, d, f, g, h, i, j], is_dual)
    }
}
/// n = 10, i = 3
fn select_1737([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_1738([a, d, b, g, h, f, c, j, i], is_dual)
    } else {
        select_1739([a, b, d, c, e, f, h, g, i, j], is_dual)
    }
}
/// n = 13, i = 4
fn select_1731([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1732([a, c, b, f, d, e, g, h, i, k, l, j, m], is_dual)
    } else {
        select_1737([b, a, d, c, f, j, g, k, i, m], is_dual)
    }
}
/// n = 8, i = 3
fn select_1746([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_49([a, b, c, d, e, f, g], is_dual)
    } else {
        select_39([d, b, c, e, f, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1745([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_255([a, b, h, e, i], is_dual)
    } else {
        select_1746([b, c, d, e, g, f, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1744([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1092([c, a, d, e, b, f, g, i, h], is_dual)
    } else {
        select_1745([c, b, d, e, a, g, f, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1743([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_402([a, b, d, f, c, g, h, i], is_dual)
    } else {
        select_1744([a, b, d, c, e, g, f, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1748([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_398([b, a, g, c, e, f, i, j, h], is_dual)
    } else {
        select_398([b, a, f, d, e, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1749([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1107([a, b, g, c, e, f, i, j, h], is_dual)
    } else {
        select_1107([a, b, f, d, e, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1747([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1748([c, a, d, e, b, f, g, h, i, j], is_dual)
    } else {
        select_1749([b, c, d, e, a, f, g, h, i, j], is_dual)
    }
}
/// n = 12, i = 4
fn select_1742([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1743([c, a, b, e, d, i, k, l, h], is_dual)
    } else {
        select_1747([a, d, c, b, f, h, g, j, k, i], is_dual)
    }
}
/// n = 13, i = 4
fn select_1730([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1731([a, c, d, e, b, f, h, g, i, k, j, l, m], is_dual)
    } else {
        select_1742([a, c, h, b, e, d, j, i, g, l, m, k], is_dual)
    }
}
/// n = 9, i = 4
fn select_1754([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_143([e, g, i, d, b, f, c], !is_dual)
    } else {
        select_500([a, b, c, e, f, d, g, i, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1753([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_1754([b, a, c, e, d, f, h, g, i], is_dual)
    } else {
        select_887([e, h, d, i, g, a, b, f], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1752([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_761([a, c, e, h, f, g, j, i], is_dual)
    } else {
        select_1753([k, h, m, a, b, l, c, d, i], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1756([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < g) || (is_dual && e > g) {
        select_37([a, h, j, e, i, b, f, g], !is_dual)
    } else {
        select_743([a, d, c, e, f, h, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1757([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_123([a, c, f, d, e, h, g, i], is_dual)
    } else {
        select_56([a, f, k, b, j, i], !is_dual)
    }
}
/// n = 11, i = 4
fn select_1755([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && c < g) || (is_dual && c > g) {
        select_1756([a, b, d, g, f, c, i, h, j, k], is_dual)
    } else {
        select_1757([a, b, d, c, e, f, g, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 5
fn select_1751([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < i) || (is_dual && k > i) {
        select_1752([a, b, c, e, d, f, g, h, i, j, l, k, m], is_dual)
    } else {
        select_1755([a, b, d, c, f, h, g, j, k, i, m], is_dual)
    }
}
/// n = 8, i = 2
fn select_1760([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1649([c, d, a, e, f, b, h, g], is_dual)
    } else {
        select_1649([c, d, b, e, f, a, h, g], is_dual)
    }
}
/// n = 9, i = 2
fn select_1761([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_912([a, c, g, e, f, b, h, i], is_dual)
    } else {
        select_1043([a, c, e, b, d, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 3
fn select_1759([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_1760([b, c, a, e, j, g, i, l], is_dual)
    } else {
        select_1761([a, d, e, f, i, g, h, k, j], is_dual)
    }
}
/// n = 14, i = 5
fn select_1758([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_1759([c, b, d, a, f, e, g, h, i, j, l, k], is_dual)
    } else {
        select_1752([a, b, g, d, c, e, h, i, k, l, j, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1750([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1751([a, b, h, c, e, d, j, i, g, l, m, k, n], is_dual)
    } else {
        select_1758([a, b, d, e, c, f, h, g, i, k, j, l, m, n], is_dual)
    }
}
/// n = 14, i = 5
fn select_1729([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1730([c, a, b, d, e, f, g, i, j, k, h, l, m], is_dual)
    } else {
        select_1750([a, b, c, d, e, f, h, i, j, k, g, l, m, n], is_dual)
    }
}
/// n = 9, i = 3
fn select_1766([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < b) || (is_dual && f > b) {
        select_509([a, c, d, b, e, g, f, h], is_dual)
    } else {
        select_188([a, c, f, d, b, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1768([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_41([d, b, c, f, h, g], is_dual)
    } else {
        select_70([a, d, e, g, f, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1767([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < f) || (is_dual && a > f) {
        select_625([b, d, c, e, f, a, g, h, i, j], is_dual)
    } else {
        select_1768([d, a, c, b, e, g, f, i, h], is_dual)
    }
}
/// n = 11, i = 4
fn select_1765([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1766([a, b, d, f, e, g, h, j, i], is_dual)
    } else {
        select_1767([a, b, c, d, g, f, i, j, k, h], is_dual)
    }
}
/// n = 8, i = 3
fn select_1770([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_1300([a, b, c, d, e, f, h, g], is_dual)
    } else {
        select_487([a, b, f, d, g, h, e], is_dual)
    }
}
/// n = 10, i = 4
fn select_1769([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1293([h, j, i, a, b, g, f, d, e], !is_dual)
    } else {
        select_1770([h, i, j, a, b, g, c, d], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1764([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1765([a, d, b, c, f, h, g, j, k, i, m], is_dual)
    } else {
        select_1769([a, b, c, e, d, i, k, l, h, m], is_dual)
    }
}
/// n = 14, i = 5
fn select_1763([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1764([a, b, h, c, e, d, j, i, g, l, m, k, n], is_dual)
    } else {
        select_1758([a, b, d, e, c, f, h, g, i, k, j, l, m, n], is_dual)
    }
}
/// n = 8, i = 3
fn select_1775([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_148([c, d, a, e, b, h, f, g], is_dual)
    } else {
        select_646([b, c, d, a, e, f, h, g], is_dual)
    }
}
/// n = 8, i = 3
fn select_1774([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_1775([b, d, c, e, a, g, h, f], is_dual)
    } else {
        select_887([b, d, a, e, c, f, g, h], is_dual)
    }
}
/// n = 13, i = 4
fn select_1773([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < k) || (is_dual && i > k) {
        select_1774([a, b, c, d, k, l, i, m], is_dual)
    } else {
        select_508([a, c, i, e, f, g, h, j, k], is_dual)
    }
}
/// n = 9, i = 2
fn select_1778([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_114([a, b, c, h, f, g, i], is_dual)
    } else {
        select_69([c, d, e, g, f, h], is_dual)
    }
}
/// n = 12, i = 3
fn select_1777([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && k < j) || (is_dual && k > j) {
        select_61([e, f, h, c, i, g, j], is_dual)
    } else {
        select_1778([a, b, d, c, g, i, h, k, l], is_dual)
    }
}
/// n = 10, i = 3
fn select_1780([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < g) || (is_dual && i > g) {
        select_62([c, e, d, g, f, h], is_dual)
    } else {
        select_82([a, b, i, j], is_dual)
    }
}
/// n = 8, i = 2
fn select_1781([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_210([a, b, g, h], is_dual)
    } else {
        select_118([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 12, i = 3
fn select_1779([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1780([a, b, c, i, d, g, h, k, j, l], is_dual)
    } else {
        select_1781([a, b, e, f, j, h, i, l], is_dual)
    }
}
/// n = 13, i = 3
fn select_1776([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1777([a, b, c, g, e, f, i, j, h, l, k, m], is_dual)
    } else {
        select_1779([a, b, c, f, d, h, i, j, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 4
fn select_1772([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < m) || (is_dual && k > m) {
        select_1773([a, b, g, c, d, f, h, i, j, l, m, k, n], is_dual)
    } else {
        select_1776([b, c, a, e, d, f, g, h, i, j, k, l, m], is_dual)
    }
}
/// n = 11, i = 4
fn select_1783([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && h < i) || (is_dual && h > i) {
        select_1497([a, b, c, d, g, f, i, j, k, h], is_dual)
    } else {
        select_1766([b, a, d, f, e, g, h, j, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1785([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_311([b, c, e, d, g, f, h], is_dual)
    } else {
        select_906([d, g, i, e, a, h], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1784([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < b) || (is_dual && e > b) {
        select_1306([h, j, i, a, g, c, f, d, e], !is_dual)
    } else {
        select_1785([j, i, h, a, g, c, b, f, d], !is_dual)
    }
}
/// n = 13, i = 5
fn select_1782([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_1783([b, c, a, d, f, h, g, j, k, i, m], is_dual)
    } else {
        select_1784([a, c, d, b, e, i, k, l, h, m], is_dual)
    }
}
/// n = 15, i = 5
fn select_1771([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < g) || (is_dual && k > g) {
        select_1772([c, b, d, e, f, a, h, i, g, j, l, m, k, n], is_dual)
    } else {
        select_1782([b, a, c, h, d, i, k, j, g, m, n, l, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1762([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && g < j) || (is_dual && g > j) {
        select_1763([a, b, c, i, d, f, j, h, k, l, g, m, n, o], is_dual)
    } else {
        select_1771([a, b, c, d, e, f, g, h, i, k, l, j, m, n, o], is_dual)
    }
}
/// n = 15, i = 5
fn select_1728([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && b < h) || (is_dual && b > h) {
        select_1729([a, c, h, j, d, f, g, b, i, k, l, m, n, o], is_dual)
    } else {
        select_1762([a, c, b, d, e, f, g, i, j, h, k, l, m, n, o], is_dual)
    }
}
/// n = 11, i = 3
fn select_1791([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < h) || (is_dual && g > h) {
        select_227([a, b, h, d, e, f, g, i, k, j], is_dual)
    } else {
        select_227([a, c, g, d, e, f, h, i, j, k], is_dual)
    }
}
/// n = 11, i = 3
fn select_1790([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1791([a, c, d, e, f, b, g, h, i, j, k], is_dual)
    } else {
        select_1791([b, c, d, e, f, a, g, h, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1793([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_604([a, c, d, e, f, b, g, i], is_dual)
    } else {
        select_1589([c, b, d, e, f, a, g, i, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1792([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < g) || (is_dual && b > g) {
        select_310([a, b, h, i, c, d, g], !is_dual)
    } else {
        select_1793([a, c, d, b, e, f, g, h, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1789([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_1790([a, b, e, f, c, d, g, h, i, j, k], is_dual)
    } else {
        select_1792([a, b, c, h, d, g, j, i, l], is_dual)
    }
}
/// n = 8, i = 3
fn select_1797([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_375([a, b, c, d, e, f, h], is_dual)
    } else {
        select_324([e, c, d, a, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1796([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_920([a, c, d, f, e, g, h, i], is_dual)
    } else {
        select_1797([e, c, b, h, g, f, j, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1798([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_920([b, c, d, f, e, g, h, i], is_dual)
    } else {
        select_188([e, a, c, h, g, f, j, i], is_dual)
    }
}
/// n = 11, i = 3
fn select_1795([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && d < a) || (is_dual && d > a) {
        select_1796([b, d, c, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1798([a, b, c, e, f, h, g, i, j, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1800([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_230([e, b, c, g, h, f], is_dual)
    } else {
        select_1107([a, b, d, c, e, f, g, h, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1802([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_42([b, d, c, f, g, e], is_dual)
    } else {
        select_143([b, a, e, g, h, d, f], is_dual)
    }
}
/// n = 9, i = 3
fn select_1803([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && g < e) || (is_dual && g > e) {
        select_130([b, c, d, e, f, h, g], is_dual)
    } else {
        select_143([b, a, g, f, i, e, h], is_dual)
    }
}
/// n = 9, i = 3
fn select_1801([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1802([a, b, d, f, h, e, g, i], is_dual)
    } else {
        select_1803([a, b, d, c, e, g, h, f, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1799([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_1800([a, f, d, c, e, h, g, i, j], is_dual)
    } else {
        select_1801([a, g, d, b, e, i, f, h, j], is_dual)
    }
}
/// n = 11, i = 3
fn select_1794([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && g < a) || (is_dual && g > a) {
        select_1795([a, b, d, c, e, f, h, g, i, j, k], is_dual)
    } else {
        select_1799([g, b, d, e, a, f, h, i, j, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1788([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1789([a, f, b, i, e, d, j, h, g, l, k, m], is_dual)
    } else {
        select_1794([a, c, d, e, g, f, h, j, i, k, l], is_dual)
    }
}
/// n = 9, i = 3
fn select_1808([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_130([b, d, c, e, g, f, h], is_dual)
    } else {
        select_322([a, b, f, d, e, g, h, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1807([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_397([c, d, b, e, f, a, g, i, h], is_dual)
    } else {
        select_1808([c, d, a, e, f, b, g, h, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1806([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < f) || (is_dual && g > f) {
        select_190([b, a, d, f, e, i, h, g], is_dual)
    } else {
        select_1807([b, a, d, c, g, h, f, j, i], is_dual)
    }
}
/// n = 9, i = 3
fn select_1810([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_458([a, c, e, d, g, f, h], is_dual)
    } else {
        select_127([a, b, c, f, g, h, i, e], is_dual)
    }
}
/// n = 7, i = 3
fn select_1812([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_81([d, a, b, f, g], is_dual)
    } else {
        select_42([a, c, b, d, e, g], is_dual)
    }
}
/// n = 11, i = 4
fn select_1813([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_62([c, e, d, g, f, h], is_dual)
    } else {
        select_521([a, b, c, d, f, g, i, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1811([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && a < h) || (is_dual && a > h) {
        select_1812([b, c, h, a, g, j, k], is_dual)
    } else {
        select_1813([b, c, d, a, e, f, g, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1809([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && b < f) || (is_dual && b > f) {
        select_1810([a, f, d, e, b, g, h, i, k], is_dual)
    } else {
        select_1811([a, b, c, d, e, g, f, i, h, j, k], is_dual)
    }
}
/// n = 11, i = 4
fn select_1805([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && j < h) || (is_dual && j > h) {
        select_1806([b, c, d, a, e, f, g, h, i, j], is_dual)
    } else {
        select_1809([b, a, d, c, e, f, g, i, j, h, k], is_dual)
    }
}
/// n = 9, i = 3
fn select_1815([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_1807([b, a, e, d, c, g, f, i, h], is_dual)
    } else {
        select_747([b, e, a, f, g, c, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1817([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_639([a, c, d, b, e, f, g, h, j], is_dual)
    } else {
        select_42([b, a, g, e, j, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1819([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && d < f) || (is_dual && d > f) {
        select_529([a, b, f, e, d, h, i], is_dual)
    } else {
        select_143([b, c, d, e, g, f, h], is_dual)
    }
}
/// n = 10, i = 4
fn select_1818([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_1819([a, b, c, e, f, g, h, i, j], is_dual)
    } else {
        select_209([b, c, d, f, h, g, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1816([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_1817([b, c, d, e, a, f, g, h, i, j], is_dual)
    } else {
        select_1818([c, a, d, e, f, b, g, h, j, i], is_dual)
    }
}
/// n = 10, i = 4
fn select_1814([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < f) || (is_dual && h > f) {
        select_1815([b, c, d, e, a, g, f, i, h], is_dual)
    } else {
        select_1816([b, a, e, c, d, g, h, i, f, j], is_dual)
    }
}
/// n = 11, i = 4
fn select_1804([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_1805([a, b, c, d, e, g, h, f, i, j, k], is_dual)
    } else {
        select_1814([f, b, c, h, d, a, g, j, i, k], is_dual)
    }
}
/// n = 13, i = 4
fn select_1787([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_1788([a, b, c, d, e, g, f, h, i, j, k, l, m], is_dual)
    } else {
        select_1804([a, b, j, h, c, g, k, i, f, l, m], is_dual)
    }
}
/// n = 10, i = 4
fn select_1824([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_398([a, b, c, d, e, f, g, h, j], is_dual)
    } else {
        select_1263([i, j, e, a, g, b, h, f], !is_dual)
    }
}
/// n = 12, i = 4
fn select_1823([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && g < i) || (is_dual && g > i) {
        select_1824([a, b, i, e, f, h, g, k, l, j], is_dual)
    } else {
        select_275([a, c, e, d, g, f, h, i, j, k], is_dual)
    }
}
/// n = 8, i = 3
fn select_1827([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_196([d, b, c, e, f], is_dual)
    } else {
        select_39([d, g, h, a, e, b], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1826([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_1827([e, h, j, a, f, i, c, g], !is_dual)
    } else {
        select_276([a, b, d, f, e, h, g, i], is_dual)
    }
}
/// n = 8, i = 3
fn select_1829([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < d) || (is_dual && g > d) {
        select_469([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_107([d, h, a, e, g, b, f], !is_dual)
    }
}
/// n = 9, i = 4
fn select_1828([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_1829([d, h, i, a, g, e, f, b], !is_dual)
    } else {
        select_1829([d, h, i, a, g, f, e, c], !is_dual)
    }
}
/// n = 10, i = 4
fn select_1825([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_1826([a, c, d, b, e, f, g, h, i, j], is_dual)
    } else {
        select_1828([a, d, g, e, f, b, i, j, h], is_dual)
    }
}
/// n = 12, i = 4
fn select_1822([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_1823([a, b, c, e, d, f, g, h, i, j, k, l], is_dual)
    } else {
        select_1825([a, b, c, h, f, j, i, g, k, l], is_dual)
    }
}
/// n = 7, i = 2
fn select_1833([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < c) || (is_dual && f > c) {
        select_114([a, b, d, c, e, f, g], is_dual)
    } else {
        select_25([d, f, e, c], is_dual)
    }
}
/// n = 8, i = 3
fn select_1832([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1833([a, c, d, b, e, g, f], is_dual)
    } else {
        select_926([a, c, b, d, f, g, e, h], is_dual)
    }
}
/// n = 9, i = 4
fn select_1834([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1247([c, a, d, e, b, g, f, h], is_dual)
    } else {
        select_1631([b, c, d, a, e, f, g, i], is_dual)
    }
}
/// n = 9, i = 4
fn select_1831([a, b, c, d, e, f, g, h, i]: [usize; 9], is_dual: bool) -> usize {
    if (!is_dual && h < e) || (is_dual && h > e) {
        select_1832([b, c, d, a, f, e, g, h], is_dual)
    } else {
        select_1834([a, b, c, d, f, g, h, e, i], is_dual)
    }
}
/// n = 10, i = 3
fn select_1836([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && e < h) || (is_dual && e > h) {
        select_215([a, h, b, c, f, g, e, i, j], is_dual)
    } else {
        select_29([b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 10, i = 3
fn select_1835([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < f) || (is_dual && i > f) {
        select_1836([a, c, b, d, e, g, f, h, j, i], is_dual)
    } else {
        select_1236([a, b, g, h, e, i, f], is_dual)
    }
}
/// n = 11, i = 4
fn select_1830([a, b, c, d, e, f, g, h, i, j, k]: [usize; 11], is_dual: bool) -> usize {
    if (!is_dual && f < i) || (is_dual && f > i) {
        select_1831([e, j, f, k, a, b, g, i, h], !is_dual)
    } else {
        select_1835([a, b, c, d, e, f, g, h, j, i], is_dual)
    }
}
/// n = 12, i = 4
fn select_1821([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_1822([a, b, d, e, c, f, g, i, h, j, k, l], is_dual)
    } else {
        select_1830([a, b, h, e, f, g, c, i, k, j, l], is_dual)
    }
}
/// n = 12, i = 4
fn select_1820([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_1821([a, c, d, e, f, b, g, h, i, j, k, l], is_dual)
    } else {
        select_1821([b, c, d, e, f, a, g, h, i, j, k, l], is_dual)
    }
}
/// n = 14, i = 4
fn select_1786([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_1787([a, b, c, d, j, f, g, i, h, k, m, l, n], is_dual)
    } else {
        select_1820([a, g, b, i, h, e, f, l, j, m, k, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1727([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && i < d) || (is_dual && i > d) {
        select_1728([a, b, c, d, e, f, g, h, j, k, i, l, m, n, o], is_dual)
    } else {
        select_1786([b, a, e, f, c, g, h, k, j, i, d, l, m, n], is_dual)
    }
}
/// n = 15, i = 5
fn select_1580([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1581([a, b, c, e, d, f, g, h, j, i, k, m, n, o, l], is_dual)
    } else {
        select_1727([a, b, c, d, e, f, g, h, j, k, i, l, n, m, o], is_dual)
    }
}
/// n = 16, i = 5
fn select_1269([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < i) || (is_dual && m > i) {
        select_1270([a, b, d, e, c, f, g, h, j, k, i, l, n, o, m, p], is_dual)
    } else {
        select_1580([a, b, d, c, e, l, g, h, j, k, m, o, i, p, n], is_dual)
    }
}
/// n = 16, i = 5
fn select_1268([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < m) || (is_dual && l > m) {
        select_1269([d, e, f, b, c, a, k, j, m, i, h, g, n, l, o, p], is_dual)
    } else {
        select_1269([d, f, e, b, c, a, k, j, l, i, h, g, n, m, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_531([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && g < c) || (is_dual && g > c) {
        select_532([a, b, c, d, e, f, h, i, g, j, k, l, n, m, o, p], is_dual)
    } else {
        select_1268([a, b, g, d, e, f, h, i, c, m, j, k, l, n, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([h, a, g, b, c, d, e, f, j, k, l, m, n, i, p, o], is_dual)
    } else {
        select_531([a, b, g, c, d, e, f, i, j, k, l, m, n, h, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < j) || (is_dual && o > j) {
        select_10([a, c, d, e, f, g, b, h, i, j, k, l, m, n, p, o], is_dual)
    } else {
        select_10([a, h, d, e, f, g, b, c, i, o, k, l, m, n, p, j], is_dual)
    }
}
/// n = 16, i = 5
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_9([a, b, c, d, e, f, g, h, i, k, l, m, n, o, p, j], is_dual)
    } else {
        select_9([b, a, c, d, e, f, g, h, j, k, l, m, n, o, p, i], is_dual)
    }
}
/// n = 16, i = 5
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_8([a, b, c, d, e, f, g, h, j, k, l, m, n, o, p, i], is_dual)
    } else {
        select_8([a, b, c, d, e, f, g, i, j, k, l, m, n, o, p, h], is_dual)
    }
}
/// n = 16, i = 5
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_7([a, b, c, d, e, f, g, h, i, k, l, m, n, o, p, j], is_dual)
    } else {
        select_7([a, b, c, d, e, f, j, h, i, k, l, m, n, o, p, g], is_dual)
    }
}
/// n = 16, i = 5
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_6([a, b, c, d, e, f, g, h, i, j, l, m, n, o, p, k], is_dual)
    } else {
        select_6([a, b, c, d, e, k, g, h, i, j, l, m, n, o, p, f], is_dual)
    }
}
/// n = 16, i = 5
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < e) || (is_dual && l > e) {
        select_5([a, b, c, d, e, f, g, h, i, j, k, m, n, o, p, l], is_dual)
    } else {
        select_5([a, b, c, d, l, f, g, h, i, j, k, m, n, o, p, e], is_dual)
    }
}
/// n = 16, i = 5
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && m < d) || (is_dual && m > d) {
        select_4([a, b, c, d, e, f, g, h, i, j, k, l, n, o, p, m], is_dual)
    } else {
        select_4([a, b, c, m, e, f, g, h, i, j, k, l, n, o, p, d], is_dual)
    }
}
/// n = 16, i = 5
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && l < a) || (is_dual && l > a) {
        select_3([a, m, n, b, c, d, e, f, g, h, i, j, k, l, o, p], is_dual)
    } else {
        select_3([l, m, n, b, c, d, e, f, g, h, i, j, k, a, o, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m, p], is_dual)
    }
}
/// n = 16, i = 5
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < o) || (is_dual && p > o) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o], is_dual)
    }
}
