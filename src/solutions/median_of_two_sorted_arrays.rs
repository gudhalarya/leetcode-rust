//This is the question number 4 we have to find the median of the two sorted arrays 
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64{
    let mut result  = Vec::with_capacity(nums1.len()+nums2.len()); 
    let (mut i,mut j) = (0,0);
    while i<nums1.len() && j <nums2.len(){
        if nums1[i]< nums2[j]{
            result.push(nums1[i]);
            i+=1;
        }else {
            result.push(nums2[j]);
            j+=1;
        }
    }
    result.extend_from_slice(&nums1[i..]);
    result.extend_from_slice(&nums2[j..]);
    

    //This is where we will be findig out the median 
    let x= result.len();
    let mid = x/2;
    if x%2 ==0{
        (result[mid-1] as f64 + result[mid] as f64)/2.0
    }
    else {
        result[mid] as f64
    }

}