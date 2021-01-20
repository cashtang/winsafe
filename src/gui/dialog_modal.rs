use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::dialog_base::DialogBase;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::handles::HWND;

#[derive(Clone)]
pub struct DialogModal {
	base: Arc<DialogBase>,
}

impl Parent for DialogModal {
	fn hwnd_ref(&self) -> &HWND {
		self.base.hwnd_ref()
	}

	fn user_events_ref(&self) -> &MsgEvents {
		self.base.user_events_ref()
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		self.base.privileged_events_ref()
	}
}

impl DialogModal {
	pub fn new(parent: &dyn Parent, dialog_id: i32) -> DialogModal {
		let dlg = Self {
			base: Arc::new(
				DialogBase::new(Some(parent), dialog_id),
			),
		};
		dlg.default_message_handlers();
		dlg
	}

	pub fn show_modal(&self) -> WinResult<i32> {
		self.base.dialog_box_param()
	}

	fn center_in_parent(&self) -> WinResult<()> {
		let rc = self.hwnd_ref().GetWindowRect().unwrap();
		let rc_parent = self.hwnd_ref().GetParent()?.GetWindowRect()?;
		self.hwnd_ref().SetWindowPos(HwndPlace::None,
			rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
			rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
			0, 0, co::SWP::NOSIZE | co::SWP::NOZORDER)?;
		Ok(())
	}

	fn default_message_handlers(&self) {
		self.privileged_events_ref().wm_init_dialog({
			let self2 = self.clone();
			move |_| { self2.center_in_parent().unwrap(); true }
		});

		self.user_events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.hwnd_ref().EndDialog(
					u16::from(co::DLGID::CANCEL) as isize,
				).ok();
			}
		});
	}
}