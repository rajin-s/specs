/* -------------------------------------------------------------------------- */
/*                                Helper Macros                               */
/* -------------------------------------------------------------------------- */

#[macro_export]
macro_rules! get {
    ($name:ident -> $member:ident : &$member_type:ty) => {
        pub fn $name(&self) -> &$member_type
        {
            return &self.$member;
        }
    };
    ($name:ident -> $member:ident : &mut $member_type:ty) => {
        pub fn $name(&mut self) -> &mut $member_type
        {
            return &mut self.$member;
        }
    };
    ($name:ident -> $member:ident : $member_type:ty) => {
        pub fn $name(&self) -> $member_type
        {
            return self.$member;
        }
    };
    ($name:ident -> $member:ident.$f:ident() : &mut $member_type:ty) => {
        pub fn $name(&mut self) -> &mut $member_type
        {
            return self.$member.$f();
        }
    };
    ($name:ident -> $member:ident.$f:ident() : $member_type:ty) => {
        pub fn $name(&self) -> $member_type
        {
            return self.$member.$f();
        }
    };
}
#[macro_export]
macro_rules! set {
    ($name:ident -> $member:ident : $member_type:ty) => {
        pub fn $name(&mut self, value: $member_type)
        {
            return self.$member = value;
        }
    };
}

#[macro_export]
macro_rules! hash_set {
    [$( $element:expr ),*,] => {
        {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            $( set.insert($element); )*
            set
        }
    };
    [$( $element:expr ),*] => {
        {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            $( set.insert($element); )*
            set
        }
    };
    [str -> String; $( $element:expr ),*,] => {
        {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            $( set.insert(String::from($element)); )*
            set
        }
    };
    [str -> String; $( $element:expr ),*] => {
        {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            $( set.insert(String::from($element)); )*
            set
        }
    };
}

#[macro_export]
macro_rules! simple_fmt_display {
    {$target:path : $format:expr, $( $item:ident ),*} => {
        impl std::fmt::Display for $target
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(f, $format, $( self.$item, )*)
            }
        }
    };
    {$target:path : $format:expr, $( $item:ident ),*,} => {
        impl std::fmt::Display for $target
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(f, $format, $( self.$item, )*)
            }
        }
    };
    {$target:path : $format:expr, $( $item:ident() ),*} => {
        impl std::fmt::Display for $target
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(f, $format, $( self.$item(), )*)
            }
        }
    };
    {$target:path : $format:expr, $( $item:ident() ),*,} => {
        impl std::fmt::Display for $target
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(f, $format, $( self.$item(), )*)
            }
        }
    };
}