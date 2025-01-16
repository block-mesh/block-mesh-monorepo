#[macro_export]
macro_rules! log_rejection {
    (
        rejection_type = $ty:ident,
        body_text = $body_text:expr,
        status = $status:expr,
    ) => {};
}

/// Private API.
#[macro_export]
macro_rules! define_rejection {
    (
        #[status = $status:ident]
        #[body = $body:literal]
        $(#[$m:meta])*
        pub struct $name:ident;
    ) => {
        $(#[$m])*
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct $name;

        impl $name {
            /// Get the response body text used for this rejection.
            pub fn body_text(&self) -> String {
                self.to_string()
            }

            /// Get the status code used for this rejection.
            pub const fn status(&self) -> http::StatusCode {
                http::StatusCode::$status
            }
        }

        impl IntoResponse for $name {
            fn into_response(self) -> Response {
                let status = self.status();

                $crate::log_rejection!(
                    rejection_type = $name,
                    body_text = $body,
                    status = status,
                );
                (status, $body).into_response()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $body)
            }
        }

        impl std::error::Error for $name {}

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }
    };

    (
        #[status = $status:ident]
        #[body = $body:literal]
        $(#[$m:meta])*
        pub struct $name:ident (Error);
    ) => {
        $(#[$m])*
        #[derive(Debug)]
        pub struct $name(pub(crate) Error);

        impl $name {
            pub(crate) fn from_err<E>(err: E) -> Self
            where
                E: Into<BoxError>,
            {
                Self(Error::new(err))
            }

            /// Get the response body text used for this rejection.
            pub fn body_text(&self) -> String {
                self.to_string()
            }

            /// Get the status code used for this rejection.
            pub const fn status(&self) -> http::StatusCode {
                http::StatusCode::$status
            }
        }

        impl IntoResponse for $name {
            fn into_response(self) -> Response {
                let status = self.status();
                let body_text = self.body_text();

                $crate::log_rejection!(
                    rejection_type = $name,
                    body_text = body_text,
                    status = status,
                );
                (status, body_text).into_response()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str($body)?;
                f.write_str(": ")?;
                self.0.fmt(f)
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(&self.0)
            }
        }
    };
}

/// Private API.
#[doc(hidden)]
#[macro_export]
macro_rules! composite_rejection {
    (
        $(#[$m:meta])*
        pub enum $name:ident {
            $($variant:ident),+
            $(,)?
        }
    ) => {
        $(#[$m])*
        #[derive(Debug)]
        #[non_exhaustive]
        pub enum $name {
            $(
                #[allow(missing_docs)]
                $variant($variant)
            ),+
        }

        impl IntoResponse for $name {
            fn into_response(self) -> Response {
                match self {
                    $(
                        Self::$variant(inner) => inner.into_response(),
                    )+
                }
            }
        }

        impl $name {
            /// Get the response body text used for this rejection.
            pub fn body_text(&self) -> String {
                match self {
                    $(
                        Self::$variant(inner) => inner.body_text(),
                    )+
                }
            }

            /// Get the status code used for this rejection.
            pub const fn status(&self) -> http::StatusCode {
                match self {
                    $(
                        Self::$variant(inner) => inner.status(),
                    )+
                }
            }
        }

        $(
            impl From<$variant> for $name {
                fn from(inner: $variant) -> Self {
                    Self::$variant(inner)
                }
            }
        )+

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(inner) => write!(f, "{inner}"),
                    )+
                }
            }
        }

        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    $(
                        Self::$variant(inner) => inner.source(),
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_deref {
    ($ident:ident) => {
        impl<T> std::ops::Deref for $ident<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $ident<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ident:ident: $ty:ty) => {
        impl std::ops::Deref for $ident {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $ident {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
