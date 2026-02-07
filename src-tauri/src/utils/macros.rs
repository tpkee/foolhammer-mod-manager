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
        let mut i = 0;
        $(
            let mut p = std::path::Path::new($seg);

            if i > 0 {
                p = match p.strip_prefix("/") {
                    Ok(stripped) => stripped,
                    Err(_) => p,
                };
            }

            path.push(p);
            i += 1;
        )+
        println!("Resolved path: {:?}", path);
        path.exists().then_some(path)
    }};
}
