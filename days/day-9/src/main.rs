use common::*;

include_input!(INPUT);

#[derive(Debug, Eq, PartialEq, Clone)]
enum FileSystemSegment {
    File { id: usize, size: u8 },
    FreeSpace(u8),
}

impl FileSystemSegment {
    fn from_enumerated_char(value: (usize, char)) -> Self {
        let (index, segment_char) = value;
        let segment_int = segment_char.to_string().parse().unwrap();

        if index % 2 == 0 {
            Self::File {
                id: index / 2,
                size: segment_int,
            }
        } else {
            Self::FreeSpace(segment_int)
        }
    }

    fn checksum(&self, index: &mut usize) -> Option<usize> {
        match self {
            Self::File { id, size } => {
                let checksum = (*index..(*index + *size as usize))
                    .map(|position| position * id)
                    .sum();

                *index += *size as usize;

                Some(checksum)
            }
            Self::FreeSpace(amount) => {
                *index += *amount as usize;
                None
            }
        }
    }

    fn take(&mut self, amount: u8) -> Option<(usize, u8)> {
        match self {
            Self::File { id, size } => {
                let current_size = *size;
                *size = size.saturating_sub(amount);

                Some((*id, current_size - *size))
            }
            Self::FreeSpace(_) => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::File { id: _, size } => *size == 0,
            Self::FreeSpace(amount) => *amount == 0,
        }
    }
}

fn parse_segments(raw: &str) -> Vec<FileSystemSegment> {
    raw.chars()
        .filter(|character| *character != '\n')
        .enumerate()
        .map(FileSystemSegment::from_enumerated_char)
        .collect()
}

fn reallocate(segments: &[FileSystemSegment]) -> Vec<FileSystemSegment> {
    let mut reallocated = Vec::with_capacity(segments.len());
    let mut end_index = segments.len() - 1;
    let mut last_element = segments[end_index].clone();

    'segment: for (i, segment) in segments.iter().enumerate() {
        if end_index == i {
            break;
        }

        match segment {
            FileSystemSegment::File { id: _, size: _ } => reallocated.push(segment.clone()),
            FileSystemSegment::FreeSpace(mut amount) => {
                while amount > 0 {
                    if let Some((id, size)) = last_element.take(amount) {
                        amount -= size;
                        reallocated.push(FileSystemSegment::File { id, size });

                        if last_element.is_empty() {
                            end_index -= 1;
                            last_element = segments[end_index].clone();

                            if end_index == i {
                                break 'segment;
                            }
                        }
                    } else {
                        end_index -= 1;
                        last_element = segments[end_index].clone();

                        if end_index == i {
                            break 'segment;
                        }
                    }
                }
            }
        }
    }

    reallocated.push(last_element);

    reallocated
}

#[allow(unused)]
fn print_seg(segments: &[FileSystemSegment]) {
    let mut output = String::new();

    for segment in segments {
        match segment {
            FileSystemSegment::FreeSpace(amount) => (0..*amount).for_each(|_| output.push('.')),
            FileSystemSegment::File { id, size } => {
                (0..*size).for_each(|_| output.push_str(&id.to_string()))
            }
        }
    }

    println!("{}", output);
}

fn reallocate_strict(mut segments: Vec<FileSystemSegment>) -> Vec<FileSystemSegment> {
    let mut i = segments.len();

    while i > 2 {
        i -= 1;

        if let FileSystemSegment::File { id: _, size } = segments[i] {
            let mut search_index = 1;
            while search_index < i {
                if let FileSystemSegment::FreeSpace(space) = &mut segments[search_index] {
                    if *space < size {
                        search_index += 1;
                        continue;
                    }

                    let space_left = *space - size;

                    if space_left > 0 {
                        *space = size;
                    }

                    segments.swap(search_index, i);

                    if space_left > 0 {
                        segments.insert(search_index + 1, FileSystemSegment::FreeSpace(space_left));
                    }

                    break;
                }

                search_index += 1;
            }
        }
    }

    segments
}

fn process_checksums(segments: &[FileSystemSegment]) -> usize {
    let mut index = 0;

    segments
        .iter()
        .filter_map(|segment| segment.checksum(&mut index))
        .sum()
}

fn part_one(segments: &[FileSystemSegment]) -> usize {
    process_checksums(&reallocate(segments))
}

fn part_two(segments: Vec<FileSystemSegment>) -> usize {
    process_checksums(&reallocate_strict(segments))
}

fn main() {
    let segments = parse_segments(INPUT);

    advent_solution(9, part_one(&segments), part_two(segments));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "2333133121414131402";

    #[test]
    fn segments() {
        let mut segments = parse_segments("123").into_iter();

        assert_eq!(
            segments.next(),
            Some(FileSystemSegment::File { id: 0, size: 1 })
        );

        assert_eq!(segments.next(), Some(FileSystemSegment::FreeSpace(2)));

        assert_eq!(
            segments.next(),
            Some(FileSystemSegment::File { id: 1, size: 3 })
        );

        assert_eq!(segments.next(), None);
    }

    #[test]
    fn example_1() {
        let segments = parse_segments(EXAMPLE_ONE);
        assert_eq!(part_one(&segments), 1928);
    }

    #[test]
    fn example_2() {
        let segments = parse_segments(EXAMPLE_ONE);
        assert_eq!(part_two(segments), 2858);
    }

    #[test]
    fn part_one_final() {
        let segments = parse_segments(INPUT);
        assert_eq!(part_one(&segments), 6356833654075);
    }

    #[test]
    fn part_two_final() {
        let segments = parse_segments(INPUT);
        assert_eq!(part_two(segments), 6389911791746);
    }
}
