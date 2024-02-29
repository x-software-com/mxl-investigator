use relm4::Controller;
use relm4_components::save_dialog::SaveDialog;

#[derive(Debug)]
pub struct CreateReportDialogInit {
    pub app_name: &'static str,
    pub binary_name: &'static str,
}

#[derive(Debug)]
pub struct CreateReportDialog {
    pub(super) app_name: &'static str,
    pub(super) binary_name: &'static str,
    pub(super) file_name: String,
    pub(super) file_chooser: Controller<SaveDialog>,
}

impl CreateReportDialog {}
