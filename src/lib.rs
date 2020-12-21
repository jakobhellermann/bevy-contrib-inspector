//! This crate provides the ability to annotate structs with a `#[derive(Inspectable)]`,
//! which opens a web interface (by default on port 5676) where you can visually edit the values of your struct live.
//!
//! Your struct will then be available to you as a bevy resource.
//!
//! ## Example
//! ```rust
//! use bevy_contrib_inspector::Inspectable;
//!
//! #[derive(Inspectable, Default)]
//! struct Data {
//!     should_render: bool,
//!     text: String,
//!     #[inspectable(min = 42.0, max = 100.0)]
//!     size: f32,
//! }
//! ```
//! Add the [`InspectorPlugin`] to your App.
//! ```rust,no_run
//! use bevy_contrib_inspector::InspectorPlugin;
//! # use bevy::prelude::*;
//!
//! # #[derive(bevy_contrib_inspector::Inspectable, Default)] struct Data {}
//! fn main() {
//!     App::build()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(InspectorPlugin::<Data>::new())
//!         .add_system(your_system.system())
//!         // ...
//!         .run();
//! }
//!
//! # fn your_system() {}
//! // fn your_system(data: Res<Data>, mut query: Query<...>) { /* */ }
//! ```
//! To automatically open the webbrowser when starting, run your program using `BEVY_INSPECTOR_OPEN=1 cargo run`.
//!
//! ## Attributes
//! When deriving the [`Inspectable`] trait, you can set options such like the port the server will run on like so:
//! ```rust
//! # struct O { a: u8, b: u8, c: u8 }
//! # #[derive(Default)] struct Type {}
//! # impl bevy_contrib_inspector::AsHtml for Type {
//! #     type Err = ();
//! #     type Options = O;
//! #     const DEFAULT_OPTIONS: Self::Options = O { a: 0, b: 0, c: 0 };
//! #     fn as_html(_: bevy_contrib_inspector::as_html::SharedOptions<Self>, _: Self::Options, _: &'static str) -> String { todo!() }
//! #     fn parse(_: &str) -> Result<Self, Self::Err> { todo!() }
//! # }
//! #
//! # use bevy_contrib_inspector::Inspectable;
//! #[derive(Inspectable, Default)]
//! #[inspectable(port = 1234)]
//! struct Data {
//!    #[inspectable(a = 1, b = 2, c = 3)]
//!    field: Type,
//! }
//! ```
//! The attribute on the struct will accept fields of the type [`InspectableOptions`],
//! while the attributes on the fields accept those of their [`<Type as AsHtml>::Options`](as_html::AsHtml).
mod html_impls;
mod inspector_server;
mod plugin;

/// derives [AsHtml](trait.AsHtml.html)
pub use bevy_contrib_inspector_derive::AsHtml;
/// derives [Inspectable](trait.Inspectable.html)
pub use bevy_contrib_inspector_derive::Inspectable;

pub use plugin::InspectorPlugin;

/// This trait describes how a struct should be rendered in HTML.
/// It is meant to be derived, see the [crate-level docs](index.html) for that.
pub trait Inspectable: Send + Sync + 'static {
    /// The HTML code which will be rendered from the webserver.
    fn html() -> String;
    /// When recieving a PUT request, its body will be parsed as `$field:$value`.
    /// The update function is supposed to parse the value into its correct type and set it on `self`.
    fn update(&mut self, field: &str, value: &str);
    /// Describes things like the webserver's port. Can be set with a `#[inspector(option = value)]` on the struct.
    fn options() -> InspectableOptions {
        InspectableOptions::default()
    }
}

/// The `InspectableOptions` control parameters like the webserver's port.
///
/// They can be set when deriving the trait using `#[inspector(option = value)], as described in the [Attributes](index.html#attributes) section.
pub struct InspectableOptions {
    pub port: u16,
}
impl Default for InspectableOptions {
    fn default() -> Self {
        InspectableOptions { port: 5676 }
    }
}

/// Attribute-Options for the [AsHtml] trait.
pub mod as_html {
    pub use crate::html_impls::*;

    pub struct SharedOptions<T> {
        pub label: std::borrow::Cow<'static, str>,
        pub default: T,
    }

    pub use crate::AsHtml;
}

/// controls how a type is rendered in HTML
///
/// It also specifies how the type is parsed from a string
/// and what attributes you can apply to it using `#[inspector(min = 1, max = 2)]`
pub trait AsHtml: Sized + 'static {
    /// The parse error type
    type Err: std::fmt::Debug;
    /// The attibutes you can set for a field
    type Options;
    /// Default options for the `Options`-type
    const DEFAULT_OPTIONS: Self::Options;

    /// HTML that needs to go to the top of the page, e.g. <script /> tags used in [`as_html`].
    fn header() -> &'static str {
        ""
    }
    /// HTML that needs to go to the bottom of the page, e.g. initializer js glue.
    fn footer() -> &'static str {
        ""
    }

    #[doc(hidden)]
    /// This function is called in order to prevent multiple headers/footers from being loaded
    fn register_header_footer(
        types: &mut std::collections::HashSet<std::any::TypeId>,
        header: &mut String,
        footer: &mut String,
    ) {
        if types.insert(std::any::TypeId::of::<Self>()) {
            header.push_str(Self::header());
            footer.push_str(Self::footer());
        }
    }

    /// The actual html content.
    ///
    /// The `options`-parameter gives you access to the attributes passed when deriving [`Inspectable`].
    /// When there is new input, the submit function should be called with it,
    /// for example like this:
    ///
    /// `format!("<input oninput="{}('new data arrived')" />, submit_fn)`.
    fn as_html(
        shared: as_html::SharedOptions<Self>,
        options: Self::Options,
        submit_fn: String,
    ) -> String;

    /// specifies how the type should be parsed
    fn parse(value: &str) -> Result<Self, Self::Err>;

    fn update(&mut self, value: &str) -> Result<(), Self::Err> {
        let value = Self::parse(value)?;
        *self = value;
        Ok(())
    }
}
