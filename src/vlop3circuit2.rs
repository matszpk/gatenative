use gatesim::*;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::vbinopcircuit::*;
use crate::VNegs::{self, *};

use crate::vlop3circuit::*;

impl<T> VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from_lop3nodes(circuit: VBinOpCircuit<T>, lop3nodes: Vec<LOP3Node<T>>) -> VLOP3Circuit<T> {
        Self {
            input_len: T::default(),
            gates: vec![],
            outputs: vec![],
        }
    }
}

fn get_small_tree<T>(circuit: &VBinOpCircuit<T>, wire_index: T) -> [Option<T>; 7]
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gates = &circuit.gates;
    let mut tree = [None; 7];
    let mut old_level_start = 0;
    let mut level_start = 1;
    tree[0] = Some(wire_index);
    for level in 1..3 {
        for pos in 0..level_start - old_level_start {
            if let Some(t) = tree[old_level_start + pos] {
                if t >= input_len_t {
                    let gi = usize::try_from(t).unwrap();
                    let g = gates[gi - input_len].0;
                    tree[level_start + (pos << 1)] = Some(g.i0);
                    tree[level_start + (pos << 1) + 1] = Some(g.i1);
                }
            }
        }
        old_level_start = level_start;
        level_start += level_start + 1;
    }
    tree
}

// special area of MTUsubtree that used to join with other MTUblocks.
#[derive(Clone, Debug)]
struct MTUArea<T> {
    root: T,
    nodes: Vec<(T, Vec<T>)>,
    cost: usize,
}

impl<T: Default> Default for MTUArea<T> {
    #[inline]
    fn default() -> Self {
        Self {
            root: T::default(),
            nodes: vec![],
            cost: 0,
        }
    }
}

const LOP3_TREE_SHAPE_TABLE: [u8; 19] = [
    0b0000001, // (R)
    0b0000110, // (C0,C1)
    0b0000111, // (R,C0,C1)
    0b0011100, // (C1,C00,C01)
    0b0011101, // (R,C1,C00,C01)
    0b0011110, // (C0,C1,C00,C01)
    0b0011111, // (R,C0,C1,C00,C01)
    0b1100010, // (C0,C10,C11)
    0b1100011, // (R,C0,C10,C11)
    0b1100110, // (C0,C1,C10,C11)
    0b1100111, // (R,C0,C1,C10,C11)
    0b1111000, // (C00,C01,C10,C11)
    0b1111001, // (R,C00,C01,C10,C11)
    0b1111010, // (C0,C00,C01,C10,C11)
    0b1111011, // (R,C0,C00,C01,C10,C11)
    0b1111100, // (C1,C00,C01,C10,C11)
    0b1111101, // (R,C1,C00,C01,C10,C11)
    0b1111110, // (C0,C1,C00,C01,C10,C11)
    0b1111111, // (R,C0,C1,C00,C01,C10,C11)
];

//
#[derive(Clone, Copy)]
struct MTUAreaConfig(u8);
// ^^--- node_mask, farest nodes

// cost calculation: number of nodes to connect with farest nodes + extra nodes,
//                   reversed minimal depth of nodes (depth=0->3,depth=1->2)
fn calc_mtu_area_config_cost(idx: u8, cfg: MTUAreaConfig) -> usize {
    let all_nodes = cfg.0 | idx;
    let mut cost = 0;
    // connection with other nodes
    if (all_nodes & 0b0000001) != 0 && (all_nodes & 0b1111110) != 0 {
        cost += 1;
    }
    if (all_nodes & 0b0000010) != 0 && (all_nodes & 0b0011000) != 0 {
        cost += 1;
    }
    if (all_nodes & 0b0000100) != 0 && (all_nodes & 0b1100000) != 0 {
        cost += 1;
    }
    // reversed min depth
    if (all_nodes & 0b0000001) != 0 {
        cost += 3;
    } else if (all_nodes & 0b0000110) != 0 {
        cost += 2;
    } else if (all_nodes & 0b1111000) != 0 {
        cost += 1;
    }
    // add number of extra nodes
    cost + cfg.0.count_ones() as usize
}

fn check_c1_node(idx: u8, cfg: MTUAreaConfig) -> bool {
    let all_nodes = cfg.0 | idx;
    (all_nodes & 0b1111001) == 0b1111001 && (all_nodes & 0b0000110) == 0
}

fn farest_nonfarest_nodes_from_mask(nodes: u8) -> (u8, u8) {
    let mut farest = 0;
    let mut nonfarest = 0;
    if (nodes & 0b0000001) != 0 {
        if ((nodes & 0b0000010) != 0 || (nodes & 0b0011000) == 0b0011000)
            && ((nodes & 0b0000100) != 0 || (nodes & 0b1100000) == 0b1100000)
        {
            nonfarest |= 0b0000001;
        } else {
            farest |= 0b0000001;
        }
    }
    if (nodes & 0b0000010) != 0 {
        if (nodes & 0b0011000) == 0b0011000 {
            nonfarest |= 0b0000010;
        } else {
            farest |= 0b0000010;
        }
    }
    if (nodes & 0b0000100) != 0 {
        if (nodes & 0b1100000) == 0b1100000 {
            nonfarest |= 0b0000100;
        } else {
            farest |= 0b0000100;
        }
    }
    farest |= nodes & 0b1111000;
    (farest, nonfarest)
}

