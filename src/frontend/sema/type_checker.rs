//! Expression-level CType inference for semantic analysis.
//!
//! This module provides CType inference for expressions using only sema-available
//! state: SymbolTable (variable/function types), TypeContext (typedefs, struct
//! layouts, enum constants), and FunctionInfo (function signatures).
//!
//! This is a key step toward making sema produce a typed AST: by inferring
//! expression types in sema rather than the lowerer, we can:
//! - Report type errors as diagnostics instead of lowering panics
//! - Enable future type annotations on AST nodes
//! - Reduce the lowerer's responsibility from type-checking + IR emission to
//!   just IR emission
//!
//! The `ExprTypeChecker` operates on immutable references and does not modify
//! any state. It is designed to be called from `SemanticAnalyzer::analyze_expr`.

use crate::common::types::{AddressSpace, CType, FunctionType};
use crate::common::symbol_table::SymbolTable;
use crate::common::fx_hash::FxHashMap;
use crate::frontend::parser::ast::{
    BinOp,
    BlockItem,
    CompoundStmt,
    DerivedDeclarator,
    Expr,
    ExprId,
    GenericAssociation,
    Stmt,
    StructFieldDecl,
    TypeSpecifier,
    UnaryOp,
};
use super::type_context::TypeContext;
use super::analysis::FunctionInfo;

/// Determine the C type of an enum constant value, following GCC's promotion rules.
/// GCC uses the progression: int -> unsigned int -> long long -> unsigned long long.
/// (On LP64 targets, long == long long, so we skip long/unsigned long.)
pub fn enum_constant_type(val: i64) -> CType {
    if val >= i32::MIN as i64 && val <= i32::MAX as i64 {
        CType::Int
    } else if val >= 0 && val <= u32::MAX as i64 {
        CType::UInt
    } else if val >= 0 {
        CType::ULongLong
    } else {
        CType::LongLong
    }
}

/// Expression type checker that infers CTypes using only sema-available state.
///
/// This struct borrows the sema state needed for type inference and provides
/// methods to compute the CType of any expression. Unlike the lowerer's
/// `get_expr_ctype`, this does not depend on IR-level state (allocas, IR types,
/// global variable metadata).
///
/// When `expr_types` is provided, previously-computed types are looked up in O(1)
/// instead of re-traversing the AST. This prevents exponential blowup on deeply
/// nested expressions like `1+1+1+...+1` where `infer_binop_ctype` would
/// otherwise recursively re-evaluate lhs/rhs types multiple times per node.
pub struct ExprTypeChecker<'a> {
    /// Symbol table for variable/function type lookup.
    pub symbols: &'a SymbolTable,
    /// Type context for typedef, enum, and struct layout resolution.
    pub types: &'a TypeContext,
    /// Function signatures for return type resolution.
    pub functions: &'a FxHashMap<String, FunctionInfo>,
    /// Pre-computed expression types from bottom-up sema walk (memoization cache).
    /// When set, `infer_expr_ctype` checks this map before recursing.
    pub expr_types: Option<&'a FxHashMap<ExprId, CType>>,
}

