use super::*;

#[test]
fn fieldpath_push_append(){
    #[allow(dead_code)]
    fn with_generics<A,B,C,D>(){
        let fp0=FieldPath::<(A,)>::new();
        let fp1=FieldPath::<(B,)>::new();
        let fp2=FieldPath::<(C,)>::new();
        let fp3=FieldPath::<(D,)>::new();

        let a:FieldPath<(A,B)>=fp0.push(fp1);
        let _:FieldPath<(A,B)>=fp0.append(fp1);
        
        let b:FieldPath<(A,B,C)>=a.push(fp2);
        let _:FieldPath<(A,B,C)>=a.append(fp2);

        let _:FieldPath<(A,B,C,D)>=b.push(fp3);
        let _:FieldPath<(A,B,C,D)>=b.append(fp3);
    }
}


#[test]
fn fieldpaths_push_append(){
    #[allow(dead_code)]
    fn with_generics<A,B,C,D>(){
        type Fp<T>=FieldPath<(T,)>;
        let fp1=Fp::<B>::new();
        let fp2=Fp::<C>::new();
        let fp3=Fp::<D>::new();

        let fps0=unsafe{ FieldPathSet::<(Fp<A>,),UniquePaths>::new() };
        let fps1=unsafe{ FieldPathSet::<(Fp<B>,),UniquePaths>::new() };
        let fps2=unsafe{ FieldPathSet::<(Fp<C>,),UniquePaths>::new() };
        let fps3=unsafe{ FieldPathSet::<(Fp<D>,),UniquePaths>::new() };

        let a:FieldPathSet<(Fp<A>,Fp<B>),AliasedPaths>=fps0.push(fps1);
        let _:FieldPathSet<(Fp<A>,Fp<B>),AliasedPaths>=fps0.push(fp1);
        let _:FieldPathSet<(Fp<A>,Fp<B>),AliasedPaths>=fps0.append(fps1);
        
        let b:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>),AliasedPaths>=a.push(fps2);
        let _:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>),AliasedPaths>=a.push(fp2);
        let _:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>),AliasedPaths>=a.append(fps2);

        let _:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>,Fp<D>),AliasedPaths>=b.push(fps3);
        let _:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>,Fp<D>),AliasedPaths>=b.push(fp3);
        let _:FieldPathSet<(Fp<A>,Fp<B>,Fp<C>,Fp<D>),AliasedPaths>=b.append(fps3);
    }
}