use cargo_snippet::snippet;
use std::cmp::Ordering;

#[snippet("BinarySearch")]
pub trait BinarySearch<T> {
    fn lower_bound(&self,x: &T) -> usize;
    fn upper_bound(&self,x: &T) -> usize;
}

#[snippet("BinarySearch")]
impl <T:Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self,key: &T) -> usize{
        let mut low = 0;
        let mut high = self.len();

        while high != low {
            let mid = (low+high)/2;
            match self[mid].cmp(key) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self,key: &T) -> usize{
        let mut low = 0;
        let mut high = self.len();

        while high != low {
            let mid = (low+high)/2;
            match self[mid].cmp(key) {
                Ordering::Less | Ordering::Equal => {
                    low = mid+1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}


#[test]
fn test_binarysearch() {
    let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.upper_bound(&4), 3);
}
