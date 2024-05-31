use gatesim::*;

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::vbinopcircuit::*;
use crate::vcircuit::VGateFunc;
use crate::VNegs::*;

use crate::vlop3circuit::*;

impl<T> VLOP3Circuit<T>
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    fn from_lop3nodes(
        circuit: VBinOpCircuit<T>,
        enableds: Vec<bool>,
        lop3nodes: Vec<LOP3Node<T>>,
    ) -> VLOP3Circuit<T> {
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let mut new_gates: Vec<VLOP3Gate<T>> = vec![];
        let gates = &circuit.gates;
        let mut trans_tbl = vec![T::default(); input_len + gates.len()];
        for (i, (en, lop3node)) in enableds.into_iter().zip(lop3nodes.into_iter()).enumerate() {
            if !en {
                continue;
            }
            let gi = input_len + i;
            let newgi = input_len + new_gates.len();
            trans_tbl[gi] = T::try_from(newgi).unwrap();
            if !lop3node.tree_paths[0].is_empty()
                && lop3node.tree_paths[1].is_empty()
                && lop3node.tree_paths[2].is_empty()
            {
                // single gate
                let func = match gates[i].0.func {
                    VGateFunc::And => VLOP3GateFunc::And,
                    VGateFunc::Or => VLOP3GateFunc::Or,
                    VGateFunc::Xor => VLOP3GateFunc::Xor,
                    _ => {
                        panic!("Unexpected!");
                    }
                };
                new_gates.push(VLOP3Gate {
                    i0: gates[i].0.i0,
                    i1: gates[i].0.i1,
                    i2: gates[i].0.i1,
                    func,
                    negs: gates[i].1,
                });
            } else {
                // more complex LOP3
                let tree = get_small_tree(&circuit, T::try_from(input_len + i).unwrap());
                let mut calcs = [
                    0b11110000u8,
                    0b11001100u8,
                    0b10101010u8,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                let mut level_start = 3;
                let mut level_end = 7;
                // calculate values for tree nodes
                let lop3moves = &lop3node.tree_paths;
                for _ in 0..3 {
                    for l in level_start..level_end {
                        let calc_l = 3 + 7 - l - 1;
                        if let Some(t) = tree[l] {
                            calcs[calc_l] = if lop3node.args[0] == t {
                                calcs[0]
                            } else if lop3node.args[1] == t {
                                calcs[1]
                            } else if lop3node.args[2] == t {
                                calcs[2]
                            } else if t >= circuit.input_len && !lop3moves[l].is_empty() {
                                let tgi = usize::try_from(t).unwrap() - input_len;
                                let l_arg0 = (l << 1) + 1;
                                let l_arg1 = (l << 1) + 2;
                                let va0 = if l_arg0 < 7 && !lop3moves[l_arg0].is_empty() {
                                    calcs[3 + 7 - l_arg0 - 1]
                                } else if gates[tgi].0.i0 == lop3node.args[0] {
                                    calcs[0]
                                } else if gates[tgi].0.i0 == lop3node.args[1] {
                                    calcs[1]
                                } else {
                                    calcs[2]
                                };
                                let va1 = if l_arg1 < 7 && !lop3moves[l_arg1].is_empty() {
                                    calcs[3 + 7 - l_arg1 - 1]
                                } else if gates[tgi].0.i1 == lop3node.args[0] {
                                    calcs[0]
                                } else if gates[tgi].0.i1 == lop3node.args[1] {
                                    calcs[1]
                                } else {
                                    calcs[2]
                                };
                                let va1 = if gates[tgi].1 == NegInput1 {
                                    va1 ^ 0xff
                                } else {
                                    va1
                                };
                                let vop = match gates[tgi].0.func {
                                    VGateFunc::And => va0 & va1,
                                    VGateFunc::Or => va0 | va1,
                                    VGateFunc::Xor => va0 ^ va1,
                                    _ => {
                                        panic!("Unexpected gate");
                                    }
                                };
                                if gates[tgi].1 == NegOutput {
                                    vop ^ 0xff
                                } else {
                                    vop
                                }
                            } else {
                                0
                            };
                        }
                    }
                    level_start >>= 1;
                    level_end >>= 1;
                }
                new_gates.push(VLOP3Gate {
                    i0: lop3node.args[0],
                    i1: lop3node.args[1],
                    i2: lop3node.args[2],
                    func: VLOP3GateFunc::LOP3(*calcs.last().unwrap()),
                    negs: NoNegs,
                });
            }
        }
        // translation of gate arguments
        for g in new_gates.iter_mut() {
            g.i0 = if g.i0 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i0).unwrap()]
            } else {
                g.i0
            };
            g.i1 = if g.i1 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i1).unwrap()]
            } else {
                g.i1
            };
            g.i2 = if g.i2 >= circuit.input_len {
                trans_tbl[usize::try_from(g.i2).unwrap()]
            } else {
                g.i2
            };
        }

        Self {
            input_len: circuit.input_len,
            gates: new_gates,
            outputs: circuit
                .outputs
                .into_iter()
                .map(|(o, n)| {
                    if o >= circuit.input_len {
                        (trans_tbl[usize::try_from(o).unwrap()], n)
                    } else {
                        (o, n)
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

fn get_small_tree_with_cov<T>(
    circuit: &VBinOpCircuit<T>,
    wire_index: T,
    cov: Option<&[T]>,
) -> [Option<T>; 7]
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gates = &circuit.gates;
    // by default is subtree is zero if no coverage supplied
    let root_subtree_index = if let Some(cov) = cov {
        if wire_index >= circuit.input_len {
            cov[usize::try_from(wire_index).unwrap() - input_len]
        } else {
            T::default() // it doesn't matter what is value
        }
    } else {
        T::default()
    };
    let mut tree = [None; 7];
    let mut old_level_start = 0;
    let mut level_start = 1;
    tree[0] = Some(wire_index);
    for _ in 1..3 {
        for pos in 0..level_start - old_level_start {
            if let Some(t) = tree[old_level_start + pos] {
                if t >= input_len_t {
                    let gi = usize::try_from(t).unwrap();
                    // by default is subtree is zero if no coverage supplied
                    // if no supplied coverage then root_subtree_index == t_subtree_index
                    let t_subtree_index = if let Some(cov) = cov {
                        cov[gi - input_len]
                    } else {
                        T::default()
                    };
                    if t_subtree_index == root_subtree_index {
                        let g = gates[gi - input_len].0;
                        tree[level_start + (pos << 1)] = Some(g.i0);
                        tree[level_start + (pos << 1) + 1] = Some(g.i1);
                    }
                }
            }
        }
        old_level_start = level_start;
        level_start += level_start + 1;
    }
    tree
}

// THIS version of get_small_tree solves following problem by reducing number of
// MTUsubtrees in small trees in paths.
// if MTUsubtrees are single gates: then possible to make LOP3s that
// creates sequence through MTUsubtrees interleavely:
// LOP3_0: (p,s0,s2,s4,s6, LOP3_1: (p,s1,s3,s5,s7) - it causes creating inefficiently
// sequences of LOP3s.
fn get_small_tree_with_one_depth<T>(
    circuit: &VBinOpCircuit<T>,
    wire_index: T,
    cov: &[T],
) -> [Option<T>; 7]
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    let input_len_t = circuit.input_len;
    let input_len = usize::try_from(input_len_t).unwrap();
    let gates = &circuit.gates;
    // by default is subtree is zero if no coverage supplied
    let root_subtree_index = if wire_index >= circuit.input_len {
        cov[usize::try_from(wire_index).unwrap() - input_len]
    } else {
        T::default() // it doesn't matter what is value
    };
    let mut tree = [None; 7];
    let mut covs = [T::default(); 7];
    let mut cov_changes = [0; 7];
    let mut old_level_start = 0;
    let mut level_start = 1;
    tree[0] = Some(wire_index);
    covs[0] = root_subtree_index;
    //let mut one_depth_subtree_indices = HashSet::new();
    for depth in 1..4 {
        for pos in 0..level_start - old_level_start {
            if let Some(t) = tree[old_level_start + pos] {
                if t >= input_len_t {
                    let gi = usize::try_from(t).unwrap();
                    // by default is subtree is zero if no coverage supplied
                    // if no supplied coverage then root_subtree_index == t_subtree_index
                    let old_cov_change = cov_changes[old_level_start + pos];
                    let t_subtree_index = covs[old_level_start + pos];
                    let g = gates[gi - input_len].0;
                    let gi0cov_change = if g.i0 >= circuit.input_len {
                        let gix0 = usize::try_from(g.i0).unwrap() - input_len;
                        let gi0_subtree_index = cov[gix0];
                        let new_cov_change =
                            old_cov_change + usize::from(gi0_subtree_index != t_subtree_index);
                        if depth < 3 {
                            covs[level_start + (pos << 1)] = gi0_subtree_index;
                            cov_changes[level_start + (pos << 1)] = new_cov_change;
                        }
                        new_cov_change
                    } else {
                        old_cov_change
                    };
                    let gi1cov_change = if g.i1 >= circuit.input_len {
                        let gix1 = usize::try_from(g.i1).unwrap() - input_len;
                        let gi1_subtree_index = cov[gix1];
                        let new_cov_change =
                            old_cov_change + usize::from(gi1_subtree_index != t_subtree_index);
                        if depth < 3 {
                            covs[level_start + (pos << 1) + 1] = gi1_subtree_index;
                            cov_changes[level_start + (pos << 1) + 1] = new_cov_change;
                        }
                        new_cov_change
                    } else {
                        old_cov_change
                    };
                    if depth < 3 && old_cov_change <= 1 && gi0cov_change <= 1 && gi1cov_change <= 1
                    {
                        tree[level_start + (pos << 1)] = Some(g.i0);
                        tree[level_start + (pos << 1) + 1] = Some(g.i1);
                    }
                    if gi0cov_change > 1 || gi1cov_change > 1 {
                        tree[old_level_start + pos] = None;
                    }
                }
            }
        }
        old_level_start = level_start;
        level_start += level_start + 1;
    }
    tree
}

fn get_small_tree<T>(circuit: &VBinOpCircuit<T>, wire_index: T) -> [Option<T>; 7]
where
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
{
    get_small_tree_with_cov(circuit, wire_index, None)
}

// special area of MTUsubtree that used to join with other MTUblocks.
#[derive(Clone, Debug, PartialEq, Eq)]
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

//
#[derive(Clone, Copy, Debug)]
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
    cost + all_nodes.count_ones() as usize
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
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
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
    fn gen_lop3nodes_and_cost(
        &mut self,
        circuit: &VBinOpCircuit<T>,
        lop3nodes: &mut [LOP3Node<T>],
        cov: &[T],
    ) -> usize {
        // IDEA:
        // use form closed form of area by nodes (for example: (R,C00,C01,C10,C11)
        // or (R,C00,C01,C1)) and it can be without root.
        // if some nodes are node supplied then add.
        // NEXT THOUGHT: include (minimal) depth of nodes in MTUarea to calculate costs
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let tree = get_small_tree_with_cov(circuit, self.root, Some(cov));
        assert!(self.root >= circuit.input_len);
        let root_subtree_index = cov[usize::try_from(self.root).unwrap() - input_len];
        let gates = &circuit.gates;
        let node_mask = tree
            .into_iter()
            .map(|t| {
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
        let do_add_c1_node = if tree[0].is_some() {
            if (all_nodes & 0b0000110) == 0 {
                let mut farest = (3..7)
                    .filter_map(|x| {
                        if ((all_nodes >> x) & 1) != 0 {
                            tree[x]
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                farest.sort();
                farest.dedup();
                farest.len() == 4
            } else {
                false
            }
        } else {
            false
        };
        let all_nodes = all_nodes | (u8::from(do_add_c1_node) << 2);
        let (_, nonfarest_nodes) = farest_nonfarest_nodes_from_mask(all_nodes);
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
                            0b0000100 => {
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
                            0b0000000 => {
                                let t2_0 = gates[gidx].0.i0;
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
                                let mut args_out = [args[0]; 3];
                                for (j, arg) in args.into_iter().enumerate() {
                                    args_out[j] = arg;
                                }
                                lop3nodes[gidx] = LOP3Node {
                                    args: args_out,
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
        let extra_nodes = all_nodes & !node_mask_u8;
        // update nodes
        for i in 0..7 {
            if ((extra_nodes >> i) & 1) != 0 {
                let t = tree[i].unwrap();
                if t >= circuit.input_len {
                    let t_subtree_index = cov[usize::try_from(t).unwrap() - input_len];
                    if t_subtree_index == root_subtree_index {
                        self.nodes.push((t, vec![]));
                    }
                }
            }
        }
        let cost = cost + usize::from(check_c1_node);
        self.cost = cost;
        cost
    }

    // and improve - fix other TouchNodes to make better result if possible
    fn improve_and_optimize_and_gen_lop3nodes(
        &mut self,
        circuit: &VBinOpCircuit<T>,
        lop3nodes: &mut [LOP3Node<T>],
        cov: &[T],
        subtrees: &[SubTree<T>],
        circuit_outputs: &HashSet<T>,
    ) {
        let input_len = usize::try_from(circuit.input_len).unwrap();
        let root_subtree_index = cov[usize::try_from(self.root).unwrap() - input_len];
        let tree = get_small_tree_with_cov(circuit, self.root, Some(cov));
        // prefereed nodes are all nodes in MTUarea (in tree except circuit inputs and
        // nodes from other MTUsubtrees).
        let preferred_nodes = tree
            .iter()
            .filter_map(|x| {
                if let Some(x) = *x {
                    if x >= circuit.input_len
                        && cov[usize::try_from(x).unwrap() - input_len] == root_subtree_index
                    {
                        Some(x)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let all_touch_nodes = {
            let mut touch_nodes = self
                .nodes
                .iter()
                .map(|(_, touch_nodes)| touch_nodes.clone())
                .flatten()
                .collect::<Vec<_>>();
            touch_nodes.sort();
            touch_nodes.dedup();
            touch_nodes
        };
        let keep_root = circuit_outputs.contains(&self.root);
        let iter_num = self.nodes.len();
        let mut checked_mtunode = HashSet::new();
        for _ in 0..iter_num {
            let mut changes = false;
            let mtunode = {
                let mut choosen = None;
                for (mtunode, _) in &self.nodes {
                    if *mtunode == self.root && keep_root {
                        continue; // do not reduce root of MTUarea
                    }
                    if checked_mtunode.contains(mtunode) {
                        continue;
                    }
                    choosen = Some(*mtunode);
                }
                if let Some(mtunode) = choosen {
                    mtunode
                } else {
                    break;
                }
            };
            if let Some(p) = self.nodes.iter().position(|(n, _)| mtunode == *n) {
                let touch_nodes = self.nodes[p].1.clone();
                let mut all_touch_nodes_removed = true;
                let mut new_variants = vec![];
                for (tni, touch_node) in touch_nodes.iter().enumerate() {
                    let gi = usize::try_from(*touch_node).unwrap() - input_len;
                    let required_args = lop3nodes[gi]
                        .args
                        .iter()
                        .filter(|x| {
                            if **x >= circuit.input_len {
                                let gix = usize::try_from(**x).unwrap() - input_len;
                                cov[gix] != root_subtree_index
                            } else {
                                true
                            }
                        })
                        .copied()
                        .collect::<Vec<_>>();
                    let variants = find_best_lop3node_variants(
                        circuit,
                        lop3nodes,
                        cov,
                        subtrees,
                        circuit_outputs,
                        *touch_node,
                        &preferred_nodes,
                        &required_args,
                    );
                    // find variant without mtunode and other mtunode
                    let mut choosen = None;
                    for (vi, variant) in variants.iter().enumerate() {
                        if variant.args.iter().all(|a| {
                            *a != mtunode &&
                            // and if any argument is other mtunode enabled by touch nodes or
                            (self.nodes.iter().any(|(mtunode2, _)| *mtunode2 == *a) ||
                            // circuit input or node from other MTUarea
                             if *a >= circuit.input_len {
                                let gix = usize::try_from(*a).unwrap() - input_len;
                                cov[gix] != root_subtree_index
                             } else {
                                 true
                             }
                        )
                        }) {
                            choosen = Some(vi);
                            break;
                        }
                    }
                    if let Some(choosen) = choosen {
                        new_variants.push((tni, variants[choosen].clone()));
                    } else {
                        all_touch_nodes_removed = false;
                        break;
                    }
                }
                if all_touch_nodes_removed {
                    for (tni, variant) in new_variants {
                        let touch_node = touch_nodes[tni];
                        let gi = usize::try_from(touch_node).unwrap() - input_len;
                        // if found then replace
                        lop3nodes[gi] = variant.clone();
                    }
                    changes = true;
                }
            }
            // update MTUarea
            if changes {
                self.nodes = vec![]; // clear nodes
                for touch_node in &all_touch_nodes {
                    let gi = usize::try_from(*touch_node).unwrap() - input_len;
                    for arg in &lop3nodes[gi].args {
                        if *arg >= circuit.input_len {
                            let arggi = usize::try_from(*arg).unwrap() - input_len;
                            if cov[arggi] == root_subtree_index {
                                self.add_node(*arg, *touch_node);
                            }
                        }
                    }
                }
                if keep_root && self.nodes.iter().all(|(n, _)| *n != self.root) {
                    // add root
                    self.nodes.push((self.root, vec![]));
                }
            }
            checked_mtunode.insert(mtunode);
        }
        // end
        self.gen_lop3nodes_and_cost(circuit, lop3nodes, cov);
    }

    fn farest_nonfarest_nodes(&self, circuit: &VBinOpCircuit<T>) -> (Vec<T>, Vec<T>) {
        let tree = get_small_tree(circuit, self.root);
        let node_mask = tree
            .into_iter()
            .map(|t| {
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
    fn is_way(self, second: bool) -> bool {
        (self.0 & (1 << u32::from(second))) != 0
    }
    #[inline]
    fn is_empty(self) -> bool {
        self.0 == 0
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
    let tree = get_small_tree_with_one_depth(circuit, wire_index, coverage);
    // let tree = get_small_tree(circuit, wire_index);
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
    let subtree_index = coverage[usize::try_from(subtree.root()).unwrap() - input_len];
    let mut mtuareas_indices = HashSet::new();
    for node in subtree
        .gates()
        .iter()
        .map(|(x, _)| *x)
        .chain(std::iter::once(subtree.root()))
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
    T: Clone + Copy + Ord + PartialEq + Eq + Hash,
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
            mtuareas[i].improve_and_optimize_and_gen_lop3nodes(
                &circuit,
                &mut lop3nodes,
                &cov,
                &subtrees,
                &circuit_outputs,
            );
            let (farest_nodes, nonfarest_nodes) = mtuareas[i].farest_nonfarest_nodes(&circuit);
            // get nonfarest nodes
            for nidx in subtree
                .gates()
                .iter()
                .map(|(x, _)| *x)
                .chain(std::iter::once(subtree.root()))
                // skip all nonfarest nodes in MTUAreaview
                .filter(|nidx| nonfarest_nodes.iter().all(|x| *x != *nidx))
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
        Self::from_lop3nodes(circuit, lop3enableds, lop3nodes)
        // optimize can be called later
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vcircuit::*;
    use crate::VNegs;
    use gatesim::Gate;

    fn vgate<T: Clone + Copy>(
        func: VLOP3GateFunc,
        i0: T,
        i1: T,
        i2: T,
        negs: VNegs,
    ) -> VLOP3Gate<T> {
        VLOP3Gate {
            i0,
            i1,
            i2,
            func,
            negs,
        }
    }

    fn vgate_and<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::And, i0, i1, i1, negs)
    }
    fn vgate_or<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::Or, i0, i1, i1, negs)
    }
    fn vgate_xor<T: Clone + Copy>(i0: T, i1: T, negs: VNegs) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::Xor, i0, i1, i1, negs)
    }
    fn vgate_lop3<T: Clone + Copy>(i0: T, i1: T, i2: T, f: u8) -> VLOP3Gate<T> {
        vgate(VLOP3GateFunc::LOP3(f), i0, i1, i2, NoNegs)
    }

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
                        args: [12, 8, 9],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 2
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
                        args: [12, 8, 9],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 5
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
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let lop3nodes = vec![LOP3Node::default(); gates.len()];
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

    // return LOP3nodes from MTUarea to check, nodes in MTUarea, cost of MTUarea
    fn call_mtuarea_gen_lop3nodes_and_cost(
        mtuarea_root: u32,
        mtuarea_nodes: Vec<u32>,
        circuit: VBinOpCircuit<u32>,
        nodes_to_check: Vec<u32>,
    ) -> (Vec<LOP3Node<u32>>, Vec<u32>, usize) {
        let subtrees = circuit.subtrees();
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        println!("Coverage: {:?}", cov);
        let mut mtuarea = MTUArea {
            root: mtuarea_root,
            nodes: mtuarea_nodes
                .into_iter()
                .map(|x| (x, vec![]))
                .collect::<Vec<_>>(),
            cost: 0,
        };
        let input_len = circuit.input_len as usize;
        let mut lop3nodes = vec![LOP3Node::default(); circuit.gates.len() - input_len];
        let cost = mtuarea.gen_lop3nodes_and_cost(&circuit, &mut lop3nodes, &cov);
        let lop3nodes = nodes_to_check
            .into_iter()
            .map(|x| lop3nodes[(x as usize) - input_len].clone())
            .collect::<Vec<_>>();
        let mtuarea_new_nodes = mtuarea
            .nodes
            .into_iter()
            .map(|(x, _)| x)
            .collect::<Vec<_>>();
        (lop3nodes, mtuarea_new_nodes, cost)
    }

    #[test]
    fn test_mtuarea_gen_lop3nodes_and_cost() {
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                // MTU area
                vbgate_and(0, 1, NoNegs),      // 4
                vbgate_and(2, 3, NoNegs),      // 5
                vbgate_or(1, 3, NoNegs),       // 6
                vbgate_xor(0, 3, NoNegs),      // 7
                vbgate_and(4, 5, NoNegs),      // 8
                vbgate_or(6, 7, NegOutput),    // 9
                vbgate_xor(5, 7, NegOutput),   // 10
                vbgate_or(4, 7, NegInput1),    // 11
                vbgate_or(8, 9, NegInput1),    // 12
                vbgate_xor(10, 11, NoNegs),    // 13
                vbgate_and(12, 13, NegOutput), // 14
                // next MTU block
                vbgate_or(14, 1, NoNegs),      // 15
                vbgate_or(2, 14, NegInput1),   // 16
                vbgate_xor(14, 3, NoNegs),     // 17
                vbgate_and(15, 16, NegInput1), // 18
            ],
            outputs: vec![(17, false), (18, false)],
        };
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [12, 13, 12],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![12, 13, 14],
                7
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![12, 13, 14],
                circuit.clone(),
                vec![12, 13, 14]
            )
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [12, 13, 12],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![13, 14, 12],
                7
            ),
            call_mtuarea_gen_lop3nodes_and_cost(14, vec![13, 14], circuit.clone(), vec![13, 14])
        );
        assert_eq!(
            (
                std::iter::repeat(LOP3Node {
                    args: [0, 0, 0],
                    tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                    mtu_cost: 0,
                })
                .take(3)
                .chain(std::iter::once(LOP3Node {
                    args: [8, 9, 13],
                    tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                }))
                .collect::<Vec<_>>(),
                vec![9, 8, 13, 14],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![9, 8, 13, 14],
                circuit.clone(),
                vec![9, 8, 13, 14]
            )
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [8, 9, 13],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![9, 14, 13, 8],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(14, vec![9, 14], circuit.clone(), vec![9, 14])
        );
        assert_eq!(
            (
                std::iter::repeat(LOP3Node {
                    args: [0, 0, 0],
                    tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                    mtu_cost: 0,
                })
                .take(3)
                .chain(std::iter::once(LOP3Node {
                    args: [12, 10, 11],
                    tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                    mtu_cost: MTU_COST_BASE + 1,
                }))
                .collect::<Vec<_>>(),
                vec![11, 10, 12, 14],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![11, 10, 12, 14],
                circuit.clone(),
                vec![11, 10, 12, 14]
            )
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [12, 10, 11],
                        tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![10, 14, 12, 11],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(14, vec![10, 14], circuit.clone(), vec![10, 14])
        );
        // checking splitting farest nodes into two parts
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [10, 11, 10],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    },
                    LOP3Node {
                        args: [8, 9, 13],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![11, 8, 9, 10, 14, 13],
                10
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![11, 8, 9, 10, 14],
                circuit.clone(),
                vec![12, 13, 14]
            )
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [8, 9, 8],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    },
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [12, 10, 11],
                        tree_paths: to_paths([3, 0, 3, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![11, 8, 9, 10, 12, 14],
                11
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![11, 8, 9, 10, 12, 14],
                circuit.clone(),
                vec![12, 13, 14]
            )
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [8, 9, 8],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    },
                    LOP3Node {
                        args: [10, 11, 10],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![11, 8, 9, 10, 12, 13],
                10
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                14,
                vec![11, 8, 9, 10, 12, 13],
                circuit.clone(),
                vec![12, 13]
            )
        );
        // with three same farest nodes
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                // MTU area
                vbgate_and(0, 1, NoNegs),      // 4
                vbgate_and(2, 3, NoNegs),      // 5
                vbgate_or(1, 3, NoNegs),       // 6
                vbgate_xor(0, 3, NoNegs),      // 7
                vbgate_and(4, 5, NoNegs),      // 8
                vbgate_or(6, 7, NegOutput),    // 9
                vbgate_or(4, 7, NegInput1),    // 10
                vbgate_or(8, 9, NegInput1),    // 11
                vbgate_xor(9, 10, NoNegs),     // 12
                vbgate_and(11, 12, NegOutput), // 13
                // next MTU block
                vbgate_or(13, 1, NoNegs),      // 14
                vbgate_or(2, 13, NegInput1),   // 15
                vbgate_xor(13, 3, NoNegs),     // 16
                vbgate_and(14, 15, NegInput1), // 17
            ],
            outputs: vec![(16, false), (17, false)],
        };
        // checking sharing farest nodes by one part
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [8, 9, 10],
                        tree_paths: to_paths([3, 3, 3, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![8, 9, 10, 13],
                10
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                13,
                vec![8, 9, 10, 13],
                circuit.clone(),
                vec![11, 12, 13]
            )
        );
        //
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                // MTU area
                vbgate_and(0, 1, NoNegs),    // 4
                vbgate_or(2, 3, NegOutput),  // 5
                vbgate_xor(1, 3, NegOutput), // 6
                vbgate_or(0, 3, NegInput1),  // 7
                vbgate_or(4, 5, NegInput1),  // 8
                vbgate_xor(6, 7, NoNegs),    // 9
                vbgate_and(8, 9, NegOutput), // 10
                // next MTU block
                vbgate_or(10, 1, NoNegs),      // 11
                vbgate_or(2, 10, NegInput1),   // 12
                vbgate_xor(10, 3, NoNegs),     // 13
                vbgate_and(11, 12, NegInput1), // 14
            ],
            outputs: vec![(13, false), (14, false)],
        };
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [4, 5, 4],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    },
                    LOP3Node {
                        args: [6, 7, 6],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![7, 4, 5, 6, 8, 9],
                10
            ),
            call_mtuarea_gen_lop3nodes_and_cost(
                10,
                vec![7, 4, 5, 6, 8, 9],
                circuit.clone(),
                vec![8, 9]
            )
        );
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                // MTU area
                vbgate_and(0, 1, NoNegs),   // 4
                vbgate_or(2, 3, NegOutput), // 5
                vbgate_or(4, 5, NegInput1), // 6
                vbgate_xor(6, 3, NoNegs),   // 7
                // next MTU block
                vbgate_or(7, 1, NoNegs),     // 8
                vbgate_or(2, 7, NegInput1),  // 9
                vbgate_xor(7, 3, NoNegs),    // 10
                vbgate_and(8, 9, NegInput1), // 11
            ],
            outputs: vec![(10, false), (11, false)],
        };
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [4, 5, 4],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    },
                    LOP3Node {
                        args: [6, 3, 6],
                        tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![4, 6, 7, 5],
                10
            ),
            call_mtuarea_gen_lop3nodes_and_cost(7, vec![4, 6, 7], circuit.clone(), vec![4, 6, 7])
        );
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [4, 5, 3],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![4, 7, 5],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(7, vec![4, 7], circuit.clone(), vec![4, 6, 7])
        );
        // with lower subtree - to check testing whether nodes in same subtree as root.
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                vbgate_xor(0, 3, NoNegs), // 4
                // MTU area
                vbgate_and(1, 2, NoNegs),   // 5
                vbgate_or(3, 4, NegOutput), // 6
                vbgate_or(5, 6, NegInput1), // 7
                vbgate_xor(7, 4, NoNegs),   // 8
                // next MTU block
                vbgate_or(8, 2, NoNegs),      // 9
                vbgate_or(3, 8, NegInput1),   // 10
                vbgate_xor(8, 4, NoNegs),     // 11
                vbgate_and(9, 10, NegInput1), // 12
            ],
            outputs: vec![(11, false), (12, false)],
        };
        assert_eq!(
            (
                vec![
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [0, 0, 0],
                        tree_paths: LOP3_SUBTREE_PATHS_DEFAULT,
                        mtu_cost: 0,
                    },
                    LOP3Node {
                        args: [5, 6, 4],
                        tree_paths: to_paths([3, 3, 0, 0, 0, 0, 0]),
                        mtu_cost: MTU_COST_BASE + 1,
                    }
                ],
                vec![5, 8, 6],
                8
            ),
            call_mtuarea_gen_lop3nodes_and_cost(8, vec![5, 8], circuit.clone(), vec![5, 7, 8])
        );
    }

    fn lop3node_1(arg0: u32, arg1: u32, arg2: u32) -> LOP3Node<u32> {
        LOP3Node {
            args: [arg0, arg1, arg2],
            tree_paths: to_paths([3, 0, 0, 0, 0, 0, 0]),
            mtu_cost: MTU_COST_BASE + 1,
        }
    }

    fn lop3node_mmask(arg0: u32, arg1: u32, arg2: u32, move_mask: u8) -> LOP3Node<u32> {
        let mut tree_paths = LOP3_SUBTREE_PATHS_DEFAULT;
        for i in 0..7 {
            tree_paths[i] = PathMove(u8::from(((move_mask >> i) & 1) != 0) * 3);
        }
        LOP3Node {
            args: [arg0, arg1, arg2],
            tree_paths,
            mtu_cost: MTU_COST_BASE + 1,
        }
    }

    fn lop3node_mmask_cost(
        arg0: u32,
        arg1: u32,
        arg2: u32,
        move_mask: u8,
        cost: usize,
    ) -> LOP3Node<u32> {
        let mut tree_paths = LOP3_SUBTREE_PATHS_DEFAULT;
        for i in 0..7 {
            tree_paths[i] = PathMove(u8::from(((move_mask >> i) & 1) != 0) * 3);
        }
        LOP3Node {
            args: [arg0, arg1, arg2],
            tree_paths,
            mtu_cost: cost,
        }
    }

    fn call_vlop3circuit_from_lopnodes(
        circuit: VBinOpCircuit<u32>,
        lop3nodes_and_enableds: Vec<(LOP3Node<u32>, bool)>,
    ) -> VLOP3Circuit<u32> {
        let (lop3nodes, enableds): (Vec<_>, Vec<_>) = lop3nodes_and_enableds.into_iter().unzip();
        VLOP3Circuit::from_lop3nodes(circuit, enableds, lop3nodes)
    }

    #[test]
    fn test_vlop3circuit_from_lop3nodes() {
        let a0 = 0b11110000u8;
        let a1 = 0b11001100u8;
        let a2 = 0b10101010u8;
        assert_eq!(
            VLOP3Circuit {
                input_len: 2,
                gates: vec![
                    vgate_and(0, 1, NoNegs),
                    vgate_and(0, 2, NegInput1),
                    vgate_and(0, 3, NegOutput),
                    vgate_or(2, 3, NoNegs),
                    vgate_or(2, 1, NegInput1),
                    vgate_or(0, 1, NegOutput),
                    vgate_xor(4, 2, NoNegs),
                    vgate_xor(0, 1, NegInput1),
                    vgate_xor(0, 1, NegOutput),
                ],
                outputs: vec![
                    (0, false),
                    (2, true),
                    (3, false),
                    (4, true),
                    (5, false),
                    (6, true),
                    (7, false),
                    (8, true),
                    (9, false),
                    (10, true),
                ],
            },
            VLOP3Circuit::from_lop3nodes(
                VBinOpCircuit {
                    input_len: 2,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),
                        vbgate_and(0, 2, NegInput1),
                        vbgate_and(0, 3, NegOutput),
                        vbgate_or(2, 3, NoNegs),
                        vbgate_or(2, 1, NegInput1),
                        vbgate_or(0, 1, NegOutput),
                        vbgate_xor(4, 2, NoNegs),
                        vbgate_xor(0, 1, NegInput1),
                        vbgate_xor(0, 1, NegOutput),
                    ],
                    outputs: vec![
                        (0, false),
                        (2, true),
                        (3, false),
                        (4, true),
                        (5, false),
                        (6, true),
                        (7, false),
                        (8, true),
                        (9, false),
                        (10, true),
                    ],
                },
                std::iter::repeat(true).take(9).collect::<Vec<_>>(),
                std::iter::repeat(lop3node_1(0, 1, 0))
                    .take(9)
                    .collect::<Vec<_>>(),
            )
        );
        // LOP3Nodes
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, !((a0 & a1) | (a0 & !a2))),
                    vgate_lop3(0, 1, 2, !(a0 & a1) ^ (a2 & !a1)),
                    vgate_lop3(0, 1, 2, !(a0 | a1) | !(a1 ^ !a2)),
                    vgate_lop3(2, 1, 0, !((a2 | !a0) & (a0 ^ a1))),
                ],
                outputs: vec![(3, false), (4, false), (5, true), (6, false)],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),      // 3
                        vbgate_and(0, 2, NegInput1),   // 4
                        vbgate_or(3, 4, NegOutput),    // 5
                        vbgate_and(0, 1, NegOutput),   // 6
                        vbgate_and(2, 1, NegInput1),   // 7
                        vbgate_xor(6, 7, NoNegs),      // 8
                        vbgate_or(0, 2, NegOutput),    // 9
                        vbgate_xor(1, 2, NegInput1),   // 10
                        vbgate_or(9, 10, NegInput1),   // 11
                        vbgate_or(0, 2, NegInput1),    // 12
                        vbgate_xor(2, 1, NoNegs),      // 13
                        vbgate_and(12, 13, NegOutput), // 14
                    ],
                    outputs: vec![(5, false), (8, false), (11, true), (14, false)],
                },
                vec![
                    (lop3node_1(0, 1, 0), false),               // 3
                    (lop3node_1(0, 2, 0), false),               // 4
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 5
                    (lop3node_1(0, 1, 0), false),               // 6
                    (lop3node_1(2, 1, 2), false),               // 7
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 8
                    (lop3node_1(0, 2, 0), false),               // 9
                    (lop3node_1(1, 2, 1), false),               // 10
                    (lop3node_mmask(0, 1, 2, 0b0000111), true), // 11
                    (lop3node_1(0, 2, 0), false),               // 12
                    (lop3node_1(2, 1, 2), false),               // 13
                    (lop3node_mmask(2, 1, 0, 0b0000111), true), // 14
                ],
            )
        );
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_lop3(0, 1, 2, a2 | (a0 & !a1)),
                    vgate_lop3(0, 1, 2, (a0 & a1) | (a2 & !a1)),
                    vgate_lop3(0, 1, 2, a1 | !(a2 & (a0 & !a1))),
                    vgate_lop3(2, 0, 1, ((a0 & !a2) & a1) | !a0),
                    vgate_lop3(0, 1, 2, (a0 & !(a2 | a1)) ^ ((a0 ^ a1) & !a2)),
                    vgate_lop3(3, 4, 5, !(a2 | (a0 ^ a1))),
                    vgate_lop3(6, 7, 3, a2 & !(a0 & a1)),
                    vgate_lop3(0, 1, 2, (a2 & !a1) | !a0),
                    vgate_lop3(0, 1, 2, ((a0 & !a1) ^ (a2 & !a1)) | !(a0 & !a1)),
                    vgate_lop3(1, 2, 1, !(a0 & a1) ^ a1),
                    vgate_lop3(4, 5, 7, ((a0 & a1) | (a1 & !a2)) ^ ((a2 & !a0) | (a0 ^ a1))),
                ],
                outputs: vec![
                    (8, true),
                    (9, false),
                    (10, true),
                    (11, false),
                    (12, true),
                    (13, true)
                ],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NegInput1), // 3
                        vbgate_or(2, 3, NoNegs),     // 4
                        //
                        vbgate_and(0, 1, NoNegs),    // 5
                        vbgate_and(2, 1, NegInput1), // 6
                        vbgate_xor(5, 6, NoNegs),    // 7
                        //
                        vbgate_and(0, 1, NegInput1), // 8
                        vbgate_and(2, 8, NoNegs),    // 9
                        vbgate_or(1, 9, NegInput1),  // 10
                        //
                        vbgate_and(2, 1, NegInput1), // 11
                        vbgate_and(11, 0, NoNegs),   // 12
                        vbgate_or(12, 2, NegInput1), // 13
                        //
                        vbgate_xor(0, 1, NoNegs),     // 14
                        vbgate_or(2, 1, NegOutput),   // 15
                        vbgate_and(0, 15, NoNegs),    // 16
                        vbgate_and(14, 2, NegInput1), // 17
                        vbgate_xor(16, 17, NoNegs),   // 18
                        //
                        vbgate_xor(4, 7, NoNegs),     // 19
                        vbgate_or(10, 19, NegOutput), // 20
                        vbgate_and(13, 18, NoNegs),   // 21
                        vbgate_and(4, 21, NegInput1), // 22
                        //
                        vbgate_and(2, 1, NegInput1), // 23
                        vbgate_or(23, 0, NegInput1), // 24
                        //
                        vbgate_xor(3, 6, NoNegs),    // 25
                        vbgate_or(25, 3, NegInput1), // 26
                        //
                        vbgate_and(1, 2, NegOutput), // 27
                        vbgate_xor(27, 2, NoNegs),   // 28
                        //
                        vbgate_and(7, 10, NoNegs),     // 29
                        vbgate_and(10, 18, NegInput1), // 30
                        vbgate_and(18, 7, NegInput1),  // 31
                        vbgate_xor(7, 10, NoNegs),     // 32
                        vbgate_or(29, 30, NoNegs),     // 33
                        vbgate_or(31, 32, NoNegs),     // 34
                        vbgate_xor(33, 34, NoNegs),    // 35
                    ],
                    outputs: vec![
                        (20, true),
                        (22, false),
                        (24, true),
                        (26, false),
                        (28, true),
                        (35, true)
                    ],
                },
                vec![
                    (lop3node_1(0, 1, 0), false),                  // 3
                    (lop3node_mmask(0, 1, 2, 0b0000101), true),    // 4
                    (lop3node_1(0, 1, 0), false),                  // 5
                    (lop3node_1(2, 1, 2), false),                  // 6
                    (lop3node_mmask(0, 1, 2, 0b0000111), true),    // 7
                    (lop3node_1(0, 1, 0), false),                  // 8
                    (lop3node_1(2, 8, 2), false),                  // 9
                    (lop3node_mmask(0, 1, 2, 0b1000101), true),    // 10
                    (lop3node_1(2, 1, 2), false),                  // 11
                    (lop3node_1(11, 0, 11), false),                // 12
                    (lop3node_mmask(2, 0, 1, 0b0001011), true),    // 13
                    (lop3node_1(0, 1, 0), false),                  // 14
                    (lop3node_1(2, 1, 2), false),                  // 15
                    (lop3node_mmask(0, 1, 2, 0b0000101), false),   // 16
                    (lop3node_mmask(0, 1, 2, 0b0000011), false),   // 17
                    (lop3node_mmask(0, 1, 2, 0b0110111), true),    // 18
                    (lop3node_1(4, 7, 4), false),                  // 19
                    (lop3node_mmask(4, 7, 10, 0b0000101), true),   // 20
                    (lop3node_1(13, 18, 13), false),               // 21
                    (lop3node_mmask(13, 18, 4, 0b0000101), true),  // 22
                    (lop3node_1(2, 1, 2), false),                  // 23
                    (lop3node_mmask(0, 1, 2, 0b0000011), true),    // 24
                    (lop3node_1(3, 6, 3), false),                  // 25
                    (lop3node_mmask(0, 1, 2, 0b0011111), true),    // 26
                    (lop3node_1(1, 2, 1), false),                  // 27
                    (lop3node_mmask(1, 2, 1, 0b0000011), true),    // 28
                    (lop3node_1(7, 10, 7), false),                 // 29
                    (lop3node_1(10, 18, 10), false),               // 30
                    (lop3node_1(18, 7, 18), false),                // 31
                    (lop3node_1(7, 10, 7), false),                 // 32
                    (lop3node_mmask(7, 10, 18, 0b0000111), false), // 33
                    (lop3node_mmask(7, 10, 18, 0b0000111), false), // 34
                    (lop3node_mmask(7, 10, 18, 0b1111111), true),  // 35
                ],
            )
        );
        assert_eq!(
            VLOP3Circuit {
                input_len: 3,
                gates: vec![
                    vgate_and(0, 1, NoNegs),    // 3
                    vgate_and(0, 2, NegInput1), // 4
                    vgate_and(1, 2, NegOutput), // 5
                    vgate_or(2, 1, NoNegs),     // 6
                    vgate_or(2, 0, NegInput1),  // 7
                    vgate_or(0, 1, NegOutput),  // 8
                    vgate_xor(0, 2, NoNegs),    // 9
                    vgate_xor(2, 1, NegInput1), // 10
                    vgate_xor(1, 0, NegOutput), // 11
                    vgate_lop3(3, 4, 5, (!(a2 & a0) | (a1 & !a0)) ^ !a2),
                    vgate_lop3(6, 7, 8, !(a0 ^ ((a1 & a2) & (a2 | !a1)))),
                    vgate_lop3(9, 10, 11, a1 | !(!(a1 & a2) & a0)),
                    vgate_lop3(4, 7, 4, !(a0 ^ (a0 & a1))),
                ],
                outputs: vec![(12, false), (13, true), (14, true), (15, false)],
            },
            call_vlop3circuit_from_lopnodes(
                VBinOpCircuit {
                    input_len: 3,
                    gates: vec![
                        vbgate_and(0, 1, NoNegs),    // 3
                        vbgate_and(0, 2, NegInput1), // 4
                        vbgate_and(1, 2, NegOutput), // 5
                        vbgate_or(2, 1, NoNegs),     // 6
                        vbgate_or(2, 0, NegInput1),  // 7
                        vbgate_or(0, 1, NegOutput),  // 8
                        vbgate_xor(0, 2, NoNegs),    // 9
                        vbgate_xor(2, 1, NegInput1), // 10
                        vbgate_xor(1, 0, NegOutput), // 11
                        //
                        vbgate_and(4, 3, NegInput1), // 12
                        vbgate_and(5, 3, NegOutput), // 13
                        vbgate_or(13, 12, NoNegs),   // 14
                        vbgate_and(7, 8, NoNegs),    // 15
                        vbgate_or(8, 7, NegInput1),  // 16
                        vbgate_and(15, 16, NoNegs),  // 17
                        //
                        vbgate_xor(14, 5, NegInput1), // 18
                        vbgate_xor(6, 17, NegOutput), // 19
                        //
                        vbgate_and(10, 11, NegOutput), // 20
                        vbgate_and(20, 9, NoNegs),     // 21
                        vbgate_or(10, 21, NegInput1),  // 22
                        //
                        vbgate_and(4, 7, NoNegs),     // 23
                        vbgate_xor(4, 23, NegOutput), // 24
                    ],
                    outputs: vec![(18, false), (19, true), (22, true), (24, false)],
                },
                vec![
                    (lop3node_1(0, 1, 0), true),                  // 3
                    (lop3node_1(0, 2, 0), true),                  // 4
                    (lop3node_1(1, 2, 1), true),                  // 5
                    (lop3node_1(2, 1, 2), true),                  // 6
                    (lop3node_1(2, 0, 2), true),                  // 7
                    (lop3node_1(0, 1, 0), true),                  // 8
                    (lop3node_1(0, 2, 0), true),                  // 9
                    (lop3node_1(2, 1, 2), true),                  // 10
                    (lop3node_1(1, 0, 1), true),                  // 11
                    (lop3node_1(4, 3, 4), false),                 // 12
                    (lop3node_1(5, 3, 5), false),                 // 13
                    (lop3node_1(12, 13, 12), false),              // 14
                    (lop3node_1(7, 8, 7), false),                 // 15
                    (lop3node_1(8, 7, 8), false),                 // 16
                    (lop3node_1(15, 16, 15), false),              // 17
                    (lop3node_mmask(3, 4, 5, 0b0011011), true),   // 18
                    (lop3node_mmask(6, 7, 8, 0b1100101), true),   // 19
                    (lop3node_1(10, 11, 10), false),              // 20
                    (lop3node_1(20, 9, 20), false),               // 21
                    (lop3node_mmask(9, 10, 11, 0b0100101), true), // 22
                    (lop3node_1(4, 7, 4), false),                 // 23
                    (lop3node_mmask(4, 7, 4, 0b0000101), true),   // 24
                ],
            )
        );
    }

    fn call_improve_and_optimize_and_gen_lop3nodes(
        circuit: VBinOpCircuit<u32>,
        mtuarea: MTUArea<u32>,
        lop3nodes: Vec<LOP3Node<u32>>,
    ) -> (MTUArea<u32>, Vec<LOP3Node<u32>>) {
        println!("Call improve_and_optimize_and_gen_lop3nodes");
        let subtrees = circuit.subtrees();
        println!("Subtrees: {:?}", subtrees);
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        let mut mtuarea = mtuarea.clone();
        let mut lop3nodes = lop3nodes.clone();
        let circuit_outputs = HashSet::from_iter(circuit.outputs.iter().map(|(x, _)| *x));
        mtuarea.improve_and_optimize_and_gen_lop3nodes(
            &circuit,
            &mut lop3nodes,
            &cov,
            &subtrees,
            &circuit_outputs,
        );
        (mtuarea, lop3nodes)
    }

    #[test]
    fn test_improve_and_optimize_and_gen_lop3nodes() {
        let (mtuarea, lop3nodes) = (
            MTUArea {
                root: 4,
                nodes: vec![],
                cost: 0,
            },
            vec![
                lop3node_1(0, 1, 0),                   // 4
                lop3node_1(0, 2, 0),                   // 5
                lop3node_1(1, 4, 1),                   // 6
                LOP3Node::default(),                   // 7
                LOP3Node::default(),                   // 8
                LOP3Node::default(),                   // 9
                lop3node_mmask(0, 1, 2, 0b1000111),    // 10
                lop3node_mmask(0, 1, 3, 0b1001111),    // 11
                lop3node_mmask(0, 1, 2, 0b1000101),    // 12
                LOP3Node::default(),                   // 13
                lop3node_mmask(10, 11, 12, 0b0000101), // 14
            ],
        );
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                vbgate_and(0, 1, NoNegs),    // 4 lop3:10 lop3:11 lop3:12
                vbgate_or(0, 2, NoNegs),     // 5 lop3:10, 2:lop3:5
                vbgate_or(1, 4, NoNegs),     // 6 lop3:10, 2:lop3:10
                vbgate_xor(4, 1, NoNegs),    // 7 lop3:11, 2:lop3:11
                vbgate_or(3, 4, NegInput1),  // 8 lop3:11, 2:lop3:11
                vbgate_and(2, 4, NegInput1), // 9 lop3:12, 2:lop3:12
                vbgate_xor(5, 6, NoNegs),    // 10 lop3:10, 2:lop3:10
                vbgate_xor(7, 8, NoNegs),    // 11 lop3:11, 2:lop3:11
                vbgate_xor(2, 9, NoNegs),    // 12 lop3:12, 2:lop3:12
                vbgate_xor(10, 11, NoNegs),  // 13 lop3:14, 2:lop3:14
                vbgate_xor(12, 13, NoNegs),  // 14 lop3:14, 2:lop3:14
            ],
            outputs: vec![(14, false)],
        };
        assert_eq!(
            (mtuarea.clone(), lop3nodes.clone()),
            call_improve_and_optimize_and_gen_lop3nodes(circuit.clone(), mtuarea, lop3nodes)
        );
        let (mtuarea, lop3nodes) = (
            MTUArea {
                root: 4,
                nodes: vec![(4, vec![10, 11, 12])],
                cost: 0,
            },
            vec![
                lop3node_1(0, 1, 0),                   // 4
                lop3node_1(0, 2, 0),                   // 5
                lop3node_1(1, 4, 1),                   // 6
                lop3node_1(4, 1, 4),                   // 7
                lop3node_1(3, 4, 3),                   // 8
                lop3node_1(2, 4, 2),                   // 9
                lop3node_mmask(1, 4, 5, 0b0000101),    // 10
                lop3node_mmask(1, 3, 4, 0b0000111),    // 11
                lop3node_mmask(2, 4, 2, 0b0000101),    // 12
                LOP3Node::default(),                   // 13
                lop3node_mmask(10, 11, 12, 0b0000101), // 14
            ],
        );
        assert_eq!(
            (
                MTUArea {
                    root: 4,
                    nodes: vec![],
                    cost: 0,
                },
                vec![
                    lop3node_1(0, 1, 0),                                        // 4
                    lop3node_1(0, 2, 0),                                        // 5
                    lop3node_1(1, 4, 1),                                        // 6
                    lop3node_1(4, 1, 4),                                        // 7
                    lop3node_1(3, 4, 3),                                        // 8
                    lop3node_1(2, 4, 2),                                        // 9
                    lop3node_mmask_cost(1, 5, 0, 0b1000101, MTU_COST_BASE + 2), // 10
                    lop3node_mmask(1, 3, 0, 0b1001111),                         // 11
                    lop3node_mmask(2, 0, 1, 0b1000101),                         // 12
                    LOP3Node::default(),                                        // 13
                    lop3node_mmask(10, 11, 12, 0b0000101),                      // 14
                ]
            ),
            call_improve_and_optimize_and_gen_lop3nodes(circuit.clone(), mtuarea, lop3nodes)
        );
        // other sides
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                vbgate_and(0, 1, NoNegs),    // 4 lop3:10 lop3:11 lop3:12
                vbgate_or(0, 2, NoNegs),     // 5 lop3:10, 2:lop3:5
                vbgate_or(4, 1, NoNegs),     // 6 lop3:10, 2:lop3:10
                vbgate_xor(4, 1, NoNegs),    // 7 lop3:11, 2:lop3:11
                vbgate_or(3, 4, NegInput1),  // 8 lop3:11, 2:lop3:11
                vbgate_and(2, 4, NegInput1), // 9 lop3:12, 2:lop3:12
                vbgate_xor(6, 5, NoNegs),    // 10 lop3:10, 2:lop3:10
                vbgate_xor(8, 7, NoNegs),    // 11 lop3:11, 2:lop3:11
                vbgate_xor(9, 2, NoNegs),    // 12 lop3:12, 2:lop3:12
                vbgate_xor(10, 11, NoNegs),  // 13 lop3:14, 2:lop3:14
                vbgate_xor(12, 13, NoNegs),  // 14 lop3:14, 2:lop3:14
            ],
            outputs: vec![(14, false)],
        };
        let (mtuarea, lop3nodes) = (
            MTUArea {
                root: 4,
                nodes: vec![(4, vec![10, 11, 12])],
                cost: 0,
            },
            vec![
                lop3node_1(0, 1, 0),                   // 4
                lop3node_1(0, 2, 0),                   // 5
                lop3node_1(4, 1, 4),                   // 6
                lop3node_1(4, 1, 4),                   // 7
                lop3node_1(3, 4, 3),                   // 8
                lop3node_1(2, 4, 2),                   // 9
                lop3node_mmask(1, 4, 5, 0b0000011),    // 10
                lop3node_mmask(1, 3, 4, 0b0000111),    // 11
                lop3node_mmask(2, 4, 2, 0b0000011),    // 12
                LOP3Node::default(),                   // 13
                lop3node_mmask(10, 11, 12, 0b0000101), // 14
            ],
        );
        assert_eq!(
            (
                MTUArea {
                    root: 4,
                    nodes: vec![],
                    cost: 0,
                },
                vec![
                    lop3node_1(0, 1, 0),                                        // 4
                    lop3node_1(0, 2, 0),                                        // 5
                    lop3node_1(4, 1, 4),                                        // 6
                    lop3node_1(4, 1, 4),                                        // 7
                    lop3node_1(3, 4, 3),                                        // 8
                    lop3node_1(2, 4, 2),                                        // 9
                    lop3node_mmask_cost(5, 1, 0, 0b0001011, MTU_COST_BASE + 2), // 10
                    lop3node_mmask(3, 1, 0, 0b0110111),                         // 11
                    lop3node_mmask(2, 0, 1, 0b0010011),                         // 12
                    LOP3Node::default(),                                        // 13
                    lop3node_mmask(10, 11, 12, 0b0000101),                      // 14
                ]
            ),
            call_improve_and_optimize_and_gen_lop3nodes(
                circuit.clone(),
                mtuarea.clone(),
                lop3nodes.clone()
            )
        );
        // mtuarea root as circuit output
        let mut circuit = circuit.clone();
        circuit.outputs.push((4, false));
        assert_eq!(
            (
                MTUArea {
                    root: 4,
                    nodes: vec![(4, vec![10, 11, 12])],
                    cost: 4,
                },
                lop3nodes.clone()
            ),
            call_improve_and_optimize_and_gen_lop3nodes(circuit.clone(), mtuarea, lop3nodes)
        );
        // new circuit with more complex MTUarea
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                // MTU area to share
                vbgate_and(0, 1, NoNegs),    // 4  *
                vbgate_xor(0, 2, NoNegs),    // 5  *
                vbgate_and(2, 0, NegInput1), // 6  *
                vbgate_or(1, 2, NegOutput),  // 7  *
                vbgate_xor(4, 5, NoNegs),    // 8  *
                vbgate_and(6, 7, NegOutput), // 9  *
                vbgate_or(9, 8, NegInput1),  // 10
                //
                vbgate_and(4, 10, NoNegs),   // 11
                vbgate_or(10, 5, NegInput1), // 12
                vbgate_or(6, 10, NegInput1), // 13
                vbgate_xor(7, 10, NoNegs),   // 14
                //
                vbgate_and(11, 12, NoNegs), // 15
                vbgate_and(13, 14, NoNegs), // 16
                vbgate_and(15, 16, NoNegs), // 17
            ],
            outputs: vec![(17, false), (3, true)],
        };
        let mtuarea = MTUArea {
            root: 10,
            nodes: vec![
                (8, vec![11, 12, 13, 14]),
                (9, vec![11, 12, 13, 14]),
                (10, vec![]),
            ],
            cost: 4,
        };
        let lop3nodes = vec![
            LOP3Node::default(), // 4
            LOP3Node::default(), // 5
            LOP3Node::default(), // 6
            LOP3Node::default(), // 7
            LOP3Node::default(), // 8
            LOP3Node::default(), // 9
            LOP3Node::default(), // 10
            //
            lop3node_mmask(4, 8, 9, 0b0000101), // 11
            lop3node_mmask(5, 8, 9, 0b0000011), // 12
            lop3node_mmask(6, 8, 9, 0b0000101), // 13
            lop3node_mmask(7, 8, 9, 0b0000101), // 14
            //
            LOP3Node::default(), // 15
            LOP3Node::default(), // 16
            LOP3Node::default(), // 17
        ];
        assert_eq!(
            (
                MTUArea {
                    root: 10,
                    nodes: vec![(8, vec![11, 12, 13, 14]), (9, vec![11, 12, 13, 14]),],
                    cost: 4,
                },
                lop3nodes.clone(),
            ),
            call_improve_and_optimize_and_gen_lop3nodes(circuit, mtuarea, lop3nodes)
        );
    }

    fn call_get_small_tree_with_one_depth(
        circuit: VBinOpCircuit<u32>,
        wire_index: u32,
    ) -> [Option<u32>; 7] {
        println!("Call get_small_tree_with_one_depth");
        let subtrees = circuit.subtrees();
        println!("Subtrees: {:?}", subtrees);
        let cov = gen_subtree_coverage(&circuit, &subtrees);
        get_small_tree_with_one_depth(&circuit, wire_index, &cov)
    }

    #[test]
    fn test_small_tree_with_one_depth() {
        let circuit = VBinOpCircuit {
            input_len: 4,
            gates: vec![
                //
                vbgate_and(0, 1, NoNegs), // 4
                vbgate_or(0, 2, NoNegs),  // 5
                vbgate_or(2, 3, NoNegs),  // 6
                vbgate_or(0, 1, NoNegs),  // 7
                vbgate_xor(4, 5, NoNegs), // 8
                vbgate_xor(6, 7, NoNegs), // 9
                vbgate_and(8, 9, NoNegs), // 10
                //
                vbgate_xor(0, 1, NoNegs),   // 11
                vbgate_xor(2, 3, NoNegs),   // 12
                vbgate_and(11, 12, NoNegs), // 13
                //
                vbgate_xor(0, 1, NoNegs),  // 14
                vbgate_xor(0, 14, NoNegs), // 15
                vbgate_xor(0, 15, NoNegs), // 16
                //
                vbgate_or(0, 14, NoNegs),  // 17
                vbgate_or(1, 14, NoNegs),  // 18
                vbgate_or(17, 18, NoNegs), // 19
                //
                vbgate_or(0, 15, NoNegs),  // 20
                vbgate_and(15, 1, NoNegs), // 21
                vbgate_or(20, 21, NoNegs), // 22
                //
                vbgate_or(16, 2, NoNegs),  // 23
                vbgate_and(2, 16, NoNegs), // 24
                vbgate_or(23, 24, NoNegs), // 25
                //
                vbgate_or(22, 2, NoNegs),  // 26
                vbgate_and(2, 25, NoNegs), // 27
                vbgate_or(26, 27, NoNegs), // 28
            ],
            outputs: vec![
                (10, false),
                (13, false),
                (16, false),
                (19, false),
                (22, false),
                (25, false),
                (28, false),
            ],
        };
        assert_eq!(
            [
                Some(10),
                Some(8),
                Some(9),
                Some(4),
                Some(5),
                Some(6),
                Some(7)
            ],
            call_get_small_tree_with_one_depth(circuit.clone(), 10)
        );
        assert_eq!(
            [
                Some(13),
                Some(11),
                Some(12),
                Some(0),
                Some(1),
                Some(2),
                Some(3)
            ],
            call_get_small_tree_with_one_depth(circuit.clone(), 13)
        );
        assert_eq!(
            [Some(14), Some(0), Some(1), None, None, None, None],
            call_get_small_tree_with_one_depth(circuit.clone(), 14)
        );
        assert_eq!(
            [Some(15), Some(0), Some(14), None, None, Some(0), Some(1)],
            call_get_small_tree_with_one_depth(circuit.clone(), 15)
        );
        assert_eq!(
            [Some(16), Some(0), None, None, None, None, None],
            call_get_small_tree_with_one_depth(circuit.clone(), 16)
        );
        assert_eq!(
            [
                Some(19),
                Some(17),
                Some(18),
                Some(0),
                Some(14),
                Some(1),
                Some(14)
            ],
            call_get_small_tree_with_one_depth(circuit.clone(), 19)
        );
        assert_eq!(
            [Some(22), Some(20), Some(21), Some(0), None, None, Some(1)],
            call_get_small_tree_with_one_depth(circuit.clone(), 22)
        );
        assert_eq!(
            [Some(25), Some(23), Some(24), None, Some(2), Some(2), None],
            call_get_small_tree_with_one_depth(circuit.clone(), 25)
        );
        assert_eq!(
            [
                Some(28),
                Some(26),
                Some(27),
                Some(22),
                Some(2),
                Some(2),
                Some(25)
            ],
            call_get_small_tree_with_one_depth(circuit.clone(), 28)
        );
        assert_eq!(
            [Some(26), Some(22), Some(2), None, None, None, None],
            call_get_small_tree_with_one_depth(circuit.clone(), 26)
        );
        assert_eq!(
            [Some(27), Some(2), Some(25), None, None, None, None],
            call_get_small_tree_with_one_depth(circuit.clone(), 27)
        );
    }
}
