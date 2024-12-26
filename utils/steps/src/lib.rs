#![no_std]
#[macro_export]
macro_rules! define_steps {
    (
        $(
            $step_name:ident => {
                value_type: $value_type:ty,
                children_types: ($($children_types:ty),*),
            }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum StepKind {
            $($step_name),*
        }

        $(
            concat_idents::concat_idents!(children_type = $step_name, Children {
                struct $step_name {
                    kind: StepKind,
                    value: $value_type,
                    children: children_type,
                }

                struct children_type($(pub $children_types),*);

                impl $step_name {
                    pub fn new(value: $value_type, children: children_type) -> Self {
                        Self {
                            kind: StepKind::$step_name,
                            value,
                            children,
                        }
                    }
                }
            });
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        define_steps! {
            StepA => {
                value_type: i32,
                children_types: (()),
            },
            StepB => {
                value_type: i32,
                children_types: (StepA),
            },
        }
    }
}
