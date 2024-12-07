use mago_ast::ast::*;
use mago_interner::StringIdentifier;
use mago_span::HasSpan;
use mago_span::Span;
use mago_walker::MutWalker;

use crate::context::Context;
use crate::symbol::Symbol;
use crate::symbol::SymbolIdentifier;
use crate::symbol::SymbolKind;
use crate::table::SymbolTable;

#[derive(Debug)]
pub struct SymbolWalker {
    pub symbols: SymbolTable,
}

impl SymbolWalker {
    pub fn new() -> Self {
        Self { symbols: SymbolTable::new() }
    }

    fn identify(
        &self,
        context: &mut Context<'_>,
        kind: SymbolKind,
        name: StringIdentifier,
        span: Span,
    ) -> SymbolIdentifier {
        let fully_qualified_name = match &kind {
            SymbolKind::Property | SymbolKind::Method | SymbolKind::EnumCase | SymbolKind::ClassLikeConstant => {
                let scope = context
                    .get_scope()
                    .expect("scope should be present for property, method, enum case and class like constant");

                match &scope.identifier {
                    Some(identifier) => {
                        let member_name = context.interner.lookup(&name);

                        let fqcn = context.interner.lookup(&identifier.fully_qualified_name);

                        // the full name of the property is the fqcn followed by `::` and the name of the member
                        let fqcn = format!("{}::{}", fqcn, member_name);

                        context.interner.intern(fqcn)
                    }
                    None => {
                        // this is an anonymous class, so the full name of the property is simply the name of the member
                        name
                    }
                }
            }
            _ => match context.get_namespace() {
                Some(namespace) => {
                    let symbol_name = context.interner.lookup(&name);

                    let fqcn = format!("{}\\{}", namespace, symbol_name);

                    context.interner.intern(fqcn)
                }
                None => name,
            },
        };

        SymbolIdentifier { name, fully_qualified_name, span }
    }

    fn construct(
        &self,
        context: &mut Context<'_>,
        kind: SymbolKind,
        identifier: Option<SymbolIdentifier>,
        definition: Span,
    ) -> Symbol {
        Symbol {
            kind,
            namespace: match context.get_namespace() {
                Some(namespace) => Some(context.interner.intern(namespace)),
                None => None,
            },
            identifier,
            scope: context.get_scope().map(|symbol| symbol.to_reference()),
            span: definition,
        }
    }

    fn construct_identified(
        &self,
        context: &mut Context<'_>,
        kind: SymbolKind,
        name: StringIdentifier,
        name_span: Span,
        span: Span,
    ) -> Symbol {
        let identifier = Some(self.identify(context, kind, name, name_span));

        self.construct(context, kind, identifier, span)
    }

    fn construct_unidentified(&self, context: &mut Context<'_>, kind: SymbolKind, span: Span) -> Symbol {
        self.construct(context, kind, None, span)
    }
}

