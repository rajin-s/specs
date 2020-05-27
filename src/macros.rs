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
    ($name:ident -> $member:ident.clone() : $member_type:ty) => {
        pub fn $name(&self) -> $member_type
        {
            return self.$member.clone();
        }
    };
    ($name:ident -> $member:ident.borrow() : $member_type:ty) => {
        pub fn $name(&self) -> $member_type
        {
            return self.$member.borrow();
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

#[macro_export]
macro_rules! get_children {
    () => {
        pub fn get_children(&self) -> Vec<OtherNode>
        {
            Vec::new()
        }
    };
    ($( $get_name:ident, $borrow_name:ident, $borrow_mut_name:ident -> $member:ident ),+,) => {
        $(
            pub fn $get_name(&self) -> OtherNode
            {
                return self.$member.clone();
            }
            pub fn $borrow_name(&self) -> Ref<Node>
            {
                return self.$member.borrow();
            }
            pub fn $borrow_mut_name(&self) -> RefMut<Node>
            {
                return self.$member.borrow_mut();
            }
        )+
        pub fn get_children(&self) -> Vec<OtherNode>
        {
            vec![ $( self.$member.clone() ),* ]
        }
    };
    ($( $get_name:ident, $borrow_name:ident, $borrow_mut_name:ident -> $member:ident ),+) => {
        get_children!{ $($get_name, $borrow_name, $borrow_mut_name -> $member,)+ }
    };
}

#[macro_export]
macro_rules! node {
    { $n:tt } => {
        expand_node!($n)
    };
}

#[allow(unused_macros)]
macro_rules! expand_node {
    ((Sequence $mode:ident; $($x:tt)+)) => {
        Sequence::new(SequenceMode::$mode, vec![ $( expand_node!($x), )+ ]).to_node()
    };
    ((Let $name:expr => $body:tt)) => {
        Binding::new($name.to_owned(), expand_node!($body)).to_node()
    };
    ((Call $op:tt => $($arg:tt)*)) => {
        Call::new(expand_node!($op), vec![ $( expand_node!($arg), )* ]).to_node()
    };
    ((If $cnd:tt => then $thn:tt)) => {
        Conditional::new(expand_node!($cnd), expand_node!($thn)), Node::Nothing.to_node()
    };
    ((If $cnd:tt => then $thn:tt => Else $els:tt)) => {
        Conditional::new(expand_node!($cnd), expand_node!($thn)), expand_node!($els).to_node()
    };

    ((Var $name:expr)) => {
        Variable::new(String::from($name)).to_node()
    };
    ((Op $name:ident)) => {
        PrimitiveOperator::new(primitive::Operator::$name).to_node()
    };
    ((Int $x:expr)) => {
        Integer::new($x).to_node()
    };

    (($name:ident: $($arg:tt)+)) => {
        $name::new( $( expand_node!($arg), )+ ).to_node()
    };

    ({$($x:tt)*}) => { vec![ $( expand_node!($x), )* ] };
    ([sym $x:expr]) => { $x.to_owned() };
    ([$x:path]) => { $x };

    ($x:ident)  => { $x };
    ($x:expr)   => { $x };
}