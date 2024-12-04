#[macro_export]
macro_rules! sample {
    ($input:literal, $($p1:ident = $v1:literal),*) => {
        #[cfg(test)]
        mod tests {

            use super::*;

            const SAMPLE: &str = $input;

            paste::item! {
                $(
                    #[test]
                    fn [<test_ $p1>]() {
                        let contents = SAMPLE.trim();
                        let result = $p1(contents);
                        assert_eq!($v1, result);
                    }
                )*
            }
        }
    };
}

#[cfg(test)]
mod tests {

    fn identity(val: &str) -> String {
        val.to_string()
    }

    sample! {
        "A",
        identity="A"
    }
}
