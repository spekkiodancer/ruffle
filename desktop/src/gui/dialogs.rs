mod bookmarks_dialog;
mod open_dialog;
mod preferences_dialog;

use crate::custom_event::RuffleEvent;
use crate::player::PlayerOptions;
use crate::preferences::GlobalPreferences;
use bookmarks_dialog::{BookmarkAddDialog, BookmarksDialog};
use open_dialog::OpenDialog;
use preferences_dialog::PreferencesDialog;
use unic_langid::LanguageIdentifier;
use url::Url;
use winit::event_loop::EventLoopProxy;

pub struct Dialogs {
    preferences_dialog: Option<PreferencesDialog>,
    bookmarks_dialog: Option<BookmarksDialog>,
    bookmark_add_dialog: Option<BookmarkAddDialog>,

    open_dialog: OpenDialog,
    is_open_dialog_visible: bool,
    preferences: GlobalPreferences,
}

impl Dialogs {
    pub fn new(
        preferences: GlobalPreferences,
        player_options: PlayerOptions,
        default_path: Option<Url>,
        event_loop: EventLoopProxy<RuffleEvent>,
    ) -> Self {
        Self {
            preferences_dialog: None,
            bookmarks_dialog: None,
            bookmark_add_dialog: None,

            open_dialog: OpenDialog::new(player_options, default_path, event_loop),
            is_open_dialog_visible: false,

            preferences,
        }
    }

    pub fn recreate_open_dialog(
        &mut self,
        opt: PlayerOptions,
        url: Option<Url>,
        event_loop: EventLoopProxy<RuffleEvent>,
    ) {
        self.is_open_dialog_visible = false;
        self.open_dialog = OpenDialog::new(opt, url, event_loop);
    }

    pub fn open_file_advanced(&mut self) {
        self.is_open_dialog_visible = true;
    }

    pub fn open_preferences(&mut self) {
        self.preferences_dialog = Some(PreferencesDialog::new(self.preferences.clone()));
    }

    pub fn open_bookmarks(&mut self) {
        self.bookmarks_dialog = Some(BookmarksDialog::new(self.preferences.clone()));
    }

    pub fn open_add_bookmark(&mut self, initial_url: Option<url::Url>) {
        self.bookmark_add_dialog = Some(BookmarkAddDialog::new(
            self.preferences.clone(),
            initial_url,
        ))
    }

    pub fn show(&mut self, locale: &LanguageIdentifier, egui_ctx: &egui::Context) {
        self.open_dialog(locale, egui_ctx);
        self.preferences_dialog(locale, egui_ctx);
        self.bookmarks_dialog(locale, egui_ctx);
        self.bookmark_add_dialog(locale, egui_ctx);
    }

    fn open_dialog(&mut self, locale: &LanguageIdentifier, egui_ctx: &egui::Context) {
        if self.is_open_dialog_visible {
            let keep_open = self.open_dialog.show(locale, egui_ctx);
            self.is_open_dialog_visible = keep_open;
        }
    }

    fn preferences_dialog(&mut self, locale: &LanguageIdentifier, egui_ctx: &egui::Context) {
        let keep_open = if let Some(dialog) = &mut self.preferences_dialog {
            dialog.show(locale, egui_ctx)
        } else {
            true
        };
        if !keep_open {
            self.preferences_dialog = None;
        }
    }

    fn bookmarks_dialog(&mut self, locale: &LanguageIdentifier, egui_ctx: &egui::Context) {
        let keep_open = if let Some(dialog) = &mut self.bookmarks_dialog {
            dialog.show(locale, egui_ctx)
        } else {
            true
        };
        if !keep_open {
            self.bookmarks_dialog = None;
        }
    }

    fn bookmark_add_dialog(&mut self, locale: &LanguageIdentifier, egui_ctx: &egui::Context) {
        let keep_open = if let Some(dialog) = &mut self.bookmark_add_dialog {
            dialog.show(locale, egui_ctx)
        } else {
            true
        };
        if !keep_open {
            self.bookmark_add_dialog = None;
        }
    }
}