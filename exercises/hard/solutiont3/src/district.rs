use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn find_parent(parent: &mut Vec<i32>, idx: usize) -> i32 {
    if parent[idx] != idx as i32 {
        parent[idx] = find_parent(parent, parent[idx] as usize);
    }
    parent[idx]
}

fn union_node(parent: &mut Vec<i32>, l: i32, r: i32, count: &mut u32) {
    let l_parent = find_parent(parent, l as usize);
    let r_parent = find_parent(parent, r as usize);

    if l_parent != r_parent {
        parent[l_parent as usize] = r_parent;
        *count -= 1;
    }
}

fn count_components(graph: Vec<(i32, i32)>, n: i32) -> u32 {
    // [ (0, 1), (2, 3) ]
    let mut parent: Vec<i32> = vec![0; n as usize];
    let mut count: u32 = n as u32;
    // init
    for (idx, elem) in parent.iter_mut().enumerate() {
        *elem = idx as i32;
    }

    // union
    for (from, to) in graph {
        union_node(&mut parent, from, to, &mut count);
    }

    count
}

fn read_json() -> String {
    // 打开文件
    let file = File::open("./district.json").expect("open file error");
    let reader = BufReader::new(file);

    // 解析 JSON
    let tmp = std::io::read_to_string(reader).unwrap();
    let tmp: Vec<&str> = tmp.split(&['{', '}'][..]).collect();
    let mut raw_data: Vec<&str> = Vec::new();
    for t in tmp.iter() {
        if t.find('[').is_some() {
            raw_data.push(&t.trim());
        }
    }

    let mut ans: String = String::new();

    for t in raw_data {
        let edges: Vec<&str> = t.split('\n').collect();
        let mut str2idx: HashMap<&str, i32> = HashMap::new();
        let mut idx = 0;

        /*
        "青浦": ["嘉定", "青浦"],
        "杭州": ["金华", "温州", "温州", "温州"]

        str2idx: {"青浦": 0, "金华": 3, "温州": 4, "杭州": 2, "嘉定": 1}

        0 -> [1, 0]
        2 -> [3, 4, 4, 4]

        graph:
        [ (0, 1), (2, 3), (2, 4), (2, 4), (2, 4) ]
        */

        let mut graph: Vec<(i32, i32)> = Vec::new();
        for edge in edges {
            let nodes: Vec<&str> = edge.split(&['"', ':', '[', ']', ' ', ','][..]).collect();
            let mut from = -1; // record if first nonnull node
            for node in nodes {
                if node.len() > 0 { // nonnull node is real node
                    if !str2idx.contains_key(node) { // edit id
                        str2idx.insert(node, idx);
                        idx += 1;
                    }
                    if from == -1 {
                        from = str2idx[node];
                    } else {
                        let to = str2idx[node];
                        if from != to {
                            graph.push((from, to));
                        }
                    }
                }
            }
        }
        let count = count_components(graph, idx).to_string();
        ans.push_str(&count);
        ans.push_str(",");
    }
    ans.pop();

    ans
}

pub fn count_provinces() -> String {
    read_json()
}
