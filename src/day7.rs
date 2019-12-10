use crate::vm::{parse_program, run};
use itertools::Itertools;
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn main() {
    let memory = parse_program(INPUT);
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

    //    let pool = ThreadPool::new(5);
    //    let candidates = Arc::new(Mutex::new(Vec::new()));
    //    for amps in (5..=9i64).permutations(5) {
    //        let (aout, bin) = channel();
    //        let (bout, cin) = channel();
    //        let (cout, din) = channel();
    //        let (dout, ein) = channel();
    //        let (eout, ain) = channel();
    //        eout.send(amps[0]);
    //        eout.send(0);
    //        aout.send(amps[1]);
    //        bout.send(amps[2]);
    //        cout.send(amps[3]);
    //        dout.send(amps[4]);
    //        let mut a = vm::VM::new(memory.clone(), ain, aout);
    //        let mut b = vm::VM::new(memory.clone(), bin, bout);
    //        let mut c = vm::VM::new(memory.clone(), cin, cout);
    //        let mut d = vm::VM::new(memory.clone(), din, dout);
    //        let mut e = vm::VM::new(memory.clone(), ein, eout);
    //        pool.execute(move || {
    //            a.run();
    //        });
    //        pool.execute(move || {
    //            b.run();
    //        });
    //        pool.execute(move || {
    //            c.run();
    //        });
    //        pool.execute(move || {
    //            d.run();
    //        });
    //        let candidates = candidates.clone();
    //        pool.execute(move || {
    //            e.run();
    //            candidates.lock().unwrap().push(ain.recv().unwrap());
    //        });
    //        pool.join();
    //    }
    //    println!("{}", candidates.lock().unwrap().iter().max().unwrap());
}

const INPUT: &str = "3,8,1001,8,10,8,105,1,0,0,21,42,67,88,101,114,195,276,357,438,99999,3,9,101,3,9,9,1002,9,4,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,101,2,9,9,102,2,9,9,1001,9,5,9,4,9,99,3,9,102,4,9,9,1001,9,3,9,102,4,9,9,101,4,9,9,4,9,99,3,9,101,2,9,9,1002,9,3,9,4,9,99,3,9,101,4,9,9,1002,9,5,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99";
