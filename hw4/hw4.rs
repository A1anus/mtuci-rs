struct Vector<T> {
    data: Box<[T]>,
    length: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Self {
            data: vec![].into_boxed_slice(),
            length: 0,
            capacity: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        let mut vec = Vector::new();
        vec.data = Vec::with_capacity(capacity).into_boxed_slice();
        vec.capacity = capacity;
        vec
    }

    fn push(&mut self, value: T) {
        if self.length == self.capacity {
            self.resize(self.capacity * 2); // Увеличиваем размер в два раза при необходимости
        }
        self.data[self.length] = value;
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
            Some(std::mem::replace(&mut self.data[self.length], unsafe { std::mem::zeroed() }))
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let removed_value = std::mem::replace(&mut self.data[index], unsafe { std::mem::zeroed() });
            for i in index..self.length - 1 {
                self.data[i] = std::mem::replace(&mut self.data[i + 1], unsafe { std::mem::zeroed() });
            }
            self.length -= 1;
            Some(removed_value)
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_data = Vec::with_capacity(new_capacity);
        new_data.extend_from_slice(&self.data[..self.length]);
        self.data = new_data.into_boxed_slice();
        self.capacity = new_capacity;
    }
}

fn main() {
    let mut vec: Vector<i32> = Vector::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);

    println!("Vector: {:?}", vec.data()); // Выводим вектор

    let popped_value = vec.pop();
    println!("Popped value: {:?}", popped_value); // Выводим удалённое значение

    let removed_value = vec.remove(0);
    println!("Removed value: {:?}", removed_value); // Выводим удалённое значение по индексу

    if let Some(value) = vec.get(1) {
        println!("Value at index 1: {:?}", value); // Выводим значение по индексу
    }
}