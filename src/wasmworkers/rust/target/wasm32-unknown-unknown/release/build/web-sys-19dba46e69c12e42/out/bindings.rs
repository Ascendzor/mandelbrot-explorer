#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CanvasRenderingContext2D` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CanvasRenderingContext2d {
    obj: Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CanvasRenderingContext2d: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CanvasRenderingContext2d {
        fn describe() {
            JsValue::describe();
        }
    }
    impl core::ops::Deref for CanvasRenderingContext2d {
        type Target = Object;
        #[inline]
        fn deref(&self) -> &Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CanvasRenderingContext2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CanvasRenderingContext2d {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CanvasRenderingContext2d {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CanvasRenderingContext2d {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CanvasRenderingContext2d {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CanvasRenderingContext2d {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CanvasRenderingContext2d>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CanvasRenderingContext2d {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CanvasRenderingContext2d {
        #[inline]
        fn from(obj: JsValue) -> CanvasRenderingContext2d {
            CanvasRenderingContext2d { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CanvasRenderingContext2d {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CanvasRenderingContext2d> for CanvasRenderingContext2d {
        #[inline]
        fn as_ref(&self) -> &CanvasRenderingContext2d {
            self
        }
    }
    impl From<CanvasRenderingContext2d> for JsValue {
        #[inline]
        fn from(obj: CanvasRenderingContext2d) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CanvasRenderingContext2d {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CanvasRenderingContext2D(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CanvasRenderingContext2D(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CanvasRenderingContext2D(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CanvasRenderingContext2d { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CanvasRenderingContext2d) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CanvasRenderingContext2d> for Object {
    #[inline]
    fn from(obj: CanvasRenderingContext2d) -> Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Object> for CanvasRenderingContext2d {
    #[inline]
    fn as_ref(&self) -> &Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_global_alpha_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `globalAlpha` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn global_alpha(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_global_alpha_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_global_alpha_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_global_alpha_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_global_alpha_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `globalAlpha` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_global_alpha(&self, global_alpha: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_global_alpha_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                global_alpha: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_global_alpha_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            global_alpha: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(global_alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let global_alpha =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(global_alpha);
                __widl_f_set_global_alpha_CanvasRenderingContext2D(self_, global_alpha)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_global_composite_operation_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `globalCompositeOperation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn global_composite_operation(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_global_composite_operation_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_global_composite_operation_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_global_composite_operation_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_global_composite_operation_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `globalCompositeOperation` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_global_composite_operation(
        &self,
        global_composite_operation: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                global_composite_operation: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            global_composite_operation: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(global_composite_operation);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let global_composite_operation =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        global_composite_operation,
                    );
                __widl_f_set_global_composite_operation_CanvasRenderingContext2D(
                    self_,
                    global_composite_operation,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_path_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `beginPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn begin_path(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_path_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_path_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_begin_path_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clip_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `clip()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clip(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clip_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clip_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_clip_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fill()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fill_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `isPointInPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_path_with_f64(&self, x: f64, y: f64) -> bool {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D(self_, x, y)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `isPointInStroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn is_point_in_stroke_with_x_and_y(&self, x: f64, y: f64) -> bool {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D(self_, x, y)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `stroke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_stroke_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `strokeStyle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_style(&self) -> ::wasm_bindgen::JsValue {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_stroke_style_CanvasRenderingContext2D(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_stroke_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `strokeStyle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_stroke_style(&self, stroke_style: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_stroke_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stroke_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_stroke_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stroke_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(stroke_style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let stroke_style =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        stroke_style,
                    );
                __widl_f_set_stroke_style_CanvasRenderingContext2D(self_, stroke_style)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fillStyle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_style(&self) -> ::wasm_bindgen::JsValue {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fill_style_CanvasRenderingContext2D(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_fill_style_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fillStyle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_fill_style(&self, fill_style: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_fill_style_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fill_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_fill_style_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fill_style: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fill_style);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let fill_style =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        fill_style,
                    );
                __widl_f_set_fill_style_CanvasRenderingContext2D(self_, fill_style)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filter_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `filter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn filter(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filter_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filter_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_filter_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_filter_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `filter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_filter(&self, filter: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_filter_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                filter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_filter_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            filter: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(filter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let filter = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(filter);
                __widl_f_set_filter_CanvasRenderingContext2D(self_, filter)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_hit_region_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `addHitRegion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn add_hit_region(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_hit_region_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_hit_region_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_add_hit_region_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_hit_regions_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `clearHitRegions()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clear_hit_regions(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_hit_regions_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_hit_regions_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_clear_hit_regions_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_hit_region_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `removeHitRegion()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn remove_hit_region(&self, id: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_hit_region_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_hit_region_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_remove_hit_region_CanvasRenderingContext2D(self_, id)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `createImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn create_image_data_with_sw_and_sh(
        &self,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D(self_, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `createImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn create_image_data_with_imagedata(
        &self,
        imagedata: &ImageData,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(imagedata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                __widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D(self_, imagedata)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_image_data_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `getImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn get_image_data(
        &self,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_image_data_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_image_data_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sx);
            drop(sy);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                let sw = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_get_image_data_CanvasRenderingContext2D(self_, sx, sy, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_image_data_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `putImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn put_image_data(
        &self,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_image_data_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_image_data_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(imagedata);
            drop(dx);
            drop(dy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                __widl_f_put_image_data_CanvasRenderingContext2D(self_, imagedata, dx, dy)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `putImageData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[allow(clippy::all)]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        &self,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
        dirty_x: f64,
        dirty_y: f64,
        dirty_width: f64,
        dirty_height: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dirty_height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imagedata: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dirty_height: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(imagedata);
            drop(dx);
            drop(dy);
            drop(dirty_x);
            drop(dirty_y);
            drop(dirty_width);
            drop(dirty_height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let imagedata =
                    <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imagedata);
                let dx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dx);
                let dy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dy);
                let dirty_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_x);
                let dirty_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_y);
                let dirty_width =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_width);
                let dirty_height =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dirty_height);
                __widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D ( self_ , imagedata , dx , dy , dirty_x , dirty_y , dirty_width , dirty_height )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_image_smoothing_enabled_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `imageSmoothingEnabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn image_smoothing_enabled(&self) -> bool {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_image_smoothing_enabled_CanvasRenderingContext2D(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `imageSmoothingEnabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_image_smoothing_enabled(&self, image_smoothing_enabled: bool) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image_smoothing_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image_smoothing_enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(image_smoothing_enabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let image_smoothing_enabled =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image_smoothing_enabled);
                __widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D(
                    self_,
                    image_smoothing_enabled,
                )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_line_dash_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `getLineDash()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn get_line_dash(&self) -> ::js_sys::Array {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_line_dash_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_line_dash_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_get_line_dash_CanvasRenderingContext2D(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_dash_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `setLineDash()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_dash(
        &self,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_dash_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                segments: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_dash_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            segments: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(segments);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let segments =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        segments,
                    );
                __widl_f_set_line_dash_CanvasRenderingContext2D(self_, segments)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_width(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_width_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineWidth` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_width(&self, line_width: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_width);
                __widl_f_set_line_width_CanvasRenderingContext2D(self_, line_width)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_cap_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineCap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_cap(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_cap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_cap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_cap_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_cap_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineCap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_cap(&self, line_cap: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_cap_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_cap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_cap_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_cap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_cap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_cap = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_cap);
                __widl_f_set_line_cap_CanvasRenderingContext2D(self_, line_cap)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_join_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineJoin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_join(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_join_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_join_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_join_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_join_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineJoin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_join(&self, line_join: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_join_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_join: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_join_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_join: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_join);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_join = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_join);
                __widl_f_set_line_join_CanvasRenderingContext2D(self_, line_join)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_miter_limit_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `miterLimit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn miter_limit(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_miter_limit_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_miter_limit_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_miter_limit_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_miter_limit_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `miterLimit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_miter_limit(&self, miter_limit: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_miter_limit_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                miter_limit: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_miter_limit_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            miter_limit: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(miter_limit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let miter_limit =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(miter_limit);
                __widl_f_set_miter_limit_CanvasRenderingContext2D(self_, miter_limit)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_dash_offset_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineDashOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_dash_offset(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_dash_offset_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_dash_offset_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_line_dash_offset_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_dash_offset_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineDashOffset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_line_dash_offset(&self, line_dash_offset: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_dash_offset_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_dash_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_dash_offset_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_dash_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_dash_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let line_dash_offset =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_dash_offset);
                __widl_f_set_line_dash_offset_CanvasRenderingContext2D(self_, line_dash_offset)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_arc_CanvasRenderingContext2D(self_, x, y, radius, start_angle, end_angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_with_anticlockwise_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `arc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_arc_with_anticlockwise_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_arc_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `arcTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn arc_to(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_arc_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_arc_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y1: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y2: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x1);
            drop(y1);
            drop(x2);
            drop(y2);
            drop(radius);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x1);
                let y1 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y1);
                let x2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x2);
                let y2 = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y2);
                let radius = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius);
                __widl_f_arc_to_CanvasRenderingContext2D(self_, x1, y1, x2, y2, radius)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bezier_curve_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `bezierCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bezier_curve_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bezier_curve_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp1y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cp2y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cp1x);
            drop(cp1y);
            drop(cp2x);
            drop(cp2y);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let cp1x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1x);
                let cp1y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp1y);
                let cp2x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2x);
                let cp2y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cp2y);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_bezier_curve_to_CanvasRenderingContext2D(
                    self_, cp1x, cp1y, cp2x, cp2y, x, y,
                )
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_path_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `closePath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn close_path(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_path_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_path_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_close_path_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                __widl_f_ellipse_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `ellipse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn ellipse_with_anticlockwise(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radius_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rotation: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            anticlockwise: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(radius_x);
            drop(radius_y);
            drop(rotation);
            drop(start_angle);
            drop(end_angle);
            drop(anticlockwise);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let radius_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_x);
                let radius_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radius_y);
                let rotation = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rotation);
                let start_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_angle);
                let end_angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_angle);
                let anticlockwise =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(anticlockwise);
                __widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D(
                    self_,
                    x,
                    y,
                    radius_x,
                    radius_y,
                    rotation,
                    start_angle,
                    end_angle,
                    anticlockwise,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `lineTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn line_to(&self, x: f64, y: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_line_to_CanvasRenderingContext2D(self_, x, y)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_move_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `moveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn move_to(&self, x: f64, y: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_move_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_move_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_move_to_CanvasRenderingContext2D(self_, x, y)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_quadratic_curve_to_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `quadraticCurveTo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_quadratic_curve_to_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_quadratic_curve_to_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cpy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cpx);
            drop(cpy);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let cpx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpx);
                let cpy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cpy);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_quadratic_curve_to_CanvasRenderingContext2D(self_, cpx, cpy, x, y)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `rect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `clearRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_clear_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fillRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_fill_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_rect_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `strokeRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_rect_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_rect_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            h: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(w);
            drop(h);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let w = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                let h = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(h);
                __widl_f_stroke_rect_CanvasRenderingContext2D(self_, x, y, w, h)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_offset_x_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_offset_x(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_offset_x_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_offset_x_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_offset_x_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_offset_x_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetX` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_offset_x(&self, shadow_offset_x: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_offset_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_offset_x =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_offset_x);
                __widl_f_set_shadow_offset_x_CanvasRenderingContext2D(self_, shadow_offset_x)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_offset_y_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_offset_y(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_offset_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_offset_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_offset_y_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_offset_y_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowOffsetY` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_offset_y(&self, shadow_offset_y: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_offset_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_offset_y =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_offset_y);
                __widl_f_set_shadow_offset_y_CanvasRenderingContext2D(self_, shadow_offset_y)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_blur_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowBlur` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_blur(&self) -> f64 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_blur_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_blur_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_blur_CanvasRenderingContext2D(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_blur_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowBlur` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_blur(&self, shadow_blur: f64) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_blur_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_blur: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_blur_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_blur: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_blur);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_blur =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_blur);
                __widl_f_set_shadow_blur_CanvasRenderingContext2D(self_, shadow_blur)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shadow_color_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn shadow_color(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shadow_color_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shadow_color_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_shadow_color_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shadow_color_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `shadowColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_shadow_color(&self, shadow_color: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shadow_color_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadow_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shadow_color_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadow_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shadow_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let shadow_color =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadow_color);
                __widl_f_set_shadow_color_CanvasRenderingContext2D(self_, shadow_color)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_restore_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `restore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn restore(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_restore_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_restore_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_restore_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_save_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `save()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn save(&self) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_save_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_save_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_save_CanvasRenderingContext2D(self_)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_text_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fillText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_text(&self, text: &str, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_text_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_text_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_fill_text_CanvasRenderingContext2D(self_, text, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fill_text_with_max_width_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `fillText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn fill_text_with_max_width(
        &self,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            drop(max_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let max_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_width);
                __widl_f_fill_text_with_max_width_CanvasRenderingContext2D(
                    self_, text, x, y, max_width,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_text_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `strokeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_text(&self, text: &str, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_text_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_text_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_stroke_text_CanvasRenderingContext2D(self_, text, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stroke_text_with_max_width_CanvasRenderingContext2D()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `strokeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn stroke_text_with_max_width(
        &self,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            drop(x);
            drop(y);
            drop(max_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let max_width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_width);
                __widl_f_stroke_text_with_max_width_CanvasRenderingContext2D(
                    self_, text, x, y, max_width,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_font_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `font` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn font(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_font_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_font_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_font_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_font_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `font` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_font(&self, font: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_font_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_font_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            font: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(font);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let font = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(font);
                __widl_f_set_font_CanvasRenderingContext2D(self_, font)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_align_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `textAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn text_align(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_align_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_align_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_text_align_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_align_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `textAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_text_align(&self, text_align: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_align_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_align_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_align);
                __widl_f_set_text_align_CanvasRenderingContext2D(self_, text_align)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_baseline_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `textBaseline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn text_baseline(&self) -> String {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_baseline_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_baseline_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_text_baseline_CanvasRenderingContext2D(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_baseline_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `textBaseline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_text_baseline(&self, text_baseline: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_baseline_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text_baseline: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_baseline_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text_baseline: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text_baseline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let text_baseline =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text_baseline);
                __widl_f_set_text_baseline_CanvasRenderingContext2D(self_, text_baseline)
            };
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reset_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `resetTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn reset_transform(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reset_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reset_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_reset_transform_CanvasRenderingContext2D(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self, angle: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_CanvasRenderingContext2D(self_, angle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn scale(&self, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_scale_CanvasRenderingContext2D(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `setTransform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn set_transform(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a);
            drop(b);
            drop(c);
            drop(d);
            drop(e);
            drop(f);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let a = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                let b = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                let c = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                let d = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                let e = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                let f = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_set_transform_CanvasRenderingContext2D(self_, a, b, c, d, e, f)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `transform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn transform(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            b: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            c: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            d: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            e: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            f: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a);
            drop(b);
            drop(c);
            drop(d);
            drop(e);
            drop(f);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let a = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                let b = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(b);
                let c = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(c);
                let d = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(d);
                let e = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(e);
                let f = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(f);
                __widl_f_transform_CanvasRenderingContext2D(self_, a, b, c, d, e, f)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_CanvasRenderingContext2D() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasRenderingContext2d as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasRenderingContext2d {
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)\n\n*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[allow(clippy::all)]
    pub fn translate(&self, x: f64, y: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_CanvasRenderingContext2D(
                self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_CanvasRenderingContext2D(
            self_: <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasRenderingContext2d as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_translate_CanvasRenderingContext2D(self_, x, y)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageData` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageData {
    obj: Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageData: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageData {
        fn describe() {
            JsValue::describe();
        }
    }
    impl core::ops::Deref for ImageData {
        type Target = Object;
        #[inline]
        fn deref(&self) -> &Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageData {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageData {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageData {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageData {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageData {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageData>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageData {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageData {
        #[inline]
        fn from(obj: JsValue) -> ImageData {
            ImageData { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageData {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageData> for ImageData {
        #[inline]
        fn as_ref(&self) -> &ImageData {
            self
        }
    }
    impl From<ImageData> for JsValue {
        #[inline]
        fn from(obj: ImageData) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageData {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageData(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageData(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageData(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageData { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageData) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageData> for Object {
    #[inline]
    fn from(obj: ImageData) -> Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Object> for ImageData {
    #[inline]
    fn as_ref(&self) -> &Object {
        use wasm_bindgen::JsCast;
        Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_sw_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_sw(sw: u32, sh: u32) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_sw_ImageData(
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_sw_ImageData(
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_new_with_sw_ImageData(sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_clamped_array_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <::wasm_bindgen::Clamped<&mut [u8]> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_clamped_array(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_clamped_array_ImageData(
                data : < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_clamped_array_ImageData(
            data: <::wasm_bindgen::Clamped<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            drop(sw);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( data ) ;
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                __widl_f_new_with_u8_clamped_array_ImageData(data, sw)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_u8_clamped_array_and_sh_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <::wasm_bindgen::Clamped<&mut [u8]> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ImageData as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn new_with_u8_clamped_array_and_sh(
        data: ::wasm_bindgen::Clamped<&mut [u8]>,
        sw: u32,
        sh: u32,
    ) -> Result<ImageData, ::wasm_bindgen::JsValue> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_u8_clamped_array_and_sh_ImageData(
                data : < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_u8_clamped_array_and_sh_ImageData(
            data: <::wasm_bindgen::Clamped<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sw: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sh: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(data);
            drop(sw);
            drop(sh);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = < :: wasm_bindgen :: Clamped < & mut [ u8 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( data ) ;
                let sw = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sw);
                let sh = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sh);
                __widl_f_new_with_u8_clamped_array_and_sh_ImageData(data, sw, sh)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageData as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_ImageData(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_ImageData(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_ImageData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageData as WasmDescribe>::describe();
    <::wasm_bindgen::Clamped<Vec<u8>> as WasmDescribe>::describe();
}
impl ImageData {
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)\n\n*This API requires the following crate features to be activated: `ImageData`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> ::wasm_bindgen::Clamped<Vec<u8>> {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_ImageData(
                self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_ImageData(
            self_: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_ImageData(self_)
            };
            <::wasm_bindgen::Clamped<Vec<u8>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
}
impl CanvasRenderingContext2d {
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2f3506ee054cf0b9: [u8; 9896usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b".\0\0\0{\"schema_version\":\"0.2.53\",\"version\":\"0.2.53\"}r&\0\0\0\0V\0\0\x02\x18CanvasRenderingContext2D*__widl_instanceof_CanvasRenderingContext2D\0\0\0\0.__widl_f_global_alpha_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BglobalAlpha\x01\x01\x05self_\x0BglobalAlpha\0\0\02__widl_f_set_global_alpha_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BglobalAlpha\x01\x02\x05self_\x0Cglobal_alpha\x0BglobalAlpha\0\0\0<__widl_f_global_composite_operation_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x18globalCompositeOperation\x01\x01\x05self_\x18globalCompositeOperation\0\0\0@__widl_f_set_global_composite_operation_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x18globalCompositeOperation\x01\x02\x05self_\x1Aglobal_composite_operation\x18globalCompositeOperation\0\0\0,__widl_f_begin_path_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\tbeginPath\0\0\0&__widl_f_clip_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04clip\0\0\0&__widl_f_fill_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04fill\0\0\0;__widl_f_is_point_in_path_with_f64_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\risPointInPath\0\0\0A__widl_f_is_point_in_stroke_with_x_and_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x0FisPointInStroke\0\0\0(__widl_f_stroke_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x06stroke\0\0\0.__widl_f_stroke_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BstrokeStyle\x01\x01\x05self_\x0BstrokeStyle\0\0\02__widl_f_set_stroke_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BstrokeStyle\x01\x02\x05self_\x0Cstroke_style\x0BstrokeStyle\0\0\0,__widl_f_fill_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\tfillStyle\x01\x01\x05self_\tfillStyle\0\0\00__widl_f_set_fill_style_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\tfillStyle\x01\x02\x05self_\nfill_style\tfillStyle\0\0\0(__widl_f_filter_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x06filter\x01\x01\x05self_\x06filter\0\0\0,__widl_f_set_filter_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x06filter\x01\x02\x05self_\x06filter\x06filter\0\0\00__widl_f_add_hit_region_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0CaddHitRegion\0\0\03__widl_f_clear_hit_regions_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0FclearHitRegions\0\0\03__widl_f_remove_hit_region_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x02id\x0FremoveHitRegion\0\0\0B__widl_f_create_image_data_with_sw_and_sh_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x02sw\x02sh\x0FcreateImageData\0\0\0B__widl_f_create_image_data_with_imagedata_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\timagedata\x0FcreateImageData\0\0\00__widl_f_get_image_data_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x02sx\x02sy\x02sw\x02sh\x0CgetImageData\0\0\00__widl_f_put_image_data_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\timagedata\x02dx\x02dy\x0CputImageData\0\0\0j__widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x08\x05self_\timagedata\x02dx\x02dy\x07dirty_x\x07dirty_y\x0Bdirty_width\x0Cdirty_height\x0CputImageData\0\0\09__widl_f_image_smoothing_enabled_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x15imageSmoothingEnabled\x01\x01\x05self_\x15imageSmoothingEnabled\0\0\0=__widl_f_set_image_smoothing_enabled_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x15imageSmoothingEnabled\x01\x02\x05self_\x17image_smoothing_enabled\x15imageSmoothingEnabled\0\0\0/__widl_f_get_line_dash_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0BgetLineDash\0\0\0/__widl_f_set_line_dash_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x08segments\x0BsetLineDash\0\0\0,__widl_f_line_width_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\tlineWidth\x01\x01\x05self_\tlineWidth\0\0\00__widl_f_set_line_width_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\tlineWidth\x01\x02\x05self_\nline_width\tlineWidth\0\0\0*__widl_f_line_cap_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x07lineCap\x01\x01\x05self_\x07lineCap\0\0\0.__widl_f_set_line_cap_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x07lineCap\x01\x02\x05self_\x08line_cap\x07lineCap\0\0\0+__widl_f_line_join_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x08lineJoin\x01\x01\x05self_\x08lineJoin\0\0\0/__widl_f_set_line_join_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x08lineJoin\x01\x02\x05self_\tline_join\x08lineJoin\0\0\0-__widl_f_miter_limit_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\nmiterLimit\x01\x01\x05self_\nmiterLimit\0\0\01__widl_f_set_miter_limit_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\nmiterLimit\x01\x02\x05self_\x0Bmiter_limit\nmiterLimit\0\0\02__widl_f_line_dash_offset_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0ElineDashOffset\x01\x01\x05self_\x0ElineDashOffset\0\0\06__widl_f_set_line_dash_offset_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0ElineDashOffset\x01\x02\x05self_\x10line_dash_offset\x0ElineDashOffset\0\0\0%__widl_f_arc_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\x03arc\0\0\08__widl_f_arc_with_anticlockwise_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01x\x01y\x06radius\x0Bstart_angle\tend_angle\ranticlockwise\x03arc\0\0\0(__widl_f_arc_to_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x06\x05self_\x02x1\x02y1\x02x2\x02y2\x06radius\x05arcTo\0\0\01__widl_f_bezier_curve_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x04cp1x\x04cp1y\x04cp2x\x04cp2y\x01x\x01y\rbezierCurveTo\0\0\0,__widl_f_close_path_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\tclosePath\0\0\0)__widl_f_ellipse_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x08\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\x07ellipse\0\0\0<__widl_f_ellipse_with_anticlockwise_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\t\x05self_\x01x\x01y\x08radius_x\x08radius_y\x08rotation\x0Bstart_angle\tend_angle\ranticlockwise\x07ellipse\0\0\0)__widl_f_line_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06lineTo\0\0\0)__widl_f_move_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x06moveTo\0\0\04__widl_f_quadratic_curve_to_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x03cpx\x03cpy\x01x\x01y\x10quadraticCurveTo\0\0\0&__widl_f_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\x04rect\0\0\0,__widl_f_clear_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\tclearRect\0\0\0+__widl_f_fill_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\x08fillRect\0\0\0-__widl_f_stroke_rect_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x01x\x01y\x01w\x01h\nstrokeRect\0\0\01__widl_f_shadow_offset_x_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\rshadowOffsetX\x01\x01\x05self_\rshadowOffsetX\0\0\05__widl_f_set_shadow_offset_x_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\rshadowOffsetX\x01\x02\x05self_\x0Fshadow_offset_x\rshadowOffsetX\0\0\01__widl_f_shadow_offset_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\rshadowOffsetY\x01\x01\x05self_\rshadowOffsetY\0\0\05__widl_f_set_shadow_offset_y_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\rshadowOffsetY\x01\x02\x05self_\x0Fshadow_offset_y\rshadowOffsetY\0\0\0-__widl_f_shadow_blur_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\nshadowBlur\x01\x01\x05self_\nshadowBlur\0\0\01__widl_f_set_shadow_blur_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\nshadowBlur\x01\x02\x05self_\x0Bshadow_blur\nshadowBlur\0\0\0.__widl_f_shadow_color_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0BshadowColor\x01\x01\x05self_\x0BshadowColor\0\0\02__widl_f_set_shadow_color_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0BshadowColor\x01\x02\x05self_\x0Cshadow_color\x0BshadowColor\0\0\0)__widl_f_restore_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x07restore\0\0\0&__widl_f_save_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x04save\0\0\0+__widl_f_fill_text_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04text\x01x\x01y\x08fillText\0\0\0:__widl_f_fill_text_with_max_width_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x04text\x01x\x01y\tmax_width\x08fillText\0\0\0-__widl_f_stroke_text_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x04\x05self_\x04text\x01x\x01y\nstrokeText\0\0\0<__widl_f_stroke_text_with_max_width_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x05\x05self_\x04text\x01x\x01y\tmax_width\nstrokeText\0\0\0&__widl_f_font_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x04font\x01\x01\x05self_\x04font\0\0\0*__widl_f_set_font_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x04font\x01\x02\x05self_\x04font\x04font\0\0\0,__widl_f_text_align_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\ttextAlign\x01\x01\x05self_\ttextAlign\0\0\00__widl_f_set_text_align_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\ttextAlign\x01\x02\x05self_\ntext_align\ttextAlign\0\0\0/__widl_f_text_baseline_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x01\x0CtextBaseline\x01\x01\x05self_\x0CtextBaseline\0\0\03__widl_f_set_text_baseline_CanvasRenderingContext2D\0\0\0\x01\x18CanvasRenderingContext2D\x01\0\x02\x0CtextBaseline\x01\x02\x05self_\rtext_baseline\x0CtextBaseline\0\0\01__widl_f_reset_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x01\x05self_\x0EresetTransform\0\0\0(__widl_f_rotate_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x02\x05self_\x05angle\x06rotate\0\0\0'__widl_f_scale_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\x05scale\0\0\0/__widl_f_set_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01a\x01b\x01c\x01d\x01e\x01f\x0CsetTransform\0\0\0+__widl_f_transform_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x07\x05self_\x01a\x01b\x01c\x01d\x01e\x01f\ttransform\0\0\0+__widl_f_translate_CanvasRenderingContext2D\x01\0\0\x01\x18CanvasRenderingContext2D\x01\0\0\x01\x03\x05self_\x01x\x01y\ttranslate\0\0\x02\tImageData\x1B__widl_instanceof_ImageData\0\0\0\0\x1E__widl_f_new_with_sw_ImageData\x01\0\0\x01\tImageData\0\x01\x02\x02sw\x02sh\x03new\0\0\0,__widl_f_new_with_u8_clamped_array_ImageData\x01\0\0\x01\tImageData\0\x01\x02\x04data\x02sw\x03new\0\0\03__widl_f_new_with_u8_clamped_array_and_sh_ImageData\x01\0\0\x01\tImageData\0\x01\x03\x04data\x02sw\x02sh\x03new\0\0\0\x18__widl_f_width_ImageData\0\0\0\x01\tImageData\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x19__widl_f_height_ImageData\0\0\0\x01\tImageData\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x17__widl_f_data_ImageData\0\0\0\x01\tImageData\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x18web-sys-aacd0323f1b8b2fc\0"
};
pub mod console {
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_assert_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data(condition: bool, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_assert_with_condition_and_data_(condition, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <bool as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_0(condition: bool) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_0_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_0_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                __widl_f_assert_with_condition_and_data_0_(condition)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_1(condition: bool, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_1_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_1_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_assert_with_condition_and_data_1_(condition, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_2(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_2_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_2_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_assert_with_condition_and_data_2_(condition, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_3(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_3_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_3_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_assert_with_condition_and_data_3_(condition, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_4(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_4_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_4_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_assert_with_condition_and_data_4_(
                    condition, data_1, data_2, data_3, data_4,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_5(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_5_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_5_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_assert_with_condition_and_data_5_(
                    condition, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_6(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_6_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_6_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_assert_with_condition_and_data_6_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_assert_with_condition_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <bool as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.assert()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn assert_with_condition_and_data_7(
        condition: bool,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_assert_with_condition_and_data_7_(
                condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_assert_with_condition_and_data_7_(
            condition: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(condition);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let condition = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(condition);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_assert_with_condition_and_data_7_(
                    condition, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_clear_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.clear()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/clear)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn clear() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_clear_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.count()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_count_reset_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_count_reset_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.countReset()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn count_reset_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_count_reset_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_count_reset_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_count_reset_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_debug_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_debug_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_debug_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_debug_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_debug_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_debug_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_debug_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_debug_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_debug_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.debug()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn debug_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_debug_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_debug_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_debug_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dir_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dir_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dir_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dir_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dir_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dir_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dir_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dir_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dir_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dir()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dir_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dir_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_dirxml_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_dirxml_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_dirxml_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_dirxml_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_dirxml_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_dirxml_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_dirxml_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_dirxml_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_dirxml_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.dirxml()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn dirxml_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dirxml_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dirxml_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_dirxml_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_error_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_error_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_error_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_error_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_error_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_error_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_error_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_error_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_error_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.error()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn error_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_error_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_exception_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_exception_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_exception_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_exception_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_exception_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_exception_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_exception_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_exception_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_exception_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.exception()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn exception_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exception_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exception_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_exception_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.group()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_group_collapsed_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_collapsed_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_group_collapsed_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_group_collapsed_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_group_collapsed_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_group_collapsed_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_group_collapsed_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_group_collapsed_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_collapsed_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupCollapsed()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_collapsed_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_collapsed_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_collapsed_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_group_collapsed_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_group_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.groupEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn group_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_group_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_group_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_group_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_info_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_info_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_info_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_info_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_info_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_info_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_info_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_info_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_info_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.info()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn info_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_info_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_info_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_info_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_log_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_log_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_log_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_log_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_log_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_log_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_log_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_log_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_log_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.log()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn log_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_log_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_log_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_log_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profile()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_profile_end_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_profile_end_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_profile_end_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_profile_end_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_profile_end_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_profile_end_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_profile_end_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_profile_end_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_profile_end_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.profileEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn profile_end_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_profile_end_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_profile_end_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_profile_end_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_table_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_table_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_table_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_table_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_table_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_table_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_table_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_table_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_table_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.table()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn table_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_table_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_table_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_table_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.time()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_end_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_end_with_label_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeEnd()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_end_with_label(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_end_with_label_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_end_with_label_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_end_with_label_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_log_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data(label: &str, data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_time_log_with_label_and_data_(label, data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&str as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_0(label: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_0_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_0_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_time_log_with_label_and_data_0_(label)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_1(label: &str, data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_1_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_1_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_time_log_with_label_and_data_1_(label, data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_2(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_2_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_2_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_time_log_with_label_and_data_2_(label, data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_3(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_3_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_3_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_time_log_with_label_and_data_3_(label, data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_4(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_4_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_4_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_time_log_with_label_and_data_4_(label, data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_5(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_5_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_5_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_time_log_with_label_and_data_5_(
                    label, data_1, data_2, data_3, data_4, data_5,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_6(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_6_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_6_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_time_log_with_label_and_data_6_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_log_with_label_and_data_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(8u32);
        <&str as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeLog()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_log_with_label_and_data_7(
        label: &str,
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_log_with_label_and_data_7_(
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_log_with_label_and_data_7_(
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(label);
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_time_log_with_label_and_data_7_(
                    label, data_1, data_2, data_3, data_4, data_5, data_6, data_7,
                )
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_time_stamp_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_with_data_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.timeStamp()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn time_stamp_with_data(data: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_with_data_(
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_with_data_(
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_time_stamp_with_data_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_trace_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_trace_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_trace_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_trace_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_trace_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_trace_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_trace_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_trace_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_trace_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.trace()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn trace_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_trace_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_trace_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_trace_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::js_sys::Array as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn(data: &::js_sys::Array) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_(
                data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_(
            data: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data = <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_warn_(data)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_0_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_0() {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_0_() -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_0_() -> () {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_warn_0_() };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_1_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(1u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_1(data_1: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_1_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_1_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                __widl_f_warn_1_(data_1)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_2_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(2u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_2_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_2_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                __widl_f_warn_2_(data_1, data_2)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_3_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(3u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_3(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_3_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_3_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                __widl_f_warn_3_(data_1, data_2, data_3)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_4_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(4u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_4(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_4_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_4_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                __widl_f_warn_4_(data_1, data_2, data_3, data_4)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_5_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(5u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_5(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_5_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_5_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                __widl_f_warn_5_(data_1, data_2, data_3, data_4, data_5)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_6_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(6u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_6(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_6_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_6_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                __widl_f_warn_6_(data_1, data_2, data_3, data_4, data_5, data_6)
            };
            ()
        }
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    #[doc(hidden)]
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    #[allow(clippy::all)]
    pub extern "C" fn __wbindgen_describe___widl_f_warn_7_() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(7u32);
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
    #[allow(bad_style)]
    #[doc = "The `console.warn()` function\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)\n\n*This API requires the following crate features to be activated: `console`*"]
    #[allow(clippy::all)]
    pub fn warn_7(
        data_1: &::wasm_bindgen::JsValue,
        data_2: &::wasm_bindgen::JsValue,
        data_3: &::wasm_bindgen::JsValue,
        data_4: &::wasm_bindgen::JsValue,
        data_5: &::wasm_bindgen::JsValue,
        data_6: &::wasm_bindgen::JsValue,
        data_7: &::wasm_bindgen::JsValue,
    ) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_warn_7_(
                data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_warn_7_(
            data_1: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_2: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_3: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_4: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_5: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_6: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data_7: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(data_1);
            drop(data_2);
            drop(data_3);
            drop(data_4);
            drop(data_5);
            drop(data_6);
            drop(data_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                 non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let data_1 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_1,
                    );
                let data_2 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_2,
                    );
                let data_3 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_3,
                    );
                let data_4 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_4,
                    );
                let data_5 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_5,
                    );
                let data_6 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_6,
                    );
                let data_7 =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data_7,
                    );
                __widl_f_warn_7_(data_1, data_2, data_3, data_4, data_5, data_6, data_7)
            };
            ()
        }
    }
    #[allow(non_upper_case_globals)]
    #[cfg(target_arch = "wasm32")]
    #[link_section = "__wasm_bindgen_unstable"]
    #[doc(hidden)]
    #[allow(clippy::all)]
    pub static __WASM_BINDGEN_GENERATED_58fa958c32b5d36a: [u8; 10756usize] = {
        static _INCLUDED_FILES: &[&str] = &[];
        * b".\0\0\0{\"schema_version\":\"0.2.53\",\"version\":\"0.2.53\"}\xCE)\0\0\0\0\x9E\x01\0\x01\x07console\0\x10__widl_f_assert_\0\0\0\0\x01\0\x06assert\0\x01\x07console\0(__widl_f_assert_with_condition_and_data_\0\x01\0\0\x01\x02\tcondition\x04data\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_0_\0\0\0\0\x01\x01\tcondition\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_1_\0\0\0\0\x01\x02\tcondition\x06data_1\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_2_\0\0\0\0\x01\x03\tcondition\x06data_1\x06data_2\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_3_\0\0\0\0\x01\x04\tcondition\x06data_1\x06data_2\x06data_3\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_4_\0\0\0\0\x01\x05\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_5_\0\0\0\0\x01\x06\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_6_\0\0\0\0\x01\x07\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06assert\0\x01\x07console\0*__widl_f_assert_with_condition_and_data_7_\0\0\0\0\x01\x08\tcondition\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06assert\0\x01\x07console\0\x0F__widl_f_clear_\0\0\0\0\x01\0\x05clear\0\x01\x07console\0\x0F__widl_f_count_\0\0\0\0\x01\0\x05count\0\x01\x07console\0\x1A__widl_f_count_with_label_\0\0\0\0\x01\x01\x05label\x05count\0\x01\x07console\0\x15__widl_f_count_reset_\0\0\0\0\x01\0\ncountReset\0\x01\x07console\0 __widl_f_count_reset_with_label_\0\0\0\0\x01\x01\x05label\ncountReset\0\x01\x07console\0\x0F__widl_f_debug_\0\x01\0\0\x01\x01\x04data\x05debug\0\x01\x07console\0\x11__widl_f_debug_0_\0\0\0\0\x01\0\x05debug\0\x01\x07console\0\x11__widl_f_debug_1_\0\0\0\0\x01\x01\x06data_1\x05debug\0\x01\x07console\0\x11__widl_f_debug_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05debug\0\x01\x07console\0\x11__widl_f_debug_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05debug\0\x01\x07console\0\x11__widl_f_debug_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05debug\0\x01\x07console\0\x11__widl_f_debug_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05debug\0\x01\x07console\0\x11__widl_f_debug_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05debug\0\x01\x07console\0\x11__widl_f_debug_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05debug\0\x01\x07console\0\r__widl_f_dir_\0\x01\0\0\x01\x01\x04data\x03dir\0\x01\x07console\0\x0F__widl_f_dir_0_\0\0\0\0\x01\0\x03dir\0\x01\x07console\0\x0F__widl_f_dir_1_\0\0\0\0\x01\x01\x06data_1\x03dir\0\x01\x07console\0\x0F__widl_f_dir_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03dir\0\x01\x07console\0\x0F__widl_f_dir_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03dir\0\x01\x07console\0\x0F__widl_f_dir_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03dir\0\x01\x07console\0\x0F__widl_f_dir_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03dir\0\x01\x07console\0\x0F__widl_f_dir_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03dir\0\x01\x07console\0\x0F__widl_f_dir_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03dir\0\x01\x07console\0\x10__widl_f_dirxml_\0\x01\0\0\x01\x01\x04data\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_0_\0\0\0\0\x01\0\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_1_\0\0\0\0\x01\x01\x06data_1\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06dirxml\0\x01\x07console\0\x12__widl_f_dirxml_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x06dirxml\0\x01\x07console\0\x0F__widl_f_error_\0\x01\0\0\x01\x01\x04data\x05error\0\x01\x07console\0\x11__widl_f_error_0_\0\0\0\0\x01\0\x05error\0\x01\x07console\0\x11__widl_f_error_1_\0\0\0\0\x01\x01\x06data_1\x05error\0\x01\x07console\0\x11__widl_f_error_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05error\0\x01\x07console\0\x11__widl_f_error_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05error\0\x01\x07console\0\x11__widl_f_error_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05error\0\x01\x07console\0\x11__widl_f_error_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05error\0\x01\x07console\0\x11__widl_f_error_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05error\0\x01\x07console\0\x11__widl_f_error_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05error\0\x01\x07console\0\x13__widl_f_exception_\0\x01\0\0\x01\x01\x04data\texception\0\x01\x07console\0\x15__widl_f_exception_0_\0\0\0\0\x01\0\texception\0\x01\x07console\0\x15__widl_f_exception_1_\0\0\0\0\x01\x01\x06data_1\texception\0\x01\x07console\0\x15__widl_f_exception_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\texception\0\x01\x07console\0\x15__widl_f_exception_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\texception\0\x01\x07console\0\x15__widl_f_exception_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\texception\0\x01\x07console\0\x15__widl_f_exception_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\texception\0\x01\x07console\0\x15__widl_f_exception_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\texception\0\x01\x07console\0\x15__widl_f_exception_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\texception\0\x01\x07console\0\x0F__widl_f_group_\0\x01\0\0\x01\x01\x04data\x05group\0\x01\x07console\0\x11__widl_f_group_0_\0\0\0\0\x01\0\x05group\0\x01\x07console\0\x11__widl_f_group_1_\0\0\0\0\x01\x01\x06data_1\x05group\0\x01\x07console\0\x11__widl_f_group_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05group\0\x01\x07console\0\x11__widl_f_group_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05group\0\x01\x07console\0\x11__widl_f_group_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05group\0\x01\x07console\0\x11__widl_f_group_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05group\0\x01\x07console\0\x11__widl_f_group_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05group\0\x01\x07console\0\x11__widl_f_group_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05group\0\x01\x07console\0\x19__widl_f_group_collapsed_\0\x01\0\0\x01\x01\x04data\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_0_\0\0\0\0\x01\0\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_1_\0\0\0\0\x01\x01\x06data_1\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x0EgroupCollapsed\0\x01\x07console\0\x1B__widl_f_group_collapsed_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x0EgroupCollapsed\0\x01\x07console\0\x13__widl_f_group_end_\0\0\0\0\x01\0\x08groupEnd\0\x01\x07console\0\x0E__widl_f_info_\0\x01\0\0\x01\x01\x04data\x04info\0\x01\x07console\0\x10__widl_f_info_0_\0\0\0\0\x01\0\x04info\0\x01\x07console\0\x10__widl_f_info_1_\0\0\0\0\x01\x01\x06data_1\x04info\0\x01\x07console\0\x10__widl_f_info_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04info\0\x01\x07console\0\x10__widl_f_info_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04info\0\x01\x07console\0\x10__widl_f_info_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04info\0\x01\x07console\0\x10__widl_f_info_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04info\0\x01\x07console\0\x10__widl_f_info_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04info\0\x01\x07console\0\x10__widl_f_info_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04info\0\x01\x07console\0\r__widl_f_log_\0\x01\0\0\x01\x01\x04data\x03log\0\x01\x07console\0\x0F__widl_f_log_0_\0\0\0\0\x01\0\x03log\0\x01\x07console\0\x0F__widl_f_log_1_\0\0\0\0\x01\x01\x06data_1\x03log\0\x01\x07console\0\x0F__widl_f_log_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x03log\0\x01\x07console\0\x0F__widl_f_log_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x03log\0\x01\x07console\0\x0F__widl_f_log_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x03log\0\x01\x07console\0\x0F__widl_f_log_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x03log\0\x01\x07console\0\x0F__widl_f_log_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x03log\0\x01\x07console\0\x0F__widl_f_log_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x03log\0\x01\x07console\0\x11__widl_f_profile_\0\x01\0\0\x01\x01\x04data\x07profile\0\x01\x07console\0\x13__widl_f_profile_0_\0\0\0\0\x01\0\x07profile\0\x01\x07console\0\x13__widl_f_profile_1_\0\0\0\0\x01\x01\x06data_1\x07profile\0\x01\x07console\0\x13__widl_f_profile_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x07profile\0\x01\x07console\0\x13__widl_f_profile_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x07profile\0\x01\x07console\0\x13__widl_f_profile_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x07profile\0\x01\x07console\0\x13__widl_f_profile_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07profile\0\x01\x07console\0\x13__widl_f_profile_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07profile\0\x01\x07console\0\x13__widl_f_profile_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07profile\0\x01\x07console\0\x15__widl_f_profile_end_\0\x01\0\0\x01\x01\x04data\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_0_\0\0\0\0\x01\0\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_1_\0\0\0\0\x01\x01\x06data_1\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\nprofileEnd\0\x01\x07console\0\x17__widl_f_profile_end_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\nprofileEnd\0\x01\x07console\0\x0F__widl_f_table_\0\x01\0\0\x01\x01\x04data\x05table\0\x01\x07console\0\x11__widl_f_table_0_\0\0\0\0\x01\0\x05table\0\x01\x07console\0\x11__widl_f_table_1_\0\0\0\0\x01\x01\x06data_1\x05table\0\x01\x07console\0\x11__widl_f_table_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05table\0\x01\x07console\0\x11__widl_f_table_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05table\0\x01\x07console\0\x11__widl_f_table_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05table\0\x01\x07console\0\x11__widl_f_table_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05table\0\x01\x07console\0\x11__widl_f_table_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05table\0\x01\x07console\0\x11__widl_f_table_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05table\0\x01\x07console\0\x0E__widl_f_time_\0\0\0\0\x01\0\x04time\0\x01\x07console\0\x19__widl_f_time_with_label_\0\0\0\0\x01\x01\x05label\x04time\0\x01\x07console\0\x12__widl_f_time_end_\0\0\0\0\x01\0\x07timeEnd\0\x01\x07console\0\x1D__widl_f_time_end_with_label_\0\0\0\0\x01\x01\x05label\x07timeEnd\0\x01\x07console\0\x12__widl_f_time_log_\0\0\0\0\x01\0\x07timeLog\0\x01\x07console\0&__widl_f_time_log_with_label_and_data_\0\x01\0\0\x01\x02\x05label\x04data\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_0_\0\0\0\0\x01\x01\x05label\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_1_\0\0\0\0\x01\x02\x05label\x06data_1\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_2_\0\0\0\0\x01\x03\x05label\x06data_1\x06data_2\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_3_\0\0\0\0\x01\x04\x05label\x06data_1\x06data_2\x06data_3\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_4_\0\0\0\0\x01\x05\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_5_\0\0\0\0\x01\x06\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_6_\0\0\0\0\x01\x07\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x07timeLog\0\x01\x07console\0(__widl_f_time_log_with_label_and_data_7_\0\0\0\0\x01\x08\x05label\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x07timeLog\0\x01\x07console\0\x14__widl_f_time_stamp_\0\0\0\0\x01\0\ttimeStamp\0\x01\x07console\0\x1E__widl_f_time_stamp_with_data_\0\0\0\0\x01\x01\x04data\ttimeStamp\0\x01\x07console\0\x0F__widl_f_trace_\0\x01\0\0\x01\x01\x04data\x05trace\0\x01\x07console\0\x11__widl_f_trace_0_\0\0\0\0\x01\0\x05trace\0\x01\x07console\0\x11__widl_f_trace_1_\0\0\0\0\x01\x01\x06data_1\x05trace\0\x01\x07console\0\x11__widl_f_trace_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x05trace\0\x01\x07console\0\x11__widl_f_trace_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x05trace\0\x01\x07console\0\x11__widl_f_trace_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x05trace\0\x01\x07console\0\x11__widl_f_trace_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x05trace\0\x01\x07console\0\x11__widl_f_trace_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x05trace\0\x01\x07console\0\x11__widl_f_trace_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x05trace\0\x01\x07console\0\x0E__widl_f_warn_\0\x01\0\0\x01\x01\x04data\x04warn\0\x01\x07console\0\x10__widl_f_warn_0_\0\0\0\0\x01\0\x04warn\0\x01\x07console\0\x10__widl_f_warn_1_\0\0\0\0\x01\x01\x06data_1\x04warn\0\x01\x07console\0\x10__widl_f_warn_2_\0\0\0\0\x01\x02\x06data_1\x06data_2\x04warn\0\x01\x07console\0\x10__widl_f_warn_3_\0\0\0\0\x01\x03\x06data_1\x06data_2\x06data_3\x04warn\0\x01\x07console\0\x10__widl_f_warn_4_\0\0\0\0\x01\x04\x06data_1\x06data_2\x06data_3\x06data_4\x04warn\0\x01\x07console\0\x10__widl_f_warn_5_\0\0\0\0\x01\x05\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x04warn\0\x01\x07console\0\x10__widl_f_warn_6_\0\0\0\0\x01\x06\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x04warn\0\x01\x07console\0\x10__widl_f_warn_7_\0\0\0\0\x01\x07\x06data_1\x06data_2\x06data_3\x06data_4\x06data_5\x06data_6\x06data_7\x04warn\0\0\0\0\x18web-sys-aacd0323f1b8b2fc\0"
    };
}
