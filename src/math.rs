use cargo_snippet::snippet;

#[snippet]
pub fn gcd(a:u64,b:u64) -> u64{
    if b == 0 {
        a
    } else {
        gcd(b,a%b)
    }
}

#[snippet(include="gcd")]
pub fn lcm(a:u64,b:u64) -> u64{
    a*b/gcd(a,b)
}

#[snippet]
pub fn prime_judge(n:usize) -> bool{
    if n == 1 {
        return false;
    }
    for i in (2..=n).take_while(|&i| i*i <= n){
        if n%i == 0 {
            return false;
        }
    }
    return true;
}

#[snippet]
pub fn div_enum(n:usize) -> Vec<usize>{
    let mut ans :Vec<usize> = Vec::new();
    for i in (1..=n).take_while(|&i| i*i <= n) {
       if n%i == 0 {
        ans.push(i);
        if n/i != i {
            ans.push(n/i);
        }
       } 
    }
    return ans;
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(57,3),3);
}

#[test]
fn test_lcm(){
    assert_eq!(lcm(57,3),57);
}

#[test]
fn test_prime_judge(){
    assert_eq!(prime_judge(2),true);
    assert_eq!(prime_judge(10),false);
}

#[test]
fn test_div_enum(){
    let mut x = div_enum(12);
    x.sort();
    assert_eq!(x,[1,2,3,4,6,12]);
}
