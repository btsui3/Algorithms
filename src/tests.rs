use crate::bubble_sort::bubble_sort;
use crate::merge_sort::merge_sort;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}




#[test]
fn test_bubble_sort() {

    let mut v = vec![4, 6, 8, 11, 13, 3, 1];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1,3,4,6,8,11,13]);
}
