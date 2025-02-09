#![no_std]
#[macro_export]
macro_rules! define_steps {
    (
        $(
            $step_name:ident => {
                title: $title:expr,
                value_type: $value_type:ty,
                children_types: ($($children_types:ty),*),
            }
        ),* $(,)?
    ) => {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum StepKind {
            $($step_name),*
        }

        $(
            concat_idents::concat_idents!(children_type = $step_name, Children {
                #[wasm_bindgen(getter_with_clone)]
                #[derive(Clone)]
                pub struct $step_name {
                    pub title: alloc::string::String,
                    pub kind: StepKind,
                    pub value: $value_type,
                    pub children: children_type,
                }

                impl Default for $step_name {
                    fn default() -> Self {
                        Self::new(Default::default(), Default::default())
                    }
                }

                #[wasm_bindgen(getter_with_clone)]
                #[derive(Clone, Default)]
                pub struct children_type($(pub $children_types),*);

                impl $step_name {
                    pub fn new(value: $value_type, children: children_type) -> Self {
                        Self {
                            title: $title,
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
