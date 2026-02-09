#![allow(nonstandard_style)]

use std::ffi::{c_char, c_double, c_float, c_int, c_void, CStr};
use mluau::ffi::*;

#[repr(C)]
pub struct LuauApi {
    //
    // State manipulation
    //
    pub lua_newstate: unsafe extern "C-unwind" fn(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State,
    pub lua_close: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_newthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut lua_State,
    pub lua_mainthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut lua_State,
    pub lua_resetthread: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_isthreadreset: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,

    //
    // Basic stack manipulation
    //
    pub lua_absindex: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_gettop: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_settop: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_pushvalue: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_remove: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_insert: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_replace: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_checkstack: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: c_int) -> c_int,
    pub lua_rawcheckstack: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: c_int),

    pub lua_xmove: unsafe extern "C-unwind" fn(from: *mut lua_State, to: *mut lua_State, n: c_int),
    pub lua_xpush: unsafe extern "C-unwind" fn(from: *mut lua_State, to: *mut lua_State, idx: c_int),

    //
    // Access functions (stack -> C)
    //
    pub lua_isnumber: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isstring: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_iscfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isLfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_type: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_typename: unsafe extern "C-unwind" fn(state: *mut lua_State, tp: c_int) -> *const c_char,

    pub lua_equal: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,
    pub lua_rawequal: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,
    pub lua_lessthan: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,

    pub lua_tonumberx: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> lua_Number,
    pub lua_tointegerx_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> c_int,
    pub lua_tounsignedx: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> lua_Unsigned,
    pub lua_tovector: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *const c_float,
    pub lua_toboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tolstring: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *const c_char,
    pub lua_tostringatom: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        atom: *mut c_int,
    ) -> *const c_char,
    pub lua_namecallatom: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        atom: *mut c_int,
    ) -> *const c_char,
    pub lua_objlen: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> usize,
    pub lua_tocfunction: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
    ) -> Option<lua_CFunction>,
    pub lua_tolightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut c_void,
    pub lua_tolightuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_touserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut c_void,
    pub lua_touserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_userdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_lightuserdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tothread: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut lua_State,
    pub lua_tobuffer: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *mut c_void,
    pub lua_topointer: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *const c_void,

    //
    // Push functions (C -> stack)
    //
    pub lua_pushnil: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_pushnumber: unsafe extern "C-unwind" fn(state: *mut lua_State, n: lua_Number),
    pub lua_pushinteger_: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_pushunsigned: unsafe extern "C-unwind" fn(state: *mut lua_State, n: lua_Unsigned),
    pub lua_pushvector: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        x: c_float,
        y: c_float,
        z: c_float,
    ),
    pub lua_pushlstring_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        s: *const c_char,
        l: usize,
    ),
    pub lua_pushstring_: unsafe extern "C-unwind" fn(state: *mut lua_State, s: *const c_char),
    // pub lua_pushfstring: unsafe extern "C-unwind" fn(
    //     state: *mut lua_State,
    //     fmt: *const c_char,
    //     ...
    // ) -> *const c_char,
    pub lua_pushcclosurek: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
        nup: c_int,
        cont: Option<lua_Continuation>,
    ),
    pub lua_pushboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, b: c_int),
    pub lua_pushthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,

    pub lua_pushlightuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        p: *mut c_void,
        tag: c_int,
    ),
    pub lua_newuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_newuserdatataggedwithmetatable: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_newuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        dtor: lua_Destructor,
    ) -> *mut c_void,

    pub lua_newbuffer: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: usize) -> *mut c_void,

    //
    // Get functions (Lua -> stack)
    //
    pub lua_gettable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_getfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ) -> c_int,
    pub lua_rawgetfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ) -> c_int,
    pub lua_rawget: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_rawgeti_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, n: c_int) -> c_int,
    pub lua_rawgetptagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        p: *const c_void,
        tag: c_int,
    ) -> c_int,
    pub lua_createtable: unsafe extern "C-unwind" fn(state: *mut lua_State, narr: c_int, nrec: c_int),

    pub lua_setreadonly: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, enabled: c_int),
    pub lua_getreadonly: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_setsafeenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, enabled: c_int),

    pub lua_getmetatable: unsafe extern "C-unwind" fn(state: *mut lua_State, objindex: c_int) -> c_int,
    pub lua_getfenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),

    //
    // Set functions (stack -> Lua)
    //
    pub lua_settable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_setfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ),
    pub lua_rawset: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_rawseti_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, n: c_int),
    pub lua_rawsetptagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        p: *const c_void,
        tag: c_int,
    ),
    pub lua_setmetatable: unsafe extern "C-unwind" fn(state: *mut lua_State, objindex: c_int) -> c_int,
    pub lua_setfenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,

    //
    // load / call
    //
    pub luau_load: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        chunkname: *const c_char,
        data: *const c_char,
        size: usize,
        env: c_int,
    ) -> c_int,
    pub lua_call: unsafe extern "C-unwind" fn(state: *mut lua_State, nargs: c_int, nresults: c_int),
    pub lua_pcall: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
    ) -> c_int,
    pub lua_cpcall: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        ud: *mut c_void,
    ) -> c_int,

    //
    // Coroutine
    //
    pub lua_yield: unsafe extern "C-unwind" fn(state: *mut lua_State, nresults: c_int) -> c_int,
    pub lua_break: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_resume_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        from: *mut lua_State,
        narg: c_int,
    ) -> c_int,
    pub lua_resumeerror: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        from: *mut lua_State,
    ) -> c_int,
    pub lua_status: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_isyieldable: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_getthreaddata: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut c_void,
    pub lua_setthreaddata: unsafe extern "C-unwind" fn(state: *mut lua_State, data: *mut c_void),

    //
    // GC
    //
    pub lua_gc: unsafe extern "C-unwind" fn(state: *mut lua_State, what: c_int, data: c_int) -> c_int,
    pub lua_gcstatename: unsafe extern "C-unwind" fn(state: c_int) -> *const c_char,
    pub lua_gcallocationrate: unsafe extern "C-unwind" fn(state: *mut lua_State) -> i64,

    //
    // Memory stats
    //
    pub lua_setmemcat: unsafe extern "C-unwind" fn(state: *mut lua_State, category: c_int),
    pub lua_totalbytes: unsafe extern "C-unwind" fn(state: *mut lua_State, category: c_int) -> usize,

    //
    // Misc
    //
    pub lua_error: unsafe extern "C-unwind" fn(state: *mut lua_State) -> !,
    pub lua_next: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_rawiter: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, iter: c_int) -> c_int,
    pub lua_concat: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_clock: unsafe extern "C-unwind" fn() -> c_double,
    pub lua_setuserdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, tag: c_int),
    pub lua_setuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
        dtor: Option<lua_Destructor>,
    ),
    pub lua_getuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
    ) -> Option<lua_Destructor>,
    pub lua_setuserdatametatable: unsafe extern "C-unwind" fn(state: *mut lua_State, tag: c_int),
    pub lua_getuserdatametatable: unsafe extern "C-unwind" fn(state: *mut lua_State, tag: c_int),
    pub lua_setlightuserdataname: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
        name: *const c_char,
    ),
    pub lua_getlightuserdataname: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
    ) -> *const c_char,
    pub lua_clonefunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_cleartable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_getallocf: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        ud: *mut *mut c_void,
    ) -> lua_Alloc,

    //
    // Reference system
    //
    pub lua_ref: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_unref: unsafe extern "C-unwind" fn(state: *mut lua_State, r#ref: c_int),

    //
    // Debug API
    //
    pub lua_stackdepth: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_getinfo: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        what: *const c_char,
        ar: *mut lua_Debug,
    ) -> c_int,
    pub lua_getargument: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> c_int,
    pub lua_getlocal: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_setlocal: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_getupvalue: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_setupvalue: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        n: c_int,
    ) -> *const c_char,

    pub lua_singlestep: unsafe extern "C-unwind" fn(state: *mut lua_State, enabled: c_int),
    pub lua_breakpoint: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        line: c_int,
        enabled: c_int,
    ) -> c_int,

    pub lua_getcoverage: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        context: *mut c_void,
        callback: lua_Coverage,
    ),

    pub lua_debugtrace: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *const c_char,

    //
    // Callbacks
    //
    pub lua_callbacks: unsafe extern "C" fn(state: *mut lua_State) -> *mut lua_Callbacks,

    //
    // Customization lib
    //
    pub luau_setfflag: unsafe extern "C" fn(name: *const c_char, value: c_int) -> c_int,
    pub lua_getmetatablepointer: unsafe extern "C" fn(
        state: *mut lua_State,
        idx: c_int,
    ) -> *const c_void,
    pub lua_gcdump: unsafe extern "C" fn(
        state: *mut lua_State,
        file: *mut c_void,
        category_name: Option<unsafe extern "C" fn(state: *mut lua_State, memcat: u8) -> *const c_char>,
    ),

    //
    // luau_try
    //
    // pub luau_try: unsafe extern "C-unwind" fn(
    //     state: *mut lua_State,
    //     func: RustCallback,
    //     data: *mut c_void,
    // ) -> RustCallbackRet,

    //
    // Inline helpers / macros implemented as Rust functions
    //
    pub lua_tonumber: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> lua_Number,
    pub lua_tointeger_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tounsigned: unsafe extern "C-unwind" fn(state: *mut lua_State, i: c_int) -> lua_Unsigned,
    pub lua_pop: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_newtable: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_newuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: usize) -> *mut c_void,
    pub lua_newuserdata_t: unsafe extern "C-unwind" fn(state: *mut lua_State, data: c_void) -> *mut c_void,
    pub lua_isfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_istable: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_islightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnil: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isvector: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isthread: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isbuffer: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnone: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnoneornil: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_pushliteral: unsafe extern "C-unwind" fn(state: *mut lua_State, s: *const c_char),
    pub lua_pushcfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, f: lua_CFunction),
    pub lua_pushcfunctiond: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
    ),
    pub lua_pushcclosure: unsafe extern "C-unwind" fn(state: *mut lua_State, f: lua_CFunction, nup: c_int),
    pub lua_pushcclosurec: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        cont: lua_Continuation,
        nup: c_int,
    ),
    pub lua_pushcclosured: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
        nup: c_int,
    ),
    pub lua_pushlightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, p: *mut c_void),
    pub lua_setglobal: unsafe extern "C-unwind" fn(state: *mut lua_State, var: *const c_char),
    pub lua_getglobal: unsafe extern "C-unwind" fn(state: *mut lua_State, var: *const c_char) -> c_int,
    pub lua_tostring: unsafe extern "C-unwind" fn(state: *mut lua_State, i: c_int) -> *const c_char,

    // --- lauxlib additions ---

    pub luaL_register: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        libname: *const c_char,
        l: *const luaL_Reg,
    ),

    pub luaL_getmetafield_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        obj: c_int,
        e: *const c_char,
    ) -> c_int,

    pub luaL_callmeta: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        obj: c_int,
        e: *const c_char,
    ) -> c_int,

    pub luaL_typeerror: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        tname: *const c_char,
    ) -> !,

    pub luaL_argerror: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        extramsg: *const c_char,
    ) -> !,

    pub luaL_checklstring: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        l: *mut usize,
    ) -> *const c_char,

    pub luaL_optlstring: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        l: *mut usize,
    ) -> *const c_char,

    pub luaL_checknumber: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> lua_Number,

    pub luaL_optnumber: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: lua_Number,
    ) -> lua_Number,

    pub luaL_checkboolean: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> c_int,

    pub luaL_optboolean: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: c_int,
    ) -> c_int,

    pub luaL_checkinteger_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> c_int,

    pub luaL_optinteger_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: c_int,
    ) -> c_int,

    pub luaL_checkunsigned: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> lua_Unsigned,

    pub luaL_optunsigned: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: lua_Unsigned,
    ) -> lua_Unsigned,

    pub luaL_checkvector: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> *const c_float,

    pub luaL_optvector: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_float,
    ) -> *const c_float,

    pub luaL_checkstack_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        sz: c_int,
        msg: *const c_char,
    ),

    pub luaL_checktype: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        t: c_int,
    ),

    pub luaL_checkany: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ),

    pub luaL_newmetatable_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        tname: *const c_char,
    ) -> c_int,

    pub luaL_checkudata: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        ud: c_int,
        tname: *const c_char,
    ) -> *mut c_void,

    pub luaL_checkbuffer: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        len: *mut usize,
    ) -> *mut c_void,

    pub luaL_where: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        lvl: c_int,
    ),

    pub luaL_error: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        fmt: *const c_char,
        // ...,
    ) -> !,

    pub luaL_checkoption: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        lst: *const *const c_char,
    ) -> c_int,

    pub luaL_tolstring_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *const c_char,

    pub luaL_newstate: unsafe extern "C-unwind" fn() -> *mut lua_State,

    pub luaL_findtable: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
        fname: *const c_char,
        szhint: c_int,
    ) -> *const c_char,

    pub luaL_typename: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
    ) -> *const c_char,

    pub luaL_callyieldable: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
    ) -> c_int,

    pub luaL_sandbox_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
    ),

    pub luaL_sandboxthread: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
    ),

    // buffer API
    pub luaL_buffinit: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        B: *mut luaL_Strbuf,
    ),

    pub luaL_buffinitsize: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        B: *mut luaL_Strbuf,
        size: usize,
    ) -> *mut c_char,

    pub luaL_prepbuffsize: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        size: usize,
    ) -> *mut c_char,

    pub luaL_addlstring: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        s: *const c_char,
        l: usize,
    ),

    pub luaL_addvalue: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
    ),

    pub luaL_addvalueany: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        idx: c_int,
    ),

    pub luaL_pushresult: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
    ),

    pub luaL_pushresultsize: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        size: usize,
    ),

}


