use std::sync::Arc;

pub trait IntoArc {
    fn into_arc(self) -> Arc<Self>;
}

impl<T> IntoArc for T {
    fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
