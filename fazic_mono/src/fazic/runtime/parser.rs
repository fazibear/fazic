use fazic::runtime::ast::{Entry, NodeElement, Opcode};
use fazic::runtime::node_builder::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__all {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use fazic::runtime::ast::{Entry, NodeElement, Opcode};
    use fazic::runtime::node_builder::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3f_22(&'input str),
        Termr_23_22_5c_22_2e_2a_5c_22_22_23(&'input str),
        Termr_23_22_5c_27_2e_2a_5c_27_22_23(&'input str),
        Termr_23_22_28_3fi_29ABS_5c_5c_28_22_23(&'input str),
        Termr_23_22_28_3fi_29AND_22_23(&'input str),
        Termr_23_22_28_3fi_29OR_22_23(&'input str),
        Termr_23_22_28_3fi_29PRINT_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt____all(Entry),
        Ntall(Entry),
        Ntall__list(Vec<NodeElement>),
        Ntcommand(NodeElement),
        Ntexpression(NodeElement),
        Ntfloat(NodeElement),
        Ntfunction(NodeElement),
        Ntinteger(NodeElement),
        Ntprec__1(NodeElement),
        Ntprec__2(NodeElement),
        Ntprec__3(NodeElement),
        Ntstring(NodeElement),
        Ntterm(NodeElement),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 18, 0, 0, 19, 20, 21, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 24, 25, 0, 0, 0, 0,
        // State 5
        0, 0, -27, -27, -27, -27, -27, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0,
        // State 6
        0, 0, -25, -25, -25, -25, -25, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0,
        // State 7
        0, 0, -26, -26, -26, -26, -26, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -12, -12, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 26, 27, 0, -16, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0,
        // State 10
        0, 0, 28, -19, -19, 29, -19, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0,
        // State 11
        0, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0,
        // State 12
        0, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, -22, -22, 0, 0, 0, 0,
        // State 13
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 14
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 15
        0, 0, -24, -24, -24, -24, -24, 0, 0, 0, 0, -24, -24, 0, 0, 0, 0,
        // State 16
        0, 0, -23, -23, -23, -23, -23, 0, 0, 0, 0, -23, -23, 0, 0, 0, 0,
        // State 17
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 18
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 19
        0, 0, -13, -13, -13, -13, -13, 0, 0, 0, 0, -13, -13, 0, 0, 0, 0,
        // State 20
        14, 0, -15, -15, -15, -15, -15, 15, 16, 17, 18, -15, -15, 19, 20, 46, 0,
        // State 21
        14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 18, 0, 0, 19, 20, 46, 0,
        // State 22
        14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 18, 0, 0, 19, 20, 46, 0,
        // State 23
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 24
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 25
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 26
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 27
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 28
        14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0, 0, 20, 46, 0,
        // State 29
        0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 60, 0, 0, 0, 0,
        // State 30
        0, -27, -27, -27, -27, -27, 0, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0,
        // State 31
        0, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, -25, -25, 0, 0, 0, 0,
        // State 32
        0, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0,
        // State 33
        0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, 0, 0, 0, 0,
        // State 34
        0, -16, 0, 61, 62, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0,
        // State 35
        0, -19, 63, -19, -19, 64, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0,
        // State 36
        0, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, -28, -28, 0, 0, 0, 0,
        // State 37
        0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, -22, -22, 0, 0, 0, 0,
        // State 38
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 39
        0, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, -24, -24, 0, 0, 0, 0,
        // State 40
        0, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, -23, -23, 0, 0, 0, 0,
        // State 41
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 42
        0, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, -13, -13, 0, 0, 0, 0,
        // State 43
        0, -15, -15, -15, -15, -15, 0, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 24, 25, 0, 0, 0, 0,
        // State 45
        0, 0, -15, -15, -15, -15, -15, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0,
        // State 46
        0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 60, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 24, 25, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, -11, -11, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, -10, -10, 0, 0, 0, 0,
        // State 53
        0, 0, 28, -17, -17, 29, -17, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0,
        // State 54
        0, 0, 28, -18, -18, 29, -18, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0,
        // State 55
        0, 0, -20, -20, -20, -20, -20, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0,
        // State 56
        0, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, -21, -21, 0, 0, 0, 0,
        // State 57
        0, 0, -29, -29, -29, -29, -29, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0,
        // State 58
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 59
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 60
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 61
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 62
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 63
        39, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 0, 0, 0, 43, 44, 0,
        // State 64
        0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 60, 0, 0, 0, 0,
        // State 65
        0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 60, 0, 0, 0, 0,
        // State 66
        0, 0, -14, -14, -14, -14, -14, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0,
        // State 67
        0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, 0, 0, 0, 0,
        // State 68
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, 0, 0, 0, 0,
        // State 69
        0, -17, 63, -17, -17, 64, 0, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0,
        // State 70
        0, -18, 63, -18, -18, 64, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0,
        // State 71
        0, -20, -20, -20, -20, -20, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0,
        // State 72
        0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, -21, -21, 0, 0, 0, 0,
        // State 73
        0, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0,
        // State 74
        0, -14, -14, -14, -14, -14, 0, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -1,
        -2,
        -4,
        -5,
        -27,
        -25,
        -26,
        -12,
        -16,
        -19,
        -28,
        -22,
        0,
        0,
        -24,
        -23,
        0,
        0,
        -13,
        -15,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -8,
        -15,
        0,
        -9,
        -3,
        -6,
        -7,
        -11,
        -10,
        -17,
        -18,
        -20,
        -21,
        -29,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -14,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 30, 31, 32, 33, 34, 35, 36, 37, 38,
        // State 14
        0, 0, 0, 0, 45, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 47, 31, 32, 33, 34, 35, 36, 37, 38,
        // State 18
        0, 0, 0, 0, 48, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 49, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 21
        0, 0, 50, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 22
        0, 0, 51, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        // State 23
        0, 0, 0, 0, 0, 6, 7, 8, 52, 10, 11, 12, 13,
        // State 24
        0, 0, 0, 0, 0, 6, 7, 8, 53, 10, 11, 12, 13,
        // State 25
        0, 0, 0, 0, 0, 6, 7, 8, 0, 0, 54, 12, 13,
        // State 26
        0, 0, 0, 0, 0, 6, 7, 8, 0, 0, 55, 12, 13,
        // State 27
        0, 0, 0, 0, 0, 6, 7, 8, 0, 0, 0, 12, 56,
        // State 28
        0, 0, 0, 0, 0, 6, 7, 8, 0, 0, 0, 12, 57,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 65, 31, 32, 33, 34, 35, 36, 37, 38,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 66, 31, 32, 33, 34, 35, 36, 37, 38,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 31, 32, 33, 68, 35, 36, 37, 38,
        // State 59
        0, 0, 0, 0, 0, 31, 32, 33, 69, 35, 36, 37, 38,
        // State 60
        0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 70, 37, 38,
        // State 61
        0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 71, 37, 38,
        // State 62
        0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0, 37, 72,
        // State 63
        0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0, 37, 73,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###""?""###,
            r###"r#"\".*\""#"###,
            r###"r#"\'.*\'"#"###,
            r###"r#"(?i)ABS\\("#"###,
            r###"r#"(?i)AND"#"###,
            r###"r#"(?i)OR"#"###,
            r###"r#"(?i)PRINT"#"###,
            r###"r#"[0-9]*\\.[0-9]+"#"###,
            r###"r#"[0-9]+"#"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_all<
        'input,
    >(
        input: &'input str,
    ) -> Result<Entry, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 17 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3f_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Termr_23_22_5c_22_2e_2a_5c_22_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Termr_23_22_5c_27_2e_2a_5c_27_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Termr_23_22_28_3fi_29ABS_5c_5c_28_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_28_3fi_29AND_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Termr_23_22_28_3fi_29OR_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22_28_3fi_29PRINT_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Entry,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __all = all => ActionFn(0);
                let __sym0 = __pop_Ntall(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // all = all_list => ActionFn(1);
                let __sym0 = __pop_Ntall__list(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntall(__nt), __end));
                1
            }
            3 => {
                // all = r#"[0-9]+"#, all_list => ActionFn(2);
                let __sym1 = __pop_Ntall__list(__symbols);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntall(__nt), __end));
                1
            }
            4 => {
                // all_list = command => ActionFn(3);
                let __sym0 = __pop_Ntcommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntall__list(__nt), __end));
                2
            }
            5 => {
                // all_list = expression => ActionFn(4);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntall__list(__nt), __end));
                2
            }
            6 => {
                // all_list = command, ":", all_list => ActionFn(5);
                let __sym2 = __pop_Ntall__list(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_Ntcommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntall__list(__nt), __end));
                2
            }
            7 => {
                // all_list = expression, ":", all_list => ActionFn(6);
                let __sym2 = __pop_Ntall__list(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntall__list(__nt), __end));
                2
            }
            8 => {
                // command = "?", expression => ActionFn(23);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_3f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntcommand(__nt), __end));
                3
            }
            9 => {
                // command = r#"(?i)PRINT"#, expression => ActionFn(24);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Termr_23_22_28_3fi_29PRINT_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntcommand(__nt), __end));
                3
            }
            10 => {
                // expression = expression, r#"(?i)OR"#, prec_1 => ActionFn(7);
                let __sym2 = __pop_Ntprec__1(__symbols);
                let __sym1 = __pop_Termr_23_22_28_3fi_29OR_22_23(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                4
            }
            11 => {
                // expression = expression, r#"(?i)AND"#, prec_1 => ActionFn(8);
                let __sym2 = __pop_Ntprec__1(__symbols);
                let __sym1 = __pop_Termr_23_22_28_3fi_29AND_22_23(__symbols);
                let __sym0 = __pop_Ntexpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                4
            }
            12 => {
                // expression = prec_1 => ActionFn(9);
                let __sym0 = __pop_Ntprec__1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntexpression(__nt), __end));
                4
            }
            13 => {
                // float = r#"[0-9]*\\.[0-9]+"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntfloat(__nt), __end));
                5
            }
            14 => {
                // function = r#"(?i)ABS\\("#, expression, ")" => ActionFn(22);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Termr_23_22_28_3fi_29ABS_5c_5c_28_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntfunction(__nt), __end));
                6
            }
            15 => {
                // integer = r#"[0-9]+"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntinteger(__nt), __end));
                7
            }
            16 => {
                // prec_1 = prec_2 => ActionFn(10);
                let __sym0 = __pop_Ntprec__2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprec__1(__nt), __end));
                8
            }
            17 => {
                // prec_2 = prec_2, "+", prec_3 => ActionFn(11);
                let __sym2 = __pop_Ntprec__3(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Ntprec__2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntprec__2(__nt), __end));
                9
            }
            18 => {
                // prec_2 = prec_2, "-", prec_3 => ActionFn(12);
                let __sym2 = __pop_Ntprec__3(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_Ntprec__2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntprec__2(__nt), __end));
                9
            }
            19 => {
                // prec_2 = prec_3 => ActionFn(13);
                let __sym0 = __pop_Ntprec__3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprec__2(__nt), __end));
                9
            }
            20 => {
                // prec_3 = prec_3, "*", term => ActionFn(14);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_Ntprec__3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntprec__3(__nt), __end));
                10
            }
            21 => {
                // prec_3 = prec_3, "/", term => ActionFn(15);
                let __sym2 = __pop_Ntterm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_Ntprec__3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntprec__3(__nt), __end));
                10
            }
            22 => {
                // prec_3 = term => ActionFn(16);
                let __sym0 = __pop_Ntterm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprec__3(__nt), __end));
                10
            }
            23 => {
                // string = r#"\'.*\'"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5c_27_2e_2a_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstring(__nt), __end));
                11
            }
            24 => {
                // string = r#"\".*\""# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5c_22_2e_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntstring(__nt), __end));
                11
            }
            25 => {
                // term = function => ActionFn(17);
                let __sym0 = __pop_Ntfunction(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                12
            }
            26 => {
                // term = integer => ActionFn(18);
                let __sym0 = __pop_Ntinteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                12
            }
            27 => {
                // term = float => ActionFn(19);
                let __sym0 = __pop_Ntfloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                12
            }
            28 => {
                // term = string => ActionFn(20);
                let __sym0 = __pop_Ntstring(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                12
            }
            29 => {
                // term = "(", expression, ")" => ActionFn(21);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Ntexpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntterm(__nt), __end));
                12
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_2e_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_2e_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_2e_2a_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_2e_2a_5c_27_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3fi_29ABS_5c_5c_28_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3fi_29ABS_5c_5c_28_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3fi_29AND_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3fi_29AND_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3fi_29OR_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3fi_29OR_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_3fi_29PRINT_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_3fi_29PRINT_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____all<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Entry, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____all(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntall<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Entry, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntall(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntall__list<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeElement>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntall__list(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntexpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntexpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntfloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntfloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntfunction<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntfunction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntinteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntinteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprec__1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprec__1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprec__2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprec__2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprec__3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprec__3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntstring<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntstring(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntterm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeElement, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntterm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__all::parse_all;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        34 => /* '\"' */ {
                            __current_state = 1;
                            continue;
                        }
                        39 => /* '\'' */ {
                            __current_state = 2;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_state = 8;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((15, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        63 => /* '?' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_state = 13;
                            continue;
                        }
                        79 => /* 'O' */ {
                            __current_state = 14;
                            continue;
                        }
                        80 => /* 'P' */ {
                            __current_state = 15;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_state = 13;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 14;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            __current_state = 18;
                            continue;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            __current_state = 20;
                            continue;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_state = 8;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((15, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        66 => /* 'B' */ {
                            __current_state = 23;
                            continue;
                        }
                        78 => /* 'N' */ {
                            __current_state = 24;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_state = 23;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        82 => /* 'R' */ {
                            __current_state = 26;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            __current_state = 18;
                            continue;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            __current_state = 18;
                            continue;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            __current_state = 20;
                            continue;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            __current_state = 20;
                            continue;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_state = 8;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((15, __index + __ch.len_utf8()));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        83 => /* 'S' */ {
                            __current_state = 29;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 29;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        68 => /* 'D' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        73 => /* 'I' */ {
                            __current_state = 31;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_state = 31;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        34 => /* '\"' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            __current_state = 18;
                            continue;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        39 => /* '\'' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            __current_state = 20;
                            continue;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        78 => /* 'N' */ {
                            __current_state = 33;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        84 => /* 'T' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Entry, usize),
) -> Entry
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, ast, _): (usize, Vec<NodeElement>, usize),
) -> Entry
{
    { entry_node(None, ast) }
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, line, _): (usize, &'input str, usize),
    (_, ast, _): (usize, Vec<NodeElement>, usize),
) -> Entry
{
    { entry_node(Some(line), ast) }
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, ast, _): (usize, NodeElement, usize),
) -> Vec<NodeElement>
{
    vec![ast]
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, ast, _): (usize, NodeElement, usize),
) -> Vec<NodeElement>
{
    vec![ast]
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, ast, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, rest, _): (usize, Vec<NodeElement>, usize),
) -> Vec<NodeElement>
{
    {
        let mut cmds = vec![ast];
        cmds.extend(rest);
        cmds

    }
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, ast, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, rest, _): (usize, Vec<NodeElement>, usize),
) -> Vec<NodeElement>
{
    {
        let mut cmds = vec![ast];
        cmds.extend(rest);
        cmds
    }
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::Or, l, r)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::And, l, r)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::Add, l, r)
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::Sub, l, r)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::Mul, l, r)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_2_node(Opcode::Div, l, r)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
) -> NodeElement
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, NodeElement, usize),
    (_, _, _): (usize, &'input str, usize),
) -> NodeElement
{
    param_1_node(Opcode::Abs, __0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_1_node(Opcode::Print, __0)
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, NodeElement, usize),
) -> NodeElement
{
    param_1_node(Opcode::Print, __0)
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> NodeElement
{
    string_node(__0)
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> NodeElement
{
    string_node(__0)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> NodeElement
{
    float_node(__0)
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> NodeElement
{
    integer_node(__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
