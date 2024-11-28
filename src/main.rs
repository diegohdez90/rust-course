use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        // list of suites - hearts/spades
        let suites = vec!["Hearts", "Spades", "Diamonds"];
        //list of values - ace/two/three
        let values = vec!["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suites {
            for value in &values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck {
            cards
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
}

fn main() {
    let mut deck: Deck = Deck::new();
    println!("Deck: {:#?}", deck);

    deck.shuffle();
    let cards = deck.deal(3);
    println!("Deal cards: {:#?}", cards);
    println!("Deck: {:#?}", deck);

}
