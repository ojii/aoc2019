use itertools::{chain, Itertools};

fn parse(data: &str) -> Vec<i16> {
    data.chars()
        .flat_map(|c| c.to_digit(10))
        .map(|x| x as i16)
        .collect()
}

fn make_pattern(counter: usize) -> impl Iterator<Item = i16> {
    let multiplier = (counter + 1);
    vec![0; multiplier]
        .into_iter()
        .chain(vec![1; multiplier].into_iter())
        .chain(vec![0; multiplier].into_iter())
        .chain(vec![-1; multiplier].into_iter())
        .cycle()
        .skip(1)
}

fn make_patterns(num: usize) -> Vec<Vec<i16>> {
    (0..num)
        .map(|counter| make_pattern(counter).take(num).collect::<Vec<i16>>())
        .collect()
}

fn phase(input: Vec<i16>, patterns: &Vec<Vec<i16>>) -> Vec<i16> {
    patterns
        .iter()
        .map(|pattern| {
            input
                .iter()
                .zip(pattern.iter())
                .map(|(&i, &p)| i * p)
                .sum::<i16>()
                .abs()
                % 10
        })
        .collect()
}

pub fn main() {
    let mut digits = parse(INPUT);
    let patterns = make_patterns(digits.len());

    for _ in 0..100 {
        digits = phase(digits, &patterns);
    }
    println!("{}", digits.iter().take(8).join(""));

    //    let offset = *&INPUT[..7].parse::<usize>().unwrap();
    //    let mut digits = parse(INPUT).repeat(10_000);
    //    let patterns = make_patterns(digits.len());
    //    println!("built patterns");
    //    for num in 0..100 {
    //        digits = phase(digits, &patterns);
    //        println!("phase {} done", num)
    //    }
    //
    //    println!("{}", digits[offset..offset + 8].iter().join(""));
}

const INPUT: &str = "59717513948900379305109702352254961099291386881456676203556183151524797037683068791860532352118123252250974130706958763348105389034831381607519427872819735052750376719383812473081415096360867340158428371353702640632449827967163188043812193288449328058464005995046093112575926165337330100634707115160053682715014464686531460025493602539343245166620098362467196933484413717749680188294435582266877493265037758875197256932099061961217414581388227153472347319505899534413848174322474743198535953826086266146686256066319093589456135923631361106367290236939056758783671975582829257390514211329195992209734175732361974503874578275698611819911236908050184158";
