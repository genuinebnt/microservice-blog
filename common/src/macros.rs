#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<$name Marker>];
            pub type [<$name Id>] = $crate::types::Id<[<$name Marker>]>;
        }
    };
}
