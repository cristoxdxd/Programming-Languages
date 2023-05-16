fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }

        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = [10, 5, 2, 3, 6, 8, 7, 9, 1, 4];
    merge_sort(&mut arr);
    println!("{:?}", arr);
}