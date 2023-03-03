use fj_math::Winding;

use crate::objects::Face;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_interior_winding(self, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// Interior of [`Face`] has invalid winding; must be opposite of exterior
    #[error(
        "Interior of `Face` has invalid winding; must be opposite of exterior\n\
        - Winding of exterior cycle: {exterior_winding:#?}\n\
        - Winding of interior cycle: {interior_winding:#?}\n\
        - `Face`: {face:#?}"
    )]
    InvalidInteriorWinding {
        /// The winding of the [`Face`]'s exterior cycle
        exterior_winding: Winding,

        /// The winding of the invalid interior cycle
        interior_winding: Winding,

        /// The face
        face: Face,
    },
}

impl FaceValidationError {
    fn check_interior_winding(face: &Face, errors: &mut Vec<ValidationError>) {
        let exterior_winding = face.exterior().winding();

        for interior in face.interiors() {
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        face: face.clone(),
                    }
                    .into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::reverse::Reverse,
        builder::{CycleBuilder, FaceBuilder},
        objects::Face,
        partial::{PartialFace, PartialObject},
        services::Services,
        validate::{FaceValidationError, Validate, ValidationError},
    };

    #[test]
    fn face_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut face = PartialFace::new(&mut services.objects);

            face.surface = Some(services.objects.surfaces.xy_plane());
            face.exterior.write().update_as_polygon_from_points(
                [[0., 0.], [3., 0.], [0., 3.]],
                &mut services.objects,
            );
            face.add_interior(&mut services.objects)
                .write()
                .update_as_polygon_from_points(
                    [[1., 1.], [1., 2.], [2., 1.]],
                    &mut services.objects,
                );
            face.build(&mut services.objects)
        };
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services.objects))
                .collect::<Vec<_>>();

            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(matches!(
            invalid.validate_and_return_first_error(),
            Err(ValidationError::Face(
                FaceValidationError::InvalidInteriorWinding { .. }
            ))
        ));

        Ok(())
    }
}
