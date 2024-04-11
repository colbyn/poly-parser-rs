use std::{process::Output, rc::Rc};

use crate::data::Parser;

pub trait Parsable {
    type Item;
    fn parser() -> Parser<Self::Item>;
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub trait Lazy where Self: Clone + 'static {
    type Item: Clone + 'static;

    fn evaluate(self) -> Self::Item;
    
    fn map<Res: Clone + 'static>(
        self,
        f: impl Fn(Self::Item) -> Res + 'static
    ) -> impl Lazy<Item=Res> {
        Thunk::wrap(move || f(self.clone().evaluate()))
    }
}

impl<O, F> Lazy for F where O: Clone + 'static, F: Fn() -> O + 'static + Clone {
    type Item = O;
    fn evaluate(self) -> Self::Item {
        (self)()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct Thunk<O> {
    pub function: Rc<dyn Fn() -> O>,
}
#[derive(Clone)]
pub struct Constant<O: Clone> {
    pub value: O,
}

impl<O> Thunk<O> {
    pub fn wrap(f: impl Fn() -> O + 'static) -> Self {
        Self { function: Rc::new(f) }
    }
    pub fn constant(value: O) -> Self where O: Clone + 'static {
        let value = value.clone();
        Self { function: Rc::new(move || value.clone()) }
    }
}

impl<O: Clone> Constant<O> {
    pub fn wrap(value: O) -> Self {
        Self { value }
    }
}

impl<O> Lazy for Thunk<O> where O: Clone + 'static {
    type Item = O;
    fn evaluate(self) -> Self::Item {
        (self.function)()
    }
}
impl<O> Lazy for Constant<O> where O: Clone + 'static {
    type Item = O;
    fn evaluate(self) -> Self::Item {
        self.value
    }
}

#[macro_export]
macro_rules! thunk {
    (($($x:ident),*$(,)?) => $expr:expr) => {
        Thunk::wrap({
            $(
                let $x = $x.clone();
            )*
            move || {
                $(
                    let $x = $x.clone();
                )*
                $expr
            }
        })
    };
    ($expr:expr) => {
        $crate::system::Thunk::wrap({
            || $expr
        })
    };
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct Lambda<I, O> {
    pub function: Rc<dyn Fn(I) -> O>
}


// impl<I, O> Lazy for Lambda<I, O> where I: Clone + 'static, O: Clone + 'static {
//     type Output = Thunk<O>;
//     fn evaluate(self) -> Self::Output {
//         Thunk::wrap(|| {

//         })
//     }
// }


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl<O> std::fmt::Debug for Thunk<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = std::any::type_name::<Thunk<O>>();
        f   .debug_tuple(name)
            .field(&"…")
            .finish()
    }
}

