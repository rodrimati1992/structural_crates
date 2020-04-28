///
/// ```
/// let _=std::panic::catch_unwind(||{
///     panic!()
/// });
/// ```
///
/// ```should_panic
/// let _=std::panic::catch_unwind(||{
///     structural::abort!()
/// });
/// ```
///
/// ```should_panic
/// let _=std::panic::catch_unwind(||{
///     structural::abort!("hello{}","world")
/// });
/// ```
pub struct TestAbortMacro;
