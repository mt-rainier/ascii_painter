//! This module includes the [`CallGraph`] type, which contains the structured information of the
//! callgraph.
use std::collections::{HashMap, HashSet};

/// Component and its function name
type Function = (String, String);
/// Component and the function it's calling
pub type FunctionCall = (String, Function);

/// A structure of callgraph information.
#[derive(Debug)]
pub struct CallGraph {
    /// A map between component identifier to its function set.
    pub components: HashMap<String, HashSet<String>>,
    /// A vec of component identifiers ordered by their occurence.
    pub components_in_order: Vec<String>,
    /// A vec of function calls ([`FunctionCall`]) by their occurence.
    pub func_calls: Vec<FunctionCall>,
}

impl CallGraph {
    /// Processes `callgraph` text and generates [`CallGraph`]
    pub fn new(callgraph: &str) -> Self {
        let mut ret = CallGraph {
            components: HashMap::new(),
            components_in_order: Vec::new(),
            func_calls: Vec::new(),
        };

        let mut function_stack: Vec<(String, usize)> = Vec::new();
        let mut last_component = String::new();
        let mut last_depth = 0;
        for line in callgraph.split('\n') {
            let parts: Vec<&str> = line.split("::").collect();
            if parts.len() < 1 {
                continue;
            }

            let mut curr_component = parts[0].to_string();
            curr_component.retain(|c: char| !c.is_whitespace());
            let mut curr_func_call = parts.get(1).unwrap_or(&"").to_string();
            curr_func_call.retain(|c: char| !c.is_whitespace());

            ret.add_component_func(&curr_component, &curr_func_call);

            let non_space_pos =
                line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
            if non_space_pos > last_depth && !last_component.is_empty() {
                function_stack.push((last_component, last_depth));
            } else if non_space_pos < last_depth {
                function_stack.pop();
            }

            let (calling_component, _) = function_stack
                .last()
                .unwrap_or(&(String::from(""), 0))
                .clone();
            if !curr_component.is_empty() && !curr_func_call.is_empty() {
                ret.func_calls.push((
                    calling_component,
                    (curr_component.clone(), curr_func_call.clone()),
                ));
            }

            last_component = curr_component;
            last_depth = non_space_pos;
        }

        ret
    }

    fn add_component_func(&mut self, component: &str, func: &str) {
        if !component.is_empty() && !self.components.contains_key(component) {
            self.components_in_order.push(component.to_string());
        }
        let set = self
            .components
            .entry(component.to_owned())
            .or_insert(HashSet::new());
        if !func.is_empty() {
            set.insert(func.to_owned());
        }
    }
}

// TODO add unit tests
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_empty_func() {
        let txt =
            fs::read_to_string("./test/callgraph_multi_section.txt").unwrap();

        let callgraph = CallGraph::new(&txt);

        let a_func = callgraph.components.get("ClassA").unwrap();
        assert!(a_func.contains("func_1"));
        let b_func = callgraph.components.get("ClassB").unwrap();
        assert!(b_func.contains("func_4"));
        assert!(b_func.contains("func_2"));
        assert!(b_func.contains("func_3"));

        assert_eq!(callgraph.components_in_order[0], "ClassA");
        assert_eq!(callgraph.components_in_order[1], "ClassB");
        assert_eq!(callgraph.components_in_order[2], "ClassC");

        let func_calls = [
            ("", ("ClassA", "func_1")),
            ("ClassA", ("ClassB", "func_2")),
            ("ClassB", ("ClassB", "func_3")),
            ("ClassB", ("ClassB", "func_4")),
            ("ClassA", ("ClassB", "func_2")),
            ("ClassC", ("ClassB", "func_3")),
        ];
        assert_eq!(func_calls.len(), callgraph.func_calls.len());
        for i in 0..func_calls.len() {
            assert_eq!(func_calls[i].0, callgraph.func_calls[i].0);
            assert_eq!(func_calls[i].1 .0, callgraph.func_calls[i].1 .0);
            assert_eq!(func_calls[i].1 .1, callgraph.func_calls[i].1 .1);
        }
    }

    #[test]
    fn test_multi_section() {
        let txt = fs::read_to_string("./test/callgraph_multi_section_2.txt")
            .unwrap();

        let callgraph = CallGraph::new(&txt);

        println!("{:?}", callgraph);

        let a_func = callgraph.components.get("ClassA").unwrap();
        assert!(a_func.contains("func_1"));
        let b_func = callgraph.components.get("ClassB").unwrap();
        assert!(b_func.contains("func_4"));
        assert!(b_func.contains("func_2"));
        assert!(b_func.contains("func_3"));
        let d_func = callgraph.components.get("ClassD").unwrap();
        assert!(d_func.contains("func_1"));
        let e_func = callgraph.components.get("ClassE").unwrap();
        assert!(e_func.contains("func_4"));
        assert!(e_func.contains("func_2"));
        assert!(e_func.contains("func_3"));

        assert_eq!(callgraph.components_in_order[0], "ClassA");
        assert_eq!(callgraph.components_in_order[1], "ClassB");
        assert_eq!(callgraph.components_in_order[2], "ClassC");
        assert_eq!(callgraph.components_in_order[3], "ClassD");
        assert_eq!(callgraph.components_in_order[4], "ClassE");

        let func_calls = [
            ("ClassA", ("ClassB", "func_2")),
            ("ClassB", ("ClassB", "func_3")),
            ("ClassB", ("ClassB", "func_4")),
            ("ClassA", ("ClassB", "func_2")),
            ("ClassC", ("ClassB", "func_3")),
            ("", ("ClassD", "func_1")),
            ("ClassD", ("ClassE", "func_2")),
            ("ClassE", ("ClassE", "func_3")),
            ("ClassE", ("ClassE", "func_4")),
            ("ClassD", ("ClassE", "func_2")),
            ("ClassF", ("ClassE", "func_3")),
            ("", ("ClassA", "func_1")),
            ("ClassA", ("ClassB", "func_2")),
            ("ClassB", ("ClassB", "func_3")),
            ("ClassB", ("ClassB", "func_4")),
            ("ClassA", ("ClassB", "func_2")),
            ("ClassC", ("ClassB", "func_3")),
            ("", ("ClassD", "func_1")),
            ("ClassD", ("ClassE", "func_2")),
            ("ClassE", ("ClassE", "func_3")),
            ("ClassE", ("ClassE", "func_4")),
            ("ClassD", ("ClassE", "func_2")),
            ("ClassF", ("ClassE", "func_3")),
        ];
        assert_eq!(func_calls.len(), callgraph.func_calls.len());
        for i in 0..func_calls.len() {
            assert_eq!(func_calls[i].0, callgraph.func_calls[i].0);
            assert_eq!(func_calls[i].1 .0, callgraph.func_calls[i].1 .0);
            assert_eq!(func_calls[i].1 .1, callgraph.func_calls[i].1 .1);
        }
    }
}
