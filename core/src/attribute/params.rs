use std::borrow::Cow;

use proc_macro2::{
    token_stream::IntoIter, Delimiter, Ident, TokenStream as TokenStream2, TokenTree,
};
use quote::ToTokens;
use smallvec::{smallvec, SmallVec};
use syn::*;

use crate::utils::{Result, *};

use super::*;

#[derive(Debug, Clone, Eq)]
pub(super) enum Arg {
    Ident(Ident),
    Path(Path),
}

#[cfg(feature = "type_analysis")]
impl Arg {
    pub(super) fn to_trimed_string(&self) -> String {
        match self {
            Arg::Ident(i) => i.to_string(),
            Arg::Path(p) => p.clone().into_token_stream().to_string().replace(" ", ""),
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Arg::Ident(i) => i.to_tokens(tokens),
            Arg::Path(p) => p.to_tokens(tokens),
        }
    }
}

impl From<Ident> for Arg {
    fn from(ident: Ident) -> Self {
        Arg::Ident(ident)
    }
}

impl From<Path> for Arg {
    fn from(path: Path) -> Self {
        Arg::Path(path)
    }
}

impl PartialEq<Arg> for Arg {
    fn eq(&self, other: &Arg) -> bool {
        match self {
            Arg::Ident(x) => match other {
                Arg::Ident(y) => x.eq(y),
                Arg::Path(y) => y.is_ident(x.to_string()),
            },
            Arg::Path(x) => match other {
                Arg::Ident(y) => x.is_ident(y.to_string()),
                Arg::Path(y) => x.eq(y),
            },
        }
    }
}

#[derive(Debug)]
pub(super) struct Params {
    args: Stack<Arg>,
    marker: Cow<'static, str>,
    marker_count: usize,
    return_count: usize,
    attr_exists: bool,
    never: bool,
    closure: bool,
}

impl Params {
    fn new(args: Stack<Arg>, never: bool, marker: Option<String>) -> Self {
        Params {
            args,
            marker: marker.map_or_else(|| Cow::Borrowed(DEFAULT_MARKER), Cow::Owned),
            marker_count: 0,
            return_count: 0,
            attr_exists: false,
            never,
            closure: false,
        }
    }

    pub(super) fn args(&self) -> &[Arg] {
        &self.args
    }
    pub(super) fn count(&self) -> usize {
        self.marker_count + self.return_count
    }
    pub(super) fn marker_ident(&self) -> &str {
        &self.marker
    }
    pub(super) fn never(&self) -> bool {
        self.never
    }
    pub(super) fn attr(&self) -> bool {
        self.attr_exists
    }

    pub(super) fn count_marker<F>(&mut self, f: F)
    where
        F: FnOnce(&mut MarkerCounter<'_>),
    {
        assert_eq!(self.marker_count, 0);

        f(&mut MarkerCounter::new(
            &self.marker,
            &mut self.marker_count,
            &mut self.attr_exists,
        ));
    }

    pub(super) fn count_return<F>(&mut self, closure: bool, f: F)
    where
        F: FnOnce(&mut ReturnCounter<'_>),
    {
        assert_eq!(self.return_count, 0);
        self.closure = closure;

        f(&mut ReturnCounter::new(
            &self.marker,
            &mut self.return_count,
        ));
    }

    pub(super) fn build(&self, builder: &mut Builder) -> Result<ItemEnum> {
        builder.reserve(self.count());
        builder.build(&self.args)
    }

    pub(super) fn replacer<'a>(&'a mut self, builder: &'a mut Builder) -> Replacer<'a> {
        Replacer::new(
            &self.marker,
            self.marker_count,
            self.return_count,
            self.closure,
            builder,
        )
    }

    #[cfg(feature = "type_analysis")]
    pub(super) fn impl_traits(&mut self, ty: &Type) {
        if let Some(traits) = collect_impl_traits(ty) {
            parse_impl_traits(&mut self.args, traits);
        }
    }
}

pub(super) fn parse_args(args: TokenStream2) -> Result<Params> {
    const ERR: &str = "expected one of `,`, `::`, or identifier, found ";

    let mut iter = args.into_iter();
    let mut args = Stack::new();
    let mut marker = None;
    let mut never = false;
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(i) => match &*i.to_string() {
                "marker" => marker_opt(i, &mut iter, &mut args, &mut marker)?,
                "never" => {
                    match iter.next() {
                        None => {}
                        Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
                        tt => {
                            args.push(path_or_ident(i, tt, &mut iter)?);
                            continue;
                        }
                    }

                    if never {
                        Err(invalid_args!("multiple `never` option"))?;
                    }
                    never = true;
                }
                _ => args.push(path_or_ident(i, iter.next(), &mut iter)?),
            },
            TokenTree::Punct(p) => match p.as_char() {
                ',' => {}
                ':' => args.push(parse_path(smallvec![p.into()], &mut iter)?),
                _ => Err(invalid_args!("{}`{}`", ERR, p))?,
            },
            _ => Err(invalid_args!("{}`{}`", ERR, tt))?,
        }
    }

    Ok(Params::new(args, never, marker))
}

fn parse_path(mut path: SmallVec<[TokenTree; 16]>, iter: &mut IntoIter) -> Result<Arg> {
    for tt in iter {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == ',' => break,
            tt => path.push(tt),
        }
    }

    syn::parse2(path.into_iter().collect())
        .map_err(|e| invalid_args!(e))
        .map(Arg::Path)
}

fn path_or_ident(ident: Ident, tt: Option<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    const ERR: &str = "expected one of `,`, or `::`, found ";

    match tt {
        None => Ok(ident.into()),
        Some(TokenTree::Punct(p)) => match p.as_char() {
            ',' => Ok(ident.into()),
            ':' => parse_path(smallvec![ident.into(), p.into()], iter),
            _ => Err(invalid_args!("{}`{}`", ERR, p)),
        },
        Some(tt) => Err(invalid_args!("{}`{}`", ERR, tt)),
    }
}

fn marker_opt(
    ident: Ident,
    iter: &mut IntoIter,
    args: &mut Stack<Arg>,
    marker: &mut Option<String>,
) -> Result<()> {
    match iter.next() {
        Some(TokenTree::Group(ref g)) if g.delimiter() != Delimiter::Parenthesis => {
            Err(invalid_args!("invalid delimiter"))?
        }
        Some(TokenTree::Group(ref g)) => {
            let mut g = g.stream().into_iter();
            match g.next() {
                Some(TokenTree::Ident(_)) if marker.is_some() => {
                    Err(invalid_args!("multiple `marker` option"))?
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => Err(invalid_args!("expected an identifier, found `{}`", tt))?,
                None => Err(invalid_args!("empty `marker` option"))?,
            }

            if g.next().is_some() {
                Err(invalid_args!("multiple identifier in `marker` option"))?;
            }
        }
        Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {
            match iter.next() {
                Some(TokenTree::Ident(_)) if marker.is_some() => {
                    Err(invalid_args!("multiple `marker` option"))?
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => Err(invalid_args!("expected an identifier, found `{}`", tt))?,
                None => Err(invalid_args!("empty `marker` option"))?,
            }
            match iter.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
                Some(_) => Err(invalid_args!("multiple identifier in `marker` option"))?,
            }
        }
        tt => args.push(path_or_ident(ident, tt, iter)?),
    }

    Ok(())
}
