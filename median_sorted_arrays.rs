impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

let mut nums1 =  nums1.clone();
nums1.extend(nums2);
let mut nums3 = nums1.clone();
let numlen =  nums3.len();
 nums3.sort();
     let ind = numlen /2;
 if numlen % 2  == 0 {
     let n1 = nums3.get(ind).unwrap();
    let n2 = nums3.get(ind-1).unwrap();

let n3 = ((n1 + n2) as f64) /2.0 ;
println!("{n3}");
return n3 as f64;


 } else {

     return *nums3.get(ind).unwrap() as f64
 }

    }
}
