#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID, PVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IDXGIObject`](crate::IDXGIObject) virtual table.
#[repr(C)]
pub struct IDXGIObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub SetPrivateData: fn(COMPTR, PCVOID, u32, PVOID) -> HRES,
	pub SetPrivateDataInterface: fn(COMPTR, PCVOID, COMPTR) -> HRES,
	pub GetPrivateData: fn(COMPTR, PCVOID, *mut u32, PVOID) -> HRES,
	pub GetParent: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
}

com_interface! { IDXGIObject: "aec22fb8-76f3-4639-9be0-28eb43a67a2e";
	/// [`IDXGIObject`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiobject)
	/// COM interface over [`IDXGIObjectVT`](crate::vt::IDXGIObjectVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIObject {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIObject`](crate::IDXGIObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIObject: ole_IUnknown {
	/// [`IDXGIObject::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-getparent)
	/// method.
	#[must_use]
	fn GetParent<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IUnknownVT>(self).QueryInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IDXGIObject::SetPrivateDataInterface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedatainterface)
	/// method.
	fn SetPrivateDataInterface<T>(&self, interface: &T) -> HrResult<()>
		where T: ole_IUnknown,
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIObjectVT>(self).SetPrivateDataInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					interface.ptr(),
				)
			},
		)
	}
}
