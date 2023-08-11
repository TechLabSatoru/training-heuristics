use proconio::input;
use rand::Rng;

#[inline]
fn get_time() -> f64 {  // sec
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 0.85
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

fn main() {
    get_time();
    // D: 日数
    // c(1-26): 不満足度
    // s(1-D, 1-26): 満足度
    // t(1-D): コンテスト
    // last(1-26): DayXにおける最後に開催された日程

    const ALPHABET: usize = 26;

    input! {
        dd: usize,
        c: [usize; ALPHABET],
        s: [[usize; ALPHABET]; dd],
        // mut t: [usize; dd],
        // mm: usize,
        // dq: [[usize; 2]; mm],
    }

    // println!("{}", dd);
    // println!("{:?}", c);
    // println!("{:?}", s);
    // println!("{:?}", t);

    // ランダムな値を生成
    let mut t: Vec<usize> = vec![0; dd];
    let mut random_generator = rand::thread_rng();
    #[cfg(feature = "seed")]
    {
        seed = 2023;
    }

    for i in 0..dd {
        // let random_value = random_generator.gen_range(1..=ALPHABET); // gen_range(1, 12)の場合は1-12
        let random_value = random_generator.gen_range(1, ALPHABET+1); // gen_range(1, 12)の場合は1-12
        t[i] = random_value;
    }

    // M回試行
    // let mm: usize = 100;

    // // dq
    // let mut dq: Vec<Vec<usize>> = vec![vec![0; 2]; mm];
    // for i in 0..mm {
    //     let random_value = random_generator.gen_range(1..=dd);
    //     // let random_value = random_generator.gen_range(1, dd+1);
    //     dq[i][0] = random_value;
    // }
    // for i in 0..mm {
    //     let random_value = random_generator.gen_range(1..=ALPHABET);
    //     // let random_value = random_generator.gen_range(1, ALPHABET+1);
    //     dq[i][1] = random_value;
    // }
    let mut best_ans: isize = 0;
    // let mut iterate_num: isize = 0;
    while get_time() < 1.98 {
        // iterate_num += 1;
        let mut ans: isize = 0;
        let mut last: [usize; ALPHABET] = Default::default();
        let d: usize = random_generator.gen_range(1, dd+1)-1;
        let q: usize = random_generator.gen_range(1, ALPHABET+1);
        
        // let old_q = t[random_generator.gen_range(1..=dd)-1];
        let old_q = t[d];
        // t[random_generator.gen_range(1..=dd)-1] = random_generator.gen_range(1..=ALPHABET);
        t[d] = q;

        for day in 0..dd {
            
            last[t[day]-1] = day+1;
            
            for alphabet_char in 1..(ALPHABET + 1){
                if alphabet_char == t[day] {
                    ans += s[day][alphabet_char-1] as isize;
                }
                else {
                    ans -= (c[alphabet_char-1]*(day+1 - last[alphabet_char-1])) as isize;
                }
            }
        }

        if best_ans < ans {
            best_ans = ans
        }
        else {
            // t[random_generator.gen_range(1..=dd)-1] = old_q;
            t[d] = old_q;
        }

        // println!("{:?}", ans);
        // println!("{}", best_ans);
    }
    for t_value in t {
        println!("{}", t_value);
    }
    // println!("{}", best_ans);
    // println!("{}", iterate_num)
}
