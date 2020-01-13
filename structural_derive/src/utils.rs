#[inline(always)]
#[allow(dead_code)]
pub(crate) fn mem_take<T>(mut_: &mut T) -> T
where
    T: Default,
{
    std::mem::replace(mut_, Default::default())
}
