use canvas::*;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

mod callgraph;
use callgraph::{CallGraph, FunctionCall};

pub struct Painter {
    components: HashMap<String, Rectangle>,
}

const EXTRA_VERTICAL_MARGIN: usize = 2;
const DEFAULT_SELF_CALL_WIDTH: usize = 5;

impl Painter {
    pub fn new() -> Self {
        Painter {
            components: HashMap::new(),
        }
    }

    fn draw_components(&mut self, canvas: &mut Canvas, callgraph: &CallGraph) {
        let mut right_boundary = 0;
        let max_rec_width = 20;
        let horizontal_gap = 5;
        for component in &callgraph.components_in_order {
            let width = min(max_rec_width, component.len() + 1);
            let rec = Rectangle {
                left: horizontal_gap + right_boundary,
                right: width + horizontal_gap + right_boundary,
                top: 1,
                bottom: (component.len() - 1) / max_rec_width + 3,
            };
            canvas.draw_rectangle_with_label(&rec, &component);
            right_boundary = rec.right;
            self.components.insert(component.to_owned(), rec);
        }
    }

    fn draw_cross_component_call(
        &self,
        canvas: &mut Canvas,
        f: &FunctionCall,
        mut bottom_boundary: usize,
    ) -> usize {
        let virtual_rec = Rectangle {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };

        let component = f.0.to_owned();
        let func = f.1.to_owned();

        let called_rec = self.components.get(&func.0).unwrap();
        let calling_rec = self.components.get(&component).unwrap_or(&virtual_rec);

        let calling_center = (calling_rec.left + calling_rec.right) / 2;
        let mut called_center = (called_rec.left + called_rec.right) / 2;

        // space for arrow
        if calling_center < called_center {
            called_center = called_center - 1;
        } else {
            called_center = called_center + 1;
        }

        let label_height = (func.1.len() - 1)
            / (max(called_center, calling_center) - min(called_center, calling_center) - 1)
            + 1;

        bottom_boundary += EXTRA_VERTICAL_MARGIN;

        canvas.draw_line_with_label(
            (
                bottom_boundary + label_height,
                calling_center,
            ),
            (
                bottom_boundary + label_height,
                called_center,
            ),
            &func.1,
            true,
        );

        bottom_boundary + label_height
    }

    fn draw_same_component_call(
        &self,
        canvas: &mut Canvas,
        f: &FunctionCall,
        mut bottom_boundary: usize,
    ) -> usize {
        let func = f.1.to_owned();

        let called_rec = self.components.get(&func.0).unwrap();

        let called_center = (called_rec.left + called_rec.right) / 2;

        bottom_boundary += EXTRA_VERTICAL_MARGIN;

        canvas.draw_line(
            &(bottom_boundary, called_center),
            &(
                bottom_boundary,
                called_center + DEFAULT_SELF_CALL_WIDTH,
            ),
        );

        canvas.draw_point(&(bottom_boundary, called_center + DEFAULT_SELF_CALL_WIDTH), '┐');

        let label_height = (func.1.len() - 1)
            / (DEFAULT_SELF_CALL_WIDTH)
            + 1;

        canvas.draw_line_with_label(
            (
                bottom_boundary,
                called_center + DEFAULT_SELF_CALL_WIDTH,
            ),
            (
                bottom_boundary + label_height + 2,
                called_center + DEFAULT_SELF_CALL_WIDTH,
            ),
            &func.1,
            false,
        );

        bottom_boundary += label_height + 2;

        canvas.draw_arrowed_line(
            &(
                bottom_boundary,
                called_center + DEFAULT_SELF_CALL_WIDTH,
            ),
            &(bottom_boundary, called_center + 1),
        );

        canvas.draw_point(&(bottom_boundary, called_center + DEFAULT_SELF_CALL_WIDTH), '┘');

        bottom_boundary
    }

    fn draw_function_calls(&self, canvas: &mut Canvas, callgraph: &CallGraph) -> usize {
        let mut bottom_boundary = 0;
        for (_component_label, rec) in &self.components {
            bottom_boundary = max(bottom_boundary, rec.bottom);
        }

        for f in &callgraph.func_calls {
            if !f.0.eq(&f.1 .0) {
                bottom_boundary = self.draw_cross_component_call(canvas, f, bottom_boundary);
            } else {
                bottom_boundary = self.draw_same_component_call(canvas, f, bottom_boundary);
            }
        }
        // return the expected bottom of the lifecycle line
        bottom_boundary + EXTRA_VERTICAL_MARGIN
    }

    fn draw_lifecycle_line(
        &self,
        canvas: &mut Canvas,
        components: &HashMap<String, Rectangle>,
        bottom: usize,
    ) {
        for (_, rec) in components {
            let center = (rec.left + rec.right) / 2;
            canvas.draw_line_under(&(rec.bottom, center), &(bottom, center));
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, callgraph_str: &str) {
        let callgraph = CallGraph::new(callgraph_str);
        self.draw_components(canvas, &callgraph);
        let length = self.draw_function_calls(canvas, &callgraph);
        self.draw_lifecycle_line(canvas, &self.components, length);
    }
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

        canvas.reset_boundary();
        let res = fs::read_to_string("./test/callgraph_res.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }

    #[test]
    fn test_callgraph_self_call() {
        let txt = fs::read_to_string("./test/callgraph_self_call.txt").unwrap();

        let mut canvas = Canvas::new(500, 500);

        let mut painter = Painter::new();

        painter.draw(&mut canvas, &txt);

        canvas.reset_boundary();
        canvas.print();
        let res = fs::read_to_string("./test/callgraph_self_call_res.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }
}
