#![allow(non_snake_case)]	// i don't want them putting chemicals in the water
#![allow(dead_code)]		// that turn the frickin vars snake
use std::fs::read_to_string;
use std::collections::HashMap;
use std::i64;


// implement your own "usize to i64" converter
#[derive(Debug)]
struct ConvertError;

fn tryintoi64(value: usize) -> Result<i64, ConvertError> {
	Ok(value as i64)
}


// read input file line by line, collect line into vector
fn readLines(filename: &str) -> Vec<String> {
	let mut lineVec = Vec::new();
	for line in read_to_string(filename).unwrap().lines() {
		lineVec.push(line.to_string());
	};
	lineVec
}


fn main() {
	let mut rightVec: Vec<i64> = Vec::new();	// create empty vector for right column
	let mut leftVec: Vec<i64> = Vec::new();		// create empty vector for left column

	// read input file, split line and push values to respective column vector
	for elem in readLines("./input.txt") {
		let tempVec: Vec<&str> = elem.split("   ").collect();
		rightVec.push(tempVec[0].parse().unwrap());
		leftVec.push(tempVec[1].parse().unwrap());
	}

	// clone vector to keep original order
	let mut rightSorted = rightVec.clone();
	let mut leftSorted = leftVec.clone();

	// sort clones without keeping element's places
	rightSorted.sort_unstable();
	leftSorted.sort_unstable();

	// empty vector to store the differences between right and left column's values
	let mut sumVec: Vec<i64> = Vec::new();

	// check if the columns are equal length, subtract right from left and push its absolute value to sumVec
	if rightSorted.len() == leftSorted.len() {
		for index in 0..rightSorted.len() {
			let difference: i64 = rightSorted[index] - leftSorted[index];
			sumVec.push(difference.abs());
		}
	}

	// sum elements of sumVec and print it
	let sum: i64 = sumVec.iter().sum();
	println!("Sum of input1 differences: {}", sum);

	// --- END OF PART 1 ---
	// --- BEGIN PRIVA- PART 2 ---

	// new hashmap for right's occurrances in left
	let mut rOccInL = HashMap::new();
	// rOccInL.insert(k, v);

	// iterate over left column's length
	for index in 0..leftVec.len() {
		let nCount = leftVec.iter()	// let nCount be ↓↓
			.filter(|&n| *n == rightVec[index])	// rightVec's current element's occurrances
			.count();	// the count of ↑
		rOccInL.insert(rightVec[index], tryintoi64(nCount).unwrap());	// insert rightVec's current element and its nCount into the HashMap as key and value
	}

	// create new vector for similarity scores
	let mut similarityVec: Vec<i64> = Vec::new();
	
	// for each key-value pair in HashMap, push product of key-value pair to vector
	for (k, v) in rOccInL {
		similarityVec.push(k*v);
	}

	// iterate over similarity vector and sum its values
	let similarityScore: i64 = similarityVec.iter().sum();
	println!("Similarity score: {}", similarityScore);

	// --- END OF PART 2 ---
	// can't believe these worked first try

}