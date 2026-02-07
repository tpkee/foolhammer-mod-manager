#[macro_export]
macro_rules! join_path {
    ($($seg:expr),+ $(,)?) => {{
        let mut path = std::path::PathBuf::new();
        $(path.push($seg);)+
        path.to_string_lossy().into_owned()
    }};
}

#[macro_export]
macro_rules! resolve_existing_path {
    ($($seg:expr),+ $(,)?) => {{
        let mut path = std::path::PathBuf::new();
        let mut first_iteration = true;
        $(
            let mut p = std::path::Path::new($seg);

            if !first_iteration {
                p = match p.strip_prefix(std::path::MAIN_SEPARATOR.to_string()) {
                    Ok(stripped) => stripped,
                    Err(_) => p,
                };
            } else {
                first_iteration = false;
            }

            path.push(p);
        )+
        path.exists().then_some(path)
    }};
}
