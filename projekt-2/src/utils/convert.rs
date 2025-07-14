use std::collections::LinkedList;

type AdjList = LinkedList<LinkedList<(usize, i64)>>;

pub fn create_listgraph(data: Vec<Vec<i64>>) -> AdjList {
    data.into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .filter_map(|(j, w)| if w > 0 { Some((j, w)) } else { None })
                .collect::<LinkedList<(usize, i64)>>()
        })
        .collect::<LinkedList<LinkedList<(usize, i64)>>>()
}
