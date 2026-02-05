// Vec3 macros

// $name:ident The ident fragment matches an identifier or keyword.
// $type:ty The ty fragment matches any kind of type expression.
// see also https://lukaswirth.dev/tlborm/decl-macros/minutiae/fragment-specifiers.html
// and https://doc.rust-lang.org/reference/macros-by-example.html#metavariables

// [$($op:ident),+] The repeated fragment
// see also https://doc.rust-lang.org/reference/macros-by-example.html#repetitions

// #[macro_export] see https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing
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
                    let result = {
                        let [a, b, c] = self.0;
                        let [d, e, f] = rhs.0;
                        [
                            a.$op(d),
                            b.$op(e),
                            c.$op(f),
                        ]
                    };
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
                    let result = {
                        let [a, b, c] = self.0;
                        [
                            a.$op(rhs),
                            b.$op(rhs),
                            c.$op(rhs),
                        ]
                    };
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
                    let result = {
                        let [a, b, c] = self.0;
                        let [d, e, f] = rhs.0;
                        [
                            a.$op(d),
                            b.$op(e),
                            c.$op(f),
                        ]
                    };
                    <$output>::new(result)
                }
            }
            impl std::ops::$trait_assign<$rhs> for $lhs {
                fn $op_assign(&mut self, rhs: $rhs) {
                    // $op Need this $trait
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
                    let result = {
                        let [a, b, c] = self.0;
                        [
                            a.$op(rhs),
                            b.$op(rhs),
                            c.$op(rhs),
                        ]
                    };
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

#[macro_export]
/// # Example
/// ```
/// struct A([N; 3]);
///struct B([N; 3]);
///
/// gen_getter! {A[x,y,z]=>N}
/// gen_getter! {B[r,g,b]=>N}
/// ```
macro_rules! gen_getter {
    (
        $type:ty
        [$($name:ident),+]
        =>$return:ty
    ) => {
        $(impl $type {
            pub fn $name(&self) -> $return {
                // ${index()} is nightly feature
                // https://lukaswirth.dev/tlborm/decl-macros/minutiae/metavar-expr.html#indexdepth
                self[${index()}]
            }
        })+
    };
}
#[macro_export]
/// ```
/// gen_builder_lite! {
/// Vector3
/// [
///     with_x,
///     with_y,
///     with_z
/// ]
/// }
/// ```
/// https://matklad.github.io/2022/05/29/builder-lite.html
macro_rules! gen_builder_lite {
    (
        $type:ty[$($name:ident),+]
    ) => {
        $(impl $type {
            pub fn $name(mut self,n:Double) -> Self {
                // ${index()} is nightly feature
                // https://lukaswirth.dev/tlborm/decl-macros/minutiae/metavar-expr.html#indexdepth
                self[${index()}]=n;
                self
            }
        })+
    };
}
/// # Example
/// ```
/// struct N(u32);
/// deref_wrapper! {N=>u32}
/// ```
macro_rules! _deref_wrapper {
    (
        $Wrapper:ty=>$Inner:ty
    ) => {
        impl<T> std::ops::Deref for $Wrapper {
            type Target = $Inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> std::ops::DerefMut for $Wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
