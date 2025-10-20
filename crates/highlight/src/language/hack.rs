use std::{mem::MaybeUninit, panic::UnwindSafe};

use tree_sitter_language::LanguageFn;

pub trait IntoPtr {
    fn into_ptr(self) -> *const ();
}

impl IntoPtr for ::tree_sitter::Language {
    #[inline(always)]
    fn into_ptr(self) -> *const () {
        self.into_raw().cast()
    }
}

impl<T: ?Sized> IntoPtr for *const T {
    #[inline(always)]
    fn into_ptr(self) -> *const () {
        self.cast()
    }
}

pub trait LangFunc: 'static + Send + Sync + Unpin + UnwindSafe + Copy {
    type Output: IntoPtr;

    fn call(self) -> Self::Output;
}

impl<R, F> LangFunc for F
where
    F: FnMut() -> R + Send + Sync + Unpin + UnwindSafe + Copy + 'static,
    R: IntoPtr,
{
    type Output = R;

    #[inline(always)]
    fn call(mut self) -> R {
        const { assert!(size_of::<Self>() == 0, "lang funcs must be zero sized") };
        self()
    }
}

/// When life gives you lemons...
///
/// SAFETY: Make sure that `F` is an actual Rust function that returns a valid language pointer.
#[inline(always)]
pub const unsafe fn receive_lemons<F: LangFunc>(_: F) -> LanguageFn {
    const {
        assert!(
            size_of::<F>() == 0,
            "lang funcs need to be zero sized so that we don't get uninitialized data",
        )
    };

    /// GET MAD! I DON'T WANT YOUR DAMN LEMONS, WHAT AM I SUPPOSED TO DO WITH THESE? DEMAND TO
    /// SEE LIFE's MANAGER! MAKE LIFE RUE THE DAY IT THOUGHT IT COULD GIVE [HERA] LEMONS! DO YOU
    /// KNOW WHO I AM? I'M THE [WOMAN] WHO'S GONNA BURN YOUR HOUSE DOWN! WITH THE LEMONS! I'M GOING
    /// TO GET MY ENGINEERS [US] TO INVENT A COMBUSTIBLE LEMON THAT BURNS YOUR HOUSE DOWN!
    unsafe extern "C" fn get_mad<F: LangFunc>() -> *const () {
        // SAFETY: We know that `F` is zero sized and returns a valid language pointer.
        let f = unsafe { MaybeUninit::<F>::uninit().assume_init() };

        f.call().into_ptr()
    }

    // SAFETY: We know that `get_mad` creates a valid `LanguageFn` pointer.
    unsafe { LanguageFn::from_raw(get_mad::<F>) }
}
