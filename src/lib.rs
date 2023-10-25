use std::collections::HashMap;

pub fn fibfaster(n: i64) -> i64 {
    let mut memo = HashMap::<i64,i64>::new();
    let mut result;
    for i in 1..(n+1) {
        if i <= 2 {
            result = 1;
        } else {
            result = memo[&(i - 1)] + memo[&(i - 2)];
        }
        memo.insert(i,result);
    }
    *memo.get(&n).unwrap()
}

pub fn min_ignore_none(a: Option<i64>, b: Option<i64>) -> Option<i64> {
    if a.is_none() {
        return b
    }
    if b.is_none() {
        return a
    }
    std::cmp::min(a, b)
    
}

pub fn minimum_coins(m: i64, coins: Vec<i64>) -> i64 {
    let mut answer = None;
    if m == 0 {
        answer = Some(0);
    } else {
        for coin in &coins {
            let subproblem = m - coin;
            if subproblem < 0 {
                continue;
            }
            answer = min_ignore_none(answer, Some(minimum_coins(subproblem, coins.clone()) + 1));
        }
    }
    answer.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_mem() {
        let result = fibfaster(50);
        assert_eq!(12586269025, result);
    }
}
