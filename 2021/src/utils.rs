// Credit: https://stackoverflow.com/a/51345372/392331
macro_rules! ok_or_return {
    ( $r:expr ) => {
        match $r {
            Ok(x) => x,
            Err(e) => return Err(e),
        }
    };
}

macro_rules! ok_or_return_s {
    ( $r:expr ) => {
        match $r {
            Ok(x) => x,
            Err(e) => return Err(format!("{}", e)),
        }
    };
}

macro_rules! some_or_return {
    ( $o:expr, $e:expr ) => {
        match $o {
            Some(x) => x,
            None => return Err($e),
        }
    };
}
