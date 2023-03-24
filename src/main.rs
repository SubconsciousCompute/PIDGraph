#![allow(non_snake_case)]

mod process;

use std::collections::HashMap;

use crate::process::Win32_Process;

pub use wmi::COMLibrary;
pub use wmi::{Variant, WMIConnection, WMIDateTime, WMIDuration};

pub type Query = Vec<HashMap<String, Variant>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path(r"Root\CIMV2", com_con)?;

    //  Represents a sequence of process events on a computer system running Windows
    let processes: Vec<Win32_Process> = wmi_con.raw_query("SELECT * FROM Win32_Process")?;
    // dbg!(&processes);

    use petgraph::{Direction, Graph};

    // assume that `processes` is the vector of `Win32_Process` structs
    let mut graph = Graph::new();
    let mut nodes = vec![];

    // create a node for each process and store its index
    for process in &processes {
        let node_idx = graph.add_node(process);
        nodes.push((process.ProcessId.unwrap(), node_idx));
    }

    // add edges based on ParentProcessId and CreationDate
    for i in 0..processes.len() {
        let process_i = &processes[i];
        let pid_i = process_i.ProcessId.unwrap();
        let cd_i = process_i.CreationDate.unwrap();

        for j in (i + 1)..processes.len() {
            let process_j = &processes[j];
            let pid_j = process_j.ProcessId.unwrap();
            let cd_j = process_j.CreationDate.unwrap();

            if process_j.ParentProcessId == Some(pid_i) && cd_j > cd_i {
                let node_i = nodes.iter().find(|&&(pid, _)| pid == pid_i).unwrap().1;
                let node_j = nodes.iter().find(|&&(pid, _)| pid == pid_j).unwrap().1;
                graph.add_edge(node_i, node_j, ());
            }
        }
    }

    // now we can answer queries about the graph
    let process_id = 4;
    let node_idx = nodes.iter().find(|&&(pid, _)| pid == process_id).unwrap().1;
    let mut parents = vec![];
    let mut children = vec![];

    // find all parents and children of the given process id
    for neighbor in graph.neighbors_directed(node_idx, Direction::Incoming) {
        parents.push(graph[neighbor]);
    }
    for neighbor in graph.neighbors_directed(node_idx, Direction::Outgoing) {
        children.push(graph[neighbor]);
    }

    println!("{:?}\n\n{:?}", parents, children);

    /*
    let mut tree = ProcessTree::new();

    for process in results {
        tree.add_process(process);
    }

    tree.print();
     */

    //let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_Thread")?;

    //dbg!(&results[0]);

    //  Represents a sequence of thread events on a computer system running Windows
    // let results: Vec<Win32_Thread> = wmi_con.raw_query("SELECT * FROM Win32_Thread")?;
    // dbg!(&results);
    Ok(())
}
