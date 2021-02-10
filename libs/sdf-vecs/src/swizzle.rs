use crate::{Dimension, VecType, component_access::ComponentAccess, scale::Scale, vec_2::Vec2, vec_3::Vec3};

trait SwizzleDim {
    type Output: Dimension;
}

trait SwizzleDim2: SwizzleDim {
    fn xx(&self) -> Self::Output;
    fn xy(&self) -> Self::Output;
    fn yx(&self) -> Self::Output;
    fn yy(&self) -> Self::Output;
}

impl SwizzleDim for Vec2 {
    type Output = Vec2;
}

impl SwizzleDim2 for Vec2 {
    fn xx(&self) -> Self::Output {
        Self::new(self.x(), self.x())
    }

    fn xy(&self) -> Self::Output {
        Self::new(self.x(), self.y())
    }

    fn yx(&self) -> Self::Output {
        Self::new(self.y(), self.x())
    }

    fn yy(&self) -> Self::Output {
        Self::new(self.y(), self.y())
    }
}

trait SwizzleDim3: SwizzleDim {
    fn xxx(&self) -> Self::Output;
    fn xxy(&self) -> Self::Output;
    fn xxz(&self) -> Self::Output;
    fn xyx(&self) -> Self::Output;
    fn xzx(&self) -> Self::Output;
    fn xzy(&self) -> Self::Output;
    fn xyz(&self) -> Self::Output;
    fn yxx(&self) -> Self::Output;
    fn zxx(&self) -> Self::Output;
    fn yyy(&self) -> Self::Output;
    fn yyx(&self) -> Self::Output;
    fn yyz(&self) -> Self::Output;
    fn yxy(&self) -> Self::Output;
    fn yzy(&self) -> Self::Output;
    fn xyy(&self) -> Self::Output;
    fn zyy(&self) -> Self::Output;
    fn yxz(&self) -> Self::Output;
    fn yzx(&self) -> Self::Output;
    fn zzz(&self) -> Self::Output;
    fn zzx(&self) -> Self::Output;
    fn zzy(&self) -> Self::Output;
    fn zxz(&self) -> Self::Output;
    fn zyz(&self) -> Self::Output;
    fn xzz(&self) -> Self::Output;
    fn yzz(&self) -> Self::Output;
    fn zxy(&self) -> Self::Output;
    fn zyx(&self) -> Self::Output;
}

impl SwizzleDim for Vec3 {
    type Output = Vec3;
}

impl SwizzleDim3 for Vec3 {
    fn xxx(&self) -> Self::Output {
        Self::new(self.x(), self.x(), self.x())
    }

    fn xxy(&self) -> Self::Output {
        Self::new(self.x(), self.x(), self.y())
    }

    fn xxz(&self) -> Self::Output {
        Self::new(self.x(), self.x(), self.z())
    }

    fn xyx(&self) -> Self::Output {
        Self::new(self.x(), self.y(), self.x())
    }

    fn xzx(&self) -> Self::Output {
        Self::new(self.x(), self.z(), self.x())
    }

    fn yxx(&self) -> Self::Output {
        Self::new(self.y(), self.x(), self.x())
    }

    fn zxx(&self) -> Self::Output {
        Self::new(self.z(), self.x(), self.x())
    }

    fn yyy(&self) -> Self::Output {
        Self::new(self.y(), self.y(), self.y())
    }

    fn yyx(&self) -> Self::Output {
        Self::new(self.y(), self.y(), self.x())
    }

    fn yyz(&self) -> Self::Output {
        Self::new(self.y(), self.y(), self.z())
    }

    fn yxy(&self) -> Self::Output {
        Self::new(self.y(), self.x(), self.y())
    }

    fn yzy(&self) -> Self::Output {
        Self::new(self.y(), self.z(), self.y())
    }

    fn xyy(&self) -> Self::Output {
        Self::new(self.x(), self.y(), self.y())
    }

    fn zyy(&self) -> Self::Output {
        Self::new(self.z(), self.y(), self.y())
    }

    fn zzz(&self) -> Self::Output {
        Self::new(self.z(), self.z(), self.z())
    }

    fn zzx(&self) -> Self::Output {
        Self::new(self.z(), self.z(), self.x())
    }

