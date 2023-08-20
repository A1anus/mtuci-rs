use futures::task::{self, ArcWake};
use std::sync::Arc;
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}