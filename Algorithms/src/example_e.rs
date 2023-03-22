
pub fn test(){
    let mut v = vec![1,3,2,5,4];
    v.sort();
    assert_eq!(v, vec![1,2,3,4,5]);
    
    let mut v = vec![1.2,31.2,2.1,4.3];
    v.sort_by(|a, b|{
        a.partial_cmp(b).unwrap()
    });
    assert_eq!(v, vec![1.2, 2.1, 4.3, 31.2]);
}