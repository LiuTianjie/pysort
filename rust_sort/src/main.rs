use std::usize;


fn main(){
    let list:Vec<i32> = vec![9,2,5,4,8,2,4,6,3,8,7,7,7,1,3,5,10,6,99,3421,21,4,1,0,2];
    let buble_list = buble_sort(&list);
    // notice that the length of the list to be sorted by quick_sort method must longer than 0, 
    // or it may cause an error, since here the second parameter is an usize type.
    let quick_list = quick_sort(&list,0,list.len()-1);
    let insert_list = insert_sort(&list);
    let merge_list = merge_sort(&list);
    let shell_list = shell_sort(&list);
    let heap_list = heap_sort(&list);
    let base_list = base_sort(&list);
    println!("1. buble sort:  {:?}",buble_list );
    println!("2. quick sort:  {:?}",quick_list);
    println!("3. insert sort: {:?}",insert_list);
    println!("4. merge sort:  {:?}",merge_list);
    println!("5. shell sort:  {:?}",shell_list);
    println!("6. heap sort:   {:?}",heap_list);
    println!("7. base srot:   {:?}",base_list);
}


// buble sort, change the position of the neighbors, once a single turn i is over,
// next time we just need to sort 0~nums.len()-1-i.
pub fn buble_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut nums = nums.to_vec();
    for i in 0..nums.len(){
        for j in 0..nums.len()-i-1{
            if nums[j] > nums[j+1]{
               nums.swap(j,j+1)
            }
        }
    }
    return nums
}


// quick sort, firstly, we choose the first number of the numbers list as a standard number,
// and wo get two pointers, one is at the start of the list while the other is
// at the end of the list. we aim to let the numbers on the left side of the standard
// is less than it, and the numbers on the right side of the standard is bigger than it.
pub fn quick_sort(nums:&Vec<i32>,left:usize,right:usize)->Vec<i32>{
    let mut nums = nums.to_vec();
    quick_sub_sort(&mut nums,left as i32,right as i32);
    return nums
}
pub fn quick_sub_sort(nums:&mut Vec<i32>,l:i32,r:i32){
    if l< r{
        let (mut i, mut j,x) = (l as usize,r as usize ,nums[l as usize]);
        while i<j{
            while i< j && nums[j]>=x{
                j-=1;
            }
            if i<j{
                nums[i] = nums[j];
            }
            while i<j && nums[i] < x{
                i+=1;
            }
            if i<j{
                nums[j] = nums[i]
            }
        }
        nums[i] = x;
        quick_sub_sort(nums, l,i as i32 -1);
        quick_sub_sort(nums, i as i32+1, r);
    }
}

// insert sort, create a new list, traverse nums and new list, when it is the end of the 
// new list, insert the poistion of the value in nums. Otherwise, compare the current values
// in nums and new list, when new list's number bigger than the current value of nums, insert
// the current value of nums in the current position of new list
pub fn insert_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut new_nums:Vec<i32> = vec![];
    for i in 0..nums.len(){
        let n = new_nums.len();
        for j in 0..=n{
            if j==n || new_nums[j]>nums[i] {
                new_nums.insert(j, nums[i]);
                break;
            }
        }
    }
    return new_nums
}


// merge sort, divede the nums into smaller intervals, then make each interval sorted,
// then merge them, you can use recursion, but there I use a in-place algorithm, this
// method can save some spaces..., ok, the fact is I cant' learn... :)
pub fn merge_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut nums = nums.to_vec();
    let mut i:u32=0;
    while 2_i32.pow(i)<= nums.len() as i32{
        for  j in (0..2_i32.pow(i+1)).step_by(nums.len()){
            merge_in_sort(&mut nums,j,j+2_i32.pow(i+1))
        }
        i+=1
    }
    return nums
}
// tool function used by merge_sort().
pub fn merge_in_sort(nums:&mut Vec<i32>,start:i32,end:i32){
    let mut end = end;
    // if the end position is beyond the boundary, limit it into the boundary.
    if end as usize > nums.len(){
        end = nums.len() as i32;
    }
    // just a buble sort inside, also can be replaced by other methods.
    for i in start..end{
        for j in start..end-i-1{
            if nums[j as usize] > nums[(j+1) as usize]{
                nums.swap(j as usize,(j+1) as usize) 
            }
        }
    }
}


// shell sort, basiclly is insert sort by groups. 
// First let step equal to half of the nums' length, then half the value each turn,
// until it equal or less than 0. Apply insert sort on the items diveded by the step,
// finally we can get the right answer.
pub fn shell_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut nums = nums.to_vec();
    fn insertion(nums: &mut Vec<i32>, start: usize, gap: usize) {
        for i in ((start + gap)..nums.len()).step_by(gap) {
            let val_current = nums[i];
            let mut pos = i;
            // make swaps
            while pos >= gap && nums[pos - gap] > val_current {
                nums[pos] = nums[pos - gap];
                pos = pos - gap;
            }
            nums[pos] = val_current;
        }
    }
    let mut count_sublist = nums.len() / 2; // makes gap as long as half of the array
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(&mut nums, pos_start, count_sublist);
        }
        count_sublist /= 2; // makes gap as half of previous
    }
    return  nums
}


// heap sort, from bottom to top, from left to right. 
// each turn we find the current biggest one and move it to the end.(each turn's end, the length is shrink
// as the turns goes by.)
pub fn heap_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut nums = nums.to_vec();
    let mut n  = nums.len() as i32;
    // from bottom to top
    while n >1{
        let mut i = n/2 -1;
        while i>=0{
            heapify(&mut nums,i ,n);
            i-=1;
        }
        // after the above while loop, the top node is the biggest one,
        // we should swap it with the last one, and next turn, we minus n to 
        // tranverse the nums beside the biggest ones.
        nums.swap(0,n as usize -1);
        n-=1;
    }
    return nums
}
// adjust the position of father and child, keep the fater node is bigger than child node.
pub fn heapify(nums:&mut Vec<i32>,i:i32,n:i32){
    // i is father node, 2*i+1 is his left child node.
    if nums[2*i as usize+1] > nums[i as usize]{
        nums.swap(2*i as usize +1, i as usize)
    }
   // i is father node, 2*i+1 is his right child node. 
    if 2*i +2<n && nums[2*i as usize+2] > nums[i as usize]{
        nums.swap(2*i as usize +2, i as usize) 
    }
    // after one turn, the current non-leaf node beacome the biggest
}


// base sort, sort the nums by each position of it.
// insede the loop, we can use different sort methods, here we simply choose buble sort.
pub fn base_sort(nums:&Vec<i32>)->Vec<i32>{
    let mut nums = nums.to_vec();
    let mut n:u32=0;
    let mut flag = false;
    loop{
        let base:i32 = 10_i32.pow(n) as i32;
        for i in 0..nums.len(){
            for j in 0..nums.len()-i-1{
                if nums[j]/base >nums[j+1]/base{
                    nums.swap(j, j+1)
                }
            }
        }
        n+=1;
        if nums[nums.len()-1]/base ==0{
            flag = true
        }
        if flag{
            break;
        }
    }
    return nums
}