const MTUAREA_CONFIG_TBL: [MTUAreaConfig; 128] = [
    MTUAreaConfig(0b0000000), // 0b0000000: ()
    MTUAreaConfig(0b0000000), // 0b0000001: (R)
    MTUAreaConfig(0b0000000), // 0b0000010: (C0)
    MTUAreaConfig(0b0000100), // 0b0000011: (R,C0,*C1)
    MTUAreaConfig(0b0000000), // 0b0000100: (C1)
    MTUAreaConfig(0b0000010), // 0b0000101: (R,*C0,C1)
    MTUAreaConfig(0b0000000), // 0b0000110: (C0,C1)
    MTUAreaConfig(0b0000000), // 0b0000111: (R,C0,C1)
    MTUAreaConfig(0b0000000), // 0b0001000: (C00)
    MTUAreaConfig(0b0010100), // 0b0001001: (R,*C1,C00,*C01)
    MTUAreaConfig(0b0010000), // 0b0001010: (C0,C00,*C01)
    MTUAreaConfig(0b0010100), // 0b0001011: (R,C0,*C1,C00,*C01)
    MTUAreaConfig(0b0000000), // 0b0001100: (C1,C00)
    MTUAreaConfig(0b0010000), // 0b0001101: (R,C1,C00,*C01)
    MTUAreaConfig(0b0010000), // 0b0001110: (C0,C1,C00,*C01)
    MTUAreaConfig(0b0010000), // 0b0001111: (R,C0,C1,C00,*C01)
    MTUAreaConfig(0b0000000), // 0b0010000: (C01)
    MTUAreaConfig(0b0001100), // 0b0010001: (R,*C1,*C00,C01)
    MTUAreaConfig(0b0001000), // 0b0010010: (C0,*C00,C01)
    MTUAreaConfig(0b0001100), // 0b0010011: (R,C0,*C1,*C00,C01)
    MTUAreaConfig(0b0000000), // 0b0010100: (C1,C01)
    MTUAreaConfig(0b0001000), // 0b0010101: (R,C1,*C00,C01)
    MTUAreaConfig(0b0001000), // 0b0010110: (C0,C1,*C00,C01)
    MTUAreaConfig(0b0001000), // 0b0010111: (R,C0,C1,*C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011000: (C00,C01)
    MTUAreaConfig(0b0000100), // 0b0011001: (R,*C1,C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011010: (C0,C00,C01)
    MTUAreaConfig(0b0000100), // 0b0011011: (R,C0,*C1,C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011100: (C1,C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011101: (R,C1,C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011110: (C0,C1,C00,C01)
    MTUAreaConfig(0b0000000), // 0b0011111: (R,C0,C1,C00,C01)
    //
    MTUAreaConfig(0b0000000), // 0b0100000: (C10)
    MTUAreaConfig(0b1000010), // 0b0100001: (R,*C0,C10,*C11)
    MTUAreaConfig(0b0000000), // 0b0100010: (C0,C10)
    MTUAreaConfig(0b1000000), // 0b0100011: (R,C0,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0100100: (C1,C10,*C11)
    MTUAreaConfig(0b1000010), // 0b0100101: (R,*C0,C1,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0100110: (C0,C1,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0100111: (R,C0,C1,C10,*C11)
    MTUAreaConfig(0b0000000), // 0b0101000: (C00,C10)
    MTUAreaConfig(0b1010000), // 0b0101001: (R,C00,*C01,C10,*C11)
    MTUAreaConfig(0b0010000), // 0b0101010: (C0,C00,*C01,C10)
    MTUAreaConfig(0b1010000), // 0b0101011: (R,C0,C00,*C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0101100: (C1,C00,C10,*C11)
    MTUAreaConfig(0b1010000), // 0b0101101: (R,C1,C00,*C01,C10,*C11)
    MTUAreaConfig(0b1010000), // 0b0101110: (C0,C1,C00,*C01,C10,*C11)
    MTUAreaConfig(0b1010000), // 0b0101111: (R,C0,C1,C00,*C01,C10,*C11)
    MTUAreaConfig(0b0000000), // 0b0110000: (C01,C10)
    MTUAreaConfig(0b1001000), // 0b0110001: (R,*C00,C01,C10,*C11)
    MTUAreaConfig(0b0001000), // 0b0110010: (C0,*C00,C01,C10)
    MTUAreaConfig(0b1001000), // 0b0110011: (R,C0,*C00,C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0110100: (C1,C01,C10,*C11)
    MTUAreaConfig(0b1001000), // 0b0110101: (R,C1,*C00,C01,C10,*C11)
    MTUAreaConfig(0b1001000), // 0b0110110: (C0,C1,*C00,C01,C10,*C11)
    MTUAreaConfig(0b1001000), // 0b0110111: (R,C0,C1,*C00,C01,C10,*C11)
    MTUAreaConfig(0b0000000), // 0b0111000: (C00,C01,C10)
    MTUAreaConfig(0b1000000), // 0b0111001: (R,C00,C01,C10,*C11)
    MTUAreaConfig(0b0000000), // 0b0111010: (C0,C00,C01,C10)
    MTUAreaConfig(0b1000000), // 0b0111011: (R,C0,C00,C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0111100: (C1,C00,C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0111101: (R,C1,C00,C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0111110: (C0,C1,C00,C01,C10,*C11)
    MTUAreaConfig(0b1000000), // 0b0111111: (R,C0,C1,C00,C01,C10,*C11)
    //
    MTUAreaConfig(0b0000000), // 0b1000000: (C11)
    MTUAreaConfig(0b0100010), // 0b1000001: (R,*C0,*C10,C11)
    MTUAreaConfig(0b0000000), // 0b1000010: (C0,C11)
    MTUAreaConfig(0b0100000), // 0b1000011: (R,C0,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1000100: (C1,*C10,C11)
    MTUAreaConfig(0b0100010), // 0b1000101: (R,*C0,C1,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1000110: (C0,C1,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1000111: (R,C0,C1,*C10,C11)
    MTUAreaConfig(0b0000000), // 0b1001000: (C00,C11)
    MTUAreaConfig(0b0110000), // 0b1001001: (R,C00,*C01,*C10,C11)
    MTUAreaConfig(0b0010000), // 0b1001010: (C0,C00,*C01,C11)
    MTUAreaConfig(0b0110000), // 0b1001011: (R,C0,C00,*C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1001100: (C1,C00,*C10,C11)
    MTUAreaConfig(0b0110000), // 0b1001101: (R,C1,C00,*C01,*C10,C11)
    MTUAreaConfig(0b0110000), // 0b1001110: (C0,C1,C00,*C01,*C10,C11)
    MTUAreaConfig(0b0110000), // 0b1001111: (R,C0,C1,C00,*C01,*C10,C11)
    MTUAreaConfig(0b0000000), // 0b1010000: (C01,C11)
    MTUAreaConfig(0b0101000), // 0b1010001: (R,*C00,C01,*C10,C11)
    MTUAreaConfig(0b0001000), // 0b1010010: (C0,*C00,C01,C11)
    MTUAreaConfig(0b0101000), // 0b1010011: (R,C0,*C00,C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1010100: (C1,C01,*C10,C11)
    MTUAreaConfig(0b0101000), // 0b1010101: (R,C1,*C00,C01,*C10,C11)
    MTUAreaConfig(0b0101000), // 0b1010110: (C0,C1,*C00,C01,*C10,C11)
    MTUAreaConfig(0b0101000), // 0b1010111: (R,C0,C1,*C00,C01,*C10,C11)
    MTUAreaConfig(0b0000000), // 0b1011000: (C00,C01,C11)
    MTUAreaConfig(0b0100000), // 0b1011001: (R,C00,C01,*C10,C11)
    MTUAreaConfig(0b0000000), // 0b1011010: (C0,C00,C01,C11)
    MTUAreaConfig(0b0100000), // 0b1011011: (R,C0,C00,C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1011100: (C1,C00,C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1011101: (R,C1,C00,C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1011110: (C0,C1,C00,C01,*C10,C11)
    MTUAreaConfig(0b0100000), // 0b1011111: (R,C0,C1,C00,C01,*C10,C11)
    //
    MTUAreaConfig(0b0000000), // 0b1100000: (C10,C11)
    MTUAreaConfig(0b0000010), // 0b1100001: (R,*C0,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1100010: (C0,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1100011: (R,C0,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1100100: (C1,C10,C11)
    MTUAreaConfig(0b0000010), // 0b1100101: (R,*C0,C1,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1100110: (C0,C1,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1100111: (R,C0,C1,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1101000: (C00,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101001: (R,C00,*C01,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101010: (C0,C00,*C01,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101011: (R,C0,C00,*C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1101100: (C1,C00,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101101: (R,C1,C00,*C01,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101110: (C0,C1,C00,*C01,C10,C11)
    MTUAreaConfig(0b0010000), // 0b1101111: (R,C0,C1,C00,*C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1110000: (C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110001: (R,*C00,C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110010: (C0,*C00,C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110011: (R,C0,*C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1110100: (C1,C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110101: (R,C1,*C00,C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110110: (C0,C1,*C00,C01,C10,C11)
    MTUAreaConfig(0b0001000), // 0b1110111: (R,C0,C1,*C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111000: (C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111001: (R,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111010: (C0,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111011: (R,C0,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111100: (C1,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111101: (R,C1,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111110: (C0,C1,C00,C01,C10,C11)
    MTUAreaConfig(0b0000000), // 0b1111111: (R,C0,C1,C00,C01,C10,C11)
];

