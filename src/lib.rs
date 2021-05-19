use anyhow::Result;

/// This function implements an 'id' in category theory. It takes an arrow and returns it: genius!
fn id<T>(x: T) -> T {
    x
}

// This function takes two functions and returns their composition
fn compose<T: 'static + Fn(V) -> W, U: 'static + Fn(W) -> Y, V, W, Y>(
    x: U,
    y: T,
) -> Box<dyn Fn(V) -> Y> {
    Box::new(move |a| x(y(a)))
}

#[cfg(test)]
mod tests {
    use super::*;

    type A = i8;
    type B = i16;
    type C = i32;

    fn dbg_type<T: std::fmt::Display>(x: T) {
        println!("[Type of value `{}`] = {}", x, std::any::type_name::<T>())
    }

    fn f<T: Into<U>, U>(x: T) -> U {
        return x.into();
    }

    fn g<T: Into<U>, U>(x: T) -> U {
        return x.into();
    }

    #[test]
    fn id_after_f() -> Result<()> {
        let a = A::from(0);
        let res = id(f::<_, B>(a));

        dbg!(res);

        Ok(())
    }

    #[test]
    fn f_after_id() -> Result<()> {
        let a = A::from(0);
        let res = f::<_, B>(id(a));

        dbg!(res);

        Ok(())
    }

    #[test]
    fn compose_f_after_id() -> Result<()> {
        let composition = compose(f::<A, B>, id::<A>);

        let val = A::from(0);

        let res = composition(val);

        dbg!(&res);
        // The type of the value before composing
        dbg_type(val);
        // The type of the value after composing
        dbg_type(res);

        Ok(())
    }

    #[test]
    fn compose_id_after_f() -> Result<()> {
        let composition = compose(id, f::<A, B>);

        let val = A::from(0);

        let res = composition(val);

        dbg!(&res);
        // The type of the value before composing
        dbg_type(val);
        // The type of the value after composing
        dbg_type(res);

        Ok(())
    }

    #[test]
    fn compose_g_after_f() -> Result<()> {
        let composition = compose(g::<_, C>, f::<A, B>);

        let val = A::from(0);

        let res = composition(val);

        dbg!(&res);
        // The type of the value before composing
        dbg_type(val);
        // The type of the value after composing
        dbg_type(res);

        Ok(())
    }
}
