use gettextrs::gettext;

use crate::ui::rebuild::rebuild_dialog::RebuildStatus;

pub fn gt_status_msg(status: RebuildStatus) -> Vec<String> {
    return match status {
        RebuildStatus::Building => vec![
            gettext("Rebuilding"),
            gettext("This may take a few minutes."),
        ],
        RebuildStatus::Success => vec![gettext("Done!"), gettext("All changes have applied!")],
        RebuildStatus::Error => vec![
            gettext("Error!"),
            gettext("Error encountered during rebuild process."),
        ],
    };
}
