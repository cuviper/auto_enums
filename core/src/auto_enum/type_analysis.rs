use quote::ToTokens;
use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::*;

pub(super) fn collect_impl_trait(args: &mut Vec<Path>, ty: &mut Type) {
    fn to_trimed_string(path: &Path) -> String {
        path.to_token_stream().to_string().replace(" ", "")
    }

    let mut traits = Vec::new();
    CollectImplTrait::new(&mut traits).visit_type_mut(ty);

    traits.into_iter().for_each(|t| {
        if !args.contains(&t) && TRAITS.contains(&&*to_trimed_string(&t)) {
            args.push(t);
        }
    });
}

struct CollectImplTrait<'a> {
    traits: &'a mut Vec<Path>,
}

impl<'a> CollectImplTrait<'a> {
    fn new(traits: &'a mut Vec<Path>) -> Self {
        Self { traits }
    }
}

impl VisitMut for CollectImplTrait<'_> {
    fn visit_type_impl_trait_mut(&mut self, node: &mut TypeImplTrait) {
        visit_mut::visit_type_impl_trait_mut(self, node);

        node.bounds.iter().for_each(|ty| {
            if let TypeParamBound::Trait(ty) = ty {
                self.traits.push(path(ty.path.segments.iter().map(|ty| ty.ident.clone().into())));
            }
        });
    }
}

const TRAITS: &[&str] = &[
    "Clone",
    "Copy",
    "PartialEq",
    "Eq",
    "PartialOrd",
    "Ord",
    "Hash",
    // core
    "AsRef",
    "AsMut",
    "Debug",
    "fmt::Debug",
    "Display",
    "fmt::Display",
    "fmt::Binary",
    "fmt::LowerExp",
    "fmt::LowerHex",
    "fmt::Octal",
    "fmt::Pointer",
    "fmt::UpperExp",
    "fmt::UpperHex",
    "fmt::Write",
    "Iterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "FusedIterator",
    "TrustedLen",
    "Extend",
    "Deref",
    "DerefMut",
    "Index",
    "IndexMut",
    "RangeBounds",
    "Fn",
    "FnMut",
    "FnOnce",
    "Generator",
    "Future",
    // std
    "Read",
    "io::Read",
    "BufRead",
    "io::BufRead",
    "Write",
    "io::Write",
    "Seek",
    "io::Seek",
    "Error",
    "error::Error",
];