impl MutWalker<Context<'_>> for SymbolWalker {
    fn walk_in_namespace(&mut self, namespace: &Namespace, context: &mut Context<'_>) {
        let name = match &namespace.name {
            Some(name) => context.interner.lookup(&name.value()).to_string(),
            None => "".to_string(),
        };

        context.enter_namespace(name);
    }

    fn walk_out_namespace(&mut self, _namespace: &Namespace, context: &mut Context<'_>) {
        context.exit_namespace();
    }

    fn walk_in_function(&mut self, function: &Function, context: &mut Context<'_>) {
        let symbol = self.construct_identified(
            context,
            SymbolKind::Function,
            function.name.value,
            function.name.span,
            function.span(),
        );

        context.enter_scope(symbol);
    }

    fn walk_out_function(&mut self, _function: &Function, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting function, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_anonymous_class(&mut self, anonymous_class: &AnonymousClass, context: &mut Context<'_>) {
        let symbol = self.construct_unidentified(context, SymbolKind::Class, anonymous_class.span());

        context.enter_scope(symbol);
    }

    fn walk_out_anonymous_class(&mut self, _anonymous_class: &AnonymousClass, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting anonymous class, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_class(&mut self, class: &Class, context: &mut Context<'_>) {
        let symbol =
            self.construct_identified(context, SymbolKind::Class, class.name.value, class.name.span, class.span());

        context.enter_scope(symbol);
    }

    fn walk_out_class(&mut self, _class: &Class, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting class, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_trait(&mut self, r#trait: &Trait, context: &mut Context<'_>) {
        let symbol = self.construct_identified(
            context,
            SymbolKind::Trait,
            r#trait.name.value,
            r#trait.name.span,
            r#trait.span(),
        );

        context.enter_scope(symbol);
    }

    fn walk_out_trait(&mut self, _trait: &Trait, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting trait, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_enum(&mut self, r#enum: &Enum, context: &mut Context<'_>) {
        let symbol =
            self.construct_identified(context, SymbolKind::Enum, r#enum.name.value, r#enum.name.span, r#enum.span());

        context.enter_scope(symbol);
    }

    fn walk_out_enum(&mut self, _enum: &Enum, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting enum, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_interface(&mut self, interface: &Interface, context: &mut Context<'_>) {
        let symbol = self.construct_identified(
            context,
            SymbolKind::Interface,
            interface.name.value,
            interface.name.span,
            interface.span(),
        );

        context.enter_scope(symbol);
    }

    fn walk_out_interface(&mut self, _interface: &Interface, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting interface, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_closure(&mut self, closure: &Closure, context: &mut Context<'_>) {
        let symbol = self.construct_unidentified(context, SymbolKind::Closure, closure.span());

        context.enter_scope(symbol);
    }

    fn walk_out_closure(&mut self, _closure: &Closure, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting closure, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_arrow_function(&mut self, arrow_function: &ArrowFunction, context: &mut Context<'_>) {
        let symbol = self.construct_unidentified(context, SymbolKind::ArrowFunction, arrow_function.span());

        context.enter_scope(symbol);
    }

    fn walk_out_arrow_function(&mut self, _arrow_function: &ArrowFunction, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting arrow function, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_in_method(&mut self, method: &Method, context: &mut Context<'_>) {
        let symbol =
            self.construct_identified(context, SymbolKind::Method, method.name.value, method.name.span, method.span());

        context.enter_scope(symbol);
    }

    fn walk_out_method(&mut self, _method: &Method, context: &mut Context<'_>) {
        let Some(symbol) = context.exit_scope() else {
            panic!("scope should be present when exiting method, this is a bug in mago, please report it.");
        };

        self.symbols.add_symbol(symbol);
    }

    fn walk_constant(&mut self, constant: &Constant, context: &mut Context<'_>) {
        for item in constant.items.iter() {
            self.symbols.add_symbol(self.construct_identified(
                context,
                SymbolKind::Constant,
                item.name.value,
                item.name.span,
                constant.span(),
            ));
        }
    }

    fn walk_class_like_constant(&mut self, class_like_constant: &ClassLikeConstant, context: &mut Context<'_>) {
        for item in class_like_constant.items.iter() {
            self.symbols.add_symbol(self.construct_identified(
                context,
                SymbolKind::ClassLikeConstant,
                item.name.value,
                item.name.span,
                class_like_constant.span(),
            ));
        }
    }

    fn walk_enum_case(&mut self, enum_case: &EnumCase, context: &mut Context<'_>) {
        let item_name = enum_case.item.name();

        self.symbols.add_symbol(self.construct_identified(
            context,
            SymbolKind::EnumCase,
            item_name.value,
            item_name.span,
            enum_case.span(),
        ));
    }

    fn walk_plain_property(&mut self, plain_property: &PlainProperty, context: &mut Context<'_>) {
        for item in plain_property.items.iter() {
            let variable = item.variable();

            self.symbols.add_symbol(self.construct_identified(
                context,
                SymbolKind::Property,
                variable.name,
                variable.span,
                plain_property.span(),
            ));
        }
    }

    fn walk_in_hooked_property(&mut self, hooked_property: &HookedProperty, context: &mut Context<'_>) {
        let variable = hooked_property.item.variable();

        self.symbols.add_symbol(self.construct_identified(
            context,
            SymbolKind::Property,
            variable.name,
            variable.span,
            hooked_property.span(),
        ));
    }
}
