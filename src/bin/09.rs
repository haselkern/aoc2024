use std::collections::VecDeque;

use aoc::*;

const INPUT: &str = include_str!("../../input/09");

fn main() {
    assert_example!(part1, "09-test", 0);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "09-test", 2858);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(_input: &str) -> usize {
    0 // TODO Paste solution from other machine…
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
                for i in 0..disk.len() {
                    if let Block::Free { position, length } = disk[i] {
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
                if space_length == file_length {
                    disk.remove(space_index);
                } else if space_length > file_length {
                    disk[space_index] = Block::Free {
                        position: space_position + file_length,
                        length: space_length - file_length,
                    }
                } else {
                    panic!("this should not be possible");
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
