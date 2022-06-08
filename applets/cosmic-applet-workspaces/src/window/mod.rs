// SPDX-License-Identifier: MPL-2.0-only

use crate::{fl, utils::Event, workspace_list::WorkspaceList};
use cascade::cascade;
use cosmic_panel_config::config::CosmicPanelConfig;
use gtk4::{
    gio,
    glib::{self, Object},
    prelude::*,
    subclass::prelude::*,
};
use tokio::sync::mpsc;

mod imp;

glib::wrapper! {
    pub struct CosmicWorkspacesWindow(ObjectSubclass<imp::CosmicWorkspacesWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl CosmicWorkspacesWindow {
    pub fn new(app: &gtk4::Application, tx: mpsc::Sender<Event>) -> Self {
        let self_: Self = Object::new(&[("application", app)])
            .expect("Failed to create `CosmicWorkspacesWindow`.");
        let imp = imp::CosmicWorkspacesWindow::from_instance(&self_);

        cascade! {
            &self_;
            ..set_width_request(1);
            ..set_height_request(1);
            ..set_decorated(false);
            ..set_resizable(false);
            ..set_title(Some(&fl!("cosmic-app-list")));
            ..add_css_class("transparent");
        };
        let config = CosmicPanelConfig::load_from_env().unwrap_or_default();

        let app_list = WorkspaceList::new(tx, config);
        self_.set_child(Some(&app_list));
        imp.inner.set(app_list).unwrap();

        self_
    }
}
