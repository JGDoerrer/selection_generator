/// n = 1, i = 0
fn select_11([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_10([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_11([a], is_dual)
    } else {
        select_11([b], is_dual)
    }
}
/// n = 3, i = 1
fn select_9([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < a) || (is_dual && c > a) {
        select_10([a, b], is_dual)
    } else {
        select_11([c], is_dual)
    }
}
/// n = 3, i = 1
fn select_8([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_9([a, b, c], is_dual)
    } else {
        select_9([a, c, b], is_dual)
    }
}
/// n = 4, i = 1
fn select_12([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_10([a, d], is_dual)
    } else {
        select_10([b, c], is_dual)
    }
}
/// n = 5, i = 2
fn select_7([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && c < e) || (is_dual && c > e) {
        select_8([a, d, e], is_dual)
    } else {
        select_12([a, b, c, d], is_dual)
    }
}
/// n = 3, i = 0
fn select_14([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_10([a, b], is_dual)
    } else {
        select_10([b, c], is_dual)
    }
}
/// n = 6, i = 2
fn select_13([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_14([b, c, d], is_dual)
    } else {
        select_9([a, e, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_6([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && a < e) || (is_dual && a > e) {
        select_7([b, e, d, a, f], is_dual)
    } else {
        select_13([b, a, c, d, e, f], is_dual)
    }
}
/// n = 4, i = 0
fn select_17([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_14([a, b, c], is_dual)
    } else {
        select_14([b, c, d], is_dual)
    }
}
/// n = 5, i = 1
fn select_16([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_17([a, b, c, d], is_dual)
    } else {
        select_11([e], is_dual)
    }
}
/// n = 5, i = 2
fn select_19([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < c) || (is_dual && e > c) {
        select_12([a, b, c, d], is_dual)
    } else {
        select_9([d, a, e], is_dual)
    }
}
/// n = 5, i = 2
fn select_18([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_19([b, c, a, d, e], is_dual)
    } else {
        select_12([a, c, b, d], is_dual)
    }
}
/// n = 7, i = 2
fn select_15([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_16([a, c, d, e, f], is_dual)
    } else {
        select_18([a, b, f, g, e], is_dual)
    }
}
/// n = 7, i = 2
fn select_5([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < f) || (is_dual && c > f) {
        select_6([a, b, f, e, c, g], is_dual)
    } else {
        select_15([a, b, c, d, e, f, g], is_dual)
    }
}
/// n = 7, i = 2
fn select_4([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_5([a, b, d, e, c, f, g], is_dual)
    } else {
        select_5([a, c, d, e, b, f, g], is_dual)
    }
}
/// n = 6, i = 1
fn select_24([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_14([c, d, e], is_dual)
    } else {
        select_14([a, b, f], is_dual)
    }
}
/// n = 7, i = 1
fn select_23([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && e < f) || (is_dual && e > f) {
        select_24([a, f, c, d, e, g], is_dual)
    } else {
        select_24([b, e, c, d, f, g], is_dual)
    }
}
/// n = 7, i = 1
fn select_22([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_23([a, c, d, e, b, f, g], is_dual)
    } else {
        select_23([b, c, d, e, a, f, g], is_dual)
    }
}
/// n = 5, i = 1
fn select_27([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_14([b, c, d], is_dual)
    } else {
        select_10([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_26([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_24([a, b, c, d, e, f], is_dual)
    } else {
        select_27([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 2
fn select_28([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && d < e) || (is_dual && d > e) {
        select_19([d, f, e, a, b], !is_dual)
    } else {
        select_19([e, f, d, a, c], !is_dual)
    }
}
/// n = 7, i = 2
fn select_25([a, b, c, d, e, f, g]: [usize; 7], is_dual: bool) -> usize {
    if (!is_dual && f < a) || (is_dual && f > a) {
        select_26([b, c, a, d, e, f], is_dual)
    } else {
        select_28([b, c, f, e, a, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_21([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && g < b) || (is_dual && g > b) {
        select_22([a, c, d, b, e, f, g], is_dual)
    } else {
        select_25([a, c, g, d, b, f, h], is_dual)
    }
}
/// n = 6, i = 2
fn select_29([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_13([a, c, d, b, e, f], is_dual)
    } else {
        select_13([b, c, d, a, e, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_20([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < c) || (is_dual && h > c) {
        select_21([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_29([a, b, f, g, h, c], is_dual)
    }
}
/// n = 8, i = 2
fn select_3([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_4([a, b, c, d, g, h, f], is_dual)
    } else {
        select_20([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_2([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_3([a, b, c, d, f, e, g, h], is_dual)
    } else {
        select_3([a, b, c, e, f, d, g, h], is_dual)
    }
}
/// n = 6, i = 1
fn select_34([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_26([a, b, d, e, c, f], is_dual)
    } else {
        select_26([a, c, d, e, b, f], is_dual)
    }
}
/// n = 8, i = 2
fn select_33([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_22([a, d, e, b, c, f, g], is_dual)
    } else {
        select_34([a, b, c, d, f, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_32([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < g) || (is_dual && f > g) {
        select_33([a, d, e, b, c, g, h, f], is_dual)
    } else {
        select_33([a, d, e, c, b, f, h, g], is_dual)
    }
}
/// n = 8, i = 2
fn select_31([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_32([a, b, d, e, f, c, g, h], is_dual)
    } else {
        select_32([a, c, d, e, f, b, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_30([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_31([a, b, c, d, f, g, e, h], is_dual)
    } else {
        select_31([a, b, c, e, f, g, d, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_1([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && f < h) || (is_dual && f > h) {
        select_2([a, b, c, d, e, g, h, f], is_dual)
    } else {
        select_30([a, b, c, d, e, f, g, h], is_dual)
    }
}
/// n = 8, i = 2
fn select_0([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_1([a, b, c, d, e, f, g, h], is_dual)
    } else {
        select_1([a, b, c, d, e, f, h, g], is_dual)
    }
}
