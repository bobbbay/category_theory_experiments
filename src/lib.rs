use anyhow::Result;

/// This function implements an id in category theory. It takes an arrow and returns it, genius!
fn id<T>(x: T) -> Result<T> {
    Ok(x)
}

fn compose<T, U, V, W>(x: T, y: U) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    type A = i8;
    type B = i16;
    type C = i32;

    fn f<T: Into<U>, U>(x: T) -> Result<U> {
        return Ok(x.into());
    }

    #[test]
    fn id_after_f() -> Result<()> {
        let a = A::from(0);
        let res = id(f::<_, B>(a)?)?;

        dbg!(res);

        Ok(())
    }

    #[test]
    fn f_after_id() -> Result<()> {
        let a = A::from(0);
        let res = f::<_, B>(id(a)?)?;

        dbg!(res);

        Ok(())
    }

    #[test]
    fn compose_f_after_id() -> Result<()> {
        let res = compose::<_, _, A, B>(&f::<A, B>, &id::<A>)?;

        // We cannot print out res, because it's a function and I don't know how to print these out.

        Ok(())
    }
}
