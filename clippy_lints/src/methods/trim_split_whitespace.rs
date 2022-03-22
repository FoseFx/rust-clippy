use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::ty::{get_associated_type, implements_trait};
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_middle::ty::Ty;
use rustc_span::{sym, Span};

use super::TRIM_SPLIT_WHITESPACE;

/// This is called when `recv.trim().split_whitespace()` was found
pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    split_ws_expr: &Expr<'_>,
    recv: &'tcx Expr<'tcx>,
    trim_fn_name: &str,
    trim_span: Span,
) {
    let recv_ty = cx.typeck_results().expr_ty(recv).peel_refs(); // type of recv

    if recv_ty.is_str() || implements_deref_str(cx, recv_ty) {
        span_lint_and_sugg(
            cx,
            TRIM_SPLIT_WHITESPACE,
            split_ws_expr.span.with_lo(trim_span.lo()),
            format!("found call to `str::{}` before `str::split_whitespace`", trim_fn_name).as_str(),
            format!("remove `{}()`", trim_fn_name).as_str(),
            "split_whitespace()".to_string(),
            Applicability::MachineApplicable,
        );
    }
}

/// does ty implement Deref<Target=str>?
fn implements_deref_str<'t>(cx: &LateContext<'t>, ty: Ty<'t>) -> bool {
    if_chain! {
        if let Some(deref_trait_id) = cx.tcx.get_diagnostic_item(sym::Deref);
        if implements_trait(cx, ty, deref_trait_id, &[]);
        if let Some(ty) = get_associated_type(cx, ty, deref_trait_id, "Target");
        if ty.is_str();
        then {
            true
        } else {
            false
        }
    }
}
