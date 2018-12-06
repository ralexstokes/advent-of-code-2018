#[cfg(test)]
mod tests {

    use regex::Regex;

    use load_and_parse_input;

    #[derive(Default, Debug, PartialEq)]
    struct Claim {
        id: u64,
        left_offset: u64,
        top_offset: u64,
        width: u64,
        height: u64,
    }

    impl Claim {
        fn max_extent(&self) -> (u64, u64) {
            (self.left_offset + self.width, self.top_offset + self.height)
        }

        fn horizontal_extent(&self) -> std::ops::Range<u64> {
            self.left_offset..self.left_offset + self.width
        }

        fn vertical_extent(&self) -> std::ops::Range<u64> {
            self.top_offset..self.top_offset + self.height
        }

        fn extent(&self) -> impl IntoIterator<Item = (u64, u64)> {
            let mut coordinates = vec![];
            for row in self.vertical_extent() {
                for col in self.horizontal_extent() {
                    coordinates.push((row, col));
                }
            }
            coordinates
        }
    }

    #[derive(Debug)]
    struct Board<'a> {
        // 2d array of every Claim `id` that touches the given coordinate
        inner: Vec<Vec<Vec<&'a Claim>>>,
    }

    impl<'a> Board<'a> {
        fn new(width: u64, height: u64) -> Self {
            Self {
                inner: vec![vec![vec![]; height as usize]; width as usize],
            }
        }

        fn add(&mut self, claim: &'a Claim) {
            claim
                .extent()
                .into_iter()
                .for_each(|(i, j)| self.inner[i as usize][j as usize].push(claim))
        }

        fn find_total_conflicts(&self) -> usize {
            self.inner.iter().fold(0, |total, row| {
                total + row.iter().fold(
                    0,
                    |total, col| {
                        if col.len() > 1 {
                            total + 1
                        } else {
                            total
                        }
                    },
                )
            })
        }

        // has_no_conflicts searches for contending ids in the extent of `claim`
        fn has_no_conflicts(&self, claim: &Claim) -> bool {
            claim.extent().into_iter().all(|(i, j)| {
                let claims = &self.inner[i as usize][j as usize];
                claims.len() == 1 && claims[0] == claim
            })
        }

        fn find_non_overlapping_claims(&self) -> Vec<&'a Claim> {
            let mut claims: Vec<&Claim> = vec![];

            for row in &self.inner {
                for col in row {
                    match col.len() {
                        1 => {
                            let claim = col[0];
                            if self.has_no_conflicts(claim) && !claims.contains(&claim) {
                                claims.push(claim);
                                // remove the next line if you want to find them all...
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            claims
        }
    }

    #[test]
    fn can_yield_extent() {
        let claim = &Claim {
            id: 0,
            left_offset: 1,
            top_offset: 1,
            width: 3,
            height: 3,
        };

        assert_eq!(claim.extent().into_iter().count(), 9)
    }

    fn parse_entry(s: String) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let captures = RE.captures(&s).unwrap();
        let id = &captures[1];
        let left_offset = &captures[2];
        let top_offset = &captures[3];
        let width = &captures[4];
        let height = &captures[5];

        Claim {
            id: str::parse(id).unwrap(),
            left_offset: str::parse(left_offset).unwrap(),
            top_offset: str::parse(top_offset).unwrap(),
            width: str::parse(width).unwrap(),
            height: str::parse(height).unwrap(),
        }
    }

    #[test]
    fn can_total_overlapping_claims() {
        let filename = "input/day_3.txt";
        let inputs = load_and_parse_input(filename, parse_entry).unwrap();
        let maximum_extents: (Vec<_>, Vec<_>) =
            inputs.iter().map(|claim| claim.max_extent()).unzip();
        let max_horizontal = maximum_extents.0.iter().max().unwrap();
        let max_vertical = maximum_extents.1.iter().max().unwrap();
        let board = Board::new(*max_horizontal + 1, *max_vertical + 1);
        let board = inputs.iter().fold(board, |mut board, claim| {
            board.add(claim);
            board
        });
        let contentious_square_inches = board.find_total_conflicts();
        assert_eq!(contentious_square_inches, 118539)
    }

    #[test]
    fn can_find_nonoverlapping_claims() {
        let filename = "input/day_3.txt";
        let inputs = load_and_parse_input(filename, parse_entry).unwrap();
        let maximum_extents: (Vec<_>, Vec<_>) =
            inputs.iter().map(|claim| claim.max_extent()).unzip();
        let max_horizontal = maximum_extents.0.iter().max().unwrap();
        let max_vertical = maximum_extents.1.iter().max().unwrap();
        let board = Board::new(*max_horizontal + 1, *max_vertical + 1);
        let board = inputs.iter().fold(board, |mut board, claim| {
            board.add(claim);
            board
        });
        if let Some(non_overlapping_claim) = board.find_non_overlapping_claims().first() {
            assert_eq!(non_overlapping_claim.id, 1270);
        } else {
            assert!(false);
        }
    }
}
