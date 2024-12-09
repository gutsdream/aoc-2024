use crate::DiskFragment::{FileFragment, SpaceFragment};
use itertools::Either::{Left, Right};
use itertools::Itertools;
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FileId {
    value: usize,
}

#[derive(Debug, Clone)]
enum DiskFragment {
    FileFragment(FileId),
    SpaceFragment,
}

#[derive(Debug, Clone)]
struct File {
    file_id: FileId,
    size: usize,
}

#[derive(Debug, Clone)]
struct DiskSpace {
    files: Vec<File>,
    capacity: usize,
}

impl DiskSpace {
    fn fragments(&self) -> Vec<DiskFragment> {
        let mut fragments = self.files.iter().map(|x| x.fragments()).collect::<Vec<_>>();
        let space_fragments = repeat(self.capacity)
            .take(self.capacity)
            .map(|_| SpaceFragment)
            .collect::<Vec<_>>();

        fragments.push(space_fragments);

        fragments.into_iter().flatten().collect()
    }

    fn new(size: usize) -> DiskSpace {
        DiskSpace {
            capacity: size,
            files: Vec::new(),
        }
    }

    fn transfer(&mut self, other: &mut DiskSpace, file_id: &FileId, size: usize) {
        self.capacity -= size;
        self.files.push(other.drain(&file_id, size));
    }

    fn drain(&mut self, file_id: &FileId, size: usize) -> File {
        self.capacity += size;
        let file = self
            .files
            .iter_mut()
            .next()
            .unwrap();

        let new_size = file.size - size;
        file.size = new_size;

        File {
            file_id: file_id.clone(),
            size,
        }
    }
}

impl File {
    fn chunk(&mut self, size: usize) -> File {
        let new_size = self.size - size;

        self.size = new_size;

        File {
            file_id: self.file_id.clone(),
            size,
        }
    }

    fn fragments(&self) -> Vec<DiskFragment> {
        repeat(self.size)
            .take(self.size)
            .map(|_| FileFragment(self.file_id.clone()))
            .collect::<Vec<_>>()
    }
}

pub struct Puzzle {
    disk_space: Vec<DiskSpace>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut disk_map: Vec<DiskSpace> = input
            .chars()
            .enumerate()
            .filter_map(|(i, x)| {
                let x: usize = x.to_digit(10)? as usize;
                match i % 2 == 0 {
                    true => Some(DiskSpace {
                        files: vec![File {
                            file_id: FileId { value: i / 2 },
                            size: x,
                        }],
                        capacity: 0,
                    }),
                    false => Some(DiskSpace {
                        files: vec![],
                        capacity: x,
                    }),
                }
            })
            .collect();

        Ok(Puzzle {
            disk_space: disk_map,
        })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u64 {
        let fragmented_disk_space = Self::fragment(&self.disk_space);
        let ordered_fragments = self.order_fragments(fragmented_disk_space);

        Self::checksum(&ordered_fragments)
    }

    pub fn part_2(&self) -> u64 {
        let ordered_disk_items = self.order_preserved();

        let fragments = Self::fragment(&ordered_disk_items);

        Self::checksum(&fragments)
    }

    fn order_fragments(&self, fragments: Vec<DiskFragment>) -> Vec<DiskFragment> {
        let mut ordered_fragments = fragments.clone();
        let (files, space_indexes): (Vec<(usize, FileId)>, Vec<(usize)>) = fragments
            .into_iter()
            .enumerate()
            .partition_map(|(i, x)| match x {
                FileFragment(file) => Left((i, file)),
                SpaceFragment => Right(i),
            });

        let mut space_indexes_iter = space_indexes.into_iter();

        _ = files
            .into_iter()
            .rev()
            .try_for_each(|(file_index, file_id)| {
                if let Some(space_index) = space_indexes_iter.next() {
                    return match space_index < file_index {
                        true => {
                            ordered_fragments.swap(space_index, file_index);
                            Ok(())
                        }
                        false => Err(()),
                    }
                }

                Err(())
            });

        ordered_fragments
    }

    fn order_preserved(&self) -> Vec<DiskSpace> {
        let (mut files, mut spaces): (Vec<(usize, DiskSpace)>, Vec<(usize, DiskSpace)>) = self
            .partition_disk_space();

        files.iter_mut().rev().for_each(|(file_index, file_space)| {
            let file = { file_space.files.iter_mut().next().unwrap().clone() };
            if let Some(target_space) = spaces
                .iter_mut()
                .filter_map(|(i, space)| match i < file_index {
                    true => match space.capacity >= file.size {
                        true => Some(space),
                        false => None,
                    },
                    false => None,
                })
                .next()
            {
                target_space.transfer(file_space, &file.file_id, file.size)
            }
        });

        Self::join_disk_space(files, spaces)
    }

    fn partition_disk_space(&self) -> (Vec<(usize, DiskSpace)>, Vec<(usize, DiskSpace)>) {
        self.disk_space
            .clone()
            .into_iter()
            .enumerate()
            .partition_map(|(i, x)| match x.files.len() > 0 {
                true => Left((i, x)),
                false => Right((i, x)),
            })
    }

    fn join_disk_space(
        files: Vec<(usize, DiskSpace)>,
        spaces: Vec<(usize, DiskSpace)>,
    ) -> Vec<DiskSpace> {
        let mut ordered: Vec<(usize, DiskSpace)> = vec![files, spaces]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        ordered.sort_by(|(a_index, _), (b_index, _)| a_index.cmp(&b_index));

        ordered.into_iter().map(|(_, item)| item).collect()
    }

    fn fragment(disk_items: &Vec<DiskSpace>) -> Vec<DiskFragment> {
        disk_items
            .clone()
            .into_iter()
            .map(|x| x.fragments())
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
