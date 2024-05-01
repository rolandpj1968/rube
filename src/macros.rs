// macros

// [] operator for all::Idx<T>
macro_rules! def_index {
    ($idxt:ty, $colt:ty, $valt:ty) => {
        impl Index<$idxt> for $colt {
            type Output = $valt;
            fn index(&self, index: $idxt) -> &Self::Output {
                debug_assert!(index != <$idxt>::NONE);
                self.index(index.0 as usize)
            }
        }
        impl Index<&$idxt> for $colt {
            type Output = $valt;
            fn index(&self, index: &$idxt) -> &Self::Output {
                debug_assert!(*index != <$idxt>::NONE);
                self.index(index.0 as usize)
            }
        }
    };
}

// mut [] operator for all::Idx<T>
macro_rules! def_index_mut {
    ($idxt:ty, $colt:ty, $valt:ty) => {
        impl IndexMut<$idxt> for $colt {
            fn index_mut(&mut self, index: $idxt) -> &mut Self::Output {
                debug_assert!(index != <$idxt>::NONE);
                self.index_mut(index.0 as usize)
            }
        }
        impl IndexMut<&$idxt> for $colt {
            fn index_mut(&mut self, index: &$idxt) -> &mut Self::Output {
                debug_assert!(*index != <$idxt>::NONE);
                self.index_mut(index.0 as usize)
            }
        }
    };
}

// [] operator for enum
macro_rules! def_enum_index {
    ($enumt:ty, $colt:ty, $valt:ty) => {
        impl Index<$enumt> for $colt {
            type Output = $valt;
            fn index(&self, index: $enumt) -> &Self::Output {
                self.index(index as usize)
            }
        }
    };
}

// mut [] operator for enum
macro_rules! def_enum_index_mut {
    ($enumt:ty, $colt:ty, $valt:ty) => {
        impl IndexMut<$enumt> for $colt {
            fn index_mut(&mut self, index: $enumt) -> &mut Self::Output {
                self.index_mut(index as usize)
            }
        }
    };
}
