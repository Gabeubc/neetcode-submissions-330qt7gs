use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let mut t_freqs = HashMap::new();
        for c in t.chars() {
            *t_freqs.entry(c).or_insert(0) += 1;
        }
        let mut s_freqs = HashMap::new();
        let mut l = 0;
        let mut have = 0;
        let need = t_freqs.len();
        let mut min_len = usize::MAX;
        let mut res_range = (0, 0);

        for r in 0..s_chars.len() {
            let r_char = s_chars[r];
            if let Some(&target_count) = t_freqs.get(&r_char) {
                let count = s_freqs.entry(r_char).or_insert(0);
                *count += 1;
                if *count == target_count {
                    have += 1;
                }
            }

            while have == need {
                let window_len = r - l + 1;
                if window_len < min_len {
                    min_len = window_len;
                    res_range = (l, r);
                }

                let l_char = s_chars[l];
                if let Some(&target_count) = t_freqs.get(&l_char) {
                    let count = s_freqs.get_mut(&l_char).unwrap();
                    if *count == target_count {
                        have -= 1;
                    }
                    *count -= 1;
                }
                l += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            s_chars[res_range.0..=res_range.1].iter().collect()
        }
    }
}
