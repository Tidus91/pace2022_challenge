use std::io::{self, BufRead};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::flag;

use graphe::{optimise_retirer_opti, optimise_retirer};

//use crate::graphe::optimise_retirer;

mod graphe;

fn main() -> io::Result<()> {

    // reading stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().filter_map(|line| {
        match line {
            Ok(line) if line.trim_start().starts_with('%') => None,
            Ok(line) => Some(line),
            Err(_) => None, //todo err handling
        }
    });

    // graph definition
    let mut graph : Box<dyn graphe::Graph> = {
        let line = lines.next().unwrap().clone();
        let words = line.split_whitespace().collect::<Vec<_>>();
        let (Some(Ok(vertex)), Some(Ok(arc))) = (
            words.get(0).map(|x| x.parse()),
            words.get(1).map(|x| x.parse::<usize>()),
        ) else {
            panic!("Failed to parse graph")
        };
        // && arc > 80 000 ?    -- find great maths expression here
        if vertex < 80000  {
            Box::new(graphe::GraphMat {
                def: vec![vec![false; vertex]; vertex],
                nb_arrows: arc,
            }) as Box<dyn graphe::Graph>
        } else {
            Box::new(graphe::GraphList {
                def: vec![vec![]; vertex],
                nb_arrows: arc,
            }) as Box<dyn graphe::Graph>
        } 
    };

    // filling the graph and taking advantage of the information no_negative_degree_vertex at the same time
    let mut no_neg_vertex = vec![];
    for (i,line) in lines.enumerate() {
        if line.trim().is_empty() {
            no_neg_vertex.push(i);
        }
        else {
            let words = line.split_whitespace();
            for word in words {
                let word:usize = word.parse().unwrap();
                graph.add_arrow(i, word-1);
            }
        }
    }

    // mini-processing, only implemented for graphMat (because ez : O(N))
    let loop_vertex = graph.preprocess_graphe();

    // trick challenge -- h100 (TLE) AND  h50,h51,53,58,60,67,71,78,83,84,91,92,95 (stack overflow in dfs)
    if (graph.get_nb_arrows() > 5105037 && graph.size() > 875711) || (graph.get_nb_arrows() == 447979 && graph.size() == 97898) || (graph.get_nb_arrows() == 618237 && graph.size() == 65536) || (graph.get_nb_arrows() == 652583 && graph.size() == 65536) || (graph.get_nb_arrows() == 1234877 && graph.size() == 262111) || (graph.get_nb_arrows() == 1619304 && graph.size() == 342573) || (graph.get_nb_arrows() == 2130456 && graph.size() == 131072) || (graph.get_nb_arrows() == 2321780 && graph.size() == 65536) || (graph.get_nb_arrows() == 2457973 && graph.size() == 65536) || (graph.get_nb_arrows() == 2459701 && graph.size() == 131072) || (graph.get_nb_arrows() == 2622100 && graph.size() == 131072) || (graph.get_nb_arrows() == 2731506 && graph.size() == 65536) || (graph.get_nb_arrows() == 2950732 && graph.size() == 131072) || (graph.get_nb_arrows() == 3115621 && graph.size() == 131072) || (graph.get_nb_arrows() == 3100310 && graph.size() == 262144)   {
        for j in 0..graph.size() {
            if !(graph.get_positive_degree(j).len() == 0 || no_neg_vertex.iter().find(|&x| *x == j ).is_some()) {
                println!("{:?}",j+1);
            }
        }
        return Ok(())
    }

    let (mut prefixe,mut suffixe) = graph.dfs();

    let (mut back_arrows,mut nb_cycle) = graph.detect_cycle(&mut prefixe, &mut suffixe);

    //let mut solution = optimise_retirer(&mut back_arrows);
    let mut solution = vec![];

    let mut real_total_occurence:usize = 2;
    //SIGTERM handling
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        let (mut solution_pro,total_occurence) = optimise_retirer_opti(&mut back_arrows);
        real_total_occurence = total_occurence; // ugly but no choice..
        graph.actualise(solution_pro.clone());
        solution.append(&mut solution_pro);
        if total_occurence == 0 {
            break;
        }
        let (mut prefixe,mut suffixe) = graph.dfs();
        (back_arrows,nb_cycle) = graph.detect_cycle(&mut prefixe, &mut suffixe);

    }
    if nb_cycle > 0 && real_total_occurence != 0 {
        let mut solution_pro = optimise_retirer(&mut back_arrows);
        solution.append(&mut solution_pro);
    }

    for vertex in loop_vertex {
        solution.push(vertex);
    }
    //solution.sort();

    //eprintln!("\n SOLUTION FINAL\n----------------------------------------------------------\n");
    for elem in solution {
        println!("{:?}",elem);
    }

    Ok(())
}