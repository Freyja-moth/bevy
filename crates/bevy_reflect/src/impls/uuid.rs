use crate::{std_traits::ReflectDefault, ReflectDeserialize, ReflectSerialize};
use bevy_reflect_derive::impl_reflect_opaque;

impl_reflect_opaque!(::bevy_platform::uuid::Uuid(
    Serialize,
    Deserialize,
    Default,
    Clone,
    Debug,
    PartialEq,
    Hash
));
