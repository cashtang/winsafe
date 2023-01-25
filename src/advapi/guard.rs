use std::ops::Deref;

use crate::advapi;
use crate::advapi::decl::HKEY;
use crate::prelude::{advapi_Hkey, Handle};

/// RAII implementation for [`HKEY`](crate::HKEY) which automatically calls
/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
/// when the object goes out of scope.
pub struct HkeyGuard {
	pub(crate) hkey: HKEY,
}

impl Drop for HkeyGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hkey.as_opt() {
			if h.0 < HKEY::CLASSES_ROOT.0 || h.0 > HKEY::PERFORMANCE_NLSTEXT.0 { // guard predefined keys
				unsafe { advapi::ffi::RegCloseKey(h.as_ptr()); } // ignore errors
			}
		}
	}
}

impl Deref for HkeyGuard {
	type Target = HKEY;

	fn deref(&self) -> &Self::Target {
		&self.hkey
	}
}