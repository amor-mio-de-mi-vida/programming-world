#[derive(Debug, Clone, PartialEq)]
struct HashMap <T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(size: usize) -> Self {
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap {size, slot, data }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.size
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key { panic!("Error: key must > 0");}

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos {
                    println!("Error: slot is full, quit insertion");
                    return;
                }
            }

            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key { panic!("Error: key must > 0"); }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T> = None;
            let mut stop = false; let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    curr = self.rehash(curr);
                    if curr == pos { stop = true; }
                }
            }

            data
        }
    }

    fn get(&self, key: usize) -> Option<&T> {
        if 0 == key { panic!("Error: key must > 0"); }

        let pos = self.hash(key);
        let mut data: Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
                data = self.data.get(curr);
            } else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }

        data
    }

    fn contains(&self, key: usize) -> bool {
        if key == 0 { panic!("Error: key must > 0"); }
        self.slot.contains(&key)
    }

    fn len(&self) -> usize {
        let mut length = 0;
        for d in self.slot.iter() {
            if &0 != d {
                length += 1;
            }
        }
        length
    }


}

fn main() {
    let mut hmap = HashMap::new(11);
    hmap.insert(10, "cat");
    hmap.insert(2, "dog");
    hmap.insert(3, "tiger");

    println!("HashMap size {:?}", hmap.len());
    println!("HashMap contains key 2: {}", hmap.contains(2));
    println!("HashMap key 3: {:?}", hmap.get(3));
    println!("Hashmap remove key 3: {:?}", hmap.remove(3));
    println!("Hashmap remove key 3: {:?}", hmap.remove(3));
}