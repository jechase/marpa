#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
extern crate libc;

mod test;

/* automatically generated by rust-bindgen */

pub type size_t = ::libc::c_ulong;
pub enum Struct_marpa_g { }
pub enum Struct_marpa_avl_table { }
pub type Marpa_Grammar = *mut Struct_marpa_g;
pub enum Struct_marpa_r { }
pub type Marpa_Recognizer = *mut Struct_marpa_r;
pub type Marpa_Recce = Marpa_Recognizer;
pub enum Struct_marpa_bocage { }
pub type Marpa_Bocage = *mut Struct_marpa_bocage;
pub enum Struct_marpa_order { }
pub type Marpa_Order = *mut Struct_marpa_order;
pub type ORDER = Marpa_Order;
pub enum Struct_marpa_tree { }
pub type Marpa_Tree = *mut Struct_marpa_tree;
pub type Marpa_Value = *mut Struct_marpa_value;
pub type Marpa_Rank = ::libc::c_int;
pub type Marpa_Event_Type = ::libc::c_int;
pub type Marpa_Error_Code = ::libc::c_int;
pub type Marpa_Symbol_ID = ::libc::c_int;
pub type Marpa_NSY_ID = ::libc::c_int;
pub type Marpa_Rule_ID = ::libc::c_int;
pub type Marpa_IRL_ID = ::libc::c_int;
pub type Marpa_AHM_ID = ::libc::c_int;
pub type Marpa_Assertion_ID = ::libc::c_int;
pub type Marpa_Earleme = ::libc::c_int;
pub type Marpa_Earley_Set_ID = ::libc::c_int;
pub type Marpa_Earley_Item_ID = ::libc::c_int;
pub type Marpa_Or_Node_ID = ::libc::c_int;
pub type Marpa_And_Node_ID = ::libc::c_int;
pub type Marpa_Nook_ID = ::libc::c_int;
pub type Marpa_Step_Type = ::libc::c_int;
pub type Marpa_Message_ID = *const ::libc::c_char;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_marpa_config {
    pub t_is_ok: ::libc::c_int,
    pub t_error: Marpa_Error_Code,
    pub t_error_string: *const ::libc::c_char,
}
impl ::std::clone::Clone for Struct_marpa_config {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_marpa_config {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Marpa_Config = Struct_marpa_config;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_marpa_event {
    pub t_type: Marpa_Event_Type,
    pub t_value: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_marpa_event {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_marpa_event {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Marpa_Event = Struct_marpa_event;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_marpa_progress_item {
    pub t_rule_id: Marpa_Rule_ID,
    pub t_position: ::libc::c_int,
    pub t_origin: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_marpa_progress_item {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_marpa_progress_item {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_marpa_value {
    pub t_step_type: Marpa_Step_Type,
    pub t_token_id: Marpa_Symbol_ID,
    pub t_token_value: ::libc::c_int,
    pub t_rule_id: Marpa_Rule_ID,
    pub t_arg_0: ::libc::c_int,
    pub t_arg_n: ::libc::c_int,
    pub t_result: ::libc::c_int,
    pub t_token_start_ys_id: Marpa_Earley_Set_ID,
    pub t_rule_start_ys_id: Marpa_Earley_Set_ID,
    pub t_ys_id: Marpa_Earley_Set_ID,
}
impl ::std::clone::Clone for Struct_marpa_value {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_marpa_value {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static marpa_major_version: ::libc::c_int;
    pub static marpa_minor_version: ::libc::c_int;
    pub static marpa_micro_version: ::libc::c_int;
    pub static marpa__out_of_memory:
    ::std::option::Option<extern "C" fn() -> *mut ::libc::c_void>;
    pub static mut marpa__debug_handler:
    ::std::option::Option<extern "C" fn(arg1:
                                        *const ::libc::c_char, ...)
                                        -> ::libc::c_int>;
    pub static mut marpa__debug_level: ::libc::c_int;
}
extern "C" {
    pub fn marpa__default_debug_handler(format: *const ::libc::c_char, ...)
                                        -> ::libc::c_int;
    pub fn marpa_check_version(required_major: ::libc::c_int,
                               required_minor: ::libc::c_int,
                               required_micro: ::libc::c_int)
                               -> Marpa_Error_Code;
    pub fn marpa_version(version: *mut ::libc::c_int) -> Marpa_Error_Code;
    pub fn marpa_c_init(config: *mut Marpa_Config) -> ::libc::c_int;
    pub fn marpa_c_error(config: *mut Marpa_Config,
                         p_error_string: *mut *const ::libc::c_char)
                         -> Marpa_Error_Code;
    pub fn marpa_g_new(configuration: *mut Marpa_Config) -> Marpa_Grammar;
    pub fn marpa_g_force_valued(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_ref(g: Marpa_Grammar) -> Marpa_Grammar;
    pub fn marpa_g_unref(g: Marpa_Grammar) -> ();
    pub fn marpa_g_start_symbol(g: Marpa_Grammar) -> Marpa_Symbol_ID;
    pub fn marpa_g_start_symbol_set(g: Marpa_Grammar, sym_id: Marpa_Symbol_ID)
                                    -> Marpa_Symbol_ID;
    pub fn marpa_g_highest_symbol_id(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_symbol_is_accessible(g: Marpa_Grammar,
                                        sym_id: Marpa_Symbol_ID)
                                        -> ::libc::c_int;
    pub fn marpa_g_symbol_is_nullable(g: Marpa_Grammar,
                                      sym_id: Marpa_Symbol_ID)
                                      -> ::libc::c_int;
    pub fn marpa_g_symbol_is_nulling(g: Marpa_Grammar,
                                     sym_id: Marpa_Symbol_ID)
                                     -> ::libc::c_int;
    pub fn marpa_g_symbol_is_productive(g: Marpa_Grammar,
                                        sym_id: Marpa_Symbol_ID)
                                        -> ::libc::c_int;
    pub fn marpa_g_symbol_is_start(g: Marpa_Grammar, sym_id: Marpa_Symbol_ID)
                                   -> ::libc::c_int;
    pub fn marpa_g_symbol_is_terminal_set(g: Marpa_Grammar,
                                          sym_id: Marpa_Symbol_ID,
                                          value: ::libc::c_int)
                                          -> ::libc::c_int;
    pub fn marpa_g_symbol_is_terminal(g: Marpa_Grammar,
                                      sym_id: Marpa_Symbol_ID)
                                      -> ::libc::c_int;
    pub fn marpa_g_symbol_new(g: Marpa_Grammar) -> Marpa_Symbol_ID;
    pub fn marpa_g_highest_rule_id(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_rule_is_accessible(g: Marpa_Grammar,
                                      rule_id: Marpa_Rule_ID)
                                      -> ::libc::c_int;
    pub fn marpa_g_rule_is_nullable(g: Marpa_Grammar, ruleid: Marpa_Rule_ID)
                                    -> ::libc::c_int;
    pub fn marpa_g_rule_is_nulling(g: Marpa_Grammar, ruleid: Marpa_Rule_ID)
                                   -> ::libc::c_int;
    pub fn marpa_g_rule_is_loop(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                                -> ::libc::c_int;
    pub fn marpa_g_rule_is_productive(g: Marpa_Grammar,
                                      rule_id: Marpa_Rule_ID)
                                      -> ::libc::c_int;
    pub fn marpa_g_rule_length(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                               -> ::libc::c_int;
    pub fn marpa_g_rule_lhs(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                            -> Marpa_Symbol_ID;
    pub fn marpa_g_rule_new(g: Marpa_Grammar, lhs_id: Marpa_Symbol_ID,
                            rhs_ids: *mut Marpa_Symbol_ID,
                            length: ::libc::c_int) -> Marpa_Rule_ID;
    pub fn marpa_g_rule_rhs(g: Marpa_Grammar, rule_id: Marpa_Rule_ID,
                            ix: ::libc::c_int) -> Marpa_Symbol_ID;
    pub fn marpa_g_rule_is_proper_separation(g: Marpa_Grammar,
                                             rule_id: Marpa_Rule_ID)
                                             -> ::libc::c_int;
    pub fn marpa_g_sequence_min(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                                -> ::libc::c_int;
    pub fn marpa_g_sequence_new(g: Marpa_Grammar, lhs_id: Marpa_Symbol_ID,
                                rhs_id: Marpa_Symbol_ID,
                                separator_id: Marpa_Symbol_ID,
                                min: ::libc::c_int, flags: ::libc::c_int)
                                -> Marpa_Rule_ID;
    pub fn marpa_g_sequence_separator(g: Marpa_Grammar,
                                      rule_id: Marpa_Rule_ID)
                                      -> ::libc::c_int;
    pub fn marpa_g_symbol_is_counted(g: Marpa_Grammar,
                                     sym_id: Marpa_Symbol_ID)
                                     -> ::libc::c_int;
    pub fn marpa_g_rule_rank_set(g: Marpa_Grammar, rule_id: Marpa_Rule_ID,
                                 rank: Marpa_Rank) -> Marpa_Rank;
    pub fn marpa_g_rule_rank(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                             -> Marpa_Rank;
    pub fn marpa_g_rule_null_high_set(g: Marpa_Grammar,
                                      rule_id: Marpa_Rule_ID,
                                      flag: ::libc::c_int) -> ::libc::c_int;
    pub fn marpa_g_rule_null_high(g: Marpa_Grammar, rule_id: Marpa_Rule_ID)
                                  -> ::libc::c_int;
    pub fn marpa_g_completion_symbol_activate(g: Marpa_Grammar,
                                              sym_id: Marpa_Symbol_ID,
                                              reactivate: ::libc::c_int)
                                              -> ::libc::c_int;
    pub fn marpa_g_nulled_symbol_activate(g: Marpa_Grammar,
                                          sym_id: Marpa_Symbol_ID,
                                          reactivate: ::libc::c_int)
                                          -> ::libc::c_int;
    pub fn marpa_g_prediction_symbol_activate(g: Marpa_Grammar,
                                              sym_id: Marpa_Symbol_ID,
                                              reactivate: ::libc::c_int)
                                              -> ::libc::c_int;
    pub fn marpa_g_symbol_is_completion_event(g: Marpa_Grammar,
                                              sym_id: Marpa_Symbol_ID)
                                              -> ::libc::c_int;
    pub fn marpa_g_symbol_is_completion_event_set(g: Marpa_Grammar,
                                                  sym_id: Marpa_Symbol_ID,
                                                  value: ::libc::c_int)
                                                  -> ::libc::c_int;
    pub fn marpa_g_symbol_is_nulled_event(g: Marpa_Grammar,
                                          sym_id: Marpa_Symbol_ID)
                                          -> ::libc::c_int;
    pub fn marpa_g_symbol_is_nulled_event_set(g: Marpa_Grammar,
                                              sym_id: Marpa_Symbol_ID,
                                              value: ::libc::c_int)
                                              -> ::libc::c_int;
    pub fn marpa_g_symbol_is_prediction_event(g: Marpa_Grammar,
                                              sym_id: Marpa_Symbol_ID)
                                              -> ::libc::c_int;
    pub fn marpa_g_symbol_is_prediction_event_set(g: Marpa_Grammar,
                                                  sym_id: Marpa_Symbol_ID,
                                                  value: ::libc::c_int)
                                                  -> ::libc::c_int;
    pub fn marpa_g_precompute(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_is_precomputed(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_has_cycle(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_r_new(g: Marpa_Grammar) -> Marpa_Recognizer;
    pub fn marpa_r_ref(r: Marpa_Recognizer) -> Marpa_Recognizer;
    pub fn marpa_r_unref(r: Marpa_Recognizer) -> ();
    pub fn marpa_r_start_input(r: Marpa_Recognizer) -> ::libc::c_int;
    pub fn marpa_r_alternative(r: Marpa_Recognizer, token_id: Marpa_Symbol_ID,
                               value: ::libc::c_int, length: ::libc::c_int)
                               -> ::libc::c_int;
    pub fn marpa_r_earleme_complete(r: Marpa_Recognizer) -> ::libc::c_int;
    pub fn marpa_r_current_earleme(r: Marpa_Recognizer) -> Marpa_Earleme;
    pub fn marpa_r_earleme(r: Marpa_Recognizer, set_id: Marpa_Earley_Set_ID)
                           -> Marpa_Earleme;
    pub fn marpa_r_earley_set_value(r: Marpa_Recognizer,
                                    earley_set: Marpa_Earley_Set_ID)
                                    -> ::libc::c_int;
    pub fn marpa_r_earley_set_values(r: Marpa_Recognizer,
                                     earley_set: Marpa_Earley_Set_ID,
                                     p_value: *mut ::libc::c_int,
                                     p_pvalue: *mut *mut ::libc::c_void)
                                     -> ::libc::c_int;
    pub fn marpa_r_furthest_earleme(r: Marpa_Recognizer) -> ::libc::c_uint;
    pub fn marpa_r_latest_earley_set(r: Marpa_Recognizer)
                                     -> Marpa_Earley_Set_ID;
    pub fn marpa_r_latest_earley_set_value_set(r: Marpa_Recognizer,
                                               value: ::libc::c_int)
                                               -> ::libc::c_int;
    pub fn marpa_r_latest_earley_set_values_set(r: Marpa_Recognizer,
                                                value: ::libc::c_int,
                                                pvalue: *mut ::libc::c_void)
                                                -> ::libc::c_int;
    pub fn marpa_r_completion_symbol_activate(r: Marpa_Recognizer,
                                              sym_id: Marpa_Symbol_ID,
                                              reactivate: ::libc::c_int)
                                              -> ::libc::c_int;
pub fn marpa_r_earley_item_warning_threshold_set(r: Marpa_Recognizer,
                                                 threshold: ::libc::c_int)
                                                 -> ::libc::c_int;
    pub fn marpa_r_earley_item_warning_threshold(r: Marpa_Recognizer)
     -> ::libc::c_int;
    pub fn marpa_r_expected_symbol_event_set(r: Marpa_Recognizer,
                                             symbol_id: Marpa_Symbol_ID,
                                             value: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_r_is_exhausted(r: Marpa_Recognizer) -> ::libc::c_int;
    pub fn marpa_r_nulled_symbol_activate(r: Marpa_Recognizer,
                                          sym_id: Marpa_Symbol_ID,
                                          boolean: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_r_prediction_symbol_activate(r: Marpa_Recognizer,
                                              sym_id: Marpa_Symbol_ID,
                                              boolean: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_r_terminals_expected(r: Marpa_Recognizer,
                                      buffer: *mut Marpa_Symbol_ID)
     -> ::libc::c_int;
    pub fn marpa_r_terminal_is_expected(r: Marpa_Recognizer,
                                        symbol_id: Marpa_Symbol_ID)
     -> ::libc::c_int;
    pub fn marpa_r_progress_report_reset(r: Marpa_Recognizer)
     -> ::libc::c_int;
    pub fn marpa_r_progress_report_start(r: Marpa_Recognizer,
                                         set_id: Marpa_Earley_Set_ID)
     -> ::libc::c_int;
    pub fn marpa_r_progress_report_finish(r: Marpa_Recognizer)
     -> ::libc::c_int;
    pub fn marpa_r_progress_item(r: Marpa_Recognizer,
                                 position: *mut ::libc::c_int,
                                 origin: *mut Marpa_Earley_Set_ID)
     -> Marpa_Rule_ID;
    pub fn marpa_b_new(r: Marpa_Recognizer,
                       earley_set_ID: Marpa_Earley_Set_ID) -> Marpa_Bocage;
    pub fn marpa_b_ref(b: Marpa_Bocage) -> Marpa_Bocage;
    pub fn marpa_b_unref(b: Marpa_Bocage) -> ();
    pub fn marpa_b_ambiguity_metric(b: Marpa_Bocage) -> ::libc::c_int;
    pub fn marpa_b_is_null(b: Marpa_Bocage) -> ::libc::c_int;
    pub fn marpa_o_new(b: Marpa_Bocage) -> Marpa_Order;
    pub fn marpa_o_ref(o: Marpa_Order) -> Marpa_Order;
    pub fn marpa_o_unref(o: Marpa_Order) -> ();
    pub fn marpa_o_ambiguity_metric(o: Marpa_Order) -> ::libc::c_int;
    pub fn marpa_o_is_null(o: Marpa_Order) -> ::libc::c_int;
    pub fn marpa_o_high_rank_only_set(o: Marpa_Order, flag: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_o_high_rank_only(o: Marpa_Order) -> ::libc::c_int;
    pub fn marpa_o_rank(o: Marpa_Order) -> ::libc::c_int;
    pub fn marpa_t_new(o: Marpa_Order) -> Marpa_Tree;
    pub fn marpa_t_ref(t: Marpa_Tree) -> Marpa_Tree;
    pub fn marpa_t_unref(t: Marpa_Tree) -> ();
    pub fn marpa_t_next(t: Marpa_Tree) -> ::libc::c_int;
    pub fn marpa_t_parse_count(t: Marpa_Tree) -> ::libc::c_int;
    pub fn marpa_v_new(t: Marpa_Tree) -> Marpa_Value;
    pub fn marpa_v_ref(v: Marpa_Value) -> Marpa_Value;
    pub fn marpa_v_unref(v: Marpa_Value) -> ();
    pub fn marpa_v_step(v: Marpa_Value) -> Marpa_Step_Type;
    pub fn marpa_g_event(g: Marpa_Grammar, event: *mut Marpa_Event,
                         ix: ::libc::c_int) -> Marpa_Event_Type;
    pub fn marpa_g_event_count(g: Marpa_Grammar) -> ::libc::c_int;
    pub fn marpa_g_error(g: Marpa_Grammar,
                         p_error_string: *mut *const ::libc::c_char)
     -> Marpa_Error_Code;
    pub fn marpa_g_error_clear(g: Marpa_Grammar) -> Marpa_Error_Code;
    pub fn marpa_g_default_rank_set(g: Marpa_Grammar, rank: Marpa_Rank)
     -> Marpa_Rank;
    pub fn marpa_g_default_rank(g: Marpa_Grammar) -> Marpa_Rank;
    pub fn marpa_g_symbol_rank_set(g: Marpa_Grammar, sym_id: Marpa_Symbol_ID,
                                   rank: Marpa_Rank) -> Marpa_Rank;
    pub fn marpa_g_symbol_rank(g: Marpa_Grammar, sym_id: Marpa_Symbol_ID)
     -> Marpa_Rank;
    pub fn marpa_g_zwa_new(g: Marpa_Grammar, default_value: ::libc::c_int)
     -> Marpa_Assertion_ID;
    pub fn marpa_g_zwa_place(g: Marpa_Grammar, zwaid: Marpa_Assertion_ID,
                             xrl_id: Marpa_Rule_ID, rhs_ix: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_r_zwa_default(r: Marpa_Recognizer, zwaid: Marpa_Assertion_ID)
     -> ::libc::c_int;
    pub fn marpa_r_zwa_default_set(r: Marpa_Recognizer,
                                   zwaid: Marpa_Assertion_ID,
                                   default_value: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_g_highest_zwa_id(g: Marpa_Grammar) -> Marpa_Assertion_ID;
    pub fn marpa_r_clean(r: Marpa_Recognizer) -> Marpa_Earleme;
    pub fn marpa_g_symbol_is_valued_set(g: Marpa_Grammar,
                                        symbol_id: Marpa_Symbol_ID,
                                        value: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_g_symbol_is_valued(g: Marpa_Grammar,
                                    symbol_id: Marpa_Symbol_ID)
     -> ::libc::c_int;
    pub fn marpa_v_symbol_is_valued_set(v: Marpa_Value,
                                        sym_id: Marpa_Symbol_ID,
                                        status: ::libc::c_int)
     -> ::libc::c_int;
    pub fn marpa_v_symbol_is_valued(v: Marpa_Value, sym_id: Marpa_Symbol_ID)
     -> ::libc::c_int;
    pub fn marpa_v_rule_is_valued_set(v: Marpa_Value, rule_id: Marpa_Rule_ID,
                                      status: ::libc::c_int) -> ::libc::c_int;
    pub fn marpa_v_rule_is_valued(v: Marpa_Value, rule_id: Marpa_Rule_ID)
     -> ::libc::c_int;
    pub fn marpa_v_valued_force(v: Marpa_Value) -> ::libc::c_int;
    pub fn marpa_debug_level_set(level: ::libc::c_int) -> ::libc::c_int;
    pub fn marpa_debug_handler_set(debug_handler:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *const ::libc::c_char, ...)
                                                                 ->
                                                                     ::libc::c_int>)
     -> ();
}

/* these were macros / define's in the libmarpa headers */

pub const MARPA_ERR_NONE: Marpa_Error_Code = 0;
