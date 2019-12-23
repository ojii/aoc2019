use std::ops::Mul;

type Deck = Vec<u16>;
type Technique = Vec<Shuffle>;

enum Shuffle {
    Cut(i16),
    NewStack,
    DealIncrement(usize),
}

impl Shuffle {
    fn apply(&self, deck: Deck) -> Deck {
        match self {
            Shuffle::Cut(n) => {
                let index = if n > &0 {
                    *n as usize
                } else {
                    deck.len() - (n.abs() as usize)
                };
                let length = deck.len();
                deck.into_iter().cycle().skip(index).take(length).collect()
            }
            Shuffle::NewStack => deck.into_iter().rev().collect(),
            Shuffle::DealIncrement(n) => {
                let length = deck.len();
                let mut new = vec![0; length];
                for (index, card) in deck.into_iter().enumerate() {
                    new[(index * *n) % length] = card;
                }

                new
            }
        }
    }
}

impl From<&str> for Shuffle {
    fn from(value: &str) -> Self {
        if value.starts_with("cut ") {
            Shuffle::Cut(value[4..].parse::<i16>().unwrap())
        } else if value.starts_with("deal with") {
            Shuffle::DealIncrement(value[20..].parse::<usize>().unwrap())
        } else {
            Shuffle::NewStack
        }
    }
}

pub fn main() {
    let deck: Deck = (0..10_007).collect();
    let technique: Technique = INPUT.lines().map(|line| line.into()).collect();
    let result = technique.iter().fold(deck, |deck, tech| tech.apply(deck));
    println!("{}", result.iter().position(|&i| i == 2019).unwrap());
}

const INPUT: &str = "deal with increment 55
cut -6791
deal with increment 9
cut -5412
deal with increment 21
deal into new stack
deal with increment 72
cut -362
deal with increment 24
cut -5369
deal with increment 22
cut 731
deal with increment 72
cut 412
deal into new stack
deal with increment 22
cut -5253
deal with increment 73
deal into new stack
cut -6041
deal into new stack
cut 6605
deal with increment 6
cut 9897
deal with increment 59
cut -9855
deal into new stack
cut -7284
deal with increment 7
cut 332
deal with increment 37
deal into new stack
deal with increment 43
deal into new stack
deal with increment 59
cut 1940
deal with increment 16
cut 3464
deal with increment 24
cut -7766
deal with increment 36
cut -156
deal with increment 18
cut 8207
deal with increment 33
cut -393
deal with increment 4
deal into new stack
cut -4002
deal into new stack
cut -8343
deal into new stack
deal with increment 70
deal into new stack
cut 995
deal with increment 22
cut 1267
deal with increment 47
cut -3161
deal into new stack
deal with increment 34
cut -6221
deal with increment 26
cut 4956
deal with increment 57
deal into new stack
cut -4983
deal with increment 36
cut -1101
deal into new stack
deal with increment 2
cut 4225
deal with increment 35
cut -721
deal with increment 17
cut 5866
deal with increment 40
cut -531
deal into new stack
deal with increment 63
cut -5839
deal with increment 30
cut 5812
deal with increment 35
deal into new stack
deal with increment 46
cut -5638
deal with increment 60
deal into new stack
deal with increment 33
cut -4690
deal with increment 7
cut 6264
deal into new stack
cut 8949
deal into new stack
cut -4329
deal with increment 52
cut 3461
deal with increment 47";
