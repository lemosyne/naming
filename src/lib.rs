/// A `Namer` is something that manages mappings from names to IDs.
pub trait Namer {
    /// The type of a name.
    type Name;
    /// The type of an ID.
    type Id;
    /// An iterator over pairs of names and IDs produced by `scan()`.
    type Scan<'a>: Iterator<Item = (&'a Self::Name, &'a Self::Id)>
    where
        Self: 'a;

    /// Inserts a new mapping from `name` to `id`.
    /// The previously mapped ID is returned if it existed.
    fn insert(&mut self, name: Self::Name, id: Self::Id) -> Option<Self::Id>;

    /// Gets the ID that `name` maps to.
    fn get(&self, name: &Self::Name) -> Option<&Self::Id>;

    /// Removes the mapping from `name` to its ID.
    fn remove(&mut self, name: &Self::Name) -> Option<Self::Id>;

    /// Remaps the ID for `old` to `new`.
    /// The previously mapped ID for `new` is returned if it existed.
    fn remap(&mut self, old: &Self::Name, new: Self::Name) -> Option<Self::Id>;

    /// Returns an iterator of name and ID pairs.
    /// The semantics of what pairs are iterated over are decided by the namer.
    fn scan(&mut self, name: &Self::Name) -> Self::Scan<'_>;
}
