use crate::{as_html::AsHtml, as_html::SharedOptions};

pub struct NumberAttributes<T> {
    pub min: T,
    pub max: T,
    pub step: T,
}

macro_rules! impl_ashtml_for_int {
    ($ty:ty => $default_options:expr ) => {
        impl AsHtml for $ty {
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
                <input class="cell" data-numscrubber type="number" min="{}" max="{}" step="{}" value="{value}" oninput="{}(this.value) id="{label}">
            </div>
            "#,
                    options.min, options.max, options.step,
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

impl_ashtml_for_int!(u8 => NumberAttributes { min: std::u8::MIN, max: std::u8::MAX, step: 1 });
impl_ashtml_for_int!(i8 => NumberAttributes { min: std::i8::MIN, max: std::i8::MAX, step: 1 });

impl_ashtml_for_int!(u16, u32, u64 => NumberAttributes { min: 0, max: 100, step: 1 });
impl_ashtml_for_int!(i16, i32, i64 => NumberAttributes { min: 0, max: 100, step: 1 });

impl_ashtml_for_int!(f32, f64 => NumberAttributes { min: 0.0, max: 1.0, step: 0.01 });

impl AsHtml for String {
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

    fn parse(value: &str) -> Result<Self, ()> {
        value.parse().map_err(drop)
    }
}
