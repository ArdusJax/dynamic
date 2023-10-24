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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_mem() {
        let result = fibfaster(50);
        assert_eq!(12586269025, result);
    }
}
