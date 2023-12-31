// Generated bindings for python v2.7.15
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::upper_case_acronyms)]

/* automatically generated by rust-bindgen */

pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
impl Default for __sbuf {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl Default for __sFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type FILE = __sFILE;
pub type Py_ssize_t = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
impl Default for _object {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyObject = _object;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyVarObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
}
impl Default for PyVarObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type unaryfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type binaryfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
pub type ternaryfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type inquiry =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> ::std::os::raw::c_int>;
pub type lenfunc = ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t>;
pub type coercion = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut *mut PyObject,
        arg2: *mut *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type ssizeargfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject,
>;
pub type ssizessizeargfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t) -> *mut PyObject,
>;
pub type ssizeobjargproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type ssizessizeobjargproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: Py_ssize_t,
        arg4: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type objobjargproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type readbufferproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> Py_ssize_t,
>;
pub type writebufferproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: *mut *mut ::std::os::raw::c_void,
    ) -> Py_ssize_t,
>;
pub type segcountproc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut Py_ssize_t) -> Py_ssize_t,
>;
pub type charbufferproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: Py_ssize_t,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> Py_ssize_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bufferinfo {
    pub buf: *mut ::std::os::raw::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: ::std::os::raw::c_int,
    pub ndim: ::std::os::raw::c_int,
    pub format: *mut ::std::os::raw::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2usize],
    pub internal: *mut ::std::os::raw::c_void,
}
impl Default for bufferinfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Py_buffer = bufferinfo;
pub type getbufferproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut Py_buffer,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type releasebufferproc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut Py_buffer)>;
pub type objobjproc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> ::std::os::raw::c_int,
>;
pub type visitproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type traverseproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: visitproc,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_divide: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_nonzero: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_coerce: coercion,
    pub nb_int: unaryfunc,
    pub nb_long: unaryfunc,
    pub nb_float: unaryfunc,
    pub nb_oct: unaryfunc,
    pub nb_hex: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_divide: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
}
impl Default for PyNumberMethods {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub sq_slice: ssizessizeargfunc,
    pub sq_ass_item: ssizeobjargproc,
    pub sq_ass_slice: ssizessizeobjargproc,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
impl Default for PySequenceMethods {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
impl Default for PyMappingMethods {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: readbufferproc,
    pub bf_getwritebuffer: writebufferproc,
    pub bf_getsegcount: segcountproc,
    pub bf_getcharbuffer: charbufferproc,
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
impl Default for PyBufferProcs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type freefunc = ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
pub type destructor = ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject)>;
pub type printfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut FILE,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type getattrfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::std::os::raw::c_char) -> *mut PyObject,
>;
pub type getattrofunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
pub type setattrfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut ::std::os::raw::c_char,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type setattrofunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type cmpfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> ::std::os::raw::c_int,
>;
pub type reprfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type hashfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> ::std::os::raw::c_long>;
pub type richcmpfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: ::std::os::raw::c_int,
    ) -> *mut PyObject,
