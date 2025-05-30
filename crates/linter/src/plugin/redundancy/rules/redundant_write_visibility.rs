use indoc::indoc;

use mago_ast::Modifier;
use mago_ast::Property;
use mago_fixer::SafetyClassification;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RedundantWriteVisibilityRule;

impl Rule for RedundantWriteVisibilityRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Redundant Write Visibility", Level::Help)
            .with_description(indoc! {"
                Detects redundant write visibility modifiers on properties.
            "})
            .with_example(RuleUsageExample::invalid(
                "A redundant write visibility modifier",
                indoc! {r#"
                    <?php

                    final class User
                    {
                        public public(set) $name;
                    }
                "#},
            ))
    }
}

impl<'a> Walker<LintContext<'a>> for RedundantWriteVisibilityRule {
    fn walk_in_property(&self, property: &Property, context: &mut LintContext<'a>) {
        let modifiers = property.modifiers();
        if modifiers.is_empty() {
            return;
        }

        let Some(write_visibility) = modifiers.get_first_write_visibility() else {
            return;
        };

        let Some(read_visibility) = modifiers.get_first_read_visibility() else {
            return;
        };

        match (read_visibility, write_visibility) {
            (Modifier::Public(_), Modifier::PublicSet(_))
            | (Modifier::Protected(_), Modifier::ProtectedSet(_))
            | (Modifier::Private(_), Modifier::PrivateSet(_)) => {
                let issue = Issue::new(context.level(), "Identical write visibility has no effect.")
                    .with_help("Remove the redundant write visibility modifier.")
                    .with_annotations(vec![
                        Annotation::primary(write_visibility.span()).with_message("Redundant write visibility."),
                        Annotation::secondary(read_visibility.span()).with_message("Read visibility."),
                    ]);

                context.report_with_fix(issue, |plan| {
                    let range = write_visibility.span().to_range();

                    plan.delete(range, SafetyClassification::PotentiallyUnsafe)
                });
            }
            _ => {}
        }
    }
}
