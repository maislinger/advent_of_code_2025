use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let inputs = {
        let mut inputs = Vec::with_capacity(12);
        for day in 1..=12 {
            let path = format!("inputs/day{:02}", day);
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    inputs.push((true, content));
                }
                Err(err) => {
                    println!("failed to read puzzle input for day {:02}: {}", day, err);
                    inputs.push((false, String::new()));
                }
            }
        }
        inputs
    };

    let functions = [
        d01::solve,
        d02::solve,
        d03::solve,
        d04::solve,
        d05::solve,
        d06::solve,
        d07::solve,
        d08::solve,
        d09::solve,
        d10::solve,
        d11::solve,
        d12::solve,
    ];

    let mut results = vec![String::new(); 12];

    let t = std::time::Instant::now();

    // ((&(bool, String), &fn(&str) -> String), &mut String)

    inputs
        .par_iter()
        .zip(functions.par_iter())
        .zip(results.par_iter_mut())
        .map(|(((do_solve, input), f), result)| {
            if *do_solve {
                *result = f(input);
            }
        })
        .count();

    let dt = t.elapsed();

    for (result, do_solve) in results
        .iter()
        .zip(inputs.iter())
        .filter(|(_, (do_solve, _))| *do_solve)
        .map(|(r, (do_solve, _))| (r, *do_solve))
    {
        if do_solve {
            println!("{}", result);
        }
    }

    println!("Elapsed time for all solutions: {:?}", dt);
}
