use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::co;
use crate::gui::controls::native_control_base::NativeControlBase;
use crate::gui::controls::poly_opts::PolyOpts;
use crate::gui::events::{EditEvents, MsgEvents};
use crate::gui::globals::{auto_ctrl_id, ui_font};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::WmSetFont;
use crate::structs::{POINT, SIZE};

/// Native
/// [edit](https://docs.microsoft.com/en-us/windows/win32/controls/about-edit-controls)
/// control.
#[derive(Clone)]
pub struct Edit {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of Edit
	base: NativeControlBase,
	poly_opts: PolyOpts<EditOpts>,
	parent_events: EditEvents,
}

unsafe impl Send for Edit {}
unsafe impl Sync for Edit {}

cref_mref!(Edit);

impl Child for Edit {
	fn create(&self) -> Result<(), co::ERROR> {
		match &self.cref().poly_opts {
			PolyOpts::Wnd(opts) => {
				let our_hwnd = self.mref().base.create_window( // may panic
					"EDIT", Some(&opts.text), opts.pos,
					SIZE{ cx: opts.width as i32, cy: opts.height as i32 },
					opts.ctrl_id,
					opts.ex_window_style,
					opts.window_style | opts.edit_style.into(),
				)?;

				our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
				Ok(())
			},
			PolyOpts::Dlg(ctrl_id) => {
				self.mref().base.create_dlg(*ctrl_id) // may panic
					.map(|_| ())
			},
		}
	}
}

impl Edit {
	/// Instantiates a new `Edit` object, to be created on the parent window with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: EditOpts) -> Edit {
		let mut opts = opts;
		opts.define_ctrl_id();
		let ctrl_id = opts.ctrl_id;

		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent),
					poly_opts: PolyOpts::Wnd(opts),
					parent_events: EditEvents::new(parent, ctrl_id),
				},
			)),
		}
	}

	/// Instantiates a new `Edit` object, to be assigned to the parent dialog
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Edit {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new(parent),
					poly_opts: PolyOpts::Dlg(ctrl_id),
					parent_events: EditEvents::new(parent, ctrl_id),
				},
			)),
		}
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		match &self.cref().poly_opts {
			PolyOpts::Wnd(opts) => opts.ctrl_id,
			PolyOpts::Dlg(ctrl_id) => *ctrl_id,
		}
	}

	/// Exposes the edit events.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &EditEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control is created.");
		} else if self.cref().base.is_parent_created() {
			panic!("Cannot add events after the parent window is created.");
		}
		&self.cref().parent_events
	}

	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on_subclass(&self) -> &MsgEvents {
		self.cref().base.on_subclass()
	}
}

//------------------------------------------------------------------------------

/// Options for [`Edit::new`](crate::gui::Edit::new).
pub struct EditOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 0x0.
	///
	/// To vertically align side-by-side with a button, add 1 to `y`. That's
	/// necessary because default button height is 23, while edit is 21.
	pub pos: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 100.
	pub width: u32,
	/// Control height, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to 21.
	///
	/// You should change the default height only in a multi-line edit.
	pub height: u32,
	/// Edit styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `ES::AUTOHSCROLL | ES::NOHIDESEL`.
	///
	/// Suggestions:
	/// * add `ES::PASSWORD` for a password input;
	/// * add `ES::NUMBER` to accept only numbers;
	/// * replace with `ES::MULTILINE | ES:WANTRETURN | ES:AUTOVSCROLL | ES::NOHIDESEL` for a multi-line edit.
	pub edit_style: co::ES,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for EditOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			pos: POINT { x: 0, y: 0 },
			width: 100,
			height: 21,
			edit_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
		}
	}
}

impl EditOpts {
	fn define_ctrl_id(&mut self) {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
	}
}