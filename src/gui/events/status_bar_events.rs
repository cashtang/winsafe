use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::structs::NMMOUSE;

/// Exposes status bar
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
pub struct StatusBarEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl StatusBarEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> StatusBarEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	nfy_event_p_bool! { nm_click, co::NM::CLICK, NMMOUSE,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-status-bar)
		/// notification.
	}

	nfy_event_p_bool! { nm_dbl_clk, co::NM::DBLCLK, NMMOUSE,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-status-bar)
		/// notification.
	}

	nfy_event_p_bool! { nm_rclick, co::NM::RCLICK, NMMOUSE,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-status-bar)
		/// notification.
	}

	nfy_event_p_bool! { nm_r_dbl_clk, co::NM::RDBLCLK, NMMOUSE,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-status-bar)
		/// notification.
	}

	nfy_event! { sbn_simple_mode_change, co::NM::SBN_SIMPLEMODECHANGE,
		/// [`SBN_SIMPLEMODECHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/sbn-simplemodechange)
		/// notification.
	}
}