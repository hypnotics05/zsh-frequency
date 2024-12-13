#[cfg(test)]
mod test {
    use zsh::gen_hash_map;

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
        let map = HashMap::from([
            (String::from("ll"), 3),
            (String::from("systemctl"), 4),
            (String::from("grep"), 2),
            (String::from("free"), 1),
            (String::from("Hyprland"), 1),
            (String::from("mpv"), 1),
            (String::from("eval"), 1),
            (String::from("cd"), 1),
        ]);
        assert_eq!(gen_hash_map(open_file("tests/collector-build-map")), map);
    }

    // tests if skip section can infact skip past utf-16 chars
    #[test]
    fn test_collector_build_map_utf_16() {
        let map = HashMap::from([(String::from("mpv"), 7)]);
        assert_eq!(gen_hash_map(open_file("tests/map-utf-16")), map);
    }

    // TODO: Finish test

    //#[test]
    //fn test_bot() {
    //    let vec = bot(gen_hash_map(open_file("tests/collector-build-map")), 3);
    //}
}
