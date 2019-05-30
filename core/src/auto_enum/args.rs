use proc_macro2::{token_stream::IntoIter, Delimiter, Group, Ident, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Path, Result};

// =============================================================================
// Arg

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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Arg::Ident(i) => i.to_tokens(tokens),
            Arg::Path(p) => p.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "type_analysis")]
impl From<Ident> for Arg {
    fn from(ident: Ident) -> Self {
        Arg::Ident(ident)
    }
}

#[cfg(feature = "type_analysis")]
impl From<Path> for Arg {
    fn from(path: Path) -> Self {
        Arg::Path(path)
    }
}

#[cfg(feature = "type_analysis")]
impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
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

#[cfg(feature = "type_analysis")]
impl Eq for Arg {}

// =============================================================================
// Parse

macro_rules! error {
    ($msg:expr) => {
        syn::Error::new($msg.span(), $msg)
    };
    ($span:expr, $msg:expr) => {
        return Err(syn::Error::new($span.span(), $msg))
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}

pub(super) fn parse_group(group: TokenStream) -> Result<(Vec<Arg>, Option<String>, bool)> {
    syn::parse2::<Group>(group).map_err(|e| error!(e)).and_then(|group| parse_args(group.stream()))
}

pub(super) fn parse_args(args: TokenStream) -> Result<(Vec<Arg>, Option<String>, bool)> {
    const ERR: &str = "expected one of `,`, `::`, or identifier, found ";

    let mut iter = args.into_iter();
    let mut args = Vec::new();
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
                        error!(i, "multiple `never` option")
                    }
                    never = true;
                }
                _ => args.push(path_or_ident(i, iter.next(), &mut iter)?),
            },
            TokenTree::Punct(p) => match p.as_char() {
                ',' => {}
                ':' => args.push(parse_path(vec![p.into()], &mut iter)?),
                _ => error!(p, "{}`{}`", ERR, p),
            },
            _ => error!(tt, "{}`{}`", ERR, tt),
        }
    }

    Ok((args, marker, never))
}

fn parse_path(mut path: Vec<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    for tt in iter {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == ',' => break,
            tt => path.push(tt),
        }
    }

    syn::parse2(path.into_iter().collect()).map_err(|e| error!(e)).map(Arg::Path)
}

fn path_or_ident(ident: Ident, tt: Option<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    const ERR: &str = "expected one of `,`, or `::`, found ";

    match tt {
        None => Ok(Arg::Ident(ident)),
        Some(TokenTree::Punct(p)) => match p.as_char() {
            ',' => Ok(Arg::Ident(ident)),
            ':' => parse_path(vec![ident.into(), p.into()], iter),
            _ => error!(p, "{}`{}`", ERR, p),
        },
        Some(tt) => error!(tt, "{}`{}`", ERR, tt),
    }
}

fn marker_opt(
    ident: Ident,
    iter: &mut IntoIter,
    args: &mut Vec<Arg>,
    marker: &mut Option<String>,
) -> Result<()> {
    match iter.next() {
        Some(TokenTree::Group(ref g)) if g.delimiter() != Delimiter::Parenthesis => {
            error!(g, "invalid delimiter")
        }
        Some(TokenTree::Group(ref g)) => {
            let mut g = g.stream().into_iter();
            match g.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => error!(tt, "expected an identifier, found `{}`", tt),
                None => error!(ident, "empty `marker` option"),
            }
            match g.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {
                    if let Some(tt) = g.next() {
                        error!(tt, "multiple identifier in `marker` option")
                    }
                }
                Some(tt) => error!(tt, "multiple identifier in `marker` option"),
            }
        }
        Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {
            match iter.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => error!(tt, "expected an identifier, found `{}`", tt),
                None => error!(p, "empty `marker` option"),
            }
            match iter.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
                Some(tt) => error!(tt, "multiple identifier in `marker` option"),
            }
        }
        tt => args.push(path_or_ident(ident, tt, iter)?),
    }

    Ok(())
}
