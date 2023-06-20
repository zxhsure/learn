use std::vec;

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
    root.name = String::from("node_1_1");

    if treeLevel < 2 {
        return root;
    }

    let mut nodeStack: Vec<&mut [Item]> = vec![];

    for m1 in 0..nodeWidth {
        let mut curItem = tempNode.clone();
        curItem.name = format!("node_{}_{}", 2, m1);
        root.children.push(curItem);
    }

    // for m2 in 0..nodeWidth {
    //     let s: &mut [Item] =
    //         &mut root.children[m2.try_into().unwrap()..(m2 + 1).try_into().unwrap()];
    // }

    let (mut a, _) = root.children.split_at_mut(1);

    nodeStack.push(a);

    println!("nodeStack: {:#?}", nodeStack);

    // for i in 3..treeLevel + 1 {
    //     let curTotal: u32 = nodeWidth.pow(i - 1);

    //     for j in 0..curTotal {
    //         let shiftItem = nodeStack.remove(0);

    //         for k1 in 0..nodeWidth {
    //             let mut curItem = tempNode.clone();
    //             curItem.name = format!("node_{}_{}", i, nodeWidth * j + k1);
    //             shiftItem[0].children.push(curItem);
    //         }

    //         for k2 in shiftItem[0].children.split_mut(|num| true) {
    //             nodeStack.push(k2);
    //         }
    //     }
    // }

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

fn main() {
    let root = initTree(5, 5);
    println!("{:#?}", root);
    // test();
}
