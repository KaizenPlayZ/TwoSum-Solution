struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[j] == target-nums[i] {
                    let answer:Vec<i32> = vec![i as i32,j as i32];
                    return answer;
                }
            }
        }
        let noAnswer:Vec<i32> = vec![0 as i32];
        return noAnswer;
    }
}
fn main() {
    println!("{:?}",Solution::two_sum(vec![1,5],6));
}