#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_get_ffi_api() -> *const LuauApi {
    static API: LuauApi = LuauApi {
        // State manipulation
        lua_newstate,
        lua_close,
        lua_newthread,
        lua_mainthread,
        lua_resetthread,
        lua_isthreadreset,

        // Basic stack manipulation
        lua_absindex,
        lua_gettop,
        lua_settop,
        lua_pushvalue,
        lua_remove,
        lua_insert,
        lua_replace,
        lua_checkstack,
        lua_rawcheckstack,
        lua_xmove,
        lua_xpush,

        // Access
        lua_isnumber,
        lua_isstring,
        lua_iscfunction,
        lua_isLfunction,
        lua_isuserdata,
        lua_type,
        lua_typename,
        lua_equal,
        lua_rawequal,
        lua_lessthan,
        lua_tonumberx,
        lua_tointegerx_,
        lua_tounsignedx,
        lua_tovector,
        lua_toboolean,
        lua_tolstring,
        lua_tostringatom,
        lua_namecallatom,
        lua_objlen,
        lua_tocfunction,
        lua_tolightuserdata,
        lua_tolightuserdatatagged,
        lua_touserdata,
        lua_touserdatatagged,
        lua_userdatatag,
        lua_lightuserdatatag,
        lua_tothread,
        lua_tobuffer,
        lua_topointer,

        // Push
        lua_pushnil,
        lua_pushnumber,
        lua_pushinteger_,
        lua_pushunsigned,
        lua_pushvector,
        lua_pushlstring_,
        lua_pushstring_,
        // lua_pushfstring: lua_pushfstring as unsafe extern "C-unwind" fn(
        //     *mut lua_State,
        //     *const c_char,
        //     ...
        // ) -> *const c_char,
        lua_pushcclosurek,
        lua_pushboolean,
        lua_pushthread,
        lua_pushlightuserdatatagged,
        lua_newuserdatatagged,
        lua_newuserdatataggedwithmetatable,
        lua_newuserdatadtor,
        lua_newbuffer,

        // Get
        lua_gettable,
        lua_getfield,
        lua_rawgetfield,
        lua_rawget,
        lua_rawgeti_,
        lua_rawgetptagged,
        lua_createtable,
        lua_setreadonly,
        lua_getreadonly,
        lua_setsafeenv,
        lua_getmetatable,
        lua_getfenv,

        // Set
        lua_settable,
        lua_setfield,
        lua_rawset,
        lua_rawseti_,
        lua_rawsetptagged,
        lua_setmetatable,
        lua_setfenv,

        // Load / call
        luau_load,
        lua_call,
        lua_pcall,
        lua_cpcall,

        // Coroutine
        lua_yield,
        lua_break,
        lua_resume_,
        lua_resumeerror,
        lua_status,
        lua_isyieldable,
        lua_getthreaddata,
        lua_setthreaddata,

        // GC
        lua_gc,
        lua_gcstatename,
        lua_gcallocationrate,

        // Memory stats
        lua_setmemcat,
        lua_totalbytes,

        // Misc
        lua_error,
        lua_next,
        lua_rawiter,
        lua_concat,
        lua_clock,
        lua_setuserdatatag,
        lua_setuserdatadtor,
        lua_getuserdatadtor,
        lua_setuserdatametatable,
        lua_getuserdatametatable,
        lua_setlightuserdataname,
        lua_getlightuserdataname,
        lua_clonefunction,
        lua_cleartable,
        lua_getallocf,

        // Reference system
        lua_ref,
        lua_unref,

        // Debug
        lua_stackdepth,
        lua_getinfo,
        lua_getargument,
        lua_getlocal,
        lua_setlocal,
        lua_getupvalue,
        lua_setupvalue,
        lua_singlestep,
        lua_breakpoint,
        lua_getcoverage,
        lua_debugtrace,

        // Callbacks
        lua_callbacks,

        // Customization
        luau_setfflag,
        lua_getmetatablepointer,
        lua_gcdump,

        // Inline helpers
        lua_tonumber: lua_tonumber_wrap,
        lua_tointeger_: lua_tointeger_wrap,
        lua_tounsigned: lua_tounsigned_wrap,
        lua_pop: lua_pop_wrap,
        lua_newtable: lua_newtable_wrap,
        lua_newuserdata: lua_newuserdata_wrap,
        lua_newuserdata_t: lua_newuserdata_t_wrap,
        lua_isfunction: lua_isfunction_wrap,
        lua_istable: lua_istable_wrap,
        lua_islightuserdata: lua_islightuserdata_wrap,
        lua_isnil: lua_isnil_wrap,
        lua_isboolean: lua_isboolean_wrap,
        lua_isvector: lua_isvector_wrap,
        lua_isthread: lua_isthread_wrap,
        lua_isbuffer: lua_isbuffer_wrap,
        lua_isnone: lua_isnone_wrap,
        lua_isnoneornil: lua_isnoneornil_wrap,
        lua_pushliteral: lua_pushliteral_wrap,
        lua_pushcfunction: lua_pushcfunction_wrap,
        lua_pushcfunctiond: lua_pushcfunctiond_wrap,
        lua_pushcclosure: lua_pushcclosure_wrap,
        lua_pushcclosurec: lua_pushcclosurec_wrap,
        lua_pushcclosured: lua_pushcclosured_wrap,
        lua_pushlightuserdata: lua_pushlightuserdata_wrap,
        lua_setglobal: lua_setglobal_wrap,
        lua_getglobal: lua_getglobal_wrap,
        lua_tostring: lua_tostring_wrap,

        luaL_register,
        luaL_getmetafield_,
        luaL_callmeta,
        luaL_typeerror,
        luaL_argerror,
        luaL_checklstring,
        luaL_optlstring,
        luaL_checknumber,
        luaL_optnumber,
        luaL_checkboolean,
        luaL_optboolean,
        luaL_checkinteger_,
        luaL_optinteger_,
        luaL_checkunsigned,
        luaL_optunsigned,
        luaL_checkvector,
        luaL_optvector,
        luaL_checkstack_,
        luaL_checktype,
        luaL_checkany,
        luaL_newmetatable_,
        luaL_checkudata,
        luaL_checkbuffer,
        luaL_where,
        luaL_error: luaL_error_wrap,
        luaL_checkoption,
        luaL_tolstring_,
        luaL_newstate,
        luaL_findtable,
        luaL_typename,
        luaL_callyieldable,
        luaL_sandbox_,
        luaL_sandboxthread,
        luaL_buffinit,
        luaL_buffinitsize,
        luaL_prepbuffsize,
        luaL_addlstring,
        luaL_addvalue,
        luaL_addvalueany,
        luaL_pushresult,
        luaL_pushresultsize,

    };

    &API
}


