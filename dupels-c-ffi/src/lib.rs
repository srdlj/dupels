use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::path::PathBuf;
use std::ptr;

use dupels_lib::{DupeLs, DupeLsConfig};

/// Error codes for the C API
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum DupelsError {
    Success = 0,
    NullPointer = 1,
    InvalidPath = 2,
    InvalidUtf8 = 3,
    IoError = 4,
    OutOfMemory = 5,
}

/// Config struct
#[repr(C)]
pub struct DupelsConfig {
    /// Base path to search
    pub base_path: *const c_char,
    /// Include dot files (1 = true, 0 = false)
    pub track_dot_files: c_uchar,
    /// Search recursively (1 = true, 0 = false)
    pub recursive: c_uchar,
    /// Maximum depth for recursive search
    pub depth: c_int,
    /// Separator string for output groups
    pub separator: *const c_char,
    /// Omit single-file groups (1 = true, 0 = false)
    pub omit: c_uchar,
    /// Maximum number of threads (0 = auto)
    pub max_threads: c_int,
}

/// Result struct
#[repr(C)]
pub struct DupelsResult {
    /// Array of strings
    pub output: *mut *mut c_char,
    /// Number of strings in the output array
    pub count: c_int,
    /// Error code
    pub error: DupelsError,
}

/// Opaque handle for dupels instance
pub struct DupelsHandle {
    inner: DupeLs,
}

impl Default for DupelsConfig {
    fn default() -> Self {
        Self {
            base_path: ptr::null(),
            track_dot_files: 0,
            recursive: 1,
            depth: 2,
            separator: ptr::null(),
            omit: 0,
            max_threads: 0,
        }
    }
}

/// Convert C config to Rust config
unsafe fn c_config_to_rust(config: *const DupelsConfig) -> Result<DupeLsConfig, DupelsError> {
    if config.is_null() {
        return Err(DupelsError::NullPointer);
    }

    let config = &*config;
    
    let base_path = if config.base_path.is_null() {
        None
    } else {
        let path_str = CStr::from_ptr(config.base_path)
            .to_str()
            .map_err(|_| DupelsError::InvalidUtf8)?;
        Some(PathBuf::from(path_str))
    };

    let separator = if config.separator.is_null() {
        "===".to_string()
    } else {
        CStr::from_ptr(config.separator)
            .to_str()
            .map_err(|_| DupelsError::InvalidUtf8)?
            .to_string()
    };

    let max_threads = if config.max_threads <= 0 {
        None
    } else {
        Some(config.max_threads as usize)
    };

    Ok(DupeLsConfig {
        base_path,
        track_dot_files: config.track_dot_files != 0,
        recursive: config.recursive != 0,
        depth: config.depth as usize,
        seperator: separator,
        omit: config.omit != 0,
        max_threads,
    })
}

