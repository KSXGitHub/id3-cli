use crate::args::{field::Field, FrameViewArgs, TextViewArgs};

pub fn view(args: Field<TextViewArgs, FrameViewArgs>) -> Result<(), String> {
    dbg!(args);
    unimplemented!()
}
