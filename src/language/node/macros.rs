#[macro_export]
macro_rules! get_children {
    {$( $get_name:ident, $get_mut_name:ident -> $member:ident ),+,} => {
        $(
            pub fn $get_name(&self) -> &Node
            {
                return self.$member.as_ref();
            }
            pub fn $get_mut_name(&mut self) -> &mut Node
            {
                return self.$member.as_mut();
            }
        )+
        // pub fn get_children(&self) -> Vec<&Node>
        // {
        //     vec![ $( self.$member.as_ref() ),* ]
        // }
        // pub fn get_children_mut(&mut self) -> Vec<&mut Node>
        // {
        //     vec![ $( self.$member.as_mut() ),* ]
        // }
    };
    {$( $get_name:ident, $get_mut_name:ident -> $member:ident ),+} => {
        get_children!{ $($get_name, $get_mut_name -> $member,)+ }
    };
}

#[macro_export]
macro_rules! impl_recur {
    {
        $data:ty []
    } =>
    {
        impl Recur<Node> for $data {}
    };
    {
        $data:ty
        [ $( $member:ident ),+ ]
    } =>
    {
        impl Recur<Node> for $data
        {
            fn get_children(&self) -> Vec<&Node>
            {
                vec![ $( self.$member.as_ref(), )+ ]
            }
            fn get_children_mut(&mut self) -> Vec<&mut Node>
            {
                vec![ $( self.$member.as_mut(), )+ ]
            }
        }
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