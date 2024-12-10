use crate::common::{calculate_checksum, DiskData};

pub fn part2(disk_data: DiskData) -> anyhow::Result<u64> {
    let mut blocks = disk_data.blocks;
    let mut spaces = disk_data.spaces;

    for file in disk_data.files.iter().rev() {
        let space_idx = spaces.iter().position(|space| space.size >= file.size && space.start < file.start);

        if let Some(space_idx) = space_idx {
            let space = &mut spaces[space_idx];

            for block in blocks.iter_mut().skip(space.start).take(file.size) { *block = Some(file.block_id); }
            for block in blocks.iter_mut().skip(file.start).take(file.size) { *block = None }

            if space.size == file.size {
                spaces.remove(space_idx);
            } else {
                space.start += file.size;
                space.size -= file.size;
            }
        }
    }

    Ok(calculate_checksum(&blocks))
}
