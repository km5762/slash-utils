macro_rules! define_error {
    ($name:ident, $msg:expr) => {
        #[derive(Debug)]
        pub struct $name;

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", $msg)
            }
        }
    };
}

macro_rules! group_errors {
    ($enum_name:ident, $($variant:ident($error_type:ty)),+) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $(
                $variant($error_type),
            )+
        }

        impl core::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(
                        $enum_name::$variant(e) => write!(f, "{}", e),
                    )+
                }
            }
        }
    };
}

define_error!(
    InvalidGeneratorError,
    "the provided generator is not on the curve"
);

define_error!(
    ZeroingKError,
    "the provided k value results in a signature with a zeroed component"
);

define_error!(
    NoInvKError,
    "the provided k value has no inverse in the given modulus"
);

define_error!(
    InvalidPointError,
    "an off curve point was generated while performing this operation"
);

define_error!(
    ParseKError,
    "the provided k value could not be parsed into an integer"
);

define_error!(
    ParseKeyError,
    "the provided key could not be parsed into an integer"
);

define_error!(
    ParseHashError,
    "the provided hash could not be parsed into an integer"
);

group_errors!(
    SigningError,
    InvalidGenerator(InvalidGeneratorError),
    ZeroingK(ZeroingKError),
    NoInvK(NoInvKError),
    InvalidPoint(InvalidPointError),
    ParseK(ParseKError),
    ParseKey(ParseKeyError),
    ParseHash(ParseHashError)
);