    fn zzy(&self) -> Self::Output {
        Self::new(self.z(), self.z(), self.y())
    }

    fn zxz(&self) -> Self::Output {
        Self::new(self.z(), self.x(), self.z())
    }

    fn zyz(&self) -> Self::Output {
        Self::new(self.z(), self.y(), self.z())
    }

    fn xzz(&self) -> Self::Output {
        Self::new(self.x(), self.z(), self.z())
    }

    fn yzz(&self) -> Self::Output {
        Self::new(self.y(), self.z(), self.z())
    }

    fn xzy(&self) -> Self::Output {
        Self::new(self.x(), self.z(), self.y())
    }

    fn xyz(&self) -> Self::Output {
        Self::new(self.x(), self.y(), self.z())
    }

    fn yxz(&self) -> Self::Output {
        Self::new(self.y(), self.x(), self.z())
    }

    fn yzx(&self) -> Self::Output {
        Self::new(self.y(), self.z(), self.x())
    }

    fn zxy(&self) -> Self::Output {
        Self::new(self.z(), self.x(), self.y())
    }

    fn zyx(&self) -> Self::Output {
        Self::new(self.z(), self.y(), self.x())
    }
}

// TODO: impl swizzles for vectype
impl SwizzleDim for VecType {
    type Output = VecType;
}

impl SwizzleDim2 for VecType {
    fn xx(&self) -> Self::Output {
        self.scale2().xx().into()
    }

    fn xy(&self) -> Self::Output {
        self.scale2().xy().into()
    }

    fn yx(&self) -> Self::Output {
        self.scale2().yx().into()
    }

    fn yy(&self) -> Self::Output {
        self.scale2().yy().into()
    }
}

impl SwizzleDim3 for VecType {
    fn xxx(&self) -> Self::Output {
        self.scale3().xxx().into()
    }

    fn xxy(&self) -> Self::Output {
        self.scale3().xxy().into()
    }

    fn xxz(&self) -> Self::Output {
        self.scale3().xxz().into()
    }

    fn xyx(&self) -> Self::Output {
        self.scale3().xyx().into()
    }

    fn xzx(&self) -> Self::Output {
        self.scale3().xzx().into()
    }

    fn xzy(&self) -> Self::Output {
        self.scale3().xzy().into()
    }

    fn xyz(&self) -> Self::Output {
        self.scale3().xyz().into()
    }

    fn yxx(&self) -> Self::Output {
        self.scale3().yxx().into()
    }

    fn zxx(&self) -> Self::Output {
        self.scale3().zxx().into()
    }

    fn yyy(&self) -> Self::Output {
        self.scale3().yyy().into()
    }

    fn yyx(&self) -> Self::Output {
        self.scale3().yyx().into()
    }

    fn yyz(&self) -> Self::Output {
        self.scale3().yyz().into()
    }

    fn yxy(&self) -> Self::Output {
        self.scale3().yxy().into()
    }

    fn yzy(&self) -> Self::Output {
        self.scale3().yzy().into()
    }

    fn xyy(&self) -> Self::Output {
        self.scale3().xyy().into()
    }

    fn zyy(&self) -> Self::Output {
        self.scale3().zyy().into()
    }

    fn yxz(&self) -> Self::Output {
        self.scale3().yxz().into()
    }

    fn yzx(&self) -> Self::Output {
        self.scale3().yzx().into()
    }

    fn zzz(&self) -> Self::Output {
        self.scale3().zzz().into()
    }

    fn zzx(&self) -> Self::Output {
        self.scale3().zzx().into()
    }

    fn zzy(&self) -> Self::Output {
        self.scale3().zzy().into()
    }

    fn zxz(&self) -> Self::Output {
        self.scale3().zxz().into()
    }

    fn zyz(&self) -> Self::Output {
        self.scale3().zyz().into()
    }

    fn xzz(&self) -> Self::Output {
        self.scale3().xzz().into()
    }

    fn yzz(&self) -> Self::Output {
        self.scale3().yzz().into()
    }

    fn zxy(&self) -> Self::Output {
        self.scale3().zxy().into()
    }

    fn zyx(&self) -> Self::Output {
        self.scale3().zyx().into()
    }
}