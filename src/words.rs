use rand;
use rand::seq::SliceRandom;

const WORDS: &'static str = "ant baboon badger bat bear beaver camel cat clam cobra cougar
coyote crow deer dog donkey duck eagle ferret fox frog goat
goose hawk lion lizard llama mole monkey moose mouse mule newt
otter owl panda parrot pigeon python rabbit ram rat raven
rhino salmon seal shark sheep skunk sloth snake spider
stork swan tiger toad trout turkey turtle weasel whale wolf
wombat zebra";

fn get_words() -> Vec<&'static str> {
    return WORDS.split_whitespace().collect();
}

pub fn get_random_word() -> String {
    let words = get_words();
    let mut rng = rand::thread_rng();
    let word = words[..].choose(&mut rng);
    word.expect("there should be at least one word in the list")
        .to_string()
}