>;
pub type getiterfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type iternextfunc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject>;
pub type descrgetfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type descrsetfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type initproc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
pub type newfunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut _typeobject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> *mut PyObject,
>;
pub type allocfunc = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut _typeobject, arg2: Py_ssize_t) -> *mut PyObject,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _typeobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const ::std::os::raw::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_compare: cmpfunc,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: ::std::os::raw::c_long,
    pub tp_doc: *const ::std::os::raw::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: ::std::os::raw::c_uint,
}
impl Default for _typeobject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyTypeObject = _typeobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyIntObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_ival: ::std::os::raw::c_long,
}
impl Default for PyIntObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _longobject {
    _unused: [u8; 0],
}
pub type PyLongObject = _longobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyFloatObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_fval: f64,
}
impl Default for PyFloatObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyStringObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_shash: ::std::os::raw::c_long,
    pub ob_sstate: ::std::os::raw::c_int,
    pub ob_sval: [::std::os::raw::c_char; 1usize],
}
impl Default for PyStringObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyTupleObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: [*mut PyObject; 1usize],
}
impl Default for PyTupleObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyListObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: *mut *mut PyObject,
    pub allocated: Py_ssize_t,
}
impl Default for PyListObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyDictEntry {
    pub me_hash: Py_ssize_t,
    pub me_key: *mut PyObject,
    pub me_value: *mut PyObject,
}
impl Default for PyDictEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyDictObject = _dictobject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _dictobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ma_fill: Py_ssize_t,
    pub ma_used: Py_ssize_t,
    pub ma_mask: Py_ssize_t,
    pub ma_table: *mut PyDictEntry,
    pub ma_lookup: ::std::option::Option<
        unsafe extern "C" fn(
            mp: *mut PyDictObject,
            key: *mut PyObject,
            hash: ::std::os::raw::c_long,
        ) -> *mut PyDictEntry,
    >,
    pub ma_smalltable: [PyDictEntry; 8usize],
}
impl Default for _dictobject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyCFunction = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyMethodDef {
    pub ml_name: *const ::std::os::raw::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: ::std::os::raw::c_int,
    pub ml_doc: *const ::std::os::raw::c_char,
}
impl Default for PyMethodDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type getter = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::std::os::raw::c_void) -> *mut PyObject,
>;
pub type setter = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyGetSetDef {
    pub name: *mut ::std::os::raw::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut ::std::os::raw::c_char,
    pub closure: *mut ::std::os::raw::c_void,
}
impl Default for PyGetSetDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _is {
    pub next: *mut _is,
    pub tstate_head: *mut _ts,
    pub modules: *mut PyObject,
    pub sysdict: *mut PyObject,
    pub builtins: *mut PyObject,
    pub modules_reloading: *mut PyObject,
    pub codec_search_path: *mut PyObject,
    pub codec_search_cache: *mut PyObject,
    pub codec_error_registry: *mut PyObject,
    pub dlopenflags: ::std::os::raw::c_int,
}
impl Default for _is {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyInterpreterState = _is;
pub type Py_tracefunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut PyObject,
        arg2: *mut _frame,
        arg3: ::std::os::raw::c_int,
        arg4: *mut PyObject,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ts {
    pub next: *mut _ts,
    pub interp: *mut PyInterpreterState,
    pub frame: *mut _frame,
    pub recursion_depth: ::std::os::raw::c_int,
    pub tracing: ::std::os::raw::c_int,
    pub use_tracing: ::std::os::raw::c_int,
    pub c_profilefunc: Py_tracefunc,
    pub c_tracefunc: Py_tracefunc,
    pub c_profileobj: *mut PyObject,
    pub c_traceobj: *mut PyObject,
    pub curexc_type: *mut PyObject,
    pub curexc_value: *mut PyObject,
    pub curexc_traceback: *mut PyObject,
    pub exc_type: *mut PyObject,
    pub exc_value: *mut PyObject,
    pub exc_traceback: *mut PyObject,
    pub dict: *mut PyObject,
    pub tick_counter: ::std::os::raw::c_int,
    pub gilstate_counter: ::std::os::raw::c_int,
    pub async_exc: *mut PyObject,
    pub thread_id: ::std::os::raw::c_long,
    pub trash_delete_nesting: ::std::os::raw::c_int,
    pub trash_delete_later: *mut PyObject,
}
impl Default for _ts {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyThreadState = _ts;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyCodeObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub co_argcount: ::std::os::raw::c_int,
    pub co_nlocals: ::std::os::raw::c_int,
    pub co_stacksize: ::std::os::raw::c_int,
    pub co_flags: ::std::os::raw::c_int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    pub co_firstlineno: ::std::os::raw::c_int,
    pub co_lnotab: *mut PyObject,
    pub co_zombieframe: *mut ::std::os::raw::c_void,
    pub co_weakreflist: *mut PyObject,
}
impl Default for PyCodeObject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyTryBlock {
    pub b_type: ::std::os::raw::c_int,
    pub b_handler: ::std::os::raw::c_int,
    pub b_level: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _frame {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub f_back: *mut _frame,
    pub f_code: *mut PyCodeObject,
    pub f_builtins: *mut PyObject,
    pub f_globals: *mut PyObject,
    pub f_locals: *mut PyObject,
    pub f_valuestack: *mut *mut PyObject,
    pub f_stacktop: *mut *mut PyObject,
    pub f_trace: *mut PyObject,
    pub f_exc_type: *mut PyObject,
    pub f_exc_value: *mut PyObject,
    pub f_exc_traceback: *mut PyObject,
    pub f_tstate: *mut PyThreadState,
    pub f_lasti: ::std::os::raw::c_int,
    pub f_lineno: ::std::os::raw::c_int,
    pub f_iblock: ::std::os::raw::c_int,
    pub f_blockstack: [PyTryBlock; 20usize],
    pub f_localsplus: [*mut PyObject; 1usize],
}
impl Default for _frame {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type PyFrameObject = _frame;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PyMemberDef {
    pub _address: u8,
}
