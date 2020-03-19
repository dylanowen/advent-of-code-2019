use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use core::mem;
use env_logger::Env;
use modinverse::modinverse;

static DEAL_INTO_NEW_STACK: &str = "deal into new stack";
static CUT: &str = "cut ";
static DEAL_WITH_INCREMENT: &str = "deal with increment ";

struct TwentyTwo {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Technique {
    DealToNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl Technique {
    fn shuffle(&self, deck: &mut Vec<usize>) {
        match self {
            Technique::DealToNewStack => deck.reverse(),
            &Technique::Cut(index) => {
                let split_index = if index > 0 {
                    index as usize
                } else {
                    (deck.len() as isize + index) as usize
                };
                let mut other = deck.split_off(split_index);
                mem::swap(deck, &mut other);

                deck.append(&mut other);
            }
            &Technique::DealWithIncrement(step) => {
                let mut old = vec![0; deck.len()];
                mem::swap(deck, &mut old);

                let mut j = 0;
                for &card in &old {
                    deck[j] = card;

                    j += step;
                    if j >= old.len() {
                        j -= old.len();
                    }
                }
            }
        }
    }

    fn mod_shuffle(&self, card: u128, deck_size: u128) -> i128 {
        match self {
            Technique::DealToNewStack => -1 - card as i128,
            &Technique::Cut(index) => index + card as i128,
            &Technique::DealWithIncrement(step) => {
                (card
                    * modinverse(step, deck_size)
                        .expect("The problem probably always works for modinverse"))
                    as i128
            }
        }
    }
}

impl Problem for TwentyTwo {
    type Input = Vec<Technique>;
    type Extra = usize;

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|row| row.trim())
            .map(|row| {
                if row.find(DEAL_INTO_NEW_STACK) == Some(0) {
                    Technique::DealToNewStack
                } else if row.find(CUT) == Some(0) {
                    Technique::Cut(
                        row[CUT.len()..]
                            .parse()
                            .expect("Cut should have a number after"),
                    )
                } else if row.find(DEAL_WITH_INCREMENT) == Some(0) {
                    Technique::DealWithIncrement(
                        row[DEAL_WITH_INCREMENT.len()..]
                            .parse()
                            .expect("Deal With Increment should have a number after"),
                    )
                } else {
                    panic!("Invalid Technique: {}", row)
                }
            })
            .collect()
    }

    fn part_1(techniques: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut deck: Vec<usize> = (0..state.extra).collect();

        for technique in techniques {
            technique.shuffle(&mut deck);
        }

        if !state.is_example {
            Some(
                deck.iter()
                    .position(|&card| card == 2019)
                    .expect("We should have card 2019")
                    .to_string(),
            )
        } else {
            Some(format!("{:?}", deck))
        }
    }

    fn part_2(techniques: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let deck_length = 119_315_717_514_047u128;
        let shuffle_amount = 101_741_582_076_661u128;

        for technique in techniques {
            technique.shuffle(&mut deck);
        }
    }

    fn problem_number() -> usize {
        22
    }
}

//fn shuffle_specific

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(TwentyTwo;
        RunFor::Part1, 10, r#"deal with increment 7
deal into new stack
deal into new stack"#,
        RunFor::Part1, 10, r#"cut 6
deal with increment 7
deal into new stack"#,
        RunFor::Part1, 10, r#"deal with increment 7
deal with increment 9
cut -2"#,
        RunFor::Part1, 10, r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#
    );

    run::<TwentyTwo>(10007, include_str!("22_input.txt"));
}

#[cfg(test)]
mod twenty_two {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<TwentyTwo>(include_str!("22_input.txt"), 10007, "5755", "53201602");
    }
}
