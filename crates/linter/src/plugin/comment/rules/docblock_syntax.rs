use indoc::indoc;

use mago_ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::rule::Rule;

/// TODO(azjezz): Enable this rule by default once we have improved the linting experience.
#[derive(Clone, Debug)]
pub struct DocblockSyntaxRule;

impl Rule for DocblockSyntaxRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::disabled("Docblock Syntax").with_description(indoc! {"
            Checks for syntax errors in docblock comments. This rule is disabled by default because
            it can be noisy and may not be relevant to all codebases.
        "})
    }
}

impl<'a> Walker<LintContext<'a>> for DocblockSyntaxRule {
    fn walk_program<'ast>(&self, program: &'ast Program, context: &mut LintContext<'a>) {
        for trivia in program.trivia.iter() {
            if let TriviaKind::DocBlockComment = trivia.kind {
                let Err(parse_error) = mago_docblock::parse_trivia(context.interner, trivia) else {
                    continue;
                };

                let issue = Issue::new(context.level(), parse_error.to_string())
                    .with_annotation(Annotation::primary(parse_error.span()))
                    .with_annotation(Annotation::secondary(trivia.span()))
                    .with_note(parse_error.note())
                    .with_help(parse_error.help());

                context.report(issue);
            }
        }
    }
}
