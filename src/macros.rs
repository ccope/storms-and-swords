#[macro_export]
macro_rules! implement_entity {
    ($ty:ty) => (
        impl Entity for $ty {
            fn get_position(&self) -> Vector3<f32> {
                self.position.clone()
            }

            fn get_scale(&self) -> Vector3<f32> {
                self.scale.clone()
            }

            fn get_rotation(&self) -> Vector3<f32> {
                self.rotation.clone()
            }

            fn get_uid(&self) -> UID {
                self.uid.clone()
            }

            fn update(&mut self) {

            }
        }
    )
}
