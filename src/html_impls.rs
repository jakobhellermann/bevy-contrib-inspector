use crate::{as_html::AsHtml, as_html::SharedOptions};

pub struct NumberAttributes<T> {
    pub min: T,
    pub max: T,
}

macro_rules! impl_ashtml_for_int {
    ($ty:ty => $default_options:expr ) => {
        impl AsHtml for $ty {
            type Options = NumberAttributes<Self>;
            const DEFAULT_OPTIONS: Self::Options = $default_options;

            fn as_html(shared_options: crate::as_html::SharedOptions<Self>, options: Self::Options, submit_fn: &str) -> String {
                format!(r#"
            <label>
            {label}
            <input type="range" min="{}" max="{}" value="{value}" oninput="{submit}(this.value)">
            </label>"#,
                    options.min, options.max,
                    submit = submit_fn,
                    value = shared_options.default,
                    label = shared_options.label,
                )
            }

            fn parse(value: &str) -> Result<Self, ()> {
                value.parse().map_err(drop)
            }
        }
    };

    ( $($ty:ty),+ => $default_options:expr ) => {
        $(impl_ashtml_for_int!{ $ty => $default_options })*
    }
}

impl_ashtml_for_int!(u8 => NumberAttributes { min: std::u8::MIN, max: std::u8::MAX });
impl_ashtml_for_int!(i8 => NumberAttributes { min: std::i8::MIN, max: std::i8::MAX });

impl_ashtml_for_int!(u16, u32, u64 => NumberAttributes { min: 0, max: 100 });
impl_ashtml_for_int!(i16, i32, i64 => NumberAttributes { min: 0, max: 100 });

impl_ashtml_for_int!(f32, f64 => NumberAttributes { min: 0.0, max: 1.0 });

impl AsHtml for String {
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(shared: SharedOptions<Self>, (): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
            <label>
            {label}
            <input type="text" value="{value}" oninput="{}(this.value)">
            </label>"#,
            submit_fn,
            value = shared.default,
            label = shared.label,
        )
    }

    fn parse(value: &str) -> Result<Self, ()> {
        Ok(value.to_string())
    }
}

impl AsHtml for bool {
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(shared: SharedOptions<Self>, (): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
            <label>
            {label}
            <input type="checkbox" {checked} onchange="{}(this.checked)">
            </label>
            "#,
            submit_fn,
            checked = if shared.default { "checked" } else { "" },
            label = shared.label,
        )
    }

    fn parse(value: &str) -> Result<Self, ()> {
        value.parse().map_err(drop)
    }
}
