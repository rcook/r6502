use std::iter::Peekable;

pub fn iter_map_file_lines(s: &str) -> Peekable<impl Iterator<Item = &'_ str>> {
    s.lines()
        .filter_map(|line| {
            let line = line.trim().trim_matches('-');
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .peekable()
}
