use crate::common::{calculate_checksum, DiskData};

pub fn part1(disk_data: DiskData) -> anyhow::Result<u64> {
    let mut blocks = disk_data.blocks;

    let mut files = disk_data.files;
    files.reverse();
    let mut file_iter = files.iter_mut();

    let mut current_file = file_iter.next().unwrap();

    'block_loop: for block_idx in 0..blocks.len() {
        if let Some(space_block) = blocks.get_mut(block_idx) {
            if space_block.is_none() {

                while current_file.size == 0 || block_idx >= current_file.start {
                    match file_iter.next() {
                        Some(next_file) => { 
                            if next_file.block_id == 0 { break 'block_loop; }
                            current_file = next_file;
                        }
                        None => { break 'block_loop; }
                    }
                } 
                
                *space_block = Some(current_file.block_id);

                if let Some(file_block) = blocks.get_mut(current_file.start) { *file_block = None; }
                current_file.size -= 1;
                current_file.start += 1;
            }
        }
    }

    Ok(calculate_checksum(&blocks))
}
