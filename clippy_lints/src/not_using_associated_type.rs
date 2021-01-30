#![allow(unused)]

use crate::utils::span_lint;
use if_chain::if_chain;
use rustc_lint::{LateLintPass, LateContext};
use rustc_middle::hir::map::Map;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::symbol::Ident;
use rustc_span::Span;
use rustc_hir::{*};

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

declare_lint_pass!(NotUsingAssociatedType => [NOT_USING_ASSOCIATED_TYPE]);

impl LateLintPass<'tcx> for NotUsingAssociatedType {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        let mut associated : Vec<(Ident, &QPath<'_>)> = Default::default();
        if let ItemKind::Impl {items: impl_item_refs, ..} = item.kind {
            // We make 2 passes over the contents of the impl.
            // For the first pass, we collect the associated types
            // that we need to check.
            for impl_item_ref in impl_item_refs.iter() {
                if_chain! {
                    if AssocItemKind::Type == impl_item_ref.kind;
                    let impl_item = cx.tcx.hir().impl_item(impl_item_ref.id);
                    if let ImplItemKind::TyAlias(concrete_ty) = impl_item.kind;
                    if let TyKind::Path(concrete_path) = &concrete_ty.kind;
                    then {
                        let associated_ident = impl_item.ident;
                        associated.push((associated_ident, &concrete_path));
                    }
                }
            }

            // For the second pass, we go into the function bodies and find occurrences of the
            // concrete type that matches the associated type
            for impl_item_ref in impl_item_refs.iter() {
                if_chain! {
                    if let AssocItemKind::Fn {..} = impl_item_ref.kind;
                    let impl_item = cx.tcx.hir().impl_item(impl_item_ref.id);
                    if let ImplItemKind::Fn(fn_sig, body_id) = &impl_item.kind;
                    let body = cx.tcx.hir().body(body_id.to_owned());
                    then {
                        //let visitor = MatchingPathVisitor {
                        //    cx: cx,
                        //    types_to_find: associated,
                        //};
                        //intravisit::walk_body(visitor, body);
                    }
                }
            }
        }
    }
}

fn compare_self_ty(first: Option<&Ty<'_>>, second: Option<&Ty<'_>>) -> bool {
    match first {
        None => second.is_none(),
        Some(first_ty) => {
            if let Some(second_ty) = second {
                // TODO
                true
            } else {
                false
            }
        }
    }
}

macro_rules! not_using_associated_type_lint {
    (cx, span) => {
        span_lint(cx, NOT_USING_ASSOCIATED_TYPE, span, "Used concrete type where associated type could be used instead");
    };
}

fn compare_path(first: &Path<'_>, second: &Path<'_>) -> bool {
    true
}

fn compare_path_segments(first: &PathSegment<'_>, second: &PathSegment<'_>) -> bool {
    true
}

struct MatchingPathVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    types_to_find: &'a Vec<(Ident, &'a QPath<'tcx>)>,
}

impl<'a, 'tcx> intravisit::Visitor<'tcx> for MatchingPathVisitor<'a, 'tcx> {
    type Map = Map<'tcx>;
    fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
        intravisit::NestedVisitorMap::None
    }

    fn visit_qpath(&mut self, visited_qpath: &QPath<'tcx>, id: HirId, span: Span) {
        for (use_instead,match_path) in self.types_to_find.iter() {
            match match_path {
                QPath::Resolved(match_self, match_path) => {
                    if_chain! {
                        if let QPath::Resolved(visited_self, visited_path) = visited_qpath;
                        if compare_self_ty(match_self, visited_self);
                        if compare_path(match_path, visited_path);
                        then {
                            span_lint(self.cx, NOT_USING_ASSOCIATED_TYPE, span, "Used concrete type where associated type could be used instead");
                        }
                    }
                },
                QPath::TypeRelative(match_rel, match_segment) => {
                    if_chain! {
                        if let QPath::TypeRelative(visited_rel, visited_segment) = visited_qpath;
                        if compare_self_ty(Some(match_rel), Some(visited_rel));
                        if compare_path_segments(match_segment, visited_segment);
                        then {
                            span_lint(self.cx, NOT_USING_ASSOCIATED_TYPE, span, "Used concrete type where associated type could be used instead");
                        }
                    }
                },
                
                QPath::LangItem(_,_) => {}
            }
            
        }
    }
}

/*

                span_lint(
                    cx,
                    NOT_USING_ASSOCIATED_TYPE,
                    item.span,
                    "This is a TyAlias not to a path",
                );
                */