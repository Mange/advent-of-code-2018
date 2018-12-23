use pest::Parser;
use std::ops::RangeInclusive;

#[derive(Parser)]
#[grammar = "claim.pest"]
struct ClaimParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Debug)]
pub struct Claim {
    pub id: u64,
    pub position: (usize, usize),
    pub size: (usize, usize),
}

impl Claim {
    pub fn parse(input: &str) -> Result<Claim, String> {
        let mut pairs = ClaimParser::parse(Rule::claim, input)
            .map_err(|e| format!("Failed to parse: {}", e))?
            .next()
            .unwrap()
            .into_inner();

        // id = (uint)
        let id = pairs.next().unwrap();
        assert!(id.as_rule() == Rule::id);

        let position = pairs.next().unwrap();
        assert!(position.as_rule() == Rule::position);

        let size = pairs.next().unwrap();
        assert!(size.as_rule() == Rule::size);

        Ok(Claim {
            id: consume_id(id)?,
            position: consume_usize_pairs(position)?,
            size: consume_usize_pairs(size)?,
        })
    }

    pub fn max_x(&self) -> usize {
        self.position.0 + self.size.0.saturating_sub(1)
    }

    pub fn max_y(&self) -> usize {
        self.position.1 + self.size.1.saturating_sub(1)
    }

    pub fn x_range(&self) -> RangeInclusive<usize> {
        self.position.0..=self.max_x()
    }

    pub fn y_range(&self) -> RangeInclusive<usize> {
        self.position.1..=self.max_y()
    }
}

fn consume_id(id: Pair<'_>) -> Result<u64, String> {
    let uint = id.into_inner().next().unwrap();
    uint.as_str()
        .parse()
        .map_err(|e| format!("Failed to parse as ID: {}", e))
}

fn consume_usize_pairs(pairs: Pair<'_>) -> Result<(usize, usize), String> {
    let mut iter = pairs.into_inner();
    let (a_pair, b_pair) = (iter.next().unwrap(), iter.next().unwrap());

    consume_usize(a_pair).and_then(|a| consume_usize(b_pair).map(|b| (a, b)))
}

fn consume_usize(uint: Pair<'_>) -> Result<usize, String> {
    assert!(uint.as_rule() == Rule::uint);

    uint.as_str()
        .parse()
        .map_err(|e| format!("Failed to parse as unsigned integer: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_claim() {
        let input = "#123 @ 3,1: 4x92";
        let claim = Claim::parse(input).unwrap();

        assert_eq!(claim.id, 123);
        assert_eq!(claim.position, (3, 1));
        assert_eq!(claim.size, (4, 92));
    }

    #[test]
    fn it_has_valid_ranges() {
        let simple = Claim {
            id: 1,
            size: (10, 10),
            position: (0, 0),
        };
        assert_eq!(simple.x_range(), 0..=9);
        assert_eq!(simple.y_range(), 0..=9);

        let empty = Claim {
            id: 1,
            size: (0, 0),
            position: (0, 0),
        };
        assert_eq!(empty.x_range(), 0..=0);
        assert_eq!(empty.y_range(), 0..=0);

        let one_dee = Claim {
            id: 1,
            size: (5, 0),
            position: (2, 7),
        };
        assert_eq!(one_dee.x_range(), 2..=6);
        assert_eq!(one_dee.y_range(), 7..=7);
    }
}
