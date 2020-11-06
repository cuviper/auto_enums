pub(crate) mod as_ref {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["AsRef"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::convert::AsRef), None, parse_quote! {
            trait AsRef<__T: ?Sized> {
                #[inline]
                fn as_ref(&self) -> &__T;
            }
        }))
    }
}

pub(crate) mod as_mut {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["AsMut"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::convert::AsMut), None, parse_quote! {
            trait AsMut<__T: ?Sized> {
                #[inline]
                fn as_mut(&mut self) -> &mut __T;
            }
        }))
    }
}
