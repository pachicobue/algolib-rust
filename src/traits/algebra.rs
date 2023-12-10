//! Traits for algebraic structures.

/// A magma is a set with a binary operation.
pub trait Magma {
    /// The type of the set.
    type S: Eq + Copy;
    /// The binary operation.  
    /// $`\mathrm{op}(x,y) \in S \quad \forall x,y \in S`$
    fn op(x: Self::S, y: Self::S) -> Self::S;
}

/// An identity is a magma with an identity element.
/// $`\mathrm{op}(\mathrm{id}(), x) = \mathrm{op}(x, \mathrm{id}()) = x \quad \forall x \in S`$
pub trait Identity: Magma {
    /// The identity element.  
    /// $`\mathrm{op}(\mathrm{id}(), x) = \mathrm{op}(x, \mathrm{id}()) = x \quad \forall x \in S`$
    fn id() -> Self::S;
    /// Identity check.
    fn identity_check(x: Self::S) -> bool {
        Self::op(Self::id(), x) == x && Self::op(x, Self::id()) == x
    }
}

/// An commutative is a magma with a commutative binary operation.  
/// $`\mathrm{op}(x,y) = \mathrm{op}(y,x) \quad \forall x,y \in S`$
pub trait Commutive: Magma {
    /// Commutative check.
    fn commutative_check(x: Self::S, y: Self::S) -> bool {
        Self::op(x, y) == Self::op(y, x)
    }
}

/// An associative is a magma with an associative binary operation.  
/// $`\mathrm{op}(\mathrm{op}(x,y),z) = \mathrm{op}(x,\mathrm{op}(y,z)) \quad \forall x,y,z \in S`$
pub trait Associative: Magma {
    /// Associative check.
    fn associative_check(x: Self::S, y: Self::S, z: Self::S) -> bool {
        Self::op(Self::op(x, y), z) == Self::op(x, Self::op(y, z))
    }
}

/// An invertible is a magma with an invertible binary operation.
/// $`\mathrm{op}(x, \mathrm{inv}(x)) = \mathrm{op}(\mathrm{inv}(x), x) = \mathrm{id}() \quad \forall x \in S`$
pub trait Invertible: Magma + Identity {
    /// The inverse element.  
    /// $`\mathrm{op}(x, \mathrm{inv}(x)) = \mathrm{op}(\mathrm{inv}(x), x) = \mathrm{id}() \quad \forall x \in S`$
    fn inv(x: Self::S) -> Self::S;
    /// Invertible check.
    fn invertible_check(x: Self::S) -> bool {
        Self::op(x, Self::inv(x)) == Self::id() && Self::op(Self::inv(x), x) == Self::id()
    }
}

/// A distributive is a magma with a distributive binary operation.  
/// $`\mathrm{mul}(x, \mathrm{add}(y,z)) = \mathrm{add}(\mathrm{mul}(x,y), \mathrm{mul}(x,z)) \quad \forall x,y,z \in S`$  
/// $`\mathrm{mul}(\mathrm{add}(x,y), z) = \mathrm{add}(\mathrm{mul}(x,z), \mathrm{mul}(y,z)) \quad \forall x,y,z \in S`$
pub trait Distributive<Additive: Magma<S = Self::S>>: Magma {
    /// Distributive check.
    fn distributive_check(x: Self::S, y: Self::S, z: Self::S) -> bool {
        Self::op(x, Additive::op(y, z)) == Additive::op(Self::op(x, y), Self::op(x, z))
            && Self::op(Additive::op(x, y), z) == Additive::op(Self::op(x, z), Self::op(y, z))
    }
}

/// A semigroup is a magma with an associative binary operation.
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// A monoid is a semigroup with an identity element.
pub trait Monoid: SemiGroup + Identity {}
impl<T: SemiGroup + Identity> Monoid for T {}

/// A group is a monoid with an invertible binary operation.
pub trait Group: Monoid + Invertible {}
impl<T: Monoid + Invertible> Group for T {}

/// An Abelian group is a group with a commutative binary operation.
pub trait AbelianGroup: Group + Commutive {}
impl<T: Group + Commutive> AbelianGroup for T {}

/// A ring is a set with two binary operations.  
/// The first operation is commutative and associative.  
/// The first operation has an identity element.  
/// The first operation has an inverse element.  
/// The second operation is associative.  
/// The second operation distributes over the first operation.  
/// The second operation has an identity element.
pub trait Ring {
    /// The type of the set.
    type S: Eq + Copy;
    /// The first binary operation.
    type Additive: AbelianGroup<S = Self::S>;
    /// The second binary operation.
    type Multiplicative: Monoid<S = Self::S>;
    /// The first binary operation.
    fn add(x: Self::S, y: Self::S) -> Self::S {
        Self::Additive::op(x, y)
    }
    /// The second binary operation.
    fn mul(x: Self::S, y: Self::S) -> Self::S {
        Self::Multiplicative::op(x, y)
    }
    /// identity element of the first binary operation.
    fn zero() -> Self::S {
        Self::Additive::id()
    }
    /// identity element of the second binary operation.
    fn one() -> Self::S {
        Self::Multiplicative::id()
    }
    /// inverse element of the first binary operation.
    fn neg(x: Self::S) -> Self::S {
        Self::Additive::inv(x)
    }
}

/// A field is a ring with an invertible second binary operation.
pub trait Field: Ring
where
    Self::Multiplicative: Invertible,
{
    /// inverse element of the second binary operation.
    fn inv(x: Self::S) -> Self::S {
        Self::Multiplicative::inv(x)
    }
}

/// Create a monoid.
///
/// # Examples
///
/// ```
/// use algolib_rust::monoid;
/// use algolib_rust::traits::algebra::{Associative, Identity, Magma};
///
/// monoid! {
///     Min = (i32, |x, y| std::cmp::min(x, y), i32::MAX)
/// }
/// assert!(Min::associative_check(0, 1, 2));
/// assert!(Min::identity_check(3));
/// assert_eq!(Min::op(3, 5), 3);
/// monoid! {
///     Xor = (u32, |x, y| x^y, 0)
/// }
/// assert!(Xor::associative_check(0, 1, 2));
/// assert!(Xor::identity_check(3));
/// assert_eq!(Xor::op(3, 5), 6);
/// ```
///
/// # Arguments
///
/// * `$name` - The name of the monoid.
/// * `$S` - The type of the set.
/// * `$op` - The binary operation.
/// * `$id` - The identity element.
///
/// # Requirements
///
/// * `$S` must implement `Eq` and `Copy`.
/// * `$op` must be a binary operation on `$S`.
/// * `$id` must be an identity element of `$op`.
///
/// # Notes
///
/// This macro defines a struct named `$name` and implements the following traits for it:
///
/// * `Magma`
/// * `Associative`
/// * `Identity`
///
/// The struct has no fields and is not intended to be instantiated.
#[macro_export]
macro_rules! monoid {
    ( $name:ident = ($S:ty, $op:expr, $id:expr) ) => {
        struct $name;
        impl Magma for $name {
            type S = $S;
            fn op(x: $S, y: $S) -> $S {
                ($op)(x, y)
            }
        }
        impl Associative for $name {}
        impl Identity for $name {
            fn id() -> $S {
                $id
            }
        }
    };
}
