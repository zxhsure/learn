use std::vec;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    children: Vec<Item>,
}

// fn initTree(treeLevel: u32, nodeWidth: u32) -> Item {
//     let tempNode = Item {
//         name: String::from(""),
//         children: vec![],
//     };

//     let mut root = tempNode.clone();
//     root.name = String::from("node_0_0");

//     if treeLevel < 2 {
//         return root;
//     }

//     let mut nodeStack = vec![];
//     nodeStack.push(&mut root);

//     for i in 1..treeLevel + 1 {
//         let curTotal: u32 = nodeWidth.pow(i);

//         for j in 0..curTotal {
//             let shiftItem = nodeStack.remove(0);

//             for k in 0..nodeWidth {
//                 let mut curItem = tempNode.clone();
//                 curItem.name = format!("node_{}_{}", treeLevel, nodeWidth * j + k);
//                 shiftItem.children.push(curItem);
//                 nodeStack.push(shiftItem.children.split_last_mut().unwrap().0);
//             }
//         }
//     }

//     root
// }

fn test() {
    let mut tempNode = Item {
        name: String::from(""),
        children: vec![],
    };

    let mut nlist = vec![];

    let mut n1 = tempNode.clone();
    n1.name = String::from("node_1");
    nlist.push(n1);

    let mut n2 = tempNode.clone();
    n2.name = String::from("node_2");
    nlist.push(n2);

    let (mut a, mut b) = nlist.split_at_mut(1);
    a[0].name = String::from("a");
    b[0].name = String::from("b");

    let rlist = vec![a, b];
    println!("{:#?}", rlist)
}

fn main() {
    // let root = initTree(5, 5);
    // println!("{:#?}", root);
    test();
}
