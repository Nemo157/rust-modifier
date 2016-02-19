/// Some implementations for chains of tuples.
///
/// FIXME(reem): Move generation of this to a build script.

use Modifier;

macro_rules! impl_modifier_tuple {
    ($($m:ident)+) => {
        impl<X, $($m: Modifier<X>),+> Modifier<X> for ($($m,)+) {
            #[allow(non_snake_case)]
            fn modify(self, x: &mut X) {
                let ($($m,)+) = self;
                $($m.modify(x));+
            }
        }
    };
}

impl_modifier_tuple! { M1 }
impl_modifier_tuple! { M1 M2 }
impl_modifier_tuple! { M1 M2 M3 }
impl_modifier_tuple! { M1 M2 M3 M4 }
impl_modifier_tuple! { M1 M2 M3 M4 M5 }
impl_modifier_tuple! { M1 M2 M3 M4 M5 M6 }
