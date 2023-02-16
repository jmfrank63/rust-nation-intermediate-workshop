use std::fmt::Display;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Replace ? with correct Generic type parameters
    fn my_filter<P>(self, predicate: P) -> MyFilter<Self, P>
    where
        P: Fn(&Self::Item) -> bool,
        Self: Sized,
    {
        MyFilter {
            iterator: self,
            predicate,
        }
    }

    fn my_map<M, V>(self, mapper: M) -> MyMap<Self, M>
    where
        M: Fn(Self::Item) -> V,
        Self: Sized,
    {
        MyMap {
            iterator: self,
            mapper,
        }
    }

    fn my_sum(mut self) -> i32
    where
        Self: MyIterator<Item = i32> + Sized,
    {
        let mut sum = 0;
        while let Some(item) = self.next() {
            sum += item;
        }
        sum
    }
}

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

impl<I, P> MyIterator for MyFilter<I, P>
where
    I: MyIterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iterator.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }

        None
    }
}

impl<I, M, V> MyIterator for MyMap<I, M>
where
    I: MyIterator,
    M: Fn(I::Item) -> V,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iterator.next() {
            Some((self.mapper)(value))
        } else {
            None
        }
    }
}
struct MyFilter<I, P> {
    iterator: I,
    predicate: P,
}

struct MyMap<I, M> {
    iterator: I,
    mapper: M,
}

fn print_iterator<T: Display>(mut iterator: impl MyIterator<Item = T>) {
    // Remember that MyIterator is not integrated to Rust
    // you will not be able to use `for elt in iterator {`
    while let Some(item) = iterator.next() {
        println!("{item}");
    }
}

fn main() {
    let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_iterator(enumeration.clone());

    let filtered = enumeration.clone().my_filter(|&item| item % 2 == 0);
    print_iterator(filtered);

    // let mapped = enumeration.clone().my_map(|item| format!("Value: {}", item));
    // print_iterator(mapped);

    // let total = enumeration.clone().my_sum();
    // println!("Total: {}", total);

    // let filtered_mapped_total = enumeration.clone()
    //     .my_filter(|&item| item % 2 == 0)
    //     .my_map(|item| item * 2)
    //     .my_sum();
    // println!("Filtered Mapped total is: {}", filtered_mapped_total);
}
