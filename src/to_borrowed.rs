use std::borrow::{Borrow, Cow};

/// A trait for downgrading the lifetime of a type.
///
/// # Examples
/// ```
/// use reuse::ToBorrowed;
/// use std::borrow::Cow;
///
/// let owned: Cow<'static, str> = Cow::Owned(String::from("Hi"));
/// let mut borrowed: Cow<'_, str> = owned.to_borrowed();
///
/// assert_eq!(borrowed, "Hi");
/// assert_matches::assert_matches!(borrowed, Cow::Borrowed(_));
///
/// borrowed = Cow::Borrowed("Bye");
/// assert_eq!(owned, "Hi");
/// ```
pub trait ToBorrowed {
    type Borrowed;

    fn to_borrowed(self) -> Self::Borrowed;
}

impl<'b, 'c, B> ToBorrowed for &'c Cow<'b, B>
where
    B: ToOwned + ?Sized + 'static,
{
    type Borrowed = Cow<'c, B>;

    #[inline]
    fn to_borrowed(self) -> Cow<'c, B> {
        Cow::Borrowed(self.borrow())
    }
}

impl<'o, T> ToBorrowed for &'o Option<T>
where
    for<'t> &'t T: ToBorrowed,
{
    type Borrowed = Option<<&'o T as ToBorrowed>::Borrowed>;

    #[inline]
    fn to_borrowed(self) -> Self::Borrowed {
        self.as_ref().map(ToBorrowed::to_borrowed)
    }
}
