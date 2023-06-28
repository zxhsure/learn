use std::{
    collections::{vec_deque, VecDeque},
    vec,
};

#[derive(Debug, Clone)]
struct Item {
    name: String,
    children: Vec<Item>,
}

fn initTree(treeLevel: u32, nodeWidth: u32) -> Item {
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
            // for s in ss.iter_mut() {
            //     nodeStack.push(*s);
            // }
        }
    }

    root
}

fn test() {
    let mut tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    let mut nlist: Vec<&mut [Item]> = vec![];

    let mut r0 = tempNode.clone();
    r0.name = String::from("node_0");

    let mut n1 = tempNode.clone();
    n1.name = String::from("node_1");

    let mut n2 = tempNode.clone();
    n2.name = String::from("node_2");

    r0.children.push(n1);
    r0.children.push(n2);

    let (mut a, mut b) = r0.children.split_at_mut(1);
    a[0].name = String::from("a");
    b[0].name = String::from("b");
    nlist.push(a);
    nlist.push(b);

    let mut n3 = tempNode.clone();
    n3.name = String::from("node_3");

    let mut n4 = tempNode.clone();
    n4.name = String::from("node_4");

    let mut shiftItem = nlist.remove(0);
    shiftItem[0].children.push(n3);
    shiftItem[0].children.push(n4);

    let (mut c, mut d) = shiftItem[0].children.split_at_mut(1);
    c[0].name = String::from("c");
    d[0].name = String::from("d");
    nlist.push(c);
    nlist.push(d);

    println!("{:#?}", nlist);
}

fn slicefun(len: usize) {
    let mut tempNode = Item {
        name: String::from(""),
        children: vec![],
    };
    let mut nodeStack: Vec<&mut Item> = vec![];
    let mut arr = vec![tempNode; len];

    for i in 0..len {
        arr[i].name = format!("node_{}", i);
    }

    let p = arr.as_mut_ptr();

    arr[len + 1].name = String::from("value");

    unsafe {
        // nodeStack.push(&mut *p.add(0));
        // nodeStack.push(&mut *p.add(1));
        // nodeStack[0].name = String::from("hello");
        (&mut *p.add(len + 1)).name = String::from("d");
    }

    println!("{:#?},{:#?}", nodeStack, arr);
}

fn testmove() {
    let mut tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    {
        let mut a = tempNode;
        tempNode = a;
    }

    let d = &mut tempNode;

    println!("{:#?}", d);
}

fn unsafeTreeFun(treeLevel: u32, nodeWidth: u32) -> Item {
    let tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    let mut root = tempNode.clone();
    root.name = String::from("node_0_0");

    if treeLevel < 2 {
        return root;
    }

    let mut nodeStack: Vec<&mut Item> = vec![];

    for m1 in 0..nodeWidth {
        let mut curItem = tempNode.clone();
        curItem.name = format!("node_{}_{}", 1, m1);
        root.children.push(curItem);
    }

    let ptr1 = root.children.as_mut_ptr();
    unsafe {
        for m2 in 0..nodeWidth {
            nodeStack.push(&mut *ptr1.add(usize::try_from(m2).unwrap()));
        }
    }

    for i in 2..treeLevel {
        let curTotal: u32 = nodeWidth.pow(i - 1);

        for j in 0..curTotal {
            let shiftItem = nodeStack.remove(0);

            // for k1 in 0..nodeWidth {
            //     let mut curItem = tempNode.clone();
            //     curItem.name = format!("node_{}_{}", i, nodeWidth * j + k1);
            //     shiftItem.children.push(curItem);
            // }

            // let ptr2 = shiftItem.children.as_mut_ptr();
            // unsafe {
            //     for k2 in 0..nodeWidth {
            //         nodeStack.push(&mut *ptr2.add(usize::try_from(k2).unwrap()));
            //     }
            // }
        }
    }

    root
}

fn main() {
    // let root = unsafeTreeFun(5, 2);
    // println!("{:#?}", root);
    // test();
    // slicefun(5);
}
