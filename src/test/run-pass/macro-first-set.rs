// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(macro_vis_matcher)]

//{{{ issue 40569 ==============================================================

macro_rules! my_struct {
    ($(#[$meta:meta])* $ident:ident) => {
        $(#[$meta])* struct $ident;
    }
}

my_struct!(#[derive(Debug, PartialEq)] Foo40569);

fn test_40569() {
    assert_eq!(Foo40569, Foo40569);
}

//}}}

//{{{ issue 26444 ==============================================================

macro_rules! foo_26444 {
    ($($beginning:ident),*; $middle:ident; $($end:ident),*) => {
        stringify!($($beginning,)* $middle $(,$end)*)
    }
}

fn test_26444() {
    assert_eq!("a , b , c , d , e", foo_26444!(a, b; c; d, e));
    assert_eq!("f", foo_26444!(; f ;));
}

macro_rules! pat_26444 {
    ($fname:ident $($arg:pat)* =) => {}
}

pat_26444!(foo 1 2 5...7 =);
pat_26444!(bar Some(ref x) Ok(ref mut y) &(w, z) =);

//}}}

//{{{ issue 40984 ==============================================================

macro_rules! thread_local_40984 {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => {
        thread_local_40984!($($rest)*);
    };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => {};
}

thread_local_40984! {
    // no docs
    #[allow(unused)]
    static FOO: i32 = 42;
    /// docs
    pub static BAR: String = String::from("bar");

    // look at these restrictions!!
    pub(crate) static BAZ: usize = 0;
    pub(in foo) static QUUX: usize = 0;
}

//}}}

//{{{ issue 35650 ==============================================================

macro_rules! size {
    ($ty:ty) => {
        std::mem::size_of::<$ty>()
    };
    ($size:tt) => {
        $size
    };
}

fn test_35650() {
    assert_eq!(size!(u64), 8);
    assert_eq!(size!(5), 5);
}

//}}}

//{{{ issue 27832 ==============================================================

macro_rules! m {
    ( $i:ident ) => ();
    ( $t:tt $j:tt ) => ();
}

m!(c);
m!(t 9);
m!(0 9);
m!(struct);
m!(struct Foo);

macro_rules! m2 {
    ( $b:expr ) => ();
    ( $t:tt $u:tt ) => ();
}

m2!(3);
m2!(1 2);
m2!(_ 1);
m2!(enum Foo);

//}}}

//{{{ issue 39964 ==============================================================

macro_rules! foo_39964 {
    ($a:ident) => {};
    (_) => {};
}

foo_39964!(_);

//}}}

//{{{ issue 34030 ==============================================================

macro_rules! foo_34030 {
    ($($t:ident),* /) => {};
}

foo_34030!(a, b/);
foo_34030!(a/);
foo_34030!(/);

//}}}

//{{{ issue 24189 ==============================================================

macro_rules! foo_24189 {
    (
        pub enum $name:ident {
            $( #[$attr:meta] )* $var:ident
        }
    ) => {
        pub enum $name {
            $( #[$attr] )* $var
        }
    };
}

foo_24189! {
    pub enum Foo24189 {
        #[doc = "Bar"] Baz
    }
}

macro_rules! serializable {
    (
        $(#[$struct_meta:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type_:ty
            ),* ,
        }
    ) => {
        $(#[$struct_meta])*
        pub struct $name {
            $(
                $(#[$field_meta])*
                $field: $type_
            ),* ,
        }
    }
}

serializable! {
    #[allow(dead_code)]
    /// This is a test
    pub struct Tester {
        #[allow(dead_code)]
        name: String,
    }
}

macro_rules! foo_24189_c {
    ( $( > )* $x:ident ) => { };
}
foo_24189_c!( > a );

fn test_24189() {
    let _ = Foo24189::Baz;
    let _ = Tester { name: "".to_owned() };
}

//}}}

//{{{ some more tests ==========================================================

macro_rules! test_block {
    (< $($b:block)* >) => {}
}

test_block!(<>);
test_block!(<{}>);
test_block!(<{1}{2}>);

macro_rules! test_ty {
    ($($t:ty),* $(,)*) => {}
}

test_ty!();
test_ty!(,);
test_ty!(u8);
test_ty!(u8,);

macro_rules! test_path {
    ($($t:path),* $(,)*) => {}
}

test_path!();
test_path!(,);
test_path!(::std);
test_path!(std::u8,);
test_path!(any, super, super::super::self::path, X<Y>::Z<'a, T=U>);

macro_rules! test_meta_block {
    ($($m:meta)* $b:block) => {};
}

test_meta_block!(windows {});

//}}}

fn main() {
    test_26444();
    test_40569();
    test_35650();
    test_24189();
}

