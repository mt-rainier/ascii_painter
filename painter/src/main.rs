use canvas::*;
use std::collections::{HashMap, HashSet};

/// Component and its function name
type Function = (String, String);
/// Component and the function it's calling
type FunctionCall = (String, Function);

struct CallGraph {
    components: HashMap<String, HashSet<String>>,
    func_calls: Vec<FunctionCall>,
}

impl CallGraph {
    fn new(callgraph: &str) -> Self {
        let mut ret = CallGraph {
            components: HashMap::new(),
            func_calls: Vec::new(),
        };

        let mut function_stack: Vec<(String, usize)> = Vec::new();
        let mut last_component = String::new();
        let mut last_function = String::new();
        let mut last_depth = 0;
        for line in callgraph.split('\n') {
            let parts: Vec<&str> = line.split("::").collect();
            println!("{:?}", parts);
            if parts.len() < 2 {
                continue;
            }

            let mut curr_component = parts[0].to_string();
            curr_component.retain(|c: char| !c.is_whitespace());
            let mut curr_func_call = parts[1].to_string();
            curr_func_call.retain(|c: char| !c.is_whitespace());

            ret.add_component_func(&curr_component, &curr_func_call);

            let non_space_pos = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
            if non_space_pos > last_depth && !last_component.is_empty() && !last_function.is_empty()
            {
                function_stack.push((last_component, last_depth));
            } else if non_space_pos < last_depth {
                function_stack.pop();
            }

            let (calling_component, _) = function_stack
                .last()
                .unwrap_or(&(String::from(""), 0))
                .clone();
            ret.func_calls.push((
                calling_component,
                (curr_component.clone(), curr_func_call.clone()),
            ));

            last_component = curr_component;
            last_function = curr_func_call;
            last_depth = non_space_pos;
        }

        ret
    }

    fn add_component_func(&mut self, component: &str, func: &str) {
        let set = self
            .components
            .entry(component.to_owned())
            .or_insert(HashSet::new());
        set.insert(func.to_owned());
    }
}

struct Painter {
    components: HashMap<String, Rectangle>,
    func_calls: HashMap<FunctionCall, Line>,
}

impl Painter {
    fn new() -> Self {
        Painter {
            components: HashMap::new(),
            func_calls: HashMap::new(),
        }
    }

    fn draw_components(&mut self, canvas: &mut Canvas, callgraph: &CallGraph) {
        let mut right_boundary = 0;
        let rec_width = 20;
        let rec_height = 5;
        for (component, _) in &callgraph.components {
            let rec = Rectangle {
                left: 5 + right_boundary,
                right: rec_width + 5 + right_boundary,
                top: 1,
                bottom: rec_height + 1,
            };
            canvas.draw_rectangle_with_label(&rec, &component);
            right_boundary = rec.right;
            self.components.insert(component.to_owned(), rec);
        }
    }

    fn draw_function_calls(&mut self, canvas: &mut Canvas, callgraph: &CallGraph) -> usize {
        let mut bottom_boundary = 3;
        let virtual_rec = Rectangle {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            };
        for f in &callgraph.func_calls {
            let component = f.0.to_owned();
            let func = f.1.to_owned();
            let called_rec = self.components.get(&func.0).unwrap();
            let calling_rec = self.components.get(&component).unwrap_or(&virtual_rec);
            let calling_center = (calling_rec.left + calling_rec.right) / 2;
            let mut called_center = (called_rec.left + called_rec.right) / 2;
            if calling_center < called_center {
                called_center = called_center - 1;
            } else {
                called_center = called_center + 1;
            }
            canvas.draw_horizontal_line_with_label(
                (called_rec.bottom + bottom_boundary, calling_center),
                (called_rec.bottom + bottom_boundary, called_center),
                &func.1,
                true,
            );
            bottom_boundary = bottom_boundary + 3;
        }
        // return length of the lifecycle line
        bottom_boundary
    }

    fn draw_lifecycle_line(
        &self,
        canvas: &mut Canvas,
        components: &HashMap<String, Rectangle>,
        length: usize,
    ) {
        for (_, rec) in components {
            let center = (rec.left + rec.right) / 2;
            canvas.draw_line(&(rec.bottom, center), &(rec.bottom + length, center));
        }
    }

    fn draw(&mut self, canvas: &mut Canvas, callgraph_str: &str) {
        let callgraph = CallGraph::new(callgraph_str);
        println!("{:?}", callgraph.func_calls);
        self.draw_components(canvas, &callgraph);
        let length = self.draw_function_calls(canvas, &callgraph);
        self.draw_lifecycle_line(canvas, &self.components, length);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::fs;

    use canvas::Canvas;

    use super::*;

    #[test]
    fn test_callgraph() {
        let txt = fs::read_to_string("./test/callgraph.txt").unwrap();

        let mut canvas = Canvas::new(500, 500);

        let mut painter = Painter::new();

        painter.draw(&mut canvas, &txt);

        canvas.print();

        println!("{:?}", txt);
    }
}
