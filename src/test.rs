#[cfg(test)]
mod test {
    use outputs::get;
    use zsh::map;

    use crate::*;
    use std::{collections::HashMap, fs::File, path::Path};

    fn open_file(path: &str) -> File {
        match File::open(Path::new(&path)) {
            Err(err) => std::panic!("Couldn't open {}: {}", path, err),
            Ok(file) => return file,
        }
    }

    #[test]
    fn test_collector_build_map() {
        assert_eq!(
            map(open_file("tests/collector-build-map")),
            HashMap::from([
                (String::from("ll"), 3),
                (String::from("systemctl"), 4),
                (String::from("grep"), 2),
                (String::from("free"), 1),
                (String::from("Hyprland"), 1),
                (String::from("mpv"), 1),
                (String::from("eval"), 1),
                (String::from("cd"), 1),
                (String::from("ssh"), 1),
            ])
        );
    }

    // tests if skip section can infact skip past utf-16 chars
    #[test]
    fn test_collector_build_map_utf_16() {
        assert_eq!(
            map(open_file("tests/map-utf-16")),
            HashMap::from([(String::from("mpv"), 7 as usize)])
        );
    }

    #[test]
    fn test_top() {
        assert_eq!(
            vec![
                (String::from("systemctl"), 4 as usize),
                (String::from("ll"), 3 as usize),
            ],
            top(map(open_file("tests/collector-build-map")), 2)
        )
    }

    #[test]
    /// Verifies that non-valid keys do not crash the program
    fn test_get() {
        assert_eq!(
            (String::from("foobar"), 0 as usize),
            get(
                map(open_file("tests/collector-build-map")),
                String::from("foobar")
            )
        )
    }
}
