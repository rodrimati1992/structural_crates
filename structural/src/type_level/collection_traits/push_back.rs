use super::PushBack_;

impl<Type> PushBack_<Type> for () {
    type Output = (Type,);
}

impl<L0, Type> PushBack_<Type> for (L0,) {
    type Output = (L0, Type);
}

impl<L0, L1, Type> PushBack_<Type> for (L0, L1) {
    type Output = (L0, L1, Type);
}

impl<L0, L1, L2, Type> PushBack_<Type> for (L0, L1, L2) {
    type Output = (L0, L1, L2, Type);
}

impl<L0, L1, L2, L3, Type> PushBack_<Type> for (L0, L1, L2, L3) {
    type Output = (L0, L1, L2, L3, Type);
}

impl<L0, L1, L2, L3, L4, Type> PushBack_<Type> for (L0, L1, L2, L3, L4) {
    type Output = (L0, L1, L2, L3, L4, Type);
}

impl<L0, L1, L2, L3, L4, L5, Type> PushBack_<Type> for (L0, L1, L2, L3, L4, L5) {
    type Output = (L0, L1, L2, L3, L4, L5, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, Type> PushBack_<Type> for (L0, L1, L2, L3, L4, L5, L6) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, Type> PushBack_<Type> for (L0, L1, L2, L3, L4, L5, L6, L7) {
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
{
    type Output = (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, Type);
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, Type> PushBack_<Type>
    for (L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
{
    type Output = (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        Type,
    );
}

impl<L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, Type> PushBack_<Type>
    for (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
    )
{
    type Output = (
        L0,
        L1,
        L2,
        L3,
        L4,
        L5,
        L6,
        L7,
        L8,
        L9,
        L10,
        L11,
        L12,
        L13,
        L14,
        Type,
    );
}
