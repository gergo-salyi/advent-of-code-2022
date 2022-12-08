use atoi::FromRadix10;
use bumpalo::{collections::Vec, Bump};

const INPUT: &[u8] = include_bytes!("../res/input07deep");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 1743217
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) // 8319096
}

/*
#[derive(Debug)]
struct Dir<'a> {
    name: &'a [u8],
    children: Vec<Entry<'a>>
}
#[derive(Debug)]
struct File<'a> {
    name: &'a [u8],
    size: u64
}
#[derive(Debug)]
enum Entry<'a> {
    Dir(Dir<'a>),
    File(File<'a>)
}
impl Entry<'_> {
    fn enter(&mut self, name: &[u8]) -> &mut Self {
        match self {
            Entry::File(_) => panic!(),
            Entry::Dir(dir) => {
                dir.children.iter_mut().find(predicate)
            }
        }
        self
    }
}
*/

#[derive(Debug)]
enum FsEntry<'a> {
    Dir((&'a [u8], Vec<'a, FsEntry<'a>>)),
    File((&'a [u8], u64)),
}
impl FsEntry<'_> {
    /*
    fn size(&self) -> u64 {
        match self {
            FsEntry::File((_, fsize)) => return *fsize,
            FsEntry::Dir((_, subdirs)) => {
                subdirs.iter().map(|subdir| subdir.size()).sum()
            }
        }
    }
    */
    fn enter(&mut self, target_dir: &[u8]) -> Option<&mut Self> {
        match self {
            FsEntry::File(_) => panic!(),
            FsEntry::Dir((_, subdirs)) => {
                subdirs.iter_mut().find(|subdir| match subdir {
                    FsEntry::File(_) => false,
                    FsEntry::Dir((name, _)) => *name == target_dir,
                })
            }
        }
    }
    fn write_sizes(&self, output: &mut Vec<u64>) -> u64 {
        match self {
            FsEntry::File((_, fsize)) => *fsize,

            FsEntry::Dir((_, children)) => {
                let children_sizes = children
                    .iter()
                    .map(|child| child.write_sizes(output))
                    .sum();

                output.push(children_sizes);

                children_sizes
            }
        }
    }
}

pub fn run1(input: &[u8]) -> u64 {
    let bump = Bump::new();

    let mut root = FsEntry::Dir((b"/", Vec::new_in(&bump)));

    let mut cwd = &mut root;

    let mut cwd_path: Vec<&[u8]> = Vec::new_in(&bump);

    let mut total_lines = 0usize;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };
        let mut words = line.split(|&b| b == b' ');

        if line[0] == b'$' {
            // cmd
            assert!(words.next().unwrap() == b"$");
            let cmd = words.next().unwrap();
            if cmd == b"cd" {
                let arg = words.next().unwrap();
                if arg[0] == b'/' {
                    cwd = &mut root;
                    cwd_path.clear();
                } else if arg.starts_with(b"..") {
                    cwd_path.pop().unwrap();

                    cwd = &mut root;

                    for path_component in cwd_path.iter() {
                        cwd = cwd.enter(path_component).unwrap();
                    }
                } else {
                    cwd = cwd.enter(arg).unwrap();

                    cwd_path.push(arg);
                }
            } else {
                // $ ls
                // IGNORE
            }
        } else if line[0] < 64 {
            // file
            // starting w/ number
            let size_decimal = words.next().unwrap();
            let size = u64::from_radix_10(size_decimal).0;
            let file_name = words.next().unwrap();

            let FsEntry::Dir((_, cwd)) = cwd else { panic!() };
            cwd.push(FsEntry::File((file_name, size)));
        } else {
            // dir
            assert!(words.next().unwrap() == b"dir");
            let dir_name = words.next().unwrap();

            // add dir to cwd
            let FsEntry::Dir((_, cwd)) = cwd else { panic!() };
            cwd.push(FsEntry::Dir((
                dir_name,
                Vec::with_capacity_in(16, &bump)
            )));
        }
        total_lines += 1;
    }

    let mut sizes: Vec<u64> = Vec::with_capacity_in(total_lines, &bump);
    root.write_sizes(&mut sizes);

    sizes.iter().filter(|&&size| size <= 100_000).sum()
}

pub fn run2(input: &[u8]) -> u64 {
    let bump = Bump::new();

    let mut root = FsEntry::Dir((b"/", Vec::new_in(&bump)));

    let mut cwd = &mut root;

    let mut cwd_path: Vec<&[u8]> = Vec::new_in(&bump);

    let mut total_lines = 0usize;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            break;
        };
        let mut words = line.split(|&b| b == b' ');

        if line[0] == b'$' {
            // cmd
            assert!(words.next().unwrap() == b"$");
            let cmd = words.next().unwrap();
            if cmd == b"cd" {
                let arg = words.next().unwrap();
                if arg[0] == b'/' {
                    cwd = &mut root;
                    cwd_path.clear();
                } else if arg.starts_with(b"..") {
                    cwd_path.pop().unwrap();

                    cwd = &mut root;

                    for path_component in cwd_path.iter() {
                        cwd = cwd.enter(path_component).unwrap();
                    }
                } else {
                    cwd = cwd.enter(arg).unwrap();

                    cwd_path.push(arg);
                }
            } else {
                // $ ls
                // IGNORE
            }
        } else if line[0] < 64 {
            // file
            // starting w/ number
            let size_decimal = words.next().unwrap();
            let size = u64::from_radix_10(size_decimal).0;
            let file_name = words.next().unwrap();

            let FsEntry::Dir((_, cwd)) = cwd else { panic!() };
            cwd.push(FsEntry::File((file_name, size)));
        } else {
            // dir
            assert!(words.next().unwrap() == b"dir");
            let dir_name = words.next().unwrap();

            // add dir to cwd
            let FsEntry::Dir((_, cwd)) = cwd else { panic!() };
            cwd.push(FsEntry::Dir((
                dir_name,
                Vec::with_capacity_in(16, &bump)
            )));
        }
        total_lines += 1;
    }

    let mut sizes: Vec<u64> = Vec::with_capacity_in(total_lines, &bump);
    let root_size = root.write_sizes(&mut sizes);

    let total = 70000000;
    let needed = 30000000;
    let free = total - root_size;
    let need_to_be_freed = needed - free;

    *sizes
        .iter()
        .filter(|&&dir| dir > need_to_be_freed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example07");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 95437)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 24933642)
    }
}
