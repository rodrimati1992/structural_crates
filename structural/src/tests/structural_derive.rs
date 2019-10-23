use crate::*;

use structural_derive::Structural;


structural_alias!{
    trait HuhInterface{
        a:u32,
        b:u32,
    }
}

#[derive(Structural)]
struct Huh{
    pub a:u32,
    pub b:u32,
    pub c:u32,
    pub d:u32,
}

#[derive(Structural)]
struct Whoah{
    pub nah:u32,
    pub yep:u32,
    pub b:u32,
    pub a:u32,
}

fn huh_printer<This>(this:This)
where
    This:HuhInterface
{
    let (a,b)=this.fields(tstr!("a","b"));
    assert_eq!(a, &10);
    assert_eq!(b, &33);
    println!("a={} b={}", a,b);
}

#[test]
fn huh_printing(){
    huh_printer(Huh{
        a:10,
        b:33,
        c:44,
        d:66,
    });

    huh_printer(Whoah{
        nah:0x000F,
        yep:!0,
        b:33,
        a:10,
    });
}