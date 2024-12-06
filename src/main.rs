mod args;
mod days;
mod utils;

use args::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    println!("--------- AOC 24 ---------");
    println!("Day: {}  Part: {}", cli.day, if cli.part2 {"2"}else{"1"});
    
    use std::time::Instant;
    let now = Instant::now();
    match cli.day {
        1 => crate::days::d1::day1_main(cli.part2, &cli.extra_args),
        2 => crate::days::d2::day2_main(cli.part2, &cli.extra_args),
        3 => crate::days::d3::day3_main(cli.part2, &cli.extra_args),
        4 => crate::days::d4::day4_main(cli.part2, &cli.extra_args),
        5 => crate::days::d5::day5_main(cli.part2, &cli.extra_args),
        6 => crate::days::d6::day6_main(cli.part2, &cli.extra_args),
        7 => crate::days::d7::day7_main(cli.part2, &cli.extra_args),
        8 => crate::days::d8::day8_main(cli.part2, &cli.extra_args),
        9 => crate::days::d9::day9_main(cli.part2, &cli.extra_args),
        10 => crate::days::d10::day10_main(cli.part2, &cli.extra_args),
        11 => crate::days::d11::day11_main(cli.part2, &cli.extra_args),
        12 => crate::days::d12::day12_main(cli.part2, &cli.extra_args),
        13 => crate::days::d13::day13_main(cli.part2, &cli.extra_args),
        14 => crate::days::d14::day14_main(cli.part2, &cli.extra_args),
        15 => crate::days::d15::day15_main(cli.part2, &cli.extra_args),
        16 => crate::days::d16::day16_main(cli.part2, &cli.extra_args),
        17 => crate::days::d17::day17_main(cli.part2, &cli.extra_args),
        18 => crate::days::d18::day18_main(cli.part2, &cli.extra_args),
        19 => crate::days::d19::day19_main(cli.part2, &cli.extra_args),
        20 => crate::days::d20::day20_main(cli.part2, &cli.extra_args),
        21 => crate::days::d21::day21_main(cli.part2, &cli.extra_args),
        22 => crate::days::d22::day22_main(cli.part2, &cli.extra_args),
        23 => crate::days::d23::day23_main(cli.part2, &cli.extra_args),
        24 => crate::days::d24::day24_main(cli.part2, &cli.extra_args),
        25 => crate::days::d25::day25_main(cli.part2, &cli.extra_args),
        _ => unreachable!("Unknown day: {}", cli.day)
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
