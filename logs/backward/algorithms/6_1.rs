/// n = 1, i = 0
fn select_7([a]: [usize; 1], _: bool) -> usize {
    a
}
/// n = 2, i = 0
fn select_6([a, b]: [usize; 2], is_dual: bool) -> usize {
    if (!is_dual && a < b) || (is_dual && a > b) {
        select_7([a], is_dual)
    } else {
        select_7([b], is_dual)
    }
}
/// n = 3, i = 0
fn select_5([a, b, c]: [usize; 3], is_dual: bool) -> usize {
    if (!is_dual && a < c) || (is_dual && a > c) {
        select_6([a, b], is_dual)
    } else {
        select_6([b, c], is_dual)
    }
}
/// n = 6, i = 1
fn select_4([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_5([c, d, e], is_dual)
    } else {
        select_5([a, b, f], is_dual)
    }
}
/// n = 5, i = 1
fn select_8([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && e < d) || (is_dual && e > d) {
        select_5([b, c, d], is_dual)
    } else {
        select_6([a, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_3([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && e < a) || (is_dual && e > a) {
        select_4([a, b, c, d, e, f], is_dual)
    } else {
        select_8([e, c, d, a, f], is_dual)
    }
}
/// n = 6, i = 1
fn select_2([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && c < b) || (is_dual && c > b) {
        select_3([a, b, d, e, c, f], is_dual)
    } else {
        select_3([a, c, d, e, b, f], is_dual)
    }
}
/// n = 4, i = 1
fn select_12([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && c < d) || (is_dual && c > d) {
        select_6([a, d], is_dual)
    } else {
        select_6([b, c], is_dual)
    }
}
/// n = 4, i = 1
fn select_11([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && b < a) || (is_dual && b > a) {
        select_12([a, c, b, d], is_dual)
    } else {
        select_12([b, c, a, d], is_dual)
    }
}
/// n = 4, i = 1
fn select_10([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && d < c) || (is_dual && d > c) {
        select_11([a, b, c, d], is_dual)
    } else {
        select_11([a, b, d, c], is_dual)
    }
}
/// n = 5, i = 1
fn select_9([a, b, c, d, e]: [usize; 5], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_10([a, b, c, e], is_dual)
    } else {
        select_11([b, c, d, e], is_dual)
    }
}
/// n = 6, i = 1
fn select_1([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < d) || (is_dual && f > d) {
        select_2([a, b, c, d, e, f], is_dual)
    } else {
        select_9([a, b, c, f, d], is_dual)
    }
}
/// n = 6, i = 1
fn select_0([a, b, c, d, e, f]: [usize; 6], is_dual: bool) -> usize {
    if (!is_dual && f < e) || (is_dual && f > e) {
        select_1([a, b, c, d, e, f], is_dual)
    } else {
        select_1([a, b, c, d, f, e], is_dual)
    }
}
