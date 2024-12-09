use crate::DiskFragment::{FileFragment, SpaceFragment};
use crate::DiskItem::{File, Space};
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct FileId {
    value: usize,
}

#[derive(Debug, Copy, Clone)]
enum DiskFragment {
    FileFragment(FileId),
    SpaceFragment,
}

#[derive(Debug, Copy, Clone)]
enum DiskItem {
    File { file_id: FileId, size: usize },
    Space { size: usize },
}

pub struct Puzzle {
    disk_items: Vec<DiskItem>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut disk_map: Vec<DiskItem> = input
            .chars()
            .enumerate()
            .filter_map(|(i, x)| {
                let x: usize = x.to_digit(10)? as usize;
                match i % 2 == 0 {
                    true => Some(File {
                        file_id: FileId { value: i / 2 },
                        size: x,
                    }),
                    false => Some(Space { size: x }),
                }
            })
            .collect();

        Ok(Puzzle {
            disk_items: disk_map,
        })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u64 {
        let ordered_disk_fragments = self.order_disk_fragments();

        Self::checksum(&ordered_disk_fragments)
    }

    pub fn part_2(&self) -> u64 {
        let ordered_disk_items = self.order_disk_items();

        let fragments = Self::fragment(&ordered_disk_items);

        Self::checksum(&fragments)
    }

    fn order_disk_fragments(&self) -> Vec<DiskFragment> {
        let fragment_disk_map: Vec<DiskFragment> = Self::fragment(&self.disk_items);

        let mut ordered_disk_map: Vec<DiskFragment> = fragment_disk_map.clone();
        let mut file_id_iter = fragment_disk_map
            .iter()
            .enumerate()
            .filter(|(_, x)| match x {
                FileFragment(_) => true,
                SpaceFragment => false,
            })
            .rev();

        _ = fragment_disk_map
            .iter()
            .enumerate()
            .filter(|(_, x)| match x {
                FileFragment(_) => false,
                SpaceFragment => true,
            })
            .try_for_each(|(space_index, _)| match file_id_iter.next() {
                Some((file_index, _)) => match space_index < file_index {
                    true => {
                        ordered_disk_map.swap(space_index, file_index);
                        Ok(())
                    }
                    false => Err(()),
                },
                _ => Err(()),
            });

        ordered_disk_map
    }

    fn order_disk_items(&self) -> Vec<DiskItem> {
        let mut ordered_disk_items: Vec<DiskItem> = self.disk_items.clone();

        _ = self
            .disk_items
            .iter()
            .rev()
            .filter_map(|x| match x {
                File { size, file_id } => Some( (size, file_id) ),
                Space { size } => None,
            })
            .for_each(|(file_size, id)| {
                let (file_index, _)=  ordered_disk_items.iter().enumerate().find(|(_,x)| match x{
                    File { file_id, .. } => {id.value == file_id.value},
                    Space { .. } => {false}
                }).unwrap();

                if let Some((space_index, space_size)) = ordered_disk_items
                    .clone()
                    .iter()
                    .enumerate()
                    .filter_map(|(space_index, x)| match space_index < file_index {
                        true => {match x {
                            File { .. } => None,
                            Space { size, .. } => match size >= file_size {
                                true => {Some((space_index, size))}
                                false => {None}
                            },
                        }}
                        false => {None}
                    })
                    .next()
                {
                    match space_size > file_size{
                        true => {
                            let space_left = space_size - file_size;

                            let space_taken = Space{size: file_size.clone() };
                            let remaining_space = Space{size: space_left };

                            let file = ordered_disk_items.remove(file_index);
                            ordered_disk_items.insert(file_index, space_taken);

                            let _ = ordered_disk_items.remove(space_index);
                            ordered_disk_items.insert(space_index, file);
                            ordered_disk_items.insert(space_index + 1, remaining_space);
                        }
                        false => {
                            ordered_disk_items.swap(space_index, file_index);
                        }
                    }
                }
            });

        ordered_disk_items
    }

    fn fragment(disk_items : &Vec<DiskItem>) -> Vec<DiskFragment> {
        disk_items
            .clone()
            .into_iter()
            .map(|x| match x {
                File { file_id, size } => repeat(size)
                    .take(size)
                    .map(|_| FileFragment(file_id))
                    .collect::<Vec<_>>(),
                Space { size } => repeat(size)
                    .take(size)
                    .map(|_| SpaceFragment)
                    .collect::<Vec<_>>(),
            })
            .flatten()
            .collect()
    }

    fn checksum(disk_map: &Vec<DiskFragment>) -> u64 {
        disk_map
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                FileFragment(id) => Some((i, id)),
                SpaceFragment => None,
            })
            .map(|(i, id)| (i * id.value) as u64)
            .fold(0, |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(1928, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(2858, sum);
    }
}
