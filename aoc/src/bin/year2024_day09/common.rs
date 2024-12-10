use separator::usize;

#[derive(Debug)]
pub struct File {
    pub block_id: u64,
    pub start: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct Space {
    pub start: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct DiskData {
    pub blocks: Vec<Option<u64>>,
    pub files: Vec<File>,
    pub spaces: Vec<Space>,
}

impl DiskData {
    fn new() -> Self {
        Self { 
            blocks: vec![], 
            files: vec![],
            spaces: vec![]
        }
    }
}

pub fn text_to_diskmap(text: &str) -> DiskData {
    text.chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
        .fold((DiskData::new(), 0_usize), |(mut data, current_position), (idx, digit)|{
            let size: usize = digit as usize;

            let mut map_data = if idx % 2 == 0 {
                let block_id: u64 = (idx/2) as u64;
                data.files.push(File {
                    block_id,
                    start: current_position,
                    size
                });
                vec![Some(block_id); size]
            } else {
                data.spaces.push(Space {
                    start: current_position,
                    size
                });
                vec![None; size]
            };

            data.blocks.append(&mut map_data);

            (data, current_position + size)
        }).0
} 

pub fn calculate_checksum(blocks: &[Option<u64>]) -> u64 {
    blocks.iter()
        .enumerate()
        .map(|(index, block)| match block {
            Some(block_id) => (index as u64) * block_id,
            None => 0
        })
    .sum()
}

