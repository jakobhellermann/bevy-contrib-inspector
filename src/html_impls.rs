use crate::AsHtml;

pub struct NumberAttributes<T> {
    pub min: T,
    pub max: T,
    pub default: T,
}

macro_rules! impl_ashtml_for_int {
    ($ty:ty, $default_options:expr ) => {
        impl AsHtml for $ty {
            type Options = NumberAttributes<Self>;
            const DEFAULT_OPTIONS: Self::Options = $default_options;

            fn as_html(options: Self::Options, submit_fn: &str) -> String {
                format!(
                    r#"
        <input type="range" min="{}" max="{}" value="{}" oninput="{}(this.value)">
        "#,
                    options.min, options.max, options.default, submit_fn
                )
            }

            fn parse(value: &str) -> Result<Self, ()> {
                value.parse().map_err(drop)
            }
        }
    };

    ( $($ty:ty),+ => $default_options:expr ) => {
        $(impl_ashtml_for_int!{ $ty, $default_options })*
    }
}

impl_ashtml_for_int!(u8, u16, u32, u64 => NumberAttributes { min: 0, max: 100, default: 50});
impl_ashtml_for_int!(i8, i16, i32, i64 => NumberAttributes { min: 0, max: 100, default: 50});
impl_ashtml_for_int!(f32, f64 => NumberAttributes { min: 0.0, max: 1.0, default: 0.5 });

impl AsHtml for String {
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html((): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
        <input type="text" oninput="{}(this.value)">
        "#,
            submit_fn
        )
    }

    fn parse(value: &str) -> Result<Self, ()> {
        Ok(value.to_string())
    }
}

impl AsHtml for bool {
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html((): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
        <input type="checkbox" onchange="{}(this.checked)">
        "#,
            submit_fn
        )
    }

    fn parse(value: &str) -> Result<Self, ()> {
        value.parse().map_err(drop)
    }
}
