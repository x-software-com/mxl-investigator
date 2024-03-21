use once_cell::sync::OnceCell;
use std::path::PathBuf;

#[allow(dead_code)]
pub(crate) const SUPPORT_EMAIL: &str = "support@x-software.com";

static PROJECT_DATA_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn init(project_data_dir: PathBuf) {
    PROJECT_DATA_DIR.set(project_data_dir).expect("Already initialized");
    crate::localization::init();
    #[cfg(any(feature = "create_report_dialog", feature = "problem_report_dialog"))]
    {
        relm4_icons::initialize_icons();
        mxl_relm4_components::init();
    }
}

pub(crate) fn get_data_dir() -> &'static PathBuf {
    PROJECT_DATA_DIR.get().expect("Need to be initialized")
}

#[cfg(feature = "with_test")]
pub fn init_test() {
    use once_cell::sync::Lazy;
    use tempfile::TempDir;

    static TMP_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().expect("Failed create tmp directory"));

    init(TMP_DIR.path().to_path_buf());
}
