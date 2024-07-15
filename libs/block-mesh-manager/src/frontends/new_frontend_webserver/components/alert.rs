use leptos::*;
use tailwind_fuse::*;

#[derive(TwVariant)]
pub enum AlertSize {
    #[tw(class = "sm:max-w-xs")]
    Xs,
    #[tw(class = "sm:max-w-sm")]
    Sm,
    #[tw(default, class = "sm:max-w-md")]
    Md,
    #[tw(class = "sm:max-w-lg")]
    Lg,
    #[tw(class = "sm:max-w-xl")]
    Xl,
    #[tw(class = "sm:max-w-2xl")]
    X2l,
    #[tw(class = "sm:max-w-3xl")]
    X3l,
    #[tw(class = "sm:max-w-4xl")]
    X4l,
    #[tw(class = "sm:max-w-5xl")]
    X5l,
}
