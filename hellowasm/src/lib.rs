use std::vec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Item {
    name: String,
    children: Vec<Item>,
}

#[wasm_bindgen]
pub fn initTree(treeLevel: u32, nodeWidth: u32) -> Item {
    let tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    let mut root = tempNode.clone();
    root.name = String::from("node_0_0");

    if treeLevel < 2 {
        return root;
    }

    let mut nodeStack: Vec<&mut [Item]> = vec![];

    for m1 in 0..nodeWidth {
        let mut curItem = tempNode.clone();
        curItem.name = format!("node_{}_{}", 1, m1);
        root.children.push(curItem);
    }

    let mut vs: Vec<&mut [Item]> = root.children.chunks_mut(1).collect();
    for m2 in vs.iter_mut() {
        nodeStack.push(*m2);
    }

    for i in 2..treeLevel {
        let curTotal: u32 = nodeWidth.pow(i - 1);

        for j in 0..curTotal {
            let shiftItem = nodeStack.remove(0);

            for k1 in 0..nodeWidth {
                let mut curItem = tempNode.clone();
                curItem.name = format!("node_{}_{}", i, nodeWidth * j + k1);
                shiftItem[0].children.push(curItem);
            }

            let mut ss: Vec<&mut [Item]> = shiftItem[0].children.chunks_mut(1).collect();
            for _ in 0..nodeWidth {
                let curSs = ss.remove(0);
                nodeStack.push(curSs);
            }
        }
    }

    root
}
