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

#[test]
fn test_gcd(){
    assert_eq!(gcd(57,3),3);
}

#[test]
fn test_lcm(){
    assert_eq!(lcm(57,3),57);
}
