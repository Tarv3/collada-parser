pub mod error;
pub mod mesh;
pub mod accessor;
pub mod source;
pub mod skin;
pub mod animation;
pub mod skeleton;
pub mod document;
pub mod util;

pub use self::animation::Animation;
pub use self::mesh::{GenericMesh, Vertex, Shape};
pub use self::skin::Skin;
pub use self::skeleton::{Skeleton, node::SkeletonNode};
pub use self::document::{*, controller::*, geometry::*, visual_scene::*};

use math::Vector3;

pub type Mesh = GenericMesh<Vector3>;
// Pos Texture Normal Color
pub type PTNCIndex = (usize, Option<usize>, Option<usize>, Option<usize>);
pub type PTNIndex = (usize, Option<usize>, Option<usize>);

