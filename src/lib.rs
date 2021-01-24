#![feature(test)]

extern crate test;
use std::convert::TryFrom;

use gamma::graph::{ DefaultGraph };
use gamma::matching::{ greedy, maximum_matching };
use gamma::traversal::{ DepthFirst, Step };

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn matching(b: &mut Bencher) {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5, 6 ],
            vec![ 0, 4, 12 ],
            vec![ 4, 7, 11 ],
            vec![ 6, 8 ],
            vec![ 7, 9 ],
            vec![ 8, 10 ],
            vec![ 9, 11 ],
            vec![ 6, 10, 12 ],
            vec![ 11, 5 ]
        ]).unwrap();

        b.iter(|| {
            let mut pairing = greedy(&graph);

            maximum_matching(&graph, &mut pairing)
        });
    }

    #[bench]
    fn depth_first(b: &mut Bencher) {
        // fluorene
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5, 6 ],
            vec![ 0, 4, 12 ],
            vec![ 4, 7, 11 ],
            vec![ 6, 8 ],
            vec![ 7, 9 ],
            vec![ 8, 10 ],
            vec![ 9, 11 ],
            vec![ 6, 10, 12 ],
            vec![ 11, 5 ]
        ]).unwrap();
        
        let mut edges: Option<Vec<Step>> = None;

        b.iter(|| {
            let traversal = DepthFirst::new(&graph, 0).unwrap();

            edges = Some(traversal.collect::<Vec<_>>());
        });

        assert_eq!(edges.unwrap().len(), 15);
    }
}