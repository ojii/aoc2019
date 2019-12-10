use std::collections::VecDeque;
use std::sync::mpsc::channel;

use itertools::Itertools;
use threadpool::ThreadPool;

use crate::vm::{run, Memory, SendOrStore};

pub fn main() {
    let memory = Memory::from(INPUT);
    println!(
        "{}",
        (0..=4i64)
            .permutations(5)
            .map(|amps| {
                let mut output = 0i64;
                for &amp in amps.iter() {
                    output = run(memory.clone(), VecDeque::from(vec![amp, output]), 0i64).1;
                }
                output
            })
            .max()
            .unwrap()
    );

    let pool = ThreadPool::new(5);
    let mut winner = 0;
    for amps in (5..=9i64).permutations(5) {
        let (signalout, signalin) = channel();
        let (aout, bin) = channel();
        let (bout, cin) = channel();
        let (cout, din) = channel();
        let (dout, ein) = channel();
        let (eout, ain) = channel();
        aout.send(amps[1]).unwrap();
        bout.send(amps[2]).unwrap();
        cout.send(amps[3]).unwrap();
        dout.send(amps[4]).unwrap();
        eout.send(amps[0]).unwrap();
        eout.send(0).unwrap();
        let eout = SendOrStore::new(eout);
        let amem = memory.clone();
        let bmem = memory.clone();
        let cmem = memory.clone();
        let dmem = memory.clone();
        let emem = memory.clone();
        pool.execute(move || {
            run(amem, ain, aout);
        });
        pool.execute(move || {
            run(bmem, bin, bout);
        });
        pool.execute(move || {
            run(cmem, cin, cout);
        });
        pool.execute(move || {
            run(dmem, din, dout);
        });
        pool.execute(move || {
            let (_, out) = run(emem, ein, eout);
            signalout.send(*out.store.last().unwrap()).unwrap();
        });
        pool.join();
        let candidate = signalin.recv().unwrap();
        if candidate > winner {
            winner = candidate;
        }
        if pool.panic_count() != 0 {
            panic!("something panicked!");
        }
    }
    println!("{}", winner);
}

const INPUT: &str = "3,8,1001,8,10,8,105,1,0,0,21,42,67,88,101,114,195,276,357,438,99999,3,9,101,3,9,9,1002,9,4,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,101,2,9,9,102,2,9,9,1001,9,5,9,4,9,99,3,9,102,4,9,9,1001,9,3,9,102,4,9,9,101,4,9,9,4,9,99,3,9,101,2,9,9,1002,9,3,9,4,9,99,3,9,101,4,9,9,1002,9,5,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99";
