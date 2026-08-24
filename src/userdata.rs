use std::{borrow::Cow, cell::RefCell, ffi::c_int};
use mluau::{AnyUserData, CallbackResult, CustomError, FromLua, FromLuaMulti, Function, IntoCallbackResult, IntoLua, IntoLuaMulti, Lua, LuaUserDataExt, Ok as LuaOk, TypedUserData, USERDATA2_TAG, UserData, UserDataFields, UserDataMethods};
use crate::WrappedError; // avoid importing prelude here due to circular deps

pub struct SealLock<T>(pub RefCell<T>);

impl<T> SealLock<T> {
    pub fn new(inner: T) -> Self {
        Self(RefCell::new(inner))
    }
}

impl<T> std::ops::Deref for SealLock<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait SealUserDataMethods<T, const TAG: c_int = USERDATA2_TAG> {
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
    
    /// Add a mutable method which accepts a `&mut T` as the first parameter.
    ///
    /// Mutable methods are implemented by overriding the `__index` metamethod and returning the
    /// accessed method. This allows them to be used with the expected `userdata:method()` syntax.
    ///
    /// If `add_meta_method` or `add_meta_method_mut` is used to set the `__index` metamethod, the `__index` metamethod will
    /// be used as a fall-back if no regular method is found.
    fn add_method_mut<M, A, R>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &mut T, A) -> R + 'static,
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

    /// Add a mutable metamethod which accepts a `&mut T` as the first parameter.
    fn add_meta_method_mut<M, A, R>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &mut T, A) -> R + 'static,
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

/// Field registry for [`SealUserData`] implementors.
pub trait SealUserDataFields<T, const TAG: c_int = USERDATA2_TAG> {
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

    /// Add a regular field getter as a method which accepts a `&T` as the parameter.
    ///
    /// Regular field getters are implemented by overriding the `__index` metamethod and returning
    /// the accessed field. This allows them to be used with the expected `userdata.field` syntax.
    ///
    /// If `add_meta_method` is used to set the `__index` metamethod, the `__index` metamethod will
    /// be used as a fall-back if no regular index property is found.
    fn add_field_method_set<M, A>(&mut self, name: impl Into<&'static str>, method: M)
    where
        M: Fn(&Lua, &mut T, A) -> Result<(), mluau::Error> + 'static, // TODO: fix this later on
        A: FromLua;
}

/// Trait for custom mutable userdata types.
///
/// See [`UserData`] for information on common userdata implementation notes.
/// 
/// Note: mutable methods will error if the userdata is already borrowed. Also note that mutable
/// userdata does incur overhead compared to normal immutable userdata. Be careful: here be dragons, 
/// this api is a *major* footgun. Users be warned.
pub trait SealUserData<const TAG: c_int = USERDATA2_TAG>: 'static + Sized {
    /// Whether or not to use __namecall optimization. See [`UserData`] 
    /// for more info on what this means.
    const USE_NAMECALL: bool = true;

    /// Type name
    fn type_name<'a>() -> Cow<'a, str>;

    /// Adds custom fields specific to this userdata.
    #[allow(unused_variables)]
    fn add_fields<F: SealUserDataFields<Self, TAG>>(fields: &mut F) {}

    /// Adds custom methods and operators specific to this userdata.
    #[allow(unused_variables)]
    fn add_methods<M: SealUserDataMethods<Self, TAG>>(methods: &mut M) {}

    fn into_mut(self) -> SealLock<Self> {
        SealLock::new(self)
    }
}