impl<T> MTUArea<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn empty_with_root(root: T) -> Self {
        Self {
            root,
            nodes: vec![],
            cost: 0,
        }
    }

    fn add_node(&mut self, node: T, touch_node: T) {
        if let Some((_, ts)) = self.nodes.iter_mut().find(|(x, _)| *x == node) {
            if ts.iter().all(|x| *x != touch_node) {
                ts.push(touch_node);
            }
        } else {
            self.nodes.push((node, vec![touch_node]));
        }
    }

    fn sort_and_dedup(&mut self) {
        self.nodes.sort();
        self.nodes.dedup();
    }

    // return cost of MTUarea
    fn gen_lop3nodes(
        &mut self,
        circuit: &VBinOpCircuit<T>,
        lop3nodes: &mut [LOP3Node<T>],
    ) -> usize {
        // IDEA:
        // use form closed form of area by nodes (for example: (R,C00,C01,C10,C11)
        // or (R,C00,C01,C1)) and it can be without root.
        // if some nodes are node supplied then add.
        // NEXT THOUGHT: include (minimal) depth of nodes in MTUarea to calculate costs
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let tree = get_small_tree(circuit, self.root);
        let gates = &circuit.gates;
        let mut node_mask = tree
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                if let Some(t) = t {
                    self.nodes.iter().any(|(x, _)| *x == t)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        let node_mask_u8 = node_mask
            .iter()
            .enumerate()
            .fold(0u8, |a, (i, x)| a | (u8::from(*x) << i));
        let mtuarea_config = MTUAREA_CONFIG_TBL[node_mask_u8 as usize];
        let all_nodes = node_mask_u8 | mtuarea_config.0;
        let cost = calc_mtu_area_config_cost(node_mask_u8, mtuarea_config);
        let check_c1_node = check_c1_node(node_mask_u8, mtuarea_config);
        let do_add_c1_node = if let Some(t) = tree[0] {
            let mut farest = (3..7).map(|x| tree[x].unwrap()).collect::<Vec<_>>();
            farest.sort();
            farest.dedup();
            farest.len() == 4
        } else {
            false
        };
        let all_nodes = all_nodes | (u8::from(do_add_c1_node) << 2);
        let (farest_nodes, nonfarest_nodes) = farest_nonfarest_nodes_from_mask(all_nodes);
        // generate lop3nodes from MTUarea
        for i in 0..7 {
            // initialize only nodes which any input connected to other any node.
            // ^^--- really??? reconsider it!
            if (nonfarest_nodes & (1u8 << i)) != 0 {
                // node to process
                let t = tree[i].unwrap();
                if t >= circuit.input_len {
                    let gidx = usize::try_from(t).unwrap() - input_len;
                    if i == 0 && (all_nodes & 0b0000110) != 0b0000110 {
                        // if root node and node available all nodes higher level
                        // add LOP3node
                        match all_nodes & 0b0000110 {
                            0b0000010 => {
                                let t2_0 = gates[gidx].0.i0;
                                let gidx2_0 = usize::try_from(t2_0).unwrap() - input_len;
                                lop3nodes[gidx] = LOP3Node {
                                    args: [
                                        gates[gidx2_0].0.i0,
                                        gates[gidx2_0].0.i1,
                                        gates[gidx].0.i1,
                                    ],
                                    tree_paths: [
                                        PathMove(3),
                                        PathMove(3),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                    ],
                                    mtu_cost: MTU_COST_BASE + 1,
                                };
                            }
                            0b0000100 => {
                                let t2_1 = gates[gidx].0.i1;
                                let gidx2_1 = usize::try_from(t2_1).unwrap() - input_len;
                                lop3nodes[gidx] = LOP3Node {
                                    args: [
                                        gates[gidx].0.i0,
                                        gates[gidx2_1].0.i0,
                                        gates[gidx2_1].0.i1,
                                    ],
                                    tree_paths: [
                                        PathMove(3),
                                        PathMove(0),
                                        PathMove(3),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                    ],
                                    mtu_cost: MTU_COST_BASE + 1,
                                };
                            }
                            0b0000000 => {
                                let t2_0 = gates[gidx].0.i1;
                                let gidx2_0 = usize::try_from(t2_0).unwrap() - input_len;
                                let t2_1 = gates[gidx].0.i1;
                                let gidx2_1 = usize::try_from(t2_1).unwrap() - input_len;
                                let mut args = vec![
                                    gates[gidx2_0].0.i0,
                                    gates[gidx2_0].0.i1,
                                    gates[gidx2_1].0.i0,
                                    gates[gidx2_1].0.i1,
                                ];
                                args.sort();
                                args.dedup();
                                lop3nodes[gidx] = LOP3Node {
                                    args: [args[0], args[1], args[2]],
                                    tree_paths: [
                                        PathMove(3),
                                        PathMove(3),
                                        PathMove(3),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                        PathMove(0),
                                    ],
                                    mtu_cost: MTU_COST_BASE + 1,
                                };
                            }
                            _ => {
                                panic!("Unexpected!");
                            }
                        }
                        //lop3nodes[gidx] =
                    } else {
                        lop3nodes[gidx] = LOP3Node {
                            args: [gates[gidx].0.i0, gates[gidx].0.i1, gates[gidx].0.i0],
                            tree_paths: [
                                PathMove(3),
                                PathMove(0),
                                PathMove(0),
                                PathMove(0),
                                PathMove(0),
                                PathMove(0),
                                PathMove(0),
                            ],
                            mtu_cost: MTU_COST_BASE + 1,
                        };
                    }
                }
            }
        }
        self.cost = cost;
        cost
    }

    // and improve - fix other TouchNodes to make better result if possible
    fn improve_and_optimize_and_gen_lop3nodes(
        &mut self,
        circuit: &VBinOpCircuit<T>,
        lop3node: &mut [LOP3Node<T>],
    ) {
    }

    fn farest_nonfarest_nodes(&self, circuit: &VBinOpCircuit<T>) -> (Vec<T>, Vec<T>) {
        let tree = get_small_tree(circuit, self.root);
        let node_mask = tree
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                if let Some(t) = t {
                    self.nodes.iter().any(|(x, _)| *x == t)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        let mut farest = vec![];
        let mut nonfarest = vec![];
        if node_mask[0] {
            if let Some(t) = tree[0] {
                if (node_mask[1] || (node_mask[3] && node_mask[4]))
                    && (node_mask[2] || (node_mask[5] && node_mask[6]))
                {
                    nonfarest.push(t);
                } else {
                    farest.push(t);
                }
            }
        }
        if node_mask[1] {
            if let Some(t) = tree[1] {
                if node_mask[3] && node_mask[4] {
                    nonfarest.push(t);
                } else {
                    farest.push(t);
                }
            }
        }
        if node_mask[2] {
            if let Some(t) = tree[2] {
                if node_mask[5] && node_mask[6] {
                    nonfarest.push(t);
                } else {
                    farest.push(t);
                }
            }
        }
        for i in 3..7 {
            if node_mask[i] {
                if let Some(t) = tree[i] {
                    farest.push(t);
                }
            }
        }
        (farest, nonfarest)
    }
}

// instead LOP3Boundary use path penetration form:
// entry: 0 - nothing, 1 - go left, 2 - go right, 3 - go left and right
// and encode in bits to save memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PathMove(u8);

impl PathMove {
    #[inline]
    fn is_first(self) -> bool {
        (self.0 & 1) != 0
    }
    #[inline]
    fn is_second(self) -> bool {
        (self.0 & 2) != 0
    }
    #[inline]
    fn is_way(self, second: bool) -> bool {
        (self.0 & (1 << u32::from(second))) != 0
    }
    #[inline]
    fn go_first(self) -> Self {
        Self(self.0 | 1)
    }
    #[inline]
    fn go_second(self) -> Self {
        Self(self.0 | 2)
    }
    #[inline]
    fn go(self, second: bool) -> Self {
        Self(self.0 | (1 << u32::from(second)))
    }
    #[inline]
    fn undo(self, second: bool) -> Self {
        Self(self.0 & !(1 << u32::from(second)))
    }
}

// tree moves organization:
//       /--------0-------\
//   /---1---\       /----2---\
//   3       4       5        6
// 0 - root, 1 - first level start, 3 - second level start
// leaves are LOP3 arguments, non-zero elements are deepest LOP3 gates.
type LOP3SubTreePaths = [PathMove; 7];

const LOP3_SUBTREE_PATHS_DEFAULT: LOP3SubTreePaths = [PathMove(0); 7];

fn lop3_fill_moves(m: LOP3SubTreePaths) -> LOP3SubTreePaths {
    let mut md = m;
    for i in (1..7).rev() {
        if m[(i - 1) >> 1].is_way(((i - 1) & 1) != 0) {
            md[(i - 1) >> 1] = md[(i - 1) >> 1].go_first();
            md[(i - 1) >> 1] = md[(i - 1) >> 1].go_second();
            md[i] = md[i].go_first();
            md[i] = md[i].go_second();
        }
    }
    md[0] = PathMove(3);
    md
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LOP3Node<T> {
    args: [T; 3],                 // arguments, also leaves of LOP3 subtree
    tree_paths: LOP3SubTreePaths, // LOP3 subtree paths
    mtu_cost: usize,
}

impl<T> Default for LOP3Node<T>
where
    T: Clone + Copy + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            args: [T::default(); 3],
            tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
            mtu_cost: 0,
        }
    }
}

const MTU_COST_BASE: usize = 100;

