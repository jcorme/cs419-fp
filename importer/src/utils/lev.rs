use std::collections::HashMap;

pub fn damlev(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    let inf = a_len + b_len;
    let mut m = Vec::new();
    m.reserve(a_len + 2);
    m.push(vec![inf; b_len + 2]);

    let mut tmp = vec![inf];
    tmp.extend(0..(b_len + 1));
    m.push(tmp);

    for i in 1..(a_len + 1) {
        let mut tmp = vec![inf, i];
        tmp.append(&mut vec![0; b_len]);
        m.push(tmp);
    }

    let mut lr = HashMap::new();
    let mut a_chars = a.chars();

    for i in 1..(a_len + 1) {
        let c_a = a_chars.next().unwrap();
        let mut lmc = 0;
        let mut b_chars = b.chars();

        for j in 1..(b_len + 1) {
            let c_b = b_chars.next().unwrap();
            let lmr = *lr.get(&c_b).unwrap_or(&0);
            let cost = if c_b == c_a { 0 } else { 1 };
            let tr_cost = std::cmp::max(i - lmr - 1, j - lmc - 1) + 1;

            m[i + 1][j + 1] = *(&[
                m[i][j] + cost,
                m[i + 1][j] + 1,
                m[i][j + 1] + 1,
                m[lmr][lmc] + tr_cost,
            ]).iter().min().unwrap();

            if cost == 0 {
                lmc = j;
            }
        }

        lr.insert(c_a, i);
    }

    *m.last().unwrap().last().unwrap()
}
