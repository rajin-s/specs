/* -------------------------------------------------------------------------- */
/*                                Helper Macros                               */
/* -------------------------------------------------------------------------- */

#[macro_export]
macro_rules! get {
    ($member:ident : $name:ident -> &Node) => {
        pub fn $name(&self) -> &Node
        {
            return self.$member.as_ref();
        }
    };
    ($member:ident : $name:ident -> &mut Node) => {
        pub fn $name(&mut self) -> &mut Node
        {
            return self.$member.as_mut();
        }
    };
    ($member:ident : $name:ident -> &$member_type:ty) => {
        pub fn $name(&self) -> &$member_type
        {
            return &self.$member;
        }
    };
    ($member:ident : $name:ident -> &mut $member_type:ty) => {
        pub fn $name(&mut self) -> &mut $member_type
        {
            return &mut self.$member;
        }
    };
    ($member:ident : $name:ident -> $member_type:ty) => {
        pub fn $name(&self) -> $member_type
        {
            return self.$member;
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