impl<const TAG: c_int, T, M> SealUserDataMethods<T, TAG> for M
where
    T: 'static,
    M: UserDataMethods<SealLock<T>, TAG>, 
{
    fn add_method<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult 
    {
        self.add_method(name, move |lua, this: TypedUserData<SealLock<T>, TAG>, args| {
            // Wrap method in WrappedError
            let inner = this.0.borrow(); 
            let result = method(lua, &*inner, args).into_callback_result(lua);
            if let CallbackResult::LuaError(ce) = result {
                return CustomError(WrappedError::from_message(ce.to_string()).into_mut()).into_callback_result(lua);
            }
            result
        });
    }

    fn add_method_mut<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &mut T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult 
    {
        self.add_method(name, move |lua, this: TypedUserData<SealLock<T>, TAG>, args| {
            let mut inner = this.0.borrow_mut(); 
            let result = method(lua, &mut *inner, args).into_callback_result(lua);
            if let CallbackResult::LuaError(ce) = result {
                return CustomError(WrappedError::from_message(ce.to_string()).into_mut()).into_callback_result(lua);
            }
            result
        });
    }

    fn add_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F)
    where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        UserDataMethods::<SealLock<T>, TAG>::add_function(self, name, function);
    }

    fn add_meta_method<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        self.add_meta_method(name, move |lua, this: TypedUserData<SealLock<T>, TAG>, args| {
            let inner = this.0.borrow(); 
            let result = method(lua, &*inner, args).into_callback_result(lua);
            if let CallbackResult::LuaError(ce) = result {
                return CustomError(WrappedError::from_message(ce.to_string()).into_mut()).into_callback_result(lua);
            }
            result
        });
    }

    fn add_meta_method_mut<F, A, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &mut T, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        self.add_meta_method(name, move |lua, this: TypedUserData<SealLock<T>, TAG>, args| {
            let mut inner = this.0.borrow_mut(); 
            let result = method(lua, &mut *inner, args).into_callback_result(lua);
            if let CallbackResult::LuaError(ce) = result {
                return CustomError(WrappedError::from_message(ce.to_string()).into_mut()).into_callback_result(lua);
            }
            result
        });
    }

    fn add_meta_function<F, A, R>(&mut self, name: impl Into<&'static str>, function: F)
    where
        F: Fn(&Lua, A) -> R + 'static,
        A: FromLuaMulti,
        R: IntoCallbackResult,
    {
        UserDataMethods::<SealLock<T>, TAG>::add_meta_function(self, name, function);
    }
}

impl<const TAG: c_int, T, M> SealUserDataFields<T, TAG> for M
where
    T: 'static,
    M: UserDataFields<SealLock<T>, TAG>,
{
    fn add_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: crate::IntoLua + 'static,
    {
        UserDataFields::<SealLock<T>, TAG>::add_field(self, name, value);
    }

    fn add_meta_field<V>(&mut self, name: impl Into<&'static str>, value: V)
    where
        V: crate::IntoLua + 'static,
    {
        UserDataFields::<SealLock<T>, TAG>::add_meta_field(self, name, value);
    }

    fn add_field_method_get<F, R>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&crate::Lua, &T) -> R + 'static,
        R: IntoCallbackResult,
    {
        self.add_field_method_get(name, move |lua, this: TypedUserData<SealLock<T>, TAG>| {
            let inner = this.0.borrow();
            method(lua, &*inner)
        });
    }

    fn add_field_method_set<F, A>(&mut self, name: impl Into<&'static str>, method: F)
    where
        F: Fn(&Lua, &mut T, A) -> Result<(), mluau::Error> + 'static,
        A: FromLua,
    {
        self.add_field_method_set(name, move |lua, this: TypedUserData<SealLock<T>, TAG>, args: A| {
            let mut inner = this.0.borrow_mut();
            method(lua, &mut *inner, args)
        });
    }
}

impl<const TAG: c_int, T: SealUserData<TAG>> UserData<TAG> for SealLock<T> {
    const USE_NAMECALL: bool = true;

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

pub trait SealUserDataExt {
    /// Create a mutable userdata
    /// 
    /// The `T` is internally wrapped in a [`SealLock`] for interior mutability purposes
    fn create_seal_userdata<const TAG: c_int, T: SealUserData<TAG>>(&self, data: T) -> mluau::Result<AnyUserData>;
}

impl SealUserDataExt for Lua {
    fn create_seal_userdata<const TAG: c_int, T: SealUserData<TAG>>(&self, data: T) -> mluau::Result<AnyUserData> {
        LuaUserDataExt::create_userdata(self, data.into_mut())
    }
}

pub trait SealUserDataBorrowExt {
    fn with_borrow<T: SealUserData<TAG>, R, const TAG: c_int>(
        &self, 
        f: impl FnOnce(&T) -> R
    ) -> mluau::Result<R>;
}

impl SealUserDataBorrowExt for AnyUserData {
    fn with_borrow<T: SealUserData<TAG>, R, const TAG: c_int>(
        &self, 
        f: impl FnOnce(&T) -> R
    ) -> mluau::Result<R> {
        let tref = self.borrow_with_tag::<SealLock<T>, TAG>()
            .ok_or_else(|| mluau::Error::FromLuaConversionError { 
                from: "userdata", 
                to: T::type_name().to_string(), 
                message: None 
            })?;
        
        let inner = tref.0.try_borrow()
            .map_err(|_| mluau::Error::RuntimeError(format!("{} has a mutable borrow currently", T::type_name())))?;

        Ok(f(&inner))
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