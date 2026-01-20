use std::collections::{HashMap, HashSet, LinkedList};

use loggit::{debug, info, trace};

use crate::types::key::Key;
use crate::types::track::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movement {
    PerfectMatch,
    EnergyBoost,
    EnergyDrop,
    EnergySwitch,
    MoodBoost,
    MoodDrop,
    EnergyRaise,
    DomKey,
    SubDomKey,
    ToneBoost,
    ToneDrop,
}

#[derive(Debug, Clone)]
pub struct MovementWeights {
    pub perfect_match: i32,
    pub energy_boost: i32,
    pub energy_drop: i32,
    pub energy_switch: i32,
    pub mood_boost: i32,
    pub mood_drop: i32,
    pub energy_raise: i32,
    pub dom_key: i32,
    pub sub_dom_key: i32,
    pub tone_boost: i32,
    pub tone_drop: i32,
}

impl Default for MovementWeights {
    fn default() -> Self {
        Self {
            perfect_match: 35,
            energy_boost: 10,
            energy_drop: 10,
            energy_switch: 10,
            mood_boost: 5,
            mood_drop: 5,
            energy_raise: 5,
            dom_key: 10,
            sub_dom_key: 10,
            tone_boost: 0,
            tone_drop: 0,
        }
    }
}

impl MovementWeights {
    pub fn weight(&self, movement: Movement) -> i32 {
        match movement {
            Movement::PerfectMatch => self.perfect_match,
            Movement::EnergyBoost => self.energy_boost,
            Movement::EnergyDrop => self.energy_drop,
            Movement::EnergySwitch => self.energy_switch,
            Movement::MoodBoost => self.mood_boost,
            Movement::MoodDrop => self.mood_drop,
            Movement::EnergyRaise => self.energy_raise,
            Movement::DomKey => self.dom_key,
            Movement::SubDomKey => self.sub_dom_key,
            Movement::ToneBoost => self.tone_boost,
            Movement::ToneDrop => self.tone_drop,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    start: usize,
    end: usize,
    weight: i32,
}

pub fn melodic_sort(tracks: &[Track]) -> LinkedList<Track> {
    melodic_sort_with_weights(tracks, &MovementWeights::default())
}

pub fn melodic_sort_with_weights(
    tracks: &[Track],
    weights: &MovementWeights,
) -> LinkedList<Track> {
    info!("melodic_sort: tracks={}", tracks.len());
    let pairs = build_pairs(tracks, weights);
    info!("melodic_sort: pairs={}", pairs.len());
    if pairs.is_empty() {
        return LinkedList::new();
    }

    let mut pairs_by_start: HashMap<usize, Vec<Pair>> = HashMap::new();
    let mut pair_weights: HashMap<(usize, usize), i32> = HashMap::new();
    for pair in pairs {
        pairs_by_start.entry(pair.start).or_default().push(pair);
        pair_weights.insert((pair.start, pair.end), pair.weight);
    }

    let mut current_layer: Vec<LinkedList<usize>> = pairs_by_start
        .values()
        .flat_map(|pairs| pairs.iter())
        .map(|pair| {
            let mut list = LinkedList::new();
            list.push_back(pair.start);
            list.push_back(pair.end);
            list
        })
        .collect();
    current_layer = trim_top_lists(current_layer, &pair_weights, 100);
    let mut layer_idx = 0usize;
    info!(
        "melodic_sort: layer={} lists={}",
        layer_idx,
        current_layer.len()
    );

    let mut next_layer =
        extend_layer(layer_idx, &current_layer, &pairs_by_start, &pair_weights, 100);
    while !next_layer.is_empty() {
        info!(
            "melodic_sort: expanded layer {} lists={} -> {}",
            layer_idx,
            current_layer.len(),
            next_layer.len()
        );
        current_layer = next_layer;
        layer_idx += 1;
        next_layer =
            extend_layer(layer_idx, &current_layer, &pairs_by_start, &pair_weights, 100);
    }
    info!(
        "melodic_sort: finished at layer={}, total_lists={}",
        layer_idx,
        current_layer.len()
    );

    let mut best_list: Option<&LinkedList<usize>> = None;
    let mut best_len = 0usize;
    let mut best_score = i32::MIN;
    for list in &current_layer {
        let len = list.len();
        let score = list_score(list, &pair_weights);
        trace!("melodic_sort: list_len={}, score={}", len, score);
        if len > best_len || (len == best_len && score > best_score) {
            best_len = len;
            best_score = score;
            best_list = Some(list);
        }
    }
    info!(
        "melodic_sort: best_list_len={}, best_score={}",
        best_len, best_score
    );

    let mut result = LinkedList::new();
    if let Some(best) = best_list {
        for &index in best {
            result.push_back(tracks[index].clone());
        }
    }

    result
}

fn extend_layer(
    layer_idx: usize,
    layer: &[LinkedList<usize>],
    pairs_by_start: &HashMap<usize, Vec<Pair>>,
    pair_weights: &HashMap<(usize, usize), i32>,
    limit: usize,
) -> Vec<LinkedList<usize>> {
    let mut next_layer = Vec::new();
    let mut seen = HashSet::new();

    debug!(
        "extend_layer: layer={}, input_lists={}",
        layer_idx,
        layer.len()
    );
    for list in layer {
        let Some(&end) = list.back() else { continue };
        let Some(pairs) = pairs_by_start.get(&end) else { continue };

        for pair in pairs {
            if list_contains(list, pair.end) {
                trace!(
                    "extend_layer: layer={}, skip duplicate list_end={} candidate_end={}",
                    layer_idx,
                    end,
                    pair.end
                );
                continue;
            }

            let mut new_list = list.clone();
            new_list.push_back(pair.end);
            let key: Vec<usize> = new_list.iter().copied().collect();
            if seen.insert(key) {
                next_layer.push(new_list);
            }
        }
    }
    let untrimmed_len = next_layer.len();
    let trimmed = trim_top_lists(next_layer, pair_weights, limit);
    debug!(
        "extend_layer: layer={}, output_lists={} trimmed_lists={}",
        layer_idx,
        untrimmed_len,
        trimmed.len()
    );

    trimmed
}

fn list_contains(list: &LinkedList<usize>, target: usize) -> bool {
    list.iter().any(|&value| value == target)
}

fn list_score(list: &LinkedList<usize>, weights: &HashMap<(usize, usize), i32>) -> i32 {
    let mut total = 0;
    let mut iter = list.iter();
    let Some(mut prev) = iter.next() else {
        return 0;
    };

    for next in iter {
        if let Some(weight) = weights.get(&(*prev, *next)) {
            total += weight;
        }
        prev = next;
    }

    total
}

fn trim_top_lists(
    lists: Vec<LinkedList<usize>>,
    weights: &HashMap<(usize, usize), i32>,
    limit: usize,
) -> Vec<LinkedList<usize>> {
    if lists.len() <= limit {
        return lists;
    }

    let mut scored: Vec<(i32, LinkedList<usize>)> = lists
        .into_iter()
        .map(|list| (list_score(&list, weights), list))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.truncate(limit);

    scored.into_iter().map(|(_, list)| list).collect()
}

fn build_pairs(tracks: &[Track], weights: &MovementWeights) -> Vec<Pair> {
    let mut pairs = Vec::new();

    for (i, start) in tracks.iter().enumerate() {
        let Some(start_key) = start.key() else { continue };
        for (j, end) in tracks.iter().enumerate() {
            if i == j {
                continue;
            }
            let Some(end_key) = end.key() else { continue };
            if let Some(movement) = movement_between(start_key, end_key) {
                trace!(
                    "build_pairs: {} -> {} movement={:?}",
                    i,
                    j,
                    movement
                );
                pairs.push(Pair {
                    start: i,
                    end: j,
                    weight: weights.weight(movement),
                });
            }
        }
    }

    pairs
}

fn movement_between(start: &Key, end: &Key) -> Option<Movement> {
    let delta = forward_delta(start.number(), end.number());
    let same_letter = start.letter() == end.letter();

    if same_letter {
        match delta {
            0 => Some(Movement::PerfectMatch),
            1 => Some(Movement::EnergyBoost),
            11 => Some(Movement::EnergyDrop),
            2 => Some(Movement::ToneBoost),
            10 => Some(Movement::ToneDrop),
            7 => Some(Movement::EnergyRaise),
            _ => None,
        }
    } else {
        match delta {
            0 => Some(Movement::EnergySwitch),
            1 => Some(Movement::DomKey),
            11 => Some(Movement::SubDomKey),
            3 => Some(Movement::MoodBoost),
            9 => Some(Movement::MoodDrop),
            _ => None,
        }
    }
}

fn forward_delta(start: u8, end: u8) -> u8 {
    let start = (start - 1) as i16;
    let end = (end - 1) as i16;
    ((end - start + 12) % 12) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::key::KeyLetter;

    #[test]
    fn movement_matches_camelot_rules() {
        let start = Key::new(7, KeyLetter::A).unwrap();
        let end = Key::new(8, KeyLetter::A).unwrap();
        assert_eq!(movement_between(&start, &end), Some(Movement::EnergyBoost));

        let end = Key::new(7, KeyLetter::B).unwrap();
        assert_eq!(movement_between(&start, &end), Some(Movement::EnergySwitch));

        let end = Key::new(4, KeyLetter::B).unwrap();
        assert_eq!(movement_between(&start, &end), Some(Movement::MoodDrop));
    }
}
