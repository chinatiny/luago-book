mod arith_ops;
mod cmp_ops;
mod lua_stack;
mod lua_state;
mod lua_value;
mod math;

pub use self::lua_state::LuaState;

pub fn new_lua_state() -> LuaState {
    LuaState::new()
}