impl<'a> ExprTypeChecker<'a> {
    /// Infer the CType of an expression.
    ///
    /// Returns `Some(CType)` when the type can be determined from sema state,
    /// `None` when the type depends on lowering-specific information (e.g.,
    /// complex expression chains through typeof).
    pub fn infer_expr_ctype(&self, expr: &Expr) -> Option<CType> {
        // Memoization: if this expression's type was already computed during
        // the bottom-up sema walk, return it in O(1) instead of re-traversing.
        // This prevents O(2^N) blowup on deep expression chains.
        if let Some(cache) = self.expr_types {
            if let Some(cached) = cache.get(&expr.id()) {
                return Some(cached.clone());
            }
        }

        match expr {
            // Literals have well-defined types
            Expr::IntLiteral(val, _) => {
                // After the lexer fix, IntLiteral should only contain values
                // that fit in int. But defensively handle larger values too:
                // C11 6.4.4.1: decimal without suffix: int -> long -> long long
                if *val >= i32::MIN as i64 && *val <= i32::MAX as i64 {
                    Some(CType::Int)
                } else if crate::common::types::target_is_32bit() {
                    // ILP32: int and long are both 32-bit, promote to long long
                    Some(CType::LongLong)
                } else {
                    // LP64: long is 64-bit
                    Some(CType::Long)
                }
            }
            Expr::CharLiteral(_, _) => Some(CType::Int),
            Expr::UIntLiteral(val, _) => {
                if crate::common::types::target_is_32bit() && *val > u32::MAX as u64 {
                    Some(CType::ULongLong)
                } else {
                    Some(CType::UInt)
                }
            }
            Expr::LongLiteral(val, _) => {
                // On ILP32, long is 32-bit. If value doesn't fit, promote to long long.
                // C11 6.4.4.1: for 'l' suffix, type is: long, long long (decimal)
                //               or: long, unsigned long, long long, unsigned long long (hex/octal)
                if crate::common::types::target_is_32bit() {
                    if *val >= i32::MIN as i64 && *val <= i32::MAX as i64 {
                        Some(CType::Long) // fits in 32-bit long
                    } else {
                        Some(CType::LongLong) // promote to 64-bit long long
                    }
                } else {
                    Some(CType::Long)
                }
            }
            Expr::ULongLiteral(val, _) => {
                // On ILP32, unsigned long is 32-bit. If value doesn't fit, promote to unsigned long long.
                if crate::common::types::target_is_32bit() && *val > u32::MAX as u64 {
                    Some(CType::ULongLong)
                } else {
                    Some(CType::ULong)
                }
            }
            // long long is always 64-bit, regardless of target
            Expr::LongLongLiteral(_, _) => Some(CType::LongLong),
            Expr::ULongLongLiteral(_, _) => Some(CType::ULongLong),
            Expr::FloatLiteral(_, _) => Some(CType::Double),
            Expr::FloatLiteralF32(_, _) => Some(CType::Float),
            Expr::FloatLiteralLongDouble(_, _, _) => Some(CType::LongDouble),
            Expr::ImaginaryLiteral(_, _) => Some(CType::ComplexDouble),
            Expr::ImaginaryLiteralF32(_, _) => Some(CType::ComplexFloat),
            Expr::ImaginaryLiteralLongDouble(_, _, _) => Some(CType::ComplexLongDouble),
            Expr::StringLiteral(_, _) => Some(CType::Pointer(Box::new(CType::Char), AddressSpace::Default)),
            Expr::WideStringLiteral(_, _) => Some(CType::Pointer(Box::new(CType::Int), AddressSpace::Default)),
            Expr::Char16StringLiteral(_, _) => Some(CType::Pointer(Box::new(CType::UShort), AddressSpace::Default)),

            // Identifiers: look up in symbol table or enum constants
            Expr::Identifier(name, _) => {
                if name == "__func__" || name == "__FUNCTION__" || name == "__PRETTY_FUNCTION__" {
                    return Some(CType::Pointer(Box::new(CType::Char), AddressSpace::Default));
                }
                if let Some(&val) = self.types.enum_constants.get(name) {
                    return Some(enum_constant_type(val));
                }
                if let Some(sym) = self.symbols.lookup(name) {
                    return Some(sym.ty.clone());
                }
                // Check function signatures for implicitly declared functions
                if let Some(func_info) = self.functions.get(name) {
                    return Some(CType::Function(Box::new(FunctionType {
                        return_type: func_info.return_type.clone(),
                        params: func_info.params.clone(),
                        variadic: func_info.variadic,
                    })));
                }
                None
            }

            // Cast: type is the target type
            Expr::Cast(type_spec, _, _) => {
                Some(self.resolve_type_spec(type_spec))
            }

            // Sizeof and Alignof always produce size_t (unsigned long on 64-bit)
            Expr::Sizeof(_, _) | Expr::Alignof(_, _) | Expr::AlignofExpr(_, _)
            | Expr::GnuAlignof(_, _) | Expr::GnuAlignofExpr(_, _) => Some(CType::ULong),

            // Address-of wraps in Pointer
            Expr::AddressOf(inner, _) => {
                if let Some(inner_ct) = self.infer_expr_ctype(inner) {
                    Some(CType::Pointer(Box::new(inner_ct), AddressSpace::Default))
                } else {
                    // Even if inner type unknown, result is some pointer
                    Some(CType::Pointer(Box::new(CType::Void), AddressSpace::Default))
                }
            }

            // Dereference peels off one Pointer/Array layer
            Expr::Deref(inner, _) => {
                if let Some(inner_ct) = self.infer_expr_ctype(inner) {
                    match inner_ct {
                        CType::Pointer(pointee, _) => Some(*pointee),
                        CType::Array(elem, _) => Some(*elem),
                        // Dereferencing a function is a no-op in C
                        CType::Function(_) => Some(inner_ct),
                        _ => None,
                    }
                } else {
                    None
                }
            }

            // Array subscript peels off one Array/Pointer layer
            Expr::ArraySubscript(base, index, _) => {
                // Try base first (arr[i])
                if let Some(base_ct) = self.infer_expr_ctype(base) {
                    match base_ct {
                        CType::Array(elem, _) => return Some(*elem),
                        CType::Pointer(pointee, _) => return Some(*pointee),
                        _ => {}
                    }
                }
                // Reverse subscript (i[arr])
                if let Some(idx_ct) = self.infer_expr_ctype(index) {
                    match idx_ct {
                        CType::Array(elem, _) => return Some(*elem),
                        CType::Pointer(pointee, _) => return Some(*pointee),
                        _ => {}
                    }
                }
                None
            }

            // Member access: look up field type in struct layout
            Expr::MemberAccess(base_expr, field_name, _) => {
                self.infer_field_ctype(base_expr, field_name, false)
            }
            Expr::PointerMemberAccess(base_expr, field_name, _) => {
                self.infer_field_ctype(base_expr, field_name, true)
            }

            // Unary operators
            Expr::UnaryOp(op, inner, _) => {
                match op {
                    UnaryOp::LogicalNot => Some(CType::Int),
                    UnaryOp::Neg | UnaryOp::Plus | UnaryOp::BitNot => {
                        if let Some(inner_ct) = self.infer_expr_ctype(inner) {
                            if inner_ct.is_integer() {
                                Some(inner_ct.integer_promoted())
                            } else {
                                Some(inner_ct)
                            }
                        } else {
                            None
                        }
                    }
                    UnaryOp::PreInc | UnaryOp::PreDec => self.infer_expr_ctype(inner),
                    UnaryOp::RealPart | UnaryOp::ImagPart => {
                        self.infer_expr_ctype(inner).map(|inner_ct| inner_ct.complex_component_type())
                    }
                }
            }

            // Postfix operators preserve the operand type
            Expr::PostfixOp(_, inner, _) => self.infer_expr_ctype(inner),

            // Binary operators
            Expr::BinaryOp(op, lhs, rhs, _) => {
                self.infer_binop_ctype(op, lhs, rhs)
            }

            // Assignment: type of the left-hand side
            Expr::Assign(lhs, _, _) | Expr::CompoundAssign(_, lhs, _, _) => {
                self.infer_expr_ctype(lhs)
            }

            // Conditional: C11 6.5.15 composite type rules
            Expr::Conditional(_, then_expr, else_expr, _) => {
                use crate::common::const_arith::is_null_pointer_constant;
                CType::conditional_composite_type(
                    self.infer_expr_ctype(then_expr),
                    self.infer_expr_ctype(else_expr),
                    is_null_pointer_constant(then_expr),
                    is_null_pointer_constant(else_expr),
                )
            }
            Expr::GnuConditional(cond, else_expr, _) => {
                use crate::common::const_arith::is_null_pointer_constant;
                CType::conditional_composite_type(
                    self.infer_expr_ctype(cond),
                    self.infer_expr_ctype(else_expr),
                    is_null_pointer_constant(cond),
                    is_null_pointer_constant(else_expr),
                )
            }

            // Comma: type of the right expression
            Expr::Comma(_, rhs, _) => self.infer_expr_ctype(rhs),

            // Function call: determine return type
            Expr::FunctionCall(func, args, _) => {
                // __builtin_choose_expr(const_expr, expr1, expr2) has the type
                // of the selected branch, not a fixed return type.
                if let Expr::Identifier(name, _) = func.as_ref() {
                    if name == "__builtin_choose_expr" && args.len() >= 3 {
                        let cond = self.eval_const_expr(&args[0]).unwrap_or(1);
                        return if cond != 0 {
                            self.infer_expr_ctype(&args[1])
                        } else {
                            self.infer_expr_ctype(&args[2])
                        };
                    }
                }
                self.infer_call_return_ctype(func)
            }

            // VaArg and CompoundLiteral: type from type specifier
            Expr::VaArg(_, type_spec, _) | Expr::CompoundLiteral(type_spec, _, _) => {
                Some(self.resolve_type_spec(type_spec))
            }

            // Statement expression: type of the last expression statement
            Expr::StmtExpr(compound, _) => {
                if let Some(BlockItem::Statement(Stmt::Expr(Some(expr)))) = compound.items.last() {
                    if let Some(ctype) = self.infer_expr_ctype(expr) {
                        return Some(ctype);
                    }
                    // If the last expr is an identifier not in the symbol table
                    // (e.g., inside typeof where the stmt expr was never executed),
                    // resolve it from declarations within this compound statement.
                    // We build a local scope as we iterate so that typeof expressions
                    // referencing earlier declarations can be resolved (e.g., the
                    // kernel's xchg macro: typeof(&field) ptr = ...; __typeof__(*ptr) ret = ...; ret;)
                    if let Expr::Identifier(name, _) = expr {
                        return self.resolve_var_from_compound(compound, name);
                    }
                }
                None
            }

            // _Generic: resolve based on controlling expression type
            Expr::GenericSelection(controlling, associations, _) => {
                self.infer_generic_selection_ctype(controlling, associations)
            }

            // Label address: void*
            Expr::LabelAddr(_, _) => Some(CType::Pointer(Box::new(CType::Void), AddressSpace::Default)),

            // __builtin_types_compatible_p: int
            Expr::BuiltinTypesCompatibleP(_, _, _) => Some(CType::Int),
        }
    }

