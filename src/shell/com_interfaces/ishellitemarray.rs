#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCVOID, PVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::ole_IUnknown;
use crate::shell::decl::IShellItem;
use crate::shell::iterators::IshellitemarrayIter;
use crate::vt::IUnknownVT;

/// [`IShellItemArray`](crate::IShellItemArray) virtual table.
#[repr(C)]
pub struct IShellItemArrayVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(COMPTR, PVOID, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStore: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyDescriptionList: fn(COMPTR, PVOID, PCVOID, *mut COMPTR) -> HRES,
	pub GetAttributes: fn(COMPTR, u32, u32, PVOID) -> HRES,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetItemAt: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub EnumItems: fn(COMPTR, *mut PVOID) -> HRES,
}

com_interface! { IShellItemArray: "b63ea76d-1f85-456f-a19c-48159efa858b";
	/// [`IShellItemArray`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
	/// COM interface over [`IShellItemArrayVT`](crate::vt::IShellItemArrayVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IShellItemArray for IShellItemArray {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItemArray`](crate::IShellItemArray).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItemArray: ole_IUnknown {
	/// Returns an iterator over the [`IShellItem`](crate::IShellItem) elements
	/// by calling
	/// [`IShellItemArray::GetCount`](crate::prelude::shell_IShellItemArray::GetCount)
	/// and
	/// [`IShellItemArray::GetItemAt`](crate::prelude::shell_IShellItemArray::GetItemAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// Iterating over the [`IShellItem`](crate::IShellItem) objects:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IShellItemArray};
	///
	/// let ish_arr: IShellItemArray; // initialized somewhere
	/// # let ish_arr = unsafe { IShellItemArray::null() };
	///
	/// for ish_item in ish_arr.iter()? {
	///     let ish_item = ish_item?;
	///     println!("Path: {}",
	///         ish_item.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	///
	/// Collecting the file paths into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HrResult, IShellItemArray};
	///
	/// let ish_arr: IShellItemArray; // initialized somewhere
	/// # let ish_arr = unsafe { IShellItemArray::null() };
	///
	/// let paths = ish_arr.iter()?
	///     .map(|shi|
	///         shi.and_then(|shi|
	///             shi.GetDisplayName(co::SIGDN::FILESYSPATH),
	///         ),
	///     )
	///     .collect::<HrResult<Vec<_>>>()?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self,
	) -> HrResult<Box<dyn Iterator<Item = HrResult<IShellItem>> + '_>>
	{
		Ok(Box::new(IshellitemarrayIter::new(self)?))
	}

	/// [`IShellItemArray::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItemArrayVT>(self).GetCount)(self.ptr(), &mut count)
			},
		).map(|_| count)
	}

	/// [`IShellItemArray::GetItemAt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	///
	/// Prefer using
	/// [`IShellItemArrayT::iter`](crate::prelude::shell_IShellItemArray::iter).
	#[must_use]
	fn GetItemAt(&self, index: u32) -> HrResult<IShellItem> {
		let mut queried = unsafe { IShellItem::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItemArrayVT>(self).GetItemAt)(
					self.ptr(),
					index,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
