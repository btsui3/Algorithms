pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _  in 0..v.len() {
        for i in 0..v.len() -1 {
            if v[i] > v[i+1] {
                v.swap(i, i+1)
            }
        }
    }
}