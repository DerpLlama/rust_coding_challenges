use std::collections::HashSet;
/*

Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

Bonus: Can you do this in one pass?

*/

// My original implementation
// O(nLog(n)) Time Complexity
// O(1) Space Complexity
#[allow(dead_code)]
fn coding_challenge(l: Vec<i32>, k: i32) -> bool {
	let mut output: bool = false;
	if l.len() > 1 {
		'outer: for i in 0..(l.len() - 1) {
			'inner: for j in (i + 1)..l.len() {
				if l[i] + l[j] == k {
					output = true;
					break 'outer;
				}
			}
		}
	}
	output
}

// Hashing implementation
// O(n) Time Complexity
// O(n) Space Complexity
#[allow(dead_code)]
fn coding_challenge_hashing(l: Vec<i32>, k: i32) -> bool {
	let mut output: bool = false;
	if l.len() > 1 {
		let mut hashes: HashSet<i32> = HashSet::new();
		hashes.insert(l[0]);
		for i in 1..l.len() {
			let temp = k - l[i];
			if hashes.contains(&temp) {
				output = true;
				break;
			}
			hashes.insert(l[i]);
		}
	}
	output
}

#[test]
fn coding_challenge_test_1() {
	assert!(coding_challenge(vec![1, 2, 3], 4));
}

#[test]
fn coding_challenge_test_2() {
	assert!(coding_challenge(vec![1, 2, 3], 3));
}

#[test]
fn coding_challenge_test_3() {
	assert!(!coding_challenge(vec![1, 2, 3], 2));
}

#[test]
fn coding_challenge_test_4() {
	assert!(coding_challenge(vec![10, 15, 3, 7], 17));
}

#[test]
fn coding_challenge_test_5() {
	assert!(coding_challenge(vec![1, 2, -3], -1));
}

#[test]
fn coding_challenge_hash_test_1() {
	assert!(coding_challenge(vec![1, 2, 3], 4));
}

#[test]
fn coding_challenge_hash_test_2() {
	assert!(coding_challenge(vec![1, 2, 3], 3));
}

#[test]
fn coding_challenge_hash_test_3() {
	assert!(!coding_challenge(vec![1, 2, 3], 2));
}

#[test]
fn coding_challenge_hash_test_4() {
	assert!(coding_challenge(vec![10, 15, 3, 7], 17));
}

#[test]
fn coding_challenge_hash_test_5() {
	assert!(coding_challenge(vec![1, 2, -3], -1));
}
