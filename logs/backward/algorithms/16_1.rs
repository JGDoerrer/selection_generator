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
/// n = 4, i = 0
fn select_15([a, b, c, d]: [usize; 4], is_dual: bool) -> usize {
    if (!is_dual && a < d) || (is_dual && a > d) {
        select_16([a, b, c], is_dual)
    } else {
        select_16([b, c, d], is_dual)
    }
}
/// n = 8, i = 1
fn select_14([a, b, c, d, e, f, g, h]: [usize; 8], is_dual: bool) -> usize {
    if (!is_dual && h < g) || (is_dual && h > g) {
        select_15([d, e, f, g], is_dual)
    } else {
        select_15([a, b, c, h], is_dual)
    }
}
/// n = 10, i = 1
fn select_13([a, b, c, d, e, f, g, h, i, j]: [usize; 10], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_14([a, b, c, f, g, h, j, i], is_dual)
    } else {
        select_14([a, b, c, d, e, i, j, h], is_dual)
    }
}
/// n = 12, i = 1
fn select_12([a, b, c, d, e, f, g, h, i, j, k, l]: [usize; 12], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_13([c, d, i, e, f, g, h, k, l, j], is_dual)
    } else {
        select_13([a, b, j, e, f, g, h, k, l, i], is_dual)
    }
}
/// n = 13, i = 1
fn select_11([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < j) || (is_dual && i > j) {
        select_12([a, j, c, d, e, f, g, h, i, k, l, m], is_dual)
    } else {
        select_12([b, i, c, d, e, f, g, h, j, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_10([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && i < h) || (is_dual && i > h) {
        select_11([a, h, b, c, d, e, f, g, j, i, k, l, m], is_dual)
    } else {
        select_11([a, i, b, c, d, e, f, g, j, h, k, l, m], is_dual)
    }
}
/// n = 13, i = 1
fn select_9([a, b, c, d, e, f, g, h, i, j, k, l, m]: [usize; 13], is_dual: bool) -> usize {
    if (!is_dual && j < g) || (is_dual && j > g) {
        select_10([g, a, b, c, d, e, f, h, i, j, k, l, m], is_dual)
    } else {
        select_10([j, a, b, c, d, e, f, h, i, g, k, l, m], is_dual)
    }
}
/// n = 14, i = 1
fn select_8([a, b, c, d, e, f, g, h, i, j, k, l, m, n]: [usize; 14], is_dual: bool) -> usize {
    if (!is_dual && k < l) || (is_dual && k > l) {
        select_9([a, l, c, d, e, f, g, h, i, j, k, m, n], is_dual)
    } else {
        select_9([b, k, c, d, e, f, g, h, i, j, l, m, n], is_dual)
    }
}
/// n = 15, i = 1
fn select_7([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && m < n) || (is_dual && m > n) {
        select_8([c, d, a, b, e, n, g, h, i, j, k, l, o, m], is_dual)
    } else {
        select_8([c, d, a, b, f, m, g, h, i, j, k, l, o, n], is_dual)
    }
}
/// n = 15, i = 1
fn select_6([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && k < f) || (is_dual && k > f) {
        select_7([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o], is_dual)
    } else {
        select_7([a, b, c, d, e, k, g, h, i, j, l, m, n, f, o], is_dual)
    }
}
/// n = 15, i = 1
fn select_5([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && h < a) || (is_dual && h > a) {
        select_6([k, l, a, i, j, b, c, d, e, f, g, h, m, n, o], is_dual)
    } else {
        select_6([k, l, h, i, j, b, c, d, e, f, g, a, m, n, o], is_dual)
    }
}
/// n = 15, i = 1
fn select_4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && j < i) || (is_dual && j > i) {
        select_5([a, b, c, d, e, f, g, h, i, k, l, m, j, n, o], is_dual)
    } else {
        select_5([a, b, c, d, e, f, g, h, j, k, l, m, i, n, o], is_dual)
    }
}
/// n = 15, i = 1
fn select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]: [usize; 15], is_dual: bool) -> usize {
    if (!is_dual && l < k) || (is_dual && l > k) {
        select_4([a, b, c, d, e, f, g, h, i, j, k, m, n, l, o], is_dual)
    } else {
        select_4([a, b, c, d, e, f, g, h, i, j, l, m, n, k, o], is_dual)
    }
}
/// n = 16, i = 1
fn select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && o < p) || (is_dual && o > p) {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, m, p, o], is_dual)
    } else {
        select_3([a, b, c, d, e, f, g, h, i, j, k, l, n, o, p], is_dual)
    }
}
/// n = 16, i = 1
fn select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && n < m) || (is_dual && n > m) {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, m, o, n, p], is_dual)
    } else {
        select_2([a, b, c, d, e, f, g, h, i, j, k, l, n, o, m, p], is_dual)
    }
}
/// n = 16, i = 1
fn select_0([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]: [usize; 16], is_dual: bool) -> usize {
    if (!is_dual && p < o) || (is_dual && p > o) {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p], is_dual)
    } else {
        select_1([a, b, c, d, e, f, g, h, i, j, k, l, m, n, p, o], is_dual)
    }
}
