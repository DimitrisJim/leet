impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut count: i32 = 0;
        for i in 0..start_time.len() {
            if start_time[i] <= query_time {
                if query_time <= end_time[i] {
                    count += 1;
                }
            }
        }
        count
    }
}
