use crate::{as_html::AsHtml, as_html::SharedOptions};
use bevy::prelude::*;

pub struct NumberAttributes<T> {
    pub min: T,
    pub max: T,
    pub step: T,
}

macro_rules! impl_ashtml_for_int {
    ($ty:ty => $default_options:expr; $err:ty ) => {
        impl AsHtml for $ty {
            type Err = $err;
            type Options = NumberAttributes<Self>;
            const DEFAULT_OPTIONS: Self::Options = $default_options;

            fn header() -> &'static str {
                concat!(
                    "<script>",
                    r#"var Numscrubber={};Numscrubber.init=function(){for(var a=document.querySelectorAll("input"),b=0;b<a.length;b++)if("number"==a[b].type&&null!==a[b].getAttribute("data-numscrubber")){a[b].readOnly=!0,a[b].setAttribute("style","-moz-appearance: textfield");var c=document.createElement("span");document.body.appendChild(c),a[b].parentElement.replaceChild(c,a[b]),c.style.position="relative",c.appendChild(a[b]),c.style.width=a[b].offsetWidth+"px",c.style.height=a[b].offsetHeight+"px";var d=document.createElement("input");d.setAttribute("type","range"),c.appendChild(d),""!=a[b].getAttribute("disabled")&&1!=a[b].getAttribute("disabled")||d.setAttribute("disabled",!0),d.setAttribute("step",a[b].getAttribute("step")),d.value=a[b].value,d.min=a[b].min,d.max=a[b].max;var e=a[b].currentStyle||window.getComputedStyle(a[b]);d.style.position="absolute",d.style.margin=e.margin,d.style.left=0,d.style.border="1px solid transparent",d.style.opacity=0,d.style.cursor="e-resize",d.style.width=a[b].offsetWidth+"px",d.style.height=a[b].offsetHeight+"px",function(b){d.addEventListener("input",function(){a[b].value=this.value;var c=new Event("input");a[b].dispatchEvent(c)})}(b)}};"#,
                    "</script>")
            }
            fn footer() -> &'static str {
                "<script>Numscrubber.init()</script>"
            }

            fn as_html(shared_options: crate::as_html::SharedOptions<Self>, options: Self::Options, submit_fn: &str) -> String {
                format!(r#"
            <div class="row">
                <label for="{label}" class="cell text-right">{label}:</label>
                <input class="cell" data-numscrubber type="number" min="{}" max="{}" step="{}" value="{value}" oninput="{}(this.value)" id="{label}">
            </div>
            "#,
                    options.min, options.max, options.step,
                    submit = submit_fn,
                    value = shared_options.default,
                    label = shared_options.label,
                )
            }

            fn parse(value: &str) -> Result<Self, Self::Err> {
                value.parse()
            }
        }
    };

    ( $($ty:ty),+ => $default_options:expr; $err:ty ) => {
        $(impl_ashtml_for_int!{ $ty => $default_options; $err })*
    }
}

impl_ashtml_for_int!(u8 => NumberAttributes { min: std::u8::MIN, max: std::u8::MAX, step: 1 } ; std::num::ParseIntError);
impl_ashtml_for_int!(i8 => NumberAttributes { min: std::i8::MIN, max: std::i8::MAX, step: 1 } ; std::num::ParseIntError);

impl_ashtml_for_int!(u16, u32, u64 => NumberAttributes { min: 0, max: 100, step: 1 } ; std::num::ParseIntError);
impl_ashtml_for_int!(i16, i32, i64 => NumberAttributes { min: 0, max: 100, step: 1 } ; std::num::ParseIntError);

impl_ashtml_for_int!(f32, f64 => NumberAttributes { min: 0.0, max: 1.0, step: 0.01 } ; std::num::ParseFloatError );

impl AsHtml for String {
    type Err = std::convert::Infallible;
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(shared: SharedOptions<Self>, (): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
            <div class="row">
                <label for="{label}" class="cell text-right">{label}:</label>
                <input class="cell" type="text" value="{value}" oninput="{}(this.value)" id="{label}">
            </div>
            "#,
            submit_fn,
            value = shared.default,
            label = shared.label,
        )
    }

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(value.to_string())
    }
}

impl AsHtml for bool {
    type Err = std::str::ParseBoolError;
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(shared: SharedOptions<Self>, (): Self::Options, submit_fn: &str) -> String {
        format!(
            r#"
            <div class="row">
                <label for="{label}" class="cell text-right">{label}:</label>
                <input class="cell" type="checkbox" {checked} oninput="{}(this.checked)" id="{label}">
            </div>
            "#,
            submit_fn,
            checked = if shared.default { "checked" } else { "" },
            label = shared.label,
        )
    }

    fn parse(value: &str) -> Result<Self, Self::Err> {
        value.parse()
    }
}

fn color_to_string(c: &Color) -> String {
    use std::fmt::Write;

    let mut s = String::with_capacity(6);
    s.push('#');
    write!(s, "{:02x}", (c.r * 255.0) as u8).unwrap();
    write!(s, "{:02x}", (c.g * 255.0) as u8).unwrap();
    write!(s, "{:02x}", (c.b * 255.0) as u8).unwrap();
    s
}
fn string_to_color(s: &str) -> Result<Color, ()> {
    if !s.starts_with('#') || !s.len() == 7 {
        return Err(());
    }

    let r = u8::from_str_radix(&s[1..=2], 16).map_err(drop)?;
    let g = u8::from_str_radix(&s[3..=4], 16).map_err(drop)?;
    let b = u8::from_str_radix(&s[5..=6], 16).map_err(drop)?;

    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    Ok(Color::rgb(r, g, b))
}
impl AsHtml for Color {
    type Err = ();
    type Options = ();

    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(shared: SharedOptions<Self>, (): Self::Options, submit_fn: &'static str) -> String {
        format!(
            r#"<div class="row">
                <label for="{label}" class="cell text-right">{label}:</label>
                <input class="cell" type="color" value={default} oninput="{}(this.value)" id="{label}">
            </div>"#,
            submit_fn,
            label = shared.label,
            default = color_to_string(&shared.default),
        )
    }

    fn parse(value: &str) -> Result<Self, Self::Err> {
        string_to_color(value)
    }
}
