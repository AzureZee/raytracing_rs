// Vec3 Marcos

#[macro_export]
/// ```
///vec3_op_vec3! {
///[Add,Sub,Mul,Div]
///[add,sub,mul,div]
///A,A =>A
///}
///vec3_op_vec3! {
///[Add,Sub,Mul,Div]
///[add,sub,mul,div]
///A,B =>B
///}
/// ```
macro_rules! vec3_op_vec3 {
    (
        [$($trait:ident),+][$($op:ident),+]
        $lhs:ty,$rhs:ty=>$output:ty
    ) => {
        $(
            impl std::ops::$trait<$rhs> for $lhs {
                type Output = $output;
                fn $op(self, rhs: $rhs) -> Self::Output {
                    let mut i = 0;
                    let result = self.map(|n| {
                        let tmp = n.$op(rhs[i]);
                        i += 1;
                        tmp
                    });
                    <$output>::new(result)
                }
            }
        )+
    };
}
#[macro_export]
/// ```
/// struct C([u32; 3]);
/// vec3_op_scalar! {
/// [Add,Sub,Mul,Div]
/// [add,sub,mul,div]
/// C,u32 =>C
/// }
/// ```
macro_rules! vec3_op_scalar {
    (
        [$($trait:ident),+][$($op:ident),+]
        $lhs:ty,$rhs:ty=>$output:ty
    ) => {
        $(
            impl std::ops::$trait<$rhs> for $lhs {
                type Output = Self;
                fn $op(self, rhs: $rhs) -> Self::Output {
                    let result = self.map(|n| {
                        n.$op(rhs)
                    });
                    Self::new(result)
                }
            }
        )+
    };
}
#[macro_export]
/// ```
/// vec3_op_vec3_and_op_assign! {
///     [Add,Sub,Mul,Div]
///     [add,sub,mul,div]
///     [AddAssign,SubAssign,MulAssign,DivAssign]
///     [add_assign,sub_assign,mul_assign,div_assign]
///     A,B =>A
/// }
/// ```
macro_rules! vec3_op_vec3_and_op_assign {
    (
        [$($trait:ident),+]
        [$($op:ident),+]
        [$($trait_assign:ident),+]
        [$($op_assign:ident),+]
        $lhs:ty,$rhs:ty =>$output:ty
    ) => {
        $(
            impl std::ops::$trait<$rhs> for $lhs {
                type Output = $output;
                fn $op(self, rhs: $rhs) -> Self::Output {
                    let mut i = 0;
                    let result = self.map(|n| {
                        let tmp = n.$op(rhs[i]);
                        i += 1;
                        tmp
                    });
                    <$output>::new(result)
                }
            }
            impl std::ops::$trait_assign<$rhs> for $lhs {
                fn $op_assign(&mut self, rhs: $rhs) {
                    // $op Need the $trait
                    use std::ops::$trait;
                    *self = self.$op(rhs);
                }
            }
        )+
    };
}
#[macro_export]
/// ```
/// vec3_op_scalar_and_op_assign! {
///     [Add,Sub,Mul,Div]
///     [add,sub,mul,div]
///     [AddAssign,SubAssign,MulAssign,DivAssign]
///     [add_assign,sub_assign,mul_assign,div_assign]
///     A,B =>A
/// }
/// ```
macro_rules! vec3_op_scalar_and_op_assign {
    (
        [$($trait:ident),+]
        [$($op:ident),+]
        [$($trait_assign:ident),+]
        [$($op_assign:ident),+]
        $lhs:ty,$rhs:ty =>$output:ty
    ) => {
        $(
            impl std::ops::$trait<$rhs> for $lhs {
                type Output = Self;
                fn $op(self, rhs: $rhs) -> Self::Output {
                    let result = self.map(|n| {
                        n.$op(rhs)
                    });
                    Self::new(result)
                }
            }
            impl std::ops::$trait_assign<$rhs> for $lhs {
                fn $op_assign(&mut self, rhs: $rhs) {
                    use std::ops::$trait;
                    *self = self.$op(rhs);
                }
            }
        )+
    };
}


/// # Example
/// ```
/// struct N(u32);
/// deref_wrapper! {N=>u32}
/// ```
macro_rules! _deref_wrapper {
    (
        $wrapper:ty=>$Inner:ty
    ) => {
        impl<T> std::ops::Deref for $wrapper {
            type Target = $Inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
#[macro_export]
/// # Example
/// ```
/// struct A([N; 3]);
///struct B([N; 3]);
///
/// gen_getter! {A[x,y,z]N}
/// gen_getter! {B[r,g,b]N}
/// ```
macro_rules! gen_getter {
    (
        $type:ty
        [$($name:ident),+]
        $return:ty
    ) => {
        $(impl $type {
            pub fn $name(&self) -> $return {
                self[${index()}]
            }
        })+
    };
}
