pub fn cartesian_product(set_a: &Vec<i32>, set_b: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut product: Vec<(i32, i32)> = Vec::new();

    for el_a in set_a {
        for el_b in set_b {
            product.push((*el_a, *el_b))
        }
    }

    return product;
}
