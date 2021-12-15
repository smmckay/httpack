use std::path::{Path, PathBuf};

use log::{debug, info};

struct FileTree<'a> {
    base: &'a Path,
    files: Vec<PathBuf>
}

#[repr(C)]
struct PackHeader {
    magic: u64,
    version: u64,
    file_count: u64,
    padding: [u8; 40]
}

const HEADER_MAGIC: u64 = 0x13243546ACBDCEDF;
const HEADER_VER: u64 = 1;

impl PackHeader {
    fn new(file_count: u64) -> Self {
        PackHeader {
            magic: HEADER_MAGIC,
            version: HEADER_VER,
            file_count,
            padding: [0; 40]
        }
    }

    fn is_valid(&self) -> bool {
        self.magic == HEADER_MAGIC && self.version == HEADER_VER && self.padding == [0; 40]
    }
}

#[repr(C)]
struct PackEntry {
    uncmp: Extent<u64>,
    gzip: Extent<u64>,
    brotli: Extent<u64>,
    name: Extent<u16>,
    padding: [u8; 6]
}

struct Extent<T> {
    offset: u64,
    len: T
}

pub fn create_pack(input_path: &Path, output_path: &Path) -> Result<(), std::io::Error> {
    let mut tree = FileTree {
        base: input_path,
        files: Vec::new()
    };
    traverse_dir_tree(Path::new(""), &mut tree)?;

    Ok(())
}

fn traverse_dir_tree(path_from_base: &Path, tree: &mut FileTree) -> Result<(), std::io::Error> {
    let current_dir = tree.base.join(path_from_base);
    debug!("Traversing {}", current_dir.display());

    for entry in current_dir.read_dir()? {
        let entry = entry?;
        let ftype = entry.file_type()?;
        let fpath = path_from_base.join(entry.file_name());
        if ftype.is_dir() {
            traverse_dir_tree(&fpath, tree)?;
        } else {
            debug!("Found {}", fpath.display());
            tree.files.push(fpath);
        }
    }
    Ok(())
}