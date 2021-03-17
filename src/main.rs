fn merge(left_half: Vec<i64>, right_half: Vec<i64>) -> Vec<i64> {
    // Setting the vars as mut
    let mut left_half = left_half;
    let mut right_half = right_half;

    // Create the new array with the merged halfs
    let mut merged: Vec<i64> = [].to_vec();

    // Indexes
    let mut left_index = 0;
    let mut right_index = 0;

    for i in 0..(left_half.len() + right_half.len()) {
        // Lengths
        let right_half_len = right_half.len();
        let left_half_len = left_half.len();

        while right_index < right_half_len {
            if left_index < left_half_len
                && right_half[right_index] > left_half[left_index]
                && right_index + 1 < right_half_len
            {
                right_index += 1;
                continue;
            }
            break;
        }

        if left_half_len <= 0 && right_index < right_half_len
            || right_index < right_half_len && right_half[right_index] <= left_half[left_index]
        {
            merged.push(right_half[right_index]);
            right_half.remove(0);
        } else if right_half_len <= 0 && left_index < left_half_len
            || left_index < left_half_len && left_half[left_index] < right_half[right_index]
        {
            merged.push(left_half[left_index]);
            left_half.remove(0);
        }
        left_index = 0;
        right_index = 0;
    }

    return merged;
}

fn merge_sort(numbers: Vec<i64>) -> Vec<i64> {
    let numbers_len = numbers.len();
    let mid_index = numbers_len / 2;

    if numbers_len <= 1 {
        return numbers;
    }
    let mut left_half = numbers[0..mid_index].to_vec();
    let mut right_half = numbers[mid_index..numbers_len].to_vec();
    if left_half.len() > 1 {
        left_half = merge_sort(left_half);
    }

    if right_half.len() > 1 {
        right_half = merge_sort(right_half);
    }

    let merged = merge(left_half, right_half);

    return merged;
}

fn main() {
    let arr = [3, 5, 6, 8, 1, 2, 4, 7].to_vec();

    println!("{:#?}", merge_sort(arr));
}
