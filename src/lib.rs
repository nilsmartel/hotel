mod hotel_map;

pub use hotel_map::*;

#[cfg(test)]
mod tests {
    use super::Hotel;

    #[test]
    fn inserting() {
        let collection = 0..10000;
        let mut hotel = Hotel::new();
        collection.for_each(|v| {
            let key = hotel.put(v);
            assert_eq!(key, v as usize);
        });
    }

    /// Guarantees that order of insertion is determinisitc even after retrieval
    #[test]
    fn order() {
        let collection = 0..10000;
        let mut hotel = Hotel::new();
        collection.for_each(|v| {
            let key = hotel.put(v);
            assert_eq!(key, v as usize);
        });

        hotel.take(0);
        hotel.take(15);
        hotel.take(32);
        hotel.take(3189);
        hotel.take(7777);

        assert_eq!(hotel.put(0), 7777);
        assert_eq!(hotel.put(0), 3189);
        assert_eq!(hotel.put(0), 32);
        assert_eq!(hotel.put(0), 15);
        assert_eq!(hotel.put(0), 0);
    }
}

/// collection to associate data with unique keys.
#[derive(Default, Debug)]
pub struct Hotel<T> {
    floor: Vec<Option<T>>,
    /// List of available slots on the floor
    holes: Vec<usize>,
}

impl<T> IntoIterator for Hotel<T> {
    type Item = (usize, T);

    type IntoIter = HotelIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        HotelIter {
            floor: self.floor,
            cursor: 0,
        }
    }
}

impl<T> Hotel<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Hotel {
            floor: Vec::with_capacity(capacity),
            holes: Vec::new(),
        }
    }

    pub fn new() -> Self {
        Hotel {
            floor: Vec::new(),
            holes: Vec::new(),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.floor.iter().filter_map(|v| v.as_ref())
    }

    pub fn put(&mut self, value: T) -> usize {
        match self.holes.pop() {
            Some(index) => {
                self.floor[index] = Some(value);
                index
            }
            None => {
                let index = self.floor.len();
                self.floor.push(Some(value));
                index
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if let Some(v) = self.floor.get(key).and_then(|v| v.as_ref()) {
            Some(v)
        } else {
            None
        }
    }

    /// Remove element with `key` from Hotel and returns it.
    /// Returns None if key is not assigned.
    pub fn take(&mut self, key: usize) -> Option<T> {
        if self.floor.len() > key && self.floor[key].is_some() {
            // Declare this position as free to be occupied
            self.holes.push(key);
            return self.floor.remove(key);
        }

        None
    }

    /// Removes an element from the Hotel, returns Err of no Element is present
    pub fn remove(&mut self, key: usize) -> Result<T, ()> {
        self.take(key).ok_or(())
    }
}

pub struct HotelIter<T> {
    floor: Vec<Option<T>>,
    cursor: usize,
}

impl<T> Iterator for HotelIter<T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor;
        if cursor >= self.floor.len() {
            return None;
        }

        self.cursor += 1;

        if self.floor[cursor].is_none() {
            return self.next();
        }

        let value = self.floor[cursor].take().unwrap();

        Some((cursor, value))
    }
}
