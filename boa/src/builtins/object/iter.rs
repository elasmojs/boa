use super::*;
use std::collections::hash_map;
use std::iter::FusedIterator;

impl Object {
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            indexed_properties: self.indexed_properties.iter(),
            properties: self.properties.iter(),
        }
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_> {
        Keys { inner: self.iter() }
    }

    #[inline]
    pub fn values(&self) -> Values<'_> {
        Values { inner: self.iter() }
    }

    #[inline]
    pub fn symbols(&self) -> Symbols<'_> {
        Symbols {
            symbols: self.symbol_properties.iter(),
        }
    }

    #[inline]
    pub fn symbol_keys(&self) -> SymbolKeys<'_> {
        SymbolKeys {
            symbols: self.symbol_properties.keys(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    indexed_properties: hash_map::Iter<'a, u32, Property>,
    properties: hash_map::Iter<'a, RcString, Property>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (PropertyKey, &'a Property);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, value)) = self.indexed_properties.next() {
            Some(((*key).into(), value))
        } else {
            let (key, value) = self.properties.next()?;
            Some((key.clone().into(), value))
        }
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.indexed_properties.len() + self.properties.len()
    }
}

impl FusedIterator for Iter<'_> {}

#[derive(Debug, Clone)]
pub struct Keys<'a> {
    inner: Iter<'a>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = PropertyKey;
    fn next(&mut self) -> Option<Self::Item> {
        let (key, _) = self.inner.next()?;
        Some(key)
    }
}

impl ExactSizeIterator for Keys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FusedIterator for Keys<'_> {}

#[derive(Debug, Clone)]
pub struct Values<'a> {
    inner: Iter<'a>,
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a Property;
    fn next(&mut self) -> Option<Self::Item> {
        let (_, value) = self.inner.next()?;
        Some(value)
    }
}

impl ExactSizeIterator for Values<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FusedIterator for Values<'_> {}

#[derive(Debug, Clone)]
pub struct Symbols<'a> {
    symbols: hash_map::Iter<'a, RcSymbol, Property>,
}

impl<'a> Iterator for Symbols<'a> {
    type Item = (&'a RcSymbol, &'a Property);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.symbols.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.symbols.size_hint()
    }
}

impl ExactSizeIterator for Symbols<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.symbols.len()
    }
}

impl FusedIterator for Symbols<'_> {}

#[derive(Debug, Clone)]
pub struct SymbolKeys<'a> {
    symbols: hash_map::Keys<'a, RcSymbol, Property>,
}

impl<'a> Iterator for SymbolKeys<'a> {
    type Item = &'a RcSymbol;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.symbols.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.symbols.size_hint()
    }
}

impl ExactSizeIterator for SymbolKeys<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.symbols.len()
    }
}

impl FusedIterator for SymbolKeys<'_> {}