#![allow(unused)]

use crate::utils::span_lint;
use if_chain::if_chain;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{impl_lint_pass, declare_tool_lint};
use rustc_hir::{ImplItem, ImplItemKind, Path, PathSegment, QPath, TyKind};

declare_clippy_lint! {
    /// **What it does:** Checks for references to a concrete type where an associated type could
    /// be used in a trait implementation.
    ///
    /// **Why is this bad?** Changes to the name of the type would require changes to the
    /// implementations that use it.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// pub struct Example;
    ///
    /// pub enum State {
    ///     A,
    ///     B,
    ///     C,
    /// }
    ///
    /// pub trait SimpleTrait {
    ///     type Associated;
    ///
    ///     fn do_something_with_associated(&self, associated: &Self::Associated);
    /// }
    ///
    /// impl SimpleTrait for Example {
    ///     type Associated = State;
    ///
    ///     fn do_something_with_associated(&self, associated: &Self::Associated) {
    ///         match associated {
    ///             State::A => println!("State::A"),
    ///             State::B => println!("State::B"),
    ///             State::C => println!("State::C"),
    ///         }
    ///     }
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// impl SimpleTrait for Example {
    ///     type Associated = State;
    ///
    ///     fn do_something_with_associated(&self, associated: &Self::Associated) {
    ///         match associated {
    ///             Self::Associated::A => println!("State::A"),
    ///             Self::Associated::B => println!("State::B"),
    ///             Self::Associated::C => println!("State::C"),
    ///         }
    ///     }
    /// }
    /// ```
    pub NOT_USING_ASSOCIATED_TYPE,
    nursery,
    "concrete type used where an associated type could be used instead in trait implementations"
}

#[derive(Default)]
pub struct NotUsingAssociatedType {
    //assoc_types : Vec<(Ty, Ident)>,
}

impl_lint_pass!(NotUsingAssociatedType => [NOT_USING_ASSOCIATED_TYPE]);

impl LateLintPass<'tcx> for NotUsingAssociatedType {
    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &ImplItem<'tcx>) {
        if let ImplItemKind::TyAlias(concrete_ty) = item.kind {
            if let TyKind::Path(concrete_path) = &concrete_ty.kind {
                match concrete_path {
                    QPath::Resolved(None, _) => {
                            span_lint(
                            cx,
                            NOT_USING_ASSOCIATED_TYPE,
                            item.span,
                            "This is a TyAlias -> Resolved(None, Path)",
                        );
                    },
                    QPath::Resolved(Some(_), _) => {
                            span_lint(
                            cx,
                            NOT_USING_ASSOCIATED_TYPE,
                            item.span,
                            "This is a TyAlias -> Resolved(Some(_), Path)",
                        );
                    },
                    QPath::TypeRelative(_, _) => {
                        span_lint(
                            cx,
                            NOT_USING_ASSOCIATED_TYPE,
                            item.span,
                            "This is a TyAlias -> TypeRelative(_, _)",
                        );
                    },
                    QPath::LangItem(_, _) => {
                        span_lint(
                            cx,
                            NOT_USING_ASSOCIATED_TYPE,
                            item.span,
                            "This is a TyAlias -> LangItem(_, _)",
                        );
                    }
                }
            } else {
                span_lint(
                    cx,
                    NOT_USING_ASSOCIATED_TYPE,
                    item.span,
                    "This is a TyAlias not to a path",
                );
            }
        }
    }
/*
    fn check_impl_item_post(&mut self, cx: &LateContext<'tcx>, item: &ImplItem<'tcx>) {

    }
*/
}
