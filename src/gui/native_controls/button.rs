use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::resizer::{Horz, Vert};
use crate::gui::events::{ButtonEvents, EventsView, WindowEvents};
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{
	AsAny,
	Child,
	NativeControl,
	NativeControlEvents,
	Parent,
	TextControl,
	Window,
};
use crate::handles::HWND;
use crate::msg::{bm, wm};
use crate::structs::{POINT, SIZE};

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button(Arc<Obj>);

struct Obj { // actual fields of Button
	base: BaseNativeControl,
	opts_id: OptsId<ButtonOpts>,
	events: ButtonEvents,
}

unsafe impl Send for Button {}

impl AsAny for Button {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for Button {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for Button {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for Button {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl NativeControlEvents<ButtonEvents> for Button {
	fn on(&self) -> &ButtonEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl TextControl for Button {}

impl Button {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: ButtonOpts) -> Button {
		let opts = ButtonOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: ButtonEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| { self2.create(horz, vert)?; Ok(0) }
		});
		new_self
	}

	/// Instantiates a new `Button` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> Button
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ButtonEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| { self2.create(horz_resize, vert_resize)?; Ok(true) }
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = SIZE::new(opts.width as _, opts.height as _);
				multiply_dpi(Some(&mut pos), Some(&mut sz))?;

				let our_hwnd = self.0.base.create_window(
					"BUTTON", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Fires the click event for the button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn trigger_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Button`](crate::gui::Button) programmatically with
/// [`Button::new`](crate::gui::Button::new).
pub struct ButtonOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 80.
	pub width: u32,
	/// Control height, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to current system DPI.
	///
	/// Defaults to 23.
	pub height: u32,
	/// Button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::PUSHBUTTON`.
	///
	/// Suggestions:
	/// * replace with `BS::DEFPUSHBUTTON` for the default button of the window;
	/// * add `BS::NOTIFY` to receive notifications other than the simple click.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
}

impl Default for ButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			width: 80,
			height: 23,
			button_style: co::BS::PUSHBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
		}
	}
}

impl ButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
