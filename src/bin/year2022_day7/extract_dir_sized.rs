use crate::parse_dirs::Dir;

pub(crate) fn extract_sizes(dir: Dir) -> Vec<Dir> {
    let mut dirs: Vec<Dir> = vec![];

    do_stuff(dir, &mut dirs);

    return dirs;
}

fn do_stuff(cwd: Dir, result: &mut Vec<Dir>) {
    todo!()
}