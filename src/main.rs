use rand::prelude::*;
use rand_distr::WeightedAliasIndex;
use std::collections::HashMap;

fn main() {}
type Stat = HashMap<String, HashMap<String, usize>>;

fn compute_stat(text: &str) -> Stat {
  let mut stat = Stat::default();
  let words = text.split(' ').collect::<Vec<_>>();
  for words in words.windows(2) {
    let w1 = words.get(0);
    let w2 = words.get(1);
    let (Some(w1), Some(w2)) = (w1, w2) else {
      continue;
    };

    let w1 = w1.to_string();
    let w2 = w2.to_string();

    let entry = stat.entry(w1).or_default();
    *entry.entry(w2).or_insert(0) += 1;
    //*occurence += 1;
  }
  stat
}

fn generate_next_word(stat: &Stat, word: &str) -> Option<String> {
  let words = stat.get(word)?;
  let mut rng = rand::thread_rng();
  let weights = words.values().into_iter().cloned().collect::<Vec<_>>();
  let dist = WeightedAliasIndex::new(weights);

  todo!()
}

// let choices = vec!['a', 'b', 'c'];
// let weights = vec![2, 1, 1];
// let dist = WeightedAliasIndex::new(weights).unwrap();
// let mut rng = thread_rng();
// for _ in 0..100 {
//     // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
//     println!("{}", choices[dist.sample(&mut rng)]);
// }

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_compute_stat_from_a_sentence() {
    let sentence = "les hommes libres peuvent rester libres ou bien vendre leur liberté";
    let stat = compute_stat(sentence);
    let expected = HashMap::from([
      ("les".to_string(), HashMap::from([("hommes".to_string(), 1)])),
      ("hommes".to_string(), HashMap::from([("libres".to_string(), 1)])),
      ("libres".to_string(), HashMap::from([("ou".to_string(), 1), ("peuvent".to_string(), 1)])),
      ("rester".to_string(), HashMap::from([("libres".to_string(), 1)])),
      ("vendre".to_string(), HashMap::from([("leur".to_string(), 1)])),
      ("peuvent".to_string(), HashMap::from([("rester".to_string(), 1)])),
      ("bien".to_string(), HashMap::from([("vendre".to_string(), 1)])),
      ("ou".to_string(), HashMap::from([("bien".to_string(), 1)])),
      ("leur".to_string(), HashMap::from([("liberté".to_string(), 1)])),
      ("rester".to_string(), HashMap::from([("libres".to_string(), 1)])),
    ]);
    assert_eq!(stat, expected);
  }
  #[test]
  fn test_generate_les_hommes() {
    let sentence = "les hommes";
    let stat = compute_stat(sentence);
    let word = "les";
    let expected = "hommes".to_string();
    let result = generate_next_word(&stat, word);
    assert_eq!(Some(expected), result)
  }

  #[test]
  fn test_les_hommes() {
    let sentence = "les hommes";
    let stat = compute_stat(sentence);
    let expected = [("les".to_string(), [("hommes".to_string(), 1)].into_iter().collect::<HashMap<_, _>>())]
      .into_iter()
      .collect::<HashMap<_, _>>();
    assert_eq!(expected, stat)
  }

  #[test]
  fn test_les_hommes_les_hommes() {
    let sentence = "les hommes les hommes";
    let stat = compute_stat(sentence);
    let expected = [
      ("les".to_string(), [("hommes".to_string(), 2)].into_iter().collect::<HashMap<_, _>>()),
      ("hommes".to_string(), [("les".to_string(), 1)].into_iter().collect::<HashMap<_, _>>()),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    assert_eq!(expected, stat)
  }

  #[test]
  fn test_les_hommes_les() {
    let sentence = "les hommes les";
    let stat = compute_stat(sentence);
    let expected = [
      ("les".to_string(), [("hommes".to_string(), 1)].into_iter().collect::<HashMap<_, _>>()),
      ("hommes".to_string(), [("les".to_string(), 1)].into_iter().collect::<HashMap<_, _>>()),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    assert_eq!(expected, stat)
  }
}