unsafe extern "C-unwind" fn lua_tonumber_wrap(state: *mut lua_State, idx: c_int) -> lua_Number {
    unsafe { lua_tonumber(state, idx) }
}

unsafe extern "C-unwind" fn lua_tointeger_wrap(state: *mut lua_State, idx: c_int) -> c_int {
    unsafe { lua_tointeger_(state, idx) }
}

unsafe extern "C-unwind" fn lua_tounsigned_wrap(state: *mut lua_State, i: c_int) -> lua_Unsigned {
    unsafe { lua_tounsigned(state, i) }
}

unsafe extern "C-unwind" fn lua_pop_wrap(state: *mut lua_State, n: c_int) {
    unsafe { lua_pop(state, n) }
}

unsafe extern "C-unwind" fn lua_newtable_wrap(state: *mut lua_State) {
    unsafe { lua_newtable(state) }
}

unsafe extern "C-unwind" fn lua_newuserdata_wrap(state: *mut lua_State, sz: usize) -> *mut c_void {
    unsafe { lua_newuserdata(state, sz) }
}

// matches your `c_void` signature
unsafe extern "C-unwind" fn lua_newuserdata_t_wrap(state: *mut lua_State, data: c_void) -> *mut c_void {
    unsafe { lua_newuserdata_t(state, data) }
}

