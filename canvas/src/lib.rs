use std::cmp::{max, min};
use std::{mem, str};

type Vertex = (usize, usize);

#[derive(Debug)]
pub struct Rectangle {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

#[allow(dead_code)]
pub struct Line {
    start: Vertex,
    end: Vertex,
}

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    boundary: Rectangle,
}

impl ToString for Canvas {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for i in self.boundary.top..=self.boundary.bottom {
            for j in self.boundary.left..=self.boundary.right {
                ret.push(self.buffer[i][j]);
            }
            ret.push('\n');
        }
        ret
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
            boundary: Rectangle {
                left: width - 1,
                right: 0,
                top: height - 1,
                bottom: 0,
            },
        }
    }

    fn change_pixel(&mut self, vertex: &Vertex, c: char) {
        self.buffer[vertex.0][vertex.1] = c;

        self.boundary.top = min(self.boundary.top, vertex.0);
        self.boundary.bottom = max(self.boundary.bottom, vertex.0);
        self.boundary.left = min(self.boundary.left, vertex.1);
        self.boundary.right = max(self.boundary.right, vertex.1);
    }

    pub fn draw_point(&mut self, vertex: &Vertex, c: char) {
        self.change_pixel(vertex, c);
    }

    fn draw_line_overwrite_or_not(&mut self, a: &Vertex, b: &Vertex, overwrite: bool) -> Line {
        if a.1 != b.1 {
            for j in (min(a.1, b.1) + 1)..(max(a.1, b.1)) {
                if self.buffer[a.0][j] == ' ' || overwrite {
                    self.change_pixel(&(a.0, j), '─');
                }
            }
        } else if a.0 != b.0 {
            for j in (min(a.0, b.0) + 1)..(max(a.0, b.0)) {
                if self.buffer[j][a.1] == ' ' || overwrite {
                    self.change_pixel(&(j, a.1), '│');
                }
            }
        }
        Line {
            start: a.to_owned(),
            end: b.to_owned(),
        }
    }

    pub fn draw_line(&mut self, a: &Vertex, b: &Vertex) -> Line {
        self.draw_line_overwrite_or_not(a, b, true)
    }

    pub fn draw_line_under(&mut self, a: &Vertex, b: &Vertex) -> Line {
        self.draw_line_overwrite_or_not(a, b, false)
    }

    pub fn draw_arrowed_line(&mut self, start: &Vertex, end: &Vertex) -> Line {
        let line = self.draw_line(start, end);
        let mut c = '?';
        if start.0 != end.0 {
            if start.0 < end.0 {
                c = '▼';
            } else {
                c = '▲';
            }
        } else if start.1 != end.1 {
            if start.1 < end.1 {
                c = '►';
            } else {
                c = '◄';
            }
        }
        self.change_pixel(end, c);
        line
    }

    fn write_label_within_rec(&mut self, rec: &Rectangle, label: &str) {
        let mut k = 0;
        let mut new_line = false;
        for i in (rec.top + 1)..rec.bottom {
            let mut j = rec.left + 1;
            while j < rec.right {
                if k < label.len() {
                    let c = label.chars().nth(k).unwrap();
                    if new_line && c == ' ' {
                    } else {
                        self.change_pixel(&(i, j), c);
                        j = j + 1;
                    }
                    k = k + 1;
                    new_line = false;
                } else {
                    return;
                }
            }
            new_line = true;
        }
    }

    pub fn draw_rectangle(&mut self, rec: &Rectangle) {
        self.draw_line(&(rec.top, rec.left), &(rec.top, rec.right));
        self.draw_line(&(rec.top, rec.right), &(rec.bottom, rec.right));
        self.draw_line(&(rec.bottom, rec.right), &(rec.bottom, rec.left));
        self.draw_line(&(rec.bottom, rec.left), &(rec.top, rec.left));

        self.draw_point(&(rec.top, rec.left), '┌');
        self.draw_point(&(rec.top, rec.right), '┐');
        self.draw_point(&(rec.bottom, rec.right), '┘');
        self.draw_point(&(rec.bottom, rec.left), '└');
    }

    fn rec_from_vertices(&self, vertices: &[Vertex]) -> Rectangle {
        let mut left = self.height - 1;
        let mut right = 0;
        let mut top = self.width - 1;
        let mut bottom = 0;

        for vertex in vertices.iter() {
            left = min(left, vertex.1);
            right = max(right, vertex.1);
            top = min(top, vertex.0);
            bottom = max(bottom, vertex.0);
        }

        Rectangle {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn draw_rectangle_with_vertices_label(&mut self, vertices: &[Vertex], label: &str) {
        let rec = self.rec_from_vertices(vertices);
        self.write_label_within_rec(&rec, label);
    }

    pub fn draw_rectangle_with_label(&mut self, rec: &Rectangle, label: &str) {
        self.draw_rectangle(rec);
        self.write_label_within_rec(rec, label);
    }

    pub fn draw_line_with_label(
        &mut self,
        mut a: Vertex,
        mut b: Vertex,
        label: &str,
        arrowed: bool,
    ) {
        if arrowed {
            self.draw_arrowed_line(&a, &b);
        } else {
            self.draw_line(&a, &b);
        }
        if a.1 != b.1 {
            if a.1 > b.1 {
                mem::swap(&mut a.1, &mut b.1);
            }
            let label_rec = Rectangle {
                left: a.1 + 1,
                right: b.1 - 1,
                top: a.0 - 1 - (label.len() - 1) / (b.1 - a.1 - 3) - 1,
                bottom: a.0,
            };
            self.write_label_within_rec(&label_rec, label);
        } else if a.0 != b.0 {
            if a.0 > b.0 {
                mem::swap(&mut a.0, &mut b.0);
            }
            let width = (label.len() - 1) / (b.0 - a.0 - 3) + 1;
            let label_rec = Rectangle {
                left: a.1 - width / 2 - 1,
                right: b.1 + width / 2 + 1,
                top: a.0 + 1,
                bottom: b.0 - 1,
            };
            self.write_label_within_rec(&label_rec, label);
        }
    }

    pub fn reset_boundary(&mut self) {
        for i in 0..self.height {
            let mut empty_line = true;
            for j in 0..self.width {
                if self.buffer[i][j] != ' ' {
                    empty_line = false;
                }
            }
            if !empty_line {
                self.boundary.top = min(self.boundary.top, i);
                self.boundary.bottom = max(self.boundary.bottom, i);
            }
        }
        for j in 0..self.width {
            let mut empty_column = true;
            for i in 0..self.height {
                if self.buffer[i][j] != ' ' {
                    empty_column = false;
                    break;
                }
            }
            if !empty_column {
                self.boundary.left = min(self.boundary.left, j);
                self.boundary.right = max(self.boundary.right, j);
            }
        }
    }

    pub fn print(&self) {
        for i in self.boundary.top..=self.boundary.bottom {
            for j in self.boundary.left..=self.boundary.right {
                print!("{}", self.buffer[i][j]);
            }
            println!();
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![vec![' '; self.width]; self.height];
        self.boundary = Rectangle {
            left: self.width - 1,
            right: 0,
            top: self.height - 1,
            bottom: 0,
        };
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn test_rec_with_label() {
        let mut canvas = Canvas::new(20, 20);
        canvas.draw_rectangle_with_label(
            &Rectangle {
                left: 1,
                right: 8,
                top: 1,
                bottom: 5,
            },
            "test",
        );
        canvas.draw_rectangle_with_label(
            &Rectangle {
                left: 1,
                right: 8,
                top: 6,
                bottom: 10,
            },
            "test a super long label",
        );
        canvas.reset_boundary();
        let res = fs::read_to_string("./test/rec_with_label.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }

    #[test]
    fn test_arrowed_line() {
        let mut canvas = Canvas::new(20, 20);
        canvas.draw_arrowed_line(&(10, 10), &(10, 18));
        canvas.reset_boundary();
        let res = fs::read_to_string("./test/arrowed_line.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }

    #[test]
    fn test_line_with_label() {
        let mut canvas = Canvas::new(20, 20);
        canvas.draw_line_with_label((10, 10), (10, 18), "func_call_name", true);
        canvas.draw_line_with_label((15, 10), (15, 18), "func_call_name", false);
        canvas.reset_boundary();
        let res = fs::read_to_string("./test/line_with_label.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }

    #[test]
    fn test_line_with_long_label() {
        let mut canvas = Canvas::new(20, 20);
        canvas.draw_line_with_label(
            (15, 10),
            (15, 18),
            "func_call_name_really_long",
            false,
        );
        canvas.reset_boundary();
        let res = fs::read_to_string("./test/line_with_long_label.txt").unwrap();
        assert_eq!(canvas.to_string(), res);
    }
}
