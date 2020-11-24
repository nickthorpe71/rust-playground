fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut new_list = list.to_vec();
    new_list.sort();
    
    let sum: i64 = new_list.iter().rev().take(n).sum();
    let product: i64 = new_list.iter().take(n).product();

    if sum > product {
        return String::from("sum");
    } else if sum == product {
        return String::from("same");
    } else {
        return String::from("product");
    }
}

sum_or_product(&[5,4,3,5,4], 3);