    /// Infer the CType of a binary operation.
    ///
    /// Evaluates each operand's type at most once to avoid exponential blowup
    /// on deeply nested expression chains like `+1+1+1+...+1` (which appear in
    /// preprocessor-generated enum initializers).
    fn infer_binop_ctype(&self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Option<CType> {
        // Comparison and logical operators always produce int
        match op {
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge
            | BinOp::LogicalAnd | BinOp::LogicalOr => {
                return Some(CType::Int);
            }
            _ => {}
        }

        // Evaluate each operand type once and reuse the results.
        let lct = self.infer_expr_ctype(lhs);
        let rct = self.infer_expr_ctype(rhs);

        // Shift operators: result type is the promoted type of the left operand
        if matches!(op, BinOp::Shl | BinOp::Shr) {
            if let Some(l) = lct {
                return Some(l.integer_promoted());
            }
            return Some(CType::Int);
        }

        // Pointer arithmetic for Add and Sub
        if matches!(op, BinOp::Add | BinOp::Sub) {
            if let Some(ref l) = lct {
                match l {
                    CType::Pointer(_, _) => {
                        if *op == BinOp::Sub {
                            if let Some(ref r) = rct {
                                if r.is_pointer_like() {
                                    return Some(CType::Long);
                                }
                            }
                        }
                        return lct;
                    }
                    CType::Array(elem, _) => {
                        if *op == BinOp::Sub {
                            if let Some(ref r) = rct {
                                if r.is_pointer_like() {
                                    return Some(CType::Long);
                                }
                            }
                        }
                        return Some(CType::Pointer(elem.clone(), AddressSpace::Default));
                    }
                    _ => {}
                }
            }
            if *op == BinOp::Add {
                // int + ptr case
                if let Some(ref r) = rct {
                    match r {
                        CType::Pointer(_, _) => return rct,
                        CType::Array(elem, _) => return Some(CType::Pointer(elem.clone(), AddressSpace::Default)),
                        _ => {}
                    }
                }
            }
        }

        // GCC vector extensions: if either operand is a vector type, the result
        // is that vector type.  usual_arithmetic_conversion doesn't handle vectors.
        if let Some(ref l) = lct {
            if l.is_vector() { return lct; }
        }
        if let Some(ref r) = rct {
            if r.is_vector() { return rct; }
        }

        // Arithmetic/bitwise: usual arithmetic conversions
        match (lct, rct) {
            (Some(l), Some(r)) => Some(CType::usual_arithmetic_conversion(&l, &r)),
            (Some(l), None) => Some(l.integer_promoted()),
            (None, Some(r)) => Some(r.integer_promoted()),
            (None, None) => None,
        }
    }

    /// Infer the return CType of a function call.
    fn infer_call_return_ctype(&self, func: &Expr) -> Option<CType> {
        // Strip Deref layers (dereferencing function pointers is a no-op in C)
        let mut stripped = func;
        while let Expr::Deref(inner, _) = stripped {
            stripped = inner;
        }

        if let Expr::Identifier(name, _) = stripped {
            // Check function signatures first
            if let Some(func_info) = self.functions.get(name.as_str()) {
                return Some(func_info.return_type.clone());
            }
            // Check builtin return types
            if let Some(ct) = Self::builtin_return_ctype(name) {
                return Some(ct);
            }
            // Check symbol table for function pointer variables
            if let Some(sym) = self.symbols.lookup(name) {
                return Self::extract_return_ctype_from_type(&sym.ty);
            }
        }

        // For complex expressions (indirect calls through computed function pointers),
        // try to infer the function pointer's type
        if let Some(func_ct) = self.infer_expr_ctype(stripped) {
            return Self::extract_return_ctype_from_type(&func_ct);
        }

        None
    }

    /// Extract the return CType from a function or function pointer type.
    fn extract_return_ctype_from_type(ct: &CType) -> Option<CType> {
        match ct {
            CType::Function(ft) => Some(ft.return_type.clone()),
            CType::Pointer(inner, _) => match inner.as_ref() {
                CType::Function(ft) => Some(ft.return_type.clone()),
                // Pointer-to-function-pointer
                CType::Pointer(inner2, _) => match inner2.as_ref() {
                    CType::Function(ft) => Some(ft.return_type.clone()),
                    _ => Some(inner.as_ref().clone()),
                },
                other => Some(other.clone()),
            },
            _ => None,
        }
    }

    /// Infer the CType of a struct/union field access.
    fn infer_field_ctype(&self, base_expr: &Expr, field_name: &str, is_pointer: bool) -> Option<CType> {
        let base_ctype = if is_pointer {
            match self.infer_expr_ctype(base_expr)? {
                CType::Pointer(inner, _) => *inner,
                CType::Array(inner, _) => *inner,
                _ => return None,
            }
        } else {
            self.infer_expr_ctype(base_expr)?
        };

        match &base_ctype {
            CType::Struct(key) | CType::Union(key) => {
                let layouts = self.types.borrow_struct_layouts();
                if let Some(layout) = layouts.get(key.as_ref()) {
                    if let Some((_offset, field_ct)) = layout.field_offset(field_name, &*layouts) {
                        return Some(field_ct);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Resolve a _Generic selection based on the controlling expression's type.
    fn infer_generic_selection_ctype(&self, controlling: &Expr, associations: &[GenericAssociation]) -> Option<CType> {
        let controlling_ct = self.infer_expr_ctype(controlling);
        let mut default_expr: Option<&Expr> = None;

        for assoc in associations {
            match &assoc.type_spec {
                None => { default_expr = Some(&assoc.expr); }
                Some(type_spec) => {
                    let assoc_ct = self.resolve_type_spec(type_spec);
                    if let Some(ref ctrl_ct) = controlling_ct {
                        if self.ctype_matches_generic(ctrl_ct, &assoc_ct) {
                            return self.infer_expr_ctype(&assoc.expr);
                        }
                    }
                }
            }
        }

        if let Some(def) = default_expr {
            return self.infer_expr_ctype(def);
        }
        None
    }

    /// Check if two CTypes match for _Generic selection purposes.
    /// Uses compatible-type rules: ignores qualifiers, matches arrays with pointers.
    fn ctype_matches_generic(&self, controlling: &CType, assoc: &CType) -> bool {
        // Exact match
        if std::mem::discriminant(controlling) == std::mem::discriminant(assoc) {
            match (controlling, assoc) {
                (CType::Pointer(a, _), CType::Pointer(b, _)) => {
                    return self.ctype_matches_generic(a, b);
                }
                (CType::Array(a, _), CType::Array(b, _)) => {
                    return self.ctype_matches_generic(a, b);
                }
                // For types that carry identity data (Struct, Union, Enum, Function,
                // Vector), we must compare the full type, not just the discriminant.
                // Primitive types (Int, Float, etc.) have no data so PartialEq works too.
                _ => return controlling == assoc,
            }
        }
        // Array decays to pointer for _Generic matching
        if let CType::Array(elem, _) = controlling {
            if let CType::Pointer(pointee, _) = assoc {
                return self.ctype_matches_generic(elem, pointee);
            }
        }
        // Enum matches int
        if matches!(controlling, CType::Enum(_)) && matches!(assoc, CType::Int) {
            return true;
        }
        if matches!(controlling, CType::Int) && matches!(assoc, CType::Enum(_)) {
            return true;
        }
        false
    }

    /// Resolve a TypeSpecifier to a CType using sema's type context.
    /// This delegates to the type_builder's shared conversion via TypeConvertContext.
    fn resolve_type_spec(&self, spec: &TypeSpecifier) -> CType {
        // Use a simple inline resolution for common cases to avoid the trait dispatch
        match spec {
            TypeSpecifier::Void => CType::Void,
            TypeSpecifier::Bool => CType::Bool,
            TypeSpecifier::Char => CType::Char,
            TypeSpecifier::UnsignedChar => CType::UChar,
            TypeSpecifier::Short => CType::Short,
            TypeSpecifier::UnsignedShort => CType::UShort,
            TypeSpecifier::Int | TypeSpecifier::Signed => CType::Int,
            TypeSpecifier::UnsignedInt | TypeSpecifier::Unsigned => CType::UInt,
            TypeSpecifier::Long => CType::Long,
            TypeSpecifier::UnsignedLong => CType::ULong,
            TypeSpecifier::LongLong => CType::LongLong,
            TypeSpecifier::UnsignedLongLong => CType::ULongLong,
            TypeSpecifier::Int128 => CType::Int128,
            TypeSpecifier::UnsignedInt128 => CType::UInt128,
            TypeSpecifier::Float => CType::Float,
            TypeSpecifier::Double => CType::Double,
            TypeSpecifier::LongDouble => CType::LongDouble,
            TypeSpecifier::ComplexFloat => CType::ComplexFloat,
            TypeSpecifier::ComplexDouble => CType::ComplexDouble,
            TypeSpecifier::ComplexLongDouble => CType::ComplexLongDouble,
            TypeSpecifier::Pointer(inner, addr_space) => {
                CType::Pointer(Box::new(self.resolve_type_spec(inner)), *addr_space)
            }
            TypeSpecifier::Array(elem, size) => {
                let elem_ct = self.resolve_type_spec(elem);
                let arr_size = size.as_ref().and_then(|s| self.eval_const_expr(s));
                CType::Array(Box::new(elem_ct), arr_size.map(|s| s as usize))
            }
            TypeSpecifier::TypedefName(name) => {
                if let Some(resolved) = self.types.typedefs.get(name) {
                    resolved.clone()
                } else {
                    CType::Int // fallback for unknown typedef
                }
            }
            TypeSpecifier::Enum(name, variants, _is_packed) => {
                // GCC extension: determine underlying enum type based on value range.
                // Uses progression: int -> unsigned int -> long long -> unsigned long long.
                let values = if let Some(vars) = variants {
                    let mut vals = Vec::new();
                    let mut next_val: i64 = 0;
                    for v in vars {
                        if let Some(ref val_expr) = v.value {
                            if let Some(val) = self.eval_const_expr(val_expr) {
                                next_val = val;
                            }
                        }
                        vals.push(next_val);
                        next_val += 1;
                    }
                    vals
                } else if let Some(n) = name {
                    if let Some(et) = self.types.packed_enum_types.get(n) {
                        et.variants.iter().map(|(_, v)| *v).collect()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };
                // Determine smallest type that fits all values
                let fits_int = values.iter().all(|v| *v >= i32::MIN as i64 && *v <= i32::MAX as i64);
                if fits_int {
                    return CType::Int;
                }
                let fits_uint = values.iter().all(|v| *v >= 0 && *v <= u32::MAX as i64);
                if fits_uint {
                    return CType::UInt;
                }
                let has_negative = values.iter().any(|v| *v < 0);
                if has_negative { CType::LongLong } else { CType::ULongLong }
            }
            TypeSpecifier::Struct(tag, fields, is_packed, pragma_pack, struct_aligned) => {
                if let Some(tag) = tag {
                    CType::Struct(format!("struct.{}", tag).into())
                } else if let Some(fs) = fields {
                    // Without a tag name but with fields, register the anonymous
                    // struct layout so member access resolution works correctly
                    // (e.g., kernel get_unaligned() macro pattern with packed structs
                    // inside statement expressions).
                    self.resolve_anon_struct_or_union(fs, false, *is_packed, *pragma_pack, *struct_aligned)
                } else {
                    // Anonymous forward declaration (no tag, no fields)
                    CType::Int
                }
            }
            TypeSpecifier::Union(tag, fields, is_packed, pragma_pack, struct_aligned) => {
                if let Some(tag) = tag {
                    CType::Union(format!("union.{}", tag).into())
                } else if let Some(fs) = fields {
                    // Same as struct: register anonymous union layout for member access
                    self.resolve_anon_struct_or_union(fs, true, *is_packed, *pragma_pack, *struct_aligned)
                } else {
                    // Anonymous forward declaration (no tag, no fields)
                    CType::Int
                }
            }
            TypeSpecifier::TypeofType(inner) => self.resolve_type_spec(inner),
            TypeSpecifier::Typeof(expr) => {
                // typeof(expr): try to infer the expression's type
                self.infer_expr_ctype(expr).unwrap_or(CType::Int)
            }
            TypeSpecifier::FunctionPointer(ret, params, variadic) => {
                let ret_ct = self.resolve_type_spec(ret);
                let param_cts: Vec<(CType, Option<String>)> = params.iter().map(|p| {
                    (self.resolve_type_spec(&p.type_spec), p.name.clone())
                }).collect();
                CType::Pointer(Box::new(CType::Function(Box::new(FunctionType {
                    return_type: ret_ct,
                    params: param_cts,
                    variadic: *variadic,
                }))), AddressSpace::Default)
            }
            // TODO: handle remaining TypeSpecifier variants
            _ => CType::Int,
        }
    }

    /// Evaluate a constant expression for array sizes and similar contexts.
    /// Delegates to SemaConstEval for the full implementation, unifying
    /// const_eval across all sema modules.
    fn eval_const_expr(&self, expr: &Expr) -> Option<i64> {
        let evaluator = super::const_eval::SemaConstEval {
            types: self.types,
            symbols: self.symbols,
            functions: self.functions,
            const_values: None,
            expr_types: self.expr_types,
        };
        evaluator.eval_const_expr(expr)?.to_i64()
    }

    /// Resolve an anonymous struct or union with inline field declarations.
    /// Generates a unique key, computes the layout, registers it in TypeContext,
    /// and returns the CType (Struct or Union).
    fn resolve_anon_struct_or_union(
        &self,
        fields: &[StructFieldDecl],
        is_union: bool,
        is_packed: bool,
        pragma_pack: Option<usize>,
        struct_aligned: Option<usize>,
    ) -> CType {
        use crate::common::types::{StructField, StructLayout};

        let struct_fields: Vec<StructField> = fields.iter().map(|f| {
            let mut ty = self.resolve_field_ctype(f);
            // GCC treats enum bitfields as unsigned
            if f.bit_width.is_some()
                && matches!(&f.type_spec, TypeSpecifier::Enum(..))
                    && ty == CType::Int {
                        ty = CType::UInt;
                    }
            StructField {
                name: f.name.clone().unwrap_or_default(),
                ty,
                bit_width: f.bit_width.as_ref().and_then(|bw| {
                    self.eval_const_expr(bw).map(|v| v as u32)
                }),
                alignment: f.alignment,
                is_packed: f.is_packed,
            }
        }).collect();

        let max_field_align = if is_packed { Some(1) } else { pragma_pack };
        let mut layout = if is_union {
            StructLayout::for_union_with_packing(&struct_fields, max_field_align, &*self.types.borrow_struct_layouts())
        } else {
            StructLayout::for_struct_with_packing(&struct_fields, max_field_align, &*self.types.borrow_struct_layouts())
        };

        // Apply struct-level __attribute__((aligned(N)))
        if let Some(a) = struct_aligned {
            if a > layout.align {
                layout.align = a;
                let mask = layout.align - 1;
                layout.size = (layout.size + mask) & !mask;
            }
        }

        let id = self.types.next_anon_struct_id();
        let key = format!("__anon_struct_{}", id);
        self.types.insert_struct_layout_scoped_from_ref(&key, layout);
        if is_union {
            CType::Union(key.into())
        } else {
            CType::Struct(key.into())
        }
    }

    /// Get the CType for a struct field declaration, accounting for derived declarators.
    fn resolve_field_ctype(&self, f: &StructFieldDecl) -> CType {
        let mut ctype = self.resolve_type_spec(&f.type_spec);
        for derived in &f.derived {
            match derived {
                DerivedDeclarator::Pointer => {
                    ctype = CType::Pointer(Box::new(ctype), AddressSpace::Default);
                }
                DerivedDeclarator::Array(Some(size_expr)) => {
                    let size = self.eval_const_expr(size_expr).unwrap_or(0) as usize;
                    ctype = CType::Array(Box::new(ctype), Some(size));
                }
                DerivedDeclarator::Array(None) => {
                    ctype = CType::Array(Box::new(ctype), None);
                }
                _ => {} // Function/FunctionPointer not expected in struct fields
            }
        }
        ctype
    }

    /// Return CType for known builtins.
    /// This is the CType-level equivalent of the lowerer's builtin_return_type (IrType).
    /// TODO: consolidate with sema/builtins.rs BuiltinInfo to avoid duplication.
    fn builtin_return_ctype(name: &str) -> Option<CType> {
        match name {
            // Float-returning builtins
            "__builtin_inf" | "__builtin_huge_val"
            | "__builtin_nan" | "__builtin_fabs" | "__builtin_sqrt"
            | "__builtin_sin" | "__builtin_cos" | "__builtin_log"
            | "__builtin_log2" | "__builtin_exp" | "__builtin_pow"
            | "__builtin_floor" | "__builtin_ceil" | "__builtin_round"
            | "__builtin_fmin" | "__builtin_fmax" | "__builtin_copysign"
            | "__builtin_nextafter" => Some(CType::Double),

            "__builtin_inff" | "__builtin_huge_valf"
            | "__builtin_nanf" | "__builtin_fabsf" | "__builtin_sqrtf"
            | "__builtin_sinf" | "__builtin_cosf" | "__builtin_logf"
            | "__builtin_expf" | "__builtin_powf" | "__builtin_floorf"
            | "__builtin_ceilf" | "__builtin_roundf"
            | "__builtin_copysignf"
            | "__builtin_nextafterf" => Some(CType::Float),

            "__builtin_infl" | "__builtin_huge_vall"
            | "__builtin_nanl" | "__builtin_fabsl"
            | "__builtin_nextafterl" => Some(CType::LongDouble),

            // Integer-returning builtins
            "__builtin_fpclassify" | "__builtin_isnan" | "__builtin_isinf"
            | "__builtin_isfinite" | "__builtin_isnormal" | "__builtin_signbit"
            | "__builtin_signbitf" | "__builtin_signbitl" | "__builtin_isinf_sign"
            | "__builtin_isgreater" | "__builtin_isgreaterequal"
            | "__builtin_isless" | "__builtin_islessequal"
            | "__builtin_islessgreater" | "__builtin_isunordered"
            | "__builtin_clz" | "__builtin_clzl" | "__builtin_clzll"
            | "__builtin_ctz" | "__builtin_ctzl" | "__builtin_ctzll"
            | "__builtin_clrsb" | "__builtin_clrsbl" | "__builtin_clrsbll"
            | "__builtin_popcount" | "__builtin_popcountl" | "__builtin_popcountll"
            | "__builtin_parity" | "__builtin_parityl" | "__builtin_parityll"
            | "__builtin_ffs" | "__builtin_ffsl" | "__builtin_ffsll"
            | "__builtin_types_compatible_p" | "__builtin_classify_type"
            | "__builtin_constant_p" | "__builtin_object_size" => Some(CType::Int),

            // Byte-swap builtins return unsigned types (GCC behavior)
            "__builtin_bswap16" | "__builtin_bswap32" => Some(CType::UInt),
            "__builtin_bswap64" => Some(CType::ULongLong),

            // __builtin_expect returns long (evaluates both args, returns first)
            "__builtin_expect" | "__builtin_expect_with_probability" => Some(CType::Long),

            // Complex component extraction
            "creal" | "__builtin_creal" | "cimag" | "__builtin_cimag" => Some(CType::Double),
            "crealf" | "__builtin_crealf" | "cimagf" | "__builtin_cimagf" => Some(CType::Float),
            "creall" | "__builtin_creall" | "cimagl" | "__builtin_cimagl" => Some(CType::LongDouble),
            "cabs" | "__builtin_cabs" => Some(CType::Double),
            "cabsf" | "__builtin_cabsf" => Some(CType::Float),
            "cabsl" | "__builtin_cabsl" => Some(CType::LongDouble),
            "carg" | "__builtin_carg" => Some(CType::Double),
            "cargf" | "__builtin_cargf" => Some(CType::Float),

            // Memory/string builtins returning pointers
            "__builtin_memcpy" | "__builtin_memmove" | "__builtin_memset"
            | "__builtin_alloca" | "__builtin_alloca_with_align"
            | "__builtin_frame_address" | "__builtin_return_address"
            | "__builtin_thread_pointer" => {
                Some(CType::Pointer(Box::new(CType::Void), AddressSpace::Default))
            }

            // Void-returning builtins
            "__builtin_va_start" | "__builtin_va_end" | "__builtin_va_copy"
            | "__builtin_abort" | "__builtin_exit" | "__builtin_trap"
            | "__builtin_unreachable" | "__builtin_prefetch"
            | "__builtin___clear_cache"
            | "__builtin_cpu_init" => Some(CType::Void),

            // CPU feature detection
            "__builtin_cpu_supports" => Some(CType::Int),

            _ => None,
        }
    }

    /// Resolve the type of a variable declared within a compound statement.
    ///
    /// Scans declarations in order, building a local scope so that typeof
    /// expressions referencing earlier declarations can be resolved. This handles
    /// patterns like: `({ typeof(&s->field) p = ...; __typeof__(*p) ret = ...; ret; })`
    /// where `__typeof__(*p)` must resolve `p` from the same compound statement.
    fn resolve_var_from_compound(&self, compound: &CompoundStmt, target_name: &str) -> Option<CType> {
        let mut local_scope: FxHashMap<String, CType> = FxHashMap::default();

        for item in &compound.items {
            if let BlockItem::Declaration(decl) = item {
                for declarator in &decl.declarators {
                    if declarator.name.is_empty() {
                        continue;
                    }
                    // Resolve this declaration's type, using the local scope
                    // for typeof expressions referencing earlier declarations.
                    let mut ctype = self.resolve_type_spec_with_scope(&decl.type_spec, &local_scope);
                    for derived in &declarator.derived {
                        match derived {
                            DerivedDeclarator::Pointer => {
                                ctype = CType::Pointer(Box::new(ctype), AddressSpace::Default);
                            }
                            DerivedDeclarator::Array(Some(size_expr)) => {
                                let size = self.eval_const_expr(size_expr).unwrap_or(0) as usize;
                                ctype = CType::Array(Box::new(ctype), Some(size));
                            }
                            DerivedDeclarator::Array(None) => {
                                ctype = CType::Array(Box::new(ctype), None);
                            }
                            _ => {} // Function/FunctionPointer not expected here
                        }
                    }
                    if declarator.name == target_name {
                        return Some(ctype);
                    }
                    local_scope.insert(declarator.name.clone(), ctype);
                }
            }
        }
        None
    }

    /// Resolve a TypeSpecifier to a CType, using a supplementary local scope
    /// for typeof expressions that reference variables not in the symbol table.
    fn resolve_type_spec_with_scope(&self, ts: &TypeSpecifier, scope: &FxHashMap<String, CType>) -> CType {
        match ts {
            TypeSpecifier::Typeof(expr) => {
                // Try normal resolution first
                if let Some(ctype) = self.infer_expr_ctype(expr) {
                    return ctype;
                }
                // Try resolving using the local scope
                if let Some(ctype) = self.infer_expr_ctype_with_scope(expr, scope) {
                    return ctype;
                }
                // TODO: report diagnostic for unresolved typeof expression
                CType::Int // fallback
            }
            TypeSpecifier::TypeofType(inner) => {
                self.resolve_type_spec_with_scope(inner, scope)
            }
            _ => self.resolve_type_spec(ts),
        }
    }

    /// Infer the CType of an expression using a supplementary local scope
    /// for identifiers not found in the symbol table.
    /// Handles common typeof patterns (identifier, deref, address-of).
    /// More complex expressions (member access, subscript, etc.) are not supported.
    fn infer_expr_ctype_with_scope(&self, expr: &Expr, scope: &FxHashMap<String, CType>) -> Option<CType> {
        match expr {
            Expr::Identifier(name, _) => {
                scope.get(name.as_str()).cloned()
            }
            Expr::Deref(inner, _) => {
                let inner_ct = self.infer_expr_ctype(inner)
                    .or_else(|| self.infer_expr_ctype_with_scope(inner, scope))?;
                match inner_ct {
                    CType::Pointer(pointee, _) => Some(*pointee),
                    CType::Array(elem, _) => Some(*elem),
                    _ => None,
                }
            }
            Expr::AddressOf(inner, _) => {
                let inner_ct = self.infer_expr_ctype(inner)
                    .or_else(|| self.infer_expr_ctype_with_scope(inner, scope))?;
                Some(CType::Pointer(Box::new(inner_ct), AddressSpace::Default))
            }
            _ => None,
        }
    }
}

// ===========================================================================
// PBT round 05 — property tests. P1: usual arithmetic conversions (C11
// 6.3.1.1p2 + 6.3.1.8) checked against an INDEPENDENT model built from the
// standard clauses, read through the real SemanticAnalyzer pipeline's
// expr_types annotations. P2: enum_constant_type's documented GCC promotion
// bands, boundary-exact. Oracle basis: C11 clauses + type_checker.rs:36-50
// doc comment; gcc 9.4 KAT gate pins the model (one gcc compile).
// ===========================================================================
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::types::CType;
    use crate::frontend::lexer::Lexer;
    use crate::frontend::parser::Parser;
    use crate::frontend::parser::ast::{Expr, ExternalDecl, Initializer};
    use crate::frontend::sema::analysis::{SemaResult, SemanticAnalyzer};
    use proptest::prelude::*;

    /// Parse + sema a whole program; return the SemaResult.
    pub(super) fn sema_of(src: &str) -> (crate::frontend::parser::ast::TranslationUnit, SemaResult) {
        let toks = Lexer::new(src, 0).tokenize();
        let mut p = Parser::new(toks);
        let tu = p.parse();
        assert_eq!(p.error_count, 0, "property program must parse cleanly:\n{src}\n");
        let mut sema = SemanticAnalyzer::new();
        let _ = sema.analyze(&tu);
        (tu, sema.into_result())
    }

    /// The first global declaration that carries an expression initializer.
    pub(super) fn init_expr_of(tu: &crate::frontend::parser::ast::TranslationUnit) -> &Expr {
        for d in &tu.decls {
            if let ExternalDecl::Declaration(decl) = d {
                if let Some(Initializer::Expr(e)) = &decl.declarators[0].init {
                    return e;
                }
            }
        }
        panic!("no expr initializer found in TU");
    }

    // ------------------------------------------------------------------
    // Independent C11 model (LP64): scalar descriptor (rank, size, signed).
    // ------------------------------------------------------------------
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub(super) struct Scalar {
        rank: u8,       // 0 float, 1 double, 2 long double, 10 int, 11 long, 12 long long
        size: u8,
        signed: bool,
        float: bool,
    }
    pub(super) fn model_scalar(sp: &str) -> Scalar {
        match sp {
            "float" => Scalar { rank: 0, size: 4, signed: true, float: true },
            "double" => Scalar { rank: 1, size: 8, signed: true, float: true },
            "long double" => Scalar { rank: 2, size: 16, signed: true, float: true },
            // sub-int types integer-promote to int (C11 6.3.1.1p2): int can
            // represent every value of char/signed char/unsigned char/short/
            // unsigned short on every target.
            "char" | "signed char" | "unsigned char" | "short" | "unsigned short" =>
                Scalar { rank: 10, size: 4, signed: true, float: false },
            "int" => Scalar { rank: 10, size: 4, signed: true, float: false },
            "unsigned int" => Scalar { rank: 10, size: 4, signed: false, float: false },
            "long" => Scalar { rank: 11, size: 8, signed: true, float: false },
            "unsigned long" => Scalar { rank: 11, size: 8, signed: false, float: false },
            "long long" => Scalar { rank: 12, size: 8, signed: true, float: false },
            "unsigned long long" => Scalar { rank: 12, size: 8, signed: false, float: false },
            _ => panic!("unknown spelling {sp}"),
        }
    }
    pub(super) fn model_result_ctype(s: Scalar) -> CType {
        match (s.rank, s.signed, s.float) {
            (0, _, true) => CType::Float,
            (1, _, true) => CType::Double,
            (2, _, true) => CType::LongDouble,
            (10, true, false) => CType::Int,
            (10, false, false) => CType::UInt,
            (11, true, false) => CType::Long,
            (11, false, false) => CType::ULong,
            (12, true, false) => CType::LongLong,
            (12, false, false) => CType::ULongLong,
            _ => unreachable!(),
        }
    }
    /// C11 6.3.1.8 usual arithmetic conversions on already-promoted scalars.
    pub(super) fn uac(a: Scalar, b: Scalar) -> Scalar {
        if a.float || b.float {
            if a.rank == 2 || b.rank == 2 { return model_scalar("long double"); }
            if a.rank == 1 || b.rank == 1 { return model_scalar("double"); }
            return model_scalar("float");
        }
        if a == b { return a; }
        if a.signed == b.signed {
            // Same signedness: the type with greater rank wins (6.3.1.8 rule 1)
            return if a.rank > b.rank { a } else { b };
        }
        let (u, s) = if !a.signed { (a, b) } else { (b, a) }; // u unsigned, s signed
        if u.rank >= s.rank { return u; }                     // 6.3.1.8 rule 3
        if s.size > u.size { return s; }                      // rule 4: signed can represent all
        // rule 5: unsigned counterpart of the signed type's rank
        Scalar { rank: s.rank, size: s.size, signed: false, float: false }
    }

    const TYPES: [&str; 14] = [
        "char", "signed char", "unsigned char", "short", "unsigned short", "int",
        "unsigned int", "long", "unsigned long", "long long", "unsigned long long",
        "float", "double", "long double",
    ];
    const ARITH_OPS: [&str; 10] = ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"];
    const CMP_OPS: [&str; 8] = ["==", "!=", "<", "<=", ">", ">=", "&&", "||"];

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p1_uac_binop_ctype(
            t1 in proptest::sample::select(TYPES.to_vec()),
            t2 in proptest::sample::select(TYPES.to_vec()),
            op in prop_oneof![
                7 => proptest::sample::select(ARITH_OPS.to_vec()),
                3 => proptest::sample::select(CMP_OPS.to_vec()),
            ],
        ) {
            let src = format!("{t1} a; {t2} b; int p = a {op} b;\n");
            let (tu, result) = sema_of(&src);
            let e = init_expr_of(&tu);
            let got = result.expr_types.get(&e.id())
                .unwrap_or_else(|| panic!("expr_types missing for {src}"));
            let expected = if matches!(op, "==" | "!=" | "<" | "<=" | ">" | ">=" | "&&" | "||") {
                CType::Int
            } else if matches!(op, "<<" | ">>") {
                // C11 6.5.7p3: promoted type of the LEFT operand only
                model_result_ctype(model_scalar(t1))
            } else {
                model_result_ctype(uac(model_scalar(t1), model_scalar(t2)))
            };
            prop_assert_eq!(got, &expected, "src: {}", src);
        }

        #[test]
        fn p2_enum_constant_type_bands(
            v in prop_oneof![
                4 => proptest::sample::select(vec![
                    i64::MIN, i64::MIN + 1, i32::MIN as i64 - 1, i32::MIN as i64,
                    -1i64, 0i64, 1i64, i32::MAX as i64 - 1, i32::MAX as i64,
                    i32::MAX as i64 + 1, u32::MAX as i64 - 1, u32::MAX as i64,
                    u32::MAX as i64 + 1, i64::MAX - 1, i64::MAX,
                ]),
                6 => any::<i64>(),
            ],
        ) {
            // Doc contract (type_checker.rs:37-38): GCC progression
            // int -> unsigned int -> long long -> unsigned long long.
            let expected = if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
                CType::Int
            } else if v >= 0 && v <= u32::MAX as i64 {
                CType::UInt
            } else if v >= 0 {
                CType::ULongLong
            } else {
                CType::LongLong
            };
            prop_assert_eq!(enum_constant_type(v), expected, "v = {}", v);
        }
    }

    /// gcc KAT gate for the P1 model: one gcc compile asserting sizeof(a op b)
    /// matches the model's result-type size for the full 14x14 x {+} grid plus
    /// representative shift/comparison ops. If gcc is unavailable the gate is
    /// skipped (model already derived from C11 clauses).
    #[test]
    fn p1_kat_gcc_model_gate() {
        let mut c = String::from("typedef unsigned long usize_t;\n");
        let mut n = 0usize;
        for t1 in TYPES {
            for t2 in TYPES {
                let m = uac(model_scalar(t1), model_scalar(t2));
                let sz = m.size as usize;
                c.push_str(&format!(
                    "_Static_assert(sizeof(({t1})0 + ({t2})0) == {sz}, \"uac {t1}+{t2}\");\n"
                ));
                n += 1;
            }
        }
        // Shift: result type is promoted LEFT operand
        c.push_str("_Static_assert(sizeof((char)1 << (long)1) == sizeof(int), \"shift left\");\n");
        c.push_str("_Static_assert(sizeof((long)1 << (char)1) == sizeof(long), \"shift right\");\n");
        c.push_str("_Static_assert(sizeof((unsigned char)1 << (char)1) == sizeof(int), \"shift uchar\");\n");
        // Comparison/logical -> int
        c.push_str("_Static_assert(sizeof((long double)1 < (double)1) == sizeof(int), \"cmp\");\n");
        c.push_str("_Static_assert(sizeof((char)1 && (char)1) == sizeof(int), \"logical\");\n");
        let dir = std::env::temp_dir().join(format!("ccc_p1_kat_{}.c", std::process::id()));
        std::fs::write(&dir, &c).unwrap();
        let out = std::process::Command::new("gcc")
            .arg("-std=c11").arg("-w").arg("-c").arg(&dir)
            .arg("-o").arg(dir.with_extension("o"))
            .output();
        match out {
            Ok(o) if o.status.success() => { /* model pinned by gcc */ let _ = n; }
            Ok(o) => panic!(
                "P1 model disagrees with gcc 9.4:\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
            Err(_) => eprintln!("[p1_kat] gcc unavailable — model stands on C11 clauses alone"),
        }
        let _ = std::fs::remove_file(&dir);
    }
}

// ===========================================================================
// PBT round 05 — deterministic regression witnesses (intentionally red).
// Each reproduces a confirmed bug with its shrunk counterexample; they stay
// failing until the SUT is fixed. See pbt-out/rounds/05_sema/bug_reports/.
// ===========================================================================
#[cfg(test)]
mod pbt_regression {
    use super::pbt_tests::{sema_of, init_expr_of, model_scalar, model_result_ctype, uac};
    use crate::common::types::CType;

    /// B1: C11 6.3.1.8 rule 5 — when the higher-ranked signed type cannot
    /// represent all values of the unsigned operand (same size), the result
    /// is the UNSIGNED counterpart. unsigned long + long long must be
    /// unsigned long long; SUT yields LongLong.
    #[test]
    fn test_usual_arith_conversion_regression_ul_plus_ll() {
        let src = "unsigned long a; long long b; int p = a + b;\n";
        let (tu, result) = sema_of(src);
        let e = init_expr_of(&tu);
        let got = result.expr_types.get(&e.id()).unwrap();
        let want = model_result_ctype(uac(model_scalar("unsigned long"), model_scalar("long long")));
        assert_eq!(*got, want, "unsigned long + long long must be unsigned long long");
    }
}
