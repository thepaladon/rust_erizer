use glam::{Mat4, Quat, Vec3};

pub enum TransformInitParams {
    Identity,
    Translation(Vec3),
    Rotation(Quat),
    Scale(Vec3),
    TranslationRotation(Vec3, Quat),
}

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn from_rotation_quat(quat: Quat) -> Self {
        Self {
            rotation: quat.normalize(),
            ..Default::default()
        }
    }

    pub fn from_rotation_euler(euler: Vec3) -> Self {
        Self {
            rotation: glam::Quat::from_euler(glam::EulerRot::XYZ, euler.x, euler.y, euler.z)
                .normalize(),
            ..Default::default()
        }
    }

    pub fn from_translation_and_rotation(translation: Vec3, quat: Quat) -> Self {
        Self {
            translation,
            rotation: quat.normalize(),
            ..Default::default()
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    pub fn local(&self) -> Mat4 {
        let mut matrix = Mat4::IDENTITY;
        matrix *= Mat4::from_translation(self.translation);
        matrix *= Mat4::from_quat(self.rotation);
        matrix *= Mat4::from_scale(self.scale);
        matrix
    }

    pub fn add_rotation(&mut self, euler: Vec3) {
        let rot =
            glam::Quat::from_euler(glam::EulerRot::XYZ, euler.x, euler.y, euler.z).normalize();
        self.rotation *= rot;
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    //translation * quat * scale - for column row.

    //Functions to have
    // local
    // right
    // forward
    // up func

    // From and Into to transform from different structures
    // Matrix4x4 -> Transform
}

impl From<TransformInitParams> for Transform {
    fn from(params: TransformInitParams) -> Self {
        match params {
            TransformInitParams::Identity => Self::IDENTITY,
            TransformInitParams::Translation(translate) => Self::from_translation(translate),
            TransformInitParams::Rotation(rotation) => Self::from_rotation_quat(rotation),
            TransformInitParams::Scale(scale) => Self::from_scale(scale),
            TransformInitParams::TranslationRotation(translation, rotation) => {
                Self::from_translation_and_rotation(translation, rotation)
            }
        }
    }
}
