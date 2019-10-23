use crate::{
    structural_alias_impl::{Access,IdentOrIndex},
};

use as_derive_utils::{
    attribute_parsing::with_nested_meta,
    datastructure::{DataStructure,Field,FieldMap},
    utils::{LinearResult,SynResultExt,SynPathExt},
    spanned_err,
};

use quote::ToTokens;

use syn::{
    Attribute,
    Ident,
    Lit,
    Meta,MetaList,MetaNameValue,
};


use std::marker::PhantomData;


pub(crate) struct StructuralOptions<'a>{
    pub(crate) field_access:FieldMap<Access>,
    pub(crate) renamed_fields:FieldMap<Option<IdentOrIndex>>,

    pub(crate) debug_print:bool,

    _marker:PhantomData<&'a ()>,
}


impl<'a> StructuralOptions<'a>{
    fn new(
        _ds: &'a DataStructure<'a>,
        this:StructuralAttrs<'a>,
    )->Result<Self,syn::Error> {
        let StructuralAttrs{
            field_access,
            renamed_fields,
            debug_print,
            errors:_,
            _marker,
        }=this;
        
        Ok(Self{
            field_access,
            renamed_fields,
            debug_print,
            _marker,
        })
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct StructuralAttrs<'a>{
    field_access:FieldMap<Access>,
    renamed_fields:FieldMap<Option<IdentOrIndex>>,

    debug_print:bool,

    errors:LinearResult<()>,
    
    _marker:PhantomData<&'a ()>,
}


#[derive(Debug,Copy, Clone)]
enum ParseContext<'a> {
    TypeAttr{
        name:&'a Ident,
    },
    Field{
        field:&'a Field<'a>,
    },
}


/// Parses the attributes for the `Structural` derive macro.
pub(crate) fn parse_attrs_for_structural<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<StructuralOptions<'a>,syn::Error> {
    let mut this = StructuralAttrs::default();

    this.field_access=FieldMap::defaulted(ds);
    this.renamed_fields=FieldMap::defaulted(ds);

    let name=ds.name;

    parse_inner(&mut this, ds.attrs, ParseContext::TypeAttr{name})?;

    for (_,field) in ds.variants[0].fields.iter().enumerate() {
        parse_inner(&mut this, field.attrs, ParseContext::Field{field} )?;
    }

    this.errors.take()?;

    StructuralOptions::new(ds, this)
}











/// Parses an individual attribute
fn parse_inner<'a,I>(
    this: &mut StructuralAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
)-> Result<(),syn::Error>
where
    I:IntoIterator<Item=&'a Attribute>
{
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                parse_attr_list(this,pctx, list)
                    .combine_into_err(&mut this.errors);
            }
            Err(e)=>{
                this.errors.push_err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parses an individual attribute list (A `#[attribute( .. )] attribute`).
fn parse_attr_list<'a>(
    this: &mut StructuralAttrs<'a>,
    pctx: ParseContext<'a>,
    list: MetaList, 
)-> Result<(),syn::Error> {
    if list.path.equals_str("struc") {
        with_nested_meta("struc", list.nested, |attr| {
            parse_sabi_attr(this,pctx, attr)
                .combine_into_err(&mut this.errors);
            Ok(())
        })?;
    }
    Ok(())
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut StructuralAttrs<'a>,
    pctx: ParseContext<'a>, 
    attr: Meta, 
)-> Result<(),syn::Error> {
    fn make_err(tokens:&dyn ToTokens)->syn::Error{
        spanned_err!(tokens,"unrecognized attribute")
    }
    match (pctx, attr) {
        (
            ParseContext::Field{field,..}, 
            Meta::NameValue(MetaNameValue{lit:Lit::Str(ref value),ref path,..})
        ) => {
            let ident=path.get_ident().ok_or_else(|| make_err(&path) )?;
            
            if ident=="rename" {
                let renamed=value.parse::<IdentOrIndex>()?;
                this.renamed_fields.insert(field,Some(renamed));
            }if ident=="access" {
                let access=value.parse::<Access>()?;
                this.field_access.insert(field,access);
            }else{
                return Err(make_err(&path))?;
            }
        }
        (ParseContext::TypeAttr{..},Meta::Path(ref path)) if path.equals_str("debug_print") =>{
            this.debug_print=true;
        }
        (
            ParseContext::TypeAttr{..},
            Meta::NameValue(MetaNameValue{lit:Lit::Str(ref unparsed_lit),ref path,..})
        )=>{
            let ident=path.get_ident().ok_or_else(|| make_err(path) )?;

            if ident=="access" {
                let access=unparsed_lit.parse::<Access>()?;
                for (_,fa) in this.field_access.iter_mut() {
                    *fa=access;
                }
            }else{
                return Err(make_err(path));
            }
        }
        (_,x) => return Err(make_err(&x)),
    }
    Ok(())
}


