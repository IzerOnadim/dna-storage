use std::collections::HashMap;
type Table = HashMap<&'static str, HashMap<&'static str, (&'static str, &'static str)>>;

pub(crate) struct FSM {
    pub(crate) input_size: usize,
    pub(crate) output_size: usize,
    pub(crate) init_state: &'static str,
    pub(crate) table: Table,
}

pub(crate) struct Path {
    pub(crate) length: usize,
    pub(crate) sequence: String,
    pub(crate) observations: String,
}

impl Path {
    fn extend(&mut self, dist: usize, input: &str, output: &str) {
        self.length += dist;
        self.sequence += input;
        self.observations += output;
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Path {
            length: self.length,
            sequence: self.sequence.clone(),
            observations: self.observations.clone(),
        }
    }
}

type Paths = HashMap<&'static str, Path>;

fn hamming_dist(one: &str, two: &str) -> usize {
    assert!(one.len() == two.len());
    return one.chars().zip(two.chars()).filter(|(a, b)| a != b).count();
}

pub(crate) fn conv(fsm: &FSM, msg: &str) -> String {
    assert!(msg.len() % fsm.input_size == 0);

    let mut state = fsm.init_state;
    let mut result = String::from("");

    for i in (0..msg.len()).step_by(fsm.input_size) {
        let input = &msg[i..i + fsm.input_size];
        let (next, output) = fsm.table[state][input];
        result += output;
        state = next;
    }

    return result;
}

pub(crate) fn viterbi(fsm: &FSM, msg: &str) -> Path {
    assert!(msg.len() % fsm.output_size == 0);

    let start_path = Path {
        length: 0,
        sequence: String::from(""),
        observations: String::from(""),
    };
    let mut paths: Paths = HashMap::from([(fsm.init_state, start_path)]);

    for i in (0..msg.len()).step_by(fsm.output_size) {
        let symbol = &msg[i..i + fsm.output_size];
        let mut extended_paths: Paths = HashMap::new();

        for (tip, path) in paths {
            for (input, (next, output)) in &fsm.table[tip] {
                let dist = hamming_dist(symbol, output);
                let mut extended = path.clone();
                extended.extend(dist, input, output);

                if !extended_paths.contains_key(next)
                    || extended.length < extended_paths[next].length
                {
                    extended_paths.insert(next, extended);
                }
            }
        }

        paths = extended_paths;
    }

    return paths
        .values()
        .reduce(|a, b| if a.length < b.length { a } else { b })
        .unwrap()
        .clone();
}