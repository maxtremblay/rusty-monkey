#[macro_export]
macro_rules! impl_node {
    ($name:ident) => {
        impl Node for $name {
            fn token_literal(&self) -> &str {
                &self.token.literal
            }
        }
    };
}

#[macro_export]
macro_rules! create_node {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name {
            pub token: Token,
        }

        crate::impl_node!($name);
    };

    ($name:ident {$($field:ident: $type:ident),+ $(,)?}) => {
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name {
            pub token: Token,
            $(pub $field: $type,)+
        }

        crate::impl_node!($name);
    };
}
