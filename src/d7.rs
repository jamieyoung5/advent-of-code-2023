#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut card_counts = [0; 14];
        let mut max_count = 0;
        let mut second_max_count = 0;
        for &card in cards {
            let count = &mut card_counts[card as usize];
            *count += 1;
            if card == Card::Joker {
                continue;
            }
            if *count > max_count {
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }
        max_count += card_counts[Card::Joker as usize];
        match (max_count, second_max_count) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type, &self.cards).cmp(&(other.hand_type, &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_input(line: &str) -> Hand {
    let mut bytes = line[..5].bytes();
    let mut cards = [Card::Joker; 5];
    cards.fill_with(|| match bytes.next().unwrap() {
        b'A' => Card::Ace,
        b'K' => Card::King,
        b'Q' => Card::Queen,
        b'J' => Card::Jack,
        b'T' => Card::Ten,
        b'9' => Card::Nine,
        b'8' => Card::Eight,
        b'7' => Card::Seven,
        b'6' => Card::Six,
        b'5' => Card::Five,
        b'4' => Card::Four,
        b'3' => Card::Three,
        b'2' => Card::Two,
        b => panic!("invalid byte {b}"),
    });
    let bid = line[6..].parse().unwrap();
    let hand_type = HandType::from_cards(&cards);

    Hand { cards, bid, hand_type }
}


pub fn day7part1(input_lines: Vec<String>) -> u32 {
    let mut hands = input_lines
        .iter()
        .map(|line| parse_input(line))
        .collect::<Vec<_>>();
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

pub fn day7part2(input_lines: Vec<String>) -> u32 {
    let mut hands = input_lines
        .iter()
        .map(|line| parse_input(line))
        .collect::<Vec<_>>();

    for hand in hands.iter_mut() {
        let mut changed_cards = false;
        for card in hand.cards.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
                changed_cards = true;
            }
        }
        if changed_cards {
            hand.hand_type = HandType::from_cards(&hand.cards);
        }
    }
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}


