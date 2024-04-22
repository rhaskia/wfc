use std::collections::HashMap;
use std::collections::HashSet;
use rand;
use rand::Rng;
use rand::prelude::IteratorRandom;

// hash of words that can be near other
type Possible = HashSet<String>;
type NearWords = HashMap<String, Vec<Possible>>;

const DIR: (i32, i32) = (-1, 2);

fn word_freq(s: String) -> NearWords
{
    let words: Vec<String> = s.split(" ").map(|x| x.to_string()).collect();
    let mut freq: NearWords = HashMap::new();

    // each sentence/message
    for i in 0..words.len()
    {
        let word_values = freq.entry(words[i].clone()).or_insert(vec![HashSet::new(); 3]);

        // runs through each word near each other word
        for x in DIR.0..DIR.1
        {
            //if x == 0 { continue; }
            let vec_index = x + DIR.0.abs();
            let words_index = i as i32 + x;

            // out of index
            if words_index >= words.len() as i32 || words_index < 0 { continue; }

            word_values[vec_index as usize].insert(words[words_index as usize].clone());
        }
    }

    freq
}

fn collapse(length: usize, nw: NearWords) -> Vec<String>
{
    let mut rng = rand::thread_rng();

    let poss = nw.keys().cloned().collect::<HashSet<String>>();
    let mut collapsed = vec![poss.clone(); length];

    // while not collapsed
    loop
    {
        // constraint management :3
        for i in 0..collapsed.len()
        {
            // runs through each word near each other word
            for x in DIR.0..DIR.1
            {
                if x == 0 { continue; }
                let vec_index = x + DIR.0.abs();
                let coll_index = i as i32 + x;

                if coll_index < 0 || coll_index >= collapsed.len() as i32 { continue; }
                if collapsed[coll_index as usize].len() != 1
                    || collapsed[i].len() == 1 { continue; }

                collapsed[i] = collapsed[i].iter().filter(|n| nw[*n][vec_index as usize]
                    .contains(collapsed[coll_index as usize]
                        .iter().next().unwrap())
                ).map(|x| x.clone()).collect::<HashSet<String>>();

                println!("nc {:?}", collapsed[i]);
            }
        }



        // collapse
        let mut smallest = vec![];
        let mut size = usize::MAX;
        for i in 0..collapsed.len()
        {
            if collapsed[i].len() == size
            {
                smallest.push(i);
            }
            else if collapsed[i].len() < size && collapsed[i].len() > 1
            {
                println!("smallest size {}", collapsed[i].len());
                smallest = vec![i];
                size = collapsed[i].len();
            }
        }

        println!("sm {:?}", smallest);

        // kills itself if empty
        if smallest.len() < 2 { break; }
        let index = smallest[rng.gen_range(0..smallest.len())];
        let mut new_set = HashSet::new();

        new_set.insert(collapsed[index].iter().choose(&mut rng).unwrap().clone());
        collapsed[index] = new_set;
        println!("col {:?}", collapsed);
    }

    // returns when all are collapsed ig
    collapsed.iter().map(|x| x.iter()
        .next().unwrap().clone())
        .collect::<Vec<String>>()
}

fn main()
{
    let input = "1 2 3 4 5 4 3 2 3 2 3 2 1 2 3 4 5 6 7 8 7 6 5 4 3 2 1";
    let word_freq = word_freq(input.to_string());
    println!("{:?}", collapse(10, word_freq));
}