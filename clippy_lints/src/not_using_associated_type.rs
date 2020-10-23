use if_chain::if_chain;
use rustc_lint::{EarlyLintPass, EarlyContext};
use rustc_middle::lint::in_external_macro;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_ast::ast::*;

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
    assoc_types : Vec<AssocItem>,
}

impl NotUsingAssociatedType {
    fn new() -> Self {
        Default::default()
    }
}

impl_lint_pass!(NotUsingAssociatedType => [NOT_USING_ASSOCIATED_TYPE]);

impl EarlyLintPass for NotUsingAssociatedType {
    
    fn check_impl_item(&mut self, cx: &EarlyContext<'_>, item: &AssocItem) {
        if let TyAlias(_,_,_,Some(concrete_ty)) = item.kind {
            self.assoc_types.push(item);
        } 
    }

    fn check_impl_item_post(&mut self, cx: &EarlyContext<'_>, item: &AssocItem) {

    }
}
