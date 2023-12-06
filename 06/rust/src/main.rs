use rayon::prelude::*;

#[derive(Copy, Clone)]
struct Dist(u64);

#[derive(Clone)]
struct Time(u32);

fn dist(hold: Time, total: Time) -> Dist {
    let tt = Into::<u64>::into(total.0);
    let th = Into::<u64>::into(hold.0);
    Dist((tt - th) * th)
}

fn possible_holds(total: Time, record: Dist) -> u32 {
    let mut possibles = 0;
    for hold in 0..=total.0 {
        if dist(Time(hold), total.clone()).0 > record.0 {
            possibles += 1
        }
    }
    possibles
}

fn possible_holds_opt(total: Time, record: Dist) -> u32 {
    let lowest_working_hold = (0..=total.0)
        .into_iter()
        .find(|hold| dist(Time(*hold), total.clone()).0 > record.0)
        .unwrap();
    let largest_working_hold = (0..=total.0)
        .into_iter()
        .rev()
        .find(|hold| dist(Time(*hold), total.clone()).0 > record.0)
        .unwrap();

    Into::<u32>::into(largest_working_hold) - Into::<u32>::into(lowest_working_hold) + 1
}

fn main() {
    let times: Vec<_> = [41, 96, 88, 94].iter().map(|n| Time(*n)).collect();
    let distances: Vec<_> = [214, 1789, 1127, 1055].iter().map(|n| Dist(*n)).collect();
    let res_p_ = times
        .par_iter()
        .zip(&distances)
        .map(|(t, d)| possible_holds_opt(t.clone(), *d))
        .reduce(|| 1, |a, b| a * b);
    let res_p = times
        .par_iter()
        .zip(distances)
        .map(|(t, d)| possible_holds(t.clone(), d))
        .reduce(|| 1, |a, b| a * b);
    assert_eq!(res_p, res_p_);

    // let res_s = times
    //     .iter()
    //     .zip(distances)
    //     .map(|(t, d)| possible_holds(t.clone(), d))
    //     .reduce(|a, b| a * b).unwrap();
    println!("{res_p_}");
    let times: Vec<_> = [41968894].iter().map(|n| Time(*n)).collect();
    let distances: Vec<_> = [214178911271055].iter().map(|n| Dist(*n)).collect();
    let res_s = times
        .iter()
        .zip(distances)
        .map(|(t, d)| possible_holds_opt(t.clone(), d))
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{res_s}");
}
