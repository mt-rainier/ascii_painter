use canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(20, 20);
    canvas.draw_rectangle_with_label(&[(1,1), (1,8), (5,1), (5,8)], "test");
    canvas.draw_arrowed_line(&(10, 10), &(10,18));
    //canvas.reset_boundary();
    canvas.print();
}
