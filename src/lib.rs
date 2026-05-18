use std::env;

use mlua::prelude::*;

fn get_os<T>(_lua: &Lua, _args: Option<T>) -> LuaResult<String> {
    Ok(env::consts::OS.to_owned())
}

fn is_windows<T>(lua: &Lua, _args: Option<T>) -> LuaResult<bool> {
    Ok(get_os::<String>(lua, None)?.to_lowercase() == "windows")
}

fn is_macos<T>(lua: &Lua, _args: Option<T>) -> LuaResult<bool> {
    Ok(get_os::<String>(lua, None)?.to_lowercase() == "macos")
}

fn is_linux<T>(lua: &Lua, _args: Option<T>) -> LuaResult<bool> {
    Ok(get_os::<String>(lua, None)?.to_lowercase() == "linux")
}

#[mlua::lua_module]
fn which_os(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("get_os", lua.create_function(get_os::<String>)?)?;
    exports.set("is_windows", lua.create_function(is_windows::<bool>)?)?;
    exports.set("is_macos", lua.create_function(is_macos::<bool>)?)?;
    exports.set("is_linux", lua.create_function(is_linux::<bool>)?)?;
    Ok(exports)
}
