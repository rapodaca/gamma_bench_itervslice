#![feature(test)]

extern crate test;

#[cfg(test)]
mod iter {
    use std::convert::TryFrom;
    use test::Bencher;
    use iter::graph::{ DefaultGraph };
    use iter::matching::{ greedy, maximum_matching };
    use iter::traversal::{ DepthFirst };

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

            maximum_matching(&graph, &mut pairing);
        });
    }

    #[bench]
    fn dfs(b: &mut Bencher) {
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
            let mut traversal = DepthFirst::new(&graph, 0).unwrap();

            while traversal.next().is_some() {

            }
        });
    }
}