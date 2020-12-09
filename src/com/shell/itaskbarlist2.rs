#![allow(non_snake_case)]

use crate::{HWND, IID};
use crate::{PPVtbl, Vtbl};
use crate::co::ERROR;
use crate::ffi::HANDLE;
use crate::shell::{ITaskbarList, ITaskbarListVtbl};

/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
#[repr(C)]
pub struct ITaskbarList2Vtbl {
	iTaskbarListVtbl: ITaskbarListVtbl,
	MarkFullscreenWindow: fn(PPVtbl<Self>, HANDLE, u32) -> u32,
}

impl Vtbl for ITaskbarList2Vtbl {
	fn IID() -> IID {
		IID::new(0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317)
	}
}

//------------------------------------------------------------------------------

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
///
/// # Examples
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// let mut obj: shell::ITaskbarList2 = CoCreateInstance(
///   &shell::clsid::TaskbarList,
///   None,
///   co::CLSCTX::INPROC_SERVER,
/// );
/// ```
pub struct ITaskbarList2 {
	/// Methods of base interface
	/// [`ITaskbarList`](crate::shell::ITaskbarList).
	pub ITaskbarList: ITaskbarList,
}

impl From<PPVtbl<ITaskbarList2Vtbl>> for ITaskbarList2 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl<ITaskbarList2Vtbl>) -> Self {
		Self {
			ITaskbarList: ITaskbarList::from(ppv as PPVtbl<ITaskbarListVtbl>),
		}
	}
}

impl ITaskbarList2 {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	pub fn MarkFullscreenWindow(
		&self, hwnd: HWND, fFullscreen: bool) -> Result<(), ERROR>
	{
		unsafe {
			let ppv = self.ITaskbarList.IUnknown.ppv::<ITaskbarList2Vtbl>();
			match ERROR::from(
				((*(*ppv)).MarkFullscreenWindow)(
					ppv, hwnd.as_ptr(), fFullscreen as u32,
				),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}