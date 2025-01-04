use crate::cache::BIT_LOOKUP_TABLE;

pub fn inverse_permutation_index(index: u64, length: usize, k: usize) -> Vec<u64> {
    let mut permutation = vec![0; length];
    let mut available_numbers: Vec<u64> = (0..=k as u64).collect();
    let mut current_index = index;

    for i in 0..length {
        let remaining = k as u64 - 1 - i as u64;
        let combinations = pick(remaining, (length - 1 - i) as u64);
        let position = current_index / combinations;
        permutation[i] = available_numbers[position as usize];
        available_numbers.remove(position as usize);
        current_index %= combinations;
    }

    permutation
}

pub fn permutation_index(arr: &[u64], k: usize) -> u64 {
    let length = arr.len();
    let mut visited = 0u64;
    let mut index = 0;
    let mut lehmer = [0; 12];
    let bit_count = &BIT_LOOKUP_TABLE;

    for i in 0..length {
        lehmer[i] = arr[i] - bit_count[first_n_bits(visited, arr[i]) as usize] as u64;
        visited |= 1 << arr[i];
    }

    for i in 0..length {
        index += lehmer[i] * pick((k - 1 - i) as u64, (length - 1 - i) as u64)
    }

    index
}

// n is 1 indexed
const fn first_n_bits(value: u64, n: u64) -> u64 {
    value & ((1 << n) - 1)
}

pub fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

pub fn comb(n: u64, k: u64) -> u64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}
pub fn pick(n: u64, k: u64) -> u64 {
    if n == k {
        return factorial(n);
    }
    factorial(n) / factorial(n - k)
}

// input array must be sorted, very janky!
pub fn get_ud_slice_combination(arr: [u64; 4]) -> u64 {
    let mut arr = arr.clone();
    arr.sort();

    let mut result = 0;
    let mut count = 0;
    for i in 0..12 {
        if i <= arr[0] {
            continue;
        }
        if i == arr[1] || i == arr[2] || i == arr[3] {
            count += 1;
            continue;
        }
        result += comb(i, count);
    }
    result
}

pub fn decode_number_base(number: u64, base: u64, length: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(length);
    let mut current = number;
    while current > 0 {
        result.push(current % base);
        current /= base;
    }
    result
}

pub const fn write_01(num: u8, value: u8) -> u8 {
    value & 0b1111_1100 | num
}
pub const fn write_23(num: u8, value: u8) -> u8 {
    value & 0b0000_0011 | num << 2
}
pub const fn write_45(num: u8, value: u8) -> u8 {
    value & 0b1111_1100 | num << 4
}
pub const fn write_67(num: u8, value: u8) -> u8 {
    value & 0b0000_0011 | num << 6
}
pub const fn read_01(value: u8) -> u8 {
    value & 0b0000_0011
}
pub const fn read_23(value: u8) -> u8 {
    (value & 0b0000_1100) >> 2
}
pub const fn read_45(value: u8) -> u8 {
    (value & 0b0011_0000) >> 4
}
pub const fn read_67(value: u8) -> u8 {
    (value & 0b1100_0000) >> 6
}
pub const fn div_8(x: usize) -> usize {
    x >> 3
}
pub const fn mod_8(x: u64) -> u64 {
    x & 7
}
