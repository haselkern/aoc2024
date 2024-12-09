use std::cmp::Ordering;
use std::collections::VecDeque;

use aoc::*;

const INPUT: &str = include_str!("../../input/09");

fn main() {
    assert_example!(part1, "09-test", 1928);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "09-test", 2858);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut disk_map = parse(input);
    let mut map_index = 0;
    let mut checksum = 0;

    'checksum: while let Some(block) = disk_map.pop_front() {
        match block {
            Block::Free {
                length: mut free_space,
                ..
            } => {
                // Fill free space
                let mut fill = Vec::new();
                while free_space > 0 {
                    let Some(tail) = disk_map.pop_back() else {
                        break 'checksum;
                    };

                    match tail {
                        Block::Free { .. } => (), // Remove free space from the end
                        Block::File(File {
                            id,
                            length,
                            position,
                        }) => {
                            let insert_length = free_space.min(length);
                            free_space -= insert_length;
                            let remaining_length = length - insert_length;
                            fill.push(Block::File(File {
                                position,
                                id,
                                length: insert_length,
                            }));
                            if remaining_length > 0 {
                                disk_map.push_back(Block::File(File {
                                    position,
                                    id,
                                    length: remaining_length,
                                }));
                            }
                        }
                    }
                }
                for block in fill.into_iter().rev() {
                    disk_map.push_front(block);
                }
            }
            Block::File(File { id, length, .. }) => {
                for _ in 0..length {
                    checksum += id * map_index;
                    map_index += 1;
                }
            }
        }
    }

    checksum
}

fn part2(input: &str) -> usize {
    let mut disk = parse(input);
    let mut moved = Vec::new();

    while let Some(block) = disk.pop_back() {
        match block {
            Block::Free { .. } => (), // Empty space can be ignored
            Block::File(File {
                position: file_position,
                id: file_id,
                length: file_length,
            }) => {
                // Find free space to put file
                let mut space = None;
                for (i, &block) in disk.iter().enumerate() {
                    if let Block::Free { position, length } = block {
                        if length >= file_length {
                            space = Some((i, position, length));
                            break;
                        }
                    }
                }

                let Some((space_index, space_position, space_length)) = space else {
                    // No space for the file, cannot move further
                    moved.push(File {
                        id: file_id,
                        position: file_position,
                        length: file_length,
                    });
                    continue;
                };

                // Move file here
                moved.push(File {
                    id: file_id,
                    position: space_position,
                    length: file_length,
                });

                // Adjust remaining space
                match space_length.cmp(&file_length) {
                    Ordering::Equal => {
                        disk.remove(space_index);
                    }
                    Ordering::Greater => {
                        disk[space_index] = Block::Free {
                            position: space_position + file_length,
                            length: space_length - file_length,
                        };
                    }
                    Ordering::Less => panic!("this should not be possible"),
                }
            }
        }
    }

    let mut checksum = 0;
    for file in moved {
        for i in file.position..(file.position + file.length) {
            checksum += i * file.id;
        }
    }
    checksum
}

fn parse(input: &str) -> VecDeque<Block> {
    let mut position = 0;
    let mut result = VecDeque::new();
    for (i, length) in input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
    {
        let block = if i % 2 == 0 {
            Block::File(File {
                position,
                id: i / 2,
                length,
            })
        } else {
            Block::Free { position, length }
        };
        position += length;
        result.push_back(block);
    }
    result
}

#[derive(Copy, Clone)]
struct File {
    position: usize,
    id: usize,
    length: usize,
}

#[derive(Copy, Clone)]
enum Block {
    Free { position: usize, length: usize },
    File(File),
}
