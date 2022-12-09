use std::time::Instant;

use clap::Parser;
use thousands::Separable;


#[derive(Parser)]
struct Opts {
    #[clap(short, default_value="1")]
    repeat: i32,
    day: Option<i32>,
    #[clap(long)]
    atleast: Option<f32>,
    #[clap(long)]
    per: bool,
    #[clap(short, long)]
    test: bool,
}

mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
static DAYS: &'static [fn(bool, bool)] = &[
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
    // day10::day10,
    // day11::day11,
    // day12::day12,
    // day13::day13,
    // day14::day14,
    // day15::day15,
    // day16::day16,
    // day17::day17,
    // day18::day18,
    // day19::day19,
    // day20::day20,
    // day21::day21,
    // day22::day22,
    // day23::day23,
    // day24::day24,
    // day25::day25,
];

fn main() {
    let opts = Opts::parse();
    assert!(opts.repeat == 1 || opts.atleast == None);
    println!("Hello, world!");
    match opts.day {
        Some(day) => println!("Day {}", day),
        None => println!("All days"),
    }

    if opts.per {
        // Benchmarks per-day.
        let atleast = opts.atleast.unwrap_or(0.5);
        let mut total = 0f64;
        for i in 0..DAYS.len() {
            let started = Instant::now();
            let mut samples = 0;
            while started.elapsed().as_secs_f32() < atleast {
                DAYS[i](opts.test, false);
                samples += 1;
            }
            let elapsed = started.elapsed();
            total += (elapsed / samples).as_secs_f64();
            println!("Day {:2} | {:>7} µs  ({} samples)", i + 1, (elapsed / samples).as_micros().separate_with_commas(), samples);
        }
        println!("Theoretical total: {} ms", total * 1000.0);
    } else {  // Benchmarks the total
        // Running one day or everything?
        let runner: Box<dyn Fn()> = match opts.day {
            Some(day) => Box::new(move || {
                DAYS[day as usize - 1](opts.test, opts.repeat == 1 && opts.atleast.is_none());
            }),
            None => Box::new(|| {
                for run_day in DAYS {
                    run_day(opts.test, false);
                }
            })
        };

        let started = Instant::now();

        let mut repeated = 0u32;
        match opts.atleast {
            None =>
                for _ in 0..opts.repeat {
                    runner();
                    repeated += 1;
                },
            Some(atleast) => {
                while started.elapsed().as_secs_f32() < atleast {
                    runner();
                    repeated += 1;
                }
            },
        }

        let elapsed = started.elapsed();
        println!("Took {:?}  ({} samples)", elapsed / repeated, repeated);
    }
}