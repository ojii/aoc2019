use std::collections::HashMap;
use std::iter::FromIterator;

type Chemicals = HashMap<String, Material>;

struct System {
    cost: usize,
    pool: HashMap<String, usize>,
    chemicals: Chemicals,
}

impl System {
    fn new(chemicals: Chemicals) -> System {
        System {
            cost: 0,
            pool: HashMap::new(),
            chemicals,
        }
    }
}

impl System {
    fn take(&mut self, amount: usize, name: String) {
        let mut have = *self.pool.get(&name).unwrap_or(&0);
        let reactive = self.chemicals.get(&name).unwrap().clone();
        while have < amount {
            have += reactive.produce(self);
        }
        self.pool.insert(name, have - amount);
    }
}

#[derive(Clone)]
enum Material {
    Ore,
    Chemical {
        name: String,
        inputs: Vec<(String, usize)>,
        output: usize,
    },
}

impl Material {
    fn produce(&self, system: &mut System) -> usize {
        match self {
            Material::Ore => {
                system.cost += 1;
                1
            }
            Material::Chemical {
                name,
                inputs,
                output,
            } => {
                for (name, amount) in inputs {
                    system.take(*amount, name.clone())
                }
                *output
            }
        }
    }
}

fn parse(line: &str) -> (String, Material) {
    let parts: Vec<&str> = line.split("=>").map(|p| p.trim()).collect();
    let lhs = parts[0];
    let rhs: Vec<&str> = parts[1].split(' ').collect();
    let output = rhs[0].parse::<usize>().unwrap();
    let name = rhs[1].to_string();
    let inputs = lhs
        .split(',')
        .map(|p| p.trim())
        .map(|p| p.split(' ').collect::<Vec<&str>>())
        .map(|v| (v[1].to_string(), v[0].parse::<usize>().unwrap()))
        .collect();
    (
        name.clone(),
        Material::Chemical {
            name,
            inputs,
            output,
        },
    )
}

pub fn main() {
    let mut chemicals = HashMap::from_iter(INPUT.lines().map(parse));
    chemicals.insert("ORE".to_string(), Material::Ore);
    let mut system = System::new(chemicals);
    system
        .chemicals
        .get(&"FUEL".to_string())
        .unwrap()
        .clone()
        .produce(&mut system);
    println!("{}", system.cost);
    //    let mut chemicals = HashMap::from_iter(INPUT.lines().map(parse));
    //    chemicals.insert("ORE".to_string(), Material::Ore);
    //    let mut system = System::new(chemicals);
    //    let fuel = system.chemicals.get(&"FUEL".to_string()).unwrap().clone();
    //    let mut produced = -1;
    //    while system.cost < 1000_000_000_000 {
    //        fuel.produce(&mut system);
    //        produced += 1;
    //    }
    //    println!("{}", produced);
}

const INPUT: &str = "2 MLVWS, 8 LJNWK => 1 TNFQ
1 BWXQJ => 2 BMWK
1 JMGP, 3 WMJW => 9 JQCF
8 BWXQJ, 10 BJWR => 6 QWSLS
3 PLSH, 1 TNFQ => 6 CTPTW
11 GQDJG, 5 BMWK, 1 FZCK => 7 RQCNC
1 VWSRH => 7 PTGXM
104 ORE => 7 VWSRH
1 PTGXM, 13 WMJW, 1 BJGD => 7 KDHF
12 QWSLS, 3 PLSH, 4 HFBPX, 2 DFTH, 11 BCTRK, 4 JPKWB, 4 MKMRC, 3 XQJZQ => 6 BDJK
1 JQCF, 3 CVSC => 2 KRQHC
128 ORE => 7 QLRXZ
32 CXLWB, 18 TZWD => 1 HFQBG
31 KDHF => 9 BWXQJ
21 MLVWS => 9 LJNWK
3 QLRXZ => 5 CXLWB
3 LQWDR, 2 WSDH, 5 JPKWB, 1 RSTQC, 2 BJWR, 1 ZFNR, 16 QWSLS => 4 JTDT
3 BWXQJ, 14 JMGP => 9 MSTS
1 KXMKM, 2 LFCR => 9 DKWLT
6 CVSC => 3 FWQVP
6 XBVH, 1 HFBPX, 2 FZCK => 9 DFTH
9 MSTS => 2 BCTRK
1 PLSH, 28 MSTS => 2 FDKZ
10 XBVH, 5 BJWR, 2 FWQVP => 6 ZFNR
2 CVSC => 6 XBVH
1 BWXQJ, 2 KXMKM => 3 XQJZQ
1 VWSRH, 1 TZWD => 4 WMJW
14 CTPTW, 19 JMGP => 8 GRWK
13 NLGS, 1 PTGXM, 3 HFQBG => 5 BLVK
2 PTGXM => 7 NLGS
123 ORE => 3 DLPZ
2 ZNRPX, 35 DKWLT => 3 WSDH
1 TZWD, 1 BLVK, 9 BWXQJ => 2 MKDQF
2 DLPZ => 2 MLVWS
8 MKDQF, 4 JQCF, 12 VLMQJ => 8 VKCL
1 KRQHC => 7 BJWR
1 GRWK, 2 FWQVP => 9 LFCR
2 MSTS => 2 GQDJG
132 ORE => 9 TZWD
1 FWQVP => 8 RHKZW
43 FDKZ, 11 BJWR, 63 RHKZW, 4 PJCZB, 1 BDJK, 13 RQCNC, 8 JTDT, 3 DKWLT, 13 JPKWB => 1 FUEL
1 LFCR, 5 DFTH => 1 RSTQC
10 GQDJG => 8 KPTF
4 BWXQJ, 1 MKDQF => 7 JMGP
10 FGNPM, 23 DFTH, 2 CXLWB, 6 KPTF, 3 DKWLT, 10 MKDQF, 1 MJSG, 6 RSTQC => 8 PJCZB
8 VWSRH, 1 DLPZ => 7 BJGD
2 BLVK => 9 HBKH
16 LQWDR, 3 MSTS => 9 HFBPX
1 TNFQ, 29 HFQBG, 4 BLVK => 2 KXMKM
11 CVSC => 8 MJSG
3 LFCR => 6 FGNPM
11 HFQBG, 13 MKDQF => 1 FZCK
11 BWXQJ, 1 QLRXZ, 1 TNFQ => 9 KBTWZ
7 XQJZQ, 6 VKCL => 7 LQWDR
1 LJNWK, 4 HBKH => 1 CVSC
4 PLSH, 2 WSDH, 2 KPTF => 5 JPKWB
1 KPTF => 8 MKMRC
5 NLGS, 2 KDHF, 1 KBTWZ => 2 VLMQJ
4 MLVWS, 1 WMJW, 8 LJNWK => 1 PLSH
3 VKCL => 7 ZNRPX";
