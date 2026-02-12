pub trait SortMods<T> {
    fn sort_mods<F>(&mut self, key_fn: F)
    where
        F: FnMut(&T) -> &str;
}

impl<T> SortMods<T> for Vec<T> {
    fn sort_mods<F>(&mut self, mut key_fn: F)
    where
        F: FnMut(&T) -> &str,
    {
        self.sort_by(|a, b| compare_mod_names(key_fn(a), key_fn(b)));
    }
}

pub fn compare_mod_names(a: &str, b: &str) -> std::cmp::Ordering {
    let a_letters = a.to_lowercase().encode_utf16().collect::<Vec<u16>>();
    let b_letters = b.to_lowercase().encode_utf16().collect::<Vec<u16>>();

    let a_len = a_letters.len();
    let b_len = b_letters.len();
    let total_len = std::cmp::max(a_len, b_len);

    for i in 0..total_len {
        if i == a_len {
            return std::cmp::Ordering::Greater;
        }

        if i == b_len {
            return std::cmp::Ordering::Less;
        }

        match a_letters[i].cmp(&b_letters[i]) {
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
        }
    }

    std::cmp::Ordering::Equal
}
