use num::{Integer, Unsigned};

trait ModInfo {
    type U;
    fn modulo(&self) -> Self::U;
}

macro_rules! static_mod_info {
    ($name: ident, $type: ty, $modulo: expr) => {
        struct $name {}
        impl ModInfo for $name {
            U = $type;
            fn modulo() -> $type {
                $modulo
            }
        }
    };
}

pub struct DynamicModInfo<U: Unsigned + Integer, const ID: u32> {}
impl DynamicModInfo<U: Unsigned + Integer, const ID: u32> {
    fn modulo_ref() -> &'static U {
        static modulo: U = U::default();
        &modulo
    }
    fn set_modulo(modulo: U) {
        assert!(!modulo.is_zero());
        self.modulo_ref() = modulo.clone();
    }
}
impl<U: Unsigned + Integer, const ID:u32> ModInfo for DynamicModInfo<U, ID> {
    type U = U;
    fn modulo(&self) -> Self::U {
        self.modulo_ref().clone()
    }
}

struct Modint<M>
where
    M: ModInfo,
{
    mod_info: M,
    value: M::U,
}
impl<M: ModInfo> Modint<M> {
    type U = M::U;
    fn new(value: U) -> Self {
        Self {
            mod_info: M::new(),
            value,
        }
    }
    fn modulo(&self) -> M::U {
        self.mod_info.modulo()
    }
    fn value(&self) -> M::U {
        self.value
    }
}
