use crate::models::docking::hierarchy::dock_node::DockNode;

/// Validates and corrects any mistakes in tab logic.
impl DockNode {
    /// A top-level method that ensures each tab node has a valid active tab.
    pub fn run_active_tab_validation(&mut self) {
        Self::run_active_tab_validation_internal(self);
    }

    fn run_active_tab_validation_internal(node: &mut DockNode) {
        match node {
            DockNode::Split { children, .. } => {
                for child in children.iter_mut() {
                    Self::run_active_tab_validation_internal(&mut child.node);
                }
            }
            DockNode::Tab { tabs, active_tab_id, .. } => {
                // Tabs with 1 child are not supported. Clear the active tab id.
                // Additionally, there is no need to recurse, as nested tabs are not supported.
                if tabs.len() == 1 {
                    active_tab_id.clear();
                }

                // If active_tab_id is invalid or hidden, pick a new active tab.
                if !Self::is_current_active_tab_valid(tabs, active_tab_id) {
                    let new_id = Self::pick_first_visible_window_id(tabs);
                    if let Some(new_active_id) = new_id {
                        *active_tab_id = new_active_id;
                    } else {
                        active_tab_id.clear();
                    }
                }
            }
            DockNode::Window { .. } => {}
        }
    }

    /// Returns whether the current `active_tab_id` in a tab node is valid + visible.
    fn is_current_active_tab_valid(
        tabs: &[DockNode],
        active_tab_id: &str,
    ) -> bool {
        if active_tab_id.is_empty() {
            return false;
        }
        // Check if there's a visible window with the same ID
        tabs.iter().any(|child| match child {
            DockNode::Window {
                window_identifier, is_visible, ..
            } => window_identifier == active_tab_id && *is_visible,
            _ => false,
        })
    }

    /// Pick the first visible window's ID from the tab list, if any.
    fn pick_first_visible_window_id(tabs: &[DockNode]) -> Option<String> {
        for child in tabs {
            match child {
                DockNode::Window {
                    window_identifier, is_visible, ..
                } if *is_visible => {
                    return Some(window_identifier.clone());
                }
                _ => {}
            }
        }
        None
    }
}
