pub const EXAMPLE_INPUT: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

pub const EXAMPLE_INPUT_2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";