unsafe extern "C-unwind" fn lua_isfunction_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isfunction(state, n) }
}

unsafe extern "C-unwind" fn lua_istable_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_istable(state, n) }
}

unsafe extern "C-unwind" fn lua_islightuserdata_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_islightuserdata(state, n) }
}

unsafe extern "C-unwind" fn lua_isnil_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isnil(state, n) }
}

unsafe extern "C-unwind" fn lua_isboolean_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isboolean(state, n) }
}

unsafe extern "C-unwind" fn lua_isvector_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isvector(state, n) }
}

unsafe extern "C-unwind" fn lua_isthread_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isthread(state, n) }
}

unsafe extern "C-unwind" fn lua_isbuffer_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isbuffer(state, n) }
}

unsafe extern "C-unwind" fn lua_isnone_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isnone(state, n) }
}

unsafe extern "C-unwind" fn lua_isnoneornil_wrap(state: *mut lua_State, n: c_int) -> c_int {
    unsafe { lua_isnoneornil(state, n) }
}

unsafe extern "C-unwind" fn lua_pushliteral_wrap(state: *mut lua_State, s: *const c_char) {
    // Convert *const c_char → &CStr
    let cstr = unsafe { CStr::from_ptr(s) };
    unsafe { lua_pushliteral(state, cstr) }
}