// register_sol - function to register solution. arguments:
//    leaves - arguments of LOP3 instruction,
//    lop3_moves (tree_paths) - tree paths for choosen LOP3
//    mtu_cost - cost of LOP3 in MTUblock
//    gate_num - number of gates that covered by LOP3
fn find_best_lop3node_generic<T, F>(
    circuit: &VBinOpCircuit<T>,
    lop3nodes: &[LOP3Node<T>],
    coverage: &[T],
    subtrees: &[SubTree<T>],
    circuit_outputs: &HashSet<T>,
    wire_index: T,
    preferred_nodes: &[T],
    mut register_sol: F,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    F: FnMut(&[T], LOP3SubTreePaths, usize, usize),
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gates = &circuit.gates;
    let current_subtree = coverage[usize::try_from(wire_index).unwrap() - input_len];
    let current_mtu = subtrees[usize::try_from(current_subtree).unwrap()].root();
    // generate tree to explore
    let tree = get_small_tree(circuit, wire_index);
    // algorithm: simple batch of combinations with difference
    #[derive(Clone, Copy)]
    enum CombBatchEntry {
        // fields: node to operate, to execute test
        AddNode(u8, bool),
        RemoveNode(u8, bool),
    }
    use CombBatchEntry::*;
    const COMB_BATCH_L1: [CombBatchEntry; 4] = [
        AddNode(0, true),    // (R)
        AddNode(1, true),    // (R,C0)
        AddNode(2, true),    // (R,C0,C1)
        RemoveNode(1, true), // (R,C1)
    ];
    const COMB_BATCH: [CombBatchEntry; 31] = [
        AddNode(0, true), // (R)
        AddNode(1, true), // (R,C0)
        //
        AddNode(3, true),     // (R,C0,C00)
        AddNode(4, true),     // (R,C0,C00,C01)
        RemoveNode(3, true),  // (R,C0,C01)
        RemoveNode(4, false), // (R,C0) *
        //
        AddNode(2, true), // (R,C0,C1)
        //
        AddNode(3, true),     // (R,C0,C1,C00)
        AddNode(4, true),     // (R,C0,C1,C00,C01)
        RemoveNode(3, true),  // (R,C0,C1,C01)
        RemoveNode(4, false), // (R,C0,C1) *
        AddNode(5, true),     // (R,C0,C1,C10)
        AddNode(3, true),     // (R,C0,C1,C00,C10)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C10)
        RemoveNode(3, true),  // (R,C0,C1,C01,C10)
        RemoveNode(4, false), // (R,C0,C1,C10) *
        AddNode(6, true),     // (R,C0,C1,C10,C11)
        AddNode(3, true),     // (R,C0,C1,C00,C10,C11)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C10,C11)
        RemoveNode(3, true),  // (R,C0,C1,C01,C10,C11)
        RemoveNode(4, false), // (R,C0,C1,C10,C11) *
        RemoveNode(5, true),  // (R,C0,C1,C11)
        AddNode(3, true),     // (R,C0,C1,C00,C11)
        AddNode(4, true),     // (R,C0,C1,C00,C01,C11)
        RemoveNode(3, true),  // (R,C0,C1,C01,C11)
        RemoveNode(4, false), // (R,C0,C1,C11) *
        RemoveNode(6, false), // (R,C0,C1) *
        //
        RemoveNode(1, true), // (R,C1)
        //
        AddNode(5, true),    // (R,C1,C10)
        AddNode(6, true),    // (R,C1,C10,C11)
        RemoveNode(5, true), // (R,C1,C11)
    ];
    let mut leaves: Vec<(T, u8)> = vec![(wire_index, 1)];
    let mut moves = LOP3_SUBTREE_PATHS_DEFAULT;
    let mut gate_num = 0;
    let comb_batch =
        if tree[3].is_some() || tree[4].is_some() || tree[5].is_some() || tree[6].is_some() {
            &COMB_BATCH[..]
        } else {
            &COMB_BATCH_L1[..]
        };
    for instr in comb_batch {
        let ex = match *instr {
            AddNode(i, ex) => {
                if let Some(tt) = tree[i as usize] {
                    if tt >= input_len_t {
                        let t = usize::try_from(tt).unwrap();
                        // remove gate from leaves
                        leaves.iter_mut().find(|(x, _)| *x == tt).unwrap().1 -= 1;
                        leaves.retain(|(_, c)| *c != 0);
                        let g = gates[t - input_len].0;
                        let a0 = g.i0;
                        let a1 = g.i1;
                        // add first input (a0) to leaves
                        if let Some(p) = leaves.iter().position(|(x, _)| *x == a0) {
                            leaves[p].1 += 1;
                        } else {
                            leaves.push((a0, 1));
                        }
                        // add second input (a1) to leaves
                        if let Some(p) = leaves.iter().position(|(x, _)| *x == a1) {
                            leaves[p].1 += 1;
                        } else {
                            leaves.push((a1, 1));
                        }
                        if i != 0 {
                            let i = i as usize;
                            // set move that go to this path
                            moves[(i - 1) >> 1] = moves[(i - 1) >> 1].go(((i - 1) & 1) != 0);
                        }
                        gate_num += 1;
                        ex
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            RemoveNode(i, ex) => {
                if let Some(tt) = tree[i as usize] {
                    if tt >= input_len_t {
                        let t = usize::try_from(tt).unwrap();
                        let g = gates[t - input_len].0;
                        let a0 = g.i0;
                        let a1 = g.i1;
                        // remove first input (a0) from leaves
                        leaves.iter_mut().find(|(x, _)| *x == a0).unwrap().1 -= 1;
                        // remove second input (a1) from leaves
                        leaves.iter_mut().find(|(x, _)| *x == a1).unwrap().1 -= 1;
                        leaves.retain(|(_, c)| *c != 0);
                        // add gate to leaves
                        if let Some(p) = leaves.iter().position(|(x, _)| *x == tt) {
                            leaves[p].1 += 1;
                        } else {
                            leaves.push((tt, 1));
                        }
                        if i != 0 {
                            let i = i as usize;
                            // undo move in that path
                            moves[(i - 1) >> 1] = moves[(i - 1) >> 1].undo(((i - 1) & 1) != 0);
                        }
                        gate_num -= 1;
                        ex
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        };
        if ex {
            if leaves.len() <= 3 {
                // calculate costs for node
                let mtu_cost = MTU_COST_BASE
                    + leaves
                        .iter()
                        .map(|(ln, _)| {
                            if *ln >= input_len_t {
                                let l = usize::try_from(*ln).unwrap() - input_len;
                                let coverage_l =
                                    subtrees[usize::try_from(coverage[l]).unwrap()].root();
                                if current_mtu == coverage_l {
                                    lop3nodes[l].mtu_cost
                                } else {
                                    MTU_COST_BASE + 1
                                        - usize::from(
                                            preferred_nodes.iter().any(|x| *x == *ln)
                                                && !circuit_outputs.contains(&coverage_l),
                                        )
                                }
                            } else {
                                MTU_COST_BASE
                            }
                        })
                        .sum::<usize>()
                    - MTU_COST_BASE * leaves.len()
                    + 1;
                // choose if better
                let leaves = leaves.iter().map(|(x, _)| *x).collect::<Vec<_>>();
                register_sol(&leaves, moves, mtu_cost, gate_num);
            }
        }
    }
}

fn find_best_lop3node<T>(
    circuit: &VBinOpCircuit<T>,
    lop3nodes: &[LOP3Node<T>],
    coverage: &[T],
    subtrees: &[SubTree<T>],
    circuit_outputs: &HashSet<T>,
    wire_index: T,
    preferred_nodes: &[T],
) -> LOP3Node<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let mut best_config = None;
    let reg_sol = |leaves: &[T], moves, mtu_cost, gate_num| {
        if let Some((_, _, best_mtu_cost, best_gate_num)) = best_config {
            use std::cmp::Reverse;
            if (mtu_cost, Reverse(gate_num)) < (best_mtu_cost, Reverse(best_gate_num)) {
                best_config = Some((leaves.to_vec(), lop3_fill_moves(moves), mtu_cost, gate_num));
            }
        } else {
            best_config = Some((leaves.to_vec(), lop3_fill_moves(moves), mtu_cost, gate_num));
        }
    };

    find_best_lop3node_generic(
        circuit,
        lop3nodes,
        coverage,
        subtrees,
        circuit_outputs,
        wire_index,
        preferred_nodes,
        reg_sol,
    );

    let best_config = best_config.unwrap();
    let mut args = [best_config.0[0]; 3];
    for (i, t) in best_config.0.iter().enumerate() {
        args[i] = *t;
    }
    LOP3Node {
        args,
        tree_paths: best_config.1,
        mtu_cost: best_config.2,
    }
}

fn find_best_lop3node_variants<T>(
    circuit: &VBinOpCircuit<T>,
    lop3nodes: &[LOP3Node<T>],
    coverage: &[T],
    subtrees: &[SubTree<T>],
    circuit_outputs: &HashSet<T>,
    wire_index: T,
    preferred_nodes: &[T],
    required_args: &[T],
) -> Vec<LOP3Node<T>>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let mut best_configs: Option<(Vec<(Vec<T>, LOP3SubTreePaths)>, usize, usize)> = None;
    let reg_sol = |leaves: &[T], moves, mtu_cost: usize, gate_num: usize| {
        if required_args
            .iter()
            .all(|x| leaves.iter().any(|y| *x == *y))
        {
            if let Some((configs, best_mtu_cost, best_gate_num)) = &mut best_configs {
                use std::cmp::Ordering;
                use std::cmp::Reverse;
                let best_cost = (*best_mtu_cost, Reverse(*best_gate_num));
                let cmp_result = (mtu_cost, Reverse(gate_num)).cmp(&best_cost);
                if cmp_result == Ordering::Less {
                    configs.clear();
                    *best_mtu_cost = mtu_cost;
                    *best_gate_num = gate_num;
                }
                if cmp_result != Ordering::Greater {
                    configs.push((leaves.to_vec(), lop3_fill_moves(moves)));
                }
            } else {
                best_configs = Some((
                    vec![(leaves.to_vec(), lop3_fill_moves(moves))],
                    mtu_cost,
                    gate_num,
                ));
            }
        }
    };
    find_best_lop3node_generic(
        circuit,
        lop3nodes,
        coverage,
        subtrees,
        circuit_outputs,
        wire_index,
        preferred_nodes,
        reg_sol,
    );

    let best_config = best_configs.unwrap();
    best_config
        .0
        .into_iter()
        .map(|(leaves, moves)| {
            let mut args = [leaves[0]; 3];
            for (i, t) in leaves.iter().enumerate() {
                args[i] = *t;
            }
            LOP3Node {
                args,
                tree_paths: moves,
                mtu_cost: best_config.1,
            }
        })
        .collect::<Vec<_>>()
}

fn update_lop3nodes_variants<T>(lop3nodes: &[LOP3Node<T>], variants: Vec<(T, Vec<T>)>) {}

fn update_mtuareas_from_lop3node<T>(
    circuit: &VBinOpCircuit<T>,
    mtuareas: &mut [MTUArea<T>],
    coverage: &[T],
    subtree: &SubTree<T>,
    lop3enableds: &[bool],
    lop3nodes: &[LOP3Node<T>],
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len).unwrap();
    let st_gates = subtree.gates();
    let subtree_index = coverage[usize::try_from(subtree.root()).unwrap() - input_len];
    let mut mtuareas_indices = HashSet::new();
    for (i, node) in subtree
        .gates()
        .iter()
        .map(|(x, _)| *x)
        .chain(std::iter::once(subtree.root()))
        .enumerate()
    {
        let gidx = usize::try_from(node).unwrap() - input_len;
        if !lop3enableds[gidx] {
            continue;
        }
        for arg in &lop3nodes[gidx].args {
            if *arg >= circuit.input_len {
                let arg_gidx = usize::try_from(*arg).unwrap() - input_len;
                let arg_subtree_index = coverage[arg_gidx];
                if arg_subtree_index != subtree_index {
                    // if this is not this subtree then fill MTUarea
                    let arg_subtree_index_u = usize::try_from(arg_subtree_index).unwrap();
                    mtuareas[arg_subtree_index_u].add_node(*arg, node);
                    mtuareas_indices.insert(arg_subtree_index_u);
                }
            }
        }
    }
    for i in mtuareas_indices {
        mtuareas[i].sort_and_dedup();
    }
}

// MTU graph and coverage: index - gate index, value - subtree index
fn gen_subtree_coverage<T>(circuit: &VBinOpCircuit<T>, subtrees: &[SubTree<T>]) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len = usize::try_from(circuit.input_len).unwrap();
    let mut coverage = vec![T::default(); circuit.gates.len()];
    for (i, st) in subtrees.iter().enumerate() {
        for (gi, _) in st
            .gates()
            .iter()
            .copied()
            .chain(std::iter::once((st.root(), T::default())))
        {
            let gi = usize::try_from(gi).unwrap();
            coverage[gi - input_len] = T::try_from(i).unwrap();
        }
    }
    coverage
}

fn get_preferred_nodes_from_mtuareas<T>(
    circuit: &VBinOpCircuit<T>,
    mtuareas: &[MTUArea<T>],
    coverage: &[T],
    wire_index: T,
) -> Vec<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let top_subtree = coverage[usize::try_from(wire_index).unwrap() - input_len];
    // generate tree to explore
    let tree = get_small_tree(circuit, wire_index);
    let mut preferred_nodes = vec![];
    for t in tree.into_iter().filter_map(|t| t) {
        if t >= input_len_t {
            let subtree = coverage[usize::try_from(t).unwrap() - input_len];
            if subtree != top_subtree {
                let subtree_u = usize::try_from(subtree).unwrap();
                if mtuareas[subtree_u].nodes.iter().any(|(x, _)| *x == t) {
                    preferred_nodes.push(t);
                }
            }
        }
    }
    preferred_nodes
}

fn filter_lop3nodes_in_mtuarea<T>(
    input_len: usize,
    lop3enableds: &mut [bool],
    lop3nodes: &[LOP3Node<T>],
    farest_nodes: &[T],
    subtree: &SubTree<T>,
) where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    #[derive(Clone)]
    struct StackEntry {
        st_node: usize,
        way: usize,
    }
    let st_gates = subtree.gates();
    let mut visited = vec![false; st_gates.len() + 1];
    for node in farest_nodes {
        let mut stack = vec![StackEntry {
            st_node: subtree.find_index(*node).unwrap(),
            way: 0,
        }];
        while !stack.is_empty() {
            let top = stack.last_mut().unwrap();
            let top_way = top.way;
            let node = if top.st_node < st_gates.len() {
                st_gates[top.st_node].0
            } else {
                subtree.root()
            };
            let gidx = usize::try_from(node).unwrap() - input_len;
            if top_way == 0 {
                if !visited[top.st_node] {
                    visited[top.st_node] = true;
                    lop3enableds[gidx] = true;
                } else {
                    stack.pop();
                    continue;
                }
            }
            if top_way < 3 {
                top.way += 1;
                let next = lop3nodes[gidx].args[top_way];
                if let Some(st_next) = subtree.find_index(next) {
                    stack.push(StackEntry {
                        st_node: st_next,
                        way: 0,
                    });
                }
            } else {
                stack.pop();
            }
        }
    }
}

