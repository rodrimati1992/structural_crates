use crate::field_path::{AliasedPaths, UniquePaths};
use crate::{FieldPathSet, NestedFieldPath, NestedFieldPathSet, VariantField, VariantName};

use core_extensions::ConstDefault;

tstr_aliases! {
    N99=99,
    Foo,
    bar,
    baz,
}

#[test]
fn to_path_to_set() {
    {
        let this: N99 = fp!(99);
        let _: NestedFieldPath<(N99,)> = this.into_path();
        let _: N99 = this.into_path().into_component();
        let _: FieldPathSet<(N99,), UniquePaths> = this.into_set();
        let _: N99 = this.into_set().into_path();
        let _: (N99,) = this.into_set().into_paths();
    }
    {
        let this: VariantName<N99> = fp!(::99);
        let _: NestedFieldPath<(VariantName<N99>,)> = this.into_path();
        let _: VariantName<N99> = this.into_path().into_component();
        let _: FieldPathSet<(VariantName<N99>,), UniquePaths> = this.into_set();
        let _: VariantName<N99> = this.into_set().into_path();
        let _: (VariantName<N99>,) = this.into_set().into_paths();
    }
    {
        let this: VariantField<Foo, N99> = fp!(::Foo.99);
        let _: NestedFieldPath<(VariantField<Foo, N99>,)> = this.into_path();
        let _: VariantField<Foo, N99> = this.into_path().into_component();
        let _: FieldPathSet<(VariantField<Foo, N99>,), UniquePaths> = this.into_set();
        let _: VariantField<Foo, N99> = this.into_set().into_path();
        let _: (VariantField<Foo, N99>,) = this.into_set().into_paths();
    }
    {
        let this: NestedFieldPath<(bar, baz)> = fp!(bar.baz);
        let _: FieldPathSet<(NestedFieldPath<(bar, baz)>,), UniquePaths> = this.into_set();
        let _: NestedFieldPath<(bar, baz)> = this.into_set().into_path();
        let _: (NestedFieldPath<(bar, baz)>,) = this.into_set().into_paths();
    }
}

#[test]
fn uniqueness_methods() {
    unsafe {
        let this: FieldPathSet<(bar, baz), AliasedPaths> = FieldPathSet::many((bar, baz));
        let unique: FieldPathSet<(bar, baz), UniquePaths> = this.upgrade_unchecked();
        let _: FieldPathSet<(bar, baz), UniquePaths> = this.set_uniqueness();
        let _: FieldPathSet<(bar, baz), AliasedPaths> = this.set_uniqueness();
        let _: FieldPathSet<(bar, baz), AliasedPaths> = unique.downgrade();
    }
    unsafe {
        let this: NestedFieldPathSet<bar, (baz,), AliasedPaths> =
            NestedFieldPathSet::new(bar, FieldPathSet::many((baz,)));

        let unique: NestedFieldPathSet<bar, (baz,), UniquePaths> = this.upgrade_unchecked();
        let _: NestedFieldPathSet<bar, (baz,), UniquePaths> = this.set_uniqueness();
        let _: NestedFieldPathSet<bar, (baz,), AliasedPaths> = this.set_uniqueness();
        let _: NestedFieldPathSet<bar, (baz,), AliasedPaths> = unique.downgrade();
    }
}

#[test]
fn assoc_constants() {
    let _: N99 = N99::NEW;
    let _: VariantField<N99, bar> = <VariantField<N99, bar>>::NEW;
    let _: VariantName<N99> = <VariantName<N99>>::NEW;
    let _: NestedFieldPath<(N99,)> = NestedFieldPath::<(N99,)>::NEW;

    let _: FieldPathSet<(N99,), AliasedPaths> = FieldPathSet::<(N99,), AliasedPaths>::NEW;
    let _: FieldPathSet<(N99,), AliasedPaths> = FieldPathSet::<(N99,), AliasedPaths>::NEW_ALIASED;
    let _: FieldPathSet<(N99,), AliasedPaths> = FieldPathSet::<(N99,), UniquePaths>::NEW_ALIASED;

    let _: NestedFieldPathSet<bar, (N99,), AliasedPaths> =
        NestedFieldPathSet::<bar, (N99,), AliasedPaths>::NEW;
    let _: NestedFieldPathSet<bar, (N99,), AliasedPaths> =
        NestedFieldPathSet::<bar, (N99,), AliasedPaths>::NEW_ALIASED;
    let _: NestedFieldPathSet<bar, (N99,), AliasedPaths> =
        NestedFieldPathSet::<bar, (N99,), UniquePaths>::NEW_ALIASED;
}

#[test]
fn fieldpath_push_append() {
    #[allow(dead_code)]
    fn with_generics<A, B, C, D>()
    where
        A: ConstDefault + Copy,
        B: ConstDefault + Copy,
        C: ConstDefault + Copy,
        D: ConstDefault + Copy,
    {
        let fp0 = NestedFieldPath::<(A,)>::NEW;
        let fp1 = NestedFieldPath::<(B,)>::NEW;
        let fp2 = NestedFieldPath::<(C,)>::NEW;
        let fp3 = NestedFieldPath::<(D,)>::NEW;

        let a: NestedFieldPath<(A, B)> = fp0.push(fp1.list.0);
        let _: NestedFieldPath<(A, B)> = fp0.append(fp1);

        let b: NestedFieldPath<(A, B, C)> = a.push(fp2.list.0);
        let _: NestedFieldPath<(A, B, C)> = a.append(fp2);

        let _: NestedFieldPath<(A, B, C, D)> = b.push(fp3.list.0);
        let _: NestedFieldPath<(A, B, C, D)> = b.append(fp3);
    }
}

#[test]
fn fieldpaths_push_append() {
    #[allow(dead_code)]
    fn with_generics<A, B, C, D>()
    where
        A: ConstDefault + Copy,
        B: ConstDefault + Copy,
        C: ConstDefault + Copy,
        D: ConstDefault + Copy,
    {
        type Fp<T> = NestedFieldPath<(T,)>;
        let fp1 = Fp::<B>::NEW;
        let fp2 = Fp::<C>::NEW;
        let fp3 = Fp::<D>::NEW;

        let fps0 = unsafe { FieldPathSet::<(Fp<A>,), _>::NEW.upgrade_unchecked() };
        let fps1 = unsafe { FieldPathSet::<(Fp<B>,), _>::NEW.upgrade_unchecked() };
        let fps2 = unsafe { FieldPathSet::<(Fp<C>,), _>::NEW.upgrade_unchecked() };
        let fps3 = unsafe { FieldPathSet::<(Fp<D>,), _>::NEW.upgrade_unchecked() };

        let a: FieldPathSet<(Fp<A>, Fp<B>), AliasedPaths> = fps0.push(fps1);
        let _: FieldPathSet<(Fp<A>, Fp<B>), AliasedPaths> = fps0.push(fp1);
        let _: FieldPathSet<(Fp<A>, Fp<B>), AliasedPaths> = fps0.append(fps1);

        let b: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>), AliasedPaths> = a.push(fps2);
        let _: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>), AliasedPaths> = a.push(fp2);
        let _: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>), AliasedPaths> = a.append(fps2);

        let _: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>, Fp<D>), AliasedPaths> = b.push(fps3);
        let _: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>, Fp<D>), AliasedPaths> = b.push(fp3);
        let _: FieldPathSet<(Fp<A>, Fp<B>, Fp<C>, Fp<D>), AliasedPaths> = b.append(fps3);
    }
}
