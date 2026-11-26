use std::{borrow::Cow, ffi::c_int};
use mluau::{AnyUserData, CallbackResult, CustomError, FromLuaMulti, Function, IntoCallbackResult, IntoLua, IntoLuaMulti, Lua, LuaUserDataExt, Ok as LuaOk, TypedUserData, USERDATA2_TAG, UserData, UserDataFields, UserDataMethods};
use crate::WrappedError; // avoid importing prelude here due to circular deps

/// What a [`WrappedUserData`] type is actually stored as inside Luau.
///
/// It's just the `T`; this newtype only exists because Rust won't let us `impl UserData for T`
/// for every `T: WrappedUserData` directly. Use [`Borrowable`](crate::prelude::Borrowable) to get at the `T`
/// instead of naming this type.
pub struct Wrapped<T>(pub T);

impl<T> std::ops::Deref for Wrapped<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Turns errors returned from userdata methods into [`WrappedError`]s, like the rest of seal's functions.
fn wrap_callback_error(lua: &Lua, result: CallbackResult) -> CallbackResult {
    if let CallbackResult::LuaError(ce) = result {
        return CustomError(WrappedError::from_message(ce.to_string()).into_wrapped()).into_callback_result(lua);
    }
    result
}

pub trait WrappedUserDataMethods<T, const TAG: c_int = USERDATA2_TAG> {
    /// Add a regular method which accepts a `&T` as the first parameter.
    ///
    /// Regular methods are implemented by overriding the `__index` metamethod and returning the
    /// accessed method. This allows them to be used with the expected `userdata:method()` syntax.
    ///
    /// If `add_meta_method` is used to set the `__index` metamethod, the `__index` metamethod will
    /// be used as a fall-back if no regular method is found.
    fn add_method<M, A, R>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult;

    /// Add a regular function which accepts generic arguments.
    ///
    /// The first argument will be a [`AnyUserData`] of type `T` if the method if it is passed in as 
    /// the first argument: `my_userdata.my_method(my_userdata, arg1, arg2)`.
    fn add_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F) where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult;

    /// Add a metamethod which accepts a `&T` as the first parameter.
    fn add_meta_method<M, A, R>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult;

    /// Add a metamethod which accepts generic arguments.
    ///
    /// Metamethods for binary operators can be triggered if either the left or right argument to
    /// the binary operator has a metatable, so the first argument here is not necessarily a
    /// userdata of type `T`.
    fn add_meta_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F)
    where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult;
}

/// Field registry for [`WrappedUserData`] implementors.
pub trait WrappedUserDataFields<T, const TAG: c_int = USERDATA2_TAG> {
    /// Add a static field to the [`UserData`].
    ///
    /// Static fields are implemented by updating the `__index` metamethod and returning the
    /// accessed field. This allows them to be used with the expected `userdata.field` syntax.
    ///
    /// Static fields are usually shared between all instances of the [`UserData`] of the same type.
    /// 
    /// Note: __index is not an allowed name here for performance purposes, use userdata v2 low-level API instead for that
    fn add_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: IntoLua + 'static;

    /// Add a metatable field.
    ///
    /// This will initialize the metatable field with `value` on [`UserData`] creation.
    /// 
    /// Note: __index is not an allowed name here for performance purposes, use userdata v2 low-level API instead for that
    fn add_meta_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: IntoLua + 'static;

    /// Add a regular field getter as a method which accepts a `&T` as the parameter.
    ///
    /// Regular field getters are implemented by overriding the `__index` metamethod and returning
    /// the accessed field. This allows them to be used with the expected `userdata.field` syntax.
    ///
    /// If `add_meta_method` is used to set the `__index` metamethod, the `__index` metamethod will
    /// be used as a fall-back if no regular index property is found.
    fn add_field_method_get<M, R>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &T) -> R + 'static,
        R: IntoCallbackResult;
}

/// Trait for seal's userdata types; errors returned from methods are turned into [`WrappedError`]s.
///
/// See [`UserData`] for information on common userdata implementation notes.
///
/// Wrapped userdata is immutable: methods only ever get a `&T`, so there's no borrow flag to check
/// on every access. Types that need to mutate state should keep a `RefCell` on just that field.
pub trait WrappedUserData<const TAG: c_int = USERDATA2_TAG>: 'static + Sized {
    /// Whether or not to use __namecall optimization. See [`UserData`] 
    /// for more info on what this means.
    const USE_NAMECALL: bool = true;

    /// Type name
    fn type_name<'a>() -> Cow<'a, str>;

    /// Adds custom fields specific to this userdata.
    #[allow(unused_variables)]
    fn add_fields<F: WrappedUserDataFields<Self, TAG>>(fields: &mut F) {}

    /// Adds custom methods and operators specific to this userdata.
    #[allow(unused_variables)]
    fn add_methods<M: WrappedUserDataMethods<Self, TAG>>(methods: &mut M) {}

    fn into_wrapped(self) -> Wrapped<Self> {
        Wrapped(self)
    }
}