impl<T> From<Circuit<T>> for VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: Circuit<T>) -> Self {
        // for especially NVIDIA LOP3 - enabled by default
        let mut vcircuit = VBinOpCircuit::from(circuit.clone());
        vcircuit.optimize_negs();
        Self::from(vcircuit)
    }
}

impl<T> From<VBinOpCircuit<T>> for VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from(circuit: VBinOpCircuit<T>) -> Self {
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let circuit_outputs = HashSet::<T>::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3enableds = vec![false; gates.len()];
        let mut mtuareas = subtrees
            .iter()
            .map(|s| {
                let mut mtuarea = MTUArea::<T>::empty_with_root(s.root());
                if circuit_outputs.contains(&s.root()) {
                    mtuarea.nodes.push((s.root(), vec![]));
                }
                mtuarea
            })
            .collect::<Vec<_>>();
        let mut lop3nodes = vec![LOP3Node::<T>::default(); gates.len()];
        // generate lop3nodes
        for i in (0..subtrees.len()).rev() {
            let subtree = &subtrees[i];
            mtuareas[i].improve_and_optimize_and_gen_lop3nodes(&circuit, &mut lop3nodes);
            let (farest_nodes, nonfarest_nodes) = mtuareas[i].farest_nonfarest_nodes(&circuit);
            // get nonfarest nodes
            for (i, nidx) in subtree
                .gates()
                .iter()
                .map(|(x, _)| *x)
                .chain(std::iter::once(subtree.root()))
                .enumerate()
                // skip all nonfarest nodes in MTUAreaview
                .filter(|(_, nidx)| nonfarest_nodes.iter().all(|x| *x != *nidx))
            {
                let gidx = usize::try_from(nidx).unwrap() - input_len;
                // get preferred nodes from mtuareas
                let preferred_nodes =
                    get_preferred_nodes_from_mtuareas(&circuit, &mtuareas, &cov, nidx);
                lop3nodes[gidx] = find_best_lop3node(
                    &circuit,
                    &lop3nodes,
                    &cov,
                    &subtrees,
                    &circuit_outputs,
                    nidx,
                    &preferred_nodes,
                );
            }
            // filter out current mtublock
            filter_lop3nodes_in_mtuarea(
                input_len,
                &mut lop3enableds,
                &lop3nodes,
                &farest_nodes,
                subtree,
            );
            update_mtuareas_from_lop3node(
                &circuit,
                &mut mtuareas,
                &cov,
                subtree,
                &lop3enableds,
                &lop3nodes,
            );
        }
        // convert inputs in lop3nodes
        Self::from_lop3nodes(circuit, lop3nodes)
        // optimize can be called later
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vcircuit::*;
    use gatesim::Gate;

    fn gen_subtree_coverage_from_circuit(circuit: Circuit<u32>) -> Vec<u32> {
        let binop_circuit = VBinOpCircuit::from(circuit.clone());
        let subtrees = binop_circuit.subtrees();
        gen_subtree_coverage(&binop_circuit, &subtrees)
    }

    #[test]
    fn test_gen_subtree_coverage() {
        assert_eq!(
            vec![0, 1, 0, 1, 0, 1, 2, 3, 4, 5, 5, 5, 5, 6, 5, 6, 6, 6, 6],
            gen_subtree_coverage_from_circuit(
                Circuit::new(
                    4,
                    [
                        Gate::new_nimpl(0, 1),   // 4
                        Gate::new_and(0, 3),     // 5
                        Gate::new_xor(1, 4),     // 6
                        Gate::new_and(3, 5),     // 7
                        Gate::new_xor(2, 6),     // 8
                        Gate::new_xor(3, 7),     // 9
                        Gate::new_nor(8, 9),     // 10
                        Gate::new_and(8, 9),     // 11
                        Gate::new_nimpl(8, 9),   // 12
                        Gate::new_nor(0, 10),    // 13
                        Gate::new_nor(1, 11),    // 14
                        Gate::new_xor(2, 12),    // 15
                        Gate::new_xor(13, 14),   // 16
                        Gate::new_and(0, 10),    // 17 tree4
                        Gate::new_nor(15, 16),   // 18 tree3
                        Gate::new_nimpl(1, 12),  // 19 tree4
                        Gate::new_nimpl(11, 17), // 20
                        Gate::new_nimpl(3, 19),  // 21
                        Gate::new_xor(20, 21),   // 22
                    ],
                    [(18, true), (22, false)],
                )
                .unwrap()
            )
        );

        assert_eq!(
            vec![0, 0, 1, 1, 1, 2, 2, 2],
            gen_subtree_coverage_from_circuit(
                Circuit::new(
                    3,
                    [
                        Gate::new_nimpl(0, 1),
                        Gate::new_xor(3, 2),
                        Gate::new_nimpl(4, 2),
                        Gate::new_and(0, 1),
                        Gate::new_nor(5, 6),
                        Gate::new_nor(2, 7),
                        Gate::new_xor(1, 7),
                        Gate::new_and(8, 9),
                    ],
                    [(4, true), (10, false)],
                )
                .unwrap()
            )
        );
    }

    fn to_paths(paths: [u8; 7]) -> LOP3SubTreePaths {
        paths.map(|x| PathMove(x))
    }

