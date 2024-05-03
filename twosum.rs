fn main(){
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![0, 0];

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    result[0] = i as i32;
                    result[1] = j as i32;
                    return result;
                }
            }
        }
       return result;
    }

    let arr = vec![3,2,4];
    println!("{:?}",two_sum(arr,6));
    

}