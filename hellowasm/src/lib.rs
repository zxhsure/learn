use std::{collections::VecDeque, convert::TryInto, vec};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn forFun(len: usize) -> usize {
    let mut total = 0;

    for i in 0..len {
        total += i;
        for j in 0..len {
            total += j;
            for m in 0..len {
                total += m;
                for n in 0..len {
                    total += n;
                }
            }
        }
    }

    total
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Item {
    name: String,
    children: Vec<Item>,
}

#[wasm_bindgen]
pub fn copyFun(len: usize) {
    let mut nodeStack = vec![];
    let tempNode = Item {
        name: String::from(""),
        children: vec![],
    };
    for i in 0..len {
        let mut curItem = tempNode.clone();
        curItem.name = format!("node_{}", i);
        nodeStack.push(curItem);
    }
}

#[wasm_bindgen]
pub fn sliceFun(len: usize) {
    let mut nodeStack: Vec<&mut [usize]> = vec![];
    let mut arr = vec![1; len];
    let mut ss: Vec<&mut [usize]> = arr.chunks_mut(1).collect();
    nodeStack.append(&mut ss);
}

#[wasm_bindgen]
pub fn unsafeSliceFun(len: usize) {
    let mut nodeStack: Vec<&mut usize> = vec![];
    let mut arr = vec![1; len];
    let ptr = arr.as_mut_ptr();
    unsafe {
        for i in 0..len {
            nodeStack.push(&mut *ptr.add(i));
        }
    }
}

#[wasm_bindgen]
pub fn forUnsafeSliceFun(len: usize) {
    let mut nodeStack: Vec<&mut usize> = vec![];
    let mut arr = vec![1; len];
    unsafe {
        for i in 0..len {
            let ptr = arr.as_mut_ptr();
            nodeStack.push(&mut *ptr.add(i));
        }
    }
}

#[wasm_bindgen]
pub fn powFun(len: usize) {
    let num: u32 = 10;
    for i in 0..len {
        num.pow(i.try_into().unwrap());
    }
}

#[wasm_bindgen]
pub fn vecFun(len: usize) {
    let mut nodeStack = vec![];
    for _ in 0..len {
        let tempNode = Item {
            name: String::from(""),
            children: vec![],
        };
        nodeStack.push(tempNode);
    }

    for _ in 0..len {
        nodeStack.remove(0);
    }
}

#[wasm_bindgen]
pub fn vecDeqFun(len: usize) {
    let mut nodeStack: VecDeque<Item> = VecDeque::new();
    for _ in 0..len {
        let item = Item {
            name: String::from(""),
            children: vec![],
        };
        nodeStack.push_back(item);
    }

    for _ in 0..len {
        nodeStack.pop_front();
    }
}

#[wasm_bindgen]
pub fn treeFun(treeLevel: u32, nodeWidth: u32) -> Item {
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

#[wasm_bindgen]
pub fn unsafeTreeFun(treeLevel: u32, nodeWidth: u32) -> Item {
    let tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    let mut root = tempNode.clone();
    root.name = String::from("node_0_0");

    if treeLevel < 2 {
        return root;
    }

    let mut nodeStack: VecDeque<&mut Item> = VecDeque::new();

    for m1 in 0..nodeWidth {
        let mut curItem = tempNode.clone();
        curItem.name = format!("node_{}_{}", 1, m1);
        root.children.push(curItem);
    }

    let ptr1 = root.children.as_mut_ptr();
    unsafe {
        for m2 in 0..nodeWidth {
            nodeStack.push_back(&mut *ptr1.add(m2.try_into().unwrap()));
        }
    }

    for i in 2..treeLevel {
        let curTotal: u32 = nodeWidth.pow(i - 1);

        for j in 0..curTotal {
            let shiftItem = nodeStack.pop_front().unwrap();

            for k1 in 0..nodeWidth {
                let mut curItem = tempNode.clone();
                curItem.name = format!("node_{}_{}", i, nodeWidth * j + k1);
                shiftItem.children.push(curItem);
            }

            let ptr2 = shiftItem.children.as_mut_ptr();
            unsafe {
                for k2 in 0..nodeWidth {
                    nodeStack.push_back(&mut *ptr2.add(k2.try_into().unwrap()));
                }
            }
        }
    }

    root
}
