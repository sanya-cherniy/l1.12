fn main() {
    let set1 = vec![1, 2, 3, 4, 5];
    let set2 = vec![4, 5, 6, 7];
    let res = intersection(set1, set2);
    println!("{:?}", res);
}

// Нахождение пересечения двух множеств
pub fn intersection(set1: Vec<i32>, set2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![]; // результирующий вектор
                          // проходим по каждому элементу множества 1
    for i in set1 {
        // если текущий элемент содержится в множестве 2 и не содержится в результирующем, добавляем элемент в результат
        if set2.contains(&i) && !res.contains(&i) {
            res.push(i);
        }
    }
    res
}
