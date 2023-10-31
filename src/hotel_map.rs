use crate::{Hotel, HotelIter};
use std::{collections::HashMap, hash::Hash};

pub struct HotelMap<K, V>
where
    K: Hash + Eq,
{
    // Hold the values inside a hotel
    hotel: Hotel<V>,
    // Store keys in a HashMap
    map: HashMap<K, usize>,
}

impl<K, V> std::fmt::Debug for HotelMap<K, V>
where
    K: Hash + Eq + std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HotelMap [")?;
        let mut first = true;
        for (k, v) in &self.map {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            let v = &self.hotel.floor[*v];
            write!(f, "{k:?} => {v:?}")?;
        }
        write!(f, "]")
    }
}

impl<K: Hash + Eq, V> Default for HotelMap<K, V> {
    fn default() -> Self {
        Self {
            hotel: Hotel::new(),
            map: HashMap::new(),
        }
    }
}

impl<K: Hash + Eq, V> IntoIterator for HotelMap<K, V> {
    type Item = (usize, V);

    type IntoIter = HotelIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.hotel.into_iter()
    }
}

impl<K, V> HotelMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> HotelMap<K, V> {
        Self {
            hotel: Hotel::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn get_by_key(&self, key: &K) -> Option<(usize, &V)> {
        let index = *self.map.get(key)?;
        Some((index, self.get_by_index(index)?))
    }

    pub fn get_by_index(&self, index: usize) -> Option<&V> {
        self.hotel.get(index)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.get(key).is_some()
    }

    /// Returns slot index of inserted item.
    pub fn insert(&mut self, key: K, value: V) -> usize {
        match self.map.entry(key) {
            std::collections::hash_map::Entry::Occupied(index) => {
                let index = *index.get();
                self.hotel.floor[index] = Some(value);
                index
            }
            std::collections::hash_map::Entry::Vacant(slot) => {
                let index = self.hotel.put(value);
                slot.insert(index);
                index
            }
        }
    }

    /// Only inserts, if no item with same key is present in HotelMap.
    /// Returns slot index of inserted item
    /// or None, if item was already present in hotel
    pub fn try_insert(&mut self, key: K, value: V) -> Option<usize> {
        match self.map.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => None,
            std::collections::hash_map::Entry::Vacant(slot) => {
                let index = self.hotel.put(value);
                slot.insert(index);
                Some(index)
            }
        }
    }

    /// Iterates only over the values and their indices
    pub fn iter_values(&self) -> impl Iterator<Item = (usize, &V)> {
        self.hotel.iter()
    }

    /// Iterate over all keys, indices and values in the hotel
    pub fn iter(&self) -> impl Iterator<Item = (&K, usize, &V)> {
        self.map.iter().map(|(key, index)| {
            (
                key,
                *index,
                self.hotel
                    .get(*index)
                    .expect("keys of HotelMap to always be valid Hotel keys"),
            )
        })
    }
}
