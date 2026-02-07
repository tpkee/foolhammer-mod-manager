#[macro_export]
macro_rules! join_path {
  ($($seg:expr),+ $(,)?) => {{
        let mut path = std::path::PathBuf::new();
        let mut _first_iteration = true; // the underscore is to avoid the unused variable warning which triggers for noreason whatsoever
        $(
            let mut p = std::path::Path::new($seg);

            if !_first_iteration {
                p = match p.strip_prefix(std::path::MAIN_SEPARATOR.to_string()) {
                    Ok(stripped) => stripped,
                    Err(_) => p,
                };
            } else {
                _first_iteration = false;
            }

            path.push(p);
        )+
        path
    }};
}

#[macro_export]
macro_rules! resolve_existing_path {
    ($($seg:expr),+ $(,)?) => {{
        let joined_path = join_path!($($seg),+);
        joined_path.exists().then_some(joined_path)
    }};
}
