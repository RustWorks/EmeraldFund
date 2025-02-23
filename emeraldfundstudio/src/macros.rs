#[macro_export]
macro_rules! create_nodes {
    ($self:ident, $($node_name:ident),*) => {
        match $self.node_name.as_str() {
            $(
                stringify!($node_name) => Box::new(serde_json::from_value::<$node_name>($self.arguments.clone())?),
            )*
            _ => return Err(anyhow!("Node {} not found", $self.node_name)),
        }
    };
}

