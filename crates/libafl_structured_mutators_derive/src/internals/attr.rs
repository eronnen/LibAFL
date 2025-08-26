use crate::internals::Ctxt;

/// Represents struct or enum attribute information.
pub struct Container {
    default: DefaultValue,
}

impl Container {
    /// Extract out the `#[serde(...)]` attributes from an item.
    pub fn from_ast(_cx: &Ctxt, _item: &syn::DeriveInput) -> Self {
        Container {
            default: DefaultValue::None,
        }
    }

    pub fn default(&self) -> &DefaultValue {
        &self.default
    }
}

/// Represents variant attribute information
pub struct Variant;

impl Variant {
    pub fn from_ast(_cx: &Ctxt, _variant: &syn::Variant) -> Self {
        Variant {}
    }
}

/// Represents the default to use for a field when deserializing.
pub enum DefaultValue {
    /// Field must always be specified because it does not have a default.
    None,
    /// The default is given by `std::default::Default::default()`.
    Default,
    /// The default is given by this function.
    Path(syn::ExprPath),
}

impl DefaultValue {
    pub fn is_none(&self) -> bool {
        match self {
            DefaultValue::None => true,
            DefaultValue::Default | DefaultValue::Path(_) => false,
        }
    }
}

/// Represents field attribute information
pub struct Field;

impl Field {
    /// Extract out the `#[serde(...)]` attributes from a struct field.
    pub fn from_ast(
        _cx: &Ctxt,
        _index: usize,
        _field: &syn::Field,
        _attrs: Option<&Variant>,
        _container_default: &DefaultValue,
    ) -> Self {
        Field {}
    }
}
