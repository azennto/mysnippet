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
