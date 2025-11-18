use color_eyre::eyre::Result;

pub trait FromRaw<R>: Sized {
    fn from_raw(r: R) -> Result<Self>;
    fn from_vec_raw(r: Vec<R>) -> Result<Vec<Self>>;
}
