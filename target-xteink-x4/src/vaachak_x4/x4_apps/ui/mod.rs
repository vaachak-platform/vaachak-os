// ui re-exports: kernel primitives + app-side font-dependent widgets
//
// kernel ui (Region, Alignment, StackFmt, statusbar constants) is
// re-exported from the Vaachak-owned X4 kernel module; font-dependent widgets (BitmapLabel,
// QuickMenu, ButtonFeedback) come from apps::widgets

// kernel-side primitives
pub use crate::vaachak_x4::x4_kernel::ui::stack_fmt;
pub use crate::vaachak_x4::x4_kernel::ui::*;

// app-side font-dependent widgets
pub use crate::vaachak_x4::x4_apps::apps::widgets::QuickMenu;
pub use crate::vaachak_x4::x4_apps::apps::widgets::bitmap_label::{BitmapDynLabel, BitmapLabel};
pub use crate::vaachak_x4::x4_apps::apps::widgets::button_feedback::{
    BUTTON_BAR_H, ButtonFeedback,
};
pub use crate::vaachak_x4::x4_apps::apps::widgets::list::ListSelection;
pub use crate::vaachak_x4::x4_apps::apps::widgets::quick_menu;
pub use crate::vaachak_x4::x4_apps::apps::widgets::selectable_row::{
    draw_selection, draw_selection_if_visible, selection_fg,
};
