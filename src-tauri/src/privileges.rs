use windows_sys::Win32::Foundation::{GetLastError, LUID};
use windows_sys::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
    TOKEN_QUERY, SE_PRIVILEGE_ENABLED, LUID_AND_ATTRIBUTES,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn enable_se_restore_privilege() -> Result<(), String> {
    unsafe {
        let mut token_handle = 0;
        let process_handle = GetCurrentProcess();
        
        if OpenProcessToken(process_handle, TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token_handle) == 0 {
            return Err(format!("OpenProcessToken failed: {}", GetLastError()));
        }

        let mut luid: LUID = std::mem::zeroed();
        let privilege_name: Vec<u16> = OsStr::new("SeRestorePrivilege")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        if LookupPrivilegeValueW(std::ptr::null(), privilege_name.as_ptr(), &mut luid) == 0 {
            return Err(format!("LookupPrivilegeValueW failed: {}", GetLastError()));
        }

        let mut token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        if AdjustTokenPrivileges(
            token_handle,
            0,
            &mut token_privileges,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ) == 0
        {
            return Err(format!("AdjustTokenPrivileges failed: {}", GetLastError()));
        }
        
        // Even if AdjustTokenPrivileges returns non-zero, we should check GetLastError for ERROR_NOT_ALL_ASSIGNED.
        // However, for this specific case, if we proceed, it might work if the user is Admin.
        
        let err = GetLastError();
        if err != 0 {
             // 1300 = ERROR_NOT_ALL_ASSIGNED
             if err == 1300 {
                 return Err("雖然嘗試啟用權限，但失敗了 (ERROR_NOT_ALL_ASSIGNED)。請確認您是真的以系統管理員身分執行程式。".to_string());
             }
        }
    }

    Ok(())
}