    #[test]
    fn test_lop3_fill_moves() {
        assert_eq!(
            to_paths([3, 0, 0, 0, 0, 0, 0]),
            lop3_fill_moves(LOP3_SUBTREE_PATHS_DEFAULT),
        );
        assert_eq!(
            to_paths([3, 3, 0, 0, 0, 0, 0]),
            lop3_fill_moves(to_paths([1, 0, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 0, 3, 0, 0, 0, 0]),
            lop3_fill_moves(to_paths([2, 0, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 3, 0, 0, 0, 0]),
            lop3_fill_moves(to_paths([3, 0, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 0, 3, 0, 0, 0]),
            lop3_fill_moves(to_paths([1, 1, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 0, 0, 3, 0, 0]),
            lop3_fill_moves(to_paths([1, 2, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 0, 3, 3, 0, 0]),
            lop3_fill_moves(to_paths([1, 3, 0, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 3, 3, 0, 0, 3]),
            lop3_fill_moves(to_paths([3, 1, 2, 0, 0, 0, 0])),
        );
        assert_eq!(
            to_paths([3, 3, 3, 0, 3, 3, 0]),
            lop3_fill_moves(to_paths([3, 2, 1, 0, 0, 0, 0])),
        );
    }

    fn vbgate<T: Clone + Copy>(func: VGateFunc, i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        (VGate { i0, i1, func: func }, negs)
    }

    fn vbgate_and<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::And, i0, i1, negs)
    }
    fn vbgate_or<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::Or, i0, i1, negs)
    }
    fn vbgate_xor<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> (VGate<T>, VNegs) {
        vbgate(VGateFunc::Xor, i0, i1, negs)
    }

    fn simple_call_find_best_lop3node(circuit: VBinOpCircuit<u32>) -> Vec<LOP3Node<u32>> {
        println!("Call find_best_lop3node");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3nodes = vec![LOP3Node::default(); gates.len()];
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        for i in input_len..input_len + gates.len() {
            lop3nodes[i - input_len] = find_best_lop3node(
                &circuit,
                &lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
                u32::try_from(i).unwrap(),
                &[],
            );
        }
        lop3nodes
    }

    fn call_find_best_lop3node_with_preferred(
        circuit: VBinOpCircuit<u32>,
        preferred_nodes: &[u32],
        skip_nodes: &[u32],
    ) -> Vec<LOP3Node<u32>> {
        println!("Call find_best_lop3node with preferred_nodes");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        println!("Coverage: {:?}", cov);
        let mut lop3nodes = vec![LOP3Node::default(); gates.len()];
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        for i in input_len..input_len + gates.len() {
            if skip_nodes.iter().any(|x| *x == u32::try_from(i).unwrap()) {
                continue;
            }
            println!("Node index {}", i);
            lop3nodes[i - input_len] = find_best_lop3node(
                &circuit,
                &lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
                u32::try_from(i).unwrap(),
                preferred_nodes,
            );
        }
        lop3nodes
    }

    #[test]
    fn test_find_best_lop3node() {
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 1, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 1, 2],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    vbgate_and(0, 1, NoNegs),
                    vbgate_or(0, 2, NegOutput),
                    vbgate_xor(3, 4, NoNegs),
                ],
                outputs: vec![(5, false)],
            })
        );
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 1, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 1, 2],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 1],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 1],
                    tree_paths: to_paths([3, 3, 3, 3, 3, 3, 3]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    vbgate_and(0, 1, NoNegs),
                    vbgate_or(0, 2, NegOutput),
                    vbgate_xor(0, 2, NegInput1),
                    vbgate_and(1, 2, NegOutput),
                    vbgate_xor(3, 4, NoNegs),
                    vbgate_and(5, 6, NegInput1),
                    vbgate_or(7, 8, NoNegs),
                ],
                outputs: vec![(9, false)],
            })
        );
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 1, 2],
                    tree_paths: to_paths([3, 3, 3, 0, 3, 0, 3]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    vbgate_or(0, 2, NegOutput),
                    vbgate_and(1, 2, NegOutput),
                    vbgate_xor(0, 3, NoNegs),
                    vbgate_and(1, 4, NegInput1),
                    vbgate_or(5, 6, NoNegs),
                ],
                outputs: vec![(7, false)],
            })
        );
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 0, 2],
                    tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 1, 0],
                    tree_paths: to_paths([3, 3, 3, 3, 0, 3, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    vbgate_or(0, 2, NegOutput),
                    vbgate_and(1, 2, NegOutput),
                    vbgate_xor(3, 2, NoNegs),
                    vbgate_and(4, 1, NegInput1),
                    vbgate_or(5, 6, NoNegs),
                ],
                outputs: vec![(7, false)],
            })
        );
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 2, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 1],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 0, 2],
                    tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 2, 0],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 3, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 3,
                gates: vec![
                    vbgate_or(0, 2, NegOutput),
                    vbgate_and(1, 2, NegOutput),
                    vbgate_xor(3, 2, NoNegs),
                    vbgate_or(4, 5, NoNegs),
                ],
                outputs: vec![(6, false)],
            })
        );
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 1, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [1, 3, 1],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 4, 2],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [3, 4, 3],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 1, 3],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 4, 3],
                    tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [9, 10, 9],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 3,
                },
            ],
            simple_call_find_best_lop3node(VBinOpCircuit {
                input_len: 5,
                gates: vec![
                    vbgate_and(0, 1, NoNegs),
                    vbgate_or(1, 3, NegOutput),
                    vbgate_xor(2, 4, NegInput1),
                    vbgate_and(3, 4, NegOutput),
                    vbgate_xor(5, 6, NoNegs),
                    vbgate_and(7, 8, NegInput1),
                    vbgate_or(9, 10, NoNegs),
                ],
                outputs: vec![(11, false)],
            })
        );
        // full adder
        assert_eq!(
            vec![
                LOP3Node {
                    args: [0, 0, 0],
                    tree_paths: to_paths([0, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: 0,
                },
                LOP3Node {
                    args: [2, 0, 1],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 0, 1],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [0, 1, 0],
                    tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
                LOP3Node {
                    args: [2, 0, 1],
                    tree_paths: to_paths([3, 3, 3, 0, 3, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                },
            ],
            call_find_best_lop3node_with_preferred(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_xor(0, 1, NoNegs),
                        vbgate_xor(2, 3, NoNegs),
                        vbgate_and(2, 3, NoNegs),
                        vbgate_and(0, 1, NoNegs),
                        vbgate_or(5, 6, NoNegs),
                    ],
                    outputs: vec![(4, false), (7, false)],
                },
                &[3][..],
                &[3][..],
            )
        );
        // full adder 2
        assert_eq!(
            std::iter::repeat(LOP3Node {
                args: [0, 0, 0],
                tree_paths: to_paths([0, 0, 0, 0, 0, 0, 0]),
                mtu_cost: 0,
            })
            .take(4)
            .chain(
                [
                    LOP3Node {
                        args: [9, 4, 5],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1
                    }, // 10
                    LOP3Node {
                        args: [9, 4, 5],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1
                    }, // 11
                    LOP3Node {
                        args: [7, 0, 1],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1
                    }, // 12
                    LOP3Node {
                        args: [6, 7, 8],
                        tree_paths: to_paths([3, 3, 3, 0, 3, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1
                    }
                ]
                .into_iter()
            )
            .collect::<Vec<_>>(),
            call_find_best_lop3node_with_preferred(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 6
                        vbgate_or(2, 3, NegInput1),  // 7
                        vbgate_xor(4, 5, NoNegs),    // 8
                        vbgate_xor(6, 7, NoNegs),    // 9
                        vbgate_xor(8, 9, NoNegs),    // 10
                        vbgate_and(8, 9, NoNegs),    // 11
                        vbgate_and(6, 7, NoNegs),    // 12
                        vbgate_or(11, 12, NoNegs),
                    ],
                    outputs: vec![(10, false), (13, false)],
                },
                &[6, 7, 8, 9][..],
                &[6, 7, 8, 9][..],
            )
        );
        // full adder 2
        assert_eq!(
            std::iter::repeat(LOP3Node {
                args: [0, 0, 0],
                tree_paths: to_paths([0, 0, 0, 0, 0, 0, 0]),
                mtu_cost: 0,
            })
            .take(4)
            .chain(
                [
                    LOP3Node {
                        args: [9, 4, 5],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 2
                    }, // 10
                    LOP3Node {
                        args: [9, 4, 5],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 2
                    }, // 11
                    LOP3Node {
                        args: [7, 0, 1],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 2
                    }, // 12
                    LOP3Node {
                        args: [6, 7, 8],
                        tree_paths: to_paths([3, 3, 3, 0, 3, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 4
                    }
                ]
                .into_iter()
            )
            .collect::<Vec<_>>(),
            call_find_best_lop3node_with_preferred(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 6
                        vbgate_or(2, 3, NegInput1),  // 7
                        vbgate_xor(4, 5, NoNegs),    // 8
                        vbgate_xor(6, 7, NoNegs),    // 9
                        vbgate_xor(8, 9, NoNegs),    // 10
                        vbgate_and(8, 9, NoNegs),    // 11
                        vbgate_and(6, 7, NoNegs),    // 12
                        vbgate_or(11, 12, NoNegs),
                    ],
                    outputs: vec![(10, false), (13, false)],
                },
                &[][..],
                &[6, 7, 8, 9][..],
            )
        );
    }

    fn simple_call_find_best_lop3node_variants(
        circuit: VBinOpCircuit<u32>,
        preferred_nodes: &[u32],
        required_args: &[u32],
        node: u32,
    ) -> Vec<LOP3Node<u32>> {
        println!("Call find_best_lop3node_variants");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3nodes = vec![LOP3Node::default(); gates.len()];
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        find_best_lop3node_variants(
            &circuit,
            &lop3nodes,
            &cov,
            &subtrees,
            &circuit_outputs,
            u32::try_from(node).unwrap(),
            preferred_nodes,
            required_args,
        )
    }

    #[test]
    fn test_find_best_lop3node_variants() {
        assert_eq!(
            vec![
                LOP3Node {
                    args: [9, 4, 5],
                    tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1
                },
                LOP3Node {
                    args: [6, 7, 8],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1
                }
            ],
            simple_call_find_best_lop3node_variants(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 6
                        vbgate_or(2, 3, NegInput1),  // 7
                        vbgate_xor(4, 5, NoNegs),    // 8
                        vbgate_xor(6, 7, NoNegs),    // 9
                        vbgate_xor(8, 9, NoNegs),    // 10
                        vbgate_and(8, 9, NoNegs),    // 11
                        vbgate_and(6, 7, NoNegs),    // 12
                        vbgate_or(11, 12, NoNegs),
                    ],
                    outputs: vec![(10, false), (13, false)],
                },
                &[6, 7, 8, 9][..],
                &[],
                10
            )
        );
        assert_eq!(
            vec![LOP3Node {
                args: [9, 4, 5],
                tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                mtu_cost: MTU_COST_BASE + 1
            },],
            simple_call_find_best_lop3node_variants(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 6
                        vbgate_or(2, 3, NegInput1),  // 7
                        vbgate_xor(4, 5, NoNegs),    // 8
                        vbgate_xor(6, 7, NoNegs),    // 9
                        vbgate_xor(8, 9, NoNegs),    // 10
                        vbgate_and(8, 9, NoNegs),    // 11
                        vbgate_and(6, 7, NoNegs),    // 12
                        vbgate_or(11, 12, NoNegs),
                    ],
                    outputs: vec![(10, false), (13, false)],
                },
                &[6, 7, 8, 9][..],
                &[4, 9],
                10
            )
        );
    }

    fn to_mtunodes(nodes: Vec<u32>) -> Vec<(u32, Vec<u32>)> {
        nodes.into_iter().map(|x| (x, vec![])).collect()
    }

    #[test]
    fn test_mtuarea_farest_nonfarest_nodes() {
        let circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                vbgate_and(0, 1, NoNegs),
                vbgate_or(0, 2, NegOutput),
                vbgate_xor(0, 2, NegInput1),
                vbgate_and(1, 2, NegOutput),
                vbgate_xor(3, 4, NoNegs),
                vbgate_and(5, 6, NegInput1),
                vbgate_or(7, 8, NoNegs),
                vbgate_xor(0, 1, NoNegs),
                vbgate_and(1, 2, NegInput1),
                vbgate_or(1, 2, NegInput1),
                vbgate_and(1, 2, NegOutput),
                vbgate_xor(10, 11, NoNegs),
                vbgate_and(13, 14, NegInput1),
                vbgate_or(14, 15, NoNegs),
            ],
            outputs: vec![(9, false), (16, false)],
        };
        assert_eq!(
            (vec![8, 3, 4], vec![7]),
            MTUArea {
                root: 9,
                nodes: to_mtunodes(vec![3, 4, 7, 8]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
        assert_eq!(
            (vec![8, 3, 4, 5], vec![9, 7]),
            MTUArea {
                root: 9,
                nodes: to_mtunodes(vec![3, 4, 5, 7, 8, 9]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
        assert_eq!(
            (vec![15, 10, 11], vec![16]),
            MTUArea {
                root: 16,
                nodes: to_mtunodes(vec![10, 11, 15, 16]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
        let circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                vbgate_and(0, 1, NoNegs),
                vbgate_xor(0, 2, NegInput1),
                vbgate_xor(3, 4, NoNegs),
            ],
            outputs: vec![(5, false)],
        };
        assert_eq!(
            (vec![3, 4], vec![5]),
            MTUArea {
                root: 5,
                nodes: to_mtunodes(vec![3, 4, 5]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
        assert_eq!(
            (vec![5, 3], vec![]),
            MTUArea {
                root: 5,
                nodes: to_mtunodes(vec![3, 5]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
        let circuit = VBinOpCircuit {
            input_len: 3,
            gates: vec![
                vbgate_and(0, 1, NoNegs),
                vbgate_or(0, 2, NegOutput),
                vbgate_xor(3, 4, NoNegs),
                vbgate_xor(1, 2, NoNegs),
                vbgate_and(5, 6, NoNegs),
            ],
            outputs: vec![(7, false)],
        };
        assert_eq!(
            (vec![6, 3, 4], vec![7]),
            MTUArea {
                root: 7,
                nodes: to_mtunodes(vec![3, 4, 6, 7]),
                cost: 0,
            }
            .farest_nonfarest_nodes(&circuit)
        );
    }

    fn simple_call_get_preferred_nodes_from_mtuareas(
        circuit: VBinOpCircuit<u32>,
        mtuareas: Vec<MTUArea<u32>>,
        wire_indices: Vec<u32>,
    ) -> Vec<Vec<u32>> {
        println!("Call get_preferred_nodes_from_mtuareas");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        println!("Coverage: {:?}", cov);
        wire_indices
            .into_iter()
            .map(|node| get_preferred_nodes_from_mtuareas(&circuit, &mtuareas, &cov, node))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_get_preferred_nodes_from_mtuareas() {
        assert_eq!(
            vec![Vec::<u32>::new(); 3],
            simple_call_get_preferred_nodes_from_mtuareas(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),
                        vbgate_or(0, 2, NegOutput),
                        vbgate_xor(3, 4, NoNegs),
                    ],
                    outputs: vec![(5, false)],
                },
                vec![MTUArea {
                    root: 5,
                    nodes: to_mtunodes(vec![5]),
                    cost: 0,
                }],
                vec![3, 4, 5]
            )
        );
        assert_eq!(
            vec![vec![3], vec![3], vec![], vec![3]],
            simple_call_get_preferred_nodes_from_mtuareas(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_xor(0, 1, NoNegs),
                        vbgate_xor(2, 3, NoNegs),
                        vbgate_and(2, 3, NoNegs),
                        vbgate_and(0, 1, NoNegs),
                        vbgate_or(5, 6, NoNegs),
                    ],
                    outputs: vec![(4, false), (7, false)],
                },
                vec![
                    MTUArea {
                        root: 3,
                        nodes: to_mtunodes(vec![3]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 4,
                        nodes: to_mtunodes(vec![4]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 7,
                        nodes: to_mtunodes(vec![7]),
                        cost: 0,
                    },
                ],
                vec![4, 5, 6, 7]
            )
        );
        assert_eq!(
            vec![vec![5, 3, 4], vec![5, 3, 4], vec![], vec![5]],
            simple_call_get_preferred_nodes_from_mtuareas(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NegOutput),
                        vbgate_xor(0, 1, NoNegs),
                        vbgate_or(3, 4, NegInput1),
                        vbgate_xor(2, 5, NoNegs),
                        vbgate_and(2, 5, NoNegs),
                        vbgate_and(0, 1, NoNegs),
                        vbgate_or(7, 8, NoNegs),
                    ],
                    outputs: vec![(6, false), (9, false)],
                },
                vec![
                    MTUArea {
                        root: 5,
                        nodes: to_mtunodes(vec![3, 4, 5]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 6,
                        nodes: to_mtunodes(vec![6]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 9,
                        nodes: to_mtunodes(vec![9]),
                        cost: 0,
                    },
                ],
                vec![6, 7, 8, 9]
            )
        );
        assert_eq!(
            vec![vec![8, 3, 7], vec![8, 3, 7], vec![3], vec![8]],
            simple_call_get_preferred_nodes_from_mtuareas(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NegOutput),
                        vbgate_xor(0, 1, NoNegs),
                        vbgate_or(3, 4, NegInput1),
                        vbgate_and(1, 2, NegOutput),
                        vbgate_xor(1, 2, NoNegs),
                        vbgate_or(6, 7, NegInput1),
                        vbgate_xor(5, 8, NoNegs),
                        vbgate_and(5, 8, NoNegs),
                        vbgate_and(0, 5, NoNegs),
                        vbgate_or(10, 11, NoNegs),
                    ],
                    outputs: vec![(9, false), (12, false)],
                },
                vec![
                    MTUArea {
                        root: 5,
                        nodes: to_mtunodes(vec![3]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 8,
                        nodes: to_mtunodes(vec![7, 8]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 9,
                        nodes: to_mtunodes(vec![9]),
                        cost: 0,
                    },
                    MTUArea {
                        root: 12,
                        nodes: to_mtunodes(vec![12]),
                        cost: 0,
                    },
                ],
                vec![9, 10, 11, 12]
            )
        );
    }

    fn simple_call_filter_lop3nodes_in_mtuarea(
        circuit: VBinOpCircuit<u32>,
        farest_nodes: Vec<u32>,
        subtree_index: usize,
    ) -> Vec<bool> {
        println!("Call get_preferred_nodes_from_mtuareas");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3nodes = vec![LOP3Node::default(); gates.len()];
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        for i in input_len..input_len + gates.len() {
            lop3nodes[i - input_len] = find_best_lop3node(
                &circuit,
                &lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
                u32::try_from(i).unwrap(),
                &[],
            );
        }
        for (i, lop3) in lop3nodes.iter().enumerate() {
            println!("LOP3 {}: {:?}", i + input_len, lop3.args);
        }
        let mut lop3enableds = vec![false; gates.len()];
        filter_lop3nodes_in_mtuarea(
            input_len,
            &mut lop3enableds,
            &lop3nodes,
            &farest_nodes,
            &subtrees[subtree_index],
        );
        lop3enableds
    }

    #[test]
    fn test_filter_lop3nodes_in_mtuarea() {
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, true, true,
                true, true, true, false, false, true, true
            ],
            simple_call_filter_lop3nodes_in_mtuarea(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),     // 6
                        vbgate_or(0, 1, NoNegs),      // 7
                        vbgate_or(1, 2, NegInput1),   // 8
                        vbgate_xor(1, 2, NoNegs),     // 9
                        vbgate_and(2, 3, NoNegs),     // 10
                        vbgate_or(2, 3, NoNegs),      // 11
                        vbgate_or(2, 4, NegInput1),   // 12
                        vbgate_xor(2, 4, NoNegs),     // 13
                        vbgate_and(3, 5, NoNegs),     // 14
                        vbgate_or(3, 5, NoNegs),      // 15
                        vbgate_or(4, 5, NegInput1),   // 16
                        vbgate_xor(4, 5, NoNegs),     // 17
                        vbgate_and(1, 3, NoNegs),     // 18
                        vbgate_or(1, 3, NoNegs),      // 19
                        vbgate_or(0, 4, NegInput1),   // 20
                        vbgate_xor(0, 4, NoNegs),     // 21
                        vbgate_and(6, 7, NoNegs),     // 22 1
                        vbgate_or(8, 9, NoNegs),      // 23
                        vbgate_or(10, 11, NegInput1), // 24
                        vbgate_xor(12, 13, NoNegs),   // 25
                        vbgate_and(14, 15, NoNegs),   // 26
                        vbgate_or(16, 17, NoNegs),    // 27
                        vbgate_or(18, 19, NegInput1), // 28
                        vbgate_xor(20, 21, NoNegs),   // 29
                        vbgate_and(22, 23, NoNegs),   // 30 2
                        vbgate_or(24, 25, NoNegs),    // 31
                        vbgate_or(26, 27, NegInput1), // 32
                        vbgate_xor(28, 29, NoNegs),   // 33
                        vbgate_and(30, 31, NoNegs),   // 34 3
                        vbgate_or(32, 33, NoNegs),    // 35
                        vbgate_xor(34, 35, NoNegs),   // 36 4
                    ],
                    outputs: vec![(36, false)],
                },
                vec![36],
                0,
            )
        );
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, true,
                true, true, false, true, true, false, false
            ],
            simple_call_filter_lop3nodes_in_mtuarea(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),     // 6
                        vbgate_or(0, 1, NoNegs),      // 7
                        vbgate_or(1, 2, NegInput1),   // 8
                        vbgate_xor(1, 2, NoNegs),     // 9
                        vbgate_and(2, 3, NoNegs),     // 10
                        vbgate_or(2, 3, NoNegs),      // 11
                        vbgate_or(2, 4, NegInput1),   // 12
                        vbgate_xor(2, 4, NoNegs),     // 13
                        vbgate_and(3, 5, NoNegs),     // 14
                        vbgate_or(3, 5, NoNegs),      // 15
                        vbgate_or(4, 5, NegInput1),   // 16
                        vbgate_xor(4, 5, NoNegs),     // 17
                        vbgate_and(1, 3, NoNegs),     // 18
                        vbgate_or(1, 3, NoNegs),      // 19
                        vbgate_or(0, 4, NegInput1),   // 20
                        vbgate_xor(0, 4, NoNegs),     // 21
                        vbgate_and(6, 7, NoNegs),     // 22 1
                        vbgate_or(8, 9, NoNegs),      // 23
                        vbgate_or(10, 11, NegInput1), // 24
                        vbgate_xor(12, 13, NoNegs),   // 25
                        vbgate_and(14, 15, NoNegs),   // 26
                        vbgate_or(16, 17, NoNegs),    // 27
                        vbgate_or(18, 19, NegInput1), // 28
                        vbgate_xor(20, 21, NoNegs),   // 29
                        vbgate_and(22, 23, NoNegs),   // 30 2
                        vbgate_or(24, 25, NoNegs),    // 31
                        vbgate_or(26, 27, NegInput1), // 32
                        vbgate_xor(28, 29, NoNegs),   // 33
                        vbgate_and(30, 31, NoNegs),   // 34 3
                        vbgate_or(32, 33, NoNegs),    // 35
                        vbgate_xor(34, 35, NoNegs),   // 36 4
                    ],
                    outputs: vec![(36, false)],
                },
                vec![33, 34],
                0,
            )
        );
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, true, true,
                false, true, true, true, false, true, false, true, true
            ],
            simple_call_filter_lop3nodes_in_mtuarea(
                VBinOpCircuit {
                    input_len: 4,
                    gates: vec![
                        vbgate_and(1, 3, NegInput1),  // 4
                        vbgate_xor(2, 3, NoNegs),     // 5
                        vbgate_and(0, 1, NoNegs),     // 6
                        vbgate_or(0, 1, NoNegs),      // 7
                        vbgate_or(1, 2, NegInput1),   // 8
                        vbgate_xor(1, 2, NoNegs),     // 9
                        vbgate_and(2, 3, NoNegs),     // 10
                        vbgate_or(2, 3, NoNegs),      // 11
                        vbgate_or(2, 4, NegInput1),   // 12
                        vbgate_xor(2, 4, NoNegs),     // 13
                        vbgate_and(3, 5, NoNegs),     // 14
                        vbgate_or(3, 5, NoNegs),      // 15
                        vbgate_or(4, 5, NegInput1),   // 16
                        vbgate_xor(4, 5, NoNegs),     // 17
                        vbgate_and(1, 3, NoNegs),     // 18
                        vbgate_or(1, 3, NoNegs),      // 19
                        vbgate_or(0, 4, NegInput1),   // 20
                        vbgate_xor(0, 4, NoNegs),     // 21
                        vbgate_and(6, 7, NoNegs),     // 22 1
                        vbgate_or(8, 9, NoNegs),      // 23
                        vbgate_or(10, 11, NegInput1), // 24
                        vbgate_xor(12, 13, NoNegs),   // 25
                        vbgate_and(14, 15, NoNegs),   // 26
                        vbgate_or(16, 17, NoNegs),    // 27
                        vbgate_or(18, 19, NegInput1), // 28
                        vbgate_xor(20, 21, NoNegs),   // 29
                        vbgate_and(22, 23, NoNegs),   // 30 2
                        vbgate_or(24, 25, NoNegs),    // 31
                        vbgate_or(26, 27, NegInput1), // 32
                        vbgate_xor(28, 29, NoNegs),   // 33
                        vbgate_and(30, 31, NoNegs),   // 34 3
                        vbgate_or(32, 33, NoNegs),    // 35
                        vbgate_xor(34, 35, NoNegs),   // 36 4
                    ],
                    outputs: vec![(36, false)],
                },
                vec![36],
                2,
            )
        );
    }

    fn simple_call_update_mtuareas_from_lop3node(
        circuit: VBinOpCircuit<u32>,
        lop3enableds: Vec<bool>,
        subtree_index: usize,
    ) -> Vec<Vec<(u32, Vec<u32>)>> {
        println!("Call get_preferred_nodes_from_mtuareas");
        let subtrees = circuit.subtrees();
        let gates = &circuit.gates;
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut lop3nodes = vec![LOP3Node::default(); gates.len()];
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        for i in input_len..input_len + gates.len() {
            lop3nodes[i - input_len] = find_best_lop3node(
                &circuit,
                &lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
                u32::try_from(i).unwrap(),
                &[],
            );
        }
        println!("SubTrees: {:?}", subtrees);
        for (i, lop3) in lop3nodes.iter().enumerate() {
            println!("LOP3 {}: {:?}", i + input_len, lop3.args);
        }
        let mut mtuareas = subtrees
            .iter()
            .map(|s| {
                let mut mtuarea = MTUArea::empty_with_root(s.root());
                if circuit_outputs.contains(&s.root()) {
                    mtuarea.nodes.push((s.root(), vec![]));
                }
                mtuarea
            })
            .collect::<Vec<_>>();
        update_mtuareas_from_lop3node(
            &circuit,
            &mut mtuareas,
            &cov,
            &subtrees[subtree_index],
            &lop3enableds,
            &lop3nodes,
        );
        mtuareas
            .into_iter()
            .map(|mtu| mtu.nodes)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_update_mtuareas_from_lop3node() {
        assert_eq!(
            vec![
                vec![(8, vec![18, 24, 26, 29])],
                vec![(11, vec![18, 26])],
                vec![(12, vec![20]), (13, vec![19]), (14, vec![23, 27])],
                vec![],
                vec![(32, vec![])]
            ],
            simple_call_update_mtuareas_from_lop3node(
                VBinOpCircuit {
                    input_len: 6,
                    gates: vec![
                        // other mtuareas
                        vbgate_xor(0, 1, NoNegs),      // 6
                        vbgate_or(0, 2, NoNegs),       // 7
                        vbgate_and(6, 7, NegInput1),   // 8
                        vbgate_or(0, 1, NegOutput),    // 9
                        vbgate_xor(2, 3, NoNegs),      // 10
                        vbgate_and(9, 10, NegInput1),  // 11
                        vbgate_and(2, 4, NegOutput),   // 12
                        vbgate_or(1, 5, NoNegs),       // 13
                        vbgate_and(12, 13, NegInput1), // 14
                        vbgate_or(3, 4, NegOutput),    // 15
                        vbgate_xor(3, 5, NoNegs),      // 16
                        vbgate_and(15, 16, NegInput1), // 17
                        // main mtuarea
                        vbgate_or(8, 11, NoNegs),   // 18
                        vbgate_or(2, 14, NoNegs),   // 19
                        vbgate_and(14, 5, NoNegs),  // 20
                        vbgate_or(2, 17, NoNegs),   // 21
                        vbgate_xor(11, 2, NoNegs),  // 22
                        vbgate_or(3, 14, NoNegs),   // 23
                        vbgate_xor(4, 8, NoNegs),   // 24
                        vbgate_or(17, 5, NoNegs),   // 25
                        vbgate_xor(18, 19, NoNegs), // 26
                        vbgate_or(20, 21, NoNegs),  // 27
                        vbgate_xor(22, 23, NoNegs), // 28
                        vbgate_or(24, 25, NoNegs),  // 29
                        vbgate_xor(26, 27, NoNegs), // 30
                        vbgate_or(28, 29, NoNegs),  // 31
                        vbgate_or(30, 31, NoNegs),  // 32
                    ],
                    outputs: vec![(32, false)],
                },
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, true, true, false, false, true, true, true, true, true, false,
                    true, false, false, false
                ],
                4,
            )
        );
    }

    #[test]
    fn test_farest_nonfarest_nodes_from_mask() {
        assert_eq!(
            (0b0000001, 0b0000000),
            farest_nonfarest_nodes_from_mask(0b0000001),
        );
        assert_eq!(
            (0b0000110, 0b0000001),
            farest_nonfarest_nodes_from_mask(0b0000111),
        );
        assert_eq!(
            (0b0011000, 0b0000010),
            farest_nonfarest_nodes_from_mask(0b0011010),
        );
        assert_eq!(
            (0b1100000, 0b0000100),
            farest_nonfarest_nodes_from_mask(0b1100100),
        );
        assert_eq!(
            (0b0011100, 0b0000001),
            farest_nonfarest_nodes_from_mask(0b0011101),
        );
        assert_eq!(
            (0b1100010, 0b0000001),
            farest_nonfarest_nodes_from_mask(0b1100011),
        );
    }

    fn call_mtuarea_gen_lop3nodes(
        mtuarea_root: u32,
        mtuarea_nodes: Vec<u32>,
        circuit: &VBinOpCircuit<u32>,
    ) {
        let mut mtuarea = MTUArea {
            root: mtuarea_root,
            nodes: mtuarea_nodes
                .into_iter()
                .map(|x| (x, vec![]))
                .collect::<Vec<_>>(),
            cost: 0,
        };
    }

    #[test]
    fn test_mtuarea_gen_lop3nodes() {}
}
