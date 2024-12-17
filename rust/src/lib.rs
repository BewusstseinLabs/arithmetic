// Copyright 2024 Bewusstsein Labs

pub trait AddAssignTo<Rhs = Self> {
    type Output;

    fn add_assign_to( self, rhs: Rhs, res: &mut Self::Output );
}

pub trait SubAssignTo<Rhs = Self> {
    type Output;

    fn sub_assign_to( self, rhs: Rhs, res: &mut Self::Output );
}

pub trait MulAssignTo<Rhs = Self> {
    type Output;

    fn mul_assign_to( self, rhs: Rhs, res: &mut Self::Output );
}

pub trait DivAssignTo<Rhs = Self> {
    type Output;

    fn div_assign_to( self, rhs: Rhs, res: &mut Self::Output );
}