#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod unsafe_send {
    use libobs::{
        obs_data, obs_display_t, obs_encoder, obs_output, obs_scene_t, obs_source,
        obs_video_info,
    };
    use windows::Win32::Foundation::HWND;
    pub struct WrappedObsData(pub *mut obs_data);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsData",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsOutput(pub *mut obs_output);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsOutput {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsOutput",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsDisplay(pub *mut obs_display_t);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsDisplay {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsDisplay",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsScene(pub *mut obs_scene_t);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsScene {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsScene",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsEncoder(pub *mut obs_encoder);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsEncoder {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsEncoder",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsVideoInfo(pub obs_video_info);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsVideoInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsVideoInfo",
                &&self.0,
            )
        }
    }
    pub struct WrappedObsSource(pub *mut obs_source);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedObsSource {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedObsSource",
                &&self.0,
            )
        }
    }
    pub struct WrappedVoidPtr(pub *mut std::ffi::c_void);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedVoidPtr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "WrappedVoidPtr",
                &&self.0,
            )
        }
    }
    pub struct WrappedHWND(pub HWND);
    #[automatically_derived]
    impl ::core::fmt::Debug for WrappedHWND {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WrappedHWND", &&self.0)
        }
    }
    impl Clone for WrappedObsVideoInfo {
        fn clone(&self) -> Self {
            WrappedObsVideoInfo(self.0.clone())
        }
    }
    impl PartialEq for WrappedObsVideoInfo {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Eq for WrappedObsVideoInfo {}
}
pub mod crash_handler {
    use std::{ffi::c_void, sync::Mutex};
    use lazy_static::lazy_static;
    #[cfg(feature = "dialog_crash_handler")]
    pub mod dialog {
        use arboard::Clipboard;
        use dialog::{Choice, DialogBox};
        use super::ObsCrashHandler;
        pub struct DialogCrashHandler {
            _private: (),
        }
        impl DialogCrashHandler {
            pub fn new() -> Self {
                Self { _private: () }
            }
        }
        impl ObsCrashHandler for DialogCrashHandler {
            fn handle_crash(&self, message: String) {
                {
                    ::std::io::_eprint(format_args!("OBS crashed: {0}\n", message));
                };
                let res = dialog::Question::new(
                        "OBS has crashed. Do you want to copy the error to clipboard?",
                    )
                    .title("OBS Crash Handler")
                    .show();
                if let Err(e) = res {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Failed to show crash handler dialog: {0:?}\n",
                                e,
                            ),
                        );
                    };
                    return;
                }
                let res = res.unwrap();
                if res == Choice::No {
                    return;
                }
                let clipboard = Clipboard::new();
                if let Err(e) = clipboard {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to create clipboard: {0:?}\n", e),
                        );
                    };
                    return;
                }
                let mut clipboard = clipboard.unwrap();
                if let Err(e) = clipboard.set_text(message.clone()) {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Failed to copy crash message to clipboard: {0:?}\n",
                                e,
                            ),
                        );
                    };
                    return;
                }
            }
        }
    }
    pub trait ObsCrashHandler: Send {
        fn handle_crash(&self, message: String);
    }
    pub struct ConsoleCrashHandler {
        _priavte: (),
    }
    impl ConsoleCrashHandler {
        pub fn new() -> Self {
            Self { _priavte: () }
        }
    }
    impl ObsCrashHandler for ConsoleCrashHandler {
        fn handle_crash(&self, message: String) {
            {
                ::std::io::_eprint(format_args!("OBS crashed: {0}\n", message));
            };
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    /// We are using this as global variable because there can only be one obs context
    pub struct CRASH_HANDLER {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub static CRASH_HANDLER: CRASH_HANDLER = CRASH_HANDLER {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for CRASH_HANDLER {
        type Target = Mutex<Box<dyn ObsCrashHandler>>;
        fn deref(&self) -> &Mutex<Box<dyn ObsCrashHandler>> {
            #[inline(always)]
            fn __static_ref_initialize() -> Mutex<Box<dyn ObsCrashHandler>> {
                {
                    #[cfg(feature = "dialog_crash_handler")]
                    { Mutex::new(Box::new(dialog::DialogCrashHandler::new())) }
                }
            }
            #[inline(always)]
            fn __stability() -> &'static Mutex<Box<dyn ObsCrashHandler>> {
                static LAZY: ::lazy_static::lazy::Lazy<
                    Mutex<Box<dyn ObsCrashHandler>>,
                > = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for CRASH_HANDLER {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub(crate) unsafe extern "C" fn main_crash_handler(
        format: *const i8,
        args: *mut i8,
        _params: *mut c_void,
    ) {
        let res = vsprintf::vsprintf(format, args);
        if res.is_err() {
            {
                ::std::io::_eprint(
                    format_args!("Failed to format crash handler message\n"),
                );
            };
            return;
        }
        let res = res.unwrap();
        CRASH_HANDLER.lock().unwrap().handle_crash(res);
    }
}
pub mod data {
    use std::ffi::{CStr, CString};
    use anyhow::bail;
    use libobs::{
        obs_data, obs_data_create, obs_data_release, obs_data_set_bool,
        obs_data_set_double, obs_data_set_int, obs_data_set_string,
    };
    use crate::{unsafe_send::WrappedObsData, utils::ObsString};
    pub mod audio {
        use libobs::obs_audio_info2;
        use crate::enums::{ObsSamplesPerSecond, ObsSpeakerLayout};
        /// Information passed to libobs when attempting to
        /// reset the audio context using `obs_reset_audio2`.
        #[repr(C)]
        pub struct ObsAudioInfo {
            samples_per_sec: ObsSamplesPerSecond,
            speakers: ObsSpeakerLayout,
            max_buffering_ms: u32,
            fixed_buffering: bool,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsAudioInfo {
            #[inline]
            fn clone(&self) -> ObsAudioInfo {
                ObsAudioInfo {
                    samples_per_sec: ::core::clone::Clone::clone(&self.samples_per_sec),
                    speakers: ::core::clone::Clone::clone(&self.speakers),
                    max_buffering_ms: ::core::clone::Clone::clone(
                        &self.max_buffering_ms,
                    ),
                    fixed_buffering: ::core::clone::Clone::clone(&self.fixed_buffering),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsAudioInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ObsAudioInfo",
                    "samples_per_sec",
                    &self.samples_per_sec,
                    "speakers",
                    &self.speakers,
                    "max_buffering_ms",
                    &self.max_buffering_ms,
                    "fixed_buffering",
                    &&self.fixed_buffering,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ObsAudioInfo {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ObsAudioInfo {
            #[inline]
            fn eq(&self, other: &ObsAudioInfo) -> bool {
                self.samples_per_sec == other.samples_per_sec
                    && self.speakers == other.speakers
                    && self.max_buffering_ms == other.max_buffering_ms
                    && self.fixed_buffering == other.fixed_buffering
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ObsAudioInfo {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ObsSamplesPerSecond>;
                let _: ::core::cmp::AssertParamIsEq<ObsSpeakerLayout>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        impl ObsAudioInfo {
            pub fn new(
                samples_per_second: ObsSamplesPerSecond,
                speakers: ObsSpeakerLayout,
                max_buffering_ms: u32,
                fixed_buffering: bool,
            ) -> Self {
                Self {
                    samples_per_sec: samples_per_second,
                    speakers,
                    max_buffering_ms,
                    fixed_buffering,
                }
            }
            pub fn new_low_latency(
                samples_per_second: ObsSamplesPerSecond,
                speakers: ObsSpeakerLayout,
            ) -> Self {
                Self::new(samples_per_second, speakers, 20, true)
            }
            pub fn as_ptr(&self) -> *const obs_audio_info2 {
                self as *const Self as *const obs_audio_info2
            }
        }
        impl Default for ObsAudioInfo {
            fn default() -> Self {
                Self {
                    samples_per_sec: ObsSamplesPerSecond::F44100,
                    speakers: ObsSpeakerLayout::Stereo,
                    max_buffering_ms: 0,
                    fixed_buffering: false,
                }
            }
        }
    }
    mod lib_support {
        //! Use the `libobs-source` crate to create sources like `window_capture` for obs
        use crate::{data::ObsData, utils::{traits::ObsUpdatable, ObjectInfo, ObsString}};
        pub trait StringEnum {
            fn to_str(&self) -> &str;
        }
        /// Trait for building OBS sources.
        pub trait ObsObjectBuilder {
            fn new(name: impl Into<ObsString>) -> Self;
            /// Returns the name of the source.
            fn get_name(&self) -> ObsString;
            /// Adds the obs source to the output on the given channel
            fn build(mut self) -> ObjectInfo
            where
                Self: Sized,
            {
                let settings = self.get_settings_mut().take();
                let hotkeys = self.get_hotkeys_mut().take();
                ObjectInfo::new(Self::get_id(), self.get_name(), settings, hotkeys)
            }
            fn get_settings(&self) -> &Option<ObsData>;
            fn get_settings_mut(&mut self) -> &mut Option<ObsData>;
            fn get_hotkeys(&self) -> &Option<ObsData>;
            fn get_hotkeys_mut(&mut self) -> &mut Option<ObsData>;
            fn get_or_create_settings(&mut self) -> &mut ObsData {
                self.get_settings_mut().get_or_insert_with(ObsData::new)
            }
            /// Returns the ID of the source.
            fn get_id() -> ObsString;
        }
        pub trait ObsObjectUpdater<'a> {
            type ToUpdate: ObsUpdatable;
            fn create_update(updatable: &'a mut Self::ToUpdate) -> Self;
            fn get_settings(&self) -> &ObsData;
            fn get_settings_mut(&mut self) -> &mut ObsData;
            fn update(self);
            /// Returns the ID of the object
            fn get_id() -> ObsString;
        }
    }
    pub mod output {
        use std::cell::RefCell;
        use std::ffi::CString;
        use std::rc::Rc;
        use std::{ffi::CStr, ptr};
        use getters0::Getters;
        use libobs::{
            audio_output, calldata_get_data, calldata_t, obs_encoder_set_audio,
            obs_encoder_set_video, obs_output_active, obs_output_create,
            obs_output_get_last_error, obs_output_get_name,
            obs_output_get_signal_handler, obs_output_release,
            obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start,
            obs_output_stop, signal_handler_connect, signal_handler_disconnect,
            video_output,
        };
        use crate::context::ObsContextShutdownZST;
        use crate::enums::ObsOutputSignal;
        use crate::signals::{rec_output_signal, OUTPUT_SIGNALS};
        use crate::unsafe_send::WrappedObsOutput;
        use crate::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};
        use crate::{
            encoders::{audio::ObsAudioEncoder, video::ObsVideoEncoder},
            utils::{ObsError, ObsString},
        };
        use super::ObsData;
        mod replay_buffer {
            use std::{ffi::c_char, mem::MaybeUninit, path::{Path, PathBuf}};
            use libobs::{
                calldata_get_string, calldata_t, obs_output_get_proc_handler,
                proc_handler_call,
            };
            use crate::utils::{ObsError, ObsString};
            use super::ObsOutputRef;
            pub trait ReplayBufferOutput {
                fn save_buffer(&self) -> Result<Box<Path>, ObsError>;
            }
            impl ReplayBufferOutput for ObsOutputRef {
                fn save_buffer(&self) -> Result<Box<Path>, ObsError> {
                    let ph = unsafe { obs_output_get_proc_handler(self.output.0) };
                    if ph.is_null() {
                        return Err(
                            ObsError::OutputSaveBufferFailure(
                                "Failed to get proc handler.".to_string(),
                            ),
                        );
                    }
                    let name = ObsString::new("save");
                    let call_success = unsafe {
                        let mut calldata = MaybeUninit::<calldata_t>::zeroed();
                        proc_handler_call(ph, name.as_ptr(), calldata.as_mut_ptr())
                    };
                    if !call_success {
                        return Err(
                            ObsError::OutputSaveBufferFailure(
                                "Failed to call proc handler.".to_string(),
                            ),
                        );
                    }
                    let func_get = ObsString::new("get_last_replay");
                    let last_replay = unsafe {
                        let mut calldata = MaybeUninit::<calldata_t>::zeroed();
                        let success = proc_handler_call(
                            ph,
                            func_get.as_ptr(),
                            calldata.as_mut_ptr(),
                        );
                        if !success {
                            return Err(
                                ObsError::OutputSaveBufferFailure(
                                    "Failed to call get_last_replay.".to_string(),
                                ),
                            );
                        }
                        calldata.assume_init()
                    };
                    let path_get = ObsString::new("path");
                    let path = unsafe {
                        let mut s = MaybeUninit::<*const c_char>::uninit();
                        let res = calldata_get_string(
                            &last_replay,
                            path_get.as_ptr(),
                            s.as_mut_ptr(),
                        );
                        if !res {
                            return Err(
                                ObsError::OutputSaveBufferFailure(
                                    "Failed to get path from last replay.".to_string(),
                                ),
                            );
                        }
                        let s: *const c_char = s.assume_init();
                        let path = std::ffi::CStr::from_ptr(s).to_str().unwrap();
                        PathBuf::from(path)
                    };
                    Ok(path.into_boxed_path())
                }
            }
        }
        pub use replay_buffer::*;
        struct _ObsDropGuard {
            output: WrappedObsOutput,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for _ObsDropGuard {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "_ObsDropGuard",
                    "output",
                    &&self.output,
                )
            }
        }
        impl Drop for _ObsDropGuard {
            fn drop(&mut self) {
                unsafe {
                    let handler = obs_output_get_signal_handler(self.output.0);
                    let signal = ObsString::new("stop");
                    signal_handler_disconnect(
                        handler,
                        signal.as_ptr(),
                        Some(signal_handler),
                        ptr::null_mut(),
                    );
                    obs_output_release(self.output.0);
                }
            }
        }
        #[skip_new]
        pub struct ObsOutputRef {
            pub(crate) settings: Rc<RefCell<Option<ObsData>>>,
            pub(crate) hotkey_data: Rc<RefCell<Option<ObsData>>>,
            #[get_mut]
            pub(crate) video_encoders: Rc<RefCell<Vec<Rc<ObsVideoEncoder>>>>,
            #[get_mut]
            pub(crate) audio_encoders: Rc<RefCell<Vec<Rc<ObsAudioEncoder>>>>,
            #[skip_getter]
            pub(crate) output: Rc<WrappedObsOutput>,
            pub(crate) id: ObsString,
            pub(crate) name: ObsString,
            #[skip_getter]
            _drop_guard: Rc<_ObsDropGuard>,
            #[skip_getter]
            _shutdown: Rc<ObsContextShutdownZST>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsOutputRef {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "settings",
                    "hotkey_data",
                    "video_encoders",
                    "audio_encoders",
                    "output",
                    "id",
                    "name",
                    "_drop_guard",
                    "_shutdown",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.settings,
                    &self.hotkey_data,
                    &self.video_encoders,
                    &self.audio_encoders,
                    &self.output,
                    &self.id,
                    &self.name,
                    &self._drop_guard,
                    &&self._shutdown,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ObsOutputRef",
                    names,
                    values,
                )
            }
        }
        impl ObsOutputRef {
            pub fn settings(&self) -> &Rc<RefCell<Option<ObsData>>> {
                &self.settings
            }
            pub fn hotkey_data(&self) -> &Rc<RefCell<Option<ObsData>>> {
                &self.hotkey_data
            }
            pub fn video_encoders(&self) -> &Rc<RefCell<Vec<Rc<ObsVideoEncoder>>>> {
                &self.video_encoders
            }
            pub fn audio_encoders(&self) -> &Rc<RefCell<Vec<Rc<ObsAudioEncoder>>>> {
                &self.audio_encoders
            }
            pub fn id(&self) -> &ObsString {
                &self.id
            }
            pub fn name(&self) -> &ObsString {
                &self.name
            }
            pub fn video_encoders_mut(
                &mut self,
            ) -> &mut Rc<RefCell<Vec<Rc<ObsVideoEncoder>>>> {
                &mut self.video_encoders
            }
            pub fn audio_encoders_mut(
                &mut self,
            ) -> &mut Rc<RefCell<Vec<Rc<ObsAudioEncoder>>>> {
                &mut self.audio_encoders
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsOutputRef {
            #[inline]
            fn clone(&self) -> ObsOutputRef {
                ObsOutputRef {
                    settings: ::core::clone::Clone::clone(&self.settings),
                    hotkey_data: ::core::clone::Clone::clone(&self.hotkey_data),
                    video_encoders: ::core::clone::Clone::clone(&self.video_encoders),
                    audio_encoders: ::core::clone::Clone::clone(&self.audio_encoders),
                    output: ::core::clone::Clone::clone(&self.output),
                    id: ::core::clone::Clone::clone(&self.id),
                    name: ::core::clone::Clone::clone(&self.name),
                    _drop_guard: ::core::clone::Clone::clone(&self._drop_guard),
                    _shutdown: ::core::clone::Clone::clone(&self._shutdown),
                }
            }
        }
        impl ObsOutputRef {
            pub(crate) fn new(
                output: OutputInfo,
                context: Rc<ObsContextShutdownZST>,
            ) -> Result<Self, ObsError> {
                let OutputInfo { id, name, settings, hotkey_data } = output;
                let settings_ptr = match settings.as_ref() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let hotkey_data_ptr = match hotkey_data.as_ref() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let output = unsafe {
                    obs_output_create(
                        id.as_ptr(),
                        name.as_ptr(),
                        settings_ptr,
                        hotkey_data_ptr,
                    )
                };
                if output == ptr::null_mut() {
                    return Err(ObsError::NullPointer);
                }
                let handler = unsafe { obs_output_get_signal_handler(output) };
                unsafe {
                    let signal = ObsString::new("stop");
                    signal_handler_connect(
                        handler,
                        signal.as_ptr(),
                        Some(signal_handler),
                        ptr::null_mut(),
                    )
                };
                Ok(Self {
                    output: Rc::new(WrappedObsOutput(output)),
                    id,
                    name,
                    settings: Rc::new(RefCell::new(settings)),
                    hotkey_data: Rc::new(RefCell::new(hotkey_data)),
                    video_encoders: Rc::new(RefCell::new(::alloc::vec::Vec::new())),
                    audio_encoders: Rc::new(RefCell::new(::alloc::vec::Vec::new())),
                    _drop_guard: Rc::new(_ObsDropGuard {
                        output: WrappedObsOutput(output),
                    }),
                    _shutdown: context,
                })
            }
            pub fn get_video_encoders(&self) -> Vec<Rc<ObsVideoEncoder>> {
                self.video_encoders.borrow().clone()
            }
            pub fn video_encoder(
                &mut self,
                info: VideoEncoderInfo,
                handler: *mut video_output,
            ) -> Result<Rc<ObsVideoEncoder>, ObsError> {
                let video_enc = ObsVideoEncoder::new(
                    info.id,
                    info.name,
                    info.settings,
                    info.hotkey_data,
                );
                return match video_enc {
                    Ok(x) => {
                        unsafe { obs_encoder_set_video(x.encoder.0, handler) }
                        unsafe {
                            obs_output_set_video_encoder(self.output.0, x.encoder.0)
                        }
                        let tmp = Rc::new(x);
                        self.video_encoders.borrow_mut().push(tmp.clone());
                        Ok(tmp)
                    }
                    Err(x) => Err(x),
                };
            }
            pub fn audio_encoder(
                &mut self,
                info: AudioEncoderInfo,
                mixer_idx: usize,
                handler: *mut audio_output,
            ) -> Result<Rc<ObsAudioEncoder>, ObsError> {
                let audio_enc = ObsAudioEncoder::new(
                    info.id,
                    info.name,
                    info.settings,
                    mixer_idx,
                    info.hotkey_data,
                );
                return match audio_enc {
                    Ok(x) => {
                        unsafe { obs_encoder_set_audio(x.encoder.0, handler) }
                        unsafe {
                            obs_output_set_audio_encoder(
                                self.output.0,
                                x.encoder.0,
                                mixer_idx,
                            )
                        }
                        let x = Rc::new(x);
                        self.audio_encoders.borrow_mut().push(x.clone());
                        Ok(x)
                    }
                    Err(x) => Err(x),
                };
            }
            pub fn start(&self) -> Result<(), ObsError> {
                if unsafe { !obs_output_active(self.output.0) } {
                    let res = unsafe { obs_output_start(self.output.0) };
                    if res {
                        return Ok(());
                    }
                    let err = unsafe { obs_output_get_last_error(self.output.0) };
                    let c_str = unsafe { CStr::from_ptr(err) };
                    let err_str = c_str.to_str().ok().map(|x| x.to_string());
                    return Err(ObsError::OutputStartFailure(err_str));
                }
                Err(ObsError::OutputAlreadyActive)
            }
            pub fn stop(&mut self) -> Result<(), ObsError> {
                if unsafe { obs_output_active(self.output.0) } {
                    unsafe { obs_output_stop(self.output.0) }
                    let signal = rec_output_signal(&self)
                        .map_err(|e| ObsError::OutputStopFailure(Some(e.to_string())))?;
                    {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Signal: {0:?}", signal),
                                    lvl,
                                    &(
                                        "libobs_wrapper::data::output",
                                        "libobs_wrapper::data::output",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    if signal == ObsOutputSignal::Success {
                        return Ok(());
                    }
                    return Err(ObsError::OutputStopFailure(Some(signal.to_string())));
                }
                return Err(
                    ObsError::OutputStopFailure(
                        Some("Output is not active.".to_string()),
                    ),
                );
            }
        }
        extern "C" fn signal_handler(_data: *mut std::ffi::c_void, cd: *mut calldata_t) {
            unsafe {
                let mut output = ptr::null_mut();
                let output_str = CString::new("output").unwrap();
                let output_got = calldata_get_data(
                    cd,
                    output_str.as_ptr(),
                    &mut output as *mut _ as *mut std::ffi::c_void,
                    size_of::<*mut std::ffi::c_void>(),
                );
                if !output_got {
                    return;
                }
                let mut code = 0i64;
                let code_str = CString::new("code").unwrap();
                let code_got = calldata_get_data(
                    cd,
                    code_str.as_ptr(),
                    &mut code as *mut _ as *mut std::ffi::c_void,
                    size_of::<i64>(),
                );
                if !code_got {
                    return;
                }
                let name = obs_output_get_name(output as *mut _);
                let name_str = CStr::from_ptr(name).to_string_lossy().to_string();
                let signal = ObsOutputSignal::try_from(code as i32);
                if signal.is_err() {
                    return;
                }
                let signal = signal.unwrap();
                let r = OUTPUT_SIGNALS.read();
                if r.is_err() {
                    return;
                }
                let r = r.unwrap().0.send((name_str, signal));
                if let Err(e) = r {
                    {
                        ::std::io::_eprint(
                            format_args!("Couldn\'t send msg {0:?}\n", e),
                        );
                    };
                    return;
                }
            }
        }
    }
    pub mod video {
        use std::ptr;
        use display_info::DisplayInfo;
        use libobs::obs_video_info;
        use crate::{
            enums::{
                ObsColorspace, ObsGraphicsModule, ObsScaleType, ObsVideoFormat,
                ObsVideoRange, OsEnumType,
            },
            unsafe_send::WrappedObsVideoInfo, utils::ObsString,
        };
        /// A wrapper for `obs_video_info`, which is used
        /// to pass information to libobs for the new OBS
        /// video context after resetting the old OBS
        /// video context.
        pub struct ObsVideoInfo {
            ovi: WrappedObsVideoInfo,
            #[allow(dead_code)]
            graphics_module: ObsString,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsVideoInfo {
            #[inline]
            fn clone(&self) -> ObsVideoInfo {
                ObsVideoInfo {
                    ovi: ::core::clone::Clone::clone(&self.ovi),
                    graphics_module: ::core::clone::Clone::clone(&self.graphics_module),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsVideoInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ObsVideoInfo",
                    "ovi",
                    &self.ovi,
                    "graphics_module",
                    &&self.graphics_module,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ObsVideoInfo {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ObsVideoInfo {
            #[inline]
            fn eq(&self, other: &ObsVideoInfo) -> bool {
                self.ovi == other.ovi && self.graphics_module == other.graphics_module
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ObsVideoInfo {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<WrappedObsVideoInfo>;
                let _: ::core::cmp::AssertParamIsEq<ObsString>;
            }
        }
        impl ObsVideoInfo {
            /// Creates a new `ObsVideoInfo`.
            ///
            /// Note that this function is not meant to
            /// be used externally. The recommended,
            /// supported way to build new `ObsVideoInfo`
            /// structs is through `ObsVideoInfoBuilder`.
            pub fn new(ovi: obs_video_info, graphics_module: ObsString) -> Self {
                Self {
                    ovi: WrappedObsVideoInfo(ovi),
                    graphics_module,
                }
            }
            /// Returns an `ObsVideoInfo` pointer.
            pub fn as_ptr(&mut self) -> *mut obs_video_info {
                &raw mut self.ovi.0
            }
            pub fn graphics_module(&self) -> &ObsString {
                &self.graphics_module
            }
        }
        impl Default for ObsVideoInfo {
            fn default() -> Self {
                ObsVideoInfoBuilder::new().build()
            }
        }
        /// A structure intended to help make
        /// creating new `ObsVideoInfo` structs
        /// easier for resetting the OBS video
        /// context.
        pub struct ObsVideoInfoBuilder {
            adapter: u32,
            graphics_module: ObsGraphicsModule,
            fps_num: u32,
            fps_den: u32,
            base_width: u32,
            base_height: u32,
            output_width: u32,
            output_height: u32,
            output_format: ObsVideoFormat,
            gpu_conversion: bool,
            colorspace: ObsColorspace,
            range: ObsVideoRange,
            scale_type: ObsScaleType,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsVideoInfoBuilder {
            #[inline]
            fn clone(&self) -> ObsVideoInfoBuilder {
                ObsVideoInfoBuilder {
                    adapter: ::core::clone::Clone::clone(&self.adapter),
                    graphics_module: ::core::clone::Clone::clone(&self.graphics_module),
                    fps_num: ::core::clone::Clone::clone(&self.fps_num),
                    fps_den: ::core::clone::Clone::clone(&self.fps_den),
                    base_width: ::core::clone::Clone::clone(&self.base_width),
                    base_height: ::core::clone::Clone::clone(&self.base_height),
                    output_width: ::core::clone::Clone::clone(&self.output_width),
                    output_height: ::core::clone::Clone::clone(&self.output_height),
                    output_format: ::core::clone::Clone::clone(&self.output_format),
                    gpu_conversion: ::core::clone::Clone::clone(&self.gpu_conversion),
                    colorspace: ::core::clone::Clone::clone(&self.colorspace),
                    range: ::core::clone::Clone::clone(&self.range),
                    scale_type: ::core::clone::Clone::clone(&self.scale_type),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsVideoInfoBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "adapter",
                    "graphics_module",
                    "fps_num",
                    "fps_den",
                    "base_width",
                    "base_height",
                    "output_width",
                    "output_height",
                    "output_format",
                    "gpu_conversion",
                    "colorspace",
                    "range",
                    "scale_type",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.adapter,
                    &self.graphics_module,
                    &self.fps_num,
                    &self.fps_den,
                    &self.base_width,
                    &self.base_height,
                    &self.output_width,
                    &self.output_height,
                    &self.output_format,
                    &self.gpu_conversion,
                    &self.colorspace,
                    &self.range,
                    &&self.scale_type,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ObsVideoInfoBuilder",
                    names,
                    values,
                )
            }
        }
        impl ObsVideoInfoBuilder {
            /// Creates a new `ObsVideoInfoBuilder`
            /// for creating new `ObsVideoInfo` to
            /// pass to the video context reset
            /// function.
            ///
            /// This function comes with
            /// sensible default values and chooses
            /// the backend depending on which
            /// if the OS supports DX11 (Windows)
            /// or not (OpenGL on MacOS and Unix).
            pub fn new() -> Self {
                let display_infos = DisplayInfo::all().unwrap();
                let (mut width, mut height) = (1920, 1080);
                for display_info in display_infos {
                    if display_info.is_primary {
                        width = display_info.width;
                        height = display_info.height;
                        break;
                    }
                }
                Self {
                    adapter: 0,
                    #[cfg(target_family = "windows")]
                    graphics_module: ObsGraphicsModule::DirectX11,
                    fps_num: 30,
                    fps_den: 1,
                    base_width: width,
                    base_height: height,
                    output_width: width,
                    output_height: height,
                    output_format: ObsVideoFormat::NV12,
                    gpu_conversion: true,
                    colorspace: ObsColorspace::CS709,
                    range: ObsVideoRange::Default,
                    scale_type: ObsScaleType::Lanczos,
                }
            }
            /// Consumes the `ObsVideoInfoBuilder`
            /// to create an `ObsVideoInfo`.
            pub fn build(self) -> ObsVideoInfo {
                let graphics_mod_str = match self.graphics_module {
                    ObsGraphicsModule::OpenGL => ObsString::new("libobs-opengl"),
                    ObsGraphicsModule::DirectX11 => ObsString::new("libobs-d3d11"),
                };
                let ovi = obs_video_info {
                    adapter: self.adapter,
                    graphics_module: graphics_mod_str.as_ptr(),
                    fps_num: self.fps_num,
                    fps_den: self.fps_den,
                    base_width: self.base_width,
                    base_height: self.base_height,
                    output_width: self.output_width,
                    output_height: self.output_height,
                    output_format: self.output_format as OsEnumType,
                    gpu_conversion: self.gpu_conversion,
                    colorspace: self.colorspace as OsEnumType,
                    range: self.range as OsEnumType,
                    scale_type: self.scale_type as OsEnumType,
                };
                drop(self);
                ObsVideoInfo {
                    ovi: WrappedObsVideoInfo(ovi),
                    graphics_module: graphics_mod_str,
                }
            }
            /// Sets the GPU adapter device
            /// that the video output is coming
            /// from.
            pub fn adapter(mut self, value: u32) -> Self {
                self.adapter = value;
                self
            }
            /// Sets the graphics backend
            /// that libobs uses to record.
            pub fn graphics_module(mut self, value: ObsGraphicsModule) -> Self {
                self.graphics_module = value;
                self
            }
            /// Sets the framerate of the
            /// output video. Note that this
            /// value may not reflect the
            /// final framerate if `fps_den`
            /// is not equal to 1.
            pub fn fps_num(mut self, value: u32) -> Self {
                self.fps_num = value;
                self
            }
            /// Divides the FPS numerator to
            /// allow for fractional FPS
            /// counts on output.
            pub fn fps_den(mut self, value: u32) -> Self {
                self.fps_den = value;
                self
            }
            /// Sets the width of the screen
            /// being recorded.
            pub fn base_width(mut self, value: u32) -> Self {
                self.base_width = value;
                self
            }
            /// Sets the height of the screen
            /// being recorded.
            pub fn base_height(mut self, value: u32) -> Self {
                self.base_height = value;
                self
            }
            /// Sets the width of the video
            /// output.
            pub fn output_width(mut self, value: u32) -> Self {
                self.output_width = value;
                self
            }
            /// Sets the height of the video
            /// output.
            pub fn output_height(mut self, value: u32) -> Self {
                self.output_height = value;
                self
            }
            /// Sets the format in which the
            /// video will be output.
            pub fn output_format(mut self, value: ObsVideoFormat) -> Self {
                self.output_format = value;
                self
            }
            /// Sets whether the GPU will handle
            /// conversion in the video.
            pub fn gpu_conversion(mut self, value: bool) -> Self {
                self.gpu_conversion = value;
                self
            }
            /// Sets the video colorspace.
            pub fn colorspace(mut self, value: ObsColorspace) -> Self {
                self.colorspace = value;
                self
            }
            /// Sets the video range.
            pub fn range(mut self, value: ObsVideoRange) -> Self {
                self.range = value;
                self
            }
            /// Sets the video scaling type.
            pub fn scale_type(mut self, value: ObsScaleType) -> Self {
                self.scale_type = value;
                self
            }
        }
        impl Default for ObsVideoInfoBuilder {
            fn default() -> Self {
                Self::new()
            }
        }
    }
    pub mod immutable {
        use libobs::obs_data_t;
        use crate::unsafe_send::WrappedObsData;
        use super::ObsData;
        /// Immutable wrapper around obs_data_t t o be prevent modification and to be used in creation of other objects.
        /// This should not be updated directly using the pointer, but instead through the corresponding update methods on the holder of this data.
        pub struct ImmutableObsData(WrappedObsData);
        #[automatically_derived]
        impl ::core::fmt::Debug for ImmutableObsData {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ImmutableObsData",
                    &&self.0,
                )
            }
        }
        impl ImmutableObsData {
            pub fn new() -> Self {
                let ptr = unsafe { libobs::obs_data_create() };
                ImmutableObsData(WrappedObsData(ptr))
            }
            pub fn as_ptr(&self) -> *mut obs_data_t {
                self.0.0
            }
        }
        impl From<ObsData> for ImmutableObsData {
            fn from(mut data: ObsData) -> Self {
                let ptr = data.obs_data.0;
                data.obs_data.0 = std::ptr::null_mut();
                ImmutableObsData(WrappedObsData(ptr))
            }
        }
        impl From<*mut obs_data_t> for ImmutableObsData {
            fn from(data: *mut obs_data_t) -> Self {
                ImmutableObsData(WrappedObsData(data))
            }
        }
        impl Drop for ImmutableObsData {
            fn drop(&mut self) {
                unsafe { libobs::obs_data_release(self.0.0) }
            }
        }
    }
    pub mod properties {
        mod enums {
            use num_derive::{FromPrimitive, ToPrimitive};
            #[repr(i32)]
            pub enum ObsPropertyType {
                Invalid = libobs::obs_property_type_OBS_PROPERTY_INVALID,
                Bool = libobs::obs_property_type_OBS_PROPERTY_BOOL,
                Int = libobs::obs_property_type_OBS_PROPERTY_INT,
                Float = libobs::obs_property_type_OBS_PROPERTY_FLOAT,
                Text = libobs::obs_property_type_OBS_PROPERTY_TEXT,
                Path = libobs::obs_property_type_OBS_PROPERTY_PATH,
                List = libobs::obs_property_type_OBS_PROPERTY_LIST,
                Color = libobs::obs_property_type_OBS_PROPERTY_COLOR,
                Button = libobs::obs_property_type_OBS_PROPERTY_BUTTON,
                Font = libobs::obs_property_type_OBS_PROPERTY_FONT,
                EditableList = libobs::obs_property_type_OBS_PROPERTY_EDITABLE_LIST,
                FrameRate = libobs::obs_property_type_OBS_PROPERTY_FRAME_RATE,
                Group = libobs::obs_property_type_OBS_PROPERTY_GROUP,
                ColorAlpha = libobs::obs_property_type_OBS_PROPERTY_COLOR_ALPHA,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsPropertyType {
                #[inline]
                fn clone(&self) -> ObsPropertyType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsPropertyType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsPropertyType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsPropertyType::Invalid => "Invalid",
                            ObsPropertyType::Bool => "Bool",
                            ObsPropertyType::Int => "Int",
                            ObsPropertyType::Float => "Float",
                            ObsPropertyType::Text => "Text",
                            ObsPropertyType::Path => "Path",
                            ObsPropertyType::List => "List",
                            ObsPropertyType::Color => "Color",
                            ObsPropertyType::Button => "Button",
                            ObsPropertyType::Font => "Font",
                            ObsPropertyType::EditableList => "EditableList",
                            ObsPropertyType::FrameRate => "FrameRate",
                            ObsPropertyType::Group => "Group",
                            ObsPropertyType::ColorAlpha => "ColorAlpha",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsPropertyType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsPropertyType {
                #[inline]
                fn eq(&self, other: &ObsPropertyType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsPropertyType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsPropertyType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsPropertyType::Invalid as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Invalid)
                        } else if n == ObsPropertyType::Bool as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Bool)
                        } else if n == ObsPropertyType::Int as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Int)
                        } else if n == ObsPropertyType::Float as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Float)
                        } else if n == ObsPropertyType::Text as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Text)
                        } else if n == ObsPropertyType::Path as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Path)
                        } else if n == ObsPropertyType::List as i64 {
                            ::core::option::Option::Some(ObsPropertyType::List)
                        } else if n == ObsPropertyType::Color as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Color)
                        } else if n == ObsPropertyType::Button as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Button)
                        } else if n == ObsPropertyType::Font as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Font)
                        } else if n == ObsPropertyType::EditableList as i64 {
                            ::core::option::Option::Some(ObsPropertyType::EditableList)
                        } else if n == ObsPropertyType::FrameRate as i64 {
                            ::core::option::Option::Some(ObsPropertyType::FrameRate)
                        } else if n == ObsPropertyType::Group as i64 {
                            ::core::option::Option::Some(ObsPropertyType::Group)
                        } else if n == ObsPropertyType::ColorAlpha as i64 {
                            ::core::option::Option::Some(ObsPropertyType::ColorAlpha)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsPropertyType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsPropertyType::Invalid => ObsPropertyType::Invalid as i64,
                                ObsPropertyType::Bool => ObsPropertyType::Bool as i64,
                                ObsPropertyType::Int => ObsPropertyType::Int as i64,
                                ObsPropertyType::Float => ObsPropertyType::Float as i64,
                                ObsPropertyType::Text => ObsPropertyType::Text as i64,
                                ObsPropertyType::Path => ObsPropertyType::Path as i64,
                                ObsPropertyType::List => ObsPropertyType::List as i64,
                                ObsPropertyType::Color => ObsPropertyType::Color as i64,
                                ObsPropertyType::Button => ObsPropertyType::Button as i64,
                                ObsPropertyType::Font => ObsPropertyType::Font as i64,
                                ObsPropertyType::EditableList => {
                                    ObsPropertyType::EditableList as i64
                                }
                                ObsPropertyType::FrameRate => {
                                    ObsPropertyType::FrameRate as i64
                                }
                                ObsPropertyType::Group => ObsPropertyType::Group as i64,
                                ObsPropertyType::ColorAlpha => {
                                    ObsPropertyType::ColorAlpha as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsComboFormat {
                Invalid = libobs::obs_combo_format_OBS_COMBO_FORMAT_INVALID,
                Int = libobs::obs_combo_format_OBS_COMBO_FORMAT_INT,
                Float = libobs::obs_combo_format_OBS_COMBO_FORMAT_FLOAT,
                String = libobs::obs_combo_format_OBS_COMBO_FORMAT_STRING,
                Bool = libobs::obs_combo_format_OBS_COMBO_FORMAT_BOOL,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsComboFormat {
                #[inline]
                fn clone(&self) -> ObsComboFormat {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsComboFormat {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsComboFormat {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsComboFormat::Invalid => "Invalid",
                            ObsComboFormat::Int => "Int",
                            ObsComboFormat::Float => "Float",
                            ObsComboFormat::String => "String",
                            ObsComboFormat::Bool => "Bool",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsComboFormat {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsComboFormat {
                #[inline]
                fn eq(&self, other: &ObsComboFormat) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsComboFormat {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsComboFormat {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsComboFormat::Invalid as i64 {
                            ::core::option::Option::Some(ObsComboFormat::Invalid)
                        } else if n == ObsComboFormat::Int as i64 {
                            ::core::option::Option::Some(ObsComboFormat::Int)
                        } else if n == ObsComboFormat::Float as i64 {
                            ::core::option::Option::Some(ObsComboFormat::Float)
                        } else if n == ObsComboFormat::String as i64 {
                            ::core::option::Option::Some(ObsComboFormat::String)
                        } else if n == ObsComboFormat::Bool as i64 {
                            ::core::option::Option::Some(ObsComboFormat::Bool)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsComboFormat {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsComboFormat::Invalid => ObsComboFormat::Invalid as i64,
                                ObsComboFormat::Int => ObsComboFormat::Int as i64,
                                ObsComboFormat::Float => ObsComboFormat::Float as i64,
                                ObsComboFormat::String => ObsComboFormat::String as i64,
                                ObsComboFormat::Bool => ObsComboFormat::Bool as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsComboType {
                Invalid = libobs::obs_combo_type_OBS_COMBO_TYPE_INVALID,
                Editable = libobs::obs_combo_type_OBS_COMBO_TYPE_EDITABLE,
                List = libobs::obs_combo_type_OBS_COMBO_TYPE_LIST,
                Radio = libobs::obs_combo_type_OBS_COMBO_TYPE_RADIO,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsComboType {
                #[inline]
                fn clone(&self) -> ObsComboType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsComboType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsComboType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsComboType::Invalid => "Invalid",
                            ObsComboType::Editable => "Editable",
                            ObsComboType::List => "List",
                            ObsComboType::Radio => "Radio",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsComboType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsComboType {
                #[inline]
                fn eq(&self, other: &ObsComboType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsComboType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsComboType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsComboType::Invalid as i64 {
                            ::core::option::Option::Some(ObsComboType::Invalid)
                        } else if n == ObsComboType::Editable as i64 {
                            ::core::option::Option::Some(ObsComboType::Editable)
                        } else if n == ObsComboType::List as i64 {
                            ::core::option::Option::Some(ObsComboType::List)
                        } else if n == ObsComboType::Radio as i64 {
                            ::core::option::Option::Some(ObsComboType::Radio)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsComboType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsComboType::Invalid => ObsComboType::Invalid as i64,
                                ObsComboType::Editable => ObsComboType::Editable as i64,
                                ObsComboType::List => ObsComboType::List as i64,
                                ObsComboType::Radio => ObsComboType::Radio as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsTextType {
                Default = libobs::obs_text_type_OBS_TEXT_DEFAULT,
                Password = libobs::obs_text_type_OBS_TEXT_PASSWORD,
                Multiline = libobs::obs_text_type_OBS_TEXT_MULTILINE,
                Info = libobs::obs_text_type_OBS_TEXT_INFO,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsTextType {
                #[inline]
                fn clone(&self) -> ObsTextType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsTextType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsTextType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsTextType::Default => "Default",
                            ObsTextType::Password => "Password",
                            ObsTextType::Multiline => "Multiline",
                            ObsTextType::Info => "Info",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsTextType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsTextType {
                #[inline]
                fn eq(&self, other: &ObsTextType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsTextType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsTextType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsTextType::Default as i64 {
                            ::core::option::Option::Some(ObsTextType::Default)
                        } else if n == ObsTextType::Password as i64 {
                            ::core::option::Option::Some(ObsTextType::Password)
                        } else if n == ObsTextType::Multiline as i64 {
                            ::core::option::Option::Some(ObsTextType::Multiline)
                        } else if n == ObsTextType::Info as i64 {
                            ::core::option::Option::Some(ObsTextType::Info)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsTextType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsTextType::Default => ObsTextType::Default as i64,
                                ObsTextType::Password => ObsTextType::Password as i64,
                                ObsTextType::Multiline => ObsTextType::Multiline as i64,
                                ObsTextType::Info => ObsTextType::Info as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsTextInfoType {
                Normal = libobs::obs_text_info_type_OBS_TEXT_INFO_NORMAL,
                Warning = libobs::obs_text_info_type_OBS_TEXT_INFO_WARNING,
                Error = libobs::obs_text_info_type_OBS_TEXT_INFO_ERROR,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsTextInfoType {
                #[inline]
                fn clone(&self) -> ObsTextInfoType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsTextInfoType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsTextInfoType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsTextInfoType::Normal => "Normal",
                            ObsTextInfoType::Warning => "Warning",
                            ObsTextInfoType::Error => "Error",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsTextInfoType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsTextInfoType {
                #[inline]
                fn eq(&self, other: &ObsTextInfoType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsTextInfoType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsTextInfoType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsTextInfoType::Normal as i64 {
                            ::core::option::Option::Some(ObsTextInfoType::Normal)
                        } else if n == ObsTextInfoType::Warning as i64 {
                            ::core::option::Option::Some(ObsTextInfoType::Warning)
                        } else if n == ObsTextInfoType::Error as i64 {
                            ::core::option::Option::Some(ObsTextInfoType::Error)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsTextInfoType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsTextInfoType::Normal => ObsTextInfoType::Normal as i64,
                                ObsTextInfoType::Warning => ObsTextInfoType::Warning as i64,
                                ObsTextInfoType::Error => ObsTextInfoType::Error as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsNumberType {
                Scroller = libobs::obs_number_type_OBS_NUMBER_SCROLLER,
                Slider = libobs::obs_number_type_OBS_NUMBER_SLIDER,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsNumberType {
                #[inline]
                fn clone(&self) -> ObsNumberType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsNumberType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsNumberType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsNumberType::Scroller => "Scroller",
                            ObsNumberType::Slider => "Slider",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsNumberType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsNumberType {
                #[inline]
                fn eq(&self, other: &ObsNumberType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsNumberType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsNumberType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsNumberType::Scroller as i64 {
                            ::core::option::Option::Some(ObsNumberType::Scroller)
                        } else if n == ObsNumberType::Slider as i64 {
                            ::core::option::Option::Some(ObsNumberType::Slider)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsNumberType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsNumberType::Scroller => ObsNumberType::Scroller as i64,
                                ObsNumberType::Slider => ObsNumberType::Slider as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsPathType {
                File = libobs::obs_path_type_OBS_PATH_FILE,
                FileSave = libobs::obs_path_type_OBS_PATH_FILE_SAVE,
                Directory = libobs::obs_path_type_OBS_PATH_DIRECTORY,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsPathType {
                #[inline]
                fn clone(&self) -> ObsPathType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsPathType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsPathType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsPathType::File => "File",
                            ObsPathType::FileSave => "FileSave",
                            ObsPathType::Directory => "Directory",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsPathType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsPathType {
                #[inline]
                fn eq(&self, other: &ObsPathType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsPathType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsPathType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsPathType::File as i64 {
                            ::core::option::Option::Some(ObsPathType::File)
                        } else if n == ObsPathType::FileSave as i64 {
                            ::core::option::Option::Some(ObsPathType::FileSave)
                        } else if n == ObsPathType::Directory as i64 {
                            ::core::option::Option::Some(ObsPathType::Directory)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsPathType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsPathType::File => ObsPathType::File as i64,
                                ObsPathType::FileSave => ObsPathType::FileSave as i64,
                                ObsPathType::Directory => ObsPathType::Directory as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsEditableListType {
                Strings = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_STRINGS,
                Files = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_FILES,
                FilesAndUrls = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_FILES_AND_URLS,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsEditableListType {
                #[inline]
                fn clone(&self) -> ObsEditableListType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsEditableListType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsEditableListType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsEditableListType::Strings => "Strings",
                            ObsEditableListType::Files => "Files",
                            ObsEditableListType::FilesAndUrls => "FilesAndUrls",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsEditableListType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsEditableListType {
                #[inline]
                fn eq(&self, other: &ObsEditableListType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsEditableListType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsEditableListType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsEditableListType::Strings as i64 {
                            ::core::option::Option::Some(ObsEditableListType::Strings)
                        } else if n == ObsEditableListType::Files as i64 {
                            ::core::option::Option::Some(ObsEditableListType::Files)
                        } else if n == ObsEditableListType::FilesAndUrls as i64 {
                            ::core::option::Option::Some(
                                ObsEditableListType::FilesAndUrls,
                            )
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsEditableListType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsEditableListType::Strings => {
                                    ObsEditableListType::Strings as i64
                                }
                                ObsEditableListType::Files => {
                                    ObsEditableListType::Files as i64
                                }
                                ObsEditableListType::FilesAndUrls => {
                                    ObsEditableListType::FilesAndUrls as i64
                                }
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsGroupType {
                Invalid = libobs::obs_group_type_OBS_COMBO_INVALID,
                Normal = libobs::obs_group_type_OBS_GROUP_NORMAL,
                Checkable = libobs::obs_group_type_OBS_GROUP_CHECKABLE,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsGroupType {
                #[inline]
                fn clone(&self) -> ObsGroupType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsGroupType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsGroupType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsGroupType::Invalid => "Invalid",
                            ObsGroupType::Normal => "Normal",
                            ObsGroupType::Checkable => "Checkable",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsGroupType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsGroupType {
                #[inline]
                fn eq(&self, other: &ObsGroupType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsGroupType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsGroupType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsGroupType::Invalid as i64 {
                            ::core::option::Option::Some(ObsGroupType::Invalid)
                        } else if n == ObsGroupType::Normal as i64 {
                            ::core::option::Option::Some(ObsGroupType::Normal)
                        } else if n == ObsGroupType::Checkable as i64 {
                            ::core::option::Option::Some(ObsGroupType::Checkable)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsGroupType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsGroupType::Invalid => ObsGroupType::Invalid as i64,
                                ObsGroupType::Normal => ObsGroupType::Normal as i64,
                                ObsGroupType::Checkable => ObsGroupType::Checkable as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
            #[repr(i32)]
            pub enum ObsButtonType {
                Default = libobs::obs_button_type_OBS_BUTTON_DEFAULT,
                Url = libobs::obs_button_type_OBS_BUTTON_URL,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsButtonType {
                #[inline]
                fn clone(&self) -> ObsButtonType {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ObsButtonType {}
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsButtonType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ObsButtonType::Default => "Default",
                            ObsButtonType::Url => "Url",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ObsButtonType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ObsButtonType {
                #[inline]
                fn eq(&self, other: &ObsButtonType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ObsButtonType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::FromPrimitive for ObsButtonType {
                    #[allow(trivial_numeric_casts)]
                    #[inline]
                    fn from_i64(n: i64) -> ::core::option::Option<Self> {
                        if n == ObsButtonType::Default as i64 {
                            ::core::option::Option::Some(ObsButtonType::Default)
                        } else if n == ObsButtonType::Url as i64 {
                            ::core::option::Option::Some(ObsButtonType::Url)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                    #[inline]
                    fn from_u64(n: u64) -> ::core::option::Option<Self> {
                        Self::from_i64(n as i64)
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_qualifications)]
            const _: () = {
                #[allow(clippy::useless_attribute)]
                #[allow(rust_2018_idioms)]
                extern crate num_traits as _num_traits;
                impl _num_traits::ToPrimitive for ObsButtonType {
                    #[inline]
                    #[allow(trivial_numeric_casts)]
                    fn to_i64(&self) -> ::core::option::Option<i64> {
                        ::core::option::Option::Some(
                            match *self {
                                ObsButtonType::Default => ObsButtonType::Default as i64,
                                ObsButtonType::Url => ObsButtonType::Url as i64,
                            },
                        )
                    }
                    #[inline]
                    fn to_u64(&self) -> ::core::option::Option<u64> {
                        self.to_i64().map(|x| x as u64)
                    }
                }
            };
        }
        mod macros {
            pub(super) use impl_general_property;
            pub(super) use assert_type;
            pub(super) use get_enum;
        }
        pub mod types {
            mod button {
                use std::ffi::CStr;
                use getters0::Getters;
                use num_traits::FromPrimitive;
                use crate::data::properties::{macros::assert_type, ObsButtonType};
                use super::PropertyCreationInfo;
                #[skip_new]
                pub struct ObsButtonProperty {
                    name: String,
                    description: String,
                    button_type: ObsButtonType,
                    url: Option<String>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsButtonProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "ObsButtonProperty",
                            "name",
                            &self.name,
                            "description",
                            &self.description,
                            "button_type",
                            &self.button_type,
                            "url",
                            &&self.url,
                        )
                    }
                }
                impl ObsButtonProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn button_type(&self) -> &ObsButtonType {
                        &self.button_type
                    }
                    pub fn url(&self) -> &Option<String> {
                        &self.url
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsButtonProperty {
                    #[inline]
                    fn clone(&self) -> ObsButtonProperty {
                        ObsButtonProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            button_type: ::core::clone::Clone::clone(&self.button_type),
                            url: ::core::clone::Clone::clone(&self.url),
                        }
                    }
                }
                impl TryFrom<PropertyCreationInfo> for ObsButtonProperty {
                    type Error = anyhow::Error;
                    fn try_from(
                        PropertyCreationInfo {
                            name,
                            description,
                            pointer,
                        }: PropertyCreationInfo,
                    ) -> Result<Self, Self::Error> {
                        {
                            use crate::data::properties::ObsPropertyType;
                            use num_traits::FromPrimitive;
                            let p_type = unsafe {
                                libobs::obs_property_get_type(pointer)
                            };
                            let p_type = ObsPropertyType::from_i32(p_type);
                            if p_type
                                .is_none_or(|e| {
                                    !match e {
                                        ObsPropertyType::Button => true,
                                        _ => false,
                                    }
                                })
                            {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Invalid property type: expected {0:?}, got {1:?}",
                                            ObsPropertyType::Button,
                                            p_type,
                                        ),
                                    );
                                };
                            }
                        };
                        let url = unsafe { libobs::obs_property_button_url(pointer) };
                        let url = if url.is_null() {
                            None
                        } else {
                            let url = unsafe {
                                libobs::obs_property_button_url(pointer)
                            };
                            let url = unsafe { CStr::from_ptr(url as _) };
                            let url = url.to_str()?.to_string();
                            if url.is_empty() { None } else { Some(url) }
                        };
                        let button_type = unsafe {
                            libobs::obs_property_button_type(pointer)
                        };
                        let button_type = ObsButtonType::from_i32(button_type);
                        if button_type
                            .is_none_or(|e| {
                                !match e {
                                    ObsButtonType::Url => true,
                                    _ => false,
                                }
                            })
                        {
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Invalid button type"),
                                    );
                                    error
                                }),
                            );
                        }
                        Ok(Self {
                            name,
                            description,
                            button_type: button_type.unwrap(),
                            url,
                        })
                    }
                }
            }
            #[skip_new]
            pub struct ObsColorProperty {
                name: String,
                description: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsColorProperty {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ObsColorProperty",
                        "name",
                        &self.name,
                        "description",
                        &&self.description,
                    )
                }
            }
            impl ObsColorProperty {
                pub fn name(&self) -> &String {
                    &self.name
                }
                pub fn description(&self) -> &String {
                    &self.description
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsColorProperty {
                #[inline]
                fn clone(&self) -> ObsColorProperty {
                    ObsColorProperty {
                        name: ::core::clone::Clone::clone(&self.name),
                        description: ::core::clone::Clone::clone(&self.description),
                    }
                }
            }
            impl From<crate::data::properties::PropertyCreationInfo>
            for ObsColorProperty {
                fn from(
                    crate::data::properties::PropertyCreationInfo {
                        name,
                        description,
                        pointer,
                    }: crate::data::properties::PropertyCreationInfo,
                ) -> Self {
                    {
                        use crate::data::properties::ObsPropertyType;
                        use num_traits::FromPrimitive;
                        let p_type = unsafe { libobs::obs_property_get_type(pointer) };
                        let p_type = ObsPropertyType::from_i32(p_type);
                        if p_type
                            .is_none_or(|e| {
                                !match e {
                                    ObsPropertyType::Color => true,
                                    _ => false,
                                }
                            })
                        {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Invalid property type: expected {0:?}, got {1:?}",
                                        ObsPropertyType::Color,
                                        p_type,
                                    ),
                                );
                            };
                        }
                    };
                    Self { name, description }
                }
            }
            mod editable_list {
                use getters0::Getters;
                use crate::data::properties::ObsEditableListType;
                #[skip_new]
                pub struct ObsEditableListProperty {
                    name: String,
                    description: String,
                    list_type: ObsEditableListType,
                    filter: String,
                    default_path: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsEditableListProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "ObsEditableListProperty",
                            "name",
                            &self.name,
                            "description",
                            &self.description,
                            "list_type",
                            &self.list_type,
                            "filter",
                            &self.filter,
                            "default_path",
                            &&self.default_path,
                        )
                    }
                }
                impl ObsEditableListProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn list_type(&self) -> &ObsEditableListType {
                        &self.list_type
                    }
                    pub fn filter(&self) -> &String {
                        &self.filter
                    }
                    pub fn default_path(&self) -> &String {
                        &self.default_path
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsEditableListProperty {
                    #[inline]
                    fn clone(&self) -> ObsEditableListProperty {
                        ObsEditableListProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            list_type: ::core::clone::Clone::clone(&self.list_type),
                            filter: ::core::clone::Clone::clone(&self.filter),
                            default_path: ::core::clone::Clone::clone(&self.default_path),
                        }
                    }
                }
            }
            mod font {
                use getters0::Getters;
                #[skip_new]
                pub struct ObsFontProperty {
                    name: String,
                    description: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsFontProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "ObsFontProperty",
                            "name",
                            &self.name,
                            "description",
                            &&self.description,
                        )
                    }
                }
                impl ObsFontProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsFontProperty {
                    #[inline]
                    fn clone(&self) -> ObsFontProperty {
                        ObsFontProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                        }
                    }
                }
            }
            #[skip_new]
            pub struct ObsFrameRateProperty {
                name: String,
                description: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ObsFrameRateProperty {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ObsFrameRateProperty",
                        "name",
                        &self.name,
                        "description",
                        &&self.description,
                    )
                }
            }
            impl ObsFrameRateProperty {
                pub fn name(&self) -> &String {
                    &self.name
                }
                pub fn description(&self) -> &String {
                    &self.description
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ObsFrameRateProperty {
                #[inline]
                fn clone(&self) -> ObsFrameRateProperty {
                    ObsFrameRateProperty {
                        name: ::core::clone::Clone::clone(&self.name),
                        description: ::core::clone::Clone::clone(&self.description),
                    }
                }
            }
            impl From<crate::data::properties::PropertyCreationInfo>
            for ObsFrameRateProperty {
                fn from(
                    crate::data::properties::PropertyCreationInfo {
                        name,
                        description,
                        pointer,
                    }: crate::data::properties::PropertyCreationInfo,
                ) -> Self {
                    {
                        use crate::data::properties::ObsPropertyType;
                        use num_traits::FromPrimitive;
                        let p_type = unsafe { libobs::obs_property_get_type(pointer) };
                        let p_type = ObsPropertyType::from_i32(p_type);
                        if p_type
                            .is_none_or(|e| {
                                !match e {
                                    ObsPropertyType::FrameRate => true,
                                    _ => false,
                                }
                            })
                        {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Invalid property type: expected {0:?}, got {1:?}",
                                        ObsPropertyType::FrameRate,
                                        p_type,
                                    ),
                                );
                            };
                        }
                    };
                    Self { name, description }
                }
            }
            mod list {
                use getters0::Getters;
                use crate::data::properties::{ObsComboFormat, ObsComboType};
                #[skip_new]
                pub struct ObsListProperty {
                    name: String,
                    description: String,
                    list_type: ObsComboType,
                    format: ObsComboFormat,
                    items: Vec<ObsListItem>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsListProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "ObsListProperty",
                            "name",
                            &self.name,
                            "description",
                            &self.description,
                            "list_type",
                            &self.list_type,
                            "format",
                            &self.format,
                            "items",
                            &&self.items,
                        )
                    }
                }
                impl ObsListProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn list_type(&self) -> &ObsComboType {
                        &self.list_type
                    }
                    pub fn format(&self) -> &ObsComboFormat {
                        &self.format
                    }
                    pub fn items(&self) -> &Vec<ObsListItem> {
                        &self.items
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsListProperty {
                    #[inline]
                    fn clone(&self) -> ObsListProperty {
                        ObsListProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            list_type: ::core::clone::Clone::clone(&self.list_type),
                            format: ::core::clone::Clone::clone(&self.format),
                            items: ::core::clone::Clone::clone(&self.items),
                        }
                    }
                }
                #[skip_new]
                pub struct ObsListItem {
                    name: String,
                    value: ObsListItemValue,
                    disabled: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsListItem {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "ObsListItem",
                            "name",
                            &self.name,
                            "value",
                            &self.value,
                            "disabled",
                            &&self.disabled,
                        )
                    }
                }
                impl ObsListItem {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn value(&self) -> &ObsListItemValue {
                        &self.value
                    }
                    pub fn disabled(&self) -> &bool {
                        &self.disabled
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsListItem {
                    #[inline]
                    fn clone(&self) -> ObsListItem {
                        ObsListItem {
                            name: ::core::clone::Clone::clone(&self.name),
                            value: ::core::clone::Clone::clone(&self.value),
                            disabled: ::core::clone::Clone::clone(&self.disabled),
                        }
                    }
                }
                #[skip_new]
                pub enum ObsListItemValue {
                    String(String),
                    Int(i64),
                    Float(f64),
                    Bool(bool),
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsListItemValue {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match self {
                            ObsListItemValue::String(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "String",
                                    &__self_0,
                                )
                            }
                            ObsListItemValue::Int(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "Int",
                                    &__self_0,
                                )
                            }
                            ObsListItemValue::Float(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "Float",
                                    &__self_0,
                                )
                            }
                            ObsListItemValue::Bool(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "Bool",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }
                impl ObsListItemValue {}
                #[automatically_derived]
                impl ::core::clone::Clone for ObsListItemValue {
                    #[inline]
                    fn clone(&self) -> ObsListItemValue {
                        match self {
                            ObsListItemValue::String(__self_0) => {
                                ObsListItemValue::String(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            ObsListItemValue::Int(__self_0) => {
                                ObsListItemValue::Int(::core::clone::Clone::clone(__self_0))
                            }
                            ObsListItemValue::Float(__self_0) => {
                                ObsListItemValue::Float(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            ObsListItemValue::Bool(__self_0) => {
                                ObsListItemValue::Bool(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                        }
                    }
                }
            }
            mod number {
                use getters0::Getters;
                use crate::data::properties::ObsNumberType;
                #[skip_new]
                pub struct ObsNumberProperty<T>
                where
                    T: Clone + Copy + std::fmt::Debug,
                {
                    name: String,
                    description: String,
                    min: T,
                    max: T,
                    step: T,
                    suffix: String,
                    number_type: ObsNumberType,
                }
                #[automatically_derived]
                impl<T: ::core::fmt::Debug> ::core::fmt::Debug for ObsNumberProperty<T>
                where
                    T: Clone + Copy + std::fmt::Debug,
                {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "name",
                            "description",
                            "min",
                            "max",
                            "step",
                            "suffix",
                            "number_type",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.name,
                            &self.description,
                            &self.min,
                            &self.max,
                            &self.step,
                            &self.suffix,
                            &&self.number_type,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "ObsNumberProperty",
                            names,
                            values,
                        )
                    }
                }
                impl<T> ObsNumberProperty<T>
                where
                    T: Clone + Copy + std::fmt::Debug,
                {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn min(&self) -> &T {
                        &self.min
                    }
                    pub fn max(&self) -> &T {
                        &self.max
                    }
                    pub fn step(&self) -> &T {
                        &self.step
                    }
                    pub fn suffix(&self) -> &String {
                        &self.suffix
                    }
                    pub fn number_type(&self) -> &ObsNumberType {
                        &self.number_type
                    }
                }
                #[automatically_derived]
                impl<T: ::core::clone::Clone> ::core::clone::Clone
                for ObsNumberProperty<T>
                where
                    T: Clone + Copy + std::fmt::Debug,
                {
                    #[inline]
                    fn clone(&self) -> ObsNumberProperty<T> {
                        ObsNumberProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            min: ::core::clone::Clone::clone(&self.min),
                            max: ::core::clone::Clone::clone(&self.max),
                            step: ::core::clone::Clone::clone(&self.step),
                            suffix: ::core::clone::Clone::clone(&self.suffix),
                            number_type: ::core::clone::Clone::clone(&self.number_type),
                        }
                    }
                }
                impl From<super::PropertyCreationInfo> for ObsNumberProperty<i32> {
                    fn from(
                        super::PropertyCreationInfo {
                            name,
                            description,
                            pointer,
                        }: super::PropertyCreationInfo,
                    ) -> Self {
                        use crate::data::properties::ObsNumberType;
                        use num_traits::FromPrimitive;
                        let min = unsafe { libobs::obs_property_int_min(pointer) };
                        let max = unsafe { libobs::obs_property_int_max(pointer) };
                        let step = unsafe { libobs::obs_property_int_step(pointer) };
                        let suffix = unsafe { libobs::obs_property_int_suffix(pointer) };
                        let suffix = if suffix.is_null() {
                            String::new()
                        } else {
                            let suffix = unsafe { std::ffi::CStr::from_ptr(suffix) };
                            let suffix = suffix.to_str().unwrap_or_default();
                            suffix.to_string()
                        };
                        let number_type = unsafe {
                            libobs::obs_property_int_type(pointer)
                        };
                        let number_type = ObsNumberType::from_i32(number_type);
                        if number_type.is_none() {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("Invalid number type got none"),
                                );
                            };
                        }
                        return ObsNumberProperty {
                            name,
                            description,
                            min,
                            max,
                            step,
                            suffix,
                            number_type: number_type.unwrap(),
                        };
                    }
                }
                impl From<super::PropertyCreationInfo> for ObsNumberProperty<f64> {
                    fn from(
                        super::PropertyCreationInfo {
                            name,
                            description,
                            pointer,
                        }: super::PropertyCreationInfo,
                    ) -> Self {
                        use crate::data::properties::ObsNumberType;
                        use num_traits::FromPrimitive;
                        let min = unsafe { libobs::obs_property_float_min(pointer) };
                        let max = unsafe { libobs::obs_property_float_max(pointer) };
                        let step = unsafe { libobs::obs_property_float_step(pointer) };
                        let suffix = unsafe {
                            libobs::obs_property_float_suffix(pointer)
                        };
                        let suffix = if suffix.is_null() {
                            String::new()
                        } else {
                            let suffix = unsafe { std::ffi::CStr::from_ptr(suffix) };
                            let suffix = suffix.to_str().unwrap_or_default();
                            suffix.to_string()
                        };
                        let number_type = unsafe {
                            libobs::obs_property_float_type(pointer)
                        };
                        let number_type = ObsNumberType::from_i32(number_type);
                        if number_type.is_none() {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("Invalid number type got none"),
                                );
                            };
                        }
                        return ObsNumberProperty {
                            name,
                            description,
                            min,
                            max,
                            step,
                            suffix,
                            number_type: number_type.unwrap(),
                        };
                    }
                }
            }
            mod path {
                use getters0::Getters;
                use crate::data::properties::{macros::assert_type, ObsPathType};
                use super::PropertyCreationInfo;
                #[skip_new]
                pub struct ObsPathProperty {
                    name: String,
                    description: String,
                    path_type: ObsPathType,
                    filter: String,
                    default_path: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsPathProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "ObsPathProperty",
                            "name",
                            &self.name,
                            "description",
                            &self.description,
                            "path_type",
                            &self.path_type,
                            "filter",
                            &self.filter,
                            "default_path",
                            &&self.default_path,
                        )
                    }
                }
                impl ObsPathProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn path_type(&self) -> &ObsPathType {
                        &self.path_type
                    }
                    pub fn filter(&self) -> &String {
                        &self.filter
                    }
                    pub fn default_path(&self) -> &String {
                        &self.default_path
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsPathProperty {
                    #[inline]
                    fn clone(&self) -> ObsPathProperty {
                        ObsPathProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            path_type: ::core::clone::Clone::clone(&self.path_type),
                            filter: ::core::clone::Clone::clone(&self.filter),
                            default_path: ::core::clone::Clone::clone(&self.default_path),
                        }
                    }
                }
                impl From<PropertyCreationInfo> for ObsPathProperty {
                    fn from(
                        PropertyCreationInfo {
                            name,
                            description,
                            pointer,
                        }: PropertyCreationInfo,
                    ) -> Self {
                        {
                            use crate::data::properties::ObsPropertyType;
                            use num_traits::FromPrimitive;
                            let p_type = unsafe {
                                libobs::obs_property_get_type(pointer)
                            };
                            let p_type = ObsPropertyType::from_i32(p_type);
                            if p_type
                                .is_none_or(|e| {
                                    !match e {
                                        ObsPropertyType::Path => true,
                                        _ => false,
                                    }
                                })
                            {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Invalid property type: expected {0:?}, got {1:?}",
                                            ObsPropertyType::Path,
                                            p_type,
                                        ),
                                    );
                                };
                            }
                        };
                    }
                }
            }
            mod text {
                use getters0::Getters;
                use num_traits::FromPrimitive;
                use crate::data::properties::{
                    get_enum, macros::assert_type, ObsTextInfoType, ObsTextType,
                };
                use super::PropertyCreationInfo;
                #[skip_new]
                pub struct ObsTextProperty {
                    name: String,
                    description: String,
                    monospace: bool,
                    text_type: ObsTextType,
                    info_type: ObsTextInfoType,
                    word_wrap: bool,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ObsTextProperty {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "name",
                            "description",
                            "monospace",
                            "text_type",
                            "info_type",
                            "word_wrap",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.name,
                            &self.description,
                            &self.monospace,
                            &self.text_type,
                            &self.info_type,
                            &&self.word_wrap,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "ObsTextProperty",
                            names,
                            values,
                        )
                    }
                }
                impl ObsTextProperty {
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                    pub fn description(&self) -> &String {
                        &self.description
                    }
                    pub fn monospace(&self) -> &bool {
                        &self.monospace
                    }
                    pub fn text_type(&self) -> &ObsTextType {
                        &self.text_type
                    }
                    pub fn info_type(&self) -> &ObsTextInfoType {
                        &self.info_type
                    }
                    pub fn word_wrap(&self) -> &bool {
                        &self.word_wrap
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObsTextProperty {
                    #[inline]
                    fn clone(&self) -> ObsTextProperty {
                        ObsTextProperty {
                            name: ::core::clone::Clone::clone(&self.name),
                            description: ::core::clone::Clone::clone(&self.description),
                            monospace: ::core::clone::Clone::clone(&self.monospace),
                            text_type: ::core::clone::Clone::clone(&self.text_type),
                            info_type: ::core::clone::Clone::clone(&self.info_type),
                            word_wrap: ::core::clone::Clone::clone(&self.word_wrap),
                        }
                    }
                }
                impl From<PropertyCreationInfo> for ObsTextProperty {
                    fn from(
                        PropertyCreationInfo {
                            name,
                            description,
                            pointer,
                        }: PropertyCreationInfo,
                    ) -> Self {
                        {
                            use crate::data::properties::ObsPropertyType;
                            use num_traits::FromPrimitive;
                            let p_type = unsafe {
                                libobs::obs_property_get_type(pointer)
                            };
                            let p_type = ObsPropertyType::from_i32(p_type);
                            if p_type
                                .is_none_or(|e| {
                                    !match e {
                                        ObsPropertyType::Text => true,
                                        _ => false,
                                    }
                                })
                            {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Invalid property type: expected {0:?}, got {1:?}",
                                            ObsPropertyType::Text,
                                            p_type,
                                        ),
                                    );
                                };
                            }
                        };
                        let info_type = {
                            let v = unsafe {
                                libobs::obs_property_text_info_type(pointer)
                            };
                            let v = ObsTextInfoType::from_i32(v);
                            if v.is_none() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Invalid {0} type got none", "text_info_type"),
                                    );
                                };
                            }
                            v.unwrap()
                        };
                        let text_type = {
                            let v = unsafe { libobs::obs_property_text_type(pointer) };
                            let v = ObsTextType::from_i32(v);
                            if v.is_none() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Invalid {0} type got none", "text_type"),
                                    );
                                };
                            }
                            v.unwrap()
                        };
                        let monospace = unsafe {
                            libobs::obs_property_text_monospace(pointer)
                        };
                        let word_wrap = unsafe {
                            libobs::obs_property_text_info_word_wrap(pointer)
                        };
                        ObsTextProperty {
                            name,
                            description,
                            monospace,
                            text_type,
                            info_type,
                            word_wrap,
                        }
                    }
                }
            }
            pub(crate) struct PropertyCreationInfo {
                pub name: String,
                pub description: String,
                pub pointer: *mut libobs::obs_property,
            }
            use std::ffi::CStr;
            pub use button::*;
            pub use editable_list::*;
            pub use font::*;
            use libobs::obs_property;
            pub use list::*;
            pub use number::*;
            pub use path::*;
            pub use text::*;
            use super::{macros::impl_general_property, ObsProperty, ObsPropertyType};
            impl ObsPropertyType {
                pub fn to_property_struct(
                    &self,
                    pointer: *mut obs_property,
                ) -> anyhow::Result<ObsProperty> {
                    let name = unsafe { libobs::obs_property_name(pointer) };
                    let name = unsafe { CStr::from_ptr(name) };
                    let name = name.to_str()?.to_string();
                    let description = unsafe {
                        libobs::obs_property_description(pointer)
                    };
                    let description = unsafe { CStr::from_ptr(description) };
                    let description = description.to_str()?.to_string();
                    let info = PropertyCreationInfo {
                        name,
                        description,
                        pointer,
                    };
                    let res = match self {
                        ObsPropertyType::Invalid => {
                            ObsProperty::Invalid("Invalid".to_string())
                        }
                        ObsPropertyType::Bool => ObsProperty::Bool,
                        ObsPropertyType::Int => {
                            ObsProperty::Int(ObsNumberProperty::<i32>::from(info))
                        }
                        ObsPropertyType::Float => {
                            ObsProperty::Float(ObsNumberProperty::<f64>::from(info))
                        }
                        ObsPropertyType::Text => {
                            ObsProperty::Text(ObsTextProperty::from(info))
                        }
                        ObsPropertyType::Path => {
                            ObsProperty::Path(ObsPathProperty::from(info))
                        }
                        ObsPropertyType::List => {
                            ObsProperty::List(ObsListProperty::from(info))
                        }
                        ObsPropertyType::Color => {
                            ObsProperty::Color(ObsColorProperty::from(info))
                        }
                        ObsPropertyType::Button => {
                            ObsProperty::Button(ObsButtonProperty::try_from(info)?)
                        }
                        ObsPropertyType::Font => {
                            ObsProperty::Font(ObsFontProperty::from(info))
                        }
                        ObsPropertyType::EditableList => {
                            ObsProperty::EditableList(
                                ObsEditableListProperty::from(info),
                            )
                        }
                        ObsPropertyType::FrameRate => {
                            ObsProperty::FrameRate(ObsFrameRateProperty::from(info))
                        }
                    };
                    Ok(res)
                }
            }
        }
        use std::ffi::CStr;
        use macros::*;
        pub use enums::*;
        use num_traits::FromPrimitive;
        use types::*;
        pub enum ObsProperty {
            /// A property that is not valid
            Invalid(String),
            /// A boolean property
            Bool,
            /// An integer property
            Int(ObsNumberProperty<i32>),
            /// A float property
            Float(ObsNumberProperty<f64>),
            /// A text property
            Text(ObsTextProperty),
            /// A path property
            Path(ObsPathProperty),
            /// A list property
            List(ObsListProperty),
            /// A color property
            Color(ObsColorProperty),
            /// A button property
            Button(ObsButtonProperty),
            /// A font property
            Font(ObsFontProperty),
            /// An editable list property
            EditableList(ObsEditableListProperty),
            /// A frame rate property
            FrameRate(ObsFrameRateProperty),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsProperty {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ObsProperty::Invalid(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Invalid",
                            &__self_0,
                        )
                    }
                    ObsProperty::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                    ObsProperty::Int(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Int",
                            &__self_0,
                        )
                    }
                    ObsProperty::Float(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Float",
                            &__self_0,
                        )
                    }
                    ObsProperty::Text(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Text",
                            &__self_0,
                        )
                    }
                    ObsProperty::Path(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Path",
                            &__self_0,
                        )
                    }
                    ObsProperty::List(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "List",
                            &__self_0,
                        )
                    }
                    ObsProperty::Color(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Color",
                            &__self_0,
                        )
                    }
                    ObsProperty::Button(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Button",
                            &__self_0,
                        )
                    }
                    ObsProperty::Font(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Font",
                            &__self_0,
                        )
                    }
                    ObsProperty::EditableList(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "EditableList",
                            &__self_0,
                        )
                    }
                    ObsProperty::FrameRate(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "FrameRate",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsProperty {
            #[inline]
            fn clone(&self) -> ObsProperty {
                match self {
                    ObsProperty::Invalid(__self_0) => {
                        ObsProperty::Invalid(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Bool => ObsProperty::Bool,
                    ObsProperty::Int(__self_0) => {
                        ObsProperty::Int(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Float(__self_0) => {
                        ObsProperty::Float(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Text(__self_0) => {
                        ObsProperty::Text(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Path(__self_0) => {
                        ObsProperty::Path(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::List(__self_0) => {
                        ObsProperty::List(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Color(__self_0) => {
                        ObsProperty::Color(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Button(__self_0) => {
                        ObsProperty::Button(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::Font(__self_0) => {
                        ObsProperty::Font(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::EditableList(__self_0) => {
                        ObsProperty::EditableList(::core::clone::Clone::clone(__self_0))
                    }
                    ObsProperty::FrameRate(__self_0) => {
                        ObsProperty::FrameRate(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        pub trait ObsPropertyObjectPrivate {
            fn get_properties_raw(&self) -> *mut libobs::obs_properties_t;
        }
        /// This trait is implemented for all obs objects that can have properties
        pub trait ObsPropertyObject: ObsPropertyObjectPrivate {
            /// Returns the properties of the object
            fn get_properties(&self) -> anyhow::Result<Vec<ObsProperty>> {
                let properties = self.get_properties_raw();
                let mut props = Vec::new();
                let mut property = unsafe { libobs::obs_properties_first(properties) };
                while !property.is_null() {
                    let name = unsafe { libobs::obs_property_name(property) };
                    let name = unsafe { CStr::from_ptr(name as _) };
                    let name = name.to_str()?.to_string();
                    let description = unsafe {
                        libobs::obs_property_description(property)
                    };
                    let description = unsafe { CStr::from_ptr(description as _) };
                    let description = description.to_str()?.to_string();
                    let p_type = unsafe { libobs::obs_property_get_type(property) };
                    let p_type = ObsPropertyType::from_i32(p_type);
                    if p_type.is_none() {
                        props.push(ObsProperty::Invalid(name));
                        continue;
                    }
                    let p_type = p_type.unwrap();
                    unsafe { libobs::obs_property_next(&mut property) };
                }
                unsafe { libobs::obs_properties_destroy(properties) };
                Ok(props)
            }
        }
    }
    pub use lib_support::*;
    /// Contains `obs_data` and its related strings. Note that
    /// this struct prevents string pointers from being freed
    /// by keeping them owned.
    pub struct ObsData {
        obs_data: WrappedObsData,
        strings: Vec<ObsString>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ObsData",
                "obs_data",
                &self.obs_data,
                "strings",
                &&self.strings,
            )
        }
    }
    impl ObsData {
        /// Creates a new empty `ObsData` wrapper for the
        /// libobs `obs_data` data structure.
        ///
        /// `ObsData` can then be populated using the set
        /// functions, which take ownership of the
        /// `ObsString` types to prevent them from being
        /// dropped prematurely. This makes it safer than
        /// using `obs_data` directly from libobs.
        pub fn new() -> Self {
            let obs_data = unsafe { obs_data_create() };
            let strings = Vec::new();
            ObsData {
                obs_data: WrappedObsData(obs_data),
                strings,
            }
        }
        /// Returns a pointer to the raw `obs_data`
        /// represented by `ObsData`.
        pub fn as_ptr(&self) -> *mut obs_data {
            self.obs_data.0
        }
        /// Sets a string in `obs_data` and stores it so
        /// it in `ObsData` does not get freed.
        pub fn set_string(
            &mut self,
            key: impl Into<ObsString>,
            value: impl Into<ObsString>,
        ) -> &mut Self {
            let key = key.into();
            let value = value.into();
            unsafe { obs_data_set_string(self.obs_data.0, key.as_ptr(), value.as_ptr()) }
            self.strings.push(key);
            self.strings.push(value);
            self
        }
        /// Sets an int in `obs_data` and stores the key
        /// in `ObsData` so it does not get freed.
        pub fn set_int(&mut self, key: impl Into<ObsString>, value: i64) -> &mut Self {
            let key = key.into();
            unsafe { obs_data_set_int(self.obs_data.0, key.as_ptr(), value.into()) }
            self.strings.push(key);
            self
        }
        /// Sets a bool in `obs_data` and stores the key
        /// in `ObsData` so it does not get freed.
        pub fn set_bool(&mut self, key: impl Into<ObsString>, value: bool) -> &mut Self {
            let key = key.into();
            unsafe { obs_data_set_bool(self.obs_data.0, key.as_ptr(), value) }
            self.strings.push(key);
            self
        }
        /// Sets a double in `obs_data` and stores the key
        /// in `ObsData` so it does not get freed.
        pub fn set_double(
            &mut self,
            key: impl Into<ObsString>,
            value: f64,
        ) -> &mut Self {
            let key = key.into();
            unsafe { obs_data_set_double(self.obs_data.0, key.as_ptr(), value) }
            self.strings.push(key);
            self
        }
        pub fn from_json(json: &str) -> anyhow::Result<Self> {
            let cstr = CString::new(json)?;
            let strings = Vec::new();
            let result = unsafe { libobs::obs_data_create_from_json(cstr.as_ptr()) };
            if result.is_null() {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Failed to set JSON in obs_data"),
                    );
                    error
                });
            }
            Ok(ObsData {
                obs_data: WrappedObsData(result),
                strings,
            })
        }
        pub fn get_json(&self) -> anyhow::Result<String> {
            let ptr = unsafe { libobs::obs_data_get_json(self.obs_data.0) };
            if ptr.is_null() {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Failed to get JSON from obs_data"),
                    );
                    error
                });
            }
            let ptr = unsafe { CStr::from_ptr(ptr) };
            Ok(ptr.to_str()?.to_string())
        }
    }
    impl Drop for ObsData {
        fn drop(&mut self) {
            unsafe { obs_data_release(self.obs_data.0) }
        }
    }
    impl Clone for ObsData {
        fn clone(&self) -> Self {
            let json = self.get_json().expect("Couldn't get JSON from obs_data");
            Self::from_json(json.as_str()).expect("Couldn't create obs_data from JSON")
        }
    }
}
pub mod sources {
    mod builder {
        use crate::{data::ObsObjectBuilder, scenes::ObsSceneRef, utils::ObsError};
        use super::ObsSourceRef;
        pub const UPDATE_SOURCE_NAME: &'static str = "OBS_INTERNAL_UPDATE (if you see this, you've build a source wrong)";
        pub trait ObsSourceBuilder: ObsObjectBuilder {
            fn add_to_scene<'a>(
                self,
                scene: &'a mut ObsSceneRef,
            ) -> Result<ObsSourceRef, ObsError>
            where
                Self: Sized,
            {
                scene.add_source(self.build())
            }
        }
    }
    pub use builder::*;
    use libobs::{
        obs_source_create, obs_source_release, obs_source_reset_settings,
        obs_source_update,
    };
    use crate::{
        data::{immutable::ImmutableObsData, ObsData},
        unsafe_send::WrappedObsSource, utils::{traits::ObsUpdatable, ObsError, ObsString},
    };
    use std::{ptr, rc::Rc};
    #[allow(dead_code)]
    pub struct ObsSourceRef {
        pub(crate) source: Rc<WrappedObsSource>,
        pub(crate) id: ObsString,
        pub(crate) name: ObsString,
        pub(crate) settings: Rc<ImmutableObsData>,
        pub(crate) hotkey_data: Rc<ImmutableObsData>,
        _guard: Rc<_ObsSourceGuard>,
    }
    #[automatically_derived]
    #[allow(dead_code)]
    impl ::core::fmt::Debug for ObsSourceRef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "source",
                "id",
                "name",
                "settings",
                "hotkey_data",
                "_guard",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.source,
                &self.id,
                &self.name,
                &self.settings,
                &self.hotkey_data,
                &&self._guard,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ObsSourceRef",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    #[allow(dead_code)]
    impl ::core::clone::Clone for ObsSourceRef {
        #[inline]
        fn clone(&self) -> ObsSourceRef {
            ObsSourceRef {
                source: ::core::clone::Clone::clone(&self.source),
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                settings: ::core::clone::Clone::clone(&self.settings),
                hotkey_data: ::core::clone::Clone::clone(&self.hotkey_data),
                _guard: ::core::clone::Clone::clone(&self._guard),
            }
        }
    }
    impl ObsSourceRef {
        pub fn new(
            id: impl Into<ObsString>,
            name: impl Into<ObsString>,
            mut settings: Option<ObsData>,
            mut hotkey_data: Option<ObsData>,
        ) -> Result<Self, ObsError> {
            let id = id.into();
            let name = name.into();
            let settings = match settings.take() {
                Some(x) => ImmutableObsData::from(x),
                None => ImmutableObsData::new(),
            };
            let hotkey_data = match hotkey_data.take() {
                Some(x) => ImmutableObsData::from(x),
                None => ImmutableObsData::new(),
            };
            let source = unsafe {
                obs_source_create(
                    id.as_ptr(),
                    name.as_ptr(),
                    settings.as_ptr(),
                    hotkey_data.as_ptr(),
                )
            };
            if source == ptr::null_mut() {
                return Err(ObsError::NullPointer);
            }
            Ok(Self {
                source: Rc::new(WrappedObsSource(source)),
                id,
                name,
                settings: Rc::new(settings),
                hotkey_data: Rc::new(hotkey_data),
                _guard: Rc::new(_ObsSourceGuard {
                    source: WrappedObsSource(source),
                }),
            })
        }
        pub fn settings(&self) -> &ImmutableObsData {
            &self.settings
        }
        pub fn hotkey_data(&self) -> &ImmutableObsData {
            &self.hotkey_data
        }
        pub fn name(&self) -> String {
            self.name.to_string()
        }
        pub fn id(&self) -> String {
            self.id.to_string()
        }
    }
    impl ObsUpdatable for ObsSourceRef {
        fn update_raw(&mut self, data: ObsData) {
            unsafe { obs_source_update(self.source.0, data.as_ptr()) }
        }
        fn reset_and_update_raw(&mut self, data: ObsData) {
            unsafe {
                obs_source_reset_settings(self.source.0, data.as_ptr());
            }
        }
    }
    struct _ObsSourceGuard {
        source: WrappedObsSource,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _ObsSourceGuard {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_ObsSourceGuard",
                "source",
                &&self.source,
            )
        }
    }
    impl Drop for _ObsSourceGuard {
        fn drop(&mut self) {
            unsafe { obs_source_release(self.source.0) }
        }
    }
}
pub mod encoders {
    use std::{ffi::CStr, os::raw::c_char};
    use num_traits::ToPrimitive;
    use crate::{context::ObsContext, enums::ObsEncoderType, utils::ENCODER_HIDE_FLAGS};
    pub mod audio {
        use libobs::{obs_audio_encoder_create, obs_encoder_release};
        use std::{borrow::Borrow, ptr};
        use crate::{
            data::ObsData, unsafe_send::WrappedObsEncoder, utils::{ObsError, ObsString},
        };
        #[allow(dead_code)]
        pub struct ObsAudioEncoder {
            pub(crate) encoder: WrappedObsEncoder,
            pub(crate) id: ObsString,
            pub(crate) name: ObsString,
            pub(crate) settings: Option<ObsData>,
            pub(crate) hotkey_data: Option<ObsData>,
        }
        #[automatically_derived]
        #[allow(dead_code)]
        impl ::core::fmt::Debug for ObsAudioEncoder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ObsAudioEncoder",
                    "encoder",
                    &self.encoder,
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "settings",
                    &self.settings,
                    "hotkey_data",
                    &&self.hotkey_data,
                )
            }
        }
        impl ObsAudioEncoder {
            pub fn new(
                id: impl Into<ObsString>,
                name: impl Into<ObsString>,
                settings: Option<ObsData>,
                mixer_idx: usize,
                hotkey_data: Option<ObsData>,
            ) -> Result<Self, ObsError> {
                let id = id.into();
                let name = name.into();
                let settings_ptr = match settings.borrow() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let hotkey_data_ptr = match hotkey_data.borrow() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let encoder = unsafe {
                    obs_audio_encoder_create(
                        id.as_ptr(),
                        name.as_ptr(),
                        settings_ptr,
                        mixer_idx,
                        hotkey_data_ptr,
                    )
                };
                if encoder == ptr::null_mut() {
                    return Err(ObsError::NullPointer);
                }
                Ok(Self {
                    encoder: WrappedObsEncoder(encoder),
                    id,
                    name,
                    settings,
                    hotkey_data,
                })
            }
        }
        impl Drop for ObsAudioEncoder {
            fn drop(&mut self) {
                unsafe { obs_encoder_release(self.encoder.0) }
            }
        }
    }
    pub mod video {
        use libobs::{obs_encoder, obs_encoder_release, obs_video_encoder_create};
        use std::{borrow::Borrow, ptr};
        use crate::{
            data::ObsData, unsafe_send::WrappedObsEncoder, utils::{ObsError, ObsString},
        };
        #[allow(dead_code)]
        pub struct ObsVideoEncoder {
            pub(crate) encoder: WrappedObsEncoder,
            pub(crate) id: ObsString,
            pub(crate) name: ObsString,
            pub(crate) settings: Option<ObsData>,
            pub(crate) hotkey_data: Option<ObsData>,
        }
        #[automatically_derived]
        #[allow(dead_code)]
        impl ::core::fmt::Debug for ObsVideoEncoder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ObsVideoEncoder",
                    "encoder",
                    &self.encoder,
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "settings",
                    &self.settings,
                    "hotkey_data",
                    &&self.hotkey_data,
                )
            }
        }
        impl ObsVideoEncoder {
            pub fn new(
                id: impl Into<ObsString>,
                name: impl Into<ObsString>,
                settings: Option<ObsData>,
                hotkey_data: Option<ObsData>,
            ) -> Result<Self, ObsError> {
                let id = id.into();
                let name = name.into();
                let settings_ptr = match settings.borrow() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let hotkey_data_ptr = match hotkey_data.borrow() {
                    Some(x) => x.as_ptr(),
                    None => ptr::null_mut(),
                };
                let encoder = unsafe {
                    obs_video_encoder_create(
                        id.as_ptr(),
                        name.as_ptr(),
                        settings_ptr,
                        hotkey_data_ptr,
                    )
                };
                if encoder == ptr::null_mut() {
                    return Err(ObsError::NullPointer);
                }
                Ok(Self {
                    encoder: WrappedObsEncoder(encoder),
                    id,
                    name,
                    settings,
                    hotkey_data,
                })
            }
            pub fn as_ptr(&self) -> *mut obs_encoder {
                self.encoder.0
            }
        }
        impl Drop for ObsVideoEncoder {
            fn drop(&mut self) {
                unsafe { obs_encoder_release(self.encoder.0) }
            }
        }
    }
    mod enums {
        use crate::utils::ObsString;
        #[allow(non_camel_case_types)]
        pub enum ObsVideoEncoderType {
            OBS_QSV11,
            OBS_QSV11_AV1,
            FFMPEG_NVENC,
            JIM_AV1_NVENC,
            H265_TEXTURE_AMF,
            FFMPEG_HEVC_NVENC,
            H264_TEXTURE_AMF,
            AV1_TEXTURE_AMF,
            OBS_X264,
            Other(String),
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for ObsVideoEncoderType {
            #[inline]
            fn clone(&self) -> ObsVideoEncoderType {
                match self {
                    ObsVideoEncoderType::OBS_QSV11 => ObsVideoEncoderType::OBS_QSV11,
                    ObsVideoEncoderType::OBS_QSV11_AV1 => {
                        ObsVideoEncoderType::OBS_QSV11_AV1
                    }
                    ObsVideoEncoderType::FFMPEG_NVENC => {
                        ObsVideoEncoderType::FFMPEG_NVENC
                    }
                    ObsVideoEncoderType::JIM_AV1_NVENC => {
                        ObsVideoEncoderType::JIM_AV1_NVENC
                    }
                    ObsVideoEncoderType::H265_TEXTURE_AMF => {
                        ObsVideoEncoderType::H265_TEXTURE_AMF
                    }
                    ObsVideoEncoderType::FFMPEG_HEVC_NVENC => {
                        ObsVideoEncoderType::FFMPEG_HEVC_NVENC
                    }
                    ObsVideoEncoderType::H264_TEXTURE_AMF => {
                        ObsVideoEncoderType::H264_TEXTURE_AMF
                    }
                    ObsVideoEncoderType::AV1_TEXTURE_AMF => {
                        ObsVideoEncoderType::AV1_TEXTURE_AMF
                    }
                    ObsVideoEncoderType::OBS_X264 => ObsVideoEncoderType::OBS_X264,
                    ObsVideoEncoderType::Other(__self_0) => {
                        ObsVideoEncoderType::Other(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for ObsVideoEncoderType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ObsVideoEncoderType::OBS_QSV11 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_QSV11")
                    }
                    ObsVideoEncoderType::OBS_QSV11_AV1 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_QSV11_AV1")
                    }
                    ObsVideoEncoderType::FFMPEG_NVENC => {
                        ::core::fmt::Formatter::write_str(f, "FFMPEG_NVENC")
                    }
                    ObsVideoEncoderType::JIM_AV1_NVENC => {
                        ::core::fmt::Formatter::write_str(f, "JIM_AV1_NVENC")
                    }
                    ObsVideoEncoderType::H265_TEXTURE_AMF => {
                        ::core::fmt::Formatter::write_str(f, "H265_TEXTURE_AMF")
                    }
                    ObsVideoEncoderType::FFMPEG_HEVC_NVENC => {
                        ::core::fmt::Formatter::write_str(f, "FFMPEG_HEVC_NVENC")
                    }
                    ObsVideoEncoderType::H264_TEXTURE_AMF => {
                        ::core::fmt::Formatter::write_str(f, "H264_TEXTURE_AMF")
                    }
                    ObsVideoEncoderType::AV1_TEXTURE_AMF => {
                        ::core::fmt::Formatter::write_str(f, "AV1_TEXTURE_AMF")
                    }
                    ObsVideoEncoderType::OBS_X264 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_X264")
                    }
                    ObsVideoEncoderType::Other(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Other",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for ObsVideoEncoderType {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for ObsVideoEncoderType {
            #[inline]
            fn eq(&self, other: &ObsVideoEncoderType) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ObsVideoEncoderType::Other(__self_0),
                            ObsVideoEncoderType::Other(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for ObsVideoEncoderType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialOrd for ObsVideoEncoderType {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ObsVideoEncoderType,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        ObsVideoEncoderType::Other(__self_0),
                        ObsVideoEncoderType::Other(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Ord for ObsVideoEncoderType {
            #[inline]
            fn cmp(&self, other: &ObsVideoEncoderType) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => {
                        match (self, other) {
                            (
                                ObsVideoEncoderType::Other(__self_0),
                                ObsVideoEncoderType::Other(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        impl From<&str> for ObsVideoEncoderType {
            fn from(value: &str) -> ObsVideoEncoderType {
                return match value {
                    "obs_qsv11" => ObsVideoEncoderType::OBS_QSV11,
                    "obs_qsv11_av1" => ObsVideoEncoderType::OBS_QSV11_AV1,
                    "ffmpeg_nvenc" => ObsVideoEncoderType::FFMPEG_NVENC,
                    "jim_av1_nvenc" => ObsVideoEncoderType::JIM_AV1_NVENC,
                    "h265_texture_amf" => ObsVideoEncoderType::H265_TEXTURE_AMF,
                    "ffmpeg_hevc_nvenc" => ObsVideoEncoderType::FFMPEG_HEVC_NVENC,
                    "h264_texture_amf" => ObsVideoEncoderType::H264_TEXTURE_AMF,
                    "av1_texture_amf" => ObsVideoEncoderType::AV1_TEXTURE_AMF,
                    "obs_x264" => ObsVideoEncoderType::OBS_X264,
                    e => ObsVideoEncoderType::Other(e.to_string()),
                };
            }
        }
        impl Into<ObsString> for ObsVideoEncoderType {
            fn into(self) -> ObsString {
                return match self {
                    ObsVideoEncoderType::OBS_QSV11 => ObsString::new("obs_qsv11"),
                    ObsVideoEncoderType::OBS_QSV11_AV1 => ObsString::new("obs_qsv11_av1"),
                    ObsVideoEncoderType::FFMPEG_NVENC => ObsString::new("ffmpeg_nvenc"),
                    ObsVideoEncoderType::JIM_AV1_NVENC => ObsString::new("jim_av1_nvenc"),
                    ObsVideoEncoderType::H265_TEXTURE_AMF => {
                        ObsString::new("h265_texture_amf")
                    }
                    ObsVideoEncoderType::FFMPEG_HEVC_NVENC => {
                        ObsString::new("ffmpeg_hevc_nvenc")
                    }
                    ObsVideoEncoderType::H264_TEXTURE_AMF => {
                        ObsString::new("h264_texture_amf")
                    }
                    ObsVideoEncoderType::AV1_TEXTURE_AMF => {
                        ObsString::new("av1_texture_amf")
                    }
                    ObsVideoEncoderType::OBS_X264 => ObsString::new("obs_x264"),
                    ObsVideoEncoderType::Other(e) => ObsString::new(&e),
                };
            }
        }
        #[allow(non_camel_case_types)]
        pub enum ObsAudioEncoderType {
            JIM_AV1,
            JIM_NVENC,
            FFMPEG_NVENC,
            AMD_AMF_AV1,
            AMD_AMF_H264,
            OBS_QSV11_AV1,
            OBS_QSV11_H264,
            OBS_X264,
            Other(String),
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for ObsAudioEncoderType {
            #[inline]
            fn clone(&self) -> ObsAudioEncoderType {
                match self {
                    ObsAudioEncoderType::JIM_AV1 => ObsAudioEncoderType::JIM_AV1,
                    ObsAudioEncoderType::JIM_NVENC => ObsAudioEncoderType::JIM_NVENC,
                    ObsAudioEncoderType::FFMPEG_NVENC => {
                        ObsAudioEncoderType::FFMPEG_NVENC
                    }
                    ObsAudioEncoderType::AMD_AMF_AV1 => ObsAudioEncoderType::AMD_AMF_AV1,
                    ObsAudioEncoderType::AMD_AMF_H264 => {
                        ObsAudioEncoderType::AMD_AMF_H264
                    }
                    ObsAudioEncoderType::OBS_QSV11_AV1 => {
                        ObsAudioEncoderType::OBS_QSV11_AV1
                    }
                    ObsAudioEncoderType::OBS_QSV11_H264 => {
                        ObsAudioEncoderType::OBS_QSV11_H264
                    }
                    ObsAudioEncoderType::OBS_X264 => ObsAudioEncoderType::OBS_X264,
                    ObsAudioEncoderType::Other(__self_0) => {
                        ObsAudioEncoderType::Other(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for ObsAudioEncoderType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ObsAudioEncoderType::JIM_AV1 => {
                        ::core::fmt::Formatter::write_str(f, "JIM_AV1")
                    }
                    ObsAudioEncoderType::JIM_NVENC => {
                        ::core::fmt::Formatter::write_str(f, "JIM_NVENC")
                    }
                    ObsAudioEncoderType::FFMPEG_NVENC => {
                        ::core::fmt::Formatter::write_str(f, "FFMPEG_NVENC")
                    }
                    ObsAudioEncoderType::AMD_AMF_AV1 => {
                        ::core::fmt::Formatter::write_str(f, "AMD_AMF_AV1")
                    }
                    ObsAudioEncoderType::AMD_AMF_H264 => {
                        ::core::fmt::Formatter::write_str(f, "AMD_AMF_H264")
                    }
                    ObsAudioEncoderType::OBS_QSV11_AV1 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_QSV11_AV1")
                    }
                    ObsAudioEncoderType::OBS_QSV11_H264 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_QSV11_H264")
                    }
                    ObsAudioEncoderType::OBS_X264 => {
                        ::core::fmt::Formatter::write_str(f, "OBS_X264")
                    }
                    ObsAudioEncoderType::Other(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Other",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for ObsAudioEncoderType {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for ObsAudioEncoderType {
            #[inline]
            fn eq(&self, other: &ObsAudioEncoderType) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ObsAudioEncoderType::Other(__self_0),
                            ObsAudioEncoderType::Other(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for ObsAudioEncoderType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialOrd for ObsAudioEncoderType {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ObsAudioEncoderType,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        ObsAudioEncoderType::Other(__self_0),
                        ObsAudioEncoderType::Other(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Ord for ObsAudioEncoderType {
            #[inline]
            fn cmp(&self, other: &ObsAudioEncoderType) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => {
                        match (self, other) {
                            (
                                ObsAudioEncoderType::Other(__self_0),
                                ObsAudioEncoderType::Other(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        impl From<&str> for ObsAudioEncoderType {
            fn from(value: &str) -> ObsAudioEncoderType {
                return match value {
                    "jim_av1" => ObsAudioEncoderType::JIM_AV1,
                    "jim_nvenc" => ObsAudioEncoderType::JIM_NVENC,
                    "ffmpeg_nvenc" => ObsAudioEncoderType::FFMPEG_NVENC,
                    "amd_amf_av1" => ObsAudioEncoderType::AMD_AMF_AV1,
                    "amd_amf_h264" => ObsAudioEncoderType::AMD_AMF_H264,
                    "obs_qsv11_av1" => ObsAudioEncoderType::OBS_QSV11_AV1,
                    "obs_qsv11_h264" => ObsAudioEncoderType::OBS_QSV11_H264,
                    "obs_x264" => ObsAudioEncoderType::OBS_X264,
                    e => ObsAudioEncoderType::Other(e.to_string()),
                };
            }
        }
        impl Into<ObsString> for ObsAudioEncoderType {
            fn into(self) -> ObsString {
                return match self {
                    ObsAudioEncoderType::JIM_AV1 => ObsString::new("jim_av1"),
                    ObsAudioEncoderType::JIM_NVENC => ObsString::new("jim_nvenc"),
                    ObsAudioEncoderType::FFMPEG_NVENC => ObsString::new("ffmpeg_nvenc"),
                    ObsAudioEncoderType::AMD_AMF_AV1 => ObsString::new("amd_amf_av1"),
                    ObsAudioEncoderType::AMD_AMF_H264 => ObsString::new("amd_amf_h264"),
                    ObsAudioEncoderType::OBS_QSV11_AV1 => ObsString::new("obs_qsv11_av1"),
                    ObsAudioEncoderType::OBS_QSV11_H264 => {
                        ObsString::new("obs_qsv11_h264")
                    }
                    ObsAudioEncoderType::OBS_X264 => ObsString::new("obs_x264"),
                    ObsAudioEncoderType::Other(e) => ObsString::new(&e),
                };
            }
        }
    }
    pub use enums::*;
    pub trait ObsContextEncoders {
        fn get_best_video_encoder() -> ObsVideoEncoderType;
        fn get_best_audio_encoder() -> ObsAudioEncoderType;
        fn get_available_audio_encoders() -> Vec<ObsAudioEncoderType>;
        fn get_available_video_encoders() -> Vec<ObsVideoEncoderType>;
    }
    fn get_encoders_raw(encoder_type: ObsEncoderType) -> Vec<String> {
        let type_primitive = encoder_type.to_i32().unwrap();
        let mut n = 0;
        let mut encoders = Vec::new();
        let mut ptr: *const c_char = unsafe { std::mem::zeroed() };
        while unsafe { libobs::obs_enum_encoder_types(n, &mut ptr) } {
            n += 1;
            let cstring = unsafe { CStr::from_ptr(ptr) };
            if let Ok(enc) = cstring.to_str() {
                unsafe {
                    let is_hidden = libobs::obs_get_encoder_caps(ptr)
                        & ENCODER_HIDE_FLAGS != 0;
                    if is_hidden || libobs::obs_get_encoder_type(ptr) != type_primitive {
                        continue;
                    }
                }
                {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!("Found encoder: {0}", enc),
                                lvl,
                                &(
                                    "libobs_wrapper::encoders",
                                    "libobs_wrapper::encoders",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                };
                encoders.push(enc.into());
            }
        }
        encoders.sort_unstable();
        encoders
    }
    impl ObsContextEncoders for ObsContext {
        fn get_best_video_encoder() -> ObsVideoEncoderType {
            Self::get_available_video_encoders().first().unwrap().clone()
        }
        fn get_best_audio_encoder() -> ObsAudioEncoderType {
            Self::get_available_audio_encoders().first().unwrap().clone()
        }
        fn get_available_audio_encoders() -> Vec<ObsAudioEncoderType> {
            get_encoders_raw(ObsEncoderType::Audio)
                .into_iter()
                .map(|x| x.as_str().into())
                .collect()
        }
        fn get_available_video_encoders() -> Vec<ObsVideoEncoderType> {
            get_encoders_raw(ObsEncoderType::Video)
                .into_iter()
                .map(|x| x.as_str().into())
                .collect()
        }
    }
}
pub mod context {
    use std::{
        cell::RefCell, collections::HashMap, ffi::CStr, pin::Pin, ptr, rc::Rc,
        sync::Mutex, thread::{self, ThreadId},
    };
    use crate::{
        crash_handler::main_crash_handler,
        data::{output::ObsOutputRef, video::ObsVideoInfo},
        display::{ObsDisplayCreationData, ObsDisplayRef},
        enums::{ObsLogLevel, ObsResetVideoStatus},
        logger::{extern_log_callback, internal_log_global, LOGGER},
        scenes::ObsSceneRef, unsafe_send::WrappedObsScene,
        utils::{
            ObsError, ObsModules, ObsString, OutputInfo, StartupInfo,
            initialization::load_debug_privilege,
        },
    };
    use anyhow::Result;
    use getters0::Getters;
    use libobs::{audio_output, video_output};
    static OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);
    /// Interface to the OBS context. Only one context
    /// can exist across all threads and any attempt to
    /// create a new context while there is an existing
    /// one will error.
    ///
    /// Note that the order of the struct values is
    /// important! OBS is super specific about how it
    /// does everything. Things are freed early to
    /// latest from top to bottom.
    #[skip_new]
    pub struct ObsContext {
        /// Stores startup info for safe-keeping. This
        /// prevents any use-after-free as these do not
        /// get copied in libobs.
        startup_info: Rc<RefCell<StartupInfo>>,
        #[get_mut]
        displays: Rc<RefCell<HashMap<usize, Rc<Pin<Box<ObsDisplayRef>>>>>>,
        /// Outputs must be stored in order to prevent
        /// early freeing.
        #[allow(dead_code)]
        #[get_mut]
        pub(crate) outputs: Rc<RefCell<Vec<ObsOutputRef>>>,
        #[get_mut]
        pub(crate) scenes: Rc<RefCell<Vec<ObsSceneRef>>>,
        #[skip_getter]
        pub(crate) active_scene: Rc<RefCell<Option<WrappedObsScene>>>,
        #[skip_getter]
        pub(crate) _obs_modules: Rc<ObsModules>,
        /// This allows us to call obs_shutdown() after
        /// everything else has been freed. Doing other-
        /// wise completely crashes the program.
        #[skip_getter]
        context_shutdown_zst: Rc<ObsContextShutdownZST>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsContext {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "startup_info",
                "displays",
                "outputs",
                "scenes",
                "active_scene",
                "_obs_modules",
                "context_shutdown_zst",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.startup_info,
                &self.displays,
                &self.outputs,
                &self.scenes,
                &self.active_scene,
                &self._obs_modules,
                &&self.context_shutdown_zst,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ObsContext",
                names,
                values,
            )
        }
    }
    impl ObsContext {
        pub fn startup_info(&self) -> &Rc<RefCell<StartupInfo>> {
            &self.startup_info
        }
        pub fn displays(
            &self,
        ) -> &Rc<RefCell<HashMap<usize, Rc<Pin<Box<ObsDisplayRef>>>>>> {
            &self.displays
        }
        pub fn outputs(&self) -> &Rc<RefCell<Vec<ObsOutputRef>>> {
            &self.outputs
        }
        pub fn scenes(&self) -> &Rc<RefCell<Vec<ObsSceneRef>>> {
            &self.scenes
        }
        pub fn displays_mut(
            &mut self,
        ) -> &mut Rc<RefCell<HashMap<usize, Rc<Pin<Box<ObsDisplayRef>>>>>> {
            &mut self.displays
        }
        pub fn outputs_mut(&mut self) -> &mut Rc<RefCell<Vec<ObsOutputRef>>> {
            &mut self.outputs
        }
        pub fn scenes_mut(&mut self) -> &mut Rc<RefCell<Vec<ObsSceneRef>>> {
            &mut self.scenes
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsContext {
        #[inline]
        fn clone(&self) -> ObsContext {
            ObsContext {
                startup_info: ::core::clone::Clone::clone(&self.startup_info),
                displays: ::core::clone::Clone::clone(&self.displays),
                outputs: ::core::clone::Clone::clone(&self.outputs),
                scenes: ::core::clone::Clone::clone(&self.scenes),
                active_scene: ::core::clone::Clone::clone(&self.active_scene),
                _obs_modules: ::core::clone::Clone::clone(&self._obs_modules),
                context_shutdown_zst: ::core::clone::Clone::clone(
                    &self.context_shutdown_zst,
                ),
            }
        }
    }
    impl ObsContext {
        /// Initializes libobs on the current thread.
        ///
        /// Note that there can be only one ObsContext
        /// initialized at a time. This is because
        /// libobs is not completely thread-safe.
        ///
        /// Also note that this might leak a very tiny
        /// amount of memory. As a result, it is
        /// probably a good idea not to restart the
        /// OBS context repeatedly over a very long
        /// period of time. If anyone can fix this, it
        /// would be nice.
        pub fn new(info: StartupInfo) -> Result<ObsContext, ObsError> {
            let mutex_lock = OBS_THREAD_ID.lock();
            if let Ok(mut mutex_value) = mutex_lock {
                if *mutex_value != None {
                    return Err(ObsError::ThreadFailure);
                }
                *mutex_value = Some(thread::current().id());
            } else {
                return Err(ObsError::MutexFailure);
            }
            Self::init(info)
        }
        pub fn get_version() -> String {
            let version = unsafe { libobs::obs_get_version_string() };
            let version_cstr = unsafe { CStr::from_ptr(version) };
            version_cstr.to_string_lossy().into_owned()
        }
        pub fn log(&self, level: ObsLogLevel, msg: &str) {
            let mut log = LOGGER.lock().unwrap();
            log.log(level, msg.to_string());
        }
        /// Initializes the libobs context and prepares
        /// it for recording.
        ///
        /// More specifically, it calls `obs_startup`,
        /// `obs_reset_video`, `obs_reset_audio`, and
        /// registers the video and audio encoders.
        ///
        /// At least on Windows x64, it seems that
        /// resetting video and audio is necessary to
        /// prevent a memory leak when restarting the
        /// OBS context. This memory leak is not severe
        /// (~10 KB per restart), but the point is
        /// safety. Thank you @tt2468 for the help!
        fn init(mut info: StartupInfo) -> Result<ObsContext, ObsError> {
            unsafe {
                libobs::obs_init_win32_crash_handler();
            }
            unsafe {
                libobs::base_set_crash_handler(
                    Some(main_crash_handler),
                    std::ptr::null_mut(),
                );
                load_debug_privilege();
                libobs::base_set_log_handler(
                    Some(extern_log_callback),
                    std::ptr::null_mut(),
                );
            }
            let mut log_callback = LOGGER.lock().map_err(|_e| ObsError::MutexFailure)?;
            *log_callback = info.logger.take().expect("Logger can never be null");
            drop(log_callback);
            let locale_str = ObsString::new("en-US");
            let startup_status = unsafe {
                libobs::obs_startup(locale_str.as_ptr(), ptr::null(), ptr::null_mut())
            };
            internal_log_global(
                ObsLogLevel::Info,
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("OBS {0}", Self::get_version()),
                    );
                    res
                }),
            );
            internal_log_global(
                ObsLogLevel::Info,
                "---------------------------------".to_string(),
            );
            if !startup_status {
                return Err(ObsError::Failure);
            }
            let mut obs_modules = ObsModules::add_paths(&info.startup_paths);
            unsafe {
                libobs::obs_reset_audio2(info.obs_audio_info.as_ptr());
            }
            let reset_video_status = Self::reset_video_internal(
                &mut info.obs_video_info,
            );
            if reset_video_status != ObsResetVideoStatus::Success {
                return Err(ObsError::ResetVideoFailure(reset_video_status));
            }
            obs_modules.load_modules();
            internal_log_global(
                ObsLogLevel::Info,
                "==== Startup complete ==============================================="
                    .to_string(),
            );
            Ok(Self {
                startup_info: Rc::new(RefCell::new(info)),
                outputs: Rc::new(RefCell::new(::alloc::vec::Vec::new())),
                displays: Rc::new(RefCell::new(HashMap::new())),
                active_scene: Rc::new(RefCell::new(None)),
                scenes: Rc::new(RefCell::new(::alloc::vec::Vec::new())),
                _obs_modules: Rc::new(obs_modules),
                context_shutdown_zst: Rc::new(ObsContextShutdownZST {}),
            })
        }
        /// Resets the OBS video context. This is often called
        /// when one wants to change a setting related to the
        /// OBS video info sent on startup.
        ///
        /// It is important to register your video encoders to
        /// a video handle after you reset the video context
        /// if you are using a video handle other than the
        /// main video handle. For convenience, this function
        /// sets all video encoder back to the main video handler
        /// by default.
        ///
        /// Note that you cannot reset the graphics module
        /// without destroying the entire OBS context. Trying
        /// so will result in an error.
        pub fn reset_video(&mut self, mut ovi: ObsVideoInfo) -> Result<(), ObsError> {
            if self.startup_info.borrow().obs_video_info.graphics_module()
                != ovi.graphics_module()
            {
                return Err(ObsError::ResetVideoFailureGraphicsModule);
            }
            let reset_video_status = Self::reset_video_internal(&mut ovi);
            if reset_video_status != ObsResetVideoStatus::Success {
                return Err(ObsError::ResetVideoFailure(reset_video_status));
            } else {
                for output in self.outputs.borrow().iter() {
                    for video_encoder in output.get_video_encoders().iter() {
                        unsafe {
                            libobs::obs_encoder_set_video(
                                video_encoder.as_ptr(),
                                ObsContext::get_video_ptr().unwrap(),
                            )
                        }
                    }
                }
                self.startup_info.borrow_mut().obs_video_info = ovi;
                return Ok(());
            }
        }
        pub fn get_video_ptr() -> Result<*mut video_output, ObsError> {
            if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
                if *mutex_value != Some(thread::current().id()) {
                    return Err(ObsError::ThreadFailure);
                }
            } else {
                return Err(ObsError::MutexFailure);
            }
            Ok(unsafe { libobs::obs_get_video() })
        }
        pub fn get_audio_ptr() -> Result<*mut audio_output, ObsError> {
            if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
                if *mutex_value != Some(thread::current().id()) {
                    return Err(ObsError::ThreadFailure);
                }
            } else {
                return Err(ObsError::MutexFailure);
            }
            Ok(unsafe { libobs::obs_get_audio() })
        }
        fn reset_video_internal(ovi: &mut ObsVideoInfo) -> ObsResetVideoStatus {
            let status = num_traits::FromPrimitive::from_i32(unsafe {
                libobs::obs_reset_video(ovi.as_ptr())
            });
            return match status {
                Some(x) => x,
                None => ObsResetVideoStatus::Failure,
            };
        }
        pub fn output(&mut self, info: OutputInfo) -> Result<ObsOutputRef, ObsError> {
            let output = ObsOutputRef::new(info, self.context_shutdown_zst.clone());
            return match output {
                Ok(x) => {
                    let tmp = x.clone();
                    self.outputs.borrow_mut().push(x);
                    Ok(tmp)
                }
                Err(x) => Err(x),
            };
        }
        /// Creates a new display and returns its ID.
        pub fn display(
            &mut self,
            data: ObsDisplayCreationData,
        ) -> Result<usize, ObsError> {
            let display = ObsDisplayRef::new(data, self.context_shutdown_zst.clone())
                .map_err(|e| ObsError::DisplayCreationError(e.to_string()))?;
            let id = display.id();
            self.displays.borrow_mut().insert(id, Rc::new(display));
            Ok(id)
        }
        pub fn remove_display(&mut self, display: &ObsDisplayRef) {
            self.remove_display_by_id(display.id());
        }
        pub fn remove_display_by_id(&mut self, id: usize) {
            self.displays.borrow_mut().remove(&id);
        }
        pub fn get_display_by_id(
            &self,
            id: usize,
        ) -> Option<Rc<Pin<Box<ObsDisplayRef>>>> {
            self.displays.borrow().get(&id).cloned()
        }
        pub fn get_output(&mut self, name: &str) -> Option<ObsOutputRef> {
            self.outputs
                .borrow()
                .iter()
                .find(|x| x.name().to_string().as_str() == name)
                .map(|e| e.clone())
        }
        pub fn scene(&mut self, name: impl Into<ObsString>) -> ObsSceneRef {
            let scene = ObsSceneRef::new(
                name.into(),
                self.active_scene.clone(),
                self.context_shutdown_zst.clone(),
            );
            let tmp = scene.clone();
            self.scenes.borrow_mut().push(scene);
            tmp
        }
    }
    pub(crate) struct ObsContextShutdownZST {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsContextShutdownZST {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "ObsContextShutdownZST")
        }
    }
    impl Drop for ObsContextShutdownZST {
        fn drop(&mut self) {
            for i in 0..libobs::MAX_CHANNELS {
                unsafe { libobs::obs_set_output_source(i, ptr::null_mut()) };
            }
            unsafe { libobs::obs_shutdown() }
            let r = LOGGER.lock();
            match r {
                Ok(mut logger) => {
                    logger.log(ObsLogLevel::Info, "OBS context shutdown.".to_string());
                    let allocs = unsafe { libobs::bnum_allocs() };
                    let level = if allocs > 1 {
                        ObsLogLevel::Error
                    } else {
                        ObsLogLevel::Info
                    };
                    logger
                        .log(
                            level,
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Number of memory leaks: {0}", allocs),
                                );
                                res
                            }),
                        )
                }
                Err(_) => {
                    {
                        ::std::io::_print(
                            format_args!(
                                "OBS context shutdown. (but couldn\'t lock logger)\n",
                            ),
                        );
                    };
                }
            }
            unsafe {
                libobs::base_set_crash_handler(None, std::ptr::null_mut());
                libobs::base_set_log_handler(None, std::ptr::null_mut());
            }
            if let Ok(mut mutex_value) = OBS_THREAD_ID.lock() {
                *mutex_value = None;
            } else if !thread::panicking() {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
    }
}
pub mod logger {
    mod console {
        use crate::enums::ObsLogLevel;
        use super::ObsLogger;
        pub struct ConsoleLogger {
            _private: (),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConsoleLogger {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConsoleLogger",
                    "_private",
                    &&self._private,
                )
            }
        }
        impl ConsoleLogger {
            pub fn new() -> Self {
                Self { _private: () }
            }
        }
        impl ObsLogger for ConsoleLogger {
            fn log(&mut self, level: ObsLogLevel, msg: String) {
                let level_str = ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", level));
                    res
                });
                #[cfg(feature = "color-logger")]
                let level_str = level.colorize(&level_str);
                {
                    ::std::io::_print(format_args!("[{0}] {1}\n", level_str, msg));
                };
            }
        }
    }
    mod file {
        use std::{fs::File, path::Path};
        use chrono::Local;
        use super::ObsLogger;
        /// A logger that writes logs to a file
        pub struct FileLogger {
            file: File,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FileLogger {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "FileLogger",
                    "file",
                    &&self.file,
                )
            }
        }
        impl FileLogger {
            pub fn from_dir(dir: &Path) -> anyhow::Result<Self> {
                let current_local = Local::now();
                let custom_format = current_local.format("%Y-%m-%d-%H-%M-%S");
                Ok(Self {
                    file: File::create(
                        dir
                            .join(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("obs-{0}.log", custom_format),
                                    );
                                    res
                                }),
                            ),
                    )?,
                })
            }
            pub fn from_file(file: &Path) -> anyhow::Result<Self> {
                Ok(Self { file: File::create(file)? })
            }
        }
        impl ObsLogger for FileLogger {
            fn log(&mut self, level: crate::enums::ObsLogLevel, msg: String) {
                use std::io::Write;
                self.file.write_fmt(format_args!("[{0:?}] {1}\n", level, msg)).unwrap();
            }
        }
    }
    pub use file::FileLogger;
    pub use console::ConsoleLogger;
    use std::{fmt::Debug, os::raw::c_void, sync::Mutex};
    use lazy_static::lazy_static;
    use num_traits::FromPrimitive;
    use vsprintf::vsprintf;
    use crate::enums::ObsLogLevel;
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    /// We are using this as global variable because there can only be one obs context
    pub struct LOGGER {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub static LOGGER: LOGGER = LOGGER { __private_field: () };
    impl ::lazy_static::__Deref for LOGGER {
        type Target = Mutex<Box<dyn ObsLogger>>;
        fn deref(&self) -> &Mutex<Box<dyn ObsLogger>> {
            #[inline(always)]
            fn __static_ref_initialize() -> Mutex<Box<dyn ObsLogger>> {
                Mutex::new(Box::new(ConsoleLogger::new()))
            }
            #[inline(always)]
            fn __stability() -> &'static Mutex<Box<dyn ObsLogger>> {
                static LAZY: ::lazy_static::lazy::Lazy<Mutex<Box<dyn ObsLogger>>> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for LOGGER {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub(crate) unsafe extern "C" fn extern_log_callback(
        log_level: i32,
        msg: *const i8,
        args: *mut i8,
        _params: *mut c_void,
    ) {
        let level = ObsLogLevel::from_i32(log_level);
        if level.is_none() {
            {
                ::std::io::_eprint(
                    format_args!("Couldn\'t find log level {0}\n", log_level),
                );
            };
            return;
        }
        let level = level.unwrap();
        let formatted = vsprintf(msg, args);
        if formatted.is_err() {
            {
                ::std::io::_eprint(format_args!("Failed to format log message\n"));
            };
            return;
        }
        let mut logger = LOGGER.lock().unwrap();
        logger.log(level, formatted.unwrap());
    }
    pub trait ObsLogger
    where
        Self: Send + Debug,
    {
        fn log(&mut self, level: ObsLogLevel, msg: String);
    }
    pub(crate) fn internal_log_global(level: ObsLogLevel, msg: String) {
        let mut logger = LOGGER.lock().unwrap();
        logger.log(level, msg);
    }
}
pub mod signals {
    use std::sync::{Mutex, RwLock};
    use anyhow::{anyhow, Result};
    use crossbeam_channel::{unbounded, Receiver, Sender};
    use lazy_static::lazy_static;
    use crate::{data::output::ObsOutputRef, enums::ObsOutputSignal};
    pub type OutputSignalType = (String, ObsOutputSignal);
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct OUTPUT_SIGNALS {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub static OUTPUT_SIGNALS: OUTPUT_SIGNALS = OUTPUT_SIGNALS {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for OUTPUT_SIGNALS {
        type Target = RwLock<(Sender<OutputSignalType>, Receiver<OutputSignalType>)>;
        fn deref(
            &self,
        ) -> &RwLock<(Sender<OutputSignalType>, Receiver<OutputSignalType>)> {
            #[inline(always)]
            fn __static_ref_initialize() -> RwLock<
                (Sender<OutputSignalType>, Receiver<OutputSignalType>),
            > {
                RwLock::new(unbounded())
            }
            #[inline(always)]
            fn __stability() -> &'static RwLock<
                (Sender<OutputSignalType>, Receiver<OutputSignalType>),
            > {
                static LAZY: ::lazy_static::lazy::Lazy<
                    RwLock<(Sender<OutputSignalType>, Receiver<OutputSignalType>)>,
                > = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for OUTPUT_SIGNALS {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    static SIGNALS: Mutex<Vec<OutputSignalType>> = Mutex::new(::alloc::vec::Vec::new());
    pub fn rec_output_signal(output: &ObsOutputRef) -> Result<ObsOutputSignal> {
        let receiver = &OUTPUT_SIGNALS.read().unwrap().1;
        let s = &mut SIGNALS
            .lock()
            .map_err(|e| ::anyhow::Error::msg(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("Failed to lock SIGNALS: {0}", e),
                    );
                    res
                }),
            ))?;
        while let Some(e) = receiver.try_recv().ok() {
            s.push(e);
        }
        for i in 0..s.len() {
            if s[i].0 == output.name().to_string() {
                let s = s.remove(i);
                return Ok(s.1);
            }
        }
        Ok(receiver.recv()?.1)
    }
}
pub mod display {
    //! For this display method to work, another preview window has to be created in order to create a swapchain
    //! This is because the main window renderer is already handled by other processes
    mod creation_data {
        use libobs::{gs_init_data, gs_window};
        use num_traits::ToPrimitive;
        use super::{GsColorFormat, GsZstencilFormat};
        pub struct ObsDisplayCreationData {
            #[cfg(target_family = "windows")]
            pub(super) parent_window: windows::Win32::Foundation::HWND,
            pub(super) x: u32,
            pub(super) y: u32,
            pub(super) width: u32,
            pub(super) height: u32,
            pub(super) format: GsColorFormat,
            pub(super) zsformat: GsZstencilFormat,
            pub(super) adapter: u32,
            pub(super) backbuffers: u32,
            pub(super) background_color: u32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsDisplayCreationData {
            #[inline]
            fn clone(&self) -> ObsDisplayCreationData {
                ObsDisplayCreationData {
                    parent_window: ::core::clone::Clone::clone(&self.parent_window),
                    x: ::core::clone::Clone::clone(&self.x),
                    y: ::core::clone::Clone::clone(&self.y),
                    width: ::core::clone::Clone::clone(&self.width),
                    height: ::core::clone::Clone::clone(&self.height),
                    format: ::core::clone::Clone::clone(&self.format),
                    zsformat: ::core::clone::Clone::clone(&self.zsformat),
                    adapter: ::core::clone::Clone::clone(&self.adapter),
                    backbuffers: ::core::clone::Clone::clone(&self.backbuffers),
                    background_color: ::core::clone::Clone::clone(&self.background_color),
                }
            }
        }
        impl ObsDisplayCreationData {
            #[cfg(target_family = "windows")]
            pub fn new(
                parent_window: isize,
                x: u32,
                y: u32,
                width: u32,
                height: u32,
            ) -> Self {
                use std::os::raw::c_void;
                use windows::Win32::Foundation::HWND;
                Self {
                    parent_window: HWND(parent_window as *mut c_void),
                    format: GsColorFormat::BGRA,
                    zsformat: GsZstencilFormat::ZSNone,
                    x,
                    y,
                    width,
                    height,
                    adapter: 0,
                    backbuffers: 0,
                    background_color: 0,
                }
            }
            pub fn set_format(mut self, format: GsColorFormat) -> Self {
                self.format = format;
                self
            }
            pub fn set_zsformat(mut self, zsformat: GsZstencilFormat) -> Self {
                self.zsformat = zsformat;
                self
            }
            pub fn set_adapter(mut self, adapter: u32) -> Self {
                self.adapter = adapter;
                self
            }
            pub fn set_backbuffers(mut self, backbuffers: u32) -> Self {
                self.backbuffers = backbuffers;
                self
            }
            pub fn set_background_color(mut self, background_color: u32) -> Self {
                self.background_color = background_color;
                self
            }
            pub(super) fn build(self, window: gs_window) -> gs_init_data {
                let data = gs_init_data {
                    cx: self.width,
                    cy: self.height,
                    format: self.format.to_i32().unwrap(),
                    zsformat: self.zsformat.to_i32().unwrap(),
                    window,
                    adapter: self.adapter,
                    num_backbuffers: self.backbuffers,
                };
                data
            }
        }
    }
    mod enums {
        use libobs::{
            gs_color_format_GS_A8, gs_color_format_GS_BGRA,
            gs_color_format_GS_BGRA_UNORM, gs_color_format_GS_BGRX,
            gs_color_format_GS_BGRX_UNORM, gs_color_format_GS_DXT1,
            gs_color_format_GS_DXT3, gs_color_format_GS_DXT5,
            gs_color_format_GS_R10G10B10A2, gs_color_format_GS_R16,
            gs_color_format_GS_R16F, gs_color_format_GS_R32F, gs_color_format_GS_R8,
            gs_color_format_GS_R8G8, gs_color_format_GS_RG16, gs_color_format_GS_RG16F,
            gs_color_format_GS_RG32F, gs_color_format_GS_RGBA, gs_color_format_GS_RGBA16,
            gs_color_format_GS_RGBA16F, gs_color_format_GS_RGBA32F,
            gs_color_format_GS_RGBA_UNORM, gs_color_format_GS_UNKNOWN,
            gs_zstencil_format_GS_Z16, gs_zstencil_format_GS_Z24_S8,
            gs_zstencil_format_GS_Z32F, gs_zstencil_format_GS_Z32F_S8X24,
            gs_zstencil_format_GS_ZS_NONE,
        };
        use num_derive::{FromPrimitive, ToPrimitive};
        #[repr(i32)]
        pub enum GsColorFormat {
            Unknown = gs_color_format_GS_UNKNOWN,
            A8 = gs_color_format_GS_A8,
            R8 = gs_color_format_GS_R8,
            RGBA = gs_color_format_GS_RGBA,
            BGRX = gs_color_format_GS_BGRX,
            BGRA = gs_color_format_GS_BGRA,
            R10G10B10A2 = gs_color_format_GS_R10G10B10A2,
            RGBA16 = gs_color_format_GS_RGBA16,
            R16 = gs_color_format_GS_R16,
            RGBA16F = gs_color_format_GS_RGBA16F,
            RGBA32F = gs_color_format_GS_RGBA32F,
            RG16F = gs_color_format_GS_RG16F,
            RG32F = gs_color_format_GS_RG32F,
            R16F = gs_color_format_GS_R16F,
            R32F = gs_color_format_GS_R32F,
            DXT1 = gs_color_format_GS_DXT1,
            DXT3 = gs_color_format_GS_DXT3,
            DXT5 = gs_color_format_GS_DXT5,
            R8G8 = gs_color_format_GS_R8G8,
            RGBAUnorm = gs_color_format_GS_RGBA_UNORM,
            BGRXUnorm = gs_color_format_GS_BGRX_UNORM,
            BGRAUnorm = gs_color_format_GS_BGRA_UNORM,
            RG16 = gs_color_format_GS_RG16,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GsColorFormat {
            #[inline]
            fn clone(&self) -> GsColorFormat {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for GsColorFormat {}
        #[automatically_derived]
        impl ::core::fmt::Debug for GsColorFormat {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        GsColorFormat::Unknown => "Unknown",
                        GsColorFormat::A8 => "A8",
                        GsColorFormat::R8 => "R8",
                        GsColorFormat::RGBA => "RGBA",
                        GsColorFormat::BGRX => "BGRX",
                        GsColorFormat::BGRA => "BGRA",
                        GsColorFormat::R10G10B10A2 => "R10G10B10A2",
                        GsColorFormat::RGBA16 => "RGBA16",
                        GsColorFormat::R16 => "R16",
                        GsColorFormat::RGBA16F => "RGBA16F",
                        GsColorFormat::RGBA32F => "RGBA32F",
                        GsColorFormat::RG16F => "RG16F",
                        GsColorFormat::RG32F => "RG32F",
                        GsColorFormat::R16F => "R16F",
                        GsColorFormat::R32F => "R32F",
                        GsColorFormat::DXT1 => "DXT1",
                        GsColorFormat::DXT3 => "DXT3",
                        GsColorFormat::DXT5 => "DXT5",
                        GsColorFormat::R8G8 => "R8G8",
                        GsColorFormat::RGBAUnorm => "RGBAUnorm",
                        GsColorFormat::BGRXUnorm => "BGRXUnorm",
                        GsColorFormat::BGRAUnorm => "BGRAUnorm",
                        GsColorFormat::RG16 => "RG16",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GsColorFormat {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GsColorFormat {
            #[inline]
            fn eq(&self, other: &GsColorFormat) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for GsColorFormat {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::FromPrimitive for GsColorFormat {
                #[allow(trivial_numeric_casts)]
                #[inline]
                fn from_i64(n: i64) -> ::core::option::Option<Self> {
                    if n == GsColorFormat::Unknown as i64 {
                        ::core::option::Option::Some(GsColorFormat::Unknown)
                    } else if n == GsColorFormat::A8 as i64 {
                        ::core::option::Option::Some(GsColorFormat::A8)
                    } else if n == GsColorFormat::R8 as i64 {
                        ::core::option::Option::Some(GsColorFormat::R8)
                    } else if n == GsColorFormat::RGBA as i64 {
                        ::core::option::Option::Some(GsColorFormat::RGBA)
                    } else if n == GsColorFormat::BGRX as i64 {
                        ::core::option::Option::Some(GsColorFormat::BGRX)
                    } else if n == GsColorFormat::BGRA as i64 {
                        ::core::option::Option::Some(GsColorFormat::BGRA)
                    } else if n == GsColorFormat::R10G10B10A2 as i64 {
                        ::core::option::Option::Some(GsColorFormat::R10G10B10A2)
                    } else if n == GsColorFormat::RGBA16 as i64 {
                        ::core::option::Option::Some(GsColorFormat::RGBA16)
                    } else if n == GsColorFormat::R16 as i64 {
                        ::core::option::Option::Some(GsColorFormat::R16)
                    } else if n == GsColorFormat::RGBA16F as i64 {
                        ::core::option::Option::Some(GsColorFormat::RGBA16F)
                    } else if n == GsColorFormat::RGBA32F as i64 {
                        ::core::option::Option::Some(GsColorFormat::RGBA32F)
                    } else if n == GsColorFormat::RG16F as i64 {
                        ::core::option::Option::Some(GsColorFormat::RG16F)
                    } else if n == GsColorFormat::RG32F as i64 {
                        ::core::option::Option::Some(GsColorFormat::RG32F)
                    } else if n == GsColorFormat::R16F as i64 {
                        ::core::option::Option::Some(GsColorFormat::R16F)
                    } else if n == GsColorFormat::R32F as i64 {
                        ::core::option::Option::Some(GsColorFormat::R32F)
                    } else if n == GsColorFormat::DXT1 as i64 {
                        ::core::option::Option::Some(GsColorFormat::DXT1)
                    } else if n == GsColorFormat::DXT3 as i64 {
                        ::core::option::Option::Some(GsColorFormat::DXT3)
                    } else if n == GsColorFormat::DXT5 as i64 {
                        ::core::option::Option::Some(GsColorFormat::DXT5)
                    } else if n == GsColorFormat::R8G8 as i64 {
                        ::core::option::Option::Some(GsColorFormat::R8G8)
                    } else if n == GsColorFormat::RGBAUnorm as i64 {
                        ::core::option::Option::Some(GsColorFormat::RGBAUnorm)
                    } else if n == GsColorFormat::BGRXUnorm as i64 {
                        ::core::option::Option::Some(GsColorFormat::BGRXUnorm)
                    } else if n == GsColorFormat::BGRAUnorm as i64 {
                        ::core::option::Option::Some(GsColorFormat::BGRAUnorm)
                    } else if n == GsColorFormat::RG16 as i64 {
                        ::core::option::Option::Some(GsColorFormat::RG16)
                    } else {
                        ::core::option::Option::None
                    }
                }
                #[inline]
                fn from_u64(n: u64) -> ::core::option::Option<Self> {
                    Self::from_i64(n as i64)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::ToPrimitive for GsColorFormat {
                #[inline]
                #[allow(trivial_numeric_casts)]
                fn to_i64(&self) -> ::core::option::Option<i64> {
                    ::core::option::Option::Some(
                        match *self {
                            GsColorFormat::Unknown => GsColorFormat::Unknown as i64,
                            GsColorFormat::A8 => GsColorFormat::A8 as i64,
                            GsColorFormat::R8 => GsColorFormat::R8 as i64,
                            GsColorFormat::RGBA => GsColorFormat::RGBA as i64,
                            GsColorFormat::BGRX => GsColorFormat::BGRX as i64,
                            GsColorFormat::BGRA => GsColorFormat::BGRA as i64,
                            GsColorFormat::R10G10B10A2 => {
                                GsColorFormat::R10G10B10A2 as i64
                            }
                            GsColorFormat::RGBA16 => GsColorFormat::RGBA16 as i64,
                            GsColorFormat::R16 => GsColorFormat::R16 as i64,
                            GsColorFormat::RGBA16F => GsColorFormat::RGBA16F as i64,
                            GsColorFormat::RGBA32F => GsColorFormat::RGBA32F as i64,
                            GsColorFormat::RG16F => GsColorFormat::RG16F as i64,
                            GsColorFormat::RG32F => GsColorFormat::RG32F as i64,
                            GsColorFormat::R16F => GsColorFormat::R16F as i64,
                            GsColorFormat::R32F => GsColorFormat::R32F as i64,
                            GsColorFormat::DXT1 => GsColorFormat::DXT1 as i64,
                            GsColorFormat::DXT3 => GsColorFormat::DXT3 as i64,
                            GsColorFormat::DXT5 => GsColorFormat::DXT5 as i64,
                            GsColorFormat::R8G8 => GsColorFormat::R8G8 as i64,
                            GsColorFormat::RGBAUnorm => GsColorFormat::RGBAUnorm as i64,
                            GsColorFormat::BGRXUnorm => GsColorFormat::BGRXUnorm as i64,
                            GsColorFormat::BGRAUnorm => GsColorFormat::BGRAUnorm as i64,
                            GsColorFormat::RG16 => GsColorFormat::RG16 as i64,
                        },
                    )
                }
                #[inline]
                fn to_u64(&self) -> ::core::option::Option<u64> {
                    self.to_i64().map(|x| x as u64)
                }
            }
        };
        #[repr(i32)]
        pub enum GsZstencilFormat {
            ZSNone = gs_zstencil_format_GS_ZS_NONE,
            Z16 = gs_zstencil_format_GS_Z16,
            Z24s8 = gs_zstencil_format_GS_Z24_S8,
            Z32F = gs_zstencil_format_GS_Z32F,
            Z32s8X24 = gs_zstencil_format_GS_Z32F_S8X24,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GsZstencilFormat {
            #[inline]
            fn clone(&self) -> GsZstencilFormat {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for GsZstencilFormat {}
        #[automatically_derived]
        impl ::core::fmt::Debug for GsZstencilFormat {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        GsZstencilFormat::ZSNone => "ZSNone",
                        GsZstencilFormat::Z16 => "Z16",
                        GsZstencilFormat::Z24s8 => "Z24s8",
                        GsZstencilFormat::Z32F => "Z32F",
                        GsZstencilFormat::Z32s8X24 => "Z32s8X24",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GsZstencilFormat {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GsZstencilFormat {
            #[inline]
            fn eq(&self, other: &GsZstencilFormat) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for GsZstencilFormat {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::FromPrimitive for GsZstencilFormat {
                #[allow(trivial_numeric_casts)]
                #[inline]
                fn from_i64(n: i64) -> ::core::option::Option<Self> {
                    if n == GsZstencilFormat::ZSNone as i64 {
                        ::core::option::Option::Some(GsZstencilFormat::ZSNone)
                    } else if n == GsZstencilFormat::Z16 as i64 {
                        ::core::option::Option::Some(GsZstencilFormat::Z16)
                    } else if n == GsZstencilFormat::Z24s8 as i64 {
                        ::core::option::Option::Some(GsZstencilFormat::Z24s8)
                    } else if n == GsZstencilFormat::Z32F as i64 {
                        ::core::option::Option::Some(GsZstencilFormat::Z32F)
                    } else if n == GsZstencilFormat::Z32s8X24 as i64 {
                        ::core::option::Option::Some(GsZstencilFormat::Z32s8X24)
                    } else {
                        ::core::option::Option::None
                    }
                }
                #[inline]
                fn from_u64(n: u64) -> ::core::option::Option<Self> {
                    Self::from_i64(n as i64)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::ToPrimitive for GsZstencilFormat {
                #[inline]
                #[allow(trivial_numeric_casts)]
                fn to_i64(&self) -> ::core::option::Option<i64> {
                    ::core::option::Option::Some(
                        match *self {
                            GsZstencilFormat::ZSNone => GsZstencilFormat::ZSNone as i64,
                            GsZstencilFormat::Z16 => GsZstencilFormat::Z16 as i64,
                            GsZstencilFormat::Z24s8 => GsZstencilFormat::Z24s8 as i64,
                            GsZstencilFormat::Z32F => GsZstencilFormat::Z32F as i64,
                            GsZstencilFormat::Z32s8X24 => {
                                GsZstencilFormat::Z32s8X24 as i64
                            }
                        },
                    )
                }
                #[inline]
                fn to_u64(&self) -> ::core::option::Option<u64> {
                    self.to_i64().map(|x| x as u64)
                }
            }
        };
    }
    mod window_manager {
        //! This
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc, Mutex,
        };
        use lazy_static::lazy_static;
        use windows::{
            core::{w, HSTRING, PCWSTR},
            Win32::{
                Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM},
                Graphics::Dwm::DwmIsCompositionEnabled,
                System::{
                    LibraryLoader::{GetModuleHandleA, GetModuleHandleW},
                    SystemInformation::{GetVersionExW, OSVERSIONINFOW},
                },
                UI::WindowsAndMessaging::{
                    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW,
                    GetWindowLongPtrW, LoadCursorW, PostMessageW, PostQuitMessage,
                    RegisterClassExW, SetLayeredWindowAttributes, SetParent,
                    SetWindowLongPtrW, TranslateMessage, CS_HREDRAW, CS_NOCLOSE,
                    CS_OWNDC, CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, HTTRANSPARENT,
                    IDC_ARROW, LWA_ALPHA, MSG, WM_NCHITTEST, WNDCLASSEXW, WS_CHILD,
                    WS_EX_COMPOSITED, WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_POPUP,
                    WS_VISIBLE,
                },
            },
        };
        use crate::unsafe_send::{WrappedHWND, WrappedObsDisplay};
        mod position_trait {
            use libobs::obs_display_resize;
            use windows::Win32::{
                Foundation::HWND,
                Graphics::Gdi::{RedrawWindow, RDW_ERASE, RDW_INVALIDATE},
                UI::WindowsAndMessaging::{
                    SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOCOPYBITS,
                    SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
                },
            };
            use crate::display::ObsDisplayRef;
            pub trait WindowPositionTrait {
                fn set_render_at_bottom(&self, render_at_bottom: bool);
                fn get_render_at_bottom(&self) -> bool;
                fn set_pos(&self, x: i32, y: i32) -> windows::core::Result<()>;
                fn set_size(&self, width: u32, height: u32) -> windows::core::Result<()>;
                fn set_scale(&self, scale: f32);
                fn get_pos(&self) -> (i32, i32);
                fn get_size(&self) -> (u32, u32);
                fn get_scale(&self) -> f32;
            }
            impl WindowPositionTrait for ObsDisplayRef {
                fn set_render_at_bottom(&self, render_at_bottom: bool) {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Set render bottom"),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    self.manager.write().render_at_bottom = render_at_bottom;
                }
                fn get_render_at_bottom(&self) -> bool {
                    self.manager.read().render_at_bottom
                }
                fn set_pos(&self, x: i32, y: i32) -> windows::core::Result<()> {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Set pos {0} {1}", x, y),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let mut m = self.manager.write();
                    if !m.obs_display.is_some() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Invalid state. The display should have been created and set, but it wasn\'t.",
                                ),
                            );
                        }
                    }
                    let insert_after = if m.render_at_bottom {
                        HWND_BOTTOM
                    } else {
                        HWND::default()
                    };
                    m.x = x;
                    m.y = y;
                    unsafe {
                        let flags = SWP_NOCOPYBITS | SWP_NOSIZE | SWP_NOACTIVATE;
                        SetWindowPos(
                            m.hwnd.0,
                            Some(insert_after),
                            x,
                            y,
                            1 as i32,
                            1 as i32,
                            flags,
                        )?;
                    }
                    Ok(())
                }
                fn get_pos(&self) -> (i32, i32) {
                    let m = self.manager.read();
                    (m.x, m.y)
                }
                fn get_size(&self) -> (u32, u32) {
                    let m = self.manager.read();
                    (m.width, m.height)
                }
                fn set_size(
                    &self,
                    width: u32,
                    height: u32,
                ) -> windows::core::Result<()> {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Set size {0} {1}", width, height),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let mut m = self.manager.write();
                    if !m.obs_display.is_some() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Invalid state. The display should have been created and set, but it wasn\'t.",
                                ),
                            );
                        }
                    }
                    m.width = width;
                    m.height = height;
                    let pointer = m.obs_display.as_ref().unwrap().0;
                    unsafe {
                        SetWindowPos(
                            m.hwnd.0,
                            None,
                            m.x,
                            m.y,
                            width as i32,
                            height as i32,
                            SWP_NOCOPYBITS | SWP_NOACTIVATE | SWP_NOZORDER
                                | SWP_SHOWWINDOW,
                        )?;
                        let _ = RedrawWindow(
                            Some(m.hwnd.0),
                            None,
                            None,
                            RDW_ERASE | RDW_INVALIDATE,
                        );
                        obs_display_resize(pointer, width, height);
                    }
                    Ok(())
                }
                fn set_scale(&self, scale: f32) {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Set scale {0}", scale),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        "libobs_wrapper::display::window_manager::position_trait",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    self.manager.write().scale = scale;
                }
                fn get_scale(&self) -> f32 {
                    self.manager.read().scale
                }
            }
        }
        mod show_hide {
            use std::sync::atomic::Ordering;
            use windows::Win32::UI::WindowsAndMessaging::{
                ShowWindow, SW_HIDE, SW_SHOWNA,
            };
            use crate::display::ObsDisplayRef;
            pub trait ShowHideTrait {
                fn show(&mut self);
                fn hide(&mut self);
                fn is_visible(&self) -> bool;
            }
            impl ShowHideTrait for ObsDisplayRef {
                fn show(&mut self) {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("show"),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::show_hide",
                                        "libobs_wrapper::display::window_manager::show_hide",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let m = self.manager.read();
                    unsafe {
                        let _ = ShowWindow(m.hwnd.0, SW_SHOWNA);
                    }
                    m.is_hidden.store(false, Ordering::Relaxed);
                }
                fn hide(&mut self) {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("hide"),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager::show_hide",
                                        "libobs_wrapper::display::window_manager::show_hide",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let m = self.manager.read();
                    unsafe {
                        let _ = ShowWindow(m.hwnd.0, SW_HIDE);
                    }
                    m.is_hidden.store(true, Ordering::Relaxed);
                }
                fn is_visible(&self) -> bool {
                    self.manager.read().is_hidden.load(Ordering::Relaxed)
                }
            }
        }
        pub use position_trait::WindowPositionTrait;
        pub use show_hide::ShowHideTrait;
        const WM_DESTROY_WINDOW: u32 = 0x8001;
        extern "system" fn wndproc(
            window: HWND,
            message: u32,
            w_param: WPARAM,
            l_param: LPARAM,
        ) -> LRESULT {
            unsafe {
                match message {
                    WM_NCHITTEST => {
                        return LRESULT(HTTRANSPARENT as _);
                    }
                    WM_DESTROY_WINDOW => {
                        PostQuitMessage(0);
                        return LRESULT(0);
                    }
                    _ => {
                        return DefWindowProcW(window, message, w_param, l_param);
                    }
                }
            }
        }
        fn is_windows8_or_greater() -> windows::core::Result<bool> {
            let mut os_info: OSVERSIONINFOW = unsafe { std::mem::zeroed() };
            os_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;
            unsafe {
                GetVersionExW(&mut os_info)?;
            }
            let r = (os_info.dwMajorVersion > 6)
                || (os_info.dwMajorVersion == 6 && os_info.dwMinorVersion >= 2);
            Ok(r)
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct REGISTERED_CLASS {
            __private_field: (),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static REGISTERED_CLASS: REGISTERED_CLASS = REGISTERED_CLASS {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for REGISTERED_CLASS {
            type Target = AtomicBool;
            fn deref(&self) -> &AtomicBool {
                #[inline(always)]
                fn __static_ref_initialize() -> AtomicBool {
                    AtomicBool::new(false)
                }
                #[inline(always)]
                fn __stability() -> &'static AtomicBool {
                    static LAZY: ::lazy_static::lazy::Lazy<AtomicBool> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for REGISTERED_CLASS {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        fn try_register_class() -> windows::core::Result<()> {
            if REGISTERED_CLASS.load(Ordering::Relaxed) {
                return Ok(());
            }
            unsafe {
                let instance = GetModuleHandleA(None)?;
                let cursor = LoadCursorW(None, IDC_ARROW)?;
                let mut style = CS_HREDRAW | CS_VREDRAW | CS_NOCLOSE;
                let enabled = DwmIsCompositionEnabled()?.as_bool();
                if is_windows8_or_greater()? || !enabled {
                    style |= CS_OWNDC;
                }
                let window_class = {
                    const INPUT: &[u8] = "Win32DisplayClass".as_bytes();
                    const OUTPUT_LEN: usize = ::windows_strings::utf16_len(INPUT) + 1;
                    const OUTPUT: &[u16; OUTPUT_LEN] = {
                        let mut buffer = [0; OUTPUT_LEN];
                        let mut input_pos = 0;
                        let mut output_pos = 0;
                        while let Some((mut code_point, new_pos)) = ::windows_strings::decode_utf8_char(
                            INPUT,
                            input_pos,
                        ) {
                            input_pos = new_pos;
                            if code_point <= 0xffff {
                                buffer[output_pos] = code_point as u16;
                                output_pos += 1;
                            } else {
                                code_point -= 0x10000;
                                buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                                output_pos += 1;
                                buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                                output_pos += 1;
                            }
                        }
                        &{ buffer }
                    };
                    ::windows_strings::PCWSTR::from_raw(OUTPUT.as_ptr())
                };
                let wc = WNDCLASSEXW {
                    cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                    hCursor: cursor,
                    hInstance: instance.into(),
                    lpszClassName: window_class,
                    style: CS_HREDRAW | CS_VREDRAW,
                    lpfnWndProc: Some(wndproc),
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    ..Default::default()
                };
                let atom = RegisterClassExW(&wc as *const _);
                if atom == 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
            }
            REGISTERED_CLASS.store(true, Ordering::Relaxed);
            Ok(())
        }
        pub struct DisplayWindowManager {
            message_thread: Option<std::thread::JoinHandle<()>>,
            should_exit: Arc<AtomicBool>,
            hwnd: WrappedHWND,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
            scale: f32,
            is_hidden: AtomicBool,
            render_at_bottom: bool,
            pub(super) obs_display: Option<WrappedObsDisplay>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DisplayWindowManager {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "message_thread",
                    "should_exit",
                    "hwnd",
                    "x",
                    "y",
                    "width",
                    "height",
                    "scale",
                    "is_hidden",
                    "render_at_bottom",
                    "obs_display",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.message_thread,
                    &self.should_exit,
                    &self.hwnd,
                    &self.x,
                    &self.y,
                    &self.width,
                    &self.height,
                    &self.scale,
                    &self.is_hidden,
                    &self.render_at_bottom,
                    &&self.obs_display,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "DisplayWindowManager",
                    names,
                    values,
                )
            }
        }
        struct SendableHWND(pub HWND);
        #[automatically_derived]
        impl ::core::fmt::Debug for SendableHWND {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SendableHWND",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SendableHWND {
            #[inline]
            fn clone(&self) -> SendableHWND {
                let _: ::core::clone::AssertParamIsClone<HWND>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SendableHWND {}
        unsafe impl Sync for SendableHWND {}
        unsafe impl Send for SendableHWND {}
        impl DisplayWindowManager {
            pub fn new(
                parent: HWND,
                x: i32,
                y: i32,
                width: u32,
                height: u32,
            ) -> anyhow::Result<Self> {
                let (tx, rx) = oneshot::channel();
                let should_exit = Arc::new(AtomicBool::new(false));
                let tmp = should_exit.clone();
                let parent = Mutex::new(SendableHWND(parent));
                let message_thread = std::thread::spawn(move || {
                    let parent = parent.lock().unwrap().0;
                    let create = move || {
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!("Registering class..."),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        try_register_class()?;
                        let win8 = is_windows8_or_greater()?;
                        let enabled = unsafe { DwmIsCompositionEnabled()?.as_bool() };
                        let mut window_style = WS_EX_TRANSPARENT;
                        if win8 && enabled {
                            window_style |= WS_EX_COMPOSITED;
                        }
                        let instance = unsafe { GetModuleHandleW(PCWSTR::null())? };
                        let class_name = HSTRING::from("Win32DisplayClass");
                        let window_name = HSTRING::from("LibObsChildWindowPreview");
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!("Creating window..."),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Creating window with x: {0}, y: {1}, width: {2}, height: {3}",
                                            x,
                                            y,
                                            width,
                                            height,
                                        ),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        let window = unsafe {
                            CreateWindowExW(
                                WS_EX_LAYERED,
                                &class_name,
                                &window_name,
                                WS_POPUP | WS_VISIBLE,
                                x,
                                y,
                                width as i32,
                                height as i32,
                                None,
                                None,
                                Some(instance.into()),
                                None,
                            )?
                        };
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!("HWND is {0:?}", window),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        if win8 || !enabled {
                            {
                                {
                                    let lvl = ::log::Level::Trace;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!("Setting attributes alpha..."),
                                            lvl,
                                            &(
                                                "libobs_wrapper::display::window_manager",
                                                "libobs_wrapper::display::window_manager",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            };
                            unsafe {
                                SetLayeredWindowAttributes(
                                    window,
                                    COLORREF(0),
                                    255,
                                    LWA_ALPHA,
                                )?;
                            }
                        }
                        unsafe {
                            {
                                {
                                    let lvl = ::log::Level::Trace;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!("Setting parent..."),
                                            lvl,
                                            &(
                                                "libobs_wrapper::display::window_manager",
                                                "libobs_wrapper::display::window_manager",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            };
                            SetParent(window, Some(parent))?;
                            {
                                {
                                    let lvl = ::log::Level::Trace;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!("Setting styles..."),
                                            lvl,
                                            &(
                                                "libobs_wrapper::display::window_manager",
                                                "libobs_wrapper::display::window_manager",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            };
                            let mut style = GetWindowLongPtrW(window, GWL_STYLE);
                            style &= !(WS_POPUP.0 as isize);
                            style |= WS_CHILD.0 as isize;
                            SetWindowLongPtrW(window, GWL_STYLE, style);
                            let mut ex_style = GetWindowLongPtrW(window, GWL_EXSTYLE);
                            ex_style |= window_style.0 as isize;
                            SetWindowLongPtrW(window, GWL_EXSTYLE, ex_style);
                        }
                        Result::<SendableHWND, anyhow::Error>::Ok(SendableHWND(window))
                    };
                    let r = create();
                    let window = r.as_ref().ok().map(|r| r.0.clone());
                    tx.send(r).unwrap();
                    if window.is_none() {
                        return;
                    }
                    let window = window.unwrap();
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Starting up message thread..."),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager",
                                        "libobs_wrapper::display::window_manager",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let mut msg = MSG::default();
                    unsafe {
                        while !tmp.load(Ordering::Relaxed)
                            && GetMessageW(&mut msg, Some(window), 0, 0).as_bool()
                        {
                            let _ = TranslateMessage(&msg);
                            DispatchMessageW(&msg);
                        }
                    }
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Exiting message thread..."),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager",
                                        "libobs_wrapper::display::window_manager",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                });
                let window = rx.recv();
                let window = window??;
                Ok(Self {
                    x,
                    y,
                    width,
                    height,
                    scale: 1.0,
                    hwnd: WrappedHWND(window.0),
                    should_exit,
                    message_thread: Some(message_thread),
                    render_at_bottom: false,
                    is_hidden: AtomicBool::new(false),
                    obs_display: None,
                })
            }
            pub fn get_child_handle(&self) -> HWND {
                self.hwnd.0.clone()
            }
        }
        impl Drop for DisplayWindowManager {
            fn drop(&mut self) {
                unsafe {
                    self.should_exit.store(true, Ordering::Relaxed);
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!("Destroying window..."),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display::window_manager",
                                        "libobs_wrapper::display::window_manager",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    let res = PostMessageW(
                        Some(self.hwnd.0),
                        WM_DESTROY_WINDOW,
                        WPARAM(0),
                        LPARAM(0),
                    );
                    if let Err(err) = res {
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Failed to post destroy window message: {0:?}",
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                    }
                    let thread = self.message_thread.take();
                    if let Some(thread) = thread {
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!("Waiting for message thread to exit..."),
                                        lvl,
                                        &(
                                            "libobs_wrapper::display::window_manager",
                                            "libobs_wrapper::display::window_manager",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        thread.join().unwrap();
                    }
                }
            }
        }
    }
    pub use creation_data::*;
    pub use enums::*;
    use parking_lot::RwLock;
    pub use window_manager::*;
    use std::{
        cell::RefCell, ffi::c_void, marker::PhantomPinned, rc::Rc,
        sync::{atomic::AtomicUsize, Arc},
    };
    use libobs::{
        gs_ortho, gs_projection_pop, gs_projection_push, gs_set_viewport,
        gs_viewport_pop, gs_viewport_push, obs_get_video_info, obs_render_main_texture,
        obs_video_info,
    };
    use crate::{
        context::ObsContextShutdownZST, unsafe_send::{WrappedObsDisplay, WrappedVoidPtr},
    };
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
    /// # NEVER STORE THIS REF DIRECTLY!!
    /// This is a wrapper around the obs_display struct and contains direct memory references.
    /// You should ALWAYS use the context to get to this struct, and as said NEVER store it.
    pub struct ObsDisplayRef {
        display: Rc<WrappedObsDisplay>,
        id: usize,
        _guard: Rc<RefCell<_DisplayDropGuard>>,
        manager: Arc<RwLock<DisplayWindowManager>>,
        /// This must not be moved in memory as the draw callback is a raw pointer to this struct
        _fixed_in_heap: PhantomPinned,
        /// Stored so the obs context is not dropped while this is alive
        _shutdown: Rc<ObsContextShutdownZST>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsDisplayRef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "display",
                "id",
                "_guard",
                "manager",
                "_fixed_in_heap",
                "_shutdown",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.display,
                &self.id,
                &self._guard,
                &self.manager,
                &self._fixed_in_heap,
                &&self._shutdown,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ObsDisplayRef",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsDisplayRef {
        #[inline]
        fn clone(&self) -> ObsDisplayRef {
            ObsDisplayRef {
                display: ::core::clone::Clone::clone(&self.display),
                id: ::core::clone::Clone::clone(&self.id),
                _guard: ::core::clone::Clone::clone(&self._guard),
                manager: ::core::clone::Clone::clone(&self.manager),
                _fixed_in_heap: ::core::clone::Clone::clone(&self._fixed_in_heap),
                _shutdown: ::core::clone::Clone::clone(&self._shutdown),
            }
        }
    }
    unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
        let s = &*(data as *mut ObsDisplayRef);
        let (x, y) = s.get_pos();
        let (width, height) = s.get_size();
        let mut ovi: obs_video_info = std::mem::zeroed();
        obs_get_video_info(&mut ovi);
        gs_viewport_push();
        gs_projection_push();
        gs_ortho(
            0.0f32,
            ovi.base_width as f32,
            0.0f32,
            ovi.base_height as f32,
            -100.0f32,
            100.0f32,
        );
        gs_set_viewport(x as i32, y as i32, width as i32, height as i32);
        obs_render_main_texture();
        gs_projection_pop();
        gs_viewport_pop();
    }
    impl ObsDisplayRef {
        #[cfg(target_family = "windows")]
        /// Call initialize to ObsDisplay#create the display
        /// NOTE: This must be pinned to prevent the draw callbacks from having a invalid pointer. DO NOT UNPIN
        pub(crate) fn new(
            data: creation_data::ObsDisplayCreationData,
            shutdown: Rc<ObsContextShutdownZST>,
        ) -> anyhow::Result<std::pin::Pin<Box<Self>>> {
            use std::{cell::RefCell, sync::atomic::Ordering};
            use anyhow::bail;
            use creation_data::ObsDisplayCreationData;
            use libobs::gs_window;
            use parking_lot::lock_api::RwLock;
            use window_manager::DisplayWindowManager;
            let ObsDisplayCreationData {
                x,
                y,
                height,
                width,
                parent_window,
                background_color,
                ..
            } = data.clone();
            let mut manager = DisplayWindowManager::new(
                parent_window.clone(),
                x as i32,
                y as i32,
                width,
                height,
            )?;
            let child_handle = manager.get_child_handle();
            let init_data = data.build(gs_window { hwnd: child_handle.0 });
            {
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("Creating obs display..."),
                            lvl,
                            &(
                                "libobs_wrapper::display",
                                "libobs_wrapper::display",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                }
            };
            let display = unsafe {
                libobs::obs_display_create(&init_data, background_color)
            };
            if display.is_null() {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!("OBS failed to create display"),
                    );
                    error
                });
            }
            manager.obs_display = Some(WrappedObsDisplay(display));
            let mut instance = Box::pin(Self {
                display: Rc::new(WrappedObsDisplay(display)),
                manager: Arc::new(RwLock::new(manager)),
                id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                _guard: Rc::new(
                    RefCell::new(_DisplayDropGuard {
                        display: WrappedObsDisplay(display),
                        self_ptr: None,
                    }),
                ),
                _fixed_in_heap: PhantomPinned,
                _shutdown: shutdown,
            });
            let instance_ptr = unsafe {
                instance.as_mut().get_unchecked_mut() as *mut _ as *mut c_void
            };
            instance._guard.borrow_mut().self_ptr = Some(WrappedVoidPtr(instance_ptr));
            {
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "Adding draw callback with display {0:?} and draw callback params at {1:?} (pos is {2:?})...",
                                instance.display,
                                instance_ptr,
                                instance.get_pos(),
                            ),
                            lvl,
                            &(
                                "libobs_wrapper::display",
                                "libobs_wrapper::display",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                }
            };
            unsafe {
                libobs::obs_display_add_draw_callback(
                    instance.display.0,
                    Some(render_display),
                    instance_ptr,
                );
            }
            Ok(instance)
        }
        pub fn id(&self) -> usize {
            self.id
        }
    }
    struct _DisplayDropGuard {
        display: WrappedObsDisplay,
        self_ptr: Option<WrappedVoidPtr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _DisplayDropGuard {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "_DisplayDropGuard",
                "display",
                &self.display,
                "self_ptr",
                &&self.self_ptr,
            )
        }
    }
    impl Drop for _DisplayDropGuard {
        fn drop(&mut self) {
            unsafe {
                if let Some(ptr) = &self.self_ptr {
                    {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "Destroying display with callback at {0:?}...",
                                        ptr.0,
                                    ),
                                    lvl,
                                    &(
                                        "libobs_wrapper::display",
                                        "libobs_wrapper::display",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                    libobs::obs_display_remove_draw_callback(
                        self.display.0,
                        Some(render_display),
                        ptr.0,
                    );
                }
                libobs::obs_display_destroy(self.display.0);
            }
        }
    }
}
pub mod scenes {
    use std::{cell::RefCell, rc::Rc};
    use getters0::Getters;
    use libobs::{obs_scene_create, obs_scene_t, obs_set_output_source, obs_source_t};
    use crate::{
        context::ObsContextShutdownZST, sources::ObsSourceRef,
        unsafe_send::WrappedObsScene, utils::{ObsError, ObsString, SourceInfo},
    };
    struct _SceneDropGuard {
        scene: WrappedObsScene,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _SceneDropGuard {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_SceneDropGuard",
                "scene",
                &&self.scene,
            )
        }
    }
    impl Drop for _SceneDropGuard {
        fn drop(&mut self) {
            unsafe {
                libobs::obs_scene_release(self.scene.0);
            }
        }
    }
    #[skip_new]
    pub struct ObsSceneRef {
        #[skip_getter]
        scene: Rc<WrappedObsScene>,
        name: ObsString,
        #[get_mut]
        pub(crate) sources: Rc<RefCell<Vec<ObsSourceRef>>>,
        #[skip_getter]
        pub(crate) active_scene: Rc<RefCell<Option<WrappedObsScene>>>,
        #[skip_getter]
        _guard: Rc<_SceneDropGuard>,
        #[skip_getter]
        _shutdown: Rc<ObsContextShutdownZST>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsSceneRef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "scene",
                "name",
                "sources",
                "active_scene",
                "_guard",
                "_shutdown",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.scene,
                &self.name,
                &self.sources,
                &self.active_scene,
                &self._guard,
                &&self._shutdown,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ObsSceneRef",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsSceneRef {
        #[inline]
        fn clone(&self) -> ObsSceneRef {
            ObsSceneRef {
                scene: ::core::clone::Clone::clone(&self.scene),
                name: ::core::clone::Clone::clone(&self.name),
                sources: ::core::clone::Clone::clone(&self.sources),
                active_scene: ::core::clone::Clone::clone(&self.active_scene),
                _guard: ::core::clone::Clone::clone(&self._guard),
                _shutdown: ::core::clone::Clone::clone(&self._shutdown),
            }
        }
    }
    impl ObsSceneRef {
        pub fn name(&self) -> &ObsString {
            &self.name
        }
        pub fn sources(&self) -> &Rc<RefCell<Vec<ObsSourceRef>>> {
            &self.sources
        }
        pub fn sources_mut(&mut self) -> &mut Rc<RefCell<Vec<ObsSourceRef>>> {
            &mut self.sources
        }
    }
    impl ObsSceneRef {
        pub(crate) fn new(
            name: ObsString,
            active_scene: Rc<RefCell<Option<WrappedObsScene>>>,
            shutdown: Rc<ObsContextShutdownZST>,
        ) -> Self {
            let scene = unsafe { obs_scene_create(name.as_ptr()) };
            Self {
                name,
                scene: Rc::new(WrappedObsScene(scene)),
                sources: Rc::new(RefCell::new(::alloc::vec::Vec::new())),
                active_scene: active_scene.clone(),
                _guard: Rc::new(_SceneDropGuard {
                    scene: WrappedObsScene(scene),
                }),
                _shutdown: shutdown,
            }
        }
        pub fn add_and_set(&self, channel: u32) {
            let mut s = self.active_scene.borrow_mut();
            *s = Some(WrappedObsScene(self.as_ptr()));
            unsafe {
                obs_set_output_source(channel, self.get_scene_source_ptr());
            }
        }
        pub fn get_scene_source_ptr(&self) -> *mut obs_source_t {
            unsafe { libobs::obs_scene_get_source(self.scene.0) }
        }
        pub fn add_source(
            &mut self,
            info: SourceInfo,
        ) -> Result<ObsSourceRef, ObsError> {
            let source = ObsSourceRef::new(
                info.id,
                info.name,
                info.settings,
                info.hotkey_data,
            );
            return match source {
                Ok(x) => {
                    unsafe {
                        libobs::obs_scene_add(self.scene.0, x.source.0);
                    }
                    let tmp = x.clone();
                    self.sources.borrow_mut().push(x);
                    Ok(tmp)
                }
                Err(x) => Err(x),
            };
        }
        pub fn get_source_by_index(&self, index: usize) -> Option<ObsSourceRef> {
            self.sources.borrow().get(index).map(|x| x.clone())
        }
        pub fn get_source_mut(&self, name: &str) -> Option<ObsSourceRef> {
            self.sources.borrow().iter().find(|x| x.name() == name).map(|x| x.clone())
        }
        pub fn as_ptr(&self) -> *mut obs_scene_t {
            self.scene.0
        }
    }
}
#[cfg(feature = "bootstrapper")]
pub mod bootstrap {
    use std::{env, path::PathBuf, process};
    use anyhow::Context;
    use async_stream::stream;
    use async_trait::async_trait;
    use download::DownloadStatus;
    use extract::ExtractStatus;
    use futures_core::Stream;
    use futures_util::{pin_mut, StreamExt};
    use lazy_static::lazy_static;
    use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
    use tokio::{fs::File, io::AsyncWriteExt, process::Command};
    use crate::context::ObsContext;
    mod download {
        use std::{env::temp_dir, path::PathBuf};
        use anyhow::Context;
        use async_stream::stream;
        use futures_core::Stream;
        use futures_util::StreamExt;
        use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER};
        use semver::Version;
        use sha2::{Digest, Sha256};
        use tokio::{fs::File, io::AsyncWriteExt};
        use uuid::Uuid;
        use super::{github_types, LIBRARY_OBS_VERSION};
        pub enum DownloadStatus {
            Error(anyhow::Error),
            Progress(f32, String),
            Done(PathBuf),
        }
        pub(crate) async fn download_obs(
            repo: &str,
        ) -> anyhow::Result<impl Stream<Item = DownloadStatus>> {
            let client = reqwest::ClientBuilder::new()
                .user_agent("clipture-rs")
                .build()?;
            let releases_url = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("https://api.github.com/repos/{0}/releases", repo),
                );
                res
            });
            let releases: github_types::Root = client
                .get(&releases_url)
                .send()
                .await?
                .json()
                .await?;
            let mut possible_versions = ::alloc::vec::Vec::new();
            for release in releases {
                let tag = release.tag_name.replace("obs-build-", "");
                let version = Version::parse(&tag).context("Parsing version")?;
                if version.major == LIBOBS_API_MAJOR_VER as u64
                    && version.minor == LIBOBS_API_MINOR_VER as u64
                {
                    possible_versions.push(release);
                }
            }
            let latest_version = possible_versions
                .iter()
                .max_by_key(|r| &r.published_at)
                .context(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Finding a matching obs version for {0}",
                                *LIBRARY_OBS_VERSION,
                            ),
                        );
                        res
                    }),
                )?;
            let archive_url = latest_version
                .assets
                .iter()
                .find(|a| a.name.ends_with(".7z"))
                .context("Finding 7z asset")?
                .browser_download_url
                .clone();
            let hash_url = latest_version
                .assets
                .iter()
                .find(|a| a.name.ends_with(".sha256"))
                .context("Finding sha256 asset")?
                .browser_download_url
                .clone();
            let res = client.get(archive_url).send().await?;
            let length = res.content_length().unwrap_or(0);
            let mut bytes_stream = res.bytes_stream();
            let path = PathBuf::new()
                .join(temp_dir())
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.7z", Uuid::new_v4()),
                        );
                        res
                    }),
                );
            let mut tmp_file = File::create_new(&path)
                .await
                .context("Creating temporary file")?;
            let mut curr_len = 0;
            let mut hasher = Sha256::new();
            Ok({
                let (mut __yield_tx, __yield_rx) = unsafe {
                    ::async_stream::__private::yielder::pair()
                };
                ::async_stream::__private::AsyncStream::new(
                    __yield_rx,
                    async move {
                        '__async_stream_private_check_scope: {
                            {
                                #[allow(unreachable_code)]
                                if false {
                                    break '__async_stream_private_check_scope (loop {});
                                }
                                __yield_tx
                                    .send(
                                        DownloadStatus::Progress(0.0, "Downloading OBS".to_string()),
                                    )
                                    .await
                            };
                            while let Some(chunk) = bytes_stream.next().await {
                                let chunk = chunk.context("Retrieving data from stream");
                                if let Err(e) = chunk {
                                    {
                                        #[allow(unreachable_code)]
                                        if false {
                                            break '__async_stream_private_check_scope (loop {});
                                        }
                                        __yield_tx.send(DownloadStatus::Error(e)).await
                                    };
                                    return;
                                }
                                let chunk = chunk.unwrap();
                                hasher.update(&chunk);
                                let r = tmp_file
                                    .write_all(&chunk)
                                    .await
                                    .context("Writing to temporary file");
                                if let Err(e) = r {
                                    {
                                        #[allow(unreachable_code)]
                                        if false {
                                            break '__async_stream_private_check_scope (loop {});
                                        }
                                        __yield_tx.send(DownloadStatus::Error(e)).await
                                    };
                                    return;
                                }
                                curr_len = std::cmp::min(
                                    curr_len + chunk.len() as u64,
                                    length,
                                );
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx
                                        .send(
                                            DownloadStatus::Progress(
                                                curr_len as f32 / length as f32,
                                                "Downloading OBS".to_string(),
                                            ),
                                        )
                                        .await
                                };
                            }
                            let remote_hash = client
                                .get(hash_url)
                                .send()
                                .await
                                .context("Fetching hash");
                            if let Err(e) = remote_hash {
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx.send(DownloadStatus::Error(e)).await
                                };
                                return;
                            }
                            let remote_hash = remote_hash
                                .unwrap()
                                .text()
                                .await
                                .context("Reading hash");
                            if let Err(e) = remote_hash {
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx.send(DownloadStatus::Error(e)).await
                                };
                                return;
                            }
                            let remote_hash = remote_hash.unwrap();
                            let remote_hash = hex::decode(remote_hash.trim())
                                .context("Decoding hash");
                            if let Err(e) = remote_hash {
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx.send(DownloadStatus::Error(e)).await
                                };
                                return;
                            }
                            let remote_hash = remote_hash.unwrap();
                            let local_hash = hasher.finalize();
                            if local_hash.as_slice() != remote_hash {
                                {
                                    #[allow(unreachable_code)]
                                    if false {
                                        break '__async_stream_private_check_scope (loop {});
                                    }
                                    __yield_tx
                                        .send(
                                            DownloadStatus::Error(
                                                ::anyhow::__private::must_use({
                                                    let error = ::anyhow::__private::format_err(
                                                        format_args!("Hash mismatch"),
                                                    );
                                                    error
                                                }),
                                            ),
                                        )
                                        .await
                                };
                                return;
                            }
                            {
                                {
                                    let lvl = ::log::Level::Info;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            { ::log::__private_api::GlobalLogger },
                                            format_args!("Hashes match"),
                                            lvl,
                                            &(
                                                "libobs_wrapper::bootstrap::download",
                                                "libobs_wrapper::bootstrap::download",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            };
                            {
                                #[allow(unreachable_code)]
                                if false {
                                    break '__async_stream_private_check_scope (loop {});
                                }
                                __yield_tx.send(DownloadStatus::Done(path)).await
                            };
                        }
                    },
                )
            })
        }
    }
    mod extract {
        use std::{env::current_exe, path::{Path, PathBuf}};
        use async_stream::stream;
        use futures_core::Stream;
        use futures_util::{pin_mut, StreamExt};
        use sevenz_rust::{default_entry_extract_fn, Password, SevenZReader};
        use tokio::task;
        pub enum ExtractStatus {
            Error(anyhow::Error),
            Progress(f32, String),
        }
        pub(crate) async fn extract_obs(
            archive_file: &Path,
        ) -> anyhow::Result<impl Stream<Item = ExtractStatus>> {
            {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!(
                                "Extracting OBS at {0}",
                                archive_file.display(),
                            ),
                            lvl,
                            &(
                                "libobs_wrapper::bootstrap::extract",
                                "libobs_wrapper::bootstrap::extract",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                }
            };
            let path = PathBuf::from(archive_file);
            let destination = current_exe()?;
            let destination = destination
                .parent()
                .ok_or_else(|| ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Should be able to get parent of exe"),
                    );
                    error
                }))?
                .join("obs_new");
            let dest = destination.clone();
            let stream = {
                let (mut __yield_tx, __yield_rx) = unsafe {
                    ::async_stream::__private::yielder::pair()
                };
                ::async_stream::__private::AsyncStream::new(
                    __yield_rx,
                    async move {
                        '__async_stream_private_check_scope: {
                            {
                                #[allow(unreachable_code)]
                                if false {
                                    break '__async_stream_private_check_scope (loop {});
                                }
                                __yield_tx
                                    .send(Ok((0.0, "Reading file...".to_string())))
                                    .await
                            };
                            let mut sz = match SevenZReader::open(
                                &path,
                                Password::empty(),
                            ) {
                                ::core::result::Result::Ok(v) => v,
                                ::core::result::Result::Err(e) => {
                                    __yield_tx
                                        .send(::core::result::Result::Err(e.into()))
                                        .await;
                                    return;
                                }
                            };
                            let (tx, mut rx) = tokio::sync::mpsc::channel(5);
                            let total = sz.archive().files.len() as f32;
                            if !dest.exists() {
                                match std::fs::create_dir_all(&dest) {
                                    ::core::result::Result::Ok(v) => v,
                                    ::core::result::Result::Err(e) => {
                                        __yield_tx
                                            .send(::core::result::Result::Err(e.into()))
                                            .await;
                                        return;
                                    }
                                };
                            }
                            let mut curr = 0;
                            let mut r = task::spawn_blocking(move || {
                                sz.for_each_entries(|entry, reader| {
                                    curr += 1;
                                    tx.blocking_send((
                                            curr as f32 / total,
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Extracting {0}", entry.name()),
                                                );
                                                res
                                            }),
                                        ))
                                        .unwrap();
                                    let dest_path = dest.join(entry.name());
                                    default_entry_extract_fn(entry, reader, &dest_path)
                                })?;
                                Result::<
                                    _,
                                    anyhow::Error,
                                >::Ok((1.0, "Extraction done".to_string()))
                            });
                            loop {
                                {
                                    #[doc(hidden)]
                                    mod __tokio_select_util {
                                        pub(super) enum Out<_0, _1> {
                                            _0(_0),
                                            _1(_1),
                                            Disabled,
                                        }
                                        pub(super) type Mask = u8;
                                    }
                                    use ::tokio::macros::support::Future;
                                    use ::tokio::macros::support::Pin;
                                    use ::tokio::macros::support::Poll::{Ready, Pending};
                                    const BRANCHES: u32 = 2;
                                    let mut disabled: __tokio_select_util::Mask = Default::default();
                                    if !true {
                                        let mask: __tokio_select_util::Mask = 1 << 0;
                                        disabled |= mask;
                                    }
                                    if !true {
                                        let mask: __tokio_select_util::Mask = 1 << 1;
                                        disabled |= mask;
                                    }
                                    let mut output = {
                                        let futures_init = (rx.recv(), &mut r);
                                        let mut futures = (
                                            ::tokio::macros::support::IntoFuture::into_future(
                                                futures_init.0,
                                            ),
                                            ::tokio::macros::support::IntoFuture::into_future(
                                                futures_init.1,
                                            ),
                                        );
                                        let mut futures = &mut futures;
                                        ::tokio::macros::support::poll_fn(|cx| {
                                                match ::tokio::macros::support::poll_budget_available(cx) {
                                                    ::core::task::Poll::Ready(t) => t,
                                                    ::core::task::Poll::Pending => {
                                                        return ::core::task::Poll::Pending;
                                                    }
                                                };
                                                let mut is_pending = false;
                                                let start = {
                                                    ::tokio::macros::support::thread_rng_n(BRANCHES)
                                                };
                                                for i in 0..BRANCHES {
                                                    let branch;
                                                    #[allow(clippy::modulo_one)]
                                                    {
                                                        branch = (start + i) % BRANCHES;
                                                    }
                                                    match branch {
                                                        #[allow(unreachable_code)]
                                                        0 => {
                                                            let mask = 1 << branch;
                                                            if disabled & mask == mask {
                                                                continue;
                                                            }
                                                            let (fut, ..) = &mut *futures;
                                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                            let out = match Future::poll(fut, cx) {
                                                                Ready(out) => out,
                                                                Pending => {
                                                                    is_pending = true;
                                                                    continue;
                                                                }
                                                            };
                                                            disabled |= mask;
                                                            #[allow(unused_variables)] #[allow(unused_mut)]
                                                            match &out {
                                                                m => {}
                                                                _ => continue,
                                                            }
                                                            return Ready(__tokio_select_util::Out::_0(out));
                                                        }
                                                        #[allow(unreachable_code)]
                                                        1 => {
                                                            let mask = 1 << branch;
                                                            if disabled & mask == mask {
                                                                continue;
                                                            }
                                                            let (_, fut, ..) = &mut *futures;
                                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                            let out = match Future::poll(fut, cx) {
                                                                Ready(out) => out,
                                                                Pending => {
                                                                    is_pending = true;
                                                                    continue;
                                                                }
                                                            };
                                                            disabled |= mask;
                                                            #[allow(unused_variables)] #[allow(unused_mut)]
                                                            match &out {
                                                                res => {}
                                                                _ => continue,
                                                            }
                                                            return Ready(__tokio_select_util::Out::_1(out));
                                                        }
                                                        _ => {
                                                            ::core::panicking::panic_fmt(
                                                                format_args!(
                                                                    "internal error: entered unreachable code: {0}",
                                                                    format_args!(
                                                                        "reaching this means there probably is an off by one bug",
                                                                    ),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                                if is_pending {
                                                    Pending
                                                } else {
                                                    Ready(__tokio_select_util::Out::Disabled)
                                                }
                                            })
                                            .await
                                    };
                                    match output {
                                        __tokio_select_util::Out::_0(m) => {
                                            match m {
                                                Some(e) => {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx.send(Ok(e)).await
                                                }
                                                None => break,
                                            }
                                        }
                                        __tokio_select_util::Out::_1(res) => {
                                            match res {
                                                Ok(e) => {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx.send(e).await
                                                }
                                                Err(e) => {
                                                    {
                                                        #[allow(unreachable_code)]
                                                        if false {
                                                            break '__async_stream_private_check_scope (loop {});
                                                        }
                                                        __yield_tx.send(Err(e.into())).await
                                                    };
                                                }
                                            }
                                            break;
                                        }
                                        __tokio_select_util::Out::Disabled => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "all branches are disabled and there is no else branch",
                                                ),
                                            );
                                        }
                                        _ => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "internal error: entered unreachable code: {0}",
                                                    format_args!("failed to match bind"),
                                                ),
                                            );
                                        }
                                    }
                                };
                            }
                            {
                                #[allow(unreachable_code)]
                                if false {
                                    break '__async_stream_private_check_scope (loop {});
                                }
                                __yield_tx
                                    .send(Ok((1.0, "Extraction done".to_string())))
                                    .await
                            };
                        }
                    },
                )
            };
            Ok({
                let (mut __yield_tx, __yield_rx) = unsafe {
                    ::async_stream::__private::yielder::pair()
                };
                ::async_stream::__private::AsyncStream::new(
                    __yield_rx,
                    async move {
                        '__async_stream_private_check_scope: {
                            let mut stream = stream;
                            #[allow(unused_mut)]
                            let mut stream = unsafe {
                                ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                    &mut stream,
                                )
                            };
                            while let Some(status) = stream.next().await {
                                match status {
                                    Ok(e) => {
                                        #[allow(unreachable_code)]
                                        if false {
                                            break '__async_stream_private_check_scope (loop {});
                                        }
                                        __yield_tx.send(ExtractStatus::Progress(e.0, e.1)).await
                                    }
                                    Err(err) => {
                                        {
                                            {
                                                let lvl = ::log::Level::Error;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api::log(
                                                        { ::log::__private_api::GlobalLogger },
                                                        format_args!("Error extracting OBS: {0:?}", err),
                                                        lvl,
                                                        &(
                                                            "libobs_wrapper::bootstrap::extract",
                                                            "libobs_wrapper::bootstrap::extract",
                                                            ::log::__private_api::loc(),
                                                        ),
                                                        (),
                                                    );
                                                }
                                            }
                                        };
                                        {
                                            #[allow(unreachable_code)]
                                            if false {
                                                break '__async_stream_private_check_scope (loop {});
                                            }
                                            __yield_tx.send(ExtractStatus::Error(err)).await
                                        };
                                        return;
                                    }
                                }
                            }
                        }
                    },
                )
            })
        }
    }
    mod github_types {
        use serde::{Deserialize, Serialize};
        pub type Root = Vec<Root2>;
        #[serde(rename_all = "camelCase")]
        pub struct Root2 {
            pub url: String,
            #[serde(rename = "assets_url")]
            pub assets_url: String,
            #[serde(rename = "upload_url")]
            pub upload_url: String,
            #[serde(rename = "html_url")]
            pub html_url: String,
            pub id: i64,
            pub author: Author,
            #[serde(rename = "node_id")]
            pub node_id: String,
            #[serde(rename = "tag_name")]
            pub tag_name: String,
            #[serde(rename = "target_commitish")]
            pub target_commitish: String,
            pub name: String,
            pub draft: bool,
            pub prerelease: bool,
            #[serde(rename = "created_at")]
            pub created_at: String,
            #[serde(rename = "published_at")]
            pub published_at: String,
            pub assets: Vec<Asset>,
            #[serde(rename = "tarball_url")]
            pub tarball_url: String,
            #[serde(rename = "zipball_url")]
            pub zipball_url: String,
            pub body: String,
        }
        #[automatically_derived]
        impl ::core::default::Default for Root2 {
            #[inline]
            fn default() -> Root2 {
                Root2 {
                    url: ::core::default::Default::default(),
                    assets_url: ::core::default::Default::default(),
                    upload_url: ::core::default::Default::default(),
                    html_url: ::core::default::Default::default(),
                    id: ::core::default::Default::default(),
                    author: ::core::default::Default::default(),
                    node_id: ::core::default::Default::default(),
                    tag_name: ::core::default::Default::default(),
                    target_commitish: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    draft: ::core::default::Default::default(),
                    prerelease: ::core::default::Default::default(),
                    created_at: ::core::default::Default::default(),
                    published_at: ::core::default::Default::default(),
                    assets: ::core::default::Default::default(),
                    tarball_url: ::core::default::Default::default(),
                    zipball_url: ::core::default::Default::default(),
                    body: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Root2 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "url",
                    "assets_url",
                    "upload_url",
                    "html_url",
                    "id",
                    "author",
                    "node_id",
                    "tag_name",
                    "target_commitish",
                    "name",
                    "draft",
                    "prerelease",
                    "created_at",
                    "published_at",
                    "assets",
                    "tarball_url",
                    "zipball_url",
                    "body",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.url,
                    &self.assets_url,
                    &self.upload_url,
                    &self.html_url,
                    &self.id,
                    &self.author,
                    &self.node_id,
                    &self.tag_name,
                    &self.target_commitish,
                    &self.name,
                    &self.draft,
                    &self.prerelease,
                    &self.created_at,
                    &self.published_at,
                    &self.assets,
                    &self.tarball_url,
                    &self.zipball_url,
                    &&self.body,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Root2",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Root2 {
            #[inline]
            fn clone(&self) -> Root2 {
                Root2 {
                    url: ::core::clone::Clone::clone(&self.url),
                    assets_url: ::core::clone::Clone::clone(&self.assets_url),
                    upload_url: ::core::clone::Clone::clone(&self.upload_url),
                    html_url: ::core::clone::Clone::clone(&self.html_url),
                    id: ::core::clone::Clone::clone(&self.id),
                    author: ::core::clone::Clone::clone(&self.author),
                    node_id: ::core::clone::Clone::clone(&self.node_id),
                    tag_name: ::core::clone::Clone::clone(&self.tag_name),
                    target_commitish: ::core::clone::Clone::clone(
                        &self.target_commitish,
                    ),
                    name: ::core::clone::Clone::clone(&self.name),
                    draft: ::core::clone::Clone::clone(&self.draft),
                    prerelease: ::core::clone::Clone::clone(&self.prerelease),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    published_at: ::core::clone::Clone::clone(&self.published_at),
                    assets: ::core::clone::Clone::clone(&self.assets),
                    tarball_url: ::core::clone::Clone::clone(&self.tarball_url),
                    zipball_url: ::core::clone::Clone::clone(&self.zipball_url),
                    body: ::core::clone::Clone::clone(&self.body),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Root2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Root2 {
            #[inline]
            fn eq(&self, other: &Root2) -> bool {
                self.url == other.url && self.assets_url == other.assets_url
                    && self.upload_url == other.upload_url
                    && self.html_url == other.html_url && self.id == other.id
                    && self.author == other.author && self.node_id == other.node_id
                    && self.tag_name == other.tag_name
                    && self.target_commitish == other.target_commitish
                    && self.name == other.name && self.draft == other.draft
                    && self.prerelease == other.prerelease
                    && self.created_at == other.created_at
                    && self.published_at == other.published_at
                    && self.assets == other.assets
                    && self.tarball_url == other.tarball_url
                    && self.zipball_url == other.zipball_url && self.body == other.body
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Root2 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Root2",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "assets_url",
                        &self.assets_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "upload_url",
                        &self.upload_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "html_url",
                        &self.html_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "author",
                        &self.author,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "node_id",
                        &self.node_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tag_name",
                        &self.tag_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "target_commitish",
                        &self.target_commitish,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "draft",
                        &self.draft,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "prerelease",
                        &self.prerelease,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_at",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "published_at",
                        &self.published_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "assets",
                        &self.assets,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tarball_url",
                        &self.tarball_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "zipball_url",
                        &self.zipball_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "body",
                        &self.body,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Root2 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::__private::Ok(__Field::__field0),
                                "assets_url" => _serde::__private::Ok(__Field::__field1),
                                "upload_url" => _serde::__private::Ok(__Field::__field2),
                                "html_url" => _serde::__private::Ok(__Field::__field3),
                                "id" => _serde::__private::Ok(__Field::__field4),
                                "author" => _serde::__private::Ok(__Field::__field5),
                                "node_id" => _serde::__private::Ok(__Field::__field6),
                                "tag_name" => _serde::__private::Ok(__Field::__field7),
                                "target_commitish" => {
                                    _serde::__private::Ok(__Field::__field8)
                                }
                                "name" => _serde::__private::Ok(__Field::__field9),
                                "draft" => _serde::__private::Ok(__Field::__field10),
                                "prerelease" => _serde::__private::Ok(__Field::__field11),
                                "created_at" => _serde::__private::Ok(__Field::__field12),
                                "published_at" => _serde::__private::Ok(__Field::__field13),
                                "assets" => _serde::__private::Ok(__Field::__field14),
                                "tarball_url" => _serde::__private::Ok(__Field::__field15),
                                "zipball_url" => _serde::__private::Ok(__Field::__field16),
                                "body" => _serde::__private::Ok(__Field::__field17),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::__private::Ok(__Field::__field0),
                                b"assets_url" => _serde::__private::Ok(__Field::__field1),
                                b"upload_url" => _serde::__private::Ok(__Field::__field2),
                                b"html_url" => _serde::__private::Ok(__Field::__field3),
                                b"id" => _serde::__private::Ok(__Field::__field4),
                                b"author" => _serde::__private::Ok(__Field::__field5),
                                b"node_id" => _serde::__private::Ok(__Field::__field6),
                                b"tag_name" => _serde::__private::Ok(__Field::__field7),
                                b"target_commitish" => {
                                    _serde::__private::Ok(__Field::__field8)
                                }
                                b"name" => _serde::__private::Ok(__Field::__field9),
                                b"draft" => _serde::__private::Ok(__Field::__field10),
                                b"prerelease" => _serde::__private::Ok(__Field::__field11),
                                b"created_at" => _serde::__private::Ok(__Field::__field12),
                                b"published_at" => _serde::__private::Ok(__Field::__field13),
                                b"assets" => _serde::__private::Ok(__Field::__field14),
                                b"tarball_url" => _serde::__private::Ok(__Field::__field15),
                                b"zipball_url" => _serde::__private::Ok(__Field::__field16),
                                b"body" => _serde::__private::Ok(__Field::__field17),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Root2>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Root2;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Root2",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Author,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match _serde::de::SeqAccess::next_element::<
                                Vec<Asset>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct Root2 with 18 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Root2 {
                                url: __field0,
                                assets_url: __field1,
                                upload_url: __field2,
                                html_url: __field3,
                                id: __field4,
                                author: __field5,
                                node_id: __field6,
                                tag_name: __field7,
                                target_commitish: __field8,
                                name: __field9,
                                draft: __field10,
                                prerelease: __field11,
                                created_at: __field12,
                                published_at: __field13,
                                assets: __field14,
                                tarball_url: __field15,
                                zipball_url: __field16,
                                body: __field17,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Author> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<Vec<Asset>> = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "assets_url",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "upload_url",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "html_url",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("author"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Author>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "node_id",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "tag_name",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "target_commitish",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("draft"),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "prerelease",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "created_at",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "published_at",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("assets"),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Vec<Asset>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "tarball_url",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "zipball_url",
                                                ),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("assets_url")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("upload_url")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("html_url")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("author")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("node_id")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("tag_name")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("target_commitish")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("draft")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("prerelease")?
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("created_at")?
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("published_at")?
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("assets")?
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("tarball_url")?
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("zipball_url")?
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("body")?
                                }
                            };
                            _serde::__private::Ok(Root2 {
                                url: __field0,
                                assets_url: __field1,
                                upload_url: __field2,
                                html_url: __field3,
                                id: __field4,
                                author: __field5,
                                node_id: __field6,
                                tag_name: __field7,
                                target_commitish: __field8,
                                name: __field9,
                                draft: __field10,
                                prerelease: __field11,
                                created_at: __field12,
                                published_at: __field13,
                                assets: __field14,
                                tarball_url: __field15,
                                zipball_url: __field16,
                                body: __field17,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "url",
                        "assets_url",
                        "upload_url",
                        "html_url",
                        "id",
                        "author",
                        "node_id",
                        "tag_name",
                        "target_commitish",
                        "name",
                        "draft",
                        "prerelease",
                        "created_at",
                        "published_at",
                        "assets",
                        "tarball_url",
                        "zipball_url",
                        "body",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Root2",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Root2>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(rename_all = "camelCase")]
        pub struct Author {
            pub login: String,
            pub id: i64,
            #[serde(rename = "node_id")]
            pub node_id: String,
            #[serde(rename = "avatar_url")]
            pub avatar_url: String,
            #[serde(rename = "gravatar_id")]
            pub gravatar_id: String,
            pub url: String,
            #[serde(rename = "html_url")]
            pub html_url: String,
            #[serde(rename = "followers_url")]
            pub followers_url: String,
            #[serde(rename = "following_url")]
            pub following_url: String,
            #[serde(rename = "gists_url")]
            pub gists_url: String,
            #[serde(rename = "starred_url")]
            pub starred_url: String,
            #[serde(rename = "subscriptions_url")]
            pub subscriptions_url: String,
            #[serde(rename = "organizations_url")]
            pub organizations_url: String,
            #[serde(rename = "repos_url")]
            pub repos_url: String,
            #[serde(rename = "events_url")]
            pub events_url: String,
            #[serde(rename = "received_events_url")]
            pub received_events_url: String,
            #[serde(rename = "type")]
            pub type_field: String,
            #[serde(rename = "user_view_type")]
            pub user_view_type: String,
            #[serde(rename = "site_admin")]
            pub site_admin: bool,
        }
        #[automatically_derived]
        impl ::core::default::Default for Author {
            #[inline]
            fn default() -> Author {
                Author {
                    login: ::core::default::Default::default(),
                    id: ::core::default::Default::default(),
                    node_id: ::core::default::Default::default(),
                    avatar_url: ::core::default::Default::default(),
                    gravatar_id: ::core::default::Default::default(),
                    url: ::core::default::Default::default(),
                    html_url: ::core::default::Default::default(),
                    followers_url: ::core::default::Default::default(),
                    following_url: ::core::default::Default::default(),
                    gists_url: ::core::default::Default::default(),
                    starred_url: ::core::default::Default::default(),
                    subscriptions_url: ::core::default::Default::default(),
                    organizations_url: ::core::default::Default::default(),
                    repos_url: ::core::default::Default::default(),
                    events_url: ::core::default::Default::default(),
                    received_events_url: ::core::default::Default::default(),
                    type_field: ::core::default::Default::default(),
                    user_view_type: ::core::default::Default::default(),
                    site_admin: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Author {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "login",
                    "id",
                    "node_id",
                    "avatar_url",
                    "gravatar_id",
                    "url",
                    "html_url",
                    "followers_url",
                    "following_url",
                    "gists_url",
                    "starred_url",
                    "subscriptions_url",
                    "organizations_url",
                    "repos_url",
                    "events_url",
                    "received_events_url",
                    "type_field",
                    "user_view_type",
                    "site_admin",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.login,
                    &self.id,
                    &self.node_id,
                    &self.avatar_url,
                    &self.gravatar_id,
                    &self.url,
                    &self.html_url,
                    &self.followers_url,
                    &self.following_url,
                    &self.gists_url,
                    &self.starred_url,
                    &self.subscriptions_url,
                    &self.organizations_url,
                    &self.repos_url,
                    &self.events_url,
                    &self.received_events_url,
                    &self.type_field,
                    &self.user_view_type,
                    &&self.site_admin,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Author",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Author {
            #[inline]
            fn clone(&self) -> Author {
                Author {
                    login: ::core::clone::Clone::clone(&self.login),
                    id: ::core::clone::Clone::clone(&self.id),
                    node_id: ::core::clone::Clone::clone(&self.node_id),
                    avatar_url: ::core::clone::Clone::clone(&self.avatar_url),
                    gravatar_id: ::core::clone::Clone::clone(&self.gravatar_id),
                    url: ::core::clone::Clone::clone(&self.url),
                    html_url: ::core::clone::Clone::clone(&self.html_url),
                    followers_url: ::core::clone::Clone::clone(&self.followers_url),
                    following_url: ::core::clone::Clone::clone(&self.following_url),
                    gists_url: ::core::clone::Clone::clone(&self.gists_url),
                    starred_url: ::core::clone::Clone::clone(&self.starred_url),
                    subscriptions_url: ::core::clone::Clone::clone(
                        &self.subscriptions_url,
                    ),
                    organizations_url: ::core::clone::Clone::clone(
                        &self.organizations_url,
                    ),
                    repos_url: ::core::clone::Clone::clone(&self.repos_url),
                    events_url: ::core::clone::Clone::clone(&self.events_url),
                    received_events_url: ::core::clone::Clone::clone(
                        &self.received_events_url,
                    ),
                    type_field: ::core::clone::Clone::clone(&self.type_field),
                    user_view_type: ::core::clone::Clone::clone(&self.user_view_type),
                    site_admin: ::core::clone::Clone::clone(&self.site_admin),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Author {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Author {
            #[inline]
            fn eq(&self, other: &Author) -> bool {
                self.login == other.login && self.id == other.id
                    && self.node_id == other.node_id
                    && self.avatar_url == other.avatar_url
                    && self.gravatar_id == other.gravatar_id && self.url == other.url
                    && self.html_url == other.html_url
                    && self.followers_url == other.followers_url
                    && self.following_url == other.following_url
                    && self.gists_url == other.gists_url
                    && self.starred_url == other.starred_url
                    && self.subscriptions_url == other.subscriptions_url
                    && self.organizations_url == other.organizations_url
                    && self.repos_url == other.repos_url
                    && self.events_url == other.events_url
                    && self.received_events_url == other.received_events_url
                    && self.type_field == other.type_field
                    && self.user_view_type == other.user_view_type
                    && self.site_admin == other.site_admin
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Author {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Author",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "login",
                        &self.login,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "node_id",
                        &self.node_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatar_url",
                        &self.avatar_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gravatar_id",
                        &self.gravatar_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "html_url",
                        &self.html_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "followers_url",
                        &self.followers_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "following_url",
                        &self.following_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gists_url",
                        &self.gists_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "starred_url",
                        &self.starred_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "subscriptions_url",
                        &self.subscriptions_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "organizations_url",
                        &self.organizations_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "repos_url",
                        &self.repos_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "events_url",
                        &self.events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "received_events_url",
                        &self.received_events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.type_field,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_view_type",
                        &self.user_view_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "site_admin",
                        &self.site_admin,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Author {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "login" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
                                "node_id" => _serde::__private::Ok(__Field::__field2),
                                "avatar_url" => _serde::__private::Ok(__Field::__field3),
                                "gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                "url" => _serde::__private::Ok(__Field::__field5),
                                "html_url" => _serde::__private::Ok(__Field::__field6),
                                "followers_url" => _serde::__private::Ok(__Field::__field7),
                                "following_url" => _serde::__private::Ok(__Field::__field8),
                                "gists_url" => _serde::__private::Ok(__Field::__field9),
                                "starred_url" => _serde::__private::Ok(__Field::__field10),
                                "subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                "organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                "repos_url" => _serde::__private::Ok(__Field::__field13),
                                "events_url" => _serde::__private::Ok(__Field::__field14),
                                "received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                "type" => _serde::__private::Ok(__Field::__field16),
                                "user_view_type" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                "site_admin" => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"login" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
                                b"node_id" => _serde::__private::Ok(__Field::__field2),
                                b"avatar_url" => _serde::__private::Ok(__Field::__field3),
                                b"gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                b"url" => _serde::__private::Ok(__Field::__field5),
                                b"html_url" => _serde::__private::Ok(__Field::__field6),
                                b"followers_url" => _serde::__private::Ok(__Field::__field7),
                                b"following_url" => _serde::__private::Ok(__Field::__field8),
                                b"gists_url" => _serde::__private::Ok(__Field::__field9),
                                b"starred_url" => _serde::__private::Ok(__Field::__field10),
                                b"subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                b"organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                b"repos_url" => _serde::__private::Ok(__Field::__field13),
                                b"events_url" => _serde::__private::Ok(__Field::__field14),
                                b"received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                b"type" => _serde::__private::Ok(__Field::__field16),
                                b"user_view_type" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                b"site_admin" => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Author>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Author;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Author",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct Author with 19 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Author {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                type_field: __field16,
                                user_view_type: __field17,
                                site_admin: __field18,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<bool> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("login"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "node_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "avatar_url",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gravatar_id",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "html_url",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "followers_url",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "following_url",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gists_url",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "starred_url",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "subscriptions_url",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "organizations_url",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "repos_url",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "events_url",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "received_events_url",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "user_view_type",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "site_admin",
                                                ),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("login")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("node_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("avatar_url")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gravatar_id")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("html_url")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("followers_url")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("following_url")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gists_url")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("starred_url")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("subscriptions_url")?
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("organizations_url")?
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("repos_url")?
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("events_url")?
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("received_events_url")?
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("type")?
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("user_view_type")?
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("site_admin")?
                                }
                            };
                            _serde::__private::Ok(Author {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                type_field: __field16,
                                user_view_type: __field17,
                                site_admin: __field18,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "login",
                        "id",
                        "node_id",
                        "avatar_url",
                        "gravatar_id",
                        "url",
                        "html_url",
                        "followers_url",
                        "following_url",
                        "gists_url",
                        "starred_url",
                        "subscriptions_url",
                        "organizations_url",
                        "repos_url",
                        "events_url",
                        "received_events_url",
                        "type",
                        "user_view_type",
                        "site_admin",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Author",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Author>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(rename_all = "camelCase")]
        pub struct Asset {
            pub url: String,
            pub id: i64,
            #[serde(rename = "node_id")]
            pub node_id: String,
            pub name: String,
            pub label: String,
            pub uploader: Uploader,
            #[serde(rename = "content_type")]
            pub content_type: String,
            pub state: String,
            pub size: i64,
            #[serde(rename = "download_count")]
            pub download_count: i64,
            #[serde(rename = "created_at")]
            pub created_at: String,
            #[serde(rename = "updated_at")]
            pub updated_at: String,
            #[serde(rename = "browser_download_url")]
            pub browser_download_url: String,
        }
        #[automatically_derived]
        impl ::core::default::Default for Asset {
            #[inline]
            fn default() -> Asset {
                Asset {
                    url: ::core::default::Default::default(),
                    id: ::core::default::Default::default(),
                    node_id: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    label: ::core::default::Default::default(),
                    uploader: ::core::default::Default::default(),
                    content_type: ::core::default::Default::default(),
                    state: ::core::default::Default::default(),
                    size: ::core::default::Default::default(),
                    download_count: ::core::default::Default::default(),
                    created_at: ::core::default::Default::default(),
                    updated_at: ::core::default::Default::default(),
                    browser_download_url: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Asset {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "url",
                    "id",
                    "node_id",
                    "name",
                    "label",
                    "uploader",
                    "content_type",
                    "state",
                    "size",
                    "download_count",
                    "created_at",
                    "updated_at",
                    "browser_download_url",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.url,
                    &self.id,
                    &self.node_id,
                    &self.name,
                    &self.label,
                    &self.uploader,
                    &self.content_type,
                    &self.state,
                    &self.size,
                    &self.download_count,
                    &self.created_at,
                    &self.updated_at,
                    &&self.browser_download_url,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Asset",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Asset {
            #[inline]
            fn clone(&self) -> Asset {
                Asset {
                    url: ::core::clone::Clone::clone(&self.url),
                    id: ::core::clone::Clone::clone(&self.id),
                    node_id: ::core::clone::Clone::clone(&self.node_id),
                    name: ::core::clone::Clone::clone(&self.name),
                    label: ::core::clone::Clone::clone(&self.label),
                    uploader: ::core::clone::Clone::clone(&self.uploader),
                    content_type: ::core::clone::Clone::clone(&self.content_type),
                    state: ::core::clone::Clone::clone(&self.state),
                    size: ::core::clone::Clone::clone(&self.size),
                    download_count: ::core::clone::Clone::clone(&self.download_count),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                    browser_download_url: ::core::clone::Clone::clone(
                        &self.browser_download_url,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Asset {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Asset {
            #[inline]
            fn eq(&self, other: &Asset) -> bool {
                self.url == other.url && self.id == other.id
                    && self.node_id == other.node_id && self.name == other.name
                    && self.label == other.label && self.uploader == other.uploader
                    && self.content_type == other.content_type
                    && self.state == other.state && self.size == other.size
                    && self.download_count == other.download_count
                    && self.created_at == other.created_at
                    && self.updated_at == other.updated_at
                    && self.browser_download_url == other.browser_download_url
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Asset {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Asset",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "node_id",
                        &self.node_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "label",
                        &self.label,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "uploader",
                        &self.uploader,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "content_type",
                        &self.content_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "state",
                        &self.state,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "size",
                        &self.size,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "download_count",
                        &self.download_count,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_at",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updated_at",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "browser_download_url",
                        &self.browser_download_url,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Asset {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
                                "node_id" => _serde::__private::Ok(__Field::__field2),
                                "name" => _serde::__private::Ok(__Field::__field3),
                                "label" => _serde::__private::Ok(__Field::__field4),
                                "uploader" => _serde::__private::Ok(__Field::__field5),
                                "content_type" => _serde::__private::Ok(__Field::__field6),
                                "state" => _serde::__private::Ok(__Field::__field7),
                                "size" => _serde::__private::Ok(__Field::__field8),
                                "download_count" => _serde::__private::Ok(__Field::__field9),
                                "created_at" => _serde::__private::Ok(__Field::__field10),
                                "updated_at" => _serde::__private::Ok(__Field::__field11),
                                "browser_download_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
                                b"node_id" => _serde::__private::Ok(__Field::__field2),
                                b"name" => _serde::__private::Ok(__Field::__field3),
                                b"label" => _serde::__private::Ok(__Field::__field4),
                                b"uploader" => _serde::__private::Ok(__Field::__field5),
                                b"content_type" => _serde::__private::Ok(__Field::__field6),
                                b"state" => _serde::__private::Ok(__Field::__field7),
                                b"size" => _serde::__private::Ok(__Field::__field8),
                                b"download_count" => {
                                    _serde::__private::Ok(__Field::__field9)
                                }
                                b"created_at" => _serde::__private::Ok(__Field::__field10),
                                b"updated_at" => _serde::__private::Ok(__Field::__field11),
                                b"browser_download_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Asset>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Asset;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Asset",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Uploader,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Asset with 13 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Asset {
                                url: __field0,
                                id: __field1,
                                node_id: __field2,
                                name: __field3,
                                label: __field4,
                                uploader: __field5,
                                content_type: __field6,
                                state: __field7,
                                size: __field8,
                                download_count: __field9,
                                created_at: __field10,
                                updated_at: __field11,
                                browser_download_url: __field12,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Uploader> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "node_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("label"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "uploader",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Uploader>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "content_type",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("state"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("size"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "download_count",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "created_at",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updated_at",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "browser_download_url",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("node_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("label")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("uploader")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("content_type")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("state")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("size")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("download_count")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("created_at")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updated_at")?
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "browser_download_url",
                                    )?
                                }
                            };
                            _serde::__private::Ok(Asset {
                                url: __field0,
                                id: __field1,
                                node_id: __field2,
                                name: __field3,
                                label: __field4,
                                uploader: __field5,
                                content_type: __field6,
                                state: __field7,
                                size: __field8,
                                download_count: __field9,
                                created_at: __field10,
                                updated_at: __field11,
                                browser_download_url: __field12,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "url",
                        "id",
                        "node_id",
                        "name",
                        "label",
                        "uploader",
                        "content_type",
                        "state",
                        "size",
                        "download_count",
                        "created_at",
                        "updated_at",
                        "browser_download_url",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Asset",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Asset>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(rename_all = "camelCase")]
        pub struct Uploader {
            pub login: String,
            pub id: i64,
            #[serde(rename = "node_id")]
            pub node_id: String,
            #[serde(rename = "avatar_url")]
            pub avatar_url: String,
            #[serde(rename = "gravatar_id")]
            pub gravatar_id: String,
            pub url: String,
            #[serde(rename = "html_url")]
            pub html_url: String,
            #[serde(rename = "followers_url")]
            pub followers_url: String,
            #[serde(rename = "following_url")]
            pub following_url: String,
            #[serde(rename = "gists_url")]
            pub gists_url: String,
            #[serde(rename = "starred_url")]
            pub starred_url: String,
            #[serde(rename = "subscriptions_url")]
            pub subscriptions_url: String,
            #[serde(rename = "organizations_url")]
            pub organizations_url: String,
            #[serde(rename = "repos_url")]
            pub repos_url: String,
            #[serde(rename = "events_url")]
            pub events_url: String,
            #[serde(rename = "received_events_url")]
            pub received_events_url: String,
            #[serde(rename = "type")]
            pub type_field: String,
            #[serde(rename = "user_view_type")]
            pub user_view_type: String,
            #[serde(rename = "site_admin")]
            pub site_admin: bool,
        }
        #[automatically_derived]
        impl ::core::default::Default for Uploader {
            #[inline]
            fn default() -> Uploader {
                Uploader {
                    login: ::core::default::Default::default(),
                    id: ::core::default::Default::default(),
                    node_id: ::core::default::Default::default(),
                    avatar_url: ::core::default::Default::default(),
                    gravatar_id: ::core::default::Default::default(),
                    url: ::core::default::Default::default(),
                    html_url: ::core::default::Default::default(),
                    followers_url: ::core::default::Default::default(),
                    following_url: ::core::default::Default::default(),
                    gists_url: ::core::default::Default::default(),
                    starred_url: ::core::default::Default::default(),
                    subscriptions_url: ::core::default::Default::default(),
                    organizations_url: ::core::default::Default::default(),
                    repos_url: ::core::default::Default::default(),
                    events_url: ::core::default::Default::default(),
                    received_events_url: ::core::default::Default::default(),
                    type_field: ::core::default::Default::default(),
                    user_view_type: ::core::default::Default::default(),
                    site_admin: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Uploader {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "login",
                    "id",
                    "node_id",
                    "avatar_url",
                    "gravatar_id",
                    "url",
                    "html_url",
                    "followers_url",
                    "following_url",
                    "gists_url",
                    "starred_url",
                    "subscriptions_url",
                    "organizations_url",
                    "repos_url",
                    "events_url",
                    "received_events_url",
                    "type_field",
                    "user_view_type",
                    "site_admin",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.login,
                    &self.id,
                    &self.node_id,
                    &self.avatar_url,
                    &self.gravatar_id,
                    &self.url,
                    &self.html_url,
                    &self.followers_url,
                    &self.following_url,
                    &self.gists_url,
                    &self.starred_url,
                    &self.subscriptions_url,
                    &self.organizations_url,
                    &self.repos_url,
                    &self.events_url,
                    &self.received_events_url,
                    &self.type_field,
                    &self.user_view_type,
                    &&self.site_admin,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Uploader",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Uploader {
            #[inline]
            fn clone(&self) -> Uploader {
                Uploader {
                    login: ::core::clone::Clone::clone(&self.login),
                    id: ::core::clone::Clone::clone(&self.id),
                    node_id: ::core::clone::Clone::clone(&self.node_id),
                    avatar_url: ::core::clone::Clone::clone(&self.avatar_url),
                    gravatar_id: ::core::clone::Clone::clone(&self.gravatar_id),
                    url: ::core::clone::Clone::clone(&self.url),
                    html_url: ::core::clone::Clone::clone(&self.html_url),
                    followers_url: ::core::clone::Clone::clone(&self.followers_url),
                    following_url: ::core::clone::Clone::clone(&self.following_url),
                    gists_url: ::core::clone::Clone::clone(&self.gists_url),
                    starred_url: ::core::clone::Clone::clone(&self.starred_url),
                    subscriptions_url: ::core::clone::Clone::clone(
                        &self.subscriptions_url,
                    ),
                    organizations_url: ::core::clone::Clone::clone(
                        &self.organizations_url,
                    ),
                    repos_url: ::core::clone::Clone::clone(&self.repos_url),
                    events_url: ::core::clone::Clone::clone(&self.events_url),
                    received_events_url: ::core::clone::Clone::clone(
                        &self.received_events_url,
                    ),
                    type_field: ::core::clone::Clone::clone(&self.type_field),
                    user_view_type: ::core::clone::Clone::clone(&self.user_view_type),
                    site_admin: ::core::clone::Clone::clone(&self.site_admin),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Uploader {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Uploader {
            #[inline]
            fn eq(&self, other: &Uploader) -> bool {
                self.login == other.login && self.id == other.id
                    && self.node_id == other.node_id
                    && self.avatar_url == other.avatar_url
                    && self.gravatar_id == other.gravatar_id && self.url == other.url
                    && self.html_url == other.html_url
                    && self.followers_url == other.followers_url
                    && self.following_url == other.following_url
                    && self.gists_url == other.gists_url
                    && self.starred_url == other.starred_url
                    && self.subscriptions_url == other.subscriptions_url
                    && self.organizations_url == other.organizations_url
                    && self.repos_url == other.repos_url
                    && self.events_url == other.events_url
                    && self.received_events_url == other.received_events_url
                    && self.type_field == other.type_field
                    && self.user_view_type == other.user_view_type
                    && self.site_admin == other.site_admin
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Uploader {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Uploader",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "login",
                        &self.login,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "node_id",
                        &self.node_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatar_url",
                        &self.avatar_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gravatar_id",
                        &self.gravatar_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "html_url",
                        &self.html_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "followers_url",
                        &self.followers_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "following_url",
                        &self.following_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gists_url",
                        &self.gists_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "starred_url",
                        &self.starred_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "subscriptions_url",
                        &self.subscriptions_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "organizations_url",
                        &self.organizations_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "repos_url",
                        &self.repos_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "events_url",
                        &self.events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "received_events_url",
                        &self.received_events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.type_field,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_view_type",
                        &self.user_view_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "site_admin",
                        &self.site_admin,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Uploader {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "login" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
                                "node_id" => _serde::__private::Ok(__Field::__field2),
                                "avatar_url" => _serde::__private::Ok(__Field::__field3),
                                "gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                "url" => _serde::__private::Ok(__Field::__field5),
                                "html_url" => _serde::__private::Ok(__Field::__field6),
                                "followers_url" => _serde::__private::Ok(__Field::__field7),
                                "following_url" => _serde::__private::Ok(__Field::__field8),
                                "gists_url" => _serde::__private::Ok(__Field::__field9),
                                "starred_url" => _serde::__private::Ok(__Field::__field10),
                                "subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                "organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                "repos_url" => _serde::__private::Ok(__Field::__field13),
                                "events_url" => _serde::__private::Ok(__Field::__field14),
                                "received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                "type" => _serde::__private::Ok(__Field::__field16),
                                "user_view_type" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                "site_admin" => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"login" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
                                b"node_id" => _serde::__private::Ok(__Field::__field2),
                                b"avatar_url" => _serde::__private::Ok(__Field::__field3),
                                b"gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                b"url" => _serde::__private::Ok(__Field::__field5),
                                b"html_url" => _serde::__private::Ok(__Field::__field6),
                                b"followers_url" => _serde::__private::Ok(__Field::__field7),
                                b"following_url" => _serde::__private::Ok(__Field::__field8),
                                b"gists_url" => _serde::__private::Ok(__Field::__field9),
                                b"starred_url" => _serde::__private::Ok(__Field::__field10),
                                b"subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                b"organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                b"repos_url" => _serde::__private::Ok(__Field::__field13),
                                b"events_url" => _serde::__private::Ok(__Field::__field14),
                                b"received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                b"type" => _serde::__private::Ok(__Field::__field16),
                                b"user_view_type" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                b"site_admin" => _serde::__private::Ok(__Field::__field18),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Uploader>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Uploader;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Uploader",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct Uploader with 19 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Uploader {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                type_field: __field16,
                                user_view_type: __field17,
                                site_admin: __field18,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<bool> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("login"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "node_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "avatar_url",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gravatar_id",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "html_url",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "followers_url",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "following_url",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gists_url",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "starred_url",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "subscriptions_url",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "organizations_url",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "repos_url",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "events_url",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "received_events_url",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "user_view_type",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "site_admin",
                                                ),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("login")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("node_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("avatar_url")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gravatar_id")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("html_url")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("followers_url")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("following_url")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gists_url")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("starred_url")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("subscriptions_url")?
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("organizations_url")?
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("repos_url")?
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("events_url")?
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("received_events_url")?
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("type")?
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("user_view_type")?
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("site_admin")?
                                }
                            };
                            _serde::__private::Ok(Uploader {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                type_field: __field16,
                                user_view_type: __field17,
                                site_admin: __field18,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "login",
                        "id",
                        "node_id",
                        "avatar_url",
                        "gravatar_id",
                        "url",
                        "html_url",
                        "followers_url",
                        "following_url",
                        "gists_url",
                        "starred_url",
                        "subscriptions_url",
                        "organizations_url",
                        "repos_url",
                        "events_url",
                        "received_events_url",
                        "type",
                        "user_view_type",
                        "site_admin",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Uploader",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Uploader>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    mod options {
        pub const GITHUB_REPO: &'static str = "sshcrack/libobs-builds";
        pub struct ObsDownloaderOptions {
            pub(crate) repository: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsDownloaderOptions {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ObsDownloaderOptions",
                    "repository",
                    &&self.repository,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsDownloaderOptions {
            #[inline]
            fn clone(&self) -> ObsDownloaderOptions {
                ObsDownloaderOptions {
                    repository: ::core::clone::Clone::clone(&self.repository),
                }
            }
        }
        impl ObsDownloaderOptions {
            pub fn new() -> Self {
                ObsDownloaderOptions {
                    repository: GITHUB_REPO.to_string(),
                }
            }
            pub fn set_repository(mut self, repository: &str) -> Self {
                self.repository = repository.to_string();
                self
            }
            pub fn get_repository(&self) -> &str {
                &self.repository
            }
        }
        impl Default for ObsDownloaderOptions {
            fn default() -> Self {
                ObsDownloaderOptions::new()
            }
        }
    }
    mod version {
        use std::path::Path;
        use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
        pub fn get_installed_version(obs_dll: &Path) -> anyhow::Result<Option<String>> {
            let dll_exists = obs_dll.exists() && obs_dll.is_file();
            if !dll_exists {
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!(
                                    "obs.dll does not exist at {0}",
                                    obs_dll.display(),
                                ),
                                lvl,
                                &(
                                    "libobs_wrapper::bootstrap::version",
                                    "libobs_wrapper::bootstrap::version",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                };
                return Ok(None);
            }
            {
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("Getting obs.dll version string"),
                            lvl,
                            &(
                                "libobs_wrapper::bootstrap::version",
                                "libobs_wrapper::bootstrap::version",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                }
            };
            let version = unsafe { libobs::obs_get_version_string() };
            if version.is_null() {
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!("obs.dll does not have a version string"),
                                lvl,
                                &(
                                    "libobs_wrapper::bootstrap::version",
                                    "libobs_wrapper::bootstrap::version",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                };
                return Ok(None);
            }
            let version_str = unsafe { std::ffi::CStr::from_ptr(version) }.to_str();
            if version_str.is_err() {
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!(
                                    "obs.dll version string is not valid UTF-8: {0}",
                                    version_str.err().unwrap(),
                                ),
                                lvl,
                                &(
                                    "libobs_wrapper::bootstrap::version",
                                    "libobs_wrapper::bootstrap::version",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                };
                return Ok(None);
            }
            return Ok(Some(version_str.unwrap().to_string()));
        }
        pub fn should_update(version_str: &str) -> anyhow::Result<bool> {
            let version = version_str.split('.').collect::<Vec<_>>();
            if version.len() != 3 {
                return ::anyhow::__private::Err(
                    ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Invalid version string: {0}", version_str),
                            );
                            res
                        }),
                    ),
                );
            }
            let major = version[0].parse::<u64>();
            let minor = version[1].parse::<u64>();
            let patch = version[2].parse::<u64>();
            if major.is_err() || minor.is_err() || patch.is_err() {
                return ::anyhow::__private::Err(
                    ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Invalid version string: {0}", version_str),
                            );
                            res
                        }),
                    ),
                );
            }
            let major = major.unwrap();
            let minor = minor.unwrap();
            let patch = patch.unwrap();
            return Ok(
                major != LIBOBS_API_MAJOR_VER as u64
                    || minor != LIBOBS_API_MINOR_VER as u64
                    || patch < LIBOBS_API_PATCH_VER as u64,
            );
        }
    }
    pub use options::ObsDownloaderOptions;
    pub enum BootstrapStatus {
        /// Downloading status (first is progress from 0.0 to 1.0 and second is message)
        Downloading(f32, String),
        /// Extracting status (first is progress from 0.0 to 1.0 and second is message)
        Extracting(f32, String),
        Error(anyhow::Error),
        /// The application must be restarted to use the new version of OBS.
        /// This is because the obs.dll file is in use by the application and can not be replaced while running.
        /// Therefore the "updater" is spawned to watch for the application to exit and rename the "obs_new.dll" file to "obs.dll".
        /// The updater will start the application again with the same arguments as the original application.
        /// Call `ObsContext::spawn_updater()`
        RestartRequired,
    }
    /// A trait for bootstrapping OBS Studio.
    ///
    /// This trait provides functionality to download, extract, and set up OBS Studio
    /// for use with libobs-rs. It also handles updates to OBS when necessary.
    ///
    /// If you want to use this bootstrapper to also install required OBS binaries at runtime,
    /// do the following:
    /// - Add a `obs.dll` file to your executable directory. This file will be replaced by the obs installer.
    /// Recommended to use is the a dll dummy (found [here](https://github.com/sshcrack/libobs-builds/releases), make sure you use the correct OBS version)
    /// and rename it to `obs.dll`.
    /// - Call `ObsContext::bootstrap()` at the start of your application. This will download the latest version of OBS and extract it in the executable directory.
    /// - If BootstrapStatus::RestartRequired is returned, call `ObsContext::spawn_updater()` to spawn the updater process.
    /// - Exit the application. The updater process will wait for the application to exit and rename the `obs_new.dll` file to `obs.dll` and restart your application with the same arguments as before.
    ///
    /// [Example project](https://github.com/joshprk/libobs-rs/tree/main/examples/download-at-runtime)
    pub trait ObsBootstrap {
        fn is_valid_installation() -> anyhow::Result<bool>;
        fn should_update() -> anyhow::Result<bool>;
        /// Downloads the latest version of OBS from the specified repository and extracts it to a temporary directory.
        /// Puts the required dll files in the directory of the executable.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn bootstrap<'async_trait>(
            options: options::ObsDownloaderOptions,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<impl Stream<Item = BootstrapStatus>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >;
        /// This function is used to spawn the updater process. For more info see `BootstrapStatus::RestartRequired`.
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn spawn_updater<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >;
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub(crate) struct LIBRARY_OBS_VERSION {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub(crate) static LIBRARY_OBS_VERSION: LIBRARY_OBS_VERSION = LIBRARY_OBS_VERSION {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for LIBRARY_OBS_VERSION {
        type Target = String;
        fn deref(&self) -> &String {
            #[inline(always)]
            fn __static_ref_initialize() -> String {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}.{1}.{2}",
                            LIBOBS_API_MAJOR_VER,
                            LIBOBS_API_MINOR_VER,
                            LIBOBS_API_PATCH_VER,
                        ),
                    );
                    res
                })
            }
            #[inline(always)]
            fn __stability() -> &'static String {
                static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for LIBRARY_OBS_VERSION {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub const UPDATER_SCRIPT: &'static str = "param (\r\n    [Parameter(Mandatory = $true)]\r\n    [string]$binary,\r\n\r\n    [Parameter(Mandatory = $false)]\r\n    [string[]]$arguments = @(),\r\n\r\n    [Parameter(Mandatory = $true)]\r\n    [int]$processPid\r\n)\r\n\r\nWrite-Host \"Waiting for process with PID $processPid to exit...\"\r\ntry {\r\n    $process = Get-Process -Id $processPid -ErrorAction Stop\r\n    $process.WaitForExit()\r\n    Write-Host \"Process with PID $processPid has exited.\"\r\n}\r\ncatch {\r\n    Write-Host \"Process with PID $processPid is not running or already exited.\"\r\n}\r\n\r\n$binaryDirectory = [System.IO.Path]::GetDirectoryName($binary)\r\n$obsNewDir = Join-Path -Path $binaryDirectory -ChildPath \"obs_new\"\r\n\r\nif (Test-Path $obsNewDir -PathType Container) {\r\n    Write-Host \"Found obs_new directory, copying all contents to binary directory\"\r\n\r\n    try {\r\n        $files = Get-ChildItem -Path $obsNewDir -Recurse\r\n        foreach ($file in $files) {\r\n            $relativePath = $file.FullName.Substring($obsNewDir.Length + 1)\r\n            $destination = Join-Path -Path $binaryDirectory -ChildPath $relativePath\r\n\r\n            # Create directory structure if needed\r\n            if ($file.PSIsContainer) {\r\n                if (-Not (Test-Path $destination -PathType Container)) {\r\n                    New-Item -ItemType Directory -Path $destination -Force | Out-Null\r\n                }\r\n                continue\r\n            }\r\n\r\n            # Remove target file if it exists\r\n            if (Test-Path $destination) {\r\n                try {\r\n                    Remove-Item -Path $destination -Force\r\n                }\r\n                catch {\r\n                    Write-Host \"Failed to remove existing file ${destination}: $_\"\r\n                    exit 1\r\n                }\r\n            }\r\n\r\n            # Copy the file\r\n            try {\r\n                Copy-Item -Path $file.FullName -Destination $destination -Force\r\n            }\r\n            catch {\r\n                Write-Host \"Failed to copy $($file.FullName) to ${destination}: $_\"\r\n                exit 1\r\n            }\r\n        }\r\n        Write-Host \"Successfully copied all contents from obs_new to binary directory\"\r\n\r\n        # Optionally remove the obs_new directory after successful copy\r\n        try {\r\n            Remove-Item -Path $obsNewDir -Recurse -Force\r\n            Write-Host \"Removed obs_new directory after copying contents\"\r\n        }\r\n        catch {\r\n            Write-Host \"Warning: Could not remove obs_new directory: $_\"\r\n        }\r\n    }\r\n    catch {\r\n        Write-Host \"Error copying files from obs_new directory: $_\"\r\n        exit 1\r\n    }\r\n}\r\nelse {\r\n    Write-Host \"Warning: obs_new directory not found in $binaryDirectory\"\r\n}\r\n\r\n# Restart the binary with given arguments\r\nWrite-Host \"Restarting $binary with arguments: $($arguments -join \' \')\"\r\ntry {\r\n    if ($arguments.Count -eq 0) {\r\n        Start-Process -FilePath $binary\r\n    }\r\n    else {\r\n        Start-Process -FilePath $binary -ArgumentList $arguments\r\n    }\r\n    Write-Host \"Successfully restarted $binary\"\r\n}\r\ncatch {\r\n    Write-Host \"Failed to restart ${binary}: $_\"\r\n    exit 1\r\n}";
    fn get_obs_dll_path() -> anyhow::Result<PathBuf> {
        let executable = env::current_exe()?;
        let obs_dll = executable
            .parent()
            .ok_or_else(|| ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(
                    format_args!("Failed to get parent directory"),
                );
                error
            }))?
            .join("obs.dll");
        Ok(obs_dll)
    }
    impl ObsBootstrap for ObsContext {
        fn is_valid_installation() -> anyhow::Result<bool> {
            let installed = version::get_installed_version(&get_obs_dll_path()?)?;
            Ok(installed.is_some())
        }
        fn should_update() -> anyhow::Result<bool> {
            let installed = version::get_installed_version(&get_obs_dll_path()?)?;
            if installed.is_none() {
                return Ok(true);
            }
            let installed = installed.unwrap();
            Ok(version::should_update(&installed)?)
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn bootstrap<'async_trait>(
            options: options::ObsDownloaderOptions,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<impl Stream<Item = BootstrapStatus>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    anyhow::Result<_>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let options = options;
                let __ret: anyhow::Result<_> = {
                    let repo = options.repository.to_string();
                    let update = Self::should_update()?;
                    Ok({
                        let (mut __yield_tx, __yield_rx) = unsafe {
                            ::async_stream::__private::yielder::pair()
                        };
                        ::async_stream::__private::AsyncStream::new(
                            __yield_rx,
                            async move {
                                '__async_stream_private_check_scope: {
                                    if !update {
                                        return;
                                    }
                                    {
                                        {
                                            let lvl = ::log::Level::Debug;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    { ::log::__private_api::GlobalLogger },
                                                    format_args!("Downloading OBS from {0}", repo),
                                                    lvl,
                                                    &(
                                                        "libobs_wrapper::bootstrap",
                                                        "libobs_wrapper::bootstrap",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        }
                                    };
                                    let download_stream = download::download_obs(&repo).await;
                                    if let Err(err) = download_stream {
                                        {
                                            #[allow(unreachable_code)]
                                            if false {
                                                break '__async_stream_private_check_scope (loop {});
                                            }
                                            __yield_tx.send(BootstrapStatus::Error(err)).await
                                        };
                                        return;
                                    }
                                    let download_stream = download_stream.unwrap();
                                    let mut download_stream = download_stream;
                                    #[allow(unused_mut)]
                                    let mut download_stream = unsafe {
                                        ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                            &mut download_stream,
                                        )
                                    };
                                    let mut file = None;
                                    while let Some(item) = download_stream.next().await {
                                        match item {
                                            DownloadStatus::Error(err) => {
                                                {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx.send(BootstrapStatus::Error(err)).await
                                                };
                                                return;
                                            }
                                            DownloadStatus::Progress(progress, message) => {
                                                {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx
                                                        .send(BootstrapStatus::Downloading(progress, message))
                                                        .await
                                                };
                                            }
                                            DownloadStatus::Done(path) => file = Some(path),
                                        }
                                    }
                                    let archive_file = file
                                        .ok_or_else(|| ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("OBS Archive could not be downloaded."),
                                            );
                                            error
                                        }));
                                    if let Err(err) = archive_file {
                                        {
                                            #[allow(unreachable_code)]
                                            if false {
                                                break '__async_stream_private_check_scope (loop {});
                                            }
                                            __yield_tx.send(BootstrapStatus::Error(err)).await
                                        };
                                        return;
                                    }
                                    {
                                        {
                                            let lvl = ::log::Level::Debug;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    { ::log::__private_api::GlobalLogger },
                                                    format_args!("Extracting OBS to {0:?}", archive_file),
                                                    lvl,
                                                    &(
                                                        "libobs_wrapper::bootstrap",
                                                        "libobs_wrapper::bootstrap",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        }
                                    };
                                    let archive_file = archive_file.unwrap();
                                    let extract_stream = extract::extract_obs(&archive_file)
                                        .await;
                                    if let Err(err) = extract_stream {
                                        {
                                            #[allow(unreachable_code)]
                                            if false {
                                                break '__async_stream_private_check_scope (loop {});
                                            }
                                            __yield_tx.send(BootstrapStatus::Error(err)).await
                                        };
                                        return;
                                    }
                                    let extract_stream = extract_stream.unwrap();
                                    let mut extract_stream = extract_stream;
                                    #[allow(unused_mut)]
                                    let mut extract_stream = unsafe {
                                        ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                            &mut extract_stream,
                                        )
                                    };
                                    while let Some(item) = extract_stream.next().await {
                                        match item {
                                            ExtractStatus::Error(err) => {
                                                {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx.send(BootstrapStatus::Error(err)).await
                                                };
                                                return;
                                            }
                                            ExtractStatus::Progress(progress, message) => {
                                                {
                                                    #[allow(unreachable_code)]
                                                    if false {
                                                        break '__async_stream_private_check_scope (loop {});
                                                    }
                                                    __yield_tx
                                                        .send(BootstrapStatus::Extracting(progress, message))
                                                        .await
                                                };
                                            }
                                        }
                                    }
                                    {
                                        #[allow(unreachable_code)]
                                        if false {
                                            break '__async_stream_private_check_scope (loop {});
                                        }
                                        __yield_tx.send(BootstrapStatus::RestartRequired).await
                                    };
                                }
                            },
                        )
                    })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn spawn_updater<'async_trait>() -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    anyhow::Result<()>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let __ret: anyhow::Result<()> = {
                    let pid = process::id();
                    let args = env::args().collect::<Vec<_>>();
                    let args = args.into_iter().skip(1).collect::<Vec<_>>();
                    let updater_path = env::temp_dir().join("libobs_updater.ps1");
                    let mut updater_file = File::create(&updater_path)
                        .await
                        .context("Creating updater script")?;
                    updater_file
                        .write_all(UPDATER_SCRIPT.as_bytes())
                        .await
                        .context("Writing updater script")?;
                    let mut command = Command::new("powershell");
                    command
                        .arg("-ExecutionPolicy")
                        .arg("Bypass")
                        .arg("-NoProfile")
                        .arg("-WindowStyle")
                        .arg("Hidden")
                        .arg("-File")
                        .arg(updater_path)
                        .arg("-processPid")
                        .arg(pid.to_string())
                        .arg("-binary")
                        .arg(env::current_exe()?.to_string_lossy().to_string());
                    if !args.is_empty() {
                        command.arg("-arguments");
                        command
                            .arg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("({0})", args.join(",").replace("\"", "`\"")),
                                    );
                                    res
                                }),
                            );
                    }
                    command.spawn().context("Spawning updater process")?;
                    Ok(())
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
pub mod utils {
    mod error {
        use std::fmt::Display;
        use crate::enums::ObsResetVideoStatus;
        /// Error type for OBS function calls.
        pub enum ObsError {
            /// The `obs_startup` function failed on libobs.
            Failure,
            /// Failed to lock mutex describing whether there is a
            /// thread using libobs or not. Report to crate maintainer.
            MutexFailure,
            /// Some or no thread is already using libobs. This is a bug!
            ThreadFailure,
            /// Unable to reset video.
            ResetVideoFailure(ObsResetVideoStatus),
            /// Unable to reset video because the program attempted to
            /// change the graphics module. This is a bug!
            ResetVideoFailureGraphicsModule,
            /// The function returned a null pointer, often indicating
            /// an error with creating the object of the requested
            /// pointer.
            NullPointer,
            OutputAlreadyActive,
            OutputStartFailure(Option<String>),
            OutputStopFailure(Option<String>),
            /// Native error from the Windows API when creating a display
            DisplayCreationError(String),
            OutputSaveBufferFailure(String),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsError {
            #[inline]
            fn clone(&self) -> ObsError {
                match self {
                    ObsError::Failure => ObsError::Failure,
                    ObsError::MutexFailure => ObsError::MutexFailure,
                    ObsError::ThreadFailure => ObsError::ThreadFailure,
                    ObsError::ResetVideoFailure(__self_0) => {
                        ObsError::ResetVideoFailure(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ObsError::ResetVideoFailureGraphicsModule => {
                        ObsError::ResetVideoFailureGraphicsModule
                    }
                    ObsError::NullPointer => ObsError::NullPointer,
                    ObsError::OutputAlreadyActive => ObsError::OutputAlreadyActive,
                    ObsError::OutputStartFailure(__self_0) => {
                        ObsError::OutputStartFailure(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ObsError::OutputStopFailure(__self_0) => {
                        ObsError::OutputStopFailure(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ObsError::DisplayCreationError(__self_0) => {
                        ObsError::DisplayCreationError(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ObsError::OutputSaveBufferFailure(__self_0) => {
                        ObsError::OutputSaveBufferFailure(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ObsError::Failure => ::core::fmt::Formatter::write_str(f, "Failure"),
                    ObsError::MutexFailure => {
                        ::core::fmt::Formatter::write_str(f, "MutexFailure")
                    }
                    ObsError::ThreadFailure => {
                        ::core::fmt::Formatter::write_str(f, "ThreadFailure")
                    }
                    ObsError::ResetVideoFailure(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ResetVideoFailure",
                            &__self_0,
                        )
                    }
                    ObsError::ResetVideoFailureGraphicsModule => {
                        ::core::fmt::Formatter::write_str(
                            f,
                            "ResetVideoFailureGraphicsModule",
                        )
                    }
                    ObsError::NullPointer => {
                        ::core::fmt::Formatter::write_str(f, "NullPointer")
                    }
                    ObsError::OutputAlreadyActive => {
                        ::core::fmt::Formatter::write_str(f, "OutputAlreadyActive")
                    }
                    ObsError::OutputStartFailure(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "OutputStartFailure",
                            &__self_0,
                        )
                    }
                    ObsError::OutputStopFailure(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "OutputStopFailure",
                            &__self_0,
                        )
                    }
                    ObsError::DisplayCreationError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "DisplayCreationError",
                            &__self_0,
                        )
                    }
                    ObsError::OutputSaveBufferFailure(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "OutputSaveBufferFailure",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ObsError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ObsError {
            #[inline]
            fn eq(&self, other: &ObsError) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ObsError::ResetVideoFailure(__self_0),
                            ObsError::ResetVideoFailure(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ObsError::OutputStartFailure(__self_0),
                            ObsError::OutputStartFailure(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ObsError::OutputStopFailure(__self_0),
                            ObsError::OutputStopFailure(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ObsError::DisplayCreationError(__self_0),
                            ObsError::DisplayCreationError(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ObsError::OutputSaveBufferFailure(__self_0),
                            ObsError::OutputSaveBufferFailure(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ObsError {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ObsResetVideoStatus>;
                let _: ::core::cmp::AssertParamIsEq<Option<String>>;
                let _: ::core::cmp::AssertParamIsEq<Option<String>>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        impl Display for ObsError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("OBS Error: "))?;
                match self {
                    ObsError::Failure => {
                        f.write_fmt(
                            format_args!("`obs-startup` function failed on libobs"),
                        )
                    }
                    ObsError::MutexFailure => {
                        f.write_fmt(
                            format_args!(
                                "Failed to lock mutex describing whether there is a thread using libobs or not. Report to crate maintainer.",
                            ),
                        )
                    }
                    ObsError::ThreadFailure => {
                        f.write_fmt(
                            format_args!(
                                "Some or no thread is already using libobs. This is a bug!",
                            ),
                        )
                    }
                    ObsError::ResetVideoFailure(status) => {
                        f.write_fmt(
                            format_args!(
                                "Could not reset obs video. Status: {0:?}",
                                status,
                            ),
                        )
                    }
                    ObsError::ResetVideoFailureGraphicsModule => {
                        f.write_fmt(
                            format_args!(
                                "Unable to reset video because the program attempted to change the graphics module. This is a bug!",
                            ),
                        )
                    }
                    ObsError::NullPointer => {
                        f.write_fmt(
                            format_args!(
                                "The function returned a null pointer, often indicating an error with creating the object of the requested pointer.",
                            ),
                        )
                    }
                    ObsError::OutputAlreadyActive => {
                        f.write_fmt(format_args!("Output is already active."))
                    }
                    ObsError::OutputStartFailure(s) => {
                        f.write_fmt(
                            format_args!("Output failed to start. Error is {0:?}", s),
                        )
                    }
                    ObsError::OutputStopFailure(s) => {
                        f.write_fmt(
                            format_args!("Output failed to stop. Error is {0:?}", s),
                        )
                    }
                    ObsError::DisplayCreationError(e) => {
                        f.write_fmt(
                            format_args!(
                                "Native error from the Windows API when creating a display: {0:?}",
                                e,
                            ),
                        )
                    }
                    ObsError::OutputSaveBufferFailure(e) => {
                        f.write_fmt(
                            format_args!("Couldn\'t save output buffer: {0:?}", e),
                        )
                    }
                }
            }
        }
        impl std::error::Error for ObsError {}
    }
    mod info {
        mod startup {
            use crate::{
                data::{audio::ObsAudioInfo, video::ObsVideoInfo},
                logger::{ConsoleLogger, ObsLogger},
                utils::{ObsPath, ObsString},
            };
            /// Contains information to start a libobs context.
            /// This is passed to the creation of `ObsContext`.
            pub struct StartupInfo {
                pub(crate) startup_paths: StartupPaths,
                pub(crate) obs_video_info: ObsVideoInfo,
                pub(crate) obs_audio_info: ObsAudioInfo,
                pub(crate) logger: Option<Box<dyn ObsLogger>>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for StartupInfo {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "StartupInfo",
                        "startup_paths",
                        &self.startup_paths,
                        "obs_video_info",
                        &self.obs_video_info,
                        "obs_audio_info",
                        &self.obs_audio_info,
                        "logger",
                        &&self.logger,
                    )
                }
            }
            impl StartupInfo {
                pub fn new() -> StartupInfo {
                    Self::default()
                }
                pub fn set_startup_paths(mut self, paths: StartupPaths) -> Self {
                    self.startup_paths = paths;
                    self
                }
                pub fn set_video_info(mut self, ovi: ObsVideoInfo) -> Self {
                    self.obs_video_info = ovi;
                    self
                }
                pub fn set_logger(mut self, logger: Box<dyn ObsLogger>) -> Self {
                    self.logger = Some(logger);
                    self
                }
            }
            impl Default for StartupInfo {
                fn default() -> StartupInfo {
                    Self {
                        startup_paths: StartupPaths::default(),
                        obs_video_info: ObsVideoInfo::default(),
                        obs_audio_info: ObsAudioInfo::default(),
                        logger: Some(Box::new(ConsoleLogger::new())),
                    }
                }
            }
            /// Contains the necessary paths for starting the
            /// libobs context built from `ObsPath`.
            ///
            /// Note that these strings are copied when parsed,
            /// meaning that these can be freed immediately
            /// after all three strings have been used.
            pub struct StartupPaths {
                libobs_data_path: ObsString,
                plugin_bin_path: ObsString,
                plugin_data_path: ObsString,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StartupPaths {
                #[inline]
                fn clone(&self) -> StartupPaths {
                    StartupPaths {
                        libobs_data_path: ::core::clone::Clone::clone(
                            &self.libobs_data_path,
                        ),
                        plugin_bin_path: ::core::clone::Clone::clone(
                            &self.plugin_bin_path,
                        ),
                        plugin_data_path: ::core::clone::Clone::clone(
                            &self.plugin_data_path,
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for StartupPaths {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "StartupPaths",
                        "libobs_data_path",
                        &self.libobs_data_path,
                        "plugin_bin_path",
                        &self.plugin_bin_path,
                        "plugin_data_path",
                        &&self.plugin_data_path,
                    )
                }
            }
            impl StartupPaths {
                pub fn new(
                    libobs_data_path: ObsPath,
                    plugin_bin_path: ObsPath,
                    plugin_data_path: ObsPath,
                ) -> StartupPaths {
                    Self {
                        libobs_data_path: libobs_data_path.build(),
                        plugin_bin_path: plugin_bin_path.build(),
                        plugin_data_path: plugin_data_path.build(),
                    }
                }
                pub fn libobs_data_path(&self) -> &ObsString {
                    &(self.libobs_data_path)
                }
                pub fn plugin_bin_path(&self) -> &ObsString {
                    &(self.plugin_bin_path)
                }
                pub fn plugin_data_path(&self) -> &ObsString {
                    &(self.plugin_data_path)
                }
            }
            impl Default for StartupPaths {
                fn default() -> Self {
                    StartupPathsBuilder::new().build()
                }
            }
            pub struct StartupPathsBuilder {
                libobs_data_path: ObsPath,
                plugin_bin_path: ObsPath,
                plugin_data_path: ObsPath,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StartupPathsBuilder {
                #[inline]
                fn clone(&self) -> StartupPathsBuilder {
                    StartupPathsBuilder {
                        libobs_data_path: ::core::clone::Clone::clone(
                            &self.libobs_data_path,
                        ),
                        plugin_bin_path: ::core::clone::Clone::clone(
                            &self.plugin_bin_path,
                        ),
                        plugin_data_path: ::core::clone::Clone::clone(
                            &self.plugin_data_path,
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for StartupPathsBuilder {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "StartupPathsBuilder",
                        "libobs_data_path",
                        &self.libobs_data_path,
                        "plugin_bin_path",
                        &self.plugin_bin_path,
                        "plugin_data_path",
                        &&self.plugin_data_path,
                    )
                }
            }
            impl StartupPathsBuilder {
                fn new() -> Self {
                    Self {
                        libobs_data_path: ObsPath::from_relative("data/libobs"),
                        plugin_bin_path: ObsPath::from_relative("obs-plugins/64bit"),
                        plugin_data_path: ObsPath::from_relative(
                            "data/obs-plugins/%module%",
                        ),
                    }
                }
                pub fn build(self) -> StartupPaths {
                    StartupPaths {
                        libobs_data_path: self.libobs_data_path.build(),
                        plugin_bin_path: self.plugin_bin_path.build(),
                        plugin_data_path: self.plugin_data_path.build(),
                    }
                }
                pub fn libobs_data_path(mut self, value: ObsPath) -> Self {
                    self.libobs_data_path = value;
                    self
                }
                pub fn plugin_bin_path(mut self, value: ObsPath) -> Self {
                    self.plugin_bin_path = value;
                    self
                }
                pub fn plugin_data_path(mut self, value: ObsPath) -> Self {
                    self.plugin_data_path = value;
                    self
                }
            }
            impl Default for StartupPathsBuilder {
                fn default() -> StartupPathsBuilder {
                    Self::new()
                }
            }
        }
        pub use startup::*;
        use crate::data::ObsData;
        use super::ObsString;
        pub struct ObjectInfo {
            pub(crate) id: ObsString,
            pub(crate) name: ObsString,
            pub(crate) settings: Option<ObsData>,
            pub(crate) hotkey_data: Option<ObsData>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObjectInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ObjectInfo",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "settings",
                    &self.settings,
                    "hotkey_data",
                    &&self.hotkey_data,
                )
            }
        }
        impl ObjectInfo {
            pub fn new(
                id: impl Into<ObsString>,
                name: impl Into<ObsString>,
                settings: Option<ObsData>,
                hotkey_data: Option<ObsData>,
            ) -> Self {
                let id = id.into();
                let name = name.into();
                Self {
                    id,
                    name,
                    settings,
                    hotkey_data,
                }
            }
        }
        pub type OutputInfo = ObjectInfo;
        pub type SourceInfo = ObjectInfo;
        pub type AudioEncoderInfo = ObjectInfo;
        pub type VideoEncoderInfo = ObjectInfo;
    }
    mod obs_string {
        use std::ffi::CString;
        use std::os::raw::c_char;
        /// String wrapper for OBS function calls.
        ///
        /// This struct wraps `CString` internally with included helper
        /// functions. Note that any NUL byte is stripped before
        /// conversion to a `CString` to prevent panicking.
        pub struct ObsString {
            c_string: CString,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsString {
            #[inline]
            fn clone(&self) -> ObsString {
                ObsString {
                    c_string: ::core::clone::Clone::clone(&self.c_string),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ObsString {
            #[inline]
            fn default() -> ObsString {
                ObsString {
                    c_string: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsString {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ObsString",
                    "c_string",
                    &&self.c_string,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ObsString {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ObsString {
            #[inline]
            fn eq(&self, other: &ObsString) -> bool {
                self.c_string == other.c_string
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ObsString {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<CString>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ObsString {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ObsString,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.c_string, &other.c_string)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ObsString {
            #[inline]
            fn cmp(&self, other: &ObsString) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.c_string, &other.c_string)
            }
        }
        impl ObsString {
            /// Creates a new `ObsString` wrapper for C-type
            /// strings used by libobs. Note that all NUL
            /// bytes are removed before conversion to a
            /// `ObsString` as C-type strings do not allow
            /// premature NUL bytes.
            ///
            /// These are CString wrappers internally, with
            /// included helper functions to reduce repetitive
            /// code and ensure safety.
            pub fn new(value: &str) -> Self {
                Self::from(value)
            }
            /// Returns a safe pointer to a C-type string
            /// used by libobs. This pointer will be valid
            /// for as long as this ObsString exists.
            ///
            /// Note that this pointer is read-only--writing
            /// to it is undefined behavior.
            pub fn as_ptr(&self) -> *const c_char {
                self.c_string.as_ptr()
            }
        }
        impl ToString for ObsString {
            fn to_string(&self) -> String {
                self.c_string.to_string_lossy().to_string()
            }
        }
        impl From<&str> for ObsString {
            fn from(value: &str) -> Self {
                let value = value.replace("\0", "");
                Self {
                    c_string: CString::new(value).unwrap(),
                }
            }
        }
        impl From<Vec<u8>> for ObsString {
            fn from(value: Vec<u8>) -> Self {
                let mut value = value
                    .into_iter()
                    .filter(|x| *x != b'\0')
                    .collect::<Vec<u8>>();
                value.push(b'\0');
                Self {
                    c_string: CString::from_vec_with_nul(value).unwrap(),
                }
            }
        }
        impl From<String> for ObsString {
            fn from(value: String) -> Self {
                let value = value.replace("\0", "");
                Self {
                    c_string: CString::new(value).unwrap(),
                }
            }
        }
    }
    mod path {
        use std::{env, path::{Path, PathBuf}};
        use super::ObsString;
        /// Builds into an `ObsString` that represents a path used
        /// by libobs.
        ///
        /// Note that only this path only supports UTF-8 for the
        /// entire absolute path because libobs only supports
        /// UTF-8.
        pub struct ObsPath {
            path: PathBuf,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ObsPath {
            #[inline]
            fn clone(&self) -> ObsPath {
                ObsPath {
                    path: ::core::clone::Clone::clone(&self.path),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ObsPath {
            #[inline]
            fn default() -> ObsPath {
                ObsPath {
                    path: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ObsPath {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ObsPath",
                    "path",
                    &&self.path,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ObsPath {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ObsPath {
            #[inline]
            fn eq(&self, other: &ObsPath) -> bool {
                self.path == other.path
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ObsPath {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<PathBuf>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ObsPath {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ObsPath,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.path, &other.path)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ObsPath {
            #[inline]
            fn cmp(&self, other: &ObsPath) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.path, &other.path)
            }
        }
        impl ObsPath {
            /// Creates a new `ObsPath` strictly using the path
            /// `path_str` without any modifications.
            ///
            /// If you want to create a relative path, use
            /// `ObsPath::from_relative`.
            pub fn new(path_str: &str) -> Self {
                Self {
                    path: Path::new(path_str).into(),
                }
            }
            /// Creates a new `ObsPath` with `path_str`
            /// appended to the path of the directory which the
            /// executable file is in.
            ///
            /// If you want to create an absolute path, use
            /// `ObsPath::new`.
            pub fn from_relative(path_str: &str) -> Self {
                let mut relative_path = env::current_exe().unwrap();
                relative_path.pop();
                let obs_path = Self { path: relative_path };
                let path_str = path_str.trim_matches('/');
                obs_path.push(path_str)
            }
            /// Modifies the path to point to the path
            /// `path_str` appended to the current path which
            /// `ObsPath` is pointing to.
            pub fn push(mut self, value: &str) -> Self {
                let split = value.split(['/', '\\'].as_ref());
                for item in split {
                    if item.len() > 0 {
                        self.path.push(item);
                    }
                }
                self
            }
            /// Modifies the path to point to its current
            /// parent. This is analogous to `Obs::push(".")`.
            pub fn pop(mut self) -> Self {
                self.path.pop();
                self
            }
            /// Consumes the `ObsPath` to create a new
            /// immutable ObsString that encodes a UTF-8
            /// C-type string which describes the path that
            /// the `ObsPath` is pointing to.
            ///
            /// Note that this function is lossy in that
            /// any non-Unicode data is completely removed
            /// from the string. This is because libobs
            /// does not support non-Unicode characters in
            /// its path.
            pub fn build(self) -> ObsString {
                let mut bytes = self.path.display().to_string().replace("\\", "/");
                if self.path.is_dir() {
                    bytes = bytes + "/";
                }
                let obs_string = ObsString::from(bytes.as_str());
                drop(self);
                obs_string
            }
        }
        impl Into<ObsString> for ObsPath {
            fn into(self) -> ObsString {
                self.build()
            }
        }
    }
    pub(crate) mod initialization {
        //! This is derived from the frontend/obs-main.cpp.
        use windows::{
            core::PCWSTR,
            Win32::{
                Foundation::{CloseHandle, HANDLE, LUID},
                Security::{
                    AdjustTokenPrivileges, LookupPrivilegeValueW, SE_DEBUG_NAME,
                    SE_INC_BASE_PRIORITY_NAME, SE_PRIVILEGE_ENABLED,
                    TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
                },
                System::Threading::{GetCurrentProcess, OpenProcessToken},
            },
        };
        pub fn load_debug_privilege() {
            unsafe {
                let flags = TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY;
                let mut tp = TOKEN_PRIVILEGES::default();
                let mut token = HANDLE::default();
                let mut val = LUID::default();
                if OpenProcessToken(GetCurrentProcess(), flags, &mut token).is_err() {
                    return;
                }
                if LookupPrivilegeValueW(PCWSTR::null(), SE_DEBUG_NAME, &mut val).is_ok()
                {
                    tp.PrivilegeCount = 1;
                    tp.Privileges[0].Luid = val;
                    tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                    let res = AdjustTokenPrivileges(
                        token,
                        false,
                        Some(&tp),
                        std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
                        None,
                        None,
                    );
                    if let Err(e) = res {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "Could not set privilege to debug process: {0:?}\n",
                                    e,
                                ),
                            );
                        };
                    }
                }
                if LookupPrivilegeValueW(
                        PCWSTR::null(),
                        SE_INC_BASE_PRIORITY_NAME,
                        &mut val,
                    )
                    .is_ok()
                {
                    tp.PrivilegeCount = 1;
                    tp.Privileges[0].Luid = val;
                    tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                    let res = AdjustTokenPrivileges(
                        token,
                        false,
                        Some(&tp),
                        std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
                        None,
                        None,
                    );
                    if let Err(e) = res {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "Could not set privilege to increase GPU priority {0:?}\n",
                                    e,
                                ),
                            );
                        };
                    }
                }
                let _ = CloseHandle(token);
            }
        }
    }
    pub mod traits {
        use crate::data::{ObsData, ObsObjectUpdater};
        pub trait ObsUpdatable {
            /// Updates the object with the current settings.
            /// Note that this example requires the `libobs-sources` crate.
            /// ## Example usage
            /// ```rust
            /// use libobs_wrapper::data::ObsObjectUpdater;
            /// let source = WindowCaptureSourceBuilder::new("test_capture")
            ///     .set_window(&window)
            ///     .add_to_scene(scene)
            ///     .unwrap();
            ///
            /// // Do other stuff with source
            ///
            /// // Update the source with the corresponding updater like so
            /// source.create_updater::<WindowCaptureSourceUpdater>();
            /// ```
            fn create_updater<'a, T: ObsObjectUpdater<'a, ToUpdate = Self>>(
                &'a mut self,
            ) -> T
            where
                Self: Sized,
            {
                T::create_update(self)
            }
            fn update_raw(&mut self, data: ObsData);
            fn reset_and_update_raw(&mut self, data: ObsData);
        }
    }
    use std::ffi::CStr;
    pub use error::*;
    pub use info::*;
    use libobs::obs_module_failure_info;
    pub use obs_string::*;
    pub use path::*;
    use crate::{enums::ObsLogLevel, logger::internal_log_global};
    pub struct ObsModules {
        paths: StartupPaths,
        /// A pointer to the module failure info structure.
        info: Option<obs_module_failure_info>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsModules {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ObsModules",
                "paths",
                &self.paths,
                "info",
                &&self.info,
            )
        }
    }
    impl ObsModules {
        pub fn add_paths(paths: &StartupPaths) -> Self {
            unsafe {
                libobs::obs_add_data_path(paths.libobs_data_path().as_ptr());
                libobs::obs_add_module_path(
                    paths.plugin_bin_path().as_ptr(),
                    paths.plugin_data_path().as_ptr(),
                );
            }
            Self {
                paths: paths.clone(),
                info: None,
            }
        }
        pub fn load_modules(&mut self) {
            unsafe {
                let mut failure_info: obs_module_failure_info = std::mem::zeroed();
                internal_log_global(
                    ObsLogLevel::Info,
                    "---------------------------------".to_string(),
                );
                libobs::obs_load_all_modules2(&mut failure_info);
                internal_log_global(
                    ObsLogLevel::Info,
                    "---------------------------------".to_string(),
                );
                libobs::obs_log_loaded_modules();
                internal_log_global(
                    ObsLogLevel::Info,
                    "---------------------------------".to_string(),
                );
                libobs::obs_post_load_modules();
                self.info = Some(failure_info);
            }
            self.log_if_failed();
        }
        pub fn log_if_failed(&self) {
            if self.info.is_none_or(|x| x.count == 0) {
                return;
            }
            let info = self.info.as_ref().unwrap();
            let mut failed_modules = Vec::new();
            for i in 0..info.count {
                let module = unsafe { info.failed_modules.offset(i as isize) };
                let plugin_name = unsafe { CStr::from_ptr(*module) };
                failed_modules.push(plugin_name.to_string_lossy());
            }
            internal_log_global(
                ObsLogLevel::Warning,
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Failed to load modules: {0}",
                            failed_modules.join(", "),
                        ),
                    );
                    res
                }),
            );
        }
    }
    impl Drop for ObsModules {
        fn drop(&mut self) {
            unsafe {
                libobs::obs_remove_data_path(self.paths.libobs_data_path().as_ptr());
            }
        }
    }
    pub const ENCODER_HIDE_FLAGS: u32 = libobs::OBS_ENCODER_CAP_DEPRECATED
        | libobs::OBS_ENCODER_CAP_INTERNAL;
}
pub mod enums {
    use core::fmt;
    use std::fmt::Display;
    use num_derive::{FromPrimitive, ToPrimitive};
    #[cfg(target_os = "windows")]
    pub(crate) type OsEnumType = i32;
    #[repr(i32)]
    /// Describes the video output format used by the
    /// OBS video context. Used in `ObsVideoInfo`.
    pub enum ObsVideoFormat {
        AYUV = libobs::video_format_VIDEO_FORMAT_AYUV,
        BGR3 = libobs::video_format_VIDEO_FORMAT_BGR3,
        BGRA = libobs::video_format_VIDEO_FORMAT_BGRA,
        BGRX = libobs::video_format_VIDEO_FORMAT_BGRX,
        I010 = libobs::video_format_VIDEO_FORMAT_I010,
        I210 = libobs::video_format_VIDEO_FORMAT_I210,
        I40A = libobs::video_format_VIDEO_FORMAT_I40A,
        I412 = libobs::video_format_VIDEO_FORMAT_I412,
        I420 = libobs::video_format_VIDEO_FORMAT_I420,
        I422 = libobs::video_format_VIDEO_FORMAT_I422,
        I42A = libobs::video_format_VIDEO_FORMAT_I42A,
        I444 = libobs::video_format_VIDEO_FORMAT_I444,
        NONE = libobs::video_format_VIDEO_FORMAT_NONE,
        NV12 = libobs::video_format_VIDEO_FORMAT_NV12,
        P010 = libobs::video_format_VIDEO_FORMAT_P010,
        P216 = libobs::video_format_VIDEO_FORMAT_P216,
        P416 = libobs::video_format_VIDEO_FORMAT_P416,
        R10L = libobs::video_format_VIDEO_FORMAT_R10L,
        RGBA = libobs::video_format_VIDEO_FORMAT_RGBA,
        UYVY = libobs::video_format_VIDEO_FORMAT_UYVY,
        V210 = libobs::video_format_VIDEO_FORMAT_V210,
        Y800 = libobs::video_format_VIDEO_FORMAT_Y800,
        YA2L = libobs::video_format_VIDEO_FORMAT_YA2L,
        YUVA = libobs::video_format_VIDEO_FORMAT_YUVA,
        YUY2 = libobs::video_format_VIDEO_FORMAT_YUY2,
        YVYU = libobs::video_format_VIDEO_FORMAT_YVYU,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsVideoFormat {
        #[inline]
        fn clone(&self) -> ObsVideoFormat {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsVideoFormat {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsVideoFormat {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsVideoFormat::AYUV => "AYUV",
                    ObsVideoFormat::BGR3 => "BGR3",
                    ObsVideoFormat::BGRA => "BGRA",
                    ObsVideoFormat::BGRX => "BGRX",
                    ObsVideoFormat::I010 => "I010",
                    ObsVideoFormat::I210 => "I210",
                    ObsVideoFormat::I40A => "I40A",
                    ObsVideoFormat::I412 => "I412",
                    ObsVideoFormat::I420 => "I420",
                    ObsVideoFormat::I422 => "I422",
                    ObsVideoFormat::I42A => "I42A",
                    ObsVideoFormat::I444 => "I444",
                    ObsVideoFormat::NONE => "NONE",
                    ObsVideoFormat::NV12 => "NV12",
                    ObsVideoFormat::P010 => "P010",
                    ObsVideoFormat::P216 => "P216",
                    ObsVideoFormat::P416 => "P416",
                    ObsVideoFormat::R10L => "R10L",
                    ObsVideoFormat::RGBA => "RGBA",
                    ObsVideoFormat::UYVY => "UYVY",
                    ObsVideoFormat::V210 => "V210",
                    ObsVideoFormat::Y800 => "Y800",
                    ObsVideoFormat::YA2L => "YA2L",
                    ObsVideoFormat::YUVA => "YUVA",
                    ObsVideoFormat::YUY2 => "YUY2",
                    ObsVideoFormat::YVYU => "YVYU",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsVideoFormat {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsVideoFormat {
        #[inline]
        fn eq(&self, other: &ObsVideoFormat) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsVideoFormat {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsVideoFormat {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsVideoFormat::AYUV as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::AYUV)
                } else if n == ObsVideoFormat::BGR3 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::BGR3)
                } else if n == ObsVideoFormat::BGRA as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::BGRA)
                } else if n == ObsVideoFormat::BGRX as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::BGRX)
                } else if n == ObsVideoFormat::I010 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I010)
                } else if n == ObsVideoFormat::I210 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I210)
                } else if n == ObsVideoFormat::I40A as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I40A)
                } else if n == ObsVideoFormat::I412 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I412)
                } else if n == ObsVideoFormat::I420 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I420)
                } else if n == ObsVideoFormat::I422 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I422)
                } else if n == ObsVideoFormat::I42A as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I42A)
                } else if n == ObsVideoFormat::I444 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::I444)
                } else if n == ObsVideoFormat::NONE as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::NONE)
                } else if n == ObsVideoFormat::NV12 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::NV12)
                } else if n == ObsVideoFormat::P010 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::P010)
                } else if n == ObsVideoFormat::P216 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::P216)
                } else if n == ObsVideoFormat::P416 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::P416)
                } else if n == ObsVideoFormat::R10L as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::R10L)
                } else if n == ObsVideoFormat::RGBA as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::RGBA)
                } else if n == ObsVideoFormat::UYVY as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::UYVY)
                } else if n == ObsVideoFormat::V210 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::V210)
                } else if n == ObsVideoFormat::Y800 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::Y800)
                } else if n == ObsVideoFormat::YA2L as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::YA2L)
                } else if n == ObsVideoFormat::YUVA as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::YUVA)
                } else if n == ObsVideoFormat::YUY2 as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::YUY2)
                } else if n == ObsVideoFormat::YVYU as i64 {
                    ::core::option::Option::Some(ObsVideoFormat::YVYU)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsVideoFormat {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsVideoFormat::AYUV => ObsVideoFormat::AYUV as i64,
                        ObsVideoFormat::BGR3 => ObsVideoFormat::BGR3 as i64,
                        ObsVideoFormat::BGRA => ObsVideoFormat::BGRA as i64,
                        ObsVideoFormat::BGRX => ObsVideoFormat::BGRX as i64,
                        ObsVideoFormat::I010 => ObsVideoFormat::I010 as i64,
                        ObsVideoFormat::I210 => ObsVideoFormat::I210 as i64,
                        ObsVideoFormat::I40A => ObsVideoFormat::I40A as i64,
                        ObsVideoFormat::I412 => ObsVideoFormat::I412 as i64,
                        ObsVideoFormat::I420 => ObsVideoFormat::I420 as i64,
                        ObsVideoFormat::I422 => ObsVideoFormat::I422 as i64,
                        ObsVideoFormat::I42A => ObsVideoFormat::I42A as i64,
                        ObsVideoFormat::I444 => ObsVideoFormat::I444 as i64,
                        ObsVideoFormat::NONE => ObsVideoFormat::NONE as i64,
                        ObsVideoFormat::NV12 => ObsVideoFormat::NV12 as i64,
                        ObsVideoFormat::P010 => ObsVideoFormat::P010 as i64,
                        ObsVideoFormat::P216 => ObsVideoFormat::P216 as i64,
                        ObsVideoFormat::P416 => ObsVideoFormat::P416 as i64,
                        ObsVideoFormat::R10L => ObsVideoFormat::R10L as i64,
                        ObsVideoFormat::RGBA => ObsVideoFormat::RGBA as i64,
                        ObsVideoFormat::UYVY => ObsVideoFormat::UYVY as i64,
                        ObsVideoFormat::V210 => ObsVideoFormat::V210 as i64,
                        ObsVideoFormat::Y800 => ObsVideoFormat::Y800 as i64,
                        ObsVideoFormat::YA2L => ObsVideoFormat::YA2L as i64,
                        ObsVideoFormat::YUVA => ObsVideoFormat::YUVA as i64,
                        ObsVideoFormat::YUY2 => ObsVideoFormat::YUY2 as i64,
                        ObsVideoFormat::YVYU => ObsVideoFormat::YVYU as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    #[repr(i32)]
    /// Describes the colorspace that an OBS video context
    /// uses. Used in `ObsVideoInfo`.
    pub enum ObsColorspace {
        CS2100HLG = libobs::video_colorspace_VIDEO_CS_2100_HLG,
        CS2100PQ = libobs::video_colorspace_VIDEO_CS_2100_PQ,
        CS601 = libobs::video_colorspace_VIDEO_CS_601,
        CS709 = libobs::video_colorspace_VIDEO_CS_709,
        Default = libobs::video_colorspace_VIDEO_CS_DEFAULT,
        CSRGB = libobs::video_colorspace_VIDEO_CS_SRGB,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsColorspace {
        #[inline]
        fn clone(&self) -> ObsColorspace {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsColorspace {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsColorspace {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsColorspace::CS2100HLG => "CS2100HLG",
                    ObsColorspace::CS2100PQ => "CS2100PQ",
                    ObsColorspace::CS601 => "CS601",
                    ObsColorspace::CS709 => "CS709",
                    ObsColorspace::Default => "Default",
                    ObsColorspace::CSRGB => "CSRGB",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsColorspace {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsColorspace {
        #[inline]
        fn eq(&self, other: &ObsColorspace) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsColorspace {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsColorspace {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsColorspace::CS2100HLG as i64 {
                    ::core::option::Option::Some(ObsColorspace::CS2100HLG)
                } else if n == ObsColorspace::CS2100PQ as i64 {
                    ::core::option::Option::Some(ObsColorspace::CS2100PQ)
                } else if n == ObsColorspace::CS601 as i64 {
                    ::core::option::Option::Some(ObsColorspace::CS601)
                } else if n == ObsColorspace::CS709 as i64 {
                    ::core::option::Option::Some(ObsColorspace::CS709)
                } else if n == ObsColorspace::Default as i64 {
                    ::core::option::Option::Some(ObsColorspace::Default)
                } else if n == ObsColorspace::CSRGB as i64 {
                    ::core::option::Option::Some(ObsColorspace::CSRGB)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsColorspace {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsColorspace::CS2100HLG => ObsColorspace::CS2100HLG as i64,
                        ObsColorspace::CS2100PQ => ObsColorspace::CS2100PQ as i64,
                        ObsColorspace::CS601 => ObsColorspace::CS601 as i64,
                        ObsColorspace::CS709 => ObsColorspace::CS709 as i64,
                        ObsColorspace::Default => ObsColorspace::Default as i64,
                        ObsColorspace::CSRGB => ObsColorspace::CSRGB as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    #[repr(i32)]
    /// Describes the minimum and maximum color levels that
    /// an OBS video context is allowed to encode. Used in
    /// `ObsVideoInfo.`
    pub enum ObsVideoRange {
        Default = libobs::video_range_type_VIDEO_RANGE_DEFAULT,
        Partial = libobs::video_range_type_VIDEO_RANGE_PARTIAL,
        Full = libobs::video_range_type_VIDEO_RANGE_FULL,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsVideoRange {
        #[inline]
        fn clone(&self) -> ObsVideoRange {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsVideoRange {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsVideoRange {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsVideoRange::Default => "Default",
                    ObsVideoRange::Partial => "Partial",
                    ObsVideoRange::Full => "Full",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsVideoRange {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsVideoRange {
        #[inline]
        fn eq(&self, other: &ObsVideoRange) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsVideoRange {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsVideoRange {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsVideoRange::Default as i64 {
                    ::core::option::Option::Some(ObsVideoRange::Default)
                } else if n == ObsVideoRange::Partial as i64 {
                    ::core::option::Option::Some(ObsVideoRange::Partial)
                } else if n == ObsVideoRange::Full as i64 {
                    ::core::option::Option::Some(ObsVideoRange::Full)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsVideoRange {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsVideoRange::Default => ObsVideoRange::Default as i64,
                        ObsVideoRange::Partial => ObsVideoRange::Partial as i64,
                        ObsVideoRange::Full => ObsVideoRange::Full as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    #[repr(i32)]
    /// Describes how libobs should reconcile non-matching
    /// base and output resolutions when creating a video
    /// context.
    pub enum ObsScaleType {
        Area = libobs::obs_scale_type_OBS_SCALE_AREA,
        Bicubic = libobs::obs_scale_type_OBS_SCALE_BICUBIC,
        Bilinear = libobs::obs_scale_type_OBS_SCALE_BILINEAR,
        Disable = libobs::obs_scale_type_OBS_SCALE_DISABLE,
        Lanczos = libobs::obs_scale_type_OBS_SCALE_LANCZOS,
        Point = libobs::obs_scale_type_OBS_SCALE_POINT,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsScaleType {
        #[inline]
        fn clone(&self) -> ObsScaleType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsScaleType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsScaleType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsScaleType::Area => "Area",
                    ObsScaleType::Bicubic => "Bicubic",
                    ObsScaleType::Bilinear => "Bilinear",
                    ObsScaleType::Disable => "Disable",
                    ObsScaleType::Lanczos => "Lanczos",
                    ObsScaleType::Point => "Point",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsScaleType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsScaleType {
        #[inline]
        fn eq(&self, other: &ObsScaleType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsScaleType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsScaleType {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsScaleType::Area as i64 {
                    ::core::option::Option::Some(ObsScaleType::Area)
                } else if n == ObsScaleType::Bicubic as i64 {
                    ::core::option::Option::Some(ObsScaleType::Bicubic)
                } else if n == ObsScaleType::Bilinear as i64 {
                    ::core::option::Option::Some(ObsScaleType::Bilinear)
                } else if n == ObsScaleType::Disable as i64 {
                    ::core::option::Option::Some(ObsScaleType::Disable)
                } else if n == ObsScaleType::Lanczos as i64 {
                    ::core::option::Option::Some(ObsScaleType::Lanczos)
                } else if n == ObsScaleType::Point as i64 {
                    ::core::option::Option::Some(ObsScaleType::Point)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsScaleType {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsScaleType::Area => ObsScaleType::Area as i64,
                        ObsScaleType::Bicubic => ObsScaleType::Bicubic as i64,
                        ObsScaleType::Bilinear => ObsScaleType::Bilinear as i64,
                        ObsScaleType::Disable => ObsScaleType::Disable as i64,
                        ObsScaleType::Lanczos => ObsScaleType::Lanczos as i64,
                        ObsScaleType::Point => ObsScaleType::Point as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    /// Describes which graphics backend should be used
    /// in the OBS video context. Used in `ObsVideoInfo`.
    pub enum ObsGraphicsModule {
        OpenGL,
        DirectX11,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsGraphicsModule {
        #[inline]
        fn clone(&self) -> ObsGraphicsModule {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsGraphicsModule {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsGraphicsModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsGraphicsModule::OpenGL => "OpenGL",
                    ObsGraphicsModule::DirectX11 => "DirectX11",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsGraphicsModule {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsGraphicsModule {
        #[inline]
        fn eq(&self, other: &ObsGraphicsModule) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsGraphicsModule {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[repr(i32)]
    /// Status types returned after attempting to
    /// reset the OBS video context using the
    /// function `obs_reset_video`.
    pub enum ObsResetVideoStatus {
        /// `obs_reset_video` was successful.
        Success = libobs::OBS_VIDEO_SUCCESS as i32,
        /// The adapter is not supported as it
        /// lacks capabilities.
        NotSupported = libobs::OBS_VIDEO_NOT_SUPPORTED,
        /// A parameter is invalid.
        InvalidParameter = libobs::OBS_VIDEO_INVALID_PARAM,
        /// An output is currently running, preventing
        /// resetting the video context.
        CurrentlyActive = libobs::OBS_VIDEO_CURRENTLY_ACTIVE,
        /// Generic error occured when attempting to
        /// reset the OBS video context.
        Failure = libobs::OBS_VIDEO_FAIL,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsResetVideoStatus {}
    #[automatically_derived]
    impl ::core::clone::Clone for ObsResetVideoStatus {
        #[inline]
        fn clone(&self) -> ObsResetVideoStatus {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsResetVideoStatus {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsResetVideoStatus::Success => "Success",
                    ObsResetVideoStatus::NotSupported => "NotSupported",
                    ObsResetVideoStatus::InvalidParameter => "InvalidParameter",
                    ObsResetVideoStatus::CurrentlyActive => "CurrentlyActive",
                    ObsResetVideoStatus::Failure => "Failure",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsResetVideoStatus {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsResetVideoStatus {
        #[inline]
        fn eq(&self, other: &ObsResetVideoStatus) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsResetVideoStatus {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsResetVideoStatus {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsResetVideoStatus::Success as i64 {
                    ::core::option::Option::Some(ObsResetVideoStatus::Success)
                } else if n == ObsResetVideoStatus::NotSupported as i64 {
                    ::core::option::Option::Some(ObsResetVideoStatus::NotSupported)
                } else if n == ObsResetVideoStatus::InvalidParameter as i64 {
                    ::core::option::Option::Some(ObsResetVideoStatus::InvalidParameter)
                } else if n == ObsResetVideoStatus::CurrentlyActive as i64 {
                    ::core::option::Option::Some(ObsResetVideoStatus::CurrentlyActive)
                } else if n == ObsResetVideoStatus::Failure as i64 {
                    ::core::option::Option::Some(ObsResetVideoStatus::Failure)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsResetVideoStatus {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsResetVideoStatus::Success => {
                            ObsResetVideoStatus::Success as i64
                        }
                        ObsResetVideoStatus::NotSupported => {
                            ObsResetVideoStatus::NotSupported as i64
                        }
                        ObsResetVideoStatus::InvalidParameter => {
                            ObsResetVideoStatus::InvalidParameter as i64
                        }
                        ObsResetVideoStatus::CurrentlyActive => {
                            ObsResetVideoStatus::CurrentlyActive as i64
                        }
                        ObsResetVideoStatus::Failure => {
                            ObsResetVideoStatus::Failure as i64
                        }
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    /// Audio samples per second options that are
    /// supported by libobs.
    #[repr(i32)]
    pub enum ObsSamplesPerSecond {
        /// 44.1 kHz
        F44100 = 44100,
        /// 48.0 kHz
        F48000 = 48000,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsSamplesPerSecond {
        #[inline]
        fn clone(&self) -> ObsSamplesPerSecond {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsSamplesPerSecond {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsSamplesPerSecond {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsSamplesPerSecond::F44100 => "F44100",
                    ObsSamplesPerSecond::F48000 => "F48000",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsSamplesPerSecond {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsSamplesPerSecond {
        #[inline]
        fn eq(&self, other: &ObsSamplesPerSecond) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsSamplesPerSecond {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[repr(i32)]
    pub enum ObsSpeakerLayout {
        S2Point1 = libobs::speaker_layout_SPEAKERS_2POINT1,
        S4Point0 = libobs::speaker_layout_SPEAKERS_4POINT0,
        S4Point1 = libobs::speaker_layout_SPEAKERS_4POINT1,
        S5Point1 = libobs::speaker_layout_SPEAKERS_5POINT1,
        S7Point1 = libobs::speaker_layout_SPEAKERS_7POINT1,
        Mono = libobs::speaker_layout_SPEAKERS_MONO,
        Stereo = libobs::speaker_layout_SPEAKERS_STEREO,
        Unknown = libobs::speaker_layout_SPEAKERS_UNKNOWN,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsSpeakerLayout {
        #[inline]
        fn clone(&self) -> ObsSpeakerLayout {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsSpeakerLayout {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsSpeakerLayout {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsSpeakerLayout::S2Point1 => "S2Point1",
                    ObsSpeakerLayout::S4Point0 => "S4Point0",
                    ObsSpeakerLayout::S4Point1 => "S4Point1",
                    ObsSpeakerLayout::S5Point1 => "S5Point1",
                    ObsSpeakerLayout::S7Point1 => "S7Point1",
                    ObsSpeakerLayout::Mono => "Mono",
                    ObsSpeakerLayout::Stereo => "Stereo",
                    ObsSpeakerLayout::Unknown => "Unknown",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsSpeakerLayout {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsSpeakerLayout {
        #[inline]
        fn eq(&self, other: &ObsSpeakerLayout) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsSpeakerLayout {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsSpeakerLayout {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsSpeakerLayout::S2Point1 as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::S2Point1)
                } else if n == ObsSpeakerLayout::S4Point0 as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::S4Point0)
                } else if n == ObsSpeakerLayout::S4Point1 as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::S4Point1)
                } else if n == ObsSpeakerLayout::S5Point1 as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::S5Point1)
                } else if n == ObsSpeakerLayout::S7Point1 as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::S7Point1)
                } else if n == ObsSpeakerLayout::Mono as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::Mono)
                } else if n == ObsSpeakerLayout::Stereo as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::Stereo)
                } else if n == ObsSpeakerLayout::Unknown as i64 {
                    ::core::option::Option::Some(ObsSpeakerLayout::Unknown)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsSpeakerLayout {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsSpeakerLayout::S2Point1 => ObsSpeakerLayout::S2Point1 as i64,
                        ObsSpeakerLayout::S4Point0 => ObsSpeakerLayout::S4Point0 as i64,
                        ObsSpeakerLayout::S4Point1 => ObsSpeakerLayout::S4Point1 as i64,
                        ObsSpeakerLayout::S5Point1 => ObsSpeakerLayout::S5Point1 as i64,
                        ObsSpeakerLayout::S7Point1 => ObsSpeakerLayout::S7Point1 as i64,
                        ObsSpeakerLayout::Mono => ObsSpeakerLayout::Mono as i64,
                        ObsSpeakerLayout::Stereo => ObsSpeakerLayout::Stereo as i64,
                        ObsSpeakerLayout::Unknown => ObsSpeakerLayout::Unknown as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    pub enum ObsOutputSignal {
        /// Successfully stopped
        Success,
        /// The specified path was invalid
        BadPath,
        /// Failed to connect to a server
        ConnectFailed,
        /// Invalid stream path
        InvalidStream,
        /// Generic error
        Error,
        /// Unexpectedly disconnected
        Disconnected,
        /// The settings, video/audio format, or codecs are unsupported by this output
        Unsupported,
        /// Ran out of disk space
        NoSpace,
        /// Encoder error
        EncodeError,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsOutputSignal {
        #[inline]
        fn clone(&self) -> ObsOutputSignal {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsOutputSignal {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsOutputSignal {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsOutputSignal::Success => "Success",
                    ObsOutputSignal::BadPath => "BadPath",
                    ObsOutputSignal::ConnectFailed => "ConnectFailed",
                    ObsOutputSignal::InvalidStream => "InvalidStream",
                    ObsOutputSignal::Error => "Error",
                    ObsOutputSignal::Disconnected => "Disconnected",
                    ObsOutputSignal::Unsupported => "Unsupported",
                    ObsOutputSignal::NoSpace => "NoSpace",
                    ObsOutputSignal::EncodeError => "EncodeError",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsOutputSignal {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsOutputSignal {
        #[inline]
        fn eq(&self, other: &ObsOutputSignal) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsOutputSignal {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl fmt::Display for ObsOutputSignal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                ObsOutputSignal::Success => "Success",
                ObsOutputSignal::BadPath => "Bad Path",
                ObsOutputSignal::ConnectFailed => "Connect Failed",
                ObsOutputSignal::InvalidStream => "Invalid Stream",
                ObsOutputSignal::Error => "Error",
                ObsOutputSignal::Disconnected => "Disconnected",
                ObsOutputSignal::Unsupported => "Unsupported",
                ObsOutputSignal::NoSpace => "No Space",
                ObsOutputSignal::EncodeError => "Encode Error",
            };
            f.write_fmt(format_args!("{0}", s))
        }
    }
    impl Into<i32> for ObsOutputSignal {
        fn into(self) -> i32 {
            match self {
                ObsOutputSignal::Success => libobs::OBS_OUTPUT_SUCCESS as i32,
                ObsOutputSignal::BadPath => libobs::OBS_OUTPUT_BAD_PATH,
                ObsOutputSignal::ConnectFailed => libobs::OBS_OUTPUT_CONNECT_FAILED,
                ObsOutputSignal::InvalidStream => libobs::OBS_OUTPUT_INVALID_STREAM,
                ObsOutputSignal::Error => libobs::OBS_OUTPUT_ERROR,
                ObsOutputSignal::Disconnected => libobs::OBS_OUTPUT_DISCONNECTED,
                ObsOutputSignal::Unsupported => libobs::OBS_OUTPUT_UNSUPPORTED,
                ObsOutputSignal::NoSpace => libobs::OBS_OUTPUT_NO_SPACE,
                ObsOutputSignal::EncodeError => libobs::OBS_OUTPUT_ENCODE_ERROR,
            }
        }
    }
    impl TryFrom<i32> for ObsOutputSignal {
        type Error = &'static str;
        fn try_from(
            value: i32,
        ) -> Result<Self, <ObsOutputSignal as TryFrom<i32>>::Error> {
            match value {
                x if x == libobs::OBS_OUTPUT_SUCCESS as i32 => {
                    Ok(ObsOutputSignal::Success)
                }
                x if x == libobs::OBS_OUTPUT_BAD_PATH => Ok(ObsOutputSignal::BadPath),
                x if x == libobs::OBS_OUTPUT_CONNECT_FAILED => {
                    Ok(ObsOutputSignal::ConnectFailed)
                }
                x if x == libobs::OBS_OUTPUT_INVALID_STREAM => {
                    Ok(ObsOutputSignal::InvalidStream)
                }
                x if x == libobs::OBS_OUTPUT_ERROR => Ok(ObsOutputSignal::Error),
                x if x == libobs::OBS_OUTPUT_DISCONNECTED => {
                    Ok(ObsOutputSignal::Disconnected)
                }
                x if x == libobs::OBS_OUTPUT_UNSUPPORTED => {
                    Ok(ObsOutputSignal::Unsupported)
                }
                x if x == libobs::OBS_OUTPUT_NO_SPACE => Ok(ObsOutputSignal::NoSpace),
                x if x == libobs::OBS_OUTPUT_ENCODE_ERROR => {
                    Ok(ObsOutputSignal::EncodeError)
                }
                _ => Err("Invalid value"),
            }
        }
    }
    #[repr(i32)]
    pub enum ObsEncoderType {
        Video = libobs::obs_encoder_type_OBS_ENCODER_VIDEO,
        Audio = libobs::obs_encoder_type_OBS_ENCODER_AUDIO,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsEncoderType {
        #[inline]
        fn clone(&self) -> ObsEncoderType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsEncoderType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsEncoderType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsEncoderType::Video => "Video",
                    ObsEncoderType::Audio => "Audio",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsEncoderType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsEncoderType {
        #[inline]
        fn eq(&self, other: &ObsEncoderType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsEncoderType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsEncoderType {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsEncoderType::Video as i64 {
                    ::core::option::Option::Some(ObsEncoderType::Video)
                } else if n == ObsEncoderType::Audio as i64 {
                    ::core::option::Option::Some(ObsEncoderType::Audio)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsEncoderType {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsEncoderType::Video => ObsEncoderType::Video as i64,
                        ObsEncoderType::Audio => ObsEncoderType::Audio as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    #[repr(i32)]
    pub enum ObsLogLevel {
        Error = libobs::LOG_ERROR,
        Warning = libobs::LOG_WARNING,
        Info = libobs::LOG_INFO,
        Debug = libobs::LOG_DEBUG,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObsLogLevel {
        #[inline]
        fn clone(&self) -> ObsLogLevel {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObsLogLevel {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ObsLogLevel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ObsLogLevel::Error => "Error",
                    ObsLogLevel::Warning => "Warning",
                    ObsLogLevel::Info => "Info",
                    ObsLogLevel::Debug => "Debug",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObsLogLevel {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObsLogLevel {
        #[inline]
        fn eq(&self, other: &ObsLogLevel) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObsLogLevel {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::FromPrimitive for ObsLogLevel {
            #[allow(trivial_numeric_casts)]
            #[inline]
            fn from_i64(n: i64) -> ::core::option::Option<Self> {
                if n == ObsLogLevel::Error as i64 {
                    ::core::option::Option::Some(ObsLogLevel::Error)
                } else if n == ObsLogLevel::Warning as i64 {
                    ::core::option::Option::Some(ObsLogLevel::Warning)
                } else if n == ObsLogLevel::Info as i64 {
                    ::core::option::Option::Some(ObsLogLevel::Info)
                } else if n == ObsLogLevel::Debug as i64 {
                    ::core::option::Option::Some(ObsLogLevel::Debug)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            fn from_u64(n: u64) -> ::core::option::Option<Self> {
                Self::from_i64(n as i64)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_qualifications)]
    const _: () = {
        #[allow(clippy::useless_attribute)]
        #[allow(rust_2018_idioms)]
        extern crate num_traits as _num_traits;
        impl _num_traits::ToPrimitive for ObsLogLevel {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn to_i64(&self) -> ::core::option::Option<i64> {
                ::core::option::Option::Some(
                    match *self {
                        ObsLogLevel::Error => ObsLogLevel::Error as i64,
                        ObsLogLevel::Warning => ObsLogLevel::Warning as i64,
                        ObsLogLevel::Info => ObsLogLevel::Info as i64,
                        ObsLogLevel::Debug => ObsLogLevel::Debug as i64,
                    },
                )
            }
            #[inline]
            fn to_u64(&self) -> ::core::option::Option<u64> {
                self.to_i64().map(|x| x as u64)
            }
        }
    };
    impl Display for ObsLogLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:?}", self))
        }
    }
    #[cfg(feature = "color-logger")]
    impl ObsLogLevel {
        pub fn colorize(&self, s: &str) -> String {
            use colored::Colorize;
            match self {
                ObsLogLevel::Error => s.on_red().to_string(),
                ObsLogLevel::Warning => s.yellow().to_string(),
                ObsLogLevel::Info => s.green().bold().to_string(),
                ObsLogLevel::Debug => s.blue().to_string(),
            }
        }
    }
}