/// Create a new dupels instance
/// 
/// # Arguments
/// * `config` - Configuration for the dupels instance
/// 
/// # Returns
/// * Pointer to the dupels handle, or null on error
#[no_mangle]
pub unsafe extern "C" fn dupels_new(config: *const DupelsConfig) -> *mut DupelsHandle {
    match c_config_to_rust(config) {
        Ok(rust_config) => {
            let dupels = DupeLs::new(rust_config);
            let handle = Box::new(DupelsHandle { inner: dupels });
            Box::into_raw(handle)
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Free a dupels instance
/// 
/// # Arguments
/// * `handle` - Handle to the dupels instance
#[no_mangle]
pub unsafe extern "C" fn dupels_free(handle: *mut DupelsHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// Run duplicate detection
/// 
/// # Arguments
/// * `handle` - Handle to the dupels instance
/// 
/// # Returns
/// * Error code
#[no_mangle]
pub unsafe extern "C" fn dupels_parse(handle: *mut DupelsHandle) -> DupelsError {
    if handle.is_null() {
        return DupelsError::NullPointer;
    }

    let handle = &mut *handle;
    handle.inner.parse();
    DupelsError::Success
}

/// Get results as an array of strings
/// 
/// # Arguments
/// * `handle` - Handle to the dupels instance
/// * `result` - Pointer to result structure to fill
/// 
/// # Returns
/// * Error code
#[no_mangle]
pub unsafe extern "C" fn dupels_get_results(
    handle: *mut DupelsHandle,
    result: *mut DupelsResult,
) -> DupelsError {
    if handle.is_null() || result.is_null() {
        return DupelsError::NullPointer;
    }

    let handle = &*handle;
    let result = &mut *result;

    let output_vec = handle.inner.get_output_vec();
    let count = output_vec.len();

    if count == 0 {
        result.output = ptr::null_mut();
        result.count = 0;
        result.error = DupelsError::Success;
        return DupelsError::Success;
    }

    // Allocate array of char pointers
    let layout = std::alloc::Layout::array::<*mut c_char>(count).unwrap();
    let array_ptr = std::alloc::alloc(layout) as *mut *mut c_char;
    
    if array_ptr.is_null() {
        result.error = DupelsError::OutOfMemory;
        return DupelsError::OutOfMemory;
    }

    // Convert each string and store pointer
    for (i, line) in output_vec.into_iter().enumerate() {
        match CString::new(line) {
            Ok(c_string) => {
                *array_ptr.add(i) = c_string.into_raw();
            }
            Err(_) => {
                // Clean up already allocated strings
                for j in 0..i {
                    let _ = CString::from_raw(*array_ptr.add(j));
                }
                std::alloc::dealloc(array_ptr as *mut u8, layout);
                result.error = DupelsError::InvalidUtf8;
                return DupelsError::InvalidUtf8;
            }
        }
    }

    result.output = array_ptr;
    result.count = count as c_int;
    result.error = DupelsError::Success;
    DupelsError::Success
}

/// Get results as a single string
/// 
/// # Arguments
/// * `handle` - Handle to the dupels instance
/// 
/// # Returns
/// * Pointer to null-terminated string (caller must free), or null on error
#[no_mangle]
pub unsafe extern "C" fn dupels_get_output_string(handle: *mut DupelsHandle) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }

    let handle = &*handle;
    let output = handle.inner.get_output_string();
    
    match CString::new(output) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string returned by dupels_get_output_string
/// 
/// # Arguments
/// * `string` - String to free
#[no_mangle]
pub unsafe extern "C" fn dupels_free_string(string: *mut c_char) {
    if !string.is_null() {
        let _ = CString::from_raw(string);
    }
}

/// Free results returned by dupels_get_results
/// 
/// # Arguments
/// * `result` - Result structure to free
#[no_mangle]
pub unsafe extern "C" fn dupels_free_results(result: *mut DupelsResult) {
    if result.is_null() {
        return;
    }

    let result = &mut *result;
    
    if !result.output.is_null() && result.count > 0 {
        // Free each string
        for i in 0..result.count {
            let string_ptr = *result.output.add(i as usize);
            if !string_ptr.is_null() {
                let _ = CString::from_raw(string_ptr);
            }
        }
        
        // Free the array of pointers
        let layout = std::alloc::Layout::array::<*mut c_char>(result.count as usize).unwrap();
        std::alloc::dealloc(result.output as *mut u8, layout);
    }

    result.output = ptr::null_mut();
    result.count = 0;
}

/// Simple convenience function to find duplicates in a directory
/// 
/// # Arguments
/// * `path` - Path to search (null-terminated string)
/// * `recursive` - Whether to search recursively (1 = true, 0 = false)
/// * `include_hidden` - Whether to include dot files (1 = true, 0 = false)
/// 
/// # Returns
/// * Pointer to null-terminated string with results (caller must free), or null on error
#[no_mangle]
pub unsafe extern "C" fn dupels_find_duplicates_simple(
    path: *const c_char,
    recursive: c_uchar,
    include_hidden: c_uchar,
) -> *mut c_char {
    let config = DupelsConfig {
        base_path: path,
        track_dot_files: include_hidden,
        recursive,
        depth: if recursive != 0 { 10 } else { 1 },
        separator: ptr::null(), // Will use default
        omit: 0,
        max_threads: 0, // Auto
    };

    let handle = dupels_new(&config);
    if handle.is_null() {
        return ptr::null_mut();
    }

    if dupels_parse(handle) != DupelsError::Success {
        dupels_free(handle);
        return ptr::null_mut();
    }

    let result = dupels_get_output_string(handle);
    dupels_free(handle);
    result
}
