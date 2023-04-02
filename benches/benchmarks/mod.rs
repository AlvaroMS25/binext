use rand::Fill;

pub mod multiple_fields;
pub mod single_buffer;

pub(self) trait FromArray<T, Args, const SIZE: usize> {
    fn from_array(array: [T; SIZE], args: Args) -> Self;
}

pub(self) fn fill<I, T, Args, const SIZE: usize>(args: Args) -> I
where
    I: FromArray<T, Args, SIZE>,
    T: Default + Copy,
    [T]: Fill
{
    use rand::Rng;

    let mut buf = [T::default(); SIZE];
    rand::thread_rng().fill(&mut buf);

    I::from_array(buf, args)
}
