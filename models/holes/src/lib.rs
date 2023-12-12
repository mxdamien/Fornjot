use fj::{
    core::{
        objects::Solid,
        operations::{
            holes::{AddHole, HoleLocation},
            insert::Insert,
            update::UpdateSolid,
        },
        services::Services,
        storage::Handle,
    },
    math::Scalar,
};

pub fn model(
    radius: impl Into<Scalar>,
    services: &mut Services,
) -> Handle<Solid> {
    let radius = radius.into();

    let size = radius * 4.;
    let cuboid = cuboid::model([size * 2., size, size], services);

    cuboid
        .update_shell(cuboid.shells().first(), |shell| {
            let bottom_face = shell.faces().first();
            let offset = size / 2.;
            let depth = size / 2.;

            let shell = shell.add_blind_hole(
                HoleLocation {
                    face: bottom_face,
                    position: [-offset, Scalar::ZERO].into(),
                },
                radius,
                [Scalar::ZERO, Scalar::ZERO, depth],
                services,
            );

            let bottom_face = shell.faces().first();
            let top_face = shell
                .faces()
                .nth(5)
                .expect("Expected shell to have top face");

            shell
                .add_through_hole(
                    [
                        HoleLocation {
                            face: bottom_face,
                            position: [offset, Scalar::ZERO].into(),
                        },
                        HoleLocation {
                            face: top_face,
                            position: [offset, Scalar::ZERO].into(),
                        },
                    ],
                    radius,
                    services,
                )
                .insert(services)
        })
        .insert(services)
}