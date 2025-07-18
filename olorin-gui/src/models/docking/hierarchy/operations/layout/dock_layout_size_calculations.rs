use crate::models::docking::hierarchy::dock_layout::DockLayout;
use crate::models::docking::hierarchy::dock_node::DockNode;
use crate::models::docking::hierarchy::types::dock_split_child::DockSplitChild;
use crate::models::docking::hierarchy::types::dock_split_direction::DockSplitDirection;

impl DockLayout {
    /// Finds the bounding rectangle of the specified window ID. Returns None if not found or not visible.
    pub fn find_window_rect(
        &self,
        node: &DockNode,
        window_id: &str,
    ) -> Option<(f32, f32, f32, f32)> {
        let path = node.find_path_to_window_id(window_id)?;
        self.find_node_rect(node, &path)
    }

    /// Find the bounding rectangle of a node at the given path.
    pub fn find_node_rect(
        &self,
        node: &DockNode,
        path: &[usize],
    ) -> Option<(f32, f32, f32, f32)> {
        let mut found = None;
        let mut path_stack = vec![];
        self.walk_with_layout_and_path(
            &node,
            0.0,
            0.0,
            self.get_available_width(),
            self.get_available_height(),
            &mut path_stack,
            &mut |_node, current_path, rect| {
                if current_path == path {
                    found = Some(rect);
                }
            },
        );

        found
    }

    /// Compute bounding rectangles for every visible node. The `visitor` receives `(node, path, (x, y, w, h))`.
    fn walk_with_layout_and_path<F>(
        &self,
        node: &DockNode,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        path: &mut Vec<usize>,
        visitor: &mut F,
    ) where
        F: FnMut(&DockNode, &[usize], (f32, f32, f32, f32)),
    {
        // Call the visitor on the current node
        visitor(node, path, (x, y, w, h));

        match node {
            DockNode::Split { direction, children } => {
                let visible_children: Vec<&DockSplitChild> = children
                    .iter()
                    .filter(|child| child.node.is_visible())
                    .collect();
                if visible_children.is_empty() {
                    return;
                }

                let total_ratio: f32 = visible_children.iter().map(|child| child.ratio).sum();
                let mut offset = 0.0;
                let child_count = visible_children.len();

                for (child_index, child) in children.iter().enumerate() {
                    if !child.node.is_visible() {
                        continue;
                    }

                    let child_ratio = if total_ratio > 0.0 {
                        child.ratio / total_ratio
                    } else {
                        1.0 / child_count as f32
                    };

                    let (cw, ch) = match direction {
                        DockSplitDirection::HorizontalDivider => (w, h * child_ratio),
                        DockSplitDirection::VerticalDivider => (w * child_ratio, h),
                    };
                    let (cx, cy) = match direction {
                        DockSplitDirection::HorizontalDivider => (x, y + offset),
                        DockSplitDirection::VerticalDivider => (x + offset, y),
                    };

                    // Push this child's index
                    path.push(child_index);
                    self.walk_with_layout_and_path(&child.node, cx, cy, cw, ch, path, visitor);
                    path.pop();

                    // Advance offset
                    match direction {
                        DockSplitDirection::HorizontalDivider => offset += ch,
                        DockSplitDirection::VerticalDivider => offset += cw,
                    }
                }
            }
            DockNode::Tab { tabs, .. } => {
                let visible_children: Vec<&DockNode> = tabs
                    .iter()
                    .filter(|child_node| child_node.is_visible())
                    .collect();
                if visible_children.is_empty() {
                    return;
                }

                // All visible tabs receive the same (x, y, w, h).
                for (original_idx, tab_child) in tabs.iter().enumerate() {
                    if !tab_child.is_visible() {
                        continue;
                    }

                    path.push(original_idx);
                    self.walk_with_layout_and_path(tab_child, x, y, w, h, path, visitor);
                    path.pop();
                }
            }
            // No children, just a window
            DockNode::Window { .. } => {}
        }
    }
}
