use std::{
    ffi::{c_void, OsStr},
    mem::{forget, ManuallyDrop},
    sync::Arc,
};

use windows::{
    core::{Error as WinError, HSTRING, PCWSTR},
    Win32::{
        Foundation::{GetLastError, BOOLEAN, ERROR_ALREADY_EXISTS, HANDLE},
        System::Threading::{
            CreateEventW, OpenEventW, RegisterWaitForSingleObject, SetEvent, EVENT_ALL_ACCESS,
            WT_EXECUTEDEFAULT,
        },
    },
};

pub struct CrossProcessAsyncEvent {
    /// Windows handle to the `Event`.
    handle: HANDLE,
    /// Callback for cleanup
    callback: Option<Arc<Box<dyn Fn()>>>,
}

impl CrossProcessAsyncEvent {
    /// Attempts to create a new windows `Event`.
    ///
    /// Also succeeds when `Event` already existed (technically throws an error, but succeeds at the same time).
    pub fn try_create(name: impl AsRef<OsStr>) -> Result<Self, WinError> {
        unsafe {
            let hstring = ManuallyDrop::new(HSTRING::from(name.as_ref()));
            let pcwstr = PCWSTR(hstring.as_wide().as_ptr());
            let handle = CreateEventW(None, false, false, pcwstr)?;
            if GetLastError() == ERROR_ALREADY_EXISTS {
                println!("Event already existed, opening it instead.");
            }
            ManuallyDrop::into_inner(hstring);
            Ok(Self {
                handle,
                callback: None,
            })
        }
    }

    /// Attempts to open an existing `Event`.
    ///
    /// Fails if it doesn't exist.
    pub fn try_open(name: impl AsRef<OsStr>) -> Result<Self, WinError> {
        unsafe {
            let hstring = ManuallyDrop::new(HSTRING::from(name.as_ref()));
            let pcwstr = PCWSTR(hstring.as_wide().as_ptr());
            let handle = OpenEventW(EVENT_ALL_ACCESS, false, pcwstr)?;
            ManuallyDrop::into_inner(hstring);
            Ok(Self {
                handle,
                callback: None,
            })
        }
    }

    /// Sets the event.
    pub fn wake(&self) -> bool {
        unsafe { SetEvent(self.handle).as_bool() }
    }

    /// Overly flexible callback wrapper.
    ///
    /// This allows us to pass a closure for easy testing.
    /// In a specific use case you'd pass something like an Atomic here and use a static function.
    ///
    /// Boolean parameter is ignored, because we aren't using timeouts.
    unsafe extern "system" fn callback_wrapper(callback_ptr: *mut c_void, _: BOOLEAN) {
        // Reverse casting from `*mut c_void`, which we were forced to use by `RegisterWaitForSingleObject`
        callback_ptr.cast::<&dyn Fn()>().read()();
    }

    /// Callback registration (separate because I'm lazy).
    ///
    /// It'd be much safer to work with generics (for the intermediate `*c_void` representation) and provide callback (or thread-safe data) during creation.
    pub fn register_callback<T>(&mut self, callback: T) -> bool
    where
        T: Fn() + 'static,
    {
        // Handle to a `WaitObject`, not sure what WE need it for, but it's required by the windows function call.
        let mut wait_handle = HANDLE::default();
        // Callback function is wrapped in `Arc<Box<_>>` before casting into `*const c_void`
        // The reasons for that are the following:
        // - Trait objects cannot be turned into pointers so we use `Box<Fn()>`
        // - `Box<Fn()>` has address of `0x1` so we use another layer `Arc<Box<Fn()>>`
        let callback: Arc<Box<dyn Fn()>> = Arc::new(Box::new(callback));
        self.callback = Some(callback.clone());
        // We leak memory here, this never gets cleaned up
        let callback_ptr = Arc::into_raw(callback) as *const c_void;
        // This also leaks memory, windows requires us to remove callbacks
        let res = unsafe {
            RegisterWaitForSingleObject(
                &mut wait_handle as *mut HANDLE,
                self.handle,
                Some(Self::callback_wrapper),
                Some(callback_ptr),
                u32::MAX,
                WT_EXECUTEDEFAULT,
            )
        }
        .as_bool();

        res
    }
}
