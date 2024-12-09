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

fn part_one(segments: &[FileSystemSegment]) -> usize {
    let mut index = 0;

    reallocate(segments)
        .iter()
        .filter_map(|x| x.checksum(&mut index))
        .sum()
}

fn main() {
    let segments = parse_segments(INPUT);

    advent_solution(9, part_one(&segments), "");
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
    fn part_one_final() {
        let segments = parse_segments(INPUT);
        assert_eq!(part_one(&segments), 6356833654075);
    }
}
