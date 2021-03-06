use segment::Segment;
use std::env;
use std::path::PathBuf;
use theme::Theme;

pub fn segment_cwd(segments: &mut Vec<Segment>, theme: &Theme, cwd_max_depth: u8, cwd_max_dir_size: u8) {
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("error"));
    if let Some(home) = env::home_dir() {
        let mut new_path = None;
        if let Ok(new) = path.strip_prefix(&home) {
            segments.push(Segment::new(theme.home_bg, theme.home_fg, "~"));
            // TODO: When non-lexical lifetimes are a thing, use drop(path) here.
            new_path = Some(new.to_path_buf());
        }
        // Bypass borrow checker kek
        if let Some(new) = new_path {
            path = new;
        }
    }

    let length = path.iter().count();
    let mut depth  = length;
    let mut shortened = false;

    let cwd_max_depth = cwd_max_depth as usize;

    for (i, path) in path.iter().enumerate() {
        let fg = if i == length-1 { theme.cwd_fg } else { theme.path_fg };

        if cwd_max_depth > 0 && (i != 0 || cwd_max_depth == 1) && i != length-1 && depth > cwd_max_depth {
            if !shortened { // First time
                segments.push(Segment::new(theme.path_bg, fg, String::from("…")));
                shortened = true;
            } else {
                depth -= 1;
            }
        } else {
            let mut path = path.to_string_lossy().into_owned();

            let cwd_max_dir_size = cwd_max_dir_size as usize;
            if cwd_max_dir_size > 0 && path.len() > cwd_max_dir_size {
                path = String::from(&path[..cwd_max_dir_size]);
                path.push('…');
            }
            segments.push(Segment::new(theme.path_bg, fg, path));
        }
    }
}