impl<const TAG: c_int, T, M> WrappedUserDataMethods<T, TAG> for M
where
    T: 'static,
    M: UserDataMethods<Wrapped<T>, TAG>, 
{
    fn add_method<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult 
    {
        UserDataMethods::<Wrapped<T>, TAG>::add_method(self, name, move |lua, this: TypedUserData<Wrapped<T>, TAG>, args| {
            let result = method(lua, &this.0, args).into_callback_result(lua);
            wrap_callback_error(lua, result)
        });
    }

    fn add_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F)
    where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        UserDataMethods::<Wrapped<T>, TAG>::add_function(self, name, function);
    }

    fn add_meta_method<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        UserDataMethods::<Wrapped<T>, TAG>::add_meta_method(self, name, move |lua, this: TypedUserData<Wrapped<T>, TAG>, args| {
            let result = method(lua, &this.0, args).into_callback_result(lua);
            wrap_callback_error(lua, result)
        });
    }

    fn add_meta_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F)
    where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        UserDataMethods::<Wrapped<T>, TAG>::add_meta_function(self, name, function);
    }
}

impl<const TAG: c_int, T, M> WrappedUserDataFields<T, TAG> for M
where
    T: 'static,
    M: UserDataFields<Wrapped<T>, TAG>,
{
    fn add_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: IntoLua + 'static,
    {
        UserDataFields::<Wrapped<T>, TAG>::add_field(self, name, value);
    }

    fn add_meta_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: IntoLua + 'static,
    {
        UserDataFields::<Wrapped<T>, TAG>::add_meta_field(self, name, value);
    }

    fn add_field_method_get<F, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &T) -> R + 'static,
        R: IntoCallbackResult,
    {
        UserDataFields::<Wrapped<T>, TAG>::add_field_method_get(self, name, move |lua, this: TypedUserData<Wrapped<T>, TAG>| {
            method(lua, &this.0)
        });
    }
}

impl<const TAG: c_int, T: WrappedUserData<TAG>> UserData<TAG> for Wrapped<T> {
    const USE_NAMECALL: bool = T::USE_NAMECALL;

    fn type_name<'a>() -> Cow<'a, str> {
        T::type_name()
    }

    fn add_fields<F: UserDataFields<Self, TAG>>(fields: &mut F) {
        T::add_fields(fields);
    }

    fn add_methods<M: UserDataMethods<Self, TAG>>(methods: &mut M) {
        T::add_methods(methods);
    }
}

pub trait WrappedUserDataExt {
    /// Create a userdata out of a [`WrappedUserData`] type
    fn create_wrapped_userdata<const TAG: c_int, T: WrappedUserData<TAG>>(&self, data: T) -> mluau::Result<AnyUserData>;
}

impl WrappedUserDataExt for Lua {
    fn create_wrapped_userdata<const TAG: c_int, T: WrappedUserData<TAG>>(&self, data: T) -> mluau::Result<AnyUserData> {
        LuaUserDataExt::create_userdata(self, data.into_wrapped())
    }
}

pub trait WrappedFunction {
    fn create_wrapped_function<A, R, F>(&self, func: F) -> mluau::Result<Function>
    where
        A: FromLuaMulti,
        R: IntoLuaMulti,
        F: Fn(&Lua, A) -> mluau::Result<R> + 'static;
}

impl WrappedFunction for Lua {
    fn create_wrapped_function<A, R, F>(&self, func: F) -> mluau::Result<Function>
    where
        A: FromLuaMulti,
        R: IntoLuaMulti,
        F: Fn(&Lua, A) -> mluau::Result<R> + 'static {
        self.create_function(move |lua, args: A| {
            match func(lua, args) {
                Ok(v) => mluau::Either::Left(LuaOk(v)),
                Err(e) => mluau::Either::Right(WrappedError::from_message(e.to_string()))
            }
        })
    }
}

pub trait CallWrapped {
    fn call_wrapped<R>(&self, args: impl IntoLuaMulti) -> mluau::Result<R> 
    where
        R: FromLuaMulti;
}

impl CallWrapped for mluau::Function {
    fn call_wrapped<R>(&self, args: impl IntoLuaMulti) -> mluau::Result<R> 
        where
            R: FromLuaMulti 
    {
        match self.call_with_err::<R, WrappedError>(args) {
            Ok(v) => Ok(v),
            Err(e) => wrap_err!(e)
        }
    }
}

// needed bc Chunk call consumes the chunk
pub trait CallWrappedChunk {
    fn call_wrapped<R>(self, args: impl IntoLuaMulti) -> mluau::Result<R> 
    where
        R: FromLuaMulti;

    fn eval_wrapped<R>(self) -> mluau::Result<R> 
    where
        R: FromLuaMulti;
}

impl<'a> CallWrappedChunk for mluau::Chunk<'a> {
    fn call_wrapped<R>(self, args: impl IntoLuaMulti) -> mluau::Result<R> 
        where
            R: FromLuaMulti 
    {
        let func = self.into_function()?;
        func.call_wrapped(args)
    }

    fn eval_wrapped<R>(self) -> mluau::Result<R> 
    where
        R: FromLuaMulti 
    {
        match self.eval_with_err::<R, WrappedError>() {
            Ok(v) => Ok(v),
            Err(e) => wrap_err!(e)
        }
    }
}