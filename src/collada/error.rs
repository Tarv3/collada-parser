use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct ArrayError;

impl Display for ArrayError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to convert string into array")
    }
}

impl Error for ArrayError {}

#[derive(Copy, Clone, Debug)]
pub struct MeshParseError;

impl Display for MeshParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse mesh")
    }
}

impl Error for MeshParseError {}

#[derive(Copy, Clone, Debug)]
pub struct IndexAccessorError;

impl Display for IndexAccessorError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse triangle accessor")
    }
}

impl Error for IndexAccessorError {}

#[derive(Copy, Clone, Debug)]
pub struct DataSourceError;

impl Display for DataSourceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse data source")
    }
}

impl Error for DataSourceError {}

#[derive(Copy, Clone, Debug)]
pub struct AccessorParseError;

impl Display for AccessorParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse accessor")
    }
}

impl Error for AccessorParseError {}

#[derive(Copy, Clone, Debug)]
pub struct PrimitiveIndicesError;

impl Display for PrimitiveIndicesError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse accessor")
    }
}

impl Error for PrimitiveIndicesError {}

#[derive(Copy, Clone, Debug)]
pub struct PrimitiveElementError;

impl Display for PrimitiveElementError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse primitive element")
    }
}

impl Error for PrimitiveElementError {}

#[derive(Copy, Clone, Debug)]
pub struct MissingSourceError;

impl Display for MissingSourceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to get attribute from source list")
    }
}

impl Error for MissingSourceError {}

#[derive(Copy, Clone, Debug)]
pub struct VerticesError;

impl Display for VerticesError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse vertices")
    }
}

impl Error for VerticesError {}

#[derive(Copy, Clone, Debug)]
pub struct MeshError;

impl Display for MeshError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to convert into a mesh")
    }
}

impl Error for MeshError {}

#[derive(Copy, Clone, Debug)]
pub struct VertexWeightsError;

impl Display for VertexWeightsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse vertex weights")
    }
}

impl Error for VertexWeightsError {}

#[derive(Copy, Clone, Debug)]
pub struct SkinParseError;

impl Display for SkinParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse skin")
    }
}

impl Error for SkinParseError {}

#[derive(Copy, Clone, Debug)]
pub struct AnimationParseError;

impl Display for AnimationParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse animation")
    }
}

impl Error for AnimationParseError {}

#[derive(Copy, Clone, Debug)]
pub struct TransformationParseError;

impl Display for TransformationParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse transformation")
    }
}

impl Error for TransformationParseError {}

#[derive(Clone, Debug)]
pub struct MissingAttributeError {
    pub attribute_name: String,
}

impl Display for MissingAttributeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Could not find attribute with name {}", self.attribute_name)
    }
}

impl Error for MissingAttributeError {}

#[derive(Copy, Clone, Debug)]
pub struct SkeletonParseError;

impl Display for SkeletonParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse skeleton")
    }
}

impl Error for SkeletonParseError {}

#[derive(Copy, Clone, Debug)]
pub struct GeometryParseError;

impl Display for GeometryParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse geometry")
    }
}

impl Error for GeometryParseError {}

#[derive(Copy, Clone, Debug)]
pub struct ControllerParseError;

impl Display for ControllerParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse controller")
    }
}

impl Error for ControllerParseError {}

#[derive(Copy, Clone, Debug)]
pub struct VisualSceneError;

impl Display for VisualSceneError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse visual scene")
    }
}

impl Error for VisualSceneError {}

#[derive(Copy, Clone, Debug)]
pub struct MissingChildrenError;

impl Display for MissingChildrenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Missing children")
    }
}

impl Error for MissingChildrenError {}

#[derive(Copy, Clone, Debug)]
pub struct SceneNodeError;

impl Display for SceneNodeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Failed to parse scene node")
    }
}

impl Error for SceneNodeError {}