unsafe extern "C-unwind" fn lua_pushcfunction_wrap(state: *mut lua_State, f: lua_CFunction) {
    unsafe { lua_pushcfunction(state, f) }
}

unsafe extern "C-unwind" fn lua_pushcfunctiond_wrap(
    state: *mut lua_State,
    f: lua_CFunction,
    debugname: *const c_char,
) {
    unsafe { lua_pushcfunctiond(state, f, debugname) }
}

unsafe extern "C-unwind" fn lua_pushcclosure_wrap(
    state: *mut lua_State,
    f: lua_CFunction,
    nup: c_int,
) {
    unsafe { lua_pushcclosure(state, f, nup) }
}

unsafe extern "C-unwind" fn lua_pushcclosurec_wrap(
    state: *mut lua_State,
    f: lua_CFunction,
    cont: lua_Continuation,
    nup: c_int,
) {
    unsafe { lua_pushcclosurec(state, f, cont, nup) }
}

unsafe extern "C-unwind" fn lua_pushcclosured_wrap(
    state: *mut lua_State,
    f: lua_CFunction,
    debugname: *const c_char,
    nup: c_int,
) {
    unsafe { lua_pushcclosured(state, f, debugname, nup) }
}

unsafe extern "C-unwind" fn lua_pushlightuserdata_wrap(state: *mut lua_State, p: *mut c_void) {
    unsafe { lua_pushlightuserdata(state, p) }
}

unsafe extern "C-unwind" fn lua_setglobal_wrap(state: *mut lua_State, var: *const c_char) {
    unsafe { lua_setglobal(state, var) }
}

unsafe extern "C-unwind" fn lua_getglobal_wrap(state: *mut lua_State, var: *const c_char) -> c_int {
    unsafe { lua_getglobal(state, var) }
}

unsafe extern "C-unwind" fn lua_tostring_wrap(state: *mut lua_State, i: c_int) -> *const c_char {
    unsafe { lua_tostring(state, i) }
}

unsafe extern "C-unwind" fn luaL_error_wrap(state: *mut lua_State, fmt: *const c_char) -> ! {
    unsafe { luaL_error(state, fmt) }